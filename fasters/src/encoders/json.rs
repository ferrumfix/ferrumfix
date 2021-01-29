use super::TransmuterPattern;
use crate::app::slr;
use crate::encoders::Codec;
use crate::encoders::Encoding;
use crate::encoders::Poll;
use crate::utils::GrowableVecU8;
use crate::Dictionary;
use serde_json::json;
use std::collections::HashMap;
use std::fmt;
use std::io;

/// Transmuter configuration for the [`Json`] encoding.
pub trait Transmuter: Clone {
    /// This setting indicates that all encoded messages should be "prettified",
    /// i.e. the JSON code will not be compressed and instead it will have
    /// indentation and other whitespace that favors human readability. Some
    /// performance loss is expected.
    ///
    /// This is turned off be default.
    const PRETTY_PRINT: bool = false;
}

/// A pretty-printer transmuter for [`Json`](Json) encoding.
#[derive(Clone)]
pub struct TransPrettyPrint;

impl Transmuter for TransPrettyPrint {
    const PRETTY_PRINT: bool = true;
}

impl<T> TransmuterPattern for T where T: Transmuter {}

pub struct Json {
    dictionaries: HashMap<String, Dictionary>,
    buffer: Vec<u8>,
    cursor: usize,
    message: slr::Message,
}

impl Json {
    pub fn new(dict: Dictionary) -> Self {
        let mut dictionaries = HashMap::new();
        dictionaries.insert(dict.get_version().to_string(), dict);
        Json {
            dictionaries,
            buffer: Vec::with_capacity(1024),
            cursor: 0,
            message: slr::Message::new(),
        }
    }

    fn decode_field(
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
                        group.push(Self::decode_component_block(dictionary, item)?);
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
        dictionary: &Dictionary,
        value: &serde_json::Value,
    ) -> Result<HashMap<i64, slr::FixFieldValue>, DecodeError> {
        let mut group = HashMap::new();
        for item in value.as_object().unwrap() {
            let (tag, field) = Self::decode_field(dictionary, item.0, item.1)?;
            group.insert(tag as i64, field);
        }
        Ok(group)
    }

    fn translate(dict: &Dictionary, field: &slr::FixFieldValue) -> serde_json::Value {
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
                        let field_value = Self::translate(dict, item.1);
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

impl<'s, Z> Codec<'s, &'s slr::Message> for (Json, Z)
where
    Z: Transmuter + 's,
{
    type EncodeError = DecodeError;
    type DecodeError = DecodeError;

    fn decode(&mut self, data: &[u8]) -> Result<Poll, Self::DecodeError> {
        self.0.buffer.extend_from_slice(data);
        let value: serde_json::Value =
            serde_json::from_reader(&self.0.buffer[..]).map_err(|_| DecodeError::Syntax)?;
        let header = value
            .get("Header")
            .and_then(|v| v.as_object())
            .ok_or(DecodeError::Schema)?;
        let body = value
            .get("Body")
            .and_then(|v| v.as_object())
            .ok_or(DecodeError::Schema)?;
        let trailer = value
            .get("Trailer")
            .and_then(|v| v.as_object())
            .ok_or(DecodeError::Schema)?;
        let _field_msg_type = header // TODO: field presence checks.
            .get("MsgType")
            .and_then(|v| v.as_str())
            .ok_or(DecodeError::Schema)?;
        let field_begin_string = header
            .get("BeginString")
            .and_then(|v| v.as_str())
            .ok_or(DecodeError::Schema)?;
        let dictionary = self
            .0
            .dictionaries
            .get(field_begin_string)
            .ok_or(DecodeError::InvalidMsgType)?;
        let mut message = slr::Message::new();
        for item in header.iter().chain(body).chain(trailer) {
            let (tag, field) = Json::decode_field(dictionary, item.0, item.1)?;
            message.add_field(tag, field);
        }
        self.0.message = message;
        Ok(Poll::Ready)
    }

    fn get_item(&self) -> &slr::Message {
        &self.0.message
    }

    fn encode(&mut self, data: &slr::Message) -> Result<&[u8], Self::EncodeError> {
        let dictionary = if let Some(slr::FixFieldValue::String(fix_version)) = data.fields.get(&8)
        {
            self.0
                .dictionaries
                .get(fix_version.as_str())
                .ok_or(DecodeError::Schema)?
        } else {
            return Err(DecodeError::Schema);
        };
        let component_std_header = dictionary.get_component("StandardHeader").unwrap();
        let component_std_traler = dictionary.get_component("StandardTrailer").unwrap();
        let msg_type = if let Some(slr::FixFieldValue::String(s)) = data.fields.get(&35) {
            s
        } else {
            return Err(DecodeError::InvalidData);
        };
        let mut map_body = json!({});
        let mut map_trailer = json!({});
        let mut map_header = json!({ "MsgType": msg_type });
        for (field_tag, field_value) in data.fields.iter() {
            let field = dictionary
                .get_field(*field_tag as u32)
                .ok_or(DecodeError::InvalidData)?;
            let field_name = field.name().to_string();
            let field_value = Json::translate(dictionary, field_value);
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
        let mut writer = GrowableVecU8 {
            bytes: &mut self.0.buffer,
        };
        if Z::PRETTY_PRINT {
            serde_json::to_writer_pretty(&mut writer, &value).unwrap()
        } else {
            serde_json::to_writer(&mut writer, &value).unwrap()
        };
        Ok(&self.0.buffer[..])
    }
}

impl<Z> Encoding<slr::Message> for (Json, Z)
where
    Z: Transmuter,
{
    type EncodeError = DecodeError;
    type DecodeError = DecodeError;

    fn decode(&self, source: &mut impl io::BufRead) -> Result<slr::Message, Self::DecodeError> {
        let value: serde_json::Value =
            serde_json::from_reader(source).map_err(|_| DecodeError::Syntax)?;
        let header = value
            .get("Header")
            .and_then(|v| v.as_object())
            .ok_or(DecodeError::Schema)?;
        let body = value
            .get("Body")
            .and_then(|v| v.as_object())
            .ok_or(DecodeError::Schema)?;
        let trailer = value
            .get("Trailer")
            .and_then(|v| v.as_object())
            .ok_or(DecodeError::Schema)?;
        let _field_msg_type = header // TODO: field presence checks.
            .get("MsgType")
            .and_then(|v| v.as_str())
            .ok_or(DecodeError::Schema)?;
        let field_begin_string = header
            .get("BeginString")
            .and_then(|v| v.as_str())
            .ok_or(DecodeError::Schema)?;
        let dictionary = self
            .0
            .dictionaries
            .get(field_begin_string)
            .ok_or(DecodeError::InvalidMsgType)?;
        let mut message = slr::Message::new();
        for item in header.iter().chain(body).chain(trailer) {
            let (tag, field) = Json::decode_field(dictionary, item.0, item.1)?;
            message.add_field(tag, field);
        }
        Ok(message)
    }

    fn encode(&mut self, message: slr::Message) -> Result<Vec<u8>, Self::EncodeError> {
        debug_assert!(self.0.buffer.is_empty());
        Codec::encode(self, &message).map(|data| data.to_vec())
    }
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

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
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

    fn encoder_fix44() -> (Json, impl Transmuter) {
        (Json::new(dict_fix44()), TransPrettyPrint)
    }

    #[test]
    fn decode_then_decode() {
        let mut encoder = encoder_fix44();
        let json_value_before: Value = from_str(MESSAGE_SIMPLE).unwrap();
        let message = encoder.decode(&mut MESSAGE_SIMPLE.as_bytes()).unwrap();
        let bytes = Codec::encode(&mut encoder, &message).unwrap();
        let json_value_after: Value = from_slice(&bytes[..]).unwrap();
        assert_eq!(json_value_before, json_value_after);
    }

    #[test]
    fn message_without_header() {
        let encoder = encoder_fix44();
        let result = encoder.decode(&mut MESSAGE_WITHOUT_HEADER.as_bytes());
        match result {
            Err(DecodeError::Schema) => (),
            _ => panic!(),
        };
    }

    #[test]
    fn invalid_json() {
        let encoder = encoder_fix44();
        let result = encoder.decode(&mut "this is invalid JSON".as_bytes());
        match result {
            Err(DecodeError::Syntax) => (),
            _ => panic!(),
        };
    }
}
