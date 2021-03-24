//! FIX Adapted for Streaming (FAST) support.

use crate::buffering::Buffer;
use crate::Dictionary;
use crate::FixMessage;
use bitvec::vec::BitVec;
use codec::decode_stop_bit_bitvec;
use errors::Error;
use std::collections::HashMap;
use template::Template;

mod codec;
pub mod decimal;
mod errors;
mod field_operators;
mod template;

pub use codec::{Codec, PresenceMap};
pub use decimal::Decimal;
pub use field_operators::*;
pub use template::*;

#[derive(Clone, Debug)]
pub struct Fast {
    dict: Dictionary,
    templates: HashMap<String, Template>,
    message: FixMessage,
}

impl Fast {
    /// Builds a new `TagValue` encoding device with an empty FIX dictionary.
    pub fn new() -> Self {
        let dict = Dictionary::empty();
        Fast {
            dict,
            templates: HashMap::new(),
            message: FixMessage::new(),
        }
    }

    pub fn with_template(mut self, template: Template) -> Self {
        self.templates.insert(template.name().to_string(), template);
        self
    }
}

impl Fast {
    pub fn decode(&mut self, mut source: &[u8]) -> Result<&FixMessage, Error> {
        let _presence_map = decode_stop_bit_bitvec(&mut source).unwrap();
        let mut presence_by_field: BitVec = BitVec::new();
        let message = FixMessage::new();
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
                        val.deserialize(&mut source)?;
                        PrimitiveValue::SInt32(val)
                    }
                    PrimitiveType::UInt32 => {
                        let mut val = 0u32;
                        val.deserialize(&mut source)?;
                        PrimitiveValue::UInt32(val)
                    }
                    PrimitiveType::SInt64 => {
                        let mut val = 0i64;
                        val.deserialize(&mut source)?;
                        PrimitiveValue::SInt64(val)
                    }
                    PrimitiveType::UInt64 => {
                        let mut val = 0u64;
                        val.deserialize(&mut source)?;
                        PrimitiveValue::UInt64(val)
                    }
                    PrimitiveType::Bytes => {
                        let mut val: Vec<u8> = Vec::new();
                        val.deserialize(&mut source)?;
                        PrimitiveValue::Bytes(&val[..])
                    }
                    PrimitiveType::Ascii => {
                        let mut val = String::new();
                        val.deserialize(&mut source)?;
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
        self.message = message;
        Ok(&self.message)
    }

    pub fn encode<B>(&mut self, buffer: &mut B, _message: &FixMessage) -> Result<usize, Error>
    where
        B: Buffer,
    {
        let _presence_by_field: BitVec = BitVec::new();
        Ok(buffer.as_slice().len())
    }
}
