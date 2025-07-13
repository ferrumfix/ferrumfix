use super::{Config, DecodeError, Decoder, Message, RawDecoder, RawFrame};
use crate::GetConfig;
use crate::{FieldType, FieldValueError};
use bytes::{Bytes, BytesMut};
use rustc_hash::FxHashMap;
use tokio_util::codec;

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
        let raw_bytes = src.split().freeze();
        let raw_bytes_clone = raw_bytes.clone();
        let result = self.decoder.decode(&raw_bytes);
        match result {
            Ok(message) => {
                // Convert the borrowed Message to an owned OwnedMessage
                let owned_message = OwnedMessage::from_message(message, raw_bytes_clone);
                Ok(Some(owned_message))
            }
            Err(DecodeError::Invalid { .. }) => Ok(None),
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
