//! A schema-less, dynamic internal representation for FIX data.

use crate::ir;
use std::collections::HashMap;
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub enum FixFieldValue {
    Int(i64),
    Float(f64),
    Char(char),
    String(String),
    Data(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub tag: i64,
    pub value: FixFieldValue,
    pub checksum: u8,
    pub len: usize,
}

impl Field {
    pub fn encode(&self, write: &mut impl io::Write) -> io::Result<usize> {
        let mut length = write.write(self.tag.to_string().as_bytes())? + 2;
        write.write_all(&[b'='])?;
        length += match &self.value {
            FixFieldValue::Char(c) => write.write(&[*c as u8]),
            FixFieldValue::String(s) => write.write(s.as_bytes()),
            FixFieldValue::Int(int) => write.write(int.to_string().as_bytes()),
            FixFieldValue::Float(float) => write.write(float.to_string().as_bytes()),
            FixFieldValue::Data(raw_data) => write.write(&raw_data),
        }?;
        write.write_all(&[1u8])?;
        Ok(length)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub fields: HashMap<i64, FixFieldValue>,
}

impl Message {
    pub fn new() -> Self {
        Message {
            fields: HashMap::new(),
        }
    }

    pub fn add_field<K: Into<i64>>(&mut self, tag: K, value: ir::FixFieldValue) {
        self.fields.insert(tag.into(), value);
    }
}
