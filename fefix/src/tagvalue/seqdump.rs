use crate::backend::field_value as val;
use crate::tagvalue::FixFieldValue;

#[derive(Debug, Clone)]
pub struct PushyMessage {
    begin_string: Vec<u8>,
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
        self.add_field(tag, FixFieldValue::string(value.into().as_bytes()).unwrap())
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
            Some(FixFieldValue::Atom(val::FieldValue::String(s))) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn seq_num(&self) -> Option<u64> {
        match self.get_field(34u32) {
            Some(FixFieldValue::Atom(val::FieldValue::Int(val::Int(n)))) => Some(*n as u64),
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

    pub fn for_each<E, F>(&self, mut f: F) -> Result<(), E>
    where
        F: FnMut(u32, &FixFieldValue) -> Result<(), E>,
    {
        for (tag, value) in self.fields.iter() {
            f(*tag, value)?;
        }
        Ok(())
    }
}

impl Default for PushyMessage {
    fn default() -> Self {
        Self {
            begin_string: Vec::new(),
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

#[derive(Debug, Clone)]
pub struct FieldsIterator {
    message: *const PushyMessage,
    field_i: usize,
    tagged_field: TaggedField,
}
