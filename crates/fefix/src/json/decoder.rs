use super::{Config, DecodeError};
use crate::dict::{FieldLocation, IsFieldDefinition};
use crate::{FieldMap, FieldType, FieldValueError, GetConfig, RepeatingGroup};
use fefix_dictionary::Dictionary;
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MessageRef<'a> {
    internal: &'a MessageInternal<'a>,
    group_map: Option<&'a Fields<'a>>,
}

/// A read-only JSON FIX message as parsed by [`Decoder`].
#[derive(Debug, Clone)]
pub struct Message<'a> {
    internal: MessageInternal<'a>,
    group_map: Option<&'a Fields<'a>>,
}

impl<'a> Message<'a> {
    /// Creates an [`Iterator`] over all FIX fields in `self`.
    pub fn iter_fields(&'a self) -> MessageFieldsIter<'a> {
        MessageFieldsIter {
            fields: self.internal.std_header.iter(),
        }
    }

    fn field_map<F>(&'a self, field: &F) -> &'a Fields<'a>
    where
        F: IsFieldDefinition,
    {
        if let Some(context) = self.group_map {
            context
        } else {
            match field.location() {
                FieldLocation::Body => &self.internal.body,
                FieldLocation::Header => &self.internal.std_header,
                FieldLocation::Trailer => &self.internal.std_trailer,
            }
        }
    }
}

impl<'a, F> FieldMap<'a, &F> for Message<'a>
where
    F: IsFieldDefinition,
{
    type Group = MessageGroup<'a>;

    fn group(
        &'a self,
        field: &F,
    ) -> Result<Self::Group, FieldValueError<<usize as FieldType>::Error>> {
        self.field_map(field)
            .get(field.name())
            .ok_or(FieldValueError::Missing)
            .and_then(|field_or_group| {
                if let FieldOrGroup::Group(ref entries) = field_or_group {
                    Ok(MessageGroup {
                        message: MessageRef {
                            internal: &self.internal,
                            group_map: None,
                        },
                        entries,
                    })
                } else {
                    Err(FieldValueError::Missing)
                }
            })
    }

    fn get_raw(&self, field: &F) -> Option<&[u8]> {
        self.field_map(field)
            .get(field.name())
            .and_then(|field_or_group| {
                if let FieldOrGroup::Field(value) = field_or_group {
                    let s: &str = value.borrow();
                    Some(s.as_bytes())
                } else {
                    None
                }
            })
    }
}

/// A repeating group within a [`Message`].
#[derive(Debug, Clone)]
pub struct MessageGroup<'a> {
    message: MessageRef<'a>,
    entries: &'a [Fields<'a>],
}

impl<'a> RepeatingGroup<'a> for MessageGroup<'a> {
    type Entry = MessageRef<'a>;

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn get(&self, i: usize) -> Option<Self::Entry> {
        self.entries.get(i).map(|context| MessageRef {
            internal: self.message.internal,
            group_map: Some(context),
        })
    }
}

#[derive(Debug)]
pub struct MessageFieldsIter<'a> {
    fields: std::collections::hash_map::Iter<'a, Cow<'a, str>, FieldOrGroup<'a>>,
}

impl<'a> Iterator for MessageFieldsIter<'a> {
    type Item = (&'a str, &'a FieldOrGroup<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.fields.next().map(|x| (x.0.borrow(), x.1))
    }
}

/// A codec for the JSON encoding type.
#[derive(Debug, Clone)]
pub struct Decoder {
    dictionaries: HashMap<String, Arc<Dictionary>>,
    message_builder: MessageInternal<'static>,
    config: Config,
}

impl Decoder {
    /// Creates a new JSON [`Decoder`]. `dict` serves as a reference for data type inference
    /// of incoming messages' fields. Configuration options are initialized via [`Default`].
    pub fn new(dict: Dictionary) -> Self {
        let mut dictionaries = HashMap::new();
        dictionaries.insert(dict.version().to_string(), Arc::new(dict));
        Self {
            dictionaries,
            message_builder: MessageInternal::default(),
            config: Config::default(),
        }
    }

    pub fn decode<'a>(&'a mut self, data: &'a [u8]) -> Result<Message<'a>, DecodeError> {
        let mut deserilizer = serde_json::Deserializer::from_slice(data);
        let mut msg = MessageInternal::default();
        MessageInternal::deserialize_in_place(&mut deserilizer, &mut msg).map_err(|err| {
            if err.is_syntax() || err.is_eof() || err.is_io() {
                DecodeError::Syntax
            } else {
                DecodeError::Schema
            }
        })?;
        Ok(Message {
            internal: msg,
            group_map: None,
        })
    }
}

impl GetConfig for Decoder {
    type Config = Config;

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn config_mut(&mut self) -> &mut Self::Config {
        &mut self.config
    }
}

type Fields<'a> = HashMap<Cow<'a, str>, FieldOrGroup<'a>>;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum FieldOrGroup<'a> {
    Field(Cow<'a, str>),
    #[serde(borrow)]
    Group(Vec<Fields<'a>>),
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
struct MessageInternal<'a> {
    #[serde(borrow, rename = "Header")]
    std_header: Fields<'a>,
    #[serde(borrow, rename = "Body")]
    body: Fields<'a>,
    #[serde(borrow, rename = "Trailer")]
    std_trailer: Fields<'a>,
}

impl<'a> std::ops::Drop for MessageInternal<'a> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<'a> MessageInternal<'a> {
    fn clear(&mut self) {
        self.std_header.clear();
        self.body.clear();
        self.std_trailer.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const MESSAGE_SIMPLE: &str = include_str!("test_data/message_simple.json");
    const MESSAGE_WITHOUT_HEADER: &str = include_str!("test_data/message_without_header.json");

    fn encoder_fix44() -> Decoder {
        Decoder::new(Dictionary::fix44())
    }

    #[test]
    fn message_without_header() {
        let mut encoder = encoder_fix44();
        let result = encoder.decode(MESSAGE_WITHOUT_HEADER.as_bytes());
        match result {
            Err(DecodeError::Schema) => (),
            _ => panic!(),
        };
    }

    #[test]
    fn simple_message() {
        let mut encoder = encoder_fix44();
        let result = encoder.decode(MESSAGE_SIMPLE.as_bytes());
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_json() {
        let mut encoder = encoder_fix44();
        let result = encoder.decode("this is invalid JSON".as_bytes());
        match result {
            Err(DecodeError::Syntax) => (),
            _ => panic!(),
        };
    }
}
