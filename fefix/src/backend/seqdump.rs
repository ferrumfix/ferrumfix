use super::value as val;
use super::Backend;
use super::FixFieldValue;
use super::*;

#[derive(Debug, Clone)]
pub struct PushyMessage {
    fields: Vec<(u32, FixFieldValue)>,
    iter: FieldsIterator,
}

impl PushyMessage {
    /// Creates a new [`Message`] without any fields.
    pub fn new() -> Self {
        Self::default()
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
}

impl Backend for PushyMessage {
    type Error = ();
    type Iter = FieldsIterator;
    type IterItem = TaggedField;

    fn field(&self, tag: u32) -> Option<&FixFieldValue> {
        self.fields
            .iter()
            .find(|(t, _)| *t == tag)
            .map(|(_, value)| value)
    }

    fn clear(&mut self) {
        self.fields.clear();
    }

    fn len(&self) -> usize {
        self.fields.len()
    }

    fn insert(&mut self, tag: u32, value: FixFieldValue) -> Result<(), Self::Error> {
        self.fields.push((tag, value));
        Ok(())
    }

    fn for_each<E, F>(&self, mut f: F) -> Result<(), E>
    where
        F: FnMut(u32, &FixFieldValue) -> Result<(), E>,
    {
        for (tag, value) in self.fields.iter() {
            f(*tag, value)?;
        }
        Ok(())
    }

    fn iter_fields(&mut self) -> &mut Self::Iter {
        &mut self.iter
    }
}

impl Default for PushyMessage {
    fn default() -> Self {
        Self {
            fields: Vec::new(),
            iter: FieldsIterator {
                message: std::ptr::null(),
                field_i: 0,
                tagged_field: TaggedField {
                    tag: 0,
                    value: FixFieldValue::from(0i64),
                },
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaggedField {
    tag: u32,
    value: FixFieldValue,
}

impl FieldRef<FixFieldValue> for TaggedField {
    fn tag(&self) -> u32 {
        self.tag
    }

    fn value(&self) -> &FixFieldValue {
        &self.value
    }
}

#[derive(Debug, Clone)]
pub struct FieldsIterator {
    message: *const PushyMessage,
    field_i: usize,
    tagged_field: TaggedField,
}

impl StreamIterator for FieldsIterator {
    type Item = TaggedField;

    fn advance(&mut self) {
        self.field_i += 1;
    }

    fn get(&self) -> Option<&Self::Item> {
        let message = unsafe { &*self.message };
        if self.field_i < message.fields.len() {
            Some(&self.tagged_field)
        } else {
            None
        }
    }
}
