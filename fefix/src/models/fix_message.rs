use super::field_value as val;
use super::{Error, FixFieldAccess, FixFieldsIter};
use crate::models::FixFieldValue;
use std::collections::HashMap;

const DEFAULT_FIELDS_LEN: usize = 64;

type Result<T> = std::result::Result<T, Error>;

/// FIX message data structure with fast associative and sequential access.
#[derive(Debug, Clone, PartialEq)]
pub struct FixMessage {
    fields: HashMap<u32, FixFieldValue>,
    insertion_order: Vec<u32>,
    i_first_cell: usize,
    i_last_cell: usize,
    len_end_header: usize,
    len_end_body: usize,
    len_end_trailer: usize,
}

impl FixMessage {
    /// Creates a new [`Message`] without any fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::FixMessage;
    ///
    /// let msg = FixMessage::new();
    /// ```
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            insertion_order: Vec::with_capacity(DEFAULT_FIELDS_LEN),
            i_first_cell: 0,
            i_last_cell: 0,
            len_end_header: 0,
            len_end_body: 0,
            len_end_trailer: 0,
        }
    }

    /// Removes all fields from `self`.
    pub fn clear(&mut self) {
        self.fields.clear();
        self.fields.shrink_to_fit();
        self.insertion_order.clear();
        self.insertion_order.reserve_exact(DEFAULT_FIELDS_LEN);
        self.i_first_cell = 0;
        self.i_last_cell = 0;
        self.len_end_body = 0;
        self.len_end_header = 0;
        self.len_end_trailer = 0;
    }

    /// Returns `true` if and only if `self` can store fields with `tag`; returns
    /// `false` otherwise.
    pub fn allows_tag(&self, _tag: u32) -> bool {
        true
    }

    /// Adds a field to `self`.
    pub fn add_field(&mut self, tag: u32, value: FixFieldValue) -> Result<()> {
        if self.fields.contains_key(&tag) {
            Err(Error::Duplicate)
        } else {
            self.fields.insert(tag, value);
            self.insertion_order.push(tag);
            Ok(())
        }
    }

    /// Adds a string field to `self`.
    ///
    /// # Panics
    ///
    /// This function panics if `tag` is a duplicate.
    pub fn add_str<S: Into<String>>(&mut self, tag: u32, value: S) {
        self.add_field(tag, FixFieldValue::string(value.into().as_bytes()).unwrap())
            .unwrap()
    }

    /// Adds an integer field to `self`.
    ///
    /// # Panics
    ///
    /// This function panics if `tag` is a duplicate.
    pub fn add_i64(&mut self, tag: u32, value: i64) {
        self.add_field(tag, FixFieldValue::from(value)).unwrap()
    }

    /// Returns an immutable reference to the field value of `tag` in `self`, if
    /// present.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::FixMessage;
    ///
    /// let message = &mut FixMessage::new();
    /// assert!(message.field(8).is_none());
    /// message.add_str(8, "FIX.4.4");
    /// assert!(message.field(8).is_some());
    /// ```
    pub fn field(&self, tag: u32) -> Option<&FixFieldValue> {
        self.fields.get(&tag)
    }

    pub fn f_msg_type(&self) -> Option<&str> {
        match self.fields.get(&35) {
            Some(FixFieldValue::Atom(val::FieldValue::String(s))) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn f_seq_num(&self) -> Option<u64> {
        match self.fields.get(&34) {
            Some(FixFieldValue::Atom(val::FieldValue::Int(val::Int(n)))) => Some(*n as u64),
            _ => None,
        }
    }

    pub fn f_test_indicator(&self) -> Option<bool> {
        let y = FixFieldValue::from('Y');
        let n = FixFieldValue::from('N');
        match self.fields.get(&464) {
            Some(f) if *f == y => Some(true),
            Some(f) if *f == n => Some(false),
            _ => Some(false),
        }
    }

    pub fn end_header(&mut self) {}
}

impl FixFieldAccess for FixMessage {
    fn field_char(&self, tag: u32) -> Option<char> {
        match self.field(tag) {
            Some(FixFieldValue::Atom(val::FieldValue::Char(c))) => Some(c.clone().into()),
            _ => None,
        }
    }

    fn field_data(&self, tag: u32) -> Option<&[u8]> {
        match self.field(tag) {
            Some(FixFieldValue::Atom(val::FieldValue::String(s))) => Some(s.as_str().as_bytes()),
            _ => None,
        }
    }

    fn field_bool(&self, tag: u32) -> Option<bool> {
        match self.field(tag) {
            Some(FixFieldValue::Atom(val::FieldValue::Boolean(x))) => Some(x.clone().into()),
            _ => None,
        }
    }

    fn field_i64(&self, tag: u32) -> Option<i64> {
        match self.field(tag) {
            Some(FixFieldValue::Atom(val::FieldValue::Int(x))) => Some(x.0 as i64),
            _ => None,
        }
    }

    fn field_str(&self, tag: u32) -> Option<&str> {
        match self.field(tag) {
            Some(FixFieldValue::Atom(val::FieldValue::String(s))) => Some(s.as_str()),
            _ => None,
        }
    }
}

impl<'a> FixFieldsIter<&'a FixFieldValue> for &'a FixMessage {
    type FieldsIter = FieldsIter<'a>;
    type FieldsIterStdHeader = FieldsIter<'a>;
    type FieldsIterBody = FieldsIter<'a>;

    /// Creates an [`Iterator`] over all FIX fields in `self`.
    fn iter_fields(&self) -> Self::FieldsIter {
        FieldsIter {
            message: self,
            i: self.i_first_cell,
            until: 0,
        }
    }

    /// Returns an [`Iterator`] over all FIX fields in the `StandardHeader`.
    fn iter_fields_in_std_header(&self) -> Self::FieldsIterStdHeader {
        FieldsIter {
            message: self,
            i: self.i_first_cell,
            until: 0,
        }
    }

    /// Returns an [`Iterator`] over all FIX fields in the body.
    fn iter_fields_in_body(&self) -> Self::FieldsIterBody {
        FieldsIter {
            message: self,
            i: self.i_first_cell,
            until: 0,
        }
    }
}

/// An [`Iterator`] over the FIX fields over a message.
///
/// This `struct` is created by [`Message::fields`] and its fellow methods.
#[derive(Debug)]
pub struct FieldsIter<'a> {
    message: &'a FixMessage,
    i: usize,
    until: usize,
}

impl<'a> Iterator for FieldsIter<'a> {
    type Item = (u32, &'a FixFieldValue);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            let value = self.message.field(self.i as u32).unwrap();
            Some((self.i as u32, &value))
        } else {
            None
        }
    }
}
