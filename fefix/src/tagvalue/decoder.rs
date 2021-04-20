use super::{
    message::{AncestryTracker, Context},
    RawDecoder, RawDecoderBuffered, RawFrame,
};
use crate::fields::fix44 as fields;
use crate::tagvalue::{Config, Configure, DecodeError, Message, MessageBuilder};
use crate::{DataType, Dictionary};
use std::fmt::Debug;

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
    current_group_entry_tag: u32,
    remaining_group_entries: u16,
    group_ancestry: AncestryTracker,
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
            dict,
            builder: MessageBuilder::new(),
            raw_decoder: RawDecoder::with_config(config),
            current_group_entry_tag: 0,
            remaining_group_entries: 0,
            group_ancestry: AncestryTracker::top_level(),
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
    pub fn decode<'a>(&'a mut self, bytes: &'a [u8]) -> Result<Message<'a>, DecodeError> {
        let frame = self.raw_decoder.decode(bytes)?;
        self.from_frame(frame)
    }

    fn from_frame<'a>(&'a mut self, frame: RawFrame<'a>) -> Result<Message<'a>, DecodeError> {
        self.builder.clear();
        let bytes = frame.as_bytes();
        let mut tag_num = 0u32;
        let mut state_is_tag = true;
        let mut i_sep;
        let mut i_equal_sign = 0usize;
        self.builder
            .add_field(
                Context::top_level(fields::BEGIN_STRING.tag()),
                BEGIN_STRING_OFFSET,
                frame.begin_string().len(),
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
                let msg = self.builder.build(bytes);
                let msg_type = msg.f_msg_type().unwrap_or("").to_string();
                self.store_field(tag_num, start, len, msg_type.as_str());
                tag_num = 0;
            } else if state_is_tag {
                tag_num = tag_num * 10 + (byte - b'0') as u32;
            }
        }
        Ok(self.builder.build(bytes))
    }

    fn store_field(&mut self, tag: u32, start: usize, len: usize, msg_type: &str) {
        let field_definition = self.dict.field_by_tag(tag).unwrap();
        if tag == self.current_group_entry_tag {
            self.remaining_group_entries -= 1;
            if self.remaining_group_entries <= 1 {
                self.group_ancestry.leave_group();
            }
        }
        if field_definition.basetype() == DataType::NumInGroup {
            // It's a "group leader".
            // FIXME
            self.current_group_entry_tag = self
                .dict
                .message_by_msgtype(msg_type)
                .unwrap()
                .group_info(tag)
                .unwrap();
            self.group_ancestry.enter_group();
            // FIXME: read value and set it.
            self.remaining_group_entries = 1;
            let context = Context {
                tag,
                ancestry: AncestryTracker::top_level().ancestry(),
            };
            self.builder.add_field(context, start, len).unwrap();
        } else {
            // It's not.
            if tag == self.current_group_entry_tag {
                self.remaining_group_entries = self.remaining_group_entries.wrapping_sub(1);
                if self.remaining_group_entries <= 1 {
                    self.current_group_entry_tag = 0;
                }
                self.group_ancestry.incr_entry();
            }
            let context = Context {
                tag,
                ancestry: AncestryTracker::top_level().ancestry(),
            };
            self.builder.add_field(context, start, len).unwrap();
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
    pub fn config_mut(&mut self) -> &mut C {
        self.decoder.config_mut()
    }

    pub fn supply_buffer(&mut self) -> &mut [u8] {
        self.raw_decoder.supply_buffer()
    }

    pub fn current_message(&mut self) -> Result<Option<Message>, DecodeError> {
        match self.raw_decoder.current_frame() {
            Ok(Some(frame)) => self.decoder.from_frame(frame).map(|msg| Some(msg)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{fields::fix44 as fields, tagvalue::Config, AppVersion};

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
        let group = message.group_ref(268).unwrap();
        assert_eq!(group.len(), 2);
        assert_eq!(
            group
                .entry(0)
                .field_ref(fields::MD_ENTRY_ID)
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
        assert_eq!(message.field_ref(fields::MSG_TYPE), Some(Ok(b"0" as &[u8])));
        assert_eq!(message.field_raw(8), Some(b"FIX.4.2" as &[u8]));
        assert_eq!(message.field_raw(35), Some(b"0" as &[u8]),);
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
