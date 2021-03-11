use crate::backend::{Backend, FieldValue};
use crate::tagvalue::{FixFieldValue};
use crate::buffering::Buffer;
use crate::json::{Config, Configurable};
use crate::{Dictionary, Encoding};
use serde_json::json;
use std::collections::{BTreeMap, HashMap};
use std::fmt;

/// A codec for the JSON encoding type.
#[derive(Debug, Clone)]
pub struct Codec<T, Z = Configurable> {
    dictionaries: HashMap<String, Dictionary>,
    message: T,
    config: Z,
}

impl<T, Z> Codec<T, Z>
where
    T: Default,
    Z: Config,
{
    /// Creates a new codec. `dict` serves as a reference for data type inference
    /// of incoming messages' fields. `config` handles encoding details. See the
    /// [`Config`](Config) trait for more information.
    pub fn new(dict: Dictionary, config: Z) -> Self {
        let mut dictionaries = HashMap::new();
        dictionaries.insert(dict.get_version().to_string(), dict);
        Self {
            dictionaries,
            message: T::default(),
            config,
        }
    }

    fn translate(&self, dict: &Dictionary, field: &FixFieldValue) -> serde_json::Value {
        match field {
            FixFieldValue::Atom(FieldValue::String(c)) => {
                serde_json::Value::String(c.as_str().to_string())
            }
            FixFieldValue::Group(array) => {
                let mut values = Vec::new();
                for group in array {
                    let mut map = serde_json::Map::new();
                    for item in group {
                        let field = dict
                            .field_by_tag(*item.0 as u32)
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

impl<Z, T> Encoding<T> for Codec<T, Z>
where
    T: Backend + Default,
    Z: Config,
{
    type DecodeError = DecodeError;
    type EncodeError = EncoderError;

    fn decode(&mut self, data: &[u8]) -> Result<&T, Self::DecodeError> {
        let value: serde_json::Value =
            serde_json::from_reader(data).map_err(|_| Self::DecodeError::Syntax)?;
        let header = value
            .get("Header")
            .and_then(|v| v.as_object())
            .ok_or(Self::DecodeError::Schema)?;
        let body = value
            .get("Body")
            .and_then(|v| v.as_object())
            .ok_or(Self::DecodeError::Schema)?;
        let trailer = value
            .get("Trailer")
            .and_then(|v| v.as_object())
            .ok_or(Self::DecodeError::Schema)?;
        let begin_string = header
            .get("BeginString")
            .and_then(|v| v.as_str())
            .ok_or(Self::DecodeError::Schema)?;
        let dictionary = self
            .dictionaries
            .get(begin_string)
            .ok_or(Self::DecodeError::InvalidMsgType)?;
        let mut message = T::default();
        let mut decode_field = |name: &str, value: &serde_json::Value| {
            decode_field(dictionary, name, value).map(|(tag, field)| message.insert(tag, field))
        };
        for (key, value) in header.iter() {
            decode_field(key, value)?.unwrap();
        }
        for (key, value) in body.iter() {
            decode_field(key, value)?.unwrap();
        }
        for (key, value) in trailer.iter() {
            decode_field(key, value)?.unwrap();
        }
        self.message = message;
        Ok(&self.message)
    }

    fn encode<B>(&mut self, mut buffer: &mut B, message: &T) -> Result<usize, Self::EncodeError>
    where
        B: Buffer,
    {
        let dictionary =
            if let Some(FixFieldValue::Atom(FieldValue::String(fix_version))) = message.field(8) {
                self.dictionaries
                    .get(fix_version.as_str())
                    .ok_or(Self::EncodeError::Dictionary)?
            } else {
                return Err(Self::EncodeError::Dictionary);
            };
        let component_std_header = dictionary
            .component_by_name("StandardHeader")
            .expect("The `StandardHeader` component is mandatory.");
        let component_std_traler = dictionary
            .component_by_name("StandardTrailer")
            .expect("The `StandardTrailer` component is mandatory.");
        let mut map_header = json!({});
        let mut map_body = json!({});
        let mut map_trailer = json!({});
        message.for_each::<Self::EncodeError, _>(|field_tag, field_value| {
            let field = dictionary
                .field_by_tag(field_tag)
                .ok_or(Self::EncodeError::Dictionary)?;
            let field_name = field.name().to_string();
            let field_value = self.translate(dictionary, field_value);
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
            Ok(())
        })?;
        let value = json!({
            "Header": map_header,
            "Body": map_body,
            "Trailer": map_trailer,
        });
        if self.config.pretty_print() {
            serde_json::to_writer_pretty(&mut buffer, &value).unwrap();
        } else {
            serde_json::to_writer(&mut buffer, &value).unwrap();
        }
        Ok(buffer.as_slice().len())
    }
}

fn decode_field(
    dictionary: &Dictionary,
    key: &str,
    value: &serde_json::Value,
) -> Result<(u32, FixFieldValue), DecodeError> {
    if let Some(field) = dictionary.field_by_name(key) {
        match value {
            serde_json::Value::String(s) => Ok((
                field.tag() as u32,
                FixFieldValue::string(s.as_bytes()).unwrap(),
            )),
            serde_json::Value::Array(values) => {
                let mut group = Vec::new();
                for item in values {
                    group.push(decode_component_block(dictionary, item)?);
                }
                Ok((field.tag() as u32, FixFieldValue::Group(group)))
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
) -> Result<BTreeMap<i64, FixFieldValue>, DecodeError> {
    let mut group = BTreeMap::new();
    for item in value.as_object().unwrap() {
        let (tag, field) = decode_field(dictionary, item.0, item.1)?;
        group.insert(tag as i64, field);
    }
    Ok(group)
}

/// The type returned in the event of an error when encoding a FIX JSON message.
#[derive(Copy, Clone, Debug)]
pub enum EncoderError {
    /// The type returned in case there is an inconsistency between
    /// `BeginString`, `MsgType`, fields presence and other encoding rules as
    /// establised by the dictionary.
    Dictionary,
}

/// The type returned in the event of an error when decoding a FIX JSON message.
#[derive(Copy, Clone, Debug)]
pub enum DecodeError {
    /// Bad JSON syntax.
    Syntax,
    /// The message is valid JSON, but not a valid FIX message.
    Schema,
    /// Unrecognized message type.
    InvalidMsgType,
    /// The data does not conform to the specified message type.
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
    use crate::json::ConfigPrettyPrint;
    use crate::tagvalue::slr;
    use crate::AppVersion;
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
        Dictionary::from_version(AppVersion::Fix44)
    }

    fn encoder_fix44() -> Codec<slr::Message, impl Config> {
        Codec::new(dict_fix44(), ConfigPrettyPrint)
    }

    #[test]
    fn decode_then_decode() {
        let mut decoder = encoder_fix44();
        let mut encoder = encoder_fix44();
        let json_value_before: Value = from_str(MESSAGE_SIMPLE).unwrap();
        let message = Encoding::decode(&mut decoder, &mut MESSAGE_SIMPLE.as_bytes()).unwrap();
        let mut buffer = Vec::<u8>::new();
        Encoding::encode(&mut encoder, &mut buffer, &message).unwrap();
        let json_value_after: Value = from_slice(&buffer[..]).unwrap();
        assert_eq!(json_value_before, json_value_after);
    }

    #[test]
    fn message_without_header() {
        let mut encoder = encoder_fix44();
        let result = Encoding::decode(&mut encoder, &mut MESSAGE_WITHOUT_HEADER.as_bytes());
        match result {
            Err(DecodeError::Schema) => (),
            _ => panic!(),
        };
    }

    #[test]
    fn invalid_json() {
        let mut encoder = encoder_fix44();
        let result = Encoding::decode(&mut encoder, &mut "this is invalid JSON".as_bytes());
        match result {
            Err(DecodeError::Syntax) => (),
            _ => panic!(),
        };
    }
}
