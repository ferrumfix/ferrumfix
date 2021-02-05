//! A schema-less, [`HashMap`]-backed internal representation for FIX messages.

use crate::app::slr;
use crate::app::TsrMessageRef;
use std::collections::BTreeMap;
use std::time::SystemTime;

/// An owned value of a FIX field.
#[derive(Clone, Debug, PartialEq)]
pub enum FixFieldValue {
    Int(i64),
    Float(f64),
    Char(char),
    String(String),
    Data(Vec<u8>),
    Group(Vec<BTreeMap<i64, FixFieldValue>>),
}

impl From<i64> for FixFieldValue {
    fn from(v: i64) -> Self {
        FixFieldValue::Int(v)
    }
}

impl From<String> for FixFieldValue {
    fn from(v: String) -> Self {
        FixFieldValue::String(v)
    }
}

impl From<f64> for FixFieldValue {
    fn from(v: f64) -> Self {
        FixFieldValue::Float(v)
    }
}

impl From<(u8, u16)> for FixFieldValue {
    fn from(v: (u8, u16)) -> Self {
        FixFieldValue::Int(((v.0 as i64) << 16) + (v.1 as i64))
    }
}

impl From<char> for FixFieldValue {
    fn from(v: char) -> Self {
        FixFieldValue::Char(v)
    }
}

impl From<usize> for FixFieldValue {
    fn from(v: usize) -> Self {
        FixFieldValue::Int(v as i64)
    }
}

impl From<Vec<u8>> for FixFieldValue {
    fn from(v: Vec<u8>) -> Self {
        FixFieldValue::Data(v)
    }
}

impl From<bool> for FixFieldValue {
    fn from(v: bool) -> Self {
        FixFieldValue::Char(if v { 't' } else { 'f' })
    }
}

impl From<u8> for FixFieldValue {
    fn from(v: u8) -> Self {
        FixFieldValue::Int(v.into())
    }
}

impl From<SystemTime> for FixFieldValue {
    fn from(v: SystemTime) -> Self {
        FixFieldValue::Int(v.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    tag: i64,
    value: FixFieldValue,
}

impl Field {
    /// Creates a new [`Field`] value with `tag` and `value`.
    pub fn new(tag: u32, value: FixFieldValue) -> Self {
        Self { tag: tag as i64, value }
    }

    /// Returns the field tag of `self`.
    pub fn tag(&self) -> i64 {
        self.tag
    }

    /// Returns an immutable reference to the value of `self`.
    pub fn value(&self) -> &FixFieldValue {
        &self.value
    }
}

/// FIX message, backed by an associative array.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Message {
    pub fields: BTreeMap<i64, FixFieldValue>,
}

impl TsrMessageRef for Message {
    fn get_field(&self, msg_type: u32) -> Option<&slr::FixFieldValue> {
        self.fields.get(&(msg_type as i64))
    }

    fn set_field(&mut self, msg_type: u32, val: slr::FixFieldValue) {
        self.fields.insert(msg_type as i64, val);
    }
}

impl<'a> Iterator for &'a Message {
    type Item = slr::FixFieldValue;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl Message {
    /// Creates a new [`Message`] without any fields.
    pub fn new() -> Self {
        Message {
            fields: BTreeMap::new(),
        }
    }

    /// Adds a field to `self`.
    pub fn add_field<K: Into<i64>>(&mut self, tag: K, value: slr::FixFieldValue) {
        self.fields.insert(tag.into(), value);
    }

    /// Adds a string field to `self`.
    pub fn add_str<K: Into<i64>, S: Into<String>>(&mut self, tag: K, value: S) {
        self.add_field(tag, slr::FixFieldValue::String(value.into()))
    }

    /// Adds an integer field to `self`.
    pub fn add_int<K: Into<i64>>(&mut self, tag: K, value: i64) {
        self.add_field(tag, slr::FixFieldValue::Int(value))
    }

    pub fn get_field<K: Into<i64>>(&self, tag: K) -> Option<&slr::FixFieldValue> {
        self.fields.get(&tag.into())
    }

    pub fn msg_type(&self) -> Option<&str> {
        match self.fields.get(&35) {
            Some(FixFieldValue::String(s)) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn seq_num(&self) -> Option<u64> {
        match self.fields.get(&34) {
            Some(FixFieldValue::Int(n)) => Some(*n as u64),
            _ => None,
        }
    }

    pub fn test_indicator(&self) -> Option<bool> {
        match self.fields.get(&464) {
            Some(FixFieldValue::Char('Y')) => Some(true),
            Some(FixFieldValue::Char('N')) => Some(false),
            _ => Some(false),
        }
    }
}
