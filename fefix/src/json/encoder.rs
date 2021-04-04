use super::{Config, Configure, DecodeError, EncodeError};
use crate::buffer::Buffer;
use crate::models::{field_value::FieldValue, FixFieldValue};
use crate::Dictionary;
use crate::FixFieldsIter;
use crate::FixMessage;
use serde_json::json;
use std::collections::HashMap;

/// A codec for the JSON encoding type.
#[derive(Debug, Clone)]
pub struct Encoder<C = Config> {
    dictionaries: HashMap<String, Dictionary>,
    message: FixMessage,
    config: C,
}

impl<C> Encoder<C>
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

    pub fn encode<B>(
        &mut self,
        mut buffer: &mut B,
        message: &FixMessage,
    ) -> Result<usize, EncodeError>
    where
        B: Buffer,
    {
        let dictionary =
            if let Some(FixFieldValue::Atom(FieldValue::String(fix_version))) = message.field(8) {
                self.dictionaries
                    .get(fix_version.as_str())
                    .ok_or(EncodeError::Dictionary)?
            } else {
                return Err(EncodeError::Dictionary);
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
        for (field_tag, field_value) in message.iter_fields_in_std_header() {
            let field = dictionary
                .field_by_tag(field_tag)
                .ok_or(EncodeError::Dictionary)?;
            let field_name = field.name().to_string();
            debug_assert!(component_std_header.contains_field(&field));
            let field_value = self.translate(dictionary, field_value);
            map_header
                .as_object_mut()
                .unwrap()
                .insert(field_name, field_value);
        }
        for (field_tag, field_value) in message.iter_fields_in_std_header() {
            let field = dictionary
                .field_by_tag(field_tag)
                .ok_or(EncodeError::Dictionary)?;
            let field_name = field.name().to_string();
            let field_value = self.translate(dictionary, field_value);
            map_body
                .as_object_mut()
                .unwrap()
                .insert(field_name, field_value);
        }
        for (field_tag, field_value) in message.iter_fields_in_std_header() {
            let field = dictionary
                .field_by_tag(field_tag)
                .ok_or(EncodeError::Dictionary)?;
            debug_assert!(component_std_traler.contains_field(&field));
            let field_name = field.name().to_string();
            let field_value = self.translate(dictionary, field_value);
            map_trailer
                .as_object_mut()
                .unwrap()
                .insert(field_name, field_value);
        }
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

#[cfg(test)]
mod test {}
