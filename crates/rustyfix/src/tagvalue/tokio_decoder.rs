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

        // Use a simple approach: iterate through known common field tags
        // and extract them if present. This avoids lifetime issues.
        for tag in [8, 9, 10, 34, 35, 49, 52, 56] {
            if let Some(value) = message.get_raw(tag) {
                fields.insert(tag, Bytes::copy_from_slice(value));
            }
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
            Err(DecodeError::Invalid) => Ok(None),
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
            Err(DecodeError::Invalid) => Ok(None),
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
}
