use super::{
    Config, Configure, DecodeError, FieldAccess, RawDecoder, RawDecoderBuffered, RawFrame,
    RepeatingGroup,
};
use crate::dict;
use crate::dict::IsFieldDefinition;
use crate::TagU16;
use crate::{dict::FixDatatype, Dictionary};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{BuildHasher, Hasher};

// Number of bytes before the start of the `BeginString` field:
//
//   ~~
//   8=FIX.4.2|...
const BEGIN_STRING_OFFSET: usize = 2;

/// Univocally locates a tag within a FIX message, even with nested groups.
///
/// Typically, every FIX tag is guaranteed to be unique within a single FIX
/// message. Repeating groups, however, break this promise and allow *multiple*
/// values with the same tag, each in a different *group entry*. This means that
/// a FIX message is a tree rather than an associative array. [`FieldLocator`]
/// generates unique identifiers for tags both outsite and within groups, which
/// allows for random (i.e. non-sequential) reads on a FIX message.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FieldLocator {
    WithinGroup {
        tag: TagU16,
        index_of_group_tag: u32,
        entry_index: u32,
    },
    TopLevel {
        tag: TagU16,
    },
}

/// FIX message decoder.
///
/// One should create a [`Decoder`] per stream of FIX messages.
#[derive(Debug)]
pub struct Decoder<C = Config>
where
    C: Configure,
{
    dict: Dictionary,
    builder: MessageBuilder<'static>,
    raw_decoder: RawDecoder<C>,
    tag_lookup: TagLookup,
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

    /// Creates a new [`Decoder`] with custom `config`. `dict` is used to parse
    /// messages.
    pub fn with_config(dict: Dictionary, config: C) -> Self {
        Self {
            dict: dict.clone(),
            builder: MessageBuilder {
                state: DecoderState {
                    group_information: Vec::new(),
                    new_group: None,
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
            },
            raw_decoder: RawDecoder::with_config(config),
            tag_lookup: TagLookup::from_dict(&dict),
        }
    }

    /// Returns an immutable reference to the [`Configure`] used by `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::{Config, Configure, Decoder};
    /// use fefix::Dictionary;
    ///
    /// let dict = Dictionary::fix44();
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
    /// use fefix::Dictionary;
    ///
    /// let dict = Dictionary::fix44();
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
    /// use fefix::tagvalue::{FieldAccess, Config, Decoder};
    /// use fefix::definitions::fix42;
    /// use fefix::Dictionary;
    ///
    /// let dict = Dictionary::fix42();
    /// let decoder = &mut Decoder::<Config>::new(dict);
    /// let data = b"8=FIX.4.2\x019=42\x0135=0\x0149=A\x0156=B\x0134=12\x0152=20100304-07:59:30\x0110=185\x01";
    /// let message = decoder.decode(data).unwrap();
    /// assert_eq!(
    ///     message.fv(fix42::SENDER_COMP_ID),
    ///     Ok("A")
    /// );
    /// ```
    #[inline]
    pub fn decode<'a>(&'a mut self, bytes: &'a [u8]) -> Result<Message<'a>, DecodeError> {
        let frame = self.raw_decoder.decode(bytes)?;
        self.from_frame(frame)
    }

    fn message_builder_mut<'a>(&'a mut self) -> &'a mut MessageBuilder<'a> {
        unsafe { std::mem::transmute(&mut self.builder) }
    }

    fn from_frame<'a>(&'a mut self, frame: RawFrame<'a>) -> Result<Message<'a>, DecodeError> {
        self.builder.clear();
        let separator = self.config().separator();
        let payload = frame.payload();
        self.store_field(
            TagU16::new(8).unwrap(),
            frame.as_bytes(),
            BEGIN_STRING_OFFSET,
            frame.begin_string().len(),
        );
        let mut i = 0;
        while i < payload.len() {
            let index_of_next_equal_sign = {
                let i_eq = (&payload[i..])
                    .iter()
                    .copied()
                    .position(|byte| byte == b'=')
                    .map(|pos| pos + i);
                if i_eq.is_none() {
                    break;
                }
                i_eq.unwrap()
            };
            let field_value_len = {
                let len = (&payload[index_of_next_equal_sign + 1..])
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
                for byte in (&payload[i..index_of_next_equal_sign]).iter().copied() {
                    tag = tag * 10 + (byte as u32 - b'0' as u32);
                }
                if let Some(tag) = TagU16::new(tag as u16) {
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
            );
            // Equal sign                ~~~
            // Separator                                       ~~~
            i = index_of_next_equal_sign + 1 + field_value_len + 1;
        }
        Ok(Message {
            builder: self.message_builder_mut(),
        })
    }

    fn store_field<'a>(
        &mut self,
        tag: TagU16,
        raw_message: &'a [u8],
        field_value_start: usize,
        field_value_len: usize,
    ) {
        let config_assoc = self.config().should_decode_associative();
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
            )
            .unwrap();
        let entry = self.tag_lookup.lookup(tag).unwrap();
        if entry.data_type() == FixDatatype::NumInGroup {
            self.builder
                .state
                .add_group(tag, self.builder.field_locators.len() - 1, field_value);
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
    /// use fefix::Dictionary;
    ///
    /// let dict = Dictionary::fix44();
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
    /// use fefix::Dictionary;
    ///
    /// let dict = Dictionary::fix44();
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
    pub fn clear(&mut self) {
        self.raw_decoder.clear();
    }

    #[inline]
    pub fn state(&mut self) -> Result<Option<()>, DecodeError> {
        match self.raw_decoder.current_frame() {
            Ok(Some(frame)) => {
                self.decoder.from_frame(frame)?;
                Ok(Some(()))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    #[inline]
    pub fn message(&self) -> Message {
        Message {
            builder: &self.decoder.builder,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TagLookupEntry {
    data_type: FixDatatype,
    first_tag_of_group: TagU16,
}

impl TagLookupEntry {
    pub fn data_type(&self) -> FixDatatype {
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

    pub fn lookup(&self, tag: TagU16) -> Option<&TagLookupEntry> {
        self.entries.get(&tag)
    }
}

/// A repeating group within a [`Message`].
#[derive(Debug, Copy, Clone)]
pub struct MessageGroup<'a> {
    message: Message<'a>,
    index_of_group_tag: u32,
    len: usize,
}

impl<'a> MessageGroup<'a> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn entry(&self, index: usize) -> MessageGroupEntry<'a> {
        MessageGroupEntry {
            group: *self,
            entry_index: index as u32,
        }
    }
}

impl<'a> FieldAccess<'a> for MessageGroupEntry<'a> {
    type Key = TagU16;
    type Group = MessageGroup<'a>;

    fn group(&self, tag: Self::Key) -> Option<Self::Group> {
        let field_locator_of_group_tag = FieldLocator::TopLevel { tag };
        let num_in_group = self
            .group
            .message
            .builder
            .fields
            .get(&field_locator_of_group_tag)?;
        let index_of_group_tag = num_in_group.2 as u32;
        let field_value_str = std::str::from_utf8(num_in_group.1).ok()?;
        let num_entries = str::parse(field_value_str).unwrap();
        Some(MessageGroup {
            message: self.group.message,
            index_of_group_tag,
            len: num_entries,
        })
    }

    fn fv_raw_with_key<'b>(&'b self, tag: Self::Key) -> Option<&'b [u8]> {
        let field_locator = FieldLocator::WithinGroup {
            tag,
            index_of_group_tag: self.group.index_of_group_tag,
            entry_index: self.entry_index,
        };
        println!(
            "field locators are {:?}",
            self.group.message.builder.field_locators,
        );
        println!("field locator for group tag is {:?}", field_locator);
        self.group
            .message
            .builder
            .fields
            .get(&field_locator)
            .map(|field| field.1)
    }

    fn fv_raw<'b, F>(&'b self, field: &'a F) -> Option<&'b [u8]>
    where
        'b: 'a,
        F: IsFieldDefinition,
    {
        self.fv_raw_with_key(field.tag())
    }
}

impl<'a> RepeatingGroup<'a> for MessageGroup<'a> {
    type Entry = MessageGroupEntry<'a>;

    fn len(&self) -> usize {
        MessageGroup::len(self)
    }

    fn entry(&self, i: usize) -> Self::Entry {
        MessageGroup::entry(self, i)
    }
}

/// A specific [`MessageGroup`] entry.
#[derive(Debug)]
pub struct MessageGroupEntry<'a> {
    group: MessageGroup<'a>,
    entry_index: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Message<'a> {
    builder: &'a MessageBuilder<'a>,
}

impl<'a> PartialEq for Message<'a> {
    fn eq(&self, other: &Self) -> bool {
        // Two messages are equal *if and only if* messages are exactly the
        // same. Fields must also have the same order (things get complicated
        // when you allow for different order of fields).
        self.fields().eq(other.fields())
    }
}

impl<'a> Eq for Message<'a> {}

impl<'a> Message<'a> {
    pub fn group_ref(&self, tag: TagU16) -> Option<MessageGroup<'a>> {
        let field_locator_of_group_tag = FieldLocator::TopLevel { tag };
        let num_in_group = self.builder.fields.get(&field_locator_of_group_tag)?;
        let index_of_group_tag = num_in_group.2 as u32;
        let field_value_str = std::str::from_utf8(num_in_group.1).ok()?;
        let num_entries = str::parse(field_value_str).unwrap();
        Some(MessageGroup {
            message: *self,
            index_of_group_tag,
            len: num_entries,
        })
    }

    pub fn fields(&'a self) -> Fields<'a> {
        Fields {
            message: &self,
            i: 0,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.builder.bytes
    }

    pub fn len(&self) -> usize {
        self.builder.field_locators.len()
    }
}

#[derive(Debug, Copy, Clone)]
struct DecoderGroupState {
    first_tag_of_every_group_entry: TagU16,
    num_entries: usize,
    current_entry_i: usize,
    index_of_group_tag: usize,
}

#[derive(Debug, Copy, Clone)]
struct DecoderStateNewGroup {
    tag: TagU16,
    index_of_group_tag: usize,
    num_entries: usize,
}

#[derive(Debug, Clone)]
struct DecoderState {
    group_information: Vec<DecoderGroupState>,
    new_group: Option<DecoderStateNewGroup>,
}

impl DecoderState {
    fn current_field_locator(&self, tag: TagU16) -> FieldLocator {
        match self.group_information.last() {
            Some(group_info) => FieldLocator::WithinGroup {
                tag,
                index_of_group_tag: group_info.index_of_group_tag as u32,
                entry_index: group_info.current_entry_i as u32,
            },
            None => FieldLocator::TopLevel { tag },
        }
    }

    fn set_new_group(&mut self, tag: TagU16) {
        assert!(self.new_group.is_some());
        let new_group = self.new_group.take().unwrap();
        self.group_information.push(DecoderGroupState {
            first_tag_of_every_group_entry: tag,
            num_entries: new_group.num_entries,
            current_entry_i: 0,
            index_of_group_tag: new_group.index_of_group_tag,
        });
    }

    fn add_group(&mut self, tag: TagU16, index_of_group_tag: usize, field_value: &[u8]) {
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
pub struct MessageBuilder<'a> {
    state: DecoderState,
    raw: &'a [u8],
    fields: HashMap<FieldLocator, (TagU16, &'a [u8], usize)>,
    field_locators: Vec<FieldLocator>,
    i_first_cell: usize,
    i_last_cell: usize,
    len_end_header: usize,
    len_end_body: usize,
    len_end_trailer: usize,
    bytes: &'a [u8],
}

impl<'a> MessageBuilder<'a> {
    fn clear(&mut self) {
        self.raw = b"";
        self.fields.clear();
        self.field_locators.clear();
    }

    fn add_field(
        &mut self,
        tag: TagU16,
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
pub struct Fields<'a> {
    message: &'a Message<'a>,
    i: usize,
}

impl<'a> Iterator for Fields<'a> {
    type Item = (TagU16, &'a [u8]);

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

impl<'a> FieldAccess<'a> for Message<'a> {
    type Key = TagU16;
    type Group = MessageGroup<'a>;

    fn group(&self, tag: Self::Key) -> Option<Self::Group> {
        self.group_ref(tag)
    }

    fn fv_raw_with_key<'b>(&'b self, tag: Self::Key) -> Option<&'b [u8]> {
        let field_locator = FieldLocator::TopLevel { tag };
        self.builder.fields.get(&field_locator).map(|field| field.1)
    }

    fn fv_raw<'b, F>(&'b self, field: &F) -> Option<&'b [u8]>
    where
        'b: 'a,
        F: dict::IsFieldDefinition,
    {
        self.fv_raw_with_key(field.tag())
    }
}

#[cfg(feature = "utils-slog")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-slog")))]
impl<'a> slog::Value for Message<'a> {
    fn serialize(
        &self,
        _rec: &slog::Record,
        key: slog::Key,
        serializer: &mut dyn slog::Serializer,
    ) -> slog::Result {
        for (tag, _value) in self.fields() {
            serializer.emit_u16(key, tag.get())?;
            serializer.emit_char(key, '=')?;
            // FIXME
            serializer.emit_char(key, '?')?;
            serializer.emit_char(key, '|')?;
        }
        Ok(())
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
    use crate::{definitions::fix44, tagvalue::Config};

    // Use http://www.validfix.com/fix-analyzer.html for testing.

    fn with_soh(msg: &str) -> String {
        msg.split("|").collect::<Vec<&str>>().join("\x01")
    }

    fn decoder() -> Decoder<Config> {
        let dict = Dictionary::fix44();
        let mut config = Config::default();
        config.set_separator(b'|');
        Decoder::with_config(dict, config)
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
        let result = decoder.decode(message.as_bytes());
        assert!(result.is_ok());
    }

    #[test]
    fn repeating_group_entries() {
        let bytes = b"8=FIX.4.2|9=196|35=X|49=A|56=B|34=12|52=20100318-03:21:11.364|262=A|268=2|279=0|269=0|278=BID|55=EUR/USD|270=1.37215|15=EUR|271=2500000|346=1|279=0|269=1|278=OFFER|55=EUR/USD|270=1.37224|15=EUR|271=2503200|346=1|10=171|";
        let decoder = &mut decoder();
        let message = decoder.decode(bytes).unwrap();
        let group = message.group_ref(TagU16::new(268).unwrap()).unwrap();
        assert_eq!(group.len(), 2);
        assert_eq!(
            group.entry(0).fv_raw(fix44::MD_ENTRY_ID).unwrap(),
            b"BID" as &[u8]
        );
    }

    #[test]
    fn top_level_tag_after_empty_group() {
        let bytes = b"8=FIX.4.4|9=17|35=X|268=0|346=1|10=171|";
        let decoder = &mut decoder();
        let message = decoder.decode(bytes).unwrap();
        let group = message.group_ref(TagU16::new(268).unwrap()).unwrap();
        assert_eq!(group.len(), 0);
        assert_eq!(
            message.fv_raw_with_key(TagU16::new(346).unwrap()),
            Some("1".as_bytes())
        );
    }

    #[test]
    fn no_skip_checksum_verification() {
        let message = "8=FIX.FOOBAR|9=5|35=0|10=000|";
        let mut codec = Decoder::<Config>::new(Dictionary::fix44());
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
        let message = codec.decode(&mut RANDOM_MESSAGES[0].as_bytes()).unwrap();
        assert_eq!(message.fv(fix44::MSG_TYPE), Ok(fix44::MsgType::Heartbeat));
        assert_eq!(
            message.fv_raw_with_key(TagU16::new(8).unwrap()),
            Some(b"FIX.4.2" as &[u8])
        );
        assert_eq!(
            message.fv_raw_with_key(TagU16::new(35).unwrap()),
            Some(b"0" as &[u8]),
        );
    }

    #[test]
    fn message_without_final_separator() {
        let message = "8=FIX.4.4|9=122|35=D|34=215|49=CLIENT12|52=20100225-19:41:57.316|56=B|1=Marcel|11=13346|21=1|40=2|44=5|54=1|59=0|60=20100225-19:39:52.020|10=072";
        let mut config = Config::default();
        config.set_separator(b'|');
        let mut codec = Decoder::with_config(Dictionary::fix44(), config);
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
