use crate::app::slr;
use crate::presentation::Encoding;
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
        let msg_type = header
            .get("MsgType")
            .and_then(|v| v.as_str())
            .ok_or(DecodeError::Schema)?;
        let dictionary = self
            .dictionaries
            .get(msg_type)
            .ok_or(DecodeError::InvalidMsgType)?;
        let mut message = slr::Message::new();
        for item in header.iter().chain(body).chain(trailer) {
            let field = dictionary.get_field_by_name(item.0);
            if let Some(field) = field {
                message.add_field(
                    field.tag(),
                    slr::FixFieldValue::String(item.1.as_str().unwrap().to_string()),
                );
            }
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
            let field_value = serde_json::Value::from(field_value);
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
            serde_json::to_vec(&value).unwrap()
        } else {
            serde_json::to_vec_pretty(&value).unwrap()
        };
        Ok(bytes)
    }
}

impl From<&slr::FixFieldValue> for serde_json::Value {
    fn from(field: &slr::FixFieldValue) -> Self {
        let string = match field {
            slr::FixFieldValue::Char(c) => c.to_string(),
            _ => panic!(),
        };
        serde_json::Value::String(string)
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
