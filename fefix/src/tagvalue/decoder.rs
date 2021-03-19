use crate::tags;
use crate::tagvalue::{
    field_value::FieldValue, Config, Configure, DecodeError, FixFieldValue, Message, RawDecoder,
    TagLookup,
};
use crate::{AppVersion, DataType, Dictionary};
use std::fmt::Debug;
use std::str;

use super::{RawDecoderBuffered, RawFrame};

/// FIX message decoder.
#[derive(Debug)]
pub struct Decoder<C = Config>
where
    C: Configure,
{
    dict: Dictionary,
    message: Message,
    raw_decoder: RawDecoder<C>,
}

impl<C> Decoder<C>
where
    C: Configure,
{
    /// Creates a new [`Decoder`] with a FIX 4.4 dictionary.
    pub fn new(config: C) -> Self {
        Self::with_dict(Dictionary::from_version(AppVersion::Fix44), config)
    }

    /// Creates a new [`Decoder`] for the tag-value format. `dict` is used to parse
    /// messages.
    pub fn with_dict(dict: Dictionary, config: C) -> Self {
        Self {
            dict,
            message: Message::new(),
            raw_decoder: RawDecoder::with_config(config),
        }
    }

    /// Returns an immutable reference to the [`Configure`] used by `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::{Config, Configure, Decoder};
    ///
    /// let decoder = Decoder::new(Config::default());
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
    ///
    /// let decoder = &mut Decoder::new(Config::default());
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
    ///
    /// let decoder = &mut Decoder::new(Config::default());
    /// let data = b"8=FIX.4.2\x019=42\x0135=0\x0149=A\x0156=B\x0134=12\x0152=20100304-07:59:30\x0110=185\x01";
    /// let message = decoder.decode(data).unwrap();
    /// assert_eq!(
    ///     message
    ///         .field(tags::SENDER_COMP_ID)
    ///         .and_then(|field| field.as_str()),
    ///     Some("A")
    /// );
    /// ```
    pub fn decode(&mut self, data: &[u8]) -> Result<&Message, DecodeError> {
        let frame = self.raw_decoder.decode(data)?;
        self.from_frame(frame)
    }

    fn from_frame(&mut self, frame: RawFrame) -> Result<&Message, DecodeError> {
        self.message.clear();
        let begin_string = frame.begin_string();
        let body = frame.payload();
        let config = self.config().clone();
        let mut fields = &mut FieldIter::new(body, &config, &self.dict);
        // Deserialize `MsgType(35)`.
        let msg_type = {
            let (tag, field_value) = fields.next().ok_or(DecodeError::Invalid)??;
            if tag != tags::MSG_TYPE {
                dbglog!("Expected MsgType (35), got ({}) instead.", tag);
                return Err(DecodeError::Invalid);
            }
            field_value
        };
        self.message
            .add_field(
                tags::BEGIN_STRING,
                FixFieldValue::string(begin_string).unwrap(),
            )
            .unwrap();
        self.message.add_field(tags::MSG_TYPE, msg_type).unwrap();
        // Iterate over all the other fields and store them to the message.
        for field_result in &mut fields {
            let (tag, field_value) = field_result?;
            self.message.add_field(tag, field_value).unwrap();
        }
        Ok(&self.message)
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
    ///
    /// let decoder = Decoder::new(Config::default()).buffered();
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
    ///
    /// let decoder = &mut Decoder::new(Config::default()).buffered();
    /// decoder.config_mut().set_separator(b'|');
    /// assert_eq!(decoder.config().separator(), b'|');
    /// ```
    pub fn config_mut(&mut self) -> &mut C {
        self.decoder.config_mut()
    }

    pub fn supply_buffer(&mut self) -> &mut [u8] {
        self.raw_decoder.supply_buffer()
    }

    pub fn current_message(&mut self) -> Result<Option<&Message>, DecodeError> {
        match self.raw_decoder.current_frame() {
            Ok(Some(frame)) => self.decoder.from_frame(frame).map(|msg| Some(msg)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

struct FieldIter<'a, C>
where
    C: Configure,
{
    data: &'a [u8],
    cursor: usize,
    config: &'a C,
    tag_lookup: C::TagLookup,
    data_field_length: usize,
}

impl<'a, C> FieldIter<'a, C>
where
    C: Configure,
{
    fn new(data: &'a [u8], config: &'a C, dictionary: &'a Dictionary) -> Self {
        Self {
            data,
            cursor: 0,
            config,
            tag_lookup: C::TagLookup::from_dict(dictionary),
            data_field_length: 0,
        }
    }
}

impl<'a, C> Iterator for &mut FieldIter<'a, C>
where
    C: Configure,
{
    type Item = Result<(u32, FixFieldValue), DecodeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.data.len() {
            return None;
        }
        let mut tag = 0u32;
        while let Some(byte) = self.data.get(self.cursor) {
            self.cursor += 1;
            if *byte == b'=' {
                if tag == 0 {
                    return Some(Err(DecodeError::Invalid));
                } else {
                    break;
                }
            }
            tag = tag * 10 + byte.wrapping_sub(b'0') as u32;
        }
        if self.data.get(self.cursor).is_none() {
            return Some(Err(DecodeError::Invalid));
        }
        debug_assert_eq!(self.data[self.cursor - 1], b'=');
        debug_assert!(tag > 0);
        let datatype = self.tag_lookup.lookup(tag);
        let mut field_value = FixFieldValue::from(0i64);
        match datatype {
            Ok(DataType::Data) => {
                field_value = FixFieldValue::Atom(FieldValue::Data(
                    self.data[self.cursor..self.cursor + self.data_field_length].to_vec(),
                ));
                self.cursor += self.data_field_length + 1;
                debug_assert_eq!(self.data[self.cursor - 1], self.config.separator());
            }
            Ok(datatype) => {
                if let Some(separator_i) = &self.data[self.cursor..]
                    .iter()
                    .position(|byte| *byte == self.config.separator())
                    .map(|i| i + self.cursor)
                {
                    field_value =
                        read_field_value(datatype, &self.data[self.cursor..*separator_i]).unwrap();
                    self.cursor = separator_i + 1;
                    debug_assert_eq!(self.data[self.cursor - 1], self.config.separator());
                    if datatype == DataType::Length {
                        self.data_field_length = field_value.as_length().unwrap();
                    }
                } else {
                    dbglog!("EOF before expected separator. Error.");
                    return Some(Err(DecodeError::Invalid));
                }
            }
            Err(_) => (),
        }
        debug_assert_eq!(self.data[self.cursor - 1], self.config.separator());
        Some(Ok((tag, field_value)))
    }
}

fn read_field_value(datatype: DataType, buf: &[u8]) -> Result<FixFieldValue, DecodeError> {
    debug_assert!(!buf.is_empty());
    Ok(match datatype {
        DataType::Char => FixFieldValue::from(buf[0] as char),
        DataType::Data => FixFieldValue::Atom(FieldValue::Data(buf.to_vec())),
        DataType::Float => FixFieldValue::Atom(FieldValue::float(
            str::from_utf8(buf)
                .map_err(|_| DecodeError::Invalid)?
                .parse::<f32>()
                .map_err(|_| DecodeError::Invalid)?,
        )),
        DataType::Int => {
            let mut n = 0i64;
            let mut multiplier = 1;
            for byte in buf.iter().rev() {
                if *byte >= b'0' && *byte <= b'9' {
                    let digit = byte - b'0';
                    n += digit as i64 * multiplier;
                } else if *byte == b'-' {
                    n *= -1;
                } else if *byte != b'+' {
                    return Err(DecodeError::Invalid);
                }
                multiplier *= 10;
            }
            FixFieldValue::from(n)
        }
        _ => FixFieldValue::string(buf).unwrap(),
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tagvalue::Config;

    // Use http://www.validfix.com/fix-analyzer.html for testing.

    fn with_soh(msg: &str) -> String {
        msg.split("|").collect::<Vec<&str>>().join("\x01")
    }

    fn decoder() -> Decoder<Config> {
        Decoder::new(Config::default().with_separator(b'|'))
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
    fn no_skip_checksum_verification() {
        let message = "8=FIX.FOOBAR|9=5|35=0|10=000|";
        let mut config = Config::default();
        config.set_separator(b'|');
        config.set_verify_checksum(true);
        let mut codec = Decoder::new(config);
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
            message.field(8),
            Some(&FixFieldValue::string(b"FIX.4.2").unwrap())
        );
        assert_eq!(
            message.field(35),
            Some(&FixFieldValue::string(b"0").unwrap())
        );
    }

    #[test]
    fn message_without_final_separator() {
        let message = "8=FIX.4.4|9=122|35=D|34=215|49=CLIENT12|52=20100225-19:41:57.316|56=B|1=Marcel|11=13346|21=1|40=2|44=5|54=1|59=0|60=20100225-19:39:52.020|10=072";
        let mut config = Config::default();
        config.set_separator(b'|');
        let mut codec = Decoder::new(config);
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
