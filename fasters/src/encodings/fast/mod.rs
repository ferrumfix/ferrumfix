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

    pub fn with_template(mut self, template: Template) -> Self {
        self.templates.insert(template.id, template);
        self
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

pub enum StaticError {
    /// It is a static error if templates encoded in the concrete XML syntax are
    /// in fact not well-formed, do not follow the rules of XML namespaces or are
    /// invalid with respect to the schema in Appendix 1.
    S1,
    /// It is a static error if an operator is specified for a field of a type to
    /// which the operator is not applicable.
    S2,
    /// It is a static error if an initial value specified by the value attribute
    /// in the concrete syntax cannot be converted to a value of the type of the
    /// field.
    S3,
    /// It is a static error if no initial value is specified for a constant
    /// operator.
    S4,
    /// It is a static error if no initial value is specified for a default
    /// operator on a mandatory field.
    S5,
}

pub enum DynamicError {
    /// It is a dynamic error if type of a field in a template cannot be
    /// converted to or from the type of the corresponding application field.
    D1,
    /// It is a dynamic error if an integer in the stream does not fall within
    /// the bounds of the specific integer type specified on the corresponding
    /// field.
    D2,
    /// It is a dynamic error if a decimal value cannot be encoded due to
    /// limitations introduced by using individual operators on exponent and
    /// mantissa.
    D3,
    /// It is a dynamic error if the type of the previous value is not the same
    /// as the type of the field of the current operator.
    D4,
    /// It is a dynamic error if a mandatory field is not present in the stream,
    /// has an undefined previous value and there is no initial value in the
    /// instruction context.
    D5,
    /// It is a dynamic error if a mandatory field is not present in the stream
    /// and has an empty previous value.
    D6,
    /// It is a dynamic error if the subtraction length exceeds the length of the
    /// base value or if it does not fall in the value rang of an int32.
    D7,
    /// It is a dynamic error if the name specified on a static template
    /// reference does not point to a template known by the encoder or decoder.
    D8,
    /// It is a dynamic error if a decoder cannot find a template associated with
    /// a template identifier appearing in the stream.
    D9,
    /// It is a dynamic error if the syntax of a string does not follow the rules
    /// for the type converted to.
    D10,
    /// It is a dynamic error if the syntax of a string does not follow the rules
    /// for the type converted to
    D11,
    /// It is a dynamic error if a block length preamble is zero.
    D12,
}

pub enum ReportableError {
    /// It is a reportable error if a decimal cannot be represented by an
    /// exponent in the range [-63 … 63] or if the mantissa does not fit in an
    /// int64.
    R1,
    /// It is a reportable error if the combined value after applying a tail or
    /// delta operator to a Unicode string is not a valid UTF-8 sequence.
    R2,
    /// It is a reportable error if a Unicode string that is being converted to
    /// an ASCII string contains characters that are outside the ASCII character
    /// set.
    R3,
    /// It is a reportable error if a value of an integer type cannot be
    /// represented in the target integer type in a conversion.
    R4,
    /// It is a reportable error if a decimal being converted to an integer has a
    /// negative exponent or if the resulting integer does not fit the target
    /// integer type.
    R5,
    /// It is a reportable error if an integer appears in an overlong encoding.
    R6,
    /// It is a reportable error if a presence map is overlong.
    R7,
    /// It is a reportable error if a presence map contains more bits than required.
    R8,
    /// It is a reportable error if a string appears in an overlong encoding.
    R9,
}

pub enum Error {
    Static(StaticError),
    Dynamic(DynamicError),
    Reportable(ReportableError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Static(StaticError::S1)
    }
}