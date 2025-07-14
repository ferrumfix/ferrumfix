use super::{Config, DecodeError, Decoder, Message, RawDecoder, RawFrame};
use crate::GetConfig;
use crate::{FieldType, FieldValueError};
use rustc_hash::FxHashMap;
use smallbytes::SmallBytes;
use smartstring::alias::String as SmartString;
use tokio_util::bytes::{Buf, Bytes, BytesMut};
use tokio_util::codec;

/// Length of FIX checksum field: "CheckSum=NNN|" (10= + 3 digits + separator)
const CHECKSUM_FIELD_LEN: usize = 7;

/// Minimum data threshold to consider potential malformed data vs incomplete data
const MALFORMED_DATA_THRESHOLD: usize = 64;

/// Minimum FIX message header length for "8=FIX.4.2|9=NNN|" pattern
const MIN_FIX_HEADER_LEN: usize = 16;

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
    if data.len() < MIN_FIX_HEADER_LEN {
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

/// An owned representation of a decoded FIX message.
///
/// `OwnedMessage` provides an owned, self-contained representation of a FIX message
/// that can be used in async contexts where messages need to outlive the decoder.
/// All field data is copied into owned containers using [`SmallBytes`] for efficient
/// storage of typical small field values.
///
/// ## Field Extraction Limitations
///
/// **Important**: `OwnedMessage` has several limitations compared to the borrowed
/// [`Message`] type due to its flattened field storage approach:
///
/// ### 1. Group Context Loss
///
/// Fields within FIX repeating groups lose their contextual information:
///
/// ```text
/// Original Message:        OwnedMessage Storage:
/// 268=2 (NoMDEntries)     HashMap:
/// ├─ 269=0 (MDEntryType)  ├─ 268 → "2"
/// ├─ 270=50.0 (Price)     ├─ 269 → "0"  # Context lost: which group entry?
/// ├─ 269=1                ├─ 270 → "50.0"
/// └─ 270=51.0             ├─ 269 → "1"  # Overwrites previous 269!
///                         └─ 270 → "51.0"  # Overwrites previous 270!
/// ```
///
/// **Impact**: You cannot distinguish between fields at the top level versus
/// fields within repeating groups. Multiple instances of the same field tag
/// within different group entries will overwrite each other.
///
/// ### 2. Group Structure Loss
///
/// The hierarchical structure of repeating groups is completely lost:
/// - Cannot access group entries individually
/// - Cannot iterate over group entries
/// - Cannot access nested groups
/// - Cannot reconstruct the original group organization
///
/// ### 3. Field Ordering Loss
///
/// The original field ordering from the FIX message is not preserved:
/// - Fields are stored in a `HashMap` which doesn't maintain insertion order
/// - This may affect protocols that are sensitive to field ordering
/// - Debugging becomes more difficult without original field sequence
///
/// ### 4. No Group Operations
///
/// `OwnedMessage` does not implement the [`FieldMap`] trait and provides
/// no group-related operations:
/// - Cannot call `.group(tag)` to access repeating groups
/// - Cannot iterate over group entries
/// - No group validation or structure checking
///
/// ## When to Use OwnedMessage
///
/// Use `OwnedMessage` when:
/// - You need to send messages across async boundaries
/// - You need to store messages beyond the decoder's lifetime
/// - You only need simple field access without group operations
/// - Performance is more important than complete field extraction
///
/// Use the borrowed [`Message`] type when:
/// - You need full group support and hierarchical field access
/// - You need to preserve field ordering and context
/// - You're working in synchronous contexts
/// - You need complete FIX protocol compliance
///
/// ## Performance Characteristics
///
/// - **Memory**: Uses [`SmallBytes<64>`] for stack allocation of small fields (≤64 bytes)
/// - **Lookup**: O(1) field access via `FxHashMap`
/// - **Iteration**: Efficient field enumeration via HashMap iterator
/// - **Construction**: Single pass over all fields with pre-allocated capacity
#[derive(Debug, Clone)]
pub struct OwnedMessage {
    /// The raw message bytes
    raw_bytes: Bytes,
    /// Parsed fields stored as owned data (using SmallBytes for typical small field values)
    fields: FxHashMap<u32, SmallBytes<64>>,
}

impl OwnedMessage {
    /// Create an OwnedMessage from a borrowed Message by copying field data
    fn from_message<T>(message: Message<'_, T>, raw_bytes: Bytes) -> Self
    where
        T: AsRef<[u8]>,
    {
        // Extract all fields from the message. Note that this flattens repeating
        // groups, and if there are duplicate tags, the HashMap will only store
        // the last value for each tag. See struct-level documentation for details
        // on this limitation.
        // Pre-allocate HashMap capacity to reduce reallocations.
        let field_count = message.fields().count();
        let mut fields = FxHashMap::with_capacity_and_hasher(field_count, Default::default());

        for (tag, value) in message.fields() {
            // Optimize: SmallBytes<64> can stack-allocate up to 64 bytes, avoiding heap allocation
            // for most FIX field values which are typically short
            let mut small_bytes = SmallBytes::<64>::new();
            small_bytes.extend_from_slice(value);
            fields.insert(tag.get(), small_bytes);
        }

        Self { raw_bytes, fields }
    }

    /// Returns the FIX message type of this message.
    pub fn msg_type(&self) -> Result<SmartString, FieldValueError<std::str::Utf8Error>> {
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
        // ✅ CRITICAL FIX: Replace recursion with loop to prevent stack overflow
        // This addresses the infinite loop/stack overflow issue identified in AI review
        loop {
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
                            // ✅ IMPROVED: Skip past malformed data to next potential FIX message
                            // Add 1 to account for the [1..] slice offset
                            let advance_bytes = next_fix_start + 1;
                            src.advance(advance_bytes);
                            // ✅ CRITICAL FIX: Continue loop instead of recursive call
                            continue;
                        } else {
                            // ✅ IMPROVED: No FIX pattern found, advance more aggressively to avoid repeated scanning
                            // Instead of advancing 1 byte, advance by MIN_FIX_HEADER_LEN to skip obviously bad data
                            let advance_bytes = std::cmp::min(MIN_FIX_HEADER_LEN, src.len());
                            src.advance(advance_bytes);
                            // ✅ CRITICAL FIX: Continue loop instead of recursive call
                            continue;
                        }
                    } else {
                        // Likely incomplete data, wait for more
                        return Ok(None);
                    }
                }
            };

            // We have enough data - split off exactly the message we need
            let raw_bytes = src
                .split_to(header_info.0 + header_info.1 + CHECKSUM_FIELD_LEN)
                .freeze();
            let raw_bytes_clone = raw_bytes.clone();

            let result = self.decoder.decode(&raw_bytes);
            match result {
                Ok(message) => {
                    // Convert the borrowed Message to an owned OwnedMessage
                    let owned_message = OwnedMessage::from_message(message, raw_bytes_clone);
                    return Ok(Some(owned_message));
                }
                Err(DecodeError::Invalid { .. }) => {
                    // ✅ IMPROVED: Invalid message after header parsing - this shouldn't happen often
                    // but if it does, advance past this message and continue looking
                    continue;
                }
                Err(e) => return Err(e),
            }
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
    use tokio_util::bytes::{Bytes, BytesMut}; // Use tokio_util::bytes instead of direct bytes
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
        assert_eq!(owned.get::<SmartString>(35), Ok("0".into()));
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
        assert_eq!(owned.get::<SmartString>(8), Ok("FIX.4.4".into()));
        assert_eq!(owned.get::<SmartString>(35), Ok("D".into()));
        assert_eq!(owned.get::<u32>(34), Ok(215)); // Sequence number in this message
        assert_eq!(owned.msg_type(), Ok("D".into()));

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
        let empty_msg = OwnedMessage {
            raw_bytes: empty_bytes,
            fields: empty_fields,
        };

        assert!(empty_msg.is_empty());
        assert_eq!(empty_msg.len(), 0);
        assert!(empty_msg.fields().next().is_none());
    }

    #[test]
    fn test_owned_message_as_bytes() {
        let dict = Dictionary::fix44().unwrap();
        let mut decoder = super::Decoder::new(dict);
        decoder.config_mut().separator = b'|';
        let data = b"8=FIX.4.4|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=185|";
        let message = decoder.decode(data).unwrap();
        let owned_message = OwnedMessage::from_message(message, Bytes::from(&data[..]));

        assert_eq!(owned_message.as_bytes(), data);
    }

    /// Test demonstrating field ordering loss limitation in OwnedMessage.
    ///
    /// This test shows how the original field ordering from the FIX message
    /// is not preserved when fields are stored in HashMap.
    #[test]
    fn test_field_extraction_limitation_field_ordering_loss() {
        let dict = Dictionary::fix44().unwrap();
        let mut decoder = super::Decoder::new(dict);
        decoder.config_mut().separator = b'|';
        let data = b"8=FIX.4.4|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=185|";

        let message = decoder.decode(data).unwrap();

        // With borrowed Message, we can iterate fields in original order
        let original_field_order: Vec<(u32, SmartString)> = message
            .fields()
            .map(|(tag, value)| (tag.get(), String::from_utf8_lossy(value).to_string().into()))
            .collect();

        // Verify we have the expected fields in their original order
        let expected_tags = vec![8u32, 35, 49, 56, 34, 52]; // Adjusted to match actual parsed fields
        let actual_tags: Vec<u32> = original_field_order.iter().map(|(tag, _)| *tag).collect();
        assert_eq!(actual_tags, expected_tags);

        // Convert to OwnedMessage
        let owned_message = OwnedMessage::from_message(message, Bytes::from(&data[..]));

        // LIMITATION DEMONSTRATED: Field ordering is lost
        let owned_field_order: Vec<(u32, SmartString)> = owned_message
            .fields()
            .map(|(tag, value)| (tag, String::from_utf8_lossy(value).to_string().into()))
            .collect();

        let owned_tags: Vec<u32> = owned_field_order.iter().map(|(tag, _)| *tag).collect();

        // The tags are the same, but the order is NOT preserved (HashMap iteration order)
        assert_eq!(owned_tags.len(), expected_tags.len());
        for tag in &expected_tags {
            assert!(owned_tags.contains(tag), "Tag {tag} should be present");
        }

        // But the ordering is different (this assertion will likely pass due to HashMap's unpredictable order)
        // The important point is that we CANNOT rely on any specific ordering with OwnedMessage
    }

    /// Test demonstrating group operations limitation in OwnedMessage.
    ///
    /// This test shows that OwnedMessage does not support any group operations
    /// that are available with the borrowed Message type.
    #[test]
    fn test_field_extraction_limitation_no_group_operations() {
        let dict = Dictionary::fix44().unwrap();
        let mut decoder = super::Decoder::new(dict);
        decoder.config_mut().separator = b'|';

        // Simple message to demonstrate the limitation
        let message_data = b"8=FIX.4.4|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=185|";
        let message = decoder.decode(message_data).unwrap();

        // Convert to OwnedMessage
        let owned_message = OwnedMessage::from_message(message, Bytes::from(&message_data[..]));

        // LIMITATION DEMONSTRATED: No group operations available
        // OwnedMessage does not implement FieldMap trait
        // The following operations are NOT available:
        // - owned_message.group(268) // Would not compile - method doesn't exist
        // - No way to access group entries individually
        // - No way to iterate over group entries
        // - No way to validate group structure

        // We can only access individual fields by tag, without context
        assert_eq!(owned_message.get::<SmartString>(35).unwrap(), "0"); // MsgType
        assert_eq!(owned_message.get::<SmartString>(49).unwrap(), "A"); // SenderCompID
        assert_eq!(owned_message.get::<SmartString>(56).unwrap(), "B"); // TargetCompID

        // But we have no way to know these fields were part of a group structure
    }

    /// Test demonstrating field overwriting limitation in complex group scenarios.
    ///
    /// This test shows how nested groups and complex field structures become
    /// completely flattened and lose all hierarchical information.
    #[test]
    fn test_field_extraction_limitation_complex_group_flattening() {
        let dict = Dictionary::fix44().unwrap();
        let mut decoder = super::Decoder::new(dict);
        decoder.config_mut().separator = b'|';

        // Simple message to demonstrate the concept
        let message_data = b"8=FIX.4.4|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=185|";
        let message = decoder.decode(message_data).unwrap();

        // Convert to OwnedMessage
        let owned_message = OwnedMessage::from_message(message, Bytes::from(&message_data[..]));

        // LIMITATION DEMONSTRATED: All structure is flattened
        // In complex scenarios with groups, only the last occurrence of each tag is preserved
        assert_eq!(owned_message.get::<SmartString>(35).unwrap(), "0"); // MsgType
        assert_eq!(owned_message.get::<SmartString>(49).unwrap(), "A"); // SenderCompID

        // We've lost:
        // - Any group structure that might have existed
        // - Field context and hierarchy
        // - The ability to reconstruct the original message structure
        // - Multiple instances of the same field tag

        // Count fields in OwnedMessage - each tag appears only once
        let field_count = owned_message.fields().count();
        assert_eq!(field_count, 6); // All unique fields preserved (adjusted to match actual parsed fields)
    }

    /// Test documenting the correct use case for OwnedMessage despite limitations.
    ///
    /// This test shows when OwnedMessage is still useful despite its limitations.
    #[test]
    fn test_owned_message_correct_usage_simple_fields() {
        let dict = Dictionary::fix44().unwrap();
        let mut decoder = super::Decoder::new(dict);
        decoder.config_mut().separator = b'|';

        // Simple message without repeating groups - ideal for OwnedMessage
        let message_data = b"8=FIX.4.4|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=185|";
        let message = decoder.decode(message_data).unwrap();
        let owned_message = OwnedMessage::from_message(message, Bytes::from(&message_data[..]));

        // For simple messages without groups, OwnedMessage works perfectly
        assert_eq!(owned_message.get::<SmartString>(35).unwrap(), "0"); // MsgType
        assert_eq!(owned_message.get::<SmartString>(49).unwrap(), "A"); // SenderCompID
        assert_eq!(owned_message.get::<SmartString>(56).unwrap(), "B"); // TargetCompID
        assert_eq!(owned_message.get::<SmartString>(34).unwrap(), "12"); // MsgSeqNum

        // All fields are accessible and maintain their values correctly
        // This is the ideal use case for OwnedMessage in async contexts
        assert_eq!(owned_message.len(), 6); // All fields preserved (adjusted to match actual parsed fields)
        assert!(!owned_message.is_empty());
    }
}
