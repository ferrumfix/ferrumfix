use super::{Config, Configure, DecodeError};
use crate::dict::FieldLocation;
use crate::dict::IsFieldDefinition;
use crate::{Dictionary, FixValue, GetConfig};
use crate::{FieldAccess, RepeatingGroup};
use serde::{Deserialize, Serialize};
use std::borrow::{Borrow, Cow};
use std::collections::HashMap;

/// A read-only JSON FIX message as parsed by [`Decoder`].
#[derive(Debug, Copy, Clone)]
pub struct Message<'a> {
    internal: &'a MessageInternal<'a>,
    group_map: Option<&'a FieldMap<'a>>,
}

impl<'a> Message<'a> {
    /// Creates an [`Iterator`] over all FIX fields in `self`.
    pub fn iter_fields(&self) -> MessageFieldsIter<'a> {
        MessageFieldsIter {
            fields: self.internal.std_header.iter(),
        }
    }

    fn field_map<F>(&self, field: &F) -> &'a FieldMap<'a>
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

impl<'a, F> FieldAccess<&F> for Message<'a>
where
    F: IsFieldDefinition,
{
    type Group = MessageGroup<'a>;

    fn group_opt(&self, field: &F) -> Option<Result<Self::Group, <usize as FixValue<'a>>::Error>> {
        self.field_map(field)
            .get(field.name())
            .and_then(|field_or_group| {
                if let FieldOrGroup::Group(ref entries) = field_or_group {
                    Some(Ok(MessageGroup {
                        message: Message {
                            internal: &self.internal,
                            group_map: None,
                        },
                        entries,
                    }))
                } else {
                    None
                }
            })
    }

    fn fv_raw(&self, field: &F) -> Option<&[u8]> {
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
    entries: &'a [FieldMap<'a>],
}

impl<'a> RepeatingGroup for MessageGroup<'a> {
    type Entry = Message<'a>;

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn entry_opt(&self, i: usize) -> Option<Self::Entry> {
        self.entries.get(i).map(|context| Message {
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
pub struct Decoder<C = Config> {
    dictionaries: HashMap<String, Dictionary>,
    message_builder: MessageInternal<'static>,
    config: C,
}

impl<C> Decoder<C>
where
    C: Configure,
{
    /// Creates a new JSON [`Decoder`]. `dict` serves as a reference for data type inference
    /// of incoming messages' fields. Configuration options are initialized via [`Default`].
    pub fn new(dict: Dictionary) -> Self {
        let mut dictionaries = HashMap::new();
        dictionaries.insert(dict.get_version().to_string(), dict);
        Self {
            dictionaries,
            message_builder: MessageInternal::default(),
            config: C::default(),
        }
    }

    pub fn decode<'a>(&'a mut self, data: &'a [u8]) -> Result<Message<'a>, DecodeError> {
        let mut deserilizer = serde_json::Deserializer::from_slice(data);
        let msg = self.message_builder();
        MessageInternal::deserialize_in_place(&mut deserilizer, msg).map_err(|err| {
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

    fn message_builder<'a>(&'a mut self) -> &'a mut MessageInternal<'a> {
        self.message_builder.clear();
        unsafe {
            std::mem::transmute::<&'a mut MessageInternal<'static>, &'a mut MessageInternal<'a>>(
                &mut self.message_builder,
            )
        }
    }
}

impl<C> GetConfig for Decoder<C> {
    type Config = C;

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn config_mut(&mut self) -> &mut Self::Config {
        &mut self.config
    }
}

type FieldMap<'a> = HashMap<Cow<'a, str>, FieldOrGroup<'a>>;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum FieldOrGroup<'a> {
    Field(Cow<'a, str>),
    #[serde(borrow)]
    Group(Vec<FieldMap<'a>>),
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
struct MessageInternal<'a> {
    #[serde(borrow, rename = "Header")]
    std_header: FieldMap<'a>,
    #[serde(borrow, rename = "Body")]
    body: FieldMap<'a>,
    #[serde(borrow, rename = "Trailer")]
    std_trailer: FieldMap<'a>,
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
        let result = encoder.decode(&mut MESSAGE_WITHOUT_HEADER.as_bytes());
        match result {
            Err(DecodeError::Schema) => (),
            _ => panic!(),
        };
    }

    #[test]
    fn simple_message() {
        let mut encoder = encoder_fix44();
        let result = encoder.decode(&mut MESSAGE_SIMPLE.as_bytes());
        assert!(result.is_ok());
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
