use crate::app::slr;
use crate::encoders::Encoding;
use crate::Dictionary;
use serde_json::json;
use std::collections::HashMap;
use std::fmt;
use std::io;

pub struct Json {
    dictionaries: HashMap<String, Dictionary>,
    pretty_print: bool,
}

impl Json {
    pub fn new(dict: Dictionary, pretty: bool) -> Self {
        let mut dictionaries = HashMap::new();
        dictionaries.insert(dict.get_version().to_string(), dict);
        Json {
            dictionaries,
            pretty_print: pretty,
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

impl Encoding<slr::Message> for Json {
    type EncodeErr = DecodeError;
    type DecodeErr = DecodeError;

    fn decode(&self, source: &mut impl io::BufRead) -> Result<slr::Message, Self::DecodeErr> {
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
            .dictionaries
            .get(field_begin_string)
            .ok_or(DecodeError::InvalidMsgType)?;
        let mut message = slr::Message::new();
        for item in header.iter().chain(body).chain(trailer) {
            let (tag, field) = Self::decode_field(dictionary, item.0, item.1)?;
            message.add_field(tag, field);
        }
        Ok(message)
    }

    fn encode(&self, message: slr::Message) -> Result<Vec<u8>, Self::EncodeErr> {
        let dictionary =
            if let Some(slr::FixFieldValue::String(fix_version)) = message.fields.get(&8) {
                self.dictionaries
                    .get(fix_version.as_str())
                    .ok_or(DecodeError::Schema)?
            } else {
                return Err(DecodeError::Schema);
            };
        let component_std_header = dictionary.get_component("StandardHeader").unwrap();
        let component_std_traler = dictionary.get_component("StandardTrailer").unwrap();
        let msg_type = if let Some(slr::FixFieldValue::String(s)) = message.fields.get(&35) {
            s
        } else {
            return Err(DecodeError::InvalidData);
        };
        let mut map_body = json!({});
        let mut map_trailer = json!({});
        let mut map_header = json!({ "MsgType": msg_type });
        for (field_tag, field_value) in message.fields.iter() {
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
        let bytes = if self.pretty_print {
            serde_json::to_vec_pretty(&value).unwrap()
        } else {
            serde_json::to_vec(&value).unwrap()
        };
        Ok(bytes)
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

    fn encoder_fix44() -> Json {
        Json::new(dict_fix44(), true)
    }

    #[test]
    fn decode_then_decode() {
        let encoder = encoder_fix44();
        let json_value_before: Value = from_str(MESSAGE_SIMPLE).unwrap();
        let message = encoder.decode(&mut MESSAGE_SIMPLE.as_bytes()).unwrap();
        let bytes = encoder.encode(message).unwrap();
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
