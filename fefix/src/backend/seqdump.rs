use super::FixFieldValue;
use super::value as val;
use super::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PushyMessage {
    fields: Vec<(u32, FixFieldValue)>,
}

impl PushyMessage {
    /// Creates a new [`Message`] without any fields.
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }

    /// Adds a field to `self`.
    pub fn add_field<K: Into<u32>>(&mut self, tag: K, value: FixFieldValue) {
        self.fields.push((tag.into(), value));
    }

    /// Adds a string field to `self`.
    pub fn add_str<K: Into<u32>, S: Into<String>>(&mut self, tag: K, value: S) {
        self.add_field(tag, FixFieldValue::String(value.into()))
    }

    /// Adds an integer field to `self`.
    pub fn add_int<K: Into<u32>>(&mut self, tag: K, value: i64) {
        self.add_field(tag, FixFieldValue::from(value))
    }

    pub fn get_field<K: Into<u32>>(&self, tag: K) -> Option<&FixFieldValue> {
        let tag = tag.into();
        let index = self.fields.iter().position(|(t, _)| *t == tag);
        index.map(|i| &self.fields[i].1)
    }

    pub fn msg_type(&self) -> Option<&str> {
        match self.get_field(35u32) {
            Some(FixFieldValue::String(s)) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn seq_num(&self) -> Option<u64> {
        match self.get_field(34u32) {
            Some(FixFieldValue::Atom(val::Atomic::Int(val::Int(n)))) => Some(*n as u64),
            _ => None,
        }
    }

    pub fn test_indicator(&self) -> Option<bool> {
        let y = FixFieldValue::from('Y');
        let n = FixFieldValue::from('N');
        match self.get_field(464u32) {
            Some(f) if *f == y => Some(true),
            Some(f) if *f == n => Some(false),
            _ => Some(false),
        }
    }

    pub fn iter_fields<'a>(&'a self) -> impl ReadFieldsSeq + 'a {
        FieldsIterator {
            message: &self,
            i: 0,
        }
    }
}

impl WriteFields for PushyMessage {
    fn set_field(&mut self, msg_type: u32, val: FixFieldValue) {
        PushyMessage::add_field(self, msg_type, val)
    }
}

impl ReadFields for PushyMessage {
    fn get_field(&self, msg_type: u32) -> Option<&FixFieldValue> {
        PushyMessage::get_field(self, msg_type)
    }
}

struct FieldsIterator<'a> {
    message: &'a PushyMessage,
    i: usize,
}

impl<'a> ReadFieldsSeq for FieldsIterator<'a> {
    fn next(&mut self) -> Option<(u32, &FixFieldValue)> {
        if self.message.fields.len() < self.i {
            let elem = &self.message.fields[self.i];
            Some((elem.0, &elem.1))
        } else {
            None
        }
    }
}
