use super::{Config, DecodeError, Decoder, Message, RawDecoder, RawFrame};
use crate::GetConfig;
use crate::{FieldType, FieldValueError};
use bytes::{Buf, Bytes, BytesMut};
use rustc_hash::FxHashMap;
use tokio_util::codec;

/// Length of FIX checksum field: "CheckSum=NNN|" (10= + 3 digits + separator)
const CHECKSUM_FIELD_LEN: usize = 7;

/// Minimum data threshold to consider potential malformed data vs incomplete data
const MALFORMED_DATA_THRESHOLD: usize = 64;

/// Parse FIX message header to extract body length and header end position.
/// Searches for the next potential start of a FIX message (8=FIX pattern).
/// Returns the position offset from the start of the search data.
fn find_next_fix_start(data: &[u8]) -> Option<usize> {
    for (i, window) in data.windows(5).enumerate() {
        if window.starts_with(b"8=FIX") {
            return Some(i);
        }
    }
    None
}

/// Returns (header_end_pos, body_length) if successful.
fn parse_fix_header(data: &[u8], separator: u8) -> Option<(usize, usize)> {
    if data.len() < 16 {
        // Minimum for "8=FIX.4.2|9=NNN|"
        return None;
    }

    let mut pos = 0;
    let find_next = |start: usize, byte: u8| {
        data[start..]
            .iter()
            .position(|&b| b == byte)
            .map(|i| start + i)
    };

    // Find first field separator (after BeginString)
    pos = find_next(pos, b'=')? + 1; // Skip "8="
    pos = find_next(pos, separator)? + 1; // Skip BeginString value and separator

    // Find BodyLength field start
    pos = find_next(pos, b'=')? + 1; // Skip "9="
    let body_len_start = pos;

    // Find BodyLength field end
    let body_len_end = find_next(pos, separator)?;

    // Parse body length value
    let mut body_length: usize = 0;
    for &byte in &data[body_len_start..body_len_end] {
        if !byte.is_ascii_digit() {
            return None;
        }
        body_length = body_length
            .wrapping_mul(10)
            .wrapping_add((byte - b'0') as usize);
    }

    Some((body_len_end + 1, body_length))
}

/// A FIX message that owns its data, suitable for use with Tokio codecs.
#[derive(Debug, Clone)]
pub struct OwnedMessage {
    /// The raw message bytes
    raw_bytes: Bytes,
    /// Parsed fields stored as owned data
    fields: FxHashMap<u32, Bytes>,
}

impl OwnedMessage {
    /// Create an OwnedMessage from raw bytes with pre-parsed fields
    fn new(raw_bytes: Bytes, fields: FxHashMap<u32, Bytes>) -> Self {
        Self { raw_bytes, fields }
    }

    /// Create an OwnedMessage from a borrowed Message by copying field data
    fn from_message<T>(message: Message<'_, T>, raw_bytes: Bytes) -> Self
    where
        T: AsRef<[u8]>,
    {
        let mut fields = FxHashMap::default();

        // Extract ALL fields from the message by iterating over them
        // This ensures no fields are lost during conversion to OwnedMessage
        for (tag, value) in message.fields() {
            fields.insert(tag.get(), Bytes::copy_from_slice(value));
        }

        Self::new(raw_bytes, fields)
    }

    /// Returns the FIX message type of this message.
    pub fn msg_type(&self) -> Result<String, FieldValueError<std::str::Utf8Error>> {
        self.get(35)
    }

    /// Returns a deserialized value of a field.
    pub fn get<V>(&self, tag: u32) -> Result<V, FieldValueError<<V as FieldType<'_>>::Error>>
    where
        V: for<'a> FieldType<'a>,
    {
        let bytes = self.get_raw(tag).ok_or(FieldValueError::Missing)?;
        V::deserialize(bytes).map_err(FieldValueError::Invalid)
    }

    /// Returns the raw byte value for a given tag.
    pub fn get_raw(&self, tag: u32) -> Option<&[u8]> {
        self.fields.get(&tag).map(|bytes| bytes.as_ref())
    }

    /// Returns the underlying byte contents of the message.
    pub fn as_bytes(&self) -> &[u8] {
        self.raw_bytes.as_ref()
    }

    /// Returns the number of FIX tags contained in this message.
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Returns `true` if this message has no fields, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Returns an iterator over all fields in this message.
    pub fn fields(&self) -> impl Iterator<Item = (u32, &[u8])> {
        self.fields
            .iter()
            .map(|(&tag, value)| (tag, value.as_ref()))
    }
}

/// A [`tokio_util::codec::Decoder`] for raw FIX frames.
#[derive(Debug)]
pub struct TokioRawDecoder {
    raw_decoder: RawDecoder,
}

impl TokioRawDecoder {
    /// Create a new TokioRawDecoder with default configuration
    pub fn new() -> Self {
        Self {
            raw_decoder: RawDecoder::new(),
        }
    }
}

impl Default for TokioRawDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl codec::Decoder for TokioRawDecoder {
    type Item = RawFrame<Bytes>;
    type Error = DecodeError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let split = src.split();
        let result = self.raw_decoder.decode(split);
        match result {
            Ok(raw_frame) => Ok(Some(RawFrame {
                data: raw_frame.data.freeze(),
                begin_string: raw_frame.begin_string,
                payload: raw_frame.payload,
            })),
            Err(DecodeError::Invalid { .. }) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

impl GetConfig for TokioRawDecoder {
    type Config = Config;

    fn config(&self) -> &Self::Config {
        self.raw_decoder.config()
    }

    fn config_mut(&mut self) -> &mut Self::Config {
        self.raw_decoder.config_mut()
    }
}

/// A [`tokio_util::codec::Decoder`] for FIX messages.
#[derive(Debug)]
#[cfg_attr(docsrs, doc(cfg(feature = "utils-tokio")))]
pub struct TokioDecoder {
    decoder: Decoder,
}

impl TokioDecoder {
    /// Create a new TokioDecoder with the given dictionary
    pub fn new(dict: crate::Dictionary) -> Self {
        Self {
            decoder: Decoder::new(dict),
        }
    }
}

impl codec::Decoder for TokioDecoder {
    type Item = OwnedMessage;
    type Error = DecodeError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // Check if we have enough data for a minimal FIX message
        if src.len() < crate::tagvalue::utils::MIN_FIX_MESSAGE_LEN_IN_BYTES {
            return Ok(None); // Need more data
        }

        // Try to parse the header to determine the expected message length
        let header_info = match parse_fix_header(src, self.decoder.config().separator) {
            Some((header_end_pos, body_length)) => {
                // Calculate total expected message length
                // FIX message format: BeginString=... | BodyLength=N | ... body (N bytes) ... | CheckSum=...
                let body_start = header_end_pos; // After BodyLength field
                let expected_total_length = body_start + body_length + CHECKSUM_FIELD_LEN; // for CheckSum=NNN|

                // Check if we have enough bytes for the complete message
                if src.len() < expected_total_length {
                    return Ok(None); // Need more data for complete message
                }
                (header_end_pos, body_length)
            }
            None => {
                // Header parsing failed - could be incomplete or malformed data
                // If we have substantial data, try to skip past malformed data to prevent infinite loops
                if src.len() >= MALFORMED_DATA_THRESHOLD {
                    // Sufficient data suggests malformed rather than incomplete
                    // Search for next potential FIX message start (8=FIX pattern)
                    if let Some(next_fix_start) = find_next_fix_start(&src[1..]) {
                        // Skip past malformed data to next potential FIX message
                        src.advance(next_fix_start + 1);
                        // Try again with the new position
                        return self.decode(src);
                    } else {
                        // No FIX pattern found, consume a byte to avoid infinite loop
                        src.advance(1);
                        return self.decode(src);
                    }
                } else {
                    // Likely incomplete data, wait for more
                    return Ok(None);
                }
            }
        };

        // We have enough data - split off exactly the message we need
        let raw_bytes = src.split_to(header_info.0 + header_info.1 + 7).freeze();
        let raw_bytes_clone = raw_bytes.clone();

        let result = self.decoder.decode(&raw_bytes);
        match result {
            Ok(message) => {
                // Convert the borrowed Message to an owned OwnedMessage
                let owned_message = OwnedMessage::from_message(message, raw_bytes_clone);
                Ok(Some(owned_message))
            }
            Err(DecodeError::Invalid { .. }) => {
                // Invalid message - skip it and try next bytes
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }
}

impl GetConfig for TokioDecoder {
    type Config = Config;

    fn config(&self) -> &Self::Config {
        self.decoder.config()
    }

    fn config_mut(&mut self) -> &mut Self::Config {
        self.decoder.config_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Dictionary;
    use bytes::BytesMut;
    use tokio_util::codec::Decoder;

    #[test]
    fn test_owned_message_basic_functionality() {
        let dict = Dictionary::fix44().unwrap();
        let mut decoder = super::Decoder::new(dict);
        decoder.config_mut().separator = b'|';

        let raw_data = b"8=FIX.4.4|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=185|";
        let message = decoder.decode(raw_data).unwrap();
        let owned = OwnedMessage::from_message(message, Bytes::from_static(raw_data));

        assert_eq!(owned.get_raw(8), Some(b"FIX.4.4" as &[u8]));
        assert_eq!(owned.get_raw(35), Some(b"0" as &[u8]));
        assert_eq!(owned.get::<String>(35), Ok("0".to_string()));
        assert!(!owned.is_empty());
        assert!(!owned.is_empty());
    }

    #[test]
    fn test_owned_message_field_access() {
        let dict = Dictionary::fix44().unwrap();
        let mut decoder = super::Decoder::new(dict);
        decoder.config_mut().separator = b'|';

        // Use a valid FIX message from the working tests
        let raw_data = b"8=FIX.4.4|9=122|35=D|34=215|49=CLIENT12|52=20100225-19:41:57.316|56=B|1=Marcel|11=13346|21=1|40=2|44=5|54=1|59=0|60=20100225-19:39:52.020|10=072|";
        let message = decoder.decode(raw_data).unwrap();
        let owned = OwnedMessage::from_message(message, Bytes::from_static(raw_data));

        // Test various field types
        assert_eq!(owned.get::<String>(8), Ok("FIX.4.4".to_string()));
        assert_eq!(owned.get::<String>(35), Ok("D".to_string()));
        assert_eq!(owned.get::<u32>(34), Ok(215)); // Sequence number in this message
        assert_eq!(owned.msg_type(), Ok("D".to_string()));

        // Test missing field
        assert!(owned.get_raw(999).is_none());
    }

    #[test]
    fn test_owned_message_fields_iteration() {
        let dict = Dictionary::fix44().unwrap();
        let mut decoder = super::Decoder::new(dict);
        decoder.config_mut().separator = b'|';

        let raw_data = b"8=FIX.4.4|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=185|";
        let message = decoder.decode(raw_data).unwrap();
        let owned = OwnedMessage::from_message(message, Bytes::from_static(raw_data));

        let fields: Vec<_> = owned.fields().collect();
        assert!(!fields.is_empty());

        // Check that we have some of the expected fields
        let has_version = fields
            .iter()
            .any(|(tag, value)| *tag == 8 && *value == b"FIX.4.4");
        let has_msg_type = fields
            .iter()
            .any(|(tag, value)| *tag == 35 && *value == b"0");
        assert!(has_version);
        assert!(has_msg_type);
    }

    #[tokio::test]
    async fn test_tokio_decoder_integration() {
        let dict = Dictionary::fix44().unwrap();
        let mut decoder = TokioDecoder::new(dict);
        decoder.config_mut().separator = b'|';

        let data = b"8=FIX.4.4|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=185|";
        let mut buf = BytesMut::from(&data[..]);

        let result = decoder.decode(&mut buf).unwrap();
        assert!(result.is_some());

        let message = result.unwrap();
        assert_eq!(message.get_raw(8), Some(b"FIX.4.4" as &[u8]));
        assert_eq!(message.get_raw(35), Some(b"0" as &[u8]));
    }

    #[tokio::test]
    async fn test_tokio_decoder_multiple_messages() {
        let dict = Dictionary::fix44().unwrap();
        let mut decoder = TokioDecoder::new(dict);
        decoder.config_mut().separator = b'|';

        // Test multiple messages in sequence
        let messages = [
            b"8=FIX.4.4|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=185|".as_slice(),
            b"8=FIX.4.4|9=42|35=1|49=A|56=B|34=13|52=20100304-07:59:31|10=186|".as_slice(),
            b"8=FIX.4.4|9=42|35=2|49=A|56=B|34=14|52=20100304-07:59:32|10=187|".as_slice(),
        ];

        for (i, msg_data) in messages.iter().enumerate() {
            let mut buf = BytesMut::from(*msg_data);
            let result = decoder.decode(&mut buf).unwrap();
            assert!(result.is_some(), "Message {i} should decode successfully");

            let message = result.unwrap();
            assert_eq!(message.get_raw(8), Some(b"FIX.4.4" as &[u8]));
            assert_eq!(message.get::<u32>(34), Ok(12 + i as u32));
        }
    }

    #[tokio::test]
    async fn test_tokio_decoder_invalid_messages() {
        let dict = Dictionary::fix44().unwrap();
        let mut decoder = TokioDecoder::new(dict);
        decoder.config_mut().separator = b'|';

        // Test incomplete message
        let incomplete_data = b"8=FIX.4.4|9=42|35=0|";
        let mut buf = BytesMut::from(&incomplete_data[..]);
        let result = decoder.decode(&mut buf);
        assert!(result.is_ok()); // Should return Ok(None) for incomplete

        // Test malformed message
        let malformed_data = b"invalid message data";
        let mut buf = BytesMut::from(&malformed_data[..]);
        let result = decoder.decode(&mut buf);
        assert!(result.is_ok()); // Should return Ok(None) for invalid
    }

    #[tokio::test]
    async fn test_tokio_raw_decoder() {
        let mut decoder = TokioRawDecoder::new();
        decoder.config_mut().separator = b'|';

        let data = b"8=FIX.4.4|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=185|";
        let mut buf = BytesMut::from(&data[..]);

        let result = decoder.decode(&mut buf).unwrap();
        assert!(result.is_some());

        let raw_frame = result.unwrap();
        assert!(!raw_frame.data.is_empty());
        assert_eq!(raw_frame.begin_string(), b"FIX.4.4");
    }

    #[tokio::test]
    async fn test_tokio_decoder_config_modification() {
        let dict = Dictionary::fix44().unwrap();
        let mut decoder = TokioDecoder::new(dict);

        // Test different separator
        decoder.config_mut().separator = b'^';

        let data = b"8=FIX.4.4^9=42^35=0^49=A^56=B^34=12^52=20100304-07:59:30^10=185^";
        let mut buf = BytesMut::from(&data[..]);

        let result = decoder.decode(&mut buf).unwrap();
        assert!(result.is_some());

        let message = result.unwrap();
        assert_eq!(message.get_raw(8), Some(b"FIX.4.4" as &[u8]));
    }

    #[test]
    fn test_owned_message_empty_and_len() {
        let empty_fields = FxHashMap::default();
        let empty_bytes = Bytes::new();
        let empty_msg = OwnedMessage::new(empty_bytes, empty_fields);

        assert!(empty_msg.is_empty());
        assert_eq!(empty_msg.len(), 0);
        assert!(empty_msg.fields().next().is_none());
    }

    #[test]
    fn test_owned_message_as_bytes() {
        let dict = Dictionary::fix44().unwrap();
        let mut decoder = super::Decoder::new(dict);
        decoder.config_mut().separator = b'|';

        let raw_data = b"8=FIX.4.4|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=185|";
        let message = decoder.decode(raw_data).unwrap();
        let owned = OwnedMessage::from_message(message, Bytes::from_static(raw_data));

        assert_eq!(owned.as_bytes(), raw_data);
    }
}
