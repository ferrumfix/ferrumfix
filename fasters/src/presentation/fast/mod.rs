use crate::app::dictionary::Dictionary;
use crate::app::slr;
use crate::presentation::Encoding;
use bitvec::vec::BitVec;
use codec::decode_stop_bit_bitvec;
use codec::Codec;
use errors::Error;
use std::collections::HashMap;
use std::io;
use template::Template;

mod codec;
mod errors;
mod field_operators;
mod template;

pub struct Fast {
    dict: Dictionary,
    templates: HashMap<String, Template>,
}

type DecodeResult<T> = std::result::Result<T, <Fast as Encoding>::DecodeErr>;
type EncodeResult<T> = std::result::Result<T, <Fast as Encoding>::EncodeErr>;

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

impl Encoding for Fast {
    type EncodeErr = Error;
    type DecodeErr = Error;

    fn decode(&self, source: &mut impl io::BufRead) -> DecodeResult<slr::Message> {
        let presence_map = decode_stop_bit_bitvec(source).unwrap();
        let mut presence_by_field: BitVec = BitVec::new();
        let mut message = slr::Message::new();
        for field in self.templates.get("").unwrap().iter_items() {
            if let template::FieldType::Primitive(f) = &field.kind() {
                presence_by_field.push(field.is_mandatory());
            } else {
                presence_by_field.push(false);
            }
        }
        for field in self.templates.get("").unwrap().iter_items() {
            if let template::FieldType::Primitive(f) = field.kind() {
                match f {
                    template::FieldPrimitiveType::SInt32 => {
                        let mut val = 0i32;
                        val.deserialize(source)?;
                        template::FieldValue::SInt32(val)
                    }
                    template::FieldPrimitiveType::UInt32 => {
                        let mut val = 0u32;
                        val.deserialize(source)?;
                        template::FieldValue::UInt32(val)
                    }
                    template::FieldPrimitiveType::SInt64 => {
                        let mut val = 0i64;
                        val.deserialize(source)?;
                        template::FieldValue::SInt64(val)
                    }
                    template::FieldPrimitiveType::UInt64 => {
                        let mut val = 0u64;
                        val.deserialize(source)?;
                        template::FieldValue::UInt64(val)
                    }
                    template::FieldPrimitiveType::ByteVector => {
                        let mut val: Vec<u8> = Vec::new();
                        val.deserialize(source)?;
                        template::FieldValue::ByteVector(val)
                    }
                    template::FieldPrimitiveType::AsciiString => {
                        let mut val = String::new();
                        val.deserialize(source)?;
                        template::FieldValue::AsciiString(val.into_bytes())
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

    fn encode(&self, message: slr::Message) -> EncodeResult<Vec<u8>> {
        let mut presence_by_field: BitVec = BitVec::new();
        let mut buffer = Vec::new();
        Ok(buffer)
    }
}
