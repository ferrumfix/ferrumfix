use crate::encodings::Encoding;
use crate::ir;
use crate::spec::{BaseType, Dictionary};
use codec::decode_stop_bit_bitvec;
use codec::Codec;
use bitvec::vec::BitVec;
use template::Template;
use std::io;
use std::collections::HashMap;

mod codec;
mod field_operators;
mod template;

pub struct Fast {
    dict: Dictionary,
    templates: HashMap<i64, Template>,
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
}

impl Encoding for Fast {
    type EncodeErr = Error;
    type DecodeErr = Error;

    fn decode(&self, source: &mut impl io::BufRead) -> DecodeResult<ir::Message> {
        let presence_map = decode_stop_bit_bitvec(source).unwrap();
        let mut presence_by_field: BitVec = BitVec::new();
        let mut message = ir::Message::new();
        for field in &self.templates.get(&1).unwrap().elements {
            if let template::ElementContent::Field(f) = &field.content {
                presence_by_field.push(f.presence);
            } else {
                presence_by_field.push(false);
            }
        }
        for field in &self.templates.get(&1).unwrap().elements {
            if let template::ElementContent::Field(f) = &field.content {
                match f.kind {
                    template::FieldType::SInt32 => {
                        let mut val = 0i32;
                        val.deserialize(source)?;
                        template::FieldValue::SInt32(val)
                    },
                    template::FieldType::UInt32 => {
                        let mut val = 0u32;
                        val.deserialize(source)?;
                        template::FieldValue::UInt32(val)
                    },
                    template::FieldType::SInt64 => {
                        let mut val = 0i64;
                        val.deserialize(source)?;
                        template::FieldValue::SInt64(val)
                    },
                    template::FieldType::UInt64 => {
                        let mut val = 0u64;
                        val.deserialize(source)?;
                        template::FieldValue::UInt64(val)
                    },
                    template::FieldType::ByteVector => {
                        let mut val: Vec<u8> = Vec::new();
                        val.deserialize(source)?;
                        template::FieldValue::ByteVector(val)
                    },
                    template::FieldType::AsciiString => {
                        let mut val = String::new();
                        val.deserialize(source)?;
                        template::FieldValue::AsciiString(val.into_bytes())
                    },
                    _ => {
                        todo!();
                    }
                };
            }
        }
        Ok(message)
    }

    fn encode(&self, message: ir::Message) -> EncodeResult<Vec<u8>> {
        let mut presence_by_field: BitVec = BitVec::new();
        let mut buffer = Vec::new();
        Ok(buffer)
    }
}

pub enum Error {
    FieldWithoutValue(u32),
    RepeatedTag(u32),
    Eof,
    InvalidStandardHeader,
    InvalidStandardTrailer,
    InvalidChecksum,
    Syntax,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Eof // FIXME
    }
}