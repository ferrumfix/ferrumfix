use super::{Config, DecodeError};
use crate::dict::{FieldLocation, IsFieldDefinition};
use crate::{FieldMap, FieldType, FieldValueError, GetConfig, RepeatingGroup};
use rustc_hash::FxHashMap;
use rustyfix_dictionary::Dictionary;
use serde::{Deserialize, Serialize};
use smartstring::alias::String as SmartString;
use std::borrow::{Borrow, Cow};
use std::sync::Arc;

/// A read-only JSON FIX message as parsed by [`Decoder`].
#[derive(Debug, Copy, Clone)]
pub struct Message<'a> {
    internal: &'a MessageInternal<'a>,
    group_map: Option<&'a Fields<'a>>,
}

impl<'a> Message<'a> {
    /// Creates an [`Iterator`] over all FIX fields in `self`.
    pub fn iter_fields(&self) -> MessageFieldsIter<'a> {
        MessageFieldsIter {
            fields: self.internal.std_header.iter(),
        }
    }

    fn field_map<F>(&self, field: &F) -> &'a Fields<'a>
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

impl<'a, F> FieldMap<&F> for Message<'a>
where
    F: IsFieldDefinition,
{
    type Group = MessageGroup<'a>;

    fn group(
        &self,
        field: &F,
    ) -> Result<Self::Group, FieldValueError<<usize as FieldType>::Error>> {
        self.field_map(field)
            .get(field.name())
            .ok_or(FieldValueError::Missing)
            .and_then(|field_or_group| {
                if let FieldOrGroup::Group(entries) = field_or_group {
                    Ok(MessageGroup {
                        message: Message {
                            internal: self.internal,
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
#[derive(Debug, Copy, Clone)]
pub struct MessageGroup<'a> {
    message: Message<'a>,
    entries: &'a [Fields<'a>],
}

impl<'a> RepeatingGroup for MessageGroup<'a> {
    type Entry = Message<'a>;

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn get(&self, i: usize) -> Option<Self::Entry> {
        self.entries.get(i).map(|context| Message {
            internal: self.message.internal,
            group_map: Some(context),
        })
    }
}

/// An [`Iterator`] over the fields of a JSON message.
#[derive(Debug)]
#[allow(dead_code)]
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
#[allow(dead_code)]
pub struct Decoder {
    dictionaries: FxHashMap<SmartString, Arc<Dictionary>>,
    message_builder: MessageInternal<'static>,
    config: Config,
}

impl Decoder {
    /// Creates a new JSON [`Decoder`]. `dict` serves as a reference for data type inference
    /// of incoming messages' fields. Configuration options are initialized via [`Default`].
    pub fn new(dict: Dictionary) -> Self {
        let mut dictionaries = FxHashMap::default();
        dictionaries.insert(dict.version().into(), Arc::new(dict));
        Self {
            dictionaries,
            message_builder: MessageInternal::default(),
            config: Config::default(),
        }
    }

    /// Decodes `data` and returns a [`Message`].
    pub fn decode<'a>(&'a mut self, data: &'a mut [u8]) -> Result<Message<'a>, DecodeError> {
        let message = self.message_builder();
        simd_json::from_slice(data)
            .map_err(|err| {
                if err.is_syntax() || err.is_eof() || err.is_io() {
                    DecodeError::Syntax
                } else {
                    DecodeError::Schema
                }
            })
            .map(|x| {
                *message = x;
                Message {
                    internal: message,
                    group_map: None,
                }
            })
    }

    fn message_builder<'a>(&'a mut self) -> &'a mut MessageInternal<'a> {
        self.message_builder.clear();
        // SAFETY: This transmute changes the lifetime parameter of MessageInternal from 'static to 'a.
        // This is sound because:
        // 1. The MessageInternal<'static> is a private field only accessible through this method
        // 2. The returned reference has lifetime 'a tied to &mut self
        // 3. MessageInternal.clear() is called immediately before, invalidating old references
        // 4. The MessageInternal only stores references to data provided in the current decode call
        // 5. The lifetime 'a ensures references cannot outlive the source data
        //
        // FIXME: This transmute should be eliminated by redesigning MessageInternal lifetime management.
        unsafe {
            std::mem::transmute::<&'a mut MessageInternal<'static>, &'a mut MessageInternal<'a>>(
                &mut self.message_builder,
            )
        }
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

type Fields<'a> = FxHashMap<Cow<'a, str>, FieldOrGroup<'a>>;

/// A field or a repeating group of fields.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum FieldOrGroup<'a> {
    /// A single field.
    Field(Cow<'a, str>),
    /// A repeating group of fields.
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
        Decoder::new(Dictionary::fix44().unwrap())
    }

    #[test]
    fn message_without_header() {
        let mut encoder = encoder_fix44();
        let mut bytes = MESSAGE_WITHOUT_HEADER.as_bytes().to_vec();
        let result = encoder.decode(&mut bytes);
        match result {
            Err(DecodeError::Schema) => (),
            _ => panic!(),
        };
    }

    #[test]
    fn simple_message() {
        let mut encoder = encoder_fix44();
        let mut bytes = MESSAGE_SIMPLE.as_bytes().to_vec();
        let result = encoder.decode(&mut bytes);
        assert!(result.is_ok());
    }

    #[test]
    fn invalid_json() {
        let mut encoder = encoder_fix44();
        let mut bytes = "{unclosed_brace".as_bytes().to_vec(); // This is truly invalid JSON syntax
        let result = encoder.decode(&mut bytes);
        match result {
            Err(DecodeError::Syntax) => (),
            _ => panic!("Expected Err(DecodeError::Syntax), got: {result:?}"),
        };
    }
}
