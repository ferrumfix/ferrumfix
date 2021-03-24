use super::Error;
use crate::tags;
use std::collections::HashMap;

const DEFAULT_FIELDS_LEN: usize = 64;

type Result<T> = std::result::Result<T, Error>;

/// FIX message data structure with fast associative and sequential access.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixMessageRef<'a> {
    bytes: &'a [u8],
    builder: &'a FixMessageRefBuilder,
}

impl<'a> FixMessageRef<'a> {
    pub fn field(&self, tag: u32) -> Option<&[u8]> {
        self.builder
            .fields
            .get(&tag)
            .map(|(start, end)| &self.bytes[*start..*end])
    }

    pub fn field_as_char(&self, tag: u32) -> Option<char> {
        self.builder
            .fields
            .get(&tag)
            .map(|(start, _end)| self.bytes[*start] as char)
    }

    pub fn field_bool(&self, tag: u32) -> Option<bool> {
        self.builder
            .fields
            .get(&tag)
            .map(|(start, _end)| self.bytes[*start] == b'Y')
    }

    pub fn field_as_i64(&self, tag: u32) -> Option<i64> {
        self.field_as_str(tag).and_then(|s| str::parse::<i64>(s).ok())
    }

    pub fn field_as_str(&self, tag: u32) -> Option<&str> {
        self.field(tag)
            .and_then(|data| std::str::from_utf8(data).ok())
    }

    pub fn f_msg_type(&self) -> Option<&str> {
        self.field_as_str(tags::MSG_TYPE)
    }

    pub fn f_seq_num(&self) -> Option<u64> {
        self.field_as_i64(tags::MSG_SEQ_NUM).map(|x| x as u64)
    }

    pub fn f_test_indicator(&self) -> Option<bool> {
        self.field_bool(tags::TEST_MESSAGE_INDICATOR)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixMessageRefBuilder {
    fields: HashMap<u32, (usize, usize)>,
    insertion_order: Vec<u32>,
    owned_data: Vec<u8>,
    i_first_cell: usize,
    i_last_cell: usize,
    len_end_header: usize,
    len_end_body: usize,
    len_end_trailer: usize,
}

impl FixMessageRefBuilder {
    /// Creates a new [`Message`] without any fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::FixMessageRefBuilder;
    ///
    /// let msg = FixMessageRefBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            insertion_order: Vec::with_capacity(DEFAULT_FIELDS_LEN),
            owned_data: Vec::new(),
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

    /// Adds a field to `self`.
    pub fn add_field(&mut self, tag: u32, start: usize, len: usize) -> Result<()> {
        if self.fields.contains_key(&tag) {
            Err(Error::Duplicate)
        } else {
            self.fields.insert(tag, (start, start + len));
            self.insertion_order.push(tag);
            Ok(())
        }
    }

    pub fn build<'a>(&'a self, bytes: &'a [u8]) -> FixMessageRef<'a> {
        FixMessageRef {
            bytes,
            builder: self,
        }
    }
}
