use crate::backend::field_value as val;
use crate::tagvalue::{FixFieldValue, MessageRnd};

#[derive(Debug, Clone)]
pub struct MessageSeq {
    fields: Vec<(u32, FixFieldValue)>,
    len_of_std_header: usize,
    len_of_body: usize,
    len_of_std_trailer: usize,
}

impl MessageSeq {
    /// Creates a new [`Message`] without any fields.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn freeze_std_header(&mut self) {
        self.len_of_std_header = self.fields.len();
    }

    pub fn freeze_body(&mut self) {
        self.len_of_body = self.fields.len();
    }

    pub fn freeze_std_trailer(&mut self) {
        self.len_of_std_trailer = self.fields.len();
    }

    /// Adds a field to `self`.
    pub fn add_field<K: Into<u32>>(&mut self, tag: K, value: FixFieldValue) {
        self.fields.push((tag.into(), value));
    }

    /// Adds a string field to `self`.
    pub fn add_str<K: Into<u32>, S: Into<String>>(&mut self, tag: K, value: S) {
        self.add_field(tag, FixFieldValue::string(value.into().as_bytes()).unwrap())
    }

    /// Adds an integer field to `self`.
    pub fn add_int<K: Into<u32>>(&mut self, tag: K, value: i64) {
        self.add_field(tag, FixFieldValue::from(value))
    }

    pub fn field(&self, tag: u32) -> Option<&FixFieldValue> {
        let index = self.fields.iter().position(|(t, _)| *t == tag);
        index.map(|i| &self.fields[i].1)
    }

    pub fn msg_type(&self) -> Option<&str> {
        match self.field(35u32) {
            Some(FixFieldValue::Atom(val::FieldValue::String(s))) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn seq_num(&self) -> Option<u64> {
        match self.field(34u32) {
            Some(FixFieldValue::Atom(val::FieldValue::Int(val::Int(n)))) => Some(*n as u64),
            _ => None,
        }
    }

    pub fn test_indicator(&self) -> Option<bool> {
        let y = FixFieldValue::from('Y');
        let n = FixFieldValue::from('N');
        match self.field(464u32) {
            Some(f) if *f == y => Some(true),
            Some(f) if *f == n => Some(false),
            _ => Some(false),
        }
    }

    pub fn for_each<E, F>(&self, mut f: F) -> Result<(), E>
    where
        F: FnMut(u32, &FixFieldValue) -> Result<(), E>,
    {
        for (tag, value) in self.fields.iter() {
            f(*tag, value)?;
        }
        Ok(())
    }

    pub fn to_message_rnd(&self) -> MessageRnd {
        let mut msg = MessageRnd::default();
        for (tag, value) in self.fields.iter() {
            msg.add_field(*tag, value.clone());
        }
        msg
    }
}

impl Default for MessageSeq {
    fn default() -> Self {
        Self {
            fields: Vec::new(),
            len_of_std_header: 0,
            len_of_body: 0,
            len_of_std_trailer: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaggedField {
    tag: u32,
    value: FixFieldValue,
}

#[derive(Debug, Clone)]
pub struct FieldsIterator {
    message: *const MessageSeq,
    field_i: usize,
    tagged_field: TaggedField,
}
