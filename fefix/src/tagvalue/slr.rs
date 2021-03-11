//! A schema-less, [`HashMap`]-backed internal representation for FIX messages.

use crate::backend::field_value as val;
use crate::backend::*;
use crate::tagvalue::slr;
use crate::tagvalue::FixFieldValue;
use crate::StreamIterator;
use std::collections::BTreeMap;

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
    pub fn tag(&self) -> u32 {
        self.tag as u32
    }

    /// Returns an immutable reference to the value of `self`.
    pub fn value(&self) -> &FixFieldValue {
        &self.value
    }

    pub fn take_value(&mut self) -> FixFieldValue {
        std::mem::replace(&mut self.value, FixFieldValue::from(0i64))
    }
}

/// FIX message, backed by an associative array.
#[derive(Debug, Clone)]
pub struct Message {
    pub fields: BTreeMap<u32, FixFieldValue>,
    iter: FieldsIterator,
}

impl Backend for Message {
    type Error = ();
    type Iter = FieldsIterator;
    type IterItem = ();

    #[inline]
    fn field(&self, tag: u32) -> Option<&FixFieldValue> {
        self.fields
            .iter()
            .find(|(t, _)| **t == tag)
            .map(|(_, value)| value)
    }

    #[inline]
    fn clear(&mut self) {
        self.fields.clear();
    }

    #[inline]
    fn len(&self) -> usize {
        self.fields.len()
    }

    #[inline]
    fn insert(&mut self, tag: u32, value: FixFieldValue) -> Result<(), Self::Error> {
        self.fields.insert(tag, value);
        Ok(())
    }

    #[inline]
    fn for_each<E, F>(&self, mut f: F) -> Result<(), E>
    where
        F: FnMut(u32, &FixFieldValue) -> Result<(), E>,
    {
        for (tag, value) in self.fields.iter() {
            f(*tag, value)?;
        }
        Ok(())
    }

    #[inline]
    fn iter_fields(&mut self) -> &mut Self::Iter {
        &mut self.iter
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            fields: BTreeMap::new(),
            iter: FieldsIterator {},
        }
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.fields == other.fields
    }
}

impl<'a> Backend for &'a mut Message {
    type Error = ();
    type Iter = FieldsIterator;
    type IterItem = ();

    #[inline]
    fn field(&self, tag: u32) -> Option<&FixFieldValue> {
        self.fields
            .iter()
            .find(|(t, _)| **t == tag)
            .map(|(_, value)| value)
    }

    #[inline]
    fn clear(&mut self) {
        self.fields.clear();
    }

    #[inline]
    fn len(&self) -> usize {
        self.fields.len()
    }

    #[inline]
    fn insert(&mut self, tag: u32, value: FixFieldValue) -> Result<(), Self::Error> {
        self.fields.insert(tag, value);
        Ok(())
    }

    #[inline]
    fn for_each<E, F>(&self, mut f: F) -> Result<(), E>
    where
        F: FnMut(u32, &FixFieldValue) -> Result<(), E>,
    {
        for (tag, value) in self.fields.iter() {
            f(*tag, value)?;
        }
        Ok(())
    }

    #[inline]
    fn iter_fields(&mut self) -> &mut Self::Iter {
        &mut self.iter
    }
}

impl FieldRef<FixFieldValue> for () {
    fn tag(&self) -> u32 {
        unimplemented!()
    }

    fn value(&self) -> &FixFieldValue {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct FieldsIterator {}

impl StreamIterator for FieldsIterator {
    type Item = ();

    fn advance(&mut self) {}

    fn get(&self) -> Option<&Self::Item> {
        Some(&())
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
        Self::default()
    }

    /// Adds a field to `self`.
    pub fn add_field<K: Into<i64>>(&mut self, tag: K, value: slr::FixFieldValue) {
        self.fields.insert(tag.into() as u32, value);
    }

    /// Adds a string field to `self`.
    pub fn add_str<K: Into<i64>, S: Into<String>>(&mut self, tag: K, value: S) {
        self.add_field(
            tag,
            slr::FixFieldValue::string(value.into().as_bytes()).unwrap(),
        )
    }

    /// Adds an integer field to `self`.
    pub fn add_int<K: Into<i64>>(&mut self, tag: K, value: i64) {
        self.add_field(tag, slr::FixFieldValue::from(value))
    }

    pub fn get_field<K: Into<i64>>(&self, tag: K) -> Option<&slr::FixFieldValue> {
        self.fields.get(&(tag.into() as u32))
    }

    pub fn msg_type(&self) -> Option<&str> {
        match self.fields.get(&35) {
            Some(FixFieldValue::Atom(val::FieldValue::String(s))) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn seq_num(&self) -> Option<u64> {
        match self.fields.get(&34) {
            Some(FixFieldValue::Atom(val::FieldValue::Int(val::Int(n)))) => Some(*n as u64),
            _ => None,
        }
    }

    pub fn iter_fields_seq(&self) -> MessageIterFields {
        MessageIterFields {
            message: self,
            i: 0,
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

#[derive(Debug)]
pub struct MessageIterFields<'a> {
    message: &'a Message,
    i: usize,
}

//impl<'a> Iterator for MessageIterFields<'a> {
//    type Item = (u32, &'a FixFieldValue);
//
//    fn next(&mut self) -> Option<Self::Item> {
//        if self.i >= self.message.fields.len() {
//            None
//        } else {
//            let field = &self.message.fields[self.i];
//            self.i += 1;
//            Some((field.0, field.1))
//        }
//    }
//}
