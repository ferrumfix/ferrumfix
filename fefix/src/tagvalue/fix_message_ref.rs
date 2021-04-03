use crate::models::Error;
use crate::tags;
use crate::{DtfDate, DtfMonthYear, DtfMulCharIter, DtfTime};
use std::{collections::HashMap, ops::Range};

const DEFAULT_FIELDS_LEN: usize = 64;

pub trait FieldAccess<E> {
    fn raw(&self) -> Result<&[u8], E>;

    fn as_char(&self) -> Result<u8, E>;

    fn as_chars(&self) -> Result<DtfMulCharIter<b' '>, E>;

    fn as_bool(&self) -> Result<bool, E>;

    fn as_i64(&self) -> Result<i64, E>;

    fn as_u64(&self) -> Result<u64, E>;

    fn as_timestamp(&self) -> Result<i64, E>;

    fn as_date(&self) -> Result<DtfDate, E>;

    fn as_time(&self) -> Result<DtfTime, E>;

    fn as_float(&self) -> Result<(), E>;

    fn as_month_year(&self) -> Result<DtfMonthYear, E>;
}

/// FIX message data structure with fast associative and sequential access.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixMessageRef<'a> {
    bytes: &'a [u8],
    builder: &'a FixMessageRefBuilder,
}

impl<'a> FixMessageRef<'a> {
    pub fn field(&self, tag: u32) -> Option<FieldRef> {
        Some(FieldRef {
            message: self.clone(),
            field: self.builder.fields.get(&tag)?,
        })
    }

    pub fn field_raw(&self, tag: u32) -> Option<&[u8]> {
        self.builder
            .fields
            .get(&tag)
            .map(|field| &self.bytes[field.range.clone()])
    }

    pub fn field_as_char(&self, tag: u32) -> Option<char> {
        self.builder
            .fields
            .get(&tag)
            .map(|field| self.bytes[field.range.start] as char)
    }

    pub fn field_as_bool(&self, tag: u32) -> Option<bool> {
        self.builder
            .fields
            .get(&tag)
            .map(|field| self.bytes[field.range.start] == b'Y')
    }

    pub fn field_as_i64(&self, tag: u32) -> Option<i64> {
        self.field_as_str(tag)
            .and_then(|s| str::parse::<i64>(s).ok())
    }

    pub fn field_as_str(&self, tag: u32) -> Option<&str> {
        self.field_raw(tag)
            .and_then(|data| std::str::from_utf8(data).ok())
    }

    pub fn field_as_chrono_dt(&self, tag: u32) -> Option<chrono::DateTime<chrono::Utc>> {
        let s = self.field_as_str(tag)?;
        let naive = chrono::NaiveDateTime::parse_from_str(s, "%Y%m%d-%H:%M:%S.%.3f").ok()?;
        let dt = chrono::DateTime::<chrono::Utc>::from_utc(naive, chrono::Utc);
        Some(dt)
    }

    pub fn group(&self, _tag: u32) -> Option<GroupRef> {
        None
    }

    pub fn f_msg_type(&self) -> Option<&str> {
        self.field_as_str(tags::MSG_TYPE)
    }

    pub fn f_seq_num(&self) -> Option<u64> {
        self.field_as_i64(tags::MSG_SEQ_NUM).map(|x| x as u64)
    }

    pub fn f_test_indicator(&self) -> Option<bool> {
        self.field_as_bool(tags::TEST_MESSAGE_INDICATOR)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Field {
    i: usize,
    range: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixMessageRefBuilder {
    fields: HashMap<u32, Field>,
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
    pub fn add_field(&mut self, tag: u32, start: usize, len: usize) -> Result<(), Error> {
        if self.fields.contains_key(&tag) {
            Err(Error::Duplicate)
        } else {
            let field = Field {
                i: self.insertion_order.len(),
                range: start..start + len,
            };
            self.fields.insert(tag, field);
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

#[derive(Debug, Clone)]
pub struct FieldRef<'a> {
    message: FixMessageRef<'a>,
    field: &'a Field,
}

impl<'a> FieldAccess<()> for FieldRef<'a> {
    fn raw(&self) -> Result<&[u8], ()> {
        Ok(&self.message.bytes[self.field.range.clone()])
    }

    fn as_char(&self) -> Result<u8, ()> {
        Ok(self.message.bytes[self.field.range.start])
    }

    fn as_bool(&self) -> Result<bool, ()> {
        Ok(self.message.bytes[self.field.range.start] == b'Y')
    }

    fn as_i64(&self) -> Result<i64, ()> {
        let data = self.raw()?;
        if data.is_empty() {
            return Err(());
        }
        let mut num = 0i64;
        for byte in data.iter().copied().rev() {
            if byte == b'-' {
                num = -num;
            } else {
                num = num * 10 + byte.wrapping_sub(b'0') as i64;
            }
        }
        Ok(num)
    }

    fn as_u64(&self) -> Result<u64, ()> {
        let data = self.raw()?;
        let mut num = 0u64;
        for byte in data.iter().rev() {
            num = num * 10 + byte.wrapping_sub(b'0') as u64;
        }
        Ok(num)
    }

    fn as_timestamp(&self) -> Result<i64, ()> {
        unimplemented!()
    }

    fn as_date(&self) -> Result<DtfDate, ()> {
        Ok(DtfDate::parse(self.raw()?).ok_or(())?)
    }

    fn as_time(&self) -> Result<DtfTime, ()> {
        Ok(DtfTime::parse(self.raw()?).ok_or(())?)
    }

    fn as_float(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn as_chars(&self) -> Result<DtfMulCharIter<b' '>, ()> {
        Ok(DtfMulCharIter::new(self.raw()?))
    }

    fn as_month_year(&self) -> Result<DtfMonthYear, ()> {
        let data = self.raw()?;
        DtfMonthYear::parse(data).ok_or(())
    }
}

#[derive(Debug, Clone)]
pub struct GroupRef<'a> {
    message: &'a FixMessageRef<'a>,
    len: usize,
    field_len: u32,
}

impl<'a> GroupRef<'a> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> GroupRefIter {
        GroupRefIter { group: self, i: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct GroupRefIter<'a> {
    group: &'a GroupRef<'a>,
    i: usize,
}
