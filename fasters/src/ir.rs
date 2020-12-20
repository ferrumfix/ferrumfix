//! A schema-less, dynamic internal representation for FIX data.

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum FixFieldValue {
    Int(i64),
    Float(f64),
    Char(char),
    String(String),
    Data(Vec<u8>),
}

pub struct Field {
    pub tag: i64,
    pub value: FixFieldValue,
    pub checksum: u8,
    pub len: usize,
}

pub struct Message {
    pub fields: HashMap<i64, FixFieldValue>,
}

impl Message {
    pub fn new() -> Self {
        Message {
            fields: HashMap::new(),
        }
    }
}
