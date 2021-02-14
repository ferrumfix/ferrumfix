//! A schema-less, [`HashMap`]-backed internal representation for FIX messages.

use super::value as val;
use crate::backend::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    tag: i64,
    value: FixFieldValue,
}

impl Field {
    /// Creates a new [`Field`] value with `tag` and `value`.
    pub fn new(tag: u32, value: FixFieldValue) -> Self {
        Self {
            tag: tag as i64,
            value,
        }
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

impl ReadFields for Message {
    fn get_field(&self, msg_type: u32) -> Option<&slr::FixFieldValue> {
        self.fields.get(&(msg_type as i64))
    }
}

impl WriteFields for Message {
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
        self.add_field(tag, slr::FixFieldValue::from(value))
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
            Some(FixFieldValue::Atom(val::Atomic::Int(val::Int(n)))) => Some(*n as u64),
            _ => None,
        }
    }

    pub fn test_indicator(&self) -> Option<bool> {
        let y = FixFieldValue::from('Y');
        let n = FixFieldValue::from('N');
        match self.fields.get(&464) {
            Some(f) if *f == y => Some(true),
            Some(f) if *f == n => Some(false),
            _ => Some(false),
        }
    }
}
