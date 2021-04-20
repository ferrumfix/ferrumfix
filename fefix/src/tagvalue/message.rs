use super::FieldAccess;
use crate::dtf;
use crate::dtf::DataField;
use crate::fields::fix44 as fields;
use crate::fields::FieldDef;
use crate::models::Error;
use std::collections::HashMap;
use std::ops::Range;

//const DEFAULT_FIELDS_LEN: usize = 64;

/// A repeating group within a [`Message`].
#[derive(Debug)]
pub struct MessageGroup<'a> {
    message: &'a Message<'a>,
    num_in_group_tag_index: usize,
    num_in_group_value: usize,
    ancestry: AncestryTracker,
}

impl<'a> MessageGroup<'a> {
    pub fn len(&self) -> usize {
        self.num_in_group_value
    }

    pub fn entry(&self, index: usize) -> MessageGroupEntry {
        let mut ancestry = self.ancestry;
        ancestry.incr_entry();
        MessageGroupEntry {
            group: self,
            start_index: 0,
            index,
            ancestry,
        }
    }
}

/// A specific [`MessageGroup`] entry.
#[derive(Debug)]
pub struct MessageGroupEntry<'a> {
    group: &'a MessageGroup<'a>,
    start_index: usize,
    index: usize,
    ancestry: AncestryTracker,
}

impl<'a> MessageGroupEntry<'a> {
    pub fn field_ref<'b, T>(
        &'b self,
        field_def: &FieldDef<'b, T>,
    ) -> Option<Result<T, <T as dtf::DataField<'b>>::Error>>
    where
        'b: 'a,
        T: dtf::DataField<'b>,
    {
        let context = Context {
            tag: field_def.tag(),
            ancestry: self.ancestry.ancestry(),
        };
        println!("CONTEXT IS {:?}", context);
        self.group
            .message
            .builder
            .fields_
            .get(&context)
            .map(|field| &self.group.message.bytes[field.range.clone()])
            .map(|bytes| T::deserialize_lossy(bytes))
    }
}

/// FIX message data structure with fast associative and sequential access.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message<'a> {
    bytes: &'a [u8],
    builder: &'a MessageBuilder,
}

impl<'a> Message<'a> {
    pub fn group_ref(&self, tag: u32) -> Option<MessageGroup> {
        let num_in_group_value = self.field_as_i64(tag)? as usize;
        Some(MessageGroup {
            message: self,
            num_in_group_tag_index: 0,
            num_in_group_value,
            ancestry: AncestryTracker::top_level(),
        })
    }

    pub fn field(&self, tag: u32) -> Option<FieldRef> {
        let context = Context::top_level(tag);
        Some(FieldRef {
            message: self.clone(),
            field: self.builder.fields_.get(&context)?,
        })
    }

    pub fn field_ref<'b, T>(
        &'b self,
        field_def: &FieldDef<'b, T>,
    ) -> Option<Result<T, <T as dtf::DataField<'b>>::Error>>
    where
        'b: 'a,
        T: dtf::DataField<'b>,
    {
        let context = Context {
            tag: field_def.tag(),
            ancestry: AncestryTracker::top_level().ancestry(),
        };
        self.builder
            .fields_
            .get(&context)
            .map(|field| &self.bytes[field.range.clone()])
            .map(|bytes| T::deserialize_lossy(bytes))
    }

    pub fn field_raw(&self, tag: u32) -> Option<&[u8]> {
        self.builder
            .fields_
            .get(&Context::top_level(tag))
            .map(|field| &self.bytes[field.range.clone()])
    }

    pub fn field_as_char(&self, tag: u32) -> Option<char> {
        self.builder
            .fields_
            .get(&Context::top_level(tag))
            .map(|field| self.bytes[field.range.start] as char)
    }

    pub fn field_as_bool(&self, tag: u32) -> Option<bool> {
        self.builder
            .fields_
            .get(&Context::top_level(tag))
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

    pub fn field_as_timestamp(&self, tag: u32) -> Option<dtf::Timestamp> {
        let raw = self.field_raw(tag)?;
        dtf::Timestamp::parse(raw)
    }

    pub fn group(&self, _tag: u32) -> Option<GroupRef> {
        None
    }

    pub fn f_msg_type(&self) -> Option<&str> {
        self.field_as_str(fields::MSG_TYPE.tag())
    }

    pub fn f_seq_num(&self) -> Option<u64> {
        self.field_as_i64(fields::MSG_SEQ_NUM.tag())
            .map(|x| x as u64)
    }

    pub fn f_test_indicator(&self) -> Option<bool> {
        self.field_as_bool(fields::TEST_MESSAGE_INDICATOR.tag())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Field {
    i: usize,
    range: Range<usize>,
}

/// Max of 2**16 entries per group.
type GroupEntryId = u16;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AncestryTracker {
    parents: [GroupEntryId; 4],
    depth: usize,
}

impl AncestryTracker {
    pub fn top_level() -> Self {
        Self {
            parents: [0; 4],
            depth: 0,
        }
    }

    pub fn enter_group(&mut self) {
        self.parents[self.depth] = 1;
        self.depth += 1;
    }

    pub fn incr_entry(&mut self) {
        self.parents[self.depth] += 1;
    }

    pub fn leave_group(&mut self) {
        self.parents[self.depth] = 0;
        self.depth -= 1;
    }

    pub fn ancestry(&self) -> Ancestry {
        Ancestry {
            parents: self.parents,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ancestry {
    pub parents: [GroupEntryId; 4],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Context {
    pub tag: u32,
    pub ancestry: Ancestry,
}

impl Context {
    pub fn top_level(tag: u32) -> Self {
        Self {
            tag,
            ancestry: AncestryTracker::top_level().ancestry(),
        }
    }
}

/// A zero-copy, allocation-free builder of [`Message`] instances.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageBuilder {
    fields_: HashMap<Context, Field>,
    //fields: HashMap<u32, Field>,
    //insertion_order: Vec<u32>,
    insertion_order_: Vec<Context>,
    owned_data: Vec<u8>,
    i_first_cell: usize,
    i_last_cell: usize,
    len_end_header: usize,
    len_end_body: usize,
    len_end_trailer: usize,
}

impl MessageBuilder {
    /// Creates a new [`Message`] without any fields.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::MessageBuilder;
    ///
    /// let msg = MessageBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            fields_: HashMap::new(),
            //fields: HashMap::new(),
            //insertion_order: Vec::with_capacity(DEFAULT_FIELDS_LEN),
            insertion_order_: vec![],
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
        // TODO: https://github.com/rust-lang/rust/issues/56431
        self.fields_.clear();
        self.insertion_order_.clear();
        self.i_first_cell = 0;
        self.i_last_cell = 0;
        self.len_end_body = 0;
        self.len_end_header = 0;
        self.len_end_trailer = 0;
    }

    pub fn add_field(&mut self, context: Context, start: usize, len: usize) -> Result<(), Error> {
        let field = Field {
            i: self.insertion_order_.len(),
            range: start..start + len,
        };
        self.fields_.insert(context, field);
        self.insertion_order_.push(context);
        Ok(())
    }

    pub fn build<'a>(&'a self, bytes: &'a [u8]) -> Message<'a> {
        Message {
            bytes,
            builder: self,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldRef<'a> {
    message: Message<'a>,
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

    fn as_date(&self) -> Result<dtf::Date, ()> {
        dtf::Date::deserialize(self.raw()?).map_err(|_| ())
    }

    fn as_time(&self) -> Result<dtf::Time, ()> {
        Ok(dtf::Time::parse(self.raw()?).ok_or(())?)
    }

    fn as_float(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn as_chars(&self) -> Result<dtf::MultipleChars, ()> {
        Ok(dtf::MultipleChars::new(self.raw()?))
    }

    fn as_month_year(&self) -> Result<dtf::MonthYear, ()> {
        let data = self.raw()?;
        dtf::MonthYear::parse(data).ok_or(())
    }
}

#[derive(Debug, Clone)]
pub struct GroupRef<'a> {
    message: &'a Message<'a>,
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
