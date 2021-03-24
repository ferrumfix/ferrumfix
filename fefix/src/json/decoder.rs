use super::{Config, Configure, DecodeError};
use crate::tagvalue::FixFieldValue;
use crate::Dictionary;
use crate::FixMessage;
use std::collections::{BTreeMap, HashMap};

/// A codec for the JSON encoding type.
#[derive(Debug, Clone)]
pub struct Decoder<C = Config> {
    dictionaries: HashMap<String, Dictionary>,
    message: FixMessage,
    config: C,
}

impl<C> Decoder<C>
where
    C: Configure,
{
    /// Creates a new codec. `dict` serves as a reference for data type inference
    /// of incoming messages' fields. `config` handles encoding details. See the
    /// [`Configure`] trait for more information.
    pub fn new(dict: Dictionary) -> Self {
        Self::with_config(dict, C::default())
    }

    /// Creates a new codec. `dict` serves as a reference for data type inference
    /// of incoming messages' fields. `config` handles encoding details. See the
    /// [`Configure`] trait for more information.
    pub fn with_config(dict: Dictionary, config: C) -> Self {
        let mut dictionaries = HashMap::new();
        dictionaries.insert(dict.get_version().to_string(), dict);
        Self {
            dictionaries,
            message: FixMessage::new(),
            config,
        }
    }

    /// Returns an immutable reference to the [`Configure`] implementor used by
    /// `self`.
    pub fn config(&self) -> &C {
        &self.config
    }

    /// Returns a mutable reference to the [`Configure`] implementor used by
    /// `self`.
    pub fn config_mut(&mut self) -> &mut C {
        &mut self.config
    }

    pub fn decode(&mut self, data: &[u8]) -> Result<&FixMessage, DecodeError> {
        let value: serde_json::Value =
            serde_json::from_reader(data).map_err(|_| DecodeError::Syntax)?;
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
        let begin_string = header
            .get("BeginString")
            .and_then(|v| v.as_str())
            .ok_or(DecodeError::Schema)?;
        let dictionary = self
            .dictionaries
            .get(begin_string)
            .ok_or(DecodeError::InvalidMsgType)?;
        let message = &mut self.message;
        message.clear();
        let mut decode_field = |name: &str, value: &serde_json::Value| {
            decode_field(dictionary, name, value).map(|(tag, field)| message.add_field(tag, field))
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
        Ok(&self.message)
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
            _ => Err(DecodeError::Schema),
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::json::ConfigPrettyPrint;
    use crate::AppVersion;

    //    const MESSAGE_SIMPLE: &str = r#"
    //{
    //    "Header": {
    //        "BeginString": "FIX.4.4",
    //        "MsgType": "W",
    //        "MsgSeqNum": "4567",
    //        "SenderCompID": "SENDER",
    //        "TargetCompID": "TARGET",
    //        "SendingTime": "20160802-21:14:38.717"
    //    },
    //    "Body": {
    //        "SecurityIDSource": "8",
    //        "SecurityID": "ESU6",
    //        "MDReqID": "789",
    //        "NoMDEntries": [
    //            { "MDEntryType": "0", "MDEntryPx": "1.50", "MDEntrySize": "75", "MDEntryTime": "21:14:38.688" },
    //            { "MDEntryType": "1", "MDEntryPx": "1.75", "MDEntrySize": "25", "MDEntryTime": "21:14:38.688" }
    //        ]
    //    },
    //    "Trailer": {
    //    }
    //}
    //    "#;

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

    fn encoder_fix44() -> Decoder<impl Configure> {
        Decoder::with_config(dict_fix44(), ConfigPrettyPrint)
    }

    //#[test]
    //fn decode_then_decode() {
    //    let mut decoder = encoder_fix44();
    //    let mut encoder = encoder_fix44();
    //    let json_value_before: Value = from_str(MESSAGE_SIMPLE).unwrap();
    //    let message = decoder.decode(&mut MESSAGE_SIMPLE.as_bytes()).unwrap();
    //    let mut buffer = Vec::<u8>::new();
    //    encoder.encode(&mut buffer, message.to_message_seq()).unwrap();
    //    let json_value_after: Value = from_slice(&buffer[..]).unwrap();
    //    assert_eq!(json_value_before, json_value_after);
    //}

    #[test]
    fn message_without_header() {
        let mut encoder = encoder_fix44();
        let result = encoder.decode(&mut MESSAGE_WITHOUT_HEADER.as_bytes());
        match result {
            Err(DecodeError::Schema) => (),
            _ => panic!(),
        };
    }

    #[test]
    fn invalid_json() {
        let mut encoder = encoder_fix44();
        let result = encoder.decode(&mut "this is invalid JSON".as_bytes());
        match result {
            Err(DecodeError::Syntax) => (),
            _ => panic!(),
        };
    }
}
