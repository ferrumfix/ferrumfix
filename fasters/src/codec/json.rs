//! JSON encoding for FIX support.

use crate::app::slr;
use crate::app::TsrMessageRef;
use crate::codec::*;
use crate::Dictionary;
use serde_json::json;
use std::collections::HashMap;
use std::fmt;

/// Transmuter configuration for the [`Codec`] encoding.
pub trait Transmuter: Clone {
    /// This setting indicates that all encoded messages should be "prettified",
    /// i.e. the JSON code will not be compressed and instead it will have
    /// indentation and other whitespace that favors human readability. Some
    /// performance loss is expected.
    ///
    /// This is turned off be default.
    #[inline(always)]
    fn pretty_print(&self) -> bool {
        false
    }
}

/// A pretty-printer [`Transmuter`].
#[derive(Debug, Clone)]
pub struct TransPrettyPrint;

impl Transmuter for TransPrettyPrint {
    fn pretty_print(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct Codec<T> {
    dictionaries: HashMap<String, Dictionary>,
    message: T,
}

impl<T> Codec<T>
where
    T: TsrMessageRef,
{
    pub fn new(dict: Dictionary) -> Self {
        let mut dictionaries = HashMap::new();
        dictionaries.insert(dict.get_version().to_string(), dict);
        Self {
            dictionaries,
            message: T::default(),
        }
    }

    fn decode_field(
        &self,
        dictionary: &Dictionary,
        key: &str,
        value: &serde_json::Value,
    ) -> Result<(u32, slr::FixFieldValue), DecodeError> {
        if let Some(field) = dictionary.get_field_by_name(key) {
            match value {
                serde_json::Value::String(s) => Ok((
                    field.tag() as u32,
                    slr::FixFieldValue::String(s.to_string()),
                )),
                serde_json::Value::Array(values) => {
                    let mut group = Vec::new();
                    for item in values {
                        group.push(self.decode_component_block(dictionary, item)?);
                    }
                    Ok((field.tag() as u32, slr::FixFieldValue::Group(group)))
                }
                _ => Err(DecodeError::InvalidData),
            }
        } else {
            Err(DecodeError::InvalidData)
        }
    }

    fn decode_component_block(
        &self,
        dictionary: &Dictionary,
        value: &serde_json::Value,
    ) -> Result<HashMap<i64, slr::FixFieldValue>, DecodeError> {
        let mut group = HashMap::new();
        for item in value.as_object().unwrap() {
            let (tag, field) = self.decode_field(dictionary, item.0, item.1)?;
            group.insert(tag as i64, field);
        }
        Ok(group)
    }

    fn translate(&self, dict: &Dictionary, field: &slr::FixFieldValue) -> serde_json::Value {
        match field {
            slr::FixFieldValue::String(c) => serde_json::Value::String(c.to_string()),
            slr::FixFieldValue::Group(array) => {
                let mut values = Vec::new();
                for group in array {
                    let mut map = serde_json::Map::new();
                    for item in group {
                        let field = dict
                            .get_field(*item.0 as u32)
                            .ok_or(DecodeError::InvalidData)
                            .unwrap();
                        let field_name = field.name().to_string();
                        let field_value = self.translate(dict, item.1);
                        map.insert(field_name, field_value);
                    }
                    values.push(serde_json::Value::Object(map));
                }
                serde_json::Value::Array(values)
            }
            _ => panic!(),
        }
    }
}

impl<Z, T> Decoder<T> for (Codec<T>, Z)
where
    T: TsrMessageRef,
    Z: Transmuter,
{
    type Error = DecodeError;

    fn decode(&mut self, data: &[u8]) -> Result<&T, Self::Error> {
        let value: serde_json::Value =
            serde_json::from_reader(data).map_err(|_| Self::Error::Syntax)?;
        let header = value
            .get("Header")
            .and_then(|v| v.as_object())
            .ok_or(Self::Error::Schema)?;
        let body = value
            .get("Body")
            .and_then(|v| v.as_object())
            .ok_or(Self::Error::Schema)?;
        let trailer = value
            .get("Trailer")
            .and_then(|v| v.as_object())
            .ok_or(Self::Error::Schema)?;
        let _field_msg_type = header // TODO: field presence checks.
            .get("MsgType")
            .and_then(|v| v.as_str())
            .ok_or(Self::Error::Schema)?;
        let field_begin_string = header
            .get("BeginString")
            .and_then(|v| v.as_str())
            .ok_or(Self::Error::Schema)?;
        let dictionary = self
            .0
            .dictionaries
            .get(field_begin_string)
            .ok_or(Self::Error::InvalidMsgType)?;
        let mut message = T::default();
        for item in header.iter().chain(body).chain(trailer) {
            let (tag, field) = self.0.decode_field(dictionary, item.0, item.1)?;
            message.set_field(tag, field);
        }
        self.0.message = message;
        Ok(&self.0.message)
    }
}

impl<Z> Encoder<slr::Message> for (Codec<slr::Message>, Z)
where
    Z: Transmuter,
{
    type Error = EncoderError;

    fn encode(
        &mut self,
        buffer: impl Buffer,
        message: &slr::Message,
    ) -> Result<usize, Self::Error> {
        let dictionary =
            if let Some(slr::FixFieldValue::String(fix_version)) = message.fields.get(&8) {
                self.0
                    .dictionaries
                    .get(fix_version.as_str())
                    .ok_or(Self::Error::Dictionary)?
            } else {
                return Err(Self::Error::Dictionary);
            };
        let component_std_header = dictionary.get_component("StandardHeader").unwrap();
        let component_std_traler = dictionary.get_component("StandardTrailer").unwrap();
        let msg_type = if let Some(slr::FixFieldValue::String(s)) = message.get_field(35) {
            s
        } else {
            return Err(Self::Error::Dictionary);
        };
        let mut map_body = json!({});
        let mut map_trailer = json!({});
        let mut map_header = json!({ "MsgType": msg_type });
        for (field_tag, field_value) in message.fields.iter() {
            let field = dictionary
                .get_field(*field_tag as u32)
                .ok_or(Self::Error::Dictionary)?;
            let field_name = field.name().to_string();
            let field_value = self.0.translate(dictionary, field_value);
            if component_std_header.contains_field(&field) {
                map_header
                    .as_object_mut()
                    .unwrap()
                    .insert(field_name, field_value);
            } else if component_std_traler.contains_field(&field) {
                map_trailer
                    .as_object_mut()
                    .unwrap()
                    .insert(field_name, field_value);
            } else {
                map_body
                    .as_object_mut()
                    .unwrap()
                    .insert(field_name, field_value);
            }
        }
        let value = json!({
            "Header": map_header,
            "Body": map_body,
            "Trailer": map_trailer,
        });
        let mut writer = BufferWriter::new(buffer);
        if self.1.pretty_print() {
            serde_json::to_writer_pretty(&mut writer, &value).unwrap();
        } else {
            serde_json::to_writer(&mut writer, &value).unwrap();
        }
        Ok(writer.as_slice().len())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum EncoderError {
    Dictionary,
}

#[derive(Copy, Clone, Debug)]
pub enum DecodeError {
    Syntax,
    Schema,
    InvalidMsgType,
    InvalidData,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FIX JSON decoding error.")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::*;

    const MESSAGE_SIMPLE: &str = r#"
{
    "Header": {
        "BeginString": "FIX.4.4",
        "MsgType": "W",
        "MsgSeqNum": "4567",
        "SenderCompID": "SENDER",
        "TargetCompID": "TARGET",
        "SendingTime": "20160802-21:14:38.717"
    },
    "Body": {
        "SecurityIDSource": "8",
        "SecurityID": "ESU6",
        "MDReqID": "789",
        "NoMDEntries": [
            { "MDEntryType": "0", "MDEntryPx": "1.50", "MDEntrySize": "75", "MDEntryTime": "21:14:38.688" },
            { "MDEntryType": "1", "MDEntryPx": "1.75", "MDEntrySize": "25", "MDEntryTime": "21:14:38.688" }
        ]
    },
    "Trailer": {
    }
}
    "#;

    const MESSAGE_WITHOUT_HEADER: &str = r#"
{
    "Body": {
        "SecurityIDSource": "8",
        "SecurityID": "ESU6",
        "MDReqID": "789",
        "NoMDEntries": [
            { "MDEntryType": "0", "MDEntryPx": "1.50", "MDEntrySize": "75", "MDEntryTime": "21:14:38.688" },
            { "MDEntryType": "1", "MDEntryPx": "1.75", "MDEntrySize": "25", "MDEntryTime": "21:14:38.688" }
        ]
    },
    "Trailer": {
    }
}
    "#;

    fn dict_fix44() -> Dictionary {
        Dictionary::from_version(crate::app::Version::Fix44)
    }

    fn encoder_fix44() -> (Codec<slr::Message>, impl Transmuter) {
        (Codec::new(dict_fix44()), TransPrettyPrint)
    }

    #[test]
    fn decode_then_decode() {
        let mut decoder = encoder_fix44();
        let mut encoder = encoder_fix44();
        let json_value_before: Value = from_str(MESSAGE_SIMPLE).unwrap();
        let message = Decoder::decode(&mut decoder, &mut MESSAGE_SIMPLE.as_bytes()).unwrap();
        let mut buffer = Vec::<u8>::new();
        Encoder::encode(&mut encoder, &mut buffer, message).unwrap();
        let json_value_after: Value = from_slice(&buffer[..]).unwrap();
        assert_eq!(json_value_before, json_value_after);
    }

    #[test]
    fn message_without_header() {
        let mut encoder = encoder_fix44();
        let result = Decoder::decode(&mut encoder, &mut MESSAGE_WITHOUT_HEADER.as_bytes());
        match result {
            Err(DecodeError::Schema) => (),
            _ => panic!(),
        };
    }

    #[test]
    fn invalid_json() {
        let mut encoder = encoder_fix44();
        let result = Decoder::decode(&mut encoder, &mut "this is invalid JSON".as_bytes());
        match result {
            Err(DecodeError::Syntax) => (),
            _ => panic!(),
        };
    }
}
