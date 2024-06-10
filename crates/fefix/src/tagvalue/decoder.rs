use super::{Config, DecodeError, RawDecoder, RawDecoderStreaming, RawFrame};
use crate::dict::{FixDatatype, IsFieldDefinition};
use crate::{
    Buffer, Dictionary, FieldMap, FieldType, FieldValueError, GetConfig, RepeatingGroup,
    StreamingDecoder, TagU32,
};
use nohash_hasher::IntMap;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Debug;
use std::iter::FusedIterator;
use std::marker::PhantomData;

/// Univocally locates a tag within a FIX message, even with nested groups.
///
/// Typically, every FIX tag is guaranteed to be unique within a single FIX
/// message. Repeating groups, however, break this promise and allow *multiple*
/// values with the same tag, each in a different *group entry*. This means that
/// a FIX message is a tree rather than an associative array. [`FieldLocator`]
/// generates unique identifiers for tags both outside and within groups, which
/// allows for random (i.e. non-sequential) reads on a FIX message.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct FieldLocator {
    pub tag: TagU32,
    pub context: FieldLocatorContext,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum FieldLocatorContext {
    TopLevel,
    WithinGroup {
        index_of_group_tag: u32,
        entry_index: u32,
    },
}

// Number of bytes before the start of the `BeginString` field:
//
//   ~~
//   8=FIX.4.2|...
const BEGIN_STRING_OFFSET: usize = 2;

/// FIX message decoder.
///
/// One should create a [`Decoder`] per stream of FIX messages.
#[derive(Debug)]
pub struct Decoder {
    builder: MessageBuilder<'static>,
    raw_decoder: RawDecoder,
    tag_lookup: IntMap<u32, FixDatatype>,
}

impl Decoder {
    /// Creates a new [`Decoder`] for the tag-value format. `dict` is used to parse
    /// messages.
    pub fn new(dict: Dictionary) -> Self {
        Self {
            builder: MessageBuilder::default(),
            raw_decoder: RawDecoder::default(),
            tag_lookup: dict
                .fields()
                .iter()
                .filter_map(|field| {
                    let mut fix_type = field.data_type().basetype();
                    if field.is_num_in_group() {
                        fix_type = FixDatatype::NumInGroup;
                    }

                    if fix_type == FixDatatype::Length || fix_type == FixDatatype::NumInGroup {
                        Some((field.tag().get(), fix_type))
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }

    /// Adds a [`Buffer`] to `self`, turning it into a [`StreamingDecoder`].
    pub fn streaming<B>(self, buffer: B) -> DecoderStreaming<B>
    where
        B: Buffer,
    {
        let raw_decoder = self.raw_decoder.clone().streaming(buffer);

        DecoderStreaming {
            decoder: self,
            raw_decoder,
            is_ready: false,
        }
    }

    /// Decodes `data` and returns an immutable reference to the obtained
    /// message.
    ///
    /// # Examples
    ///
    /// ```no_run FIXME
    /// use fefix::tagvalue::{Config, Decoder};
    /// use fefix::prelude::*;
    ///
    /// let dict = Dictionary::fix44();
    /// let mut decoder = Decoder::new(dict);
    /// decoder.config_mut().separator = b'|';
    /// let data = b"8=FIX.4.4|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=185|";
    /// let message = decoder.decode(data).unwrap();
    /// assert_eq!(message.get(fix44::SENDER_COMP_ID), Ok("A"));
    /// ```
    pub fn decode<T>(&mut self, bytes: T) -> Result<Message<'_, T>, DecodeError>
    where
        T: AsRef<[u8]>,
    {
        let frame = self.raw_decoder.decode(bytes)?;
        self.from_frame(frame)
    }

    fn message_builder_mut(&mut self) -> &mut MessageBuilder<'_> {
        unsafe { std::mem::transmute(&mut self.builder) }
    }

    fn from_frame<T>(&mut self, frame: RawFrame<T>) -> Result<Message<'_, T>, DecodeError>
    where
        T: AsRef<[u8]>,
    {
        self.builder.clear();
        self.message_builder_mut().bytes = frame.as_bytes();
        let separator = self.config().separator;
        let payload = frame.payload();
        self.store_field(
            TagU32::new(8).unwrap(),
            frame.as_bytes(),
            BEGIN_STRING_OFFSET,
            frame.begin_string().len(),
        )?;
        let mut i = 0;
        while i < payload.len() {
            let index_of_next_equal_sign = {
                let i_eq = payload[i..]
                    .iter()
                    .copied()
                    .position(|byte| byte == b'=')
                    .map(|pos| pos + i);
                if i_eq.is_none() {
                    break;
                }
                i_eq.unwrap()
            };
            let field_value_len = if let Some(len) = self.builder.state.data_field_length {
                self.builder.state.data_field_length = None;
                len
            } else {
                let len = payload[index_of_next_equal_sign + 1..]
                    .iter()
                    .copied()
                    .position(|byte| byte == separator);
                if len.is_none() {
                    break;
                }
                len.unwrap()
            };
            let tag_num = {
                let mut tag = 0u32;
                for byte in payload[i..index_of_next_equal_sign].iter().copied() {
                    tag = tag * 10 + (byte as u32 - b'0' as u32);
                }
                if let Some(tag) = TagU32::new(tag) {
                    tag
                } else {
                    break;
                }
            };
            self.store_field(
                tag_num,
                frame.payload(),
                index_of_next_equal_sign + 1,
                field_value_len,
            )?;
            // Equal sign                ~~~
            // Separator                                       ~~~
            i = index_of_next_equal_sign + 1 + field_value_len + 1;
        }
        Ok(Message {
            builder: self.message_builder_mut(),
            phantom: PhantomData,
            field_locator_context: FieldLocatorContext::TopLevel,
        })
    }

    fn store_field(
        &mut self,
        tag: TagU32,
        raw_message: &[u8],
        field_value_start: usize,
        field_value_len: usize,
    ) -> Result<(), DecodeError> {
        if field_value_start + field_value_len > raw_message.len() {
            return Err(DecodeError::Invalid);
        }
        let config_assoc = self.config().should_decode_associative;
        let field_value = &raw_message[field_value_start..][..field_value_len];
        if self.builder.state.new_group.is_some() {
            // We are entering a new group, but we still don't know which tag
            // will be the first one in each entry.
            self.builder.state.set_new_group(tag);
        } else if let Some(group_info) = self.builder.state.group_information.last_mut() {
            if group_info.current_entry_i >= group_info.num_entries {
                self.builder.state.group_information.pop();
            } else if tag == group_info.first_tag_of_every_group_entry {
                group_info.current_entry_i += 1;
            }
        }
        self.message_builder_mut()
            .add_field(
                tag,
                &raw_message[field_value_start..][..field_value_len],
                config_assoc,
            ).map_err(|e| DecodeError::Invalid)?;
        let fix_type = self.tag_lookup.get(&tag.get());
        if fix_type == Some(&FixDatatype::NumInGroup) {
            self.builder
                .state
                .add_group(tag, self.builder.field_locators.len() - 1, field_value);
        } else if fix_type == Some(&FixDatatype::Length) {
            // FIXME
            let last_field_locator = self.builder.field_locators.last()
                .ok_or(DecodeError::FieldPresence)?;
            let last_field = self.builder.fields.get(last_field_locator)
                .ok_or(DecodeError::FieldPresence)?;
            let last_field_value = last_field.1;
            let s = std::str::from_utf8(last_field_value).map_err(|_| DecodeError::Invalid)?;
            let data_field_length = str::parse(s).map_err(|_| DecodeError::Invalid)?;
            self.builder.state.data_field_length = Some(data_field_length);
        }
        Ok(())
    }
}

impl GetConfig for Decoder {
    type Config = Config;

    fn config(&self) -> &Self::Config {
        self.raw_decoder.config()
    }

    fn config_mut(&mut self) -> &mut Self::Config {
        self.raw_decoder.config_mut()
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
pub struct DecoderStreaming<B> {
    decoder: Decoder,
    raw_decoder: RawDecoderStreaming<B>,
    is_ready: bool,
}

impl<B> StreamingDecoder for DecoderStreaming<B>
where
    B: Buffer,
{
    type Buffer = B;
    type Error = DecodeError;

    fn buffer(&mut self) -> &mut Self::Buffer {
        self.raw_decoder.buffer()
    }

    fn clear(&mut self) {
        self.raw_decoder.clear();
        self.is_ready = false;
    }

    fn num_bytes_required(&self) -> usize {
        self.raw_decoder.num_bytes_required()
    }

    fn try_parse(&mut self) -> Result<Option<()>, DecodeError> {
        match self.raw_decoder.try_parse()? {
            Some(()) => {
                self.decoder.from_frame(self.raw_decoder.raw_frame())?;
                self.is_ready = true;
                Ok(Some(()))
            }
            None => Ok(None),
        }
    }
}

impl<B> DecoderStreaming<B>
where
    B: Buffer,
{
    /// # Panics
    ///
    /// Panics if [`DecoderStreaming::try_parse()`] didn't return [`Ok(Some(()))`].
    pub fn message(&self) -> Message<&[u8]> {
        assert!(self.is_ready);

        Message {
            builder: &self.decoder.builder,
            phantom: PhantomData,
            field_locator_context: FieldLocatorContext::TopLevel,
        }
    }
}

impl<B> GetConfig for DecoderStreaming<B> {
    type Config = Config;

    fn config(&self) -> &Self::Config {
        self.decoder.config()
    }

    fn config_mut(&mut self) -> &mut Self::Config {
        self.decoder.config_mut()
    }
}

/// A repeating group within a [`Message`].
#[derive(Debug, Clone)]
pub struct MessageGroup<'a, T>
where
    T: AsRef<[u8]>,
{
    message: Message<'a, T>,
    index_of_group_tag: u32,
    len: usize,
}

impl<'a, T> RepeatingGroup for MessageGroup<'a, T>
where
    T: AsRef<[u8]> + Clone,
{
    type Entry = Message<'a, T>;

    fn len(&self) -> usize {
        self.len
    }

    fn get(&self, i: usize) -> Option<Self::Entry> {
        if i < self.len {
            Some(Message {
                builder: self.message.builder,
                phantom: PhantomData,
                field_locator_context: FieldLocatorContext::WithinGroup {
                    index_of_group_tag: self.index_of_group_tag,
                    entry_index: i.try_into().unwrap(),
                },
            })
        } else {
            None
        }
    }
}

/// A FIX message returned by [`Decoder`] or [`DecoderStreaming`].
#[derive(Debug, Copy, Clone)]
pub struct Message<'a, T> {
    builder: &'a MessageBuilder<'a>,
    phantom: PhantomData<T>,
    field_locator_context: FieldLocatorContext,
}

impl<'a, T> Message<'a, T> {
    /// Returns an [`Iterator`] over all fields in `self`, in sequential order
    /// starting from the very first field.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::{Config, Decoder};
    /// use fefix::prelude::*;
    ///
    /// const DATA: &[u8] = b"8=FIX.4.4|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=185|";
    ///
    /// let mut decoder = Decoder::new(Dictionary::fix44());
    /// decoder.config_mut().separator = b'|';
    ///
    /// let message = decoder.decode(DATA).unwrap();
    /// let first_field = message.fields().next();
    ///
    /// assert_eq!(first_field, Some((TagU32::new(8).unwrap(), b"FIX.4.4" as &[u8])));
    /// ```
    pub fn fields(&'a self) -> Fields<'a, T> {
        Fields {
            message: self,
            i: 0,
        }
    }

    /// Returns the underlying byte contents of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::{Config, Decoder};
    /// use fefix::prelude::*;
    ///
    /// const DATA: &[u8] = b"8=FIX.4.4|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=185|";
    ///
    /// let mut decoder = Decoder::new(Dictionary::fix44());
    /// decoder.config_mut().separator = b'|';
    ///
    /// let message = decoder.decode(DATA).unwrap();
    /// assert_eq!(message.as_bytes(), DATA);
    /// ```
    pub fn as_bytes(&self) -> &[u8] {
        self.builder.bytes
    }

    /// Returns the number of FIX tags contained in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::{Config, Decoder};
    /// use fefix::prelude::*;
    ///
    /// const DATA: &[u8] = b"8=FIX.4.4|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=185|";
    ///
    /// let mut decoder = Decoder::new(Dictionary::fix44());
    /// decoder.config_mut().separator = b'|';
    ///
    /// let message = decoder.decode(DATA).unwrap();
    /// assert_eq!(message.len(), message.fields().count());
    /// ```
    pub fn len(&self) -> usize {
        self.builder.field_locators.len()
    }
}

impl<'a, T> PartialEq for Message<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        // Two messages are equal *if and only if* messages are exactly the
        // same. Fields must also have the same order (things get complicated
        // when you allow for different order of fields).
        self.fields().eq(other.fields())
    }
}

impl<'a, T> Eq for Message<'a, T> {}

#[derive(Debug, Copy, Clone)]
struct DecoderGroupState {
    first_tag_of_every_group_entry: TagU32,
    num_entries: usize,
    current_entry_i: usize,
    index_of_group_tag: usize,
}

#[derive(Debug, Copy, Clone)]
struct DecoderStateNewGroup {
    tag: TagU32,
    index_of_group_tag: usize,
    num_entries: usize,
}

#[derive(Debug, Clone)]
struct DecoderState {
    group_information: Vec<DecoderGroupState>,
    new_group: Option<DecoderStateNewGroup>,
    data_field_length: Option<usize>,
}

impl DecoderState {
    fn current_field_locator(&self, tag: TagU32) -> FieldLocator {
        FieldLocator {
            tag,
            context: match self.group_information.last() {
                Some(group_info) => FieldLocatorContext::WithinGroup {
                    index_of_group_tag: group_info.index_of_group_tag as u32,
                    entry_index: group_info.current_entry_i as u32,
                },
                None => FieldLocatorContext::TopLevel,
            },
        }
    }

    fn set_new_group(&mut self, tag: TagU32) {
        assert!(self.new_group.is_some());
        let new_group = self.new_group.take().unwrap();
        self.group_information.push(DecoderGroupState {
            first_tag_of_every_group_entry: tag,
            num_entries: new_group.num_entries,
            current_entry_i: 0,
            index_of_group_tag: new_group.index_of_group_tag,
        });
    }

    fn add_group(&mut self, tag: TagU32, index_of_group_tag: usize, field_value: &[u8]) {
        let field_value_str = std::str::from_utf8(field_value).unwrap();
        let num_entries = str::parse(field_value_str).unwrap();
        if num_entries > 0 {
            self.new_group = Some(DecoderStateNewGroup {
                tag,
                index_of_group_tag,
                num_entries,
            });
        }
    }
}

/// FIX message data structure with fast associative and sequential access.
#[derive(Debug, Clone)]
struct MessageBuilder<'a> {
    state: DecoderState,
    raw: &'a [u8],
    fields: HashMap<FieldLocator, (TagU32, &'a [u8], usize)>,
    field_locators: Vec<FieldLocator>,
    i_first_cell: usize,
    i_last_cell: usize,
    len_end_header: usize,
    len_end_body: usize,
    len_end_trailer: usize,
    bytes: &'a [u8],
}

impl<'a> Default for MessageBuilder<'a> {
    fn default() -> Self {
        Self {
            state: DecoderState {
                group_information: Vec::new(),
                new_group: None,
                data_field_length: None,
            },
            raw: b"",
            field_locators: Vec::new(),
            fields: HashMap::new(),
            i_first_cell: 0,
            i_last_cell: 0,
            len_end_body: 0,
            len_end_trailer: 0,
            len_end_header: 0,
            bytes: b"",
        }
    }
}

impl<'a> MessageBuilder<'a> {
    fn clear(&mut self) {
        *self = Self::default();
    }

    fn add_field(
        &mut self,
        tag: TagU32,
        field_value: &'a [u8],
        associative: bool,
    ) -> Result<(), DecodeError> {
        let field_locator = self.state.current_field_locator(tag);
        let i = self.field_locators.len();
        if associative {
            self.fields.insert(field_locator, (tag, field_value, i));
        }
        self.field_locators.push(field_locator);
        Ok(())
    }
}

/// An [`Iterator`] over fields and groups within a FIX message.
#[derive(Debug)]
pub struct Fields<'a, T> {
    message: &'a Message<'a, T>,
    i: usize,
}

impl<'a, T> ExactSizeIterator for Fields<'a, T> {
    fn len(&self) -> usize {
        self.message.len()
    }
}

impl<'a, T> FusedIterator for Fields<'a, T> {}

impl<'a, T> Iterator for Fields<'a, T> {
    type Item = (TagU32, &'a [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.message.len() {
            None
        } else {
            let context = self.message.builder.field_locators[self.i];
            let field = self.message.builder.fields.get(&context).unwrap();
            self.i += 1;
            Some((field.0, field.1))
        }
    }
}

impl<'a, T> FieldMap<u32> for Message<'a, T>
where
    T: AsRef<[u8]> + Clone,
{
    type Group = MessageGroup<'a, T>;

    fn group(&self, tag: u32) -> Result<Self::Group, FieldValueError<<usize as FieldType>::Error>> {
        let tag = TagU32::new(tag).ok_or(FieldValueError::Missing)?;
        let field_locator_of_group_tag = FieldLocator {
            tag,
            context: self.field_locator_context,
        };
        let num_in_group = self
            .builder
            .fields
            .get(&field_locator_of_group_tag)
            .ok_or(FieldValueError::Missing)?;
        let num_entries = usize::deserialize(num_in_group.1).map_err(FieldValueError::Invalid)?;
        let index_of_group_tag = num_in_group.2 as u32;
        Ok(MessageGroup {
            message: Message {
                builder: self.builder,
                phantom: PhantomData,
                field_locator_context: FieldLocatorContext::TopLevel,
            },
            index_of_group_tag,
            len: num_entries,
        })
    }

    fn get_raw(&self, tag: u32) -> Option<&[u8]> {
        let tag = TagU32::new(tag)?;
        let field_locator = FieldLocator {
            tag,
            context: self.field_locator_context,
        };
        dbglog!("looking for {:?}", field_locator);
        self.builder.fields.get(&field_locator).map(|field| field.1)
    }
}

impl<'a, F, T> FieldMap<&F> for Message<'a, T>
where
    F: IsFieldDefinition,
    T: AsRef<[u8]> + Clone,
{
    type Group = MessageGroup<'a, T>;

    fn group(
        &self,
        field: &F,
    ) -> Result<Self::Group, FieldValueError<<usize as FieldType>::Error>> {
        self.group(field.tag().get())
    }

    fn get_raw(&self, field: &F) -> Option<&[u8]> {
        self.get_raw(field.tag().get())
    }
}

#[cfg(feature = "utils-slog")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-slog")))]
impl<'a, T> slog::Value for Message<'a, T>
where
    T: AsRef<[u8]>,
{
    fn serialize(
        &self,
        _rec: &slog::Record,
        key: slog::Key,
        serializer: &mut dyn slog::Serializer,
    ) -> slog::Result {
        for (tag, _value) in self.fields() {
            serializer.emit_u32(key, tag.get())?;
            serializer.emit_char(key, '=')?;
            // FIXME
            serializer.emit_char(key, '?')?;
            serializer.emit_char(key, '|')?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct GroupRef<'a, T>
where
    T: AsRef<[u8]>,
{
    message: &'a Message<'a, T>,
    len: usize,
    field_len: u32,
}

#[derive(Debug, Clone)]
pub struct GroupRefIter<'a, T>
where
    T: AsRef<[u8]>,
{
    group: &'a GroupRef<'a, T>,
    i: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    // Use http://www.validfix.com/fix-analyzer.html for testing.

    const RANDOM_MESSAGES: &[&str] = &[
        "8=FIX.4.2|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=185|",
        "8=FIX.4.2|9=97|35=6|49=BKR|56=IM|34=14|52=20100204-09:18:42|23=115685|28=N|55=SPMI.MI|54=2|44=2200.75|27=S|25=H|10=248|",
        "8=FIX.4.4|9=117|35=AD|34=2|49=A|50=1|52=20100219-14:33:32.258|56=B|57=M|263=1|568=1|569=0|580=1|75=20100218|60=20100218-00:00:00.000|10=202|",
        "8=FIX.4.4|9=94|35=3|34=214|49=A|50=U1|52=20100304-09:42:23.130|56=AB|128=B1|45=176|58=txt|371=15|372=X|373=1|10=058|",
        "8=FIX.4.4|9=70|35=4|49=A|56=XYZ|34=129|52=20100302-19:38:21|43=Y|57=LOL|123=Y|36=175|10=192|",
        "8=FIX.4.4|9=122|35=D|34=215|49=CLIENT12|52=20100225-19:41:57.316|56=B|1=Marcel|11=13346|21=1|40=2|44=5|54=1|59=0|60=20100225-19:39:52.020|10=072|",
        "8=FIX.4.2|9=196|35=X|49=A|56=B|34=12|52=20100318-03:21:11.364|262=A|268=2|279=0|269=0|278=BID|55=EUR/USD|270=1.37215|15=EUR|271=2500000|346=1|279=0|269=1|278=OFFER|55=EUR/USD|270=1.37224|15=EUR|271=2503200|346=1|10=171|",
    ];

    fn with_soh(msg: &str) -> String {
        msg.split('|').collect::<Vec<&str>>().join("\x01")
    }

    fn decoder() -> Decoder {
        let mut decoder = Decoder::new(Dictionary::fix44());
        decoder.config_mut().separator = b'|';
        decoder
    }

    #[test]
    fn can_parse_simple_message() {
        let message = "8=FIX.4.2|9=40|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=091|";
        let mut decoder = decoder();
        let result = decoder.decode(message.as_bytes());
        assert!(result.is_ok());
    }

    #[test]
    fn skip_checksum_verification() {
        let message = "8=FIX.FOOBAR|9=5|35=0|10=000|";
        let mut decoder = decoder();
        let result = decoder.decode(message.as_bytes());
        assert!(result.is_ok());
    }

    #[test]
    fn repeating_group_entries() {
        let bytes = b"8=FIX.4.2|9=196|35=X|49=A|56=B|34=12|52=20100318-03:21:11.364|262=A|268=2|279=0|269=0|278=BID|55=EUR/USD|270=1.37215|15=EUR|271=2500000|346=1|279=0|269=1|278=OFFER|55=EUR/USD|270=1.37224|15=EUR|271=2503200|346=1|10=171|";
        let decoder = &mut decoder();
        let message = decoder.decode(bytes).unwrap();
        let group = message.group(268).unwrap();
        assert_eq!(group.len(), 2);
        assert_eq!(group.get(0).unwrap().get_raw(278).unwrap(), b"BID" as &[u8]);
    }

    #[test]
    fn top_level_tag_after_empty_group() {
        let bytes = b"8=FIX.4.4|9=17|35=X|268=0|346=1|10=171|";
        let mut decoder = decoder();
        let message = decoder.decode(&bytes).unwrap();
        let group = message.group(268).unwrap();
        assert_eq!(group.len(), 0);
        assert_eq!(message.get_raw(346), Some("1".as_bytes()));
    }

    #[test]
    fn assortment_of_random_messages_is_ok() {
        for msg_with_vertical_bar in RANDOM_MESSAGES {
            let message = with_soh(msg_with_vertical_bar);
            let mut codec = decoder();
            codec.config_mut().separator = 0x1;
            let result = codec.decode(message.as_bytes());
            result.unwrap();
        }
    }

    #[test]
    fn heartbeat_message_fields_are_ok() {
        let mut codec = decoder();
        let message = codec.decode(RANDOM_MESSAGES[0].as_bytes()).unwrap();
        assert_eq!(message.get(35), Ok(b"0"));
        assert_eq!(message.get_raw(8), Some(b"FIX.4.2" as &[u8]));
        assert_eq!(message.get(34), Ok(12));
        assert_eq!(message.get_raw(34), Some(b"12" as &[u8]));
    }

    #[test]
    fn message_without_final_separator() {
        let mut codec = decoder();
        let message = "8=FIX.4.4|9=122|35=D|34=215|49=CLIENT12|52=20100225-19:41:57.316|56=B|1=Marcel|11=13346|21=1|40=2|44=5|54=1|59=0|60=20100225-19:39:52.020|10=072";
        let result = codec.decode(message.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn message_must_end_with_separator() {
        let msg = "8=FIX.4.2|9=41|35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|10=127";
        let mut codec = decoder();
        let result = codec.decode(msg.as_bytes());
        assert!(matches!(result, Err(DecodeError::Invalid)));
    }

    #[test]
    fn message_without_checksum() {
        let msg = "8=FIX.4.4|9=37|35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|";
        let mut codec = decoder();
        let result = codec.decode(msg.as_bytes());
        assert!(matches!(result, Err(DecodeError::Invalid)));
    }

    #[test]
    fn message_with_data_field() {
        let msg =
            "8=FIX.4.4|9=58|35=D|49=AFUNDMGR|56=ABROKERt|15=USD|39=0|93=8|89=foo|\x01bar|10=000|";
        let mut codec = decoder();
        let result = codec.decode(msg.as_bytes()).unwrap();
        assert_eq!(result.get(93), Ok(8));
        assert!(matches!(result.get_raw(89), Some(b"foo|\x01bar")));
    }

    #[test]
    fn message_without_standard_header() {
        let msg = "35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|10=000|";
        let mut codec = decoder();
        let result = codec.decode(msg.as_bytes());
        assert!(matches!(result, Err(DecodeError::Invalid)));
    }

    #[test]
    fn detect_incorrect_checksum() {
        let msg = "8=FIX.4.2|9=43|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=146|";
        let mut codec = decoder();
        let result = codec.decode(msg.as_bytes());
        assert!(matches!(result, Err(DecodeError::Invalid)));
    }

    #[test]
    fn decoder_streaming_state_management() {
        use std::io::{Cursor, Read};
        let mut stream = Cursor::new(b"\
            8=FIX.4.2|9=40|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=091|\
            8=FIX.4.2|9=196|35=X|49=A|56=B|34=12|52=20100318-03:21:11.364|262=A|268=2|279=0|269=0|278=BID|55=EUR/USD|270=1.37215|15=EUR|271=2500000|346=1|279=0|269=1|278=OFFER|55=EUR/USD|270=1.37224|15=EUR|271=2503200|346=1|10=171|\
        ");
        let mut codec = decoder().streaming(vec![]);
        for msg_type in [b"D", b"X"] {
            loop {
                stream.read_exact(codec.fillable()).unwrap();
                if codec.try_parse().unwrap().is_some() {
                    assert_eq!(codec.message().get_raw(35), Some(&msg_type[..]));
                    break;
                }
            }
            codec.clear();
        }
    }
}
