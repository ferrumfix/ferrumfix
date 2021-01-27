use crate::app::slr;
use crate::dictionary::Dictionary;
use crate::encoders::Encoding;
use bitvec::vec::BitVec;
use codec::decode_stop_bit_bitvec;
use errors::Error;
use std::collections::HashMap;
use std::io;
use template::Template;

mod codec;
mod errors;
mod field_operators;
mod template;
pub mod decimal;


pub use codec::Codec;
pub use decimal::Decimal;
pub use field_operators::*;
pub use template::*;

pub struct Fast {
    dict: Dictionary,
    templates: HashMap<String, Template>,
}

type DecodeResult<T> = std::result::Result<T, <Fast as Encoding<slr::Message>>::DecodeErr>;
type EncodeResult<T> = std::result::Result<T, <Fast as Encoding<slr::Message>>::EncodeErr>;

impl Fast {
    /// Builds a new `TagValue` encoding device with an empty FIX dictionary.
    pub fn new() -> Self {
        Fast {
            dict: Dictionary::empty(),
            templates: HashMap::new(),
        }
    }

    pub fn with_template(mut self, template: Template) -> Self {
        self.templates.insert(template.name().to_string(), template);
        self
    }
}

impl Encoding<slr::Message> for Fast {
    type EncodeErr = Error;
    type DecodeErr = Error;

    fn decode(&self, source: &mut impl io::BufRead) -> DecodeResult<slr::Message> {
        let _presence_map = decode_stop_bit_bitvec(source).unwrap();
        let mut presence_by_field: BitVec = BitVec::new();
        let message = slr::Message::new();
        for field in self.templates.get("").unwrap().iter_items() {
            if let template::FieldType::Primitive(_f) = &field.kind() {
                presence_by_field.push(field.is_mandatory());
            } else {
                presence_by_field.push(false);
            }
        }
        for field in self.templates.get("").unwrap().iter_items() {
            if let template::FieldType::Primitive(f) = field.kind() {
                match f {
                    PrimitiveType::SInt32 => {
                        let mut val = 0i32;
                        val.deserialize(source)?;
                        PrimitiveValue::SInt32(val)
                    }
                    PrimitiveType::UInt32 => {
                        let mut val = 0u32;
                        val.deserialize(source)?;
                        PrimitiveValue::UInt32(val)
                    }
                    PrimitiveType::SInt64 => {
                        let mut val = 0i64;
                        val.deserialize(source)?;
                        PrimitiveValue::SInt64(val)
                    }
                    PrimitiveType::UInt64 => {
                        let mut val = 0u64;
                        val.deserialize(source)?;
                        PrimitiveValue::UInt64(val)
                    }
                    PrimitiveType::Bytes => {
                        let mut val: Vec<u8> = Vec::new();
                        val.deserialize(source)?;
                        PrimitiveValue::Bytes(&val[..])
                    }
                    PrimitiveType::Ascii => {
                        let mut val = String::new();
                        val.deserialize(source)?;
                        PrimitiveValue::Ascii(val.as_bytes())
                    }
                    _ => {
                        todo!();
                    }
                };
            } else {
                // Sequence or group.
                todo!();
            }
        }
        Ok(message)
    }

    fn encode(&self, _message: slr::Message) -> EncodeResult<Vec<u8>> {
        let _presence_by_field: BitVec = BitVec::new();
        let buffer = Vec::new();
        Ok(buffer)
    }
}
