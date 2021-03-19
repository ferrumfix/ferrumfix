use crate::tagvalue::field_value as val;
use crate::tagvalue::FixFieldValue;
use std::collections::HashMap;

const DEFAULT_FIELDS_LEN: usize = 64;

/// FIX message data structure with fast associative and sequential access.
#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    fields: HashMap<u32, FixFieldValue>,
    insertion_order: Vec<u32>,
    i_first_cell: usize,
    i_last_cell: usize,
    len_end_header: usize,
    len_end_body: usize,
    len_end_trailer: usize,
}

impl Message {
    /// Creates a new [`Message`] without any fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::Message;
    ///
    /// let msg = Message::new();
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
    pub fn add_int(&mut self, tag: u32, value: i64) {
        self.add_field(tag, FixFieldValue::from(value)).unwrap()
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

    pub fn test_indicator(&self) -> Option<bool> {
        let y = FixFieldValue::from('Y');
        let n = FixFieldValue::from('N');
        match self.fields.get(&464) {
            Some(f) if *f == y => Some(true),
            Some(f) if *f == n => Some(false),
            _ => Some(false),
        }
    }

    /// Returns an immutable reference to the field value of `tag` in `self`, if
    /// present.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::Message;
    ///
    /// let message = &mut Message::new();
    /// assert!(message.field(8).is_none());
    /// message.add_str(8, "FIX.4.4");
    /// assert!(message.field(8).is_some());
    /// ```
    pub fn field(&self, tag: u32) -> Option<&FixFieldValue> {
        self.fields.get(&tag)
    }

    /// Creates an [`Iterator`] over all FIX fields in `self`.
    pub fn fields(&self) -> FieldsIter {
        FieldsIter {
            message: self,
            i: self.i_first_cell,
            until: 0,
        }
    }

    pub fn end_header(&mut self) {}

    /// Returns an [`Iterator`] over all FIX fields in the `StandardHeader`.
    pub fn fields_in_std_header(&self) -> FieldsIter {
        FieldsIter {
            message: self,
            i: self.i_first_cell,
            until: 0,
        }
    }

    /// Returns an [`Iterator`] over all FIX fields in the body.
    pub fn fields_in_body(&self) -> FieldsIter {
        FieldsIter {
            message: self,
            i: self.i_first_cell,
            until: 0,
        }
    }

    /// Returns an [`Iterator`] over all FIX fields in the `StandardTrailer`.
    pub fn fields_in_std_trailer(&self) -> FieldsIter {
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
    message: &'a Message,
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

#[derive(Debug, Clone)]
pub enum Error {
    Duplicate,
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_message_has_no_fields() {
        let msg = Message::new();
        assert_eq!(msg.fields().count(), 0);
    }
}
