use super::{Config, EncodeError};
use crate::dict::{IsFieldDefinition, FieldLocation};
use crate::{FieldType, GetConfig, SetField};
use rustyfix_dictionary::Dictionary;
use serde::Serialize;
use std::borrow::Cow;
use std::sync::Arc;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;

type Fields<'a> = FxHashMap<Cow<'a, str>, FieldOrGroup<'a>>;

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
enum FieldOrGroup<'a> {
    Field(Cow<'a, str>),
    Group(SmallVec<[Fields<'a>; 8]>),
}

#[derive(Serialize, Debug, Clone, Default)]
struct MessageInternal<'a> {
    #[serde(rename = "Header")]
    std_header: Fields<'a>,
    #[serde(rename = "Body")]
    body: Fields<'a>,
    #[serde(rename = "Trailer")]
    std_trailer: Fields<'a>,
}

#[derive(Debug)]
pub struct Encoder<'a> {
    dictionary: Arc<Dictionary>,
    config: Config,
    handle: EncoderHandle<'a>,
}

#[derive(Debug)]
pub struct EncoderHandle<'a> {
    message: MessageInternal<'a>,
    dictionary: Arc<Dictionary>,
}

impl<'a> EncoderHandle<'a> {
    pub fn done(self) -> Result<Vec<u8>, simd_json::Error> {
        simd_json::to_vec(&self.message)
    }
}

impl<'b, F> SetField<F> for EncoderHandle<'_>
where
    F: IsFieldDefinition,
{
    fn set_with<'s, V>(&'s mut self, field: F, value: V, _settings: V::SerializeSettings)
    where
        V: FieldType<'s>,
    {
        let field_name = field.name();
        let field_value_as_string = value.to_string();

        let value_cow = Cow::Owned(field_value_as_string);
        let field_or_group = FieldOrGroup::Field(value_cow);

        let target_map = match field.location() {
            FieldLocation::Header => &mut self.message.std_header,
            FieldLocation::Body => &mut self.message.body,
            FieldLocation::Trailer => &mut self.message.std_trailer,
        };

        target_map.insert(Cow::Borrowed(field_name), field_or_group);
    }
}

impl<'a> Encoder<'a> {
    pub fn new(dict: Dictionary) -> Self {
        let dict = Arc::new(dict);
        Self {
            dictionary: dict.clone(),
            config: Config::default(),
            handle: EncoderHandle {
                message: MessageInternal::default(),
                dictionary: dict,
            },
        }
    }

    pub fn start_message(
        &'a mut self,
        _begin_string: &[u8],
        _msg_type: &[u8],
    ) -> &'a mut EncoderHandle<'a> {
        // In JSON encoding, we build up the structure and serialize at the end.
        // The handle can be reused.
        self.handle.message = MessageInternal::default();
        &mut self.handle
    }
}

impl GetConfig for Encoder<'_> {
    type Config = Config;

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn config_mut(&mut self) -> &mut Self::Config {
        &mut self.config
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fix44;
    use crate::prelude::*;
    use rustyfix_dictionary::Dictionary;

    #[test]
    fn simple_message() {
        let dict = Dictionary::fix44();
        let mut encoder = Encoder::new(dict);
        let handle = encoder.start_message(b"FIX.4.4", b"A");

        handle.set(fix44::BEGIN_STRING, "FIX.4.4");
        handle.set(fix44::MSG_TYPE, "A");

        let json_bytes = handle.done().unwrap();
        let json_string = String::from_utf8(json_bytes).unwrap();

        let value: serde_json::Value = serde_json::from_str(&json_string).unwrap();

        assert_eq!(value["Header"]["BeginString"], "FIX.4.4");
        assert_eq!(value["Header"]["MsgType"], "A");
    }
}
