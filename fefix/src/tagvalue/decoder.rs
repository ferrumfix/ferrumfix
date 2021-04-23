use super::{
    Config, Configure, DecodeError, DecodeError as Error, FieldAccess, Fv, RawDecoder,
    RawDecoderBuffered, RawFrame,
};
use crate::definitions::fix44;
use crate::{dtf, dtf::DataField, DataType, Dictionary, FieldDef};
use crate::{OptError, OptResult, TagU16};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{BuildHasher, Hasher};
use std::ops::Range;

const BEGIN_STRING_OFFSET: usize = 2;

/// FIX message decoder.
#[derive(Debug)]
pub struct Decoder<C = Config>
where
    C: Configure,
{
    dict: Dictionary,
    builder: MessageBuilder,
    raw_decoder: RawDecoder<C>,
    current_group_entry_tag: TagU16,
    remaining_group_entries: u16,
    group_ancestry: AncestryTracker,
    ancestry_id: u64,
    tag_lookup: TagLookup,
    is_beginning_group: bool,
}

impl<C> Decoder<C>
where
    C: Configure,
{
    /// Creates a new [`Decoder`] for the tag-value format. `dict` is used to parse
    /// messages.
    pub fn new(dict: Dictionary) -> Self {
        Self::with_config(dict, C::default())
    }

    pub fn with_config(dict: Dictionary, config: C) -> Self {
        Self {
            dict: dict.clone(),
            builder: MessageBuilder::new(),
            raw_decoder: RawDecoder::with_config(config),
            current_group_entry_tag: TagU16::new(1).unwrap(),
            remaining_group_entries: 0,
            group_ancestry: AncestryTracker::top_level(),
            ancestry_id: 0,
            tag_lookup: TagLookup::from_dict(&dict),
            is_beginning_group: false,
        }
    }

    /// Returns an immutable reference to the [`Configure`] used by `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::{Config, Configure, Decoder};
    /// use fefix::{AppVersion, Dictionary};
    ///
    /// let dict = Dictionary::from_version(AppVersion::Fix44);
    /// let decoder = Decoder::<Config>::new(dict);
    /// assert_eq!(decoder.config().separator(), 0x1);
    /// ```
    #[inline]
    pub fn config(&self) -> &C {
        self.raw_decoder.config()
    }

    /// Returns a mutable reference to the [`Configure`] used by `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::{Config, Configure, Decoder};
    /// use fefix::{AppVersion, Dictionary};
    ///
    /// let dict = Dictionary::from_version(AppVersion::Fix44);
    /// let decoder = &mut Decoder::<Config>::new(dict);
    /// decoder.config_mut().set_separator(b'|');
    /// assert_eq!(decoder.config().separator(), b'|');
    /// ```
    #[inline]
    pub fn config_mut(&mut self) -> &mut C {
        self.raw_decoder.config_mut()
    }

    /// Turns `self` into a [`DecoderBuffered`] by allocating an internal buffer.
    pub fn buffered(self) -> DecoderBuffered<C> {
        let raw_decoder = self.raw_decoder.clone().buffered();
        DecoderBuffered {
            decoder: self,
            raw_decoder,
        }
    }

    /// Decodes `data` and returns an immutable reference to the obtained
    /// message.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::{Config, Decoder};
    /// use fefix::tags::fix42 as tags;
    /// use fefix::{AppVersion, Dictionary, FixFieldAccess};
    ///
    /// let dict = Dictionary::from_version(AppVersion::Fix44);
    /// let decoder = &mut Decoder::<Config>::new(dict);
    /// let data = b"8=FIX.4.2\x019=42\x0135=0\x0149=A\x0156=B\x0134=12\x0152=20100304-07:59:30\x0110=185\x01";
    /// let message = decoder.decode(data).unwrap();
    /// assert_eq!(
    ///     message.field_str(tags::SENDER_COMP_ID),
    ///     Some("A")
    /// );
    /// ```
    #[inline]
    pub fn decode<'a>(&'a mut self, bytes: &'a [u8]) -> Result<Message<'a>, DecodeError> {
        let frame = self.raw_decoder.decode(bytes)?;
        self.from_frame(frame)
    }

    fn from_frame<'a>(&'a mut self, frame: RawFrame<'a>) -> Result<Message<'a>, DecodeError> {
        self.builder.clear();
        let bytes = frame.as_bytes();
        let mut tag_num = 0u16;
        let mut state_is_tag = true;
        let mut i_sep;
        let mut i_equal_sign = 0usize;
        self.builder
            .add_field(
                Context::top_level(fix44::BEGIN_STRING.tag()),
                BEGIN_STRING_OFFSET,
                frame.begin_string().len(),
                self.config().should_decode_associative(),
                self.config().should_decode_sequential(),
            )
            .unwrap();
        for i in 0..frame.payload().len() {
            let byte = frame.payload()[i];
            if byte == b'=' {
                i_equal_sign = i;
                state_is_tag = false;
            } else if byte == self.config().separator() {
                i_sep = i;
                state_is_tag = true;
                let start = frame.payload_offset() + i_equal_sign + 1;
                let len = i_sep - i_equal_sign - 1;
                self.store_field(
                    TagU16::new(tag_num).unwrap(),
                    &bytes[start..start + len],
                    start,
                    len,
                    bytes,
                );
                tag_num = 0;
            } else if state_is_tag {
                tag_num = (tag_num * 10 + (byte - b'0') as u16) as u16;
            }
        }
        Ok(self.builder.build(bytes))
    }

    fn store_field(&mut self, tag: TagU16, content: &[u8], start: usize, len: usize, bytes: &[u8]) {
        let msg = self.builder.build(bytes);
        let _msg_type = msg.fv_raw(fix44::MSG_TYPE).unwrap_or(b"");
        let config_assoc = self.config().should_decode_associative();
        let config_seq = self.config().should_decode_sequential();
        let entry = self.tag_lookup.lookup(tag).unwrap();
        if self.is_beginning_group {
            self.current_group_entry_tag = tag;
            self.is_beginning_group = false;
        }
        if tag == self.current_group_entry_tag {
            if self.ancestry_id & 0xff <= 1 {
                self.ancestry_id >>= 16;
            } else {
                self.ancestry_id -= 1;
            }
            if self.remaining_group_entries <= 1 {
                self.group_ancestry.leave_group();
            }
        }
        let context = Context {
            tag,
            ancestry: Ancestry::from_u64(self.ancestry_id),
        };
        self.builder
            .add_field(context, start, len, config_assoc, config_seq)
            .unwrap();
        if entry.data_type() == DataType::NumInGroup {
            self.is_beginning_group = true;
            let s = std::str::from_utf8(content).unwrap();
            let entries_count = str::parse::<u16>(s).unwrap();
            self.current_group_entry_tag = entry.first_tag_of_group;
            self.ancestry_id = (self.ancestry_id << 16) + entries_count as u64;
            self.group_ancestry.enter_group();
            self.remaining_group_entries = entries_count;
        }
    }
}

/// A (de)serializer for the classic FIX tag-value encoding.
///
/// The FIX tag-value encoding is designed to be both human-readable and easy for
/// machines to parse.
///
/// Please reach out to the FIX official documentation[^1][^2] for more information.
///
/// [^1]: [FIX TagValue Encoding: Online reference.](https://www.fixtrading.org/standards/tagvalue-online)
///
/// [^2]: [FIX TagValue Encoding: PDF.](https://www.fixtrading.org/standards/tagvalue/)
#[derive(Debug)]
pub struct DecoderBuffered<C = Config>
where
    C: Configure,
{
    decoder: Decoder<C>,
    raw_decoder: RawDecoderBuffered<C>,
}

impl<C> DecoderBuffered<C>
where
    C: Configure,
{
    /// Returns an immutable reference to the [`Configure`] used by `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::{Config, Configure, Decoder};
    /// use fefix::{AppVersion, Dictionary};
    ///
    /// let dict = Dictionary::from_version(AppVersion::Fix44);
    /// let decoder = Decoder::<Config>::new(dict);
    /// assert_eq!(decoder.config().separator(), 0x1);
    /// ```
    #[inline]
    pub fn config(&self) -> &C {
        self.decoder.config()
    }

    /// Returns a mutable reference to the [`Configure`] used by `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::{Config, Configure, Decoder};
    /// use fefix::{AppVersion, Dictionary};
    ///
    /// let dict = Dictionary::from_version(AppVersion::Fix44);
    /// let decoder = &mut Decoder::<Config>::new(dict);
    /// decoder.config_mut().set_separator(b'|');
    /// assert_eq!(decoder.config().separator(), b'|');
    /// ```
    #[inline]
    pub fn config_mut(&mut self) -> &mut C {
        self.decoder.config_mut()
    }

    #[inline]
    pub fn supply_buffer(&mut self) -> &mut [u8] {
        self.raw_decoder.supply_buffer()
    }

    #[inline]
    pub fn current_message(&mut self) -> Result<Option<Message>, DecodeError> {
        match self.raw_decoder.current_frame() {
            Ok(Some(frame)) => self.decoder.from_frame(frame).map(|msg| Some(msg)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn message(&self) -> Message {
        self.decoder.builder.build(
            self.raw_decoder
                .current_frame()
                .unwrap()
                .unwrap()
                .as_bytes(),
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TagLookupEntry {
    data_type: DataType,
    first_tag_of_group: TagU16,
}

impl TagLookupEntry {
    pub fn data_type(&self) -> DataType {
        self.data_type
    }
}

pub struct TagHasher {
    hash: u64,
}

impl Hasher for TagHasher {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes.iter().copied() {
            self.hash = self
                .hash
                .wrapping_mul(10)
                .wrapping_add(byte.wrapping_sub(b'0') as u64);
        }
    }
}

#[derive(Clone)]
pub struct TagHashBuilder {}

impl BuildHasher for TagHashBuilder {
    type Hasher = TagHasher;

    fn build_hasher(&self) -> Self::Hasher {
        TagHasher { hash: 0 }
    }
}

#[derive(Debug)]
pub struct TagLookup {
    current_dict: Dictionary,
    entries: HashMap<TagU16, TagLookupEntry, TagHashBuilder>,
}

impl TagLookup {
    pub fn from_dict(dict: &Dictionary) -> Self {
        let mut entries = HashMap::with_hasher(TagHashBuilder {});
        for field in dict.iter_fields() {
            entries.insert(
                field.tag(),
                TagLookupEntry {
                    data_type: field.data_type().basetype(),
                    first_tag_of_group: TagU16::new(1).unwrap(),
                },
            );
        }
        Self {
            current_dict: dict.clone(),
            entries,
        }
    }

    pub fn lookup(&mut self, tag: TagU16) -> Option<&TagLookupEntry> {
        self.entries.get(&tag)
    }
}

/// A repeating group within a [`Message`].
#[derive(Debug)]
pub struct MessageGroup<'a> {
    message: &'a Message<'a>,
    num_in_group_tag_index: usize,
    num_in_group_value: usize,
    ancestry_id: u64,
}

impl<'a> MessageGroup<'a> {
    pub fn len(&self) -> usize {
        self.num_in_group_value
    }

    pub fn entry(&self, index: usize) -> MessageGroupEntry {
        MessageGroupEntry {
            group: self,
            start_index: 0,
            index,
            ancestry_id: (self.ancestry_id << 16) + (index as u64 + 1),
        }
    }
}

/// A specific [`MessageGroup`] entry.
#[derive(Debug)]
pub struct MessageGroupEntry<'a> {
    group: &'a MessageGroup<'a>,
    start_index: usize,
    index: usize,
    ancestry_id: u64,
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
            ancestry: Ancestry::from_u64(self.ancestry_id),
        };
        self.group
            .message
            .builder
            .fields
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
    pub fn group_ref(&self, tag: TagU16) -> Option<MessageGroup> {
        let num_in_group_value = self.fv_with_key::<usize>(&tag).ok()?;
        let context = Context {
            tag,
            ancestry: Ancestry::from_u64(0),
        };
        let field_i = self.builder.fields.get(&context)?.i;
        Some(MessageGroup {
            message: self,
            num_in_group_tag_index: field_i,
            num_in_group_value,
            ancestry_id: 0,
        })
    }

    //pub fn field(&self, tag: u32) -> Option<FieldRef> {
    //    let context = Context::top_level(tag);
    //    Some(FieldRef {
    //        message: self.clone(),
    //        field: self.builder.fields.get(&context)?,
    //    })
    //}

    fn field_ref_simple<'b, T>(&'b self, field_def: &FieldDef<'b, T>) -> OptResult<T, T::Error>
    where
        'b: 'a,
        T: dtf::DataField<'b>,
    {
        let context = Context {
            tag: field_def.tag(),
            ancestry: AncestryTracker::top_level().ancestry(),
        };
        self.builder
            .fields
            .get(&context)
            .ok_or(OptError::None)
            .map(|field| &self.bytes[field.range.clone()])
            .and_then(|bytes| T::deserialize_lossy(bytes).map_err(|err| OptError::Other(err)))
    }

    pub fn field_ref<'b, T, S>(&'b self, field_def: &FieldDef<'b, T>) -> OptResult<S, S::Error>
    where
        'b: 'a,
        T: dtf::DataField<'b>,
        S: dtf::SubDataField<'b, T>,
    {
        match self.field_ref_simple(field_def) {
            Ok(dtf) => S::convert(dtf).map_err(|err| OptError::Other(err)),
            Err(OptError::Other(err)) => Err(OptError::Other(S::Error::from(err))),
            Err(OptError::None) => Err(OptError::None),
        }
    }

    pub fn field_ref_opt<'b, T>(
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
            .fields
            .get(&context)
            .map(|field| &self.bytes[field.range.clone()])
            .map(|bytes| T::deserialize_lossy(bytes))
    }

    pub fn field_raw(&self, tag: TagU16) -> Option<&[u8]> {
        self.builder
            .fields
            .get(&Context::top_level(tag))
            .map(|field| &self.bytes[field.range.clone()])
    }
}

impl<'a> Fv<'a> for Message<'a> {
    type Key = TagU16;

    fn fv_raw_with_key<'b>(&'b self, key: &Self::Key) -> Option<&'b [u8]> {
        self.field_raw(*key)
    }

    fn fv_raw<'b, T>(&'b self, field: &FieldDef<'b, T>) -> Option<&'b [u8]>
    where
        'b: 'a,
        T: dtf::DataField<'b>,
    {
        self.fv_raw_with_key(&field.tag())
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
    id: u64,
    parents: [GroupEntryId; 4],
    depth: usize,
}

impl AncestryTracker {
    pub fn top_level() -> Self {
        Self {
            id: 0,
            parents: [0; 4],
            depth: 0,
        }
    }

    pub fn enter_group(&mut self) {
        self.id = (self.id << 16) | 1;
        self.parents[self.depth] = 1;
        self.depth += 1;
    }

    pub fn leave_group(&mut self) {
        self.id = self.id >> 16;
        self.parents[self.depth] = 0;
        self.depth = self.depth.wrapping_sub(1);
    }

    pub fn ancestry(&self) -> Ancestry {
        Ancestry {
            id: ((self.parents[0] as u64) << 48)
                + ((self.parents[1] as u64) << 32)
                + ((self.parents[2] as u64) << 16)
                + self.parents[3] as u64,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Ancestry {
    pub id: u64,
}

impl Ancestry {
    pub fn from_u64(id: u64) -> Self {
        Self { id }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Context {
    pub tag: TagU16,
    pub ancestry: Ancestry,
}

impl Context {
    pub fn top_level(tag: TagU16) -> Self {
        Self {
            tag,
            ancestry: AncestryTracker::top_level().ancestry(),
        }
    }
}

/// A zero-copy, allocation-free builder of [`Message`] instances.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageBuilder {
    fields: HashMap<Context, Field, TagHashBuilder>,
    insertion_order: Vec<Context>,
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
            fields: HashMap::with_capacity_and_hasher(20, TagHashBuilder {}),
            insertion_order: vec![],
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
        self.fields.clear();
        self.insertion_order.clear();
        self.i_first_cell = 0;
        self.i_last_cell = 0;
        self.len_end_body = 0;
        self.len_end_header = 0;
        self.len_end_trailer = 0;
    }

    pub fn add_field(
        &mut self,
        context: Context,
        start: usize,
        len: usize,
        assoc: bool,
        seq: bool,
    ) -> Result<(), Error> {
        let field = Field {
            i: self.insertion_order.len(),
            range: start..start + len,
        };
        if assoc {
            self.fields.insert(context, field);
        }
        if seq {
            self.insertion_order.push(context);
        }
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
        dtf::Time::deserialize(self.raw()?).map_err(|_| ())
    }

    fn as_float(&self) -> Result<(), ()> {
        unimplemented!()
    }

    fn as_chars(&self) -> Result<dtf::MultipleChars, ()> {
        Ok(dtf::MultipleChars::new(self.raw()?))
    }

    fn as_month_year(&self) -> Result<dtf::MonthYear, ()> {
        let data = self.raw()?;
        dtf::MonthYear::deserialize(data).map_err(|_| ())
    }
}

#[derive(Debug, Clone)]
pub struct GroupRef<'a> {
    message: &'a Message<'a>,
    len: usize,
    field_len: u32,
}

#[derive(Debug, Clone)]
pub struct GroupRefIter<'a> {
    group: &'a GroupRef<'a>,
    i: usize,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{definitions::fix44, tagvalue::Config, AppVersion};

    // Use http://www.validfix.com/fix-analyzer.html for testing.

    fn with_soh(msg: &str) -> String {
        msg.split("|").collect::<Vec<&str>>().join("\x01")
    }

    fn decoder() -> Decoder<Config> {
        let dict = Dictionary::from_version(AppVersion::Fix44);
        Decoder::with_config(dict, Config::default().with_separator(b'|'))
    }

    #[test]
    fn can_parse_simple_message() {
        let message = "8=FIX.4.2|9=40|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=091|";
        let decoder = &mut decoder();
        let result = decoder.decode(message.as_bytes());
        assert!(result.is_ok());
    }

    const RANDOM_MESSAGES: &[&str] = &[
        "8=FIX.4.2|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=185|",
        "8=FIX.4.2|9=97|35=6|49=BKR|56=IM|34=14|52=20100204-09:18:42|23=115685|28=N|55=SPMI.MI|54=2|44=2200.75|27=S|25=H|10=248|",
        "8=FIX.4.4|9=117|35=AD|34=2|49=A|50=1|52=20100219-14:33:32.258|56=B|57=M|263=1|568=1|569=0|580=1|75=20100218|60=20100218-00:00:00.000|10=202|",
        "8=FIX.4.4|9=94|35=3|34=214|49=A|50=U1|52=20100304-09:42:23.130|56=AB|128=B1|45=176|58=txt|371=15|372=X|373=1|10=058|",
        "8=FIX.4.4|9=70|35=4|49=A|56=XYZ|34=129|52=20100302-19:38:21|43=Y|57=LOL|123=Y|36=175|10=192|",
        "8=FIX.4.4|9=122|35=D|34=215|49=CLIENT12|52=20100225-19:41:57.316|56=B|1=Marcel|11=13346|21=1|40=2|44=5|54=1|59=0|60=20100225-19:39:52.020|10=072|",
        "8=FIX.4.2|9=196|35=X|49=A|56=B|34=12|52=20100318-03:21:11.364|262=A|268=2|279=0|269=0|278=BID|55=EUR/USD|270=1.37215|15=EUR|271=2500000|346=1|279=0|269=1|278=OFFER|55=EUR/USD|270=1.37224|15=EUR|271=2503200|346=1|10=171|",
    ];

    #[test]
    fn skip_checksum_verification() {
        let message = "8=FIX.FOOBAR|9=5|35=0|10=000|";
        let decoder = &mut decoder();
        decoder.config_mut().set_verify_checksum(false);
        let result = decoder.decode(message.as_bytes());
        assert!(result.is_ok());
    }

    #[test]
    fn repeating_group_entries() {
        let bytes = b"8=FIX.4.2|9=196|35=X|49=A|56=B|34=12|52=20100318-03:21:11.364|262=A|268=2|279=0|269=0|278=BID|55=EUR/USD|270=1.37215|15=EUR|271=2500000|346=1|279=0|269=1|278=OFFER|55=EUR/USD|270=1.37224|15=EUR|271=2503200|346=1|10=171|";
        let decoder = &mut decoder();
        decoder.config_mut().set_separator(b'|');
        decoder.config_mut().set_verify_checksum(false);
        let message = decoder.decode(bytes).unwrap();
        let group = message.group_ref(TagU16::new(268).unwrap()).unwrap();
        assert_eq!(group.len(), 2);
        assert_eq!(
            group
                .entry(0)
                .field_ref(fix44::MD_ENTRY_ID)
                .unwrap()
                .unwrap(),
            b"BID"
        );
    }

    #[test]
    fn no_skip_checksum_verification() {
        let message = "8=FIX.FOOBAR|9=5|35=0|10=000|";
        let mut codec = Decoder::<Config>::new(Dictionary::from_version(AppVersion::Fix44));
        codec.config_mut().set_separator(b'|');
        codec.config_mut().set_verify_checksum(true);
        let result = codec.decode(message.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn assortment_of_random_messages_is_ok() {
        for msg_with_vertical_bar in RANDOM_MESSAGES {
            let message = with_soh(msg_with_vertical_bar);
            let mut codec = decoder();
            codec.config_mut().set_separator(0x1);
            let result = codec.decode(message.as_bytes());
            result.unwrap();
        }
    }

    #[test]
    fn heartbeat_message_fields_are_ok() {
        let mut codec = decoder();
        codec.config_mut().set_verify_checksum(false);
        let message = codec.decode(&mut RANDOM_MESSAGES[0].as_bytes()).unwrap();
        assert_eq!(
            message.field_ref(fix44::MSG_TYPE),
            Ok(fix44::MsgType::Heartbeat)
        );
        assert_eq!(
            message.field_raw(TagU16::new(8).unwrap()),
            Some(b"FIX.4.2" as &[u8])
        );
        assert_eq!(
            message.field_raw(TagU16::new(35).unwrap()),
            Some(b"0" as &[u8]),
        );
    }

    #[test]
    fn message_without_final_separator() {
        let message = "8=FIX.4.4|9=122|35=D|34=215|49=CLIENT12|52=20100225-19:41:57.316|56=B|1=Marcel|11=13346|21=1|40=2|44=5|54=1|59=0|60=20100225-19:39:52.020|10=072";
        let mut config = Config::default();
        config.set_separator(b'|');
        let mut codec = Decoder::with_config(Dictionary::from_version(AppVersion::Fix44), config);
        let result = codec.decode(message.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn message_must_end_with_separator() {
        let msg = "8=FIX.4.2|9=41|35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|10=127";
        let mut codec = decoder();
        let result = codec.decode(&mut msg.as_bytes());
        assert_eq!(result, Err(DecodeError::Invalid));
    }

    #[test]
    fn message_without_checksum() {
        let msg = "8=FIX.4.4|9=37|35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|";
        let mut codec = decoder();
        let result = codec.decode(&mut msg.as_bytes());
        assert_eq!(result, Err(DecodeError::Invalid));
    }

    #[test]
    fn message_without_standard_header() {
        let msg = "35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|10=000|";
        let mut codec = decoder();
        let result = codec.decode(&mut msg.as_bytes());
        assert_eq!(result, Err(DecodeError::Invalid));
    }

    #[test]
    fn detect_incorrect_checksum() {
        let msg = "8=FIX.4.2|9=43|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=146|";
        let mut codec = decoder();
        let result = codec.decode(&mut msg.as_bytes());
        assert_eq!(result, Err(DecodeError::Invalid));
    }
}
