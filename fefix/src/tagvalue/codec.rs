use crate::backend::{field_value::TagNum, Backend, FieldValue};
use crate::buffering::Buffer;
use crate::tagvalue::{
    slr, utils, Config, Configurable, DecodeError, EncodeError, FixFieldValue, PushyMessage,
    RawDecoder, TagLookup,
};
use crate::{AppVersion, DataType, Dictionary};
use std::str;
use std::{fmt::Debug, marker::PhantomData};

/// Easy-to-use [`Encoding`] that accomodates for most use cases.
#[derive(Debug)]
pub struct Codec<T, Z = Configurable>
where
    Z: Config,
{
    dict: Dictionary,
    message: slr::Message,
    config: Z,
    phantom: PhantomData<T>,
}

impl<T, Z> Codec<T, Z>
where
    T: Default + Backend,
    Z: Config,
{
    /// Builds a new [`Codec`] encoding device with a FIX 4.4 dictionary.
    pub fn new(config: Z) -> Self {
        Self::with_dict(Dictionary::from_version(AppVersion::Fix44), config)
    }

    /// Creates a new codec for the tag-value format. `dict` is used to parse messages.
    pub fn with_dict(dict: Dictionary, config: Z) -> Self {
        Self {
            dict,
            message: slr::Message::default(),
            config,
            phantom: PhantomData::default(),
        }
    }

    /// Turns `self` into a [`CodecBuffered`] by allocating an internal buffer.
    pub fn buffered(self) -> CodecBuffered<T, Z> {
        CodecBuffered {
            buffer: Vec::new(),
            buffer_relevant_len: 0,
            buffer_additional_len: 0,
            codec: self,
        }
    }

    /// Returns an immutable reference to the [`Config`] used by `self`.
    pub fn config(&self) -> &Z {
        &self.config
    }

    /// Returns a mutable reference to the [`Config`] used by `self`.
    pub fn config_mut(&mut self) -> &mut Z {
        &mut self.config
    }

    pub fn decode(&mut self, data: &[u8]) -> Result<&slr::Message, DecodeError> {
        let decoder = RawDecoder::new()
            .with_separator(self.config.separator())
            .with_checksum_verification(self.config.verify_checksum());
        // Take care of `BeginString`, `BodyLength` and `CheckSum`.
        let frame = decoder.decode(data)?;
        let begin_string = frame.begin_string();
        let body = frame.payload();
        // Empty the message.
        self.message.clear();
        let mut fields = &mut FieldIter::new(body, &self.config, &self.dict);
        // Deserialize `MsgType(35)`.
        let msg_type = {
            let mut f = fields.next().ok_or(DecodeError::Syntax)??;
            if f.tag() != 35 {
                dbglog!("Expected MsgType (35), got ({}) instead.", f.tag());
                return Err(DecodeError::Syntax);
            }
            f.take_value()
        };
        self.message
            .insert(8, FixFieldValue::string(begin_string).unwrap())
            .unwrap();
        self.message.insert(35, msg_type).unwrap();
        // Iterate over all the other fields and store them to the message.
        for field_result in &mut fields {
            let mut field = field_result?;
            dbglog!("Finished parsing field <{}>.", field.tag());
            self.message
                .insert(field.tag(), field.take_value())
                .unwrap();
        }
        Ok(&self.message)
    }

    pub fn encode<B>(
        &mut self,
        buffer: &mut B,
        message: &PushyMessage,
    ) -> Result<usize, EncodeError>
    where
        B: Buffer,
    {
        let body_writer = |buffer: &mut B| {
            let start_i = buffer.as_slice().len();
            message
                .for_each::<(), _>(|tag, value| {
                    if tag != 8 {
                        encode_field(
                            TagNum::from(tag as u16),
                            value,
                            buffer,
                            self.config.separator(),
                        );
                    }
                    Ok(())
                })
                .unwrap();
            buffer.as_slice().len() - start_i
        };
        let begin_string = message
            .get_field(8u32)
            .unwrap()
            .as_str()
            .unwrap()
            .as_bytes();
        utils::encode_raw(begin_string, body_writer, buffer, self.config.separator())
    }
}

fn encode_field(tag: TagNum, value: &FixFieldValue, write: &mut impl Buffer, separator: u8) {
    write.extend_from_slice(tag.to_string().as_bytes());
    write.extend_from_slice(&[b'=']);
    match &value {
        FixFieldValue::Group(_) => panic!("Can't encode a group!"),
        FixFieldValue::Atom(field) => write.extend_from_slice(field.to_string().as_bytes()),
    };
    write.extend_from_slice(&[separator]);
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
pub struct CodecBuffered<T, Z = Configurable>
where
    Z: Config,
{
    buffer: Vec<u8>,
    buffer_relevant_len: usize,
    buffer_additional_len: usize,
    codec: Codec<T, Z>,
}

impl<T, Z> CodecBuffered<T, Z>
where
    T: Default + Backend,
    Z: Config,
{
    /// Builds a new `Codec` encoding device with a FIX 4.4 dictionary.
    pub fn new(config: Z) -> Self {
        Self::with_dict(Dictionary::from_version(AppVersion::Fix44), config)
    }

    /// Creates a new codec for the tag-value format. `dict` is used to parse messages.
    pub fn with_dict(dict: Dictionary, config: Z) -> Self {
        Self {
            buffer: Vec::new(),
            buffer_relevant_len: 0,
            buffer_additional_len: 0,
            codec: Codec::with_dict(dict.clone(), config.clone()),
        }
    }
}

//impl<T, Z> StreamingDecoder<T> for CodecBuffered<T, Z>
//where
//    T: Backend + Default,
//    Z: Config,
//{
//    type Error = DecodeError;
//
//    fn supply_buffer(&mut self) -> (&mut usize, &mut [u8]) {
//        // 512 bytes at a time. Not optimal, but a reasonable default.
//        let len = 512;
//        self.buffer.resize(self.buffer_relevant_len + len, 0);
//        (
//            &mut self.buffer_additional_len,
//            &mut self.buffer[self.buffer_relevant_len..self.buffer_relevant_len + len],
//        )
//    }
//
//    fn attempt_decoding(&mut self) -> Result<Option<&T>, Self::Error> {
//        self.buffer_relevant_len += self.buffer_additional_len;
//        assert!(self.buffer_relevant_len <= self.buffer.len());
//        if self.buffer_relevant_len < 10 {
//            Ok(None)
//        } else if &self.buffer[self.buffer_relevant_len - 7..self.buffer_relevant_len - 3] == b"10="
//        {
//            self.codec
//                .decode(&self.buffer[..self.buffer_relevant_len])
//                .map(|message| Some(message))
//        } else {
//            Ok(None)
//        }
//    }
//
//    fn get(&self) -> &T {
//        &self.codec.message
//    }
//}

struct FieldIter<'a, Z>
where
    Z: Config,
{
    data: &'a [u8],
    cursor: usize,
    config: &'a Z,
    tag_lookup: Z::TagLookup,
    data_field_length: usize,
}

impl<'a, Z> FieldIter<'a, Z>
where
    Z: Config,
{
    fn new(data: &'a [u8], config: &'a Z, dictionary: &'a Dictionary) -> Self {
        Self {
            data,
            cursor: 0,
            config,
            tag_lookup: Z::TagLookup::from_dict(dictionary),
            data_field_length: 0,
        }
    }
}

impl<'a, Z> Iterator for &mut FieldIter<'a, Z>
where
    Z: Config,
{
    type Item = Result<slr::Field, DecodeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.data.len() {
            return None;
        }
        let mut tag = 0u32;
        while let Some(byte) = self.data.get(self.cursor) {
            self.cursor += 1;
            if *byte == b'=' {
                if tag == 0 {
                    return Some(Err(DecodeError::Syntax));
                } else {
                    break;
                }
            }
            tag = tag * 10 + byte.wrapping_sub(b'0') as u32;
        }
        if self.data.get(self.cursor).is_none() {
            return Some(Err(DecodeError::Syntax));
        }
        debug_assert_eq!(self.data[self.cursor - 1], b'=');
        debug_assert!(tag > 0);
        let datatype = self.tag_lookup.lookup(tag);
        dbglog!("Parsing a field with data type '{:?}'.", &datatype);
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
                dbglog!(
                    "Parsing the field value of <{}> (residual data as lossy UTF-8 is '{}').",
                    tag,
                    String::from_utf8_lossy(&self.data[self.cursor..]),
                );
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
                    return Some(Err(DecodeError::Syntax));
                }
            }
            Err(_) => (),
        }
        debug_assert_eq!(self.data[self.cursor - 1], self.config.separator());
        Some(Ok(slr::Field::new(tag, field_value)))
    }
}

fn read_field_value(datatype: DataType, buf: &[u8]) -> Result<FixFieldValue, DecodeError> {
    debug_assert!(!buf.is_empty());
    Ok(match datatype {
        DataType::Char => FixFieldValue::from(buf[0] as char),
        DataType::Data => FixFieldValue::Atom(FieldValue::Data(buf.to_vec())),
        DataType::Float => FixFieldValue::Atom(FieldValue::float(
            str::from_utf8(buf)
                .map_err(|_| DecodeError::Syntax)?
                .parse::<f32>()
                .map_err(|_| DecodeError::Syntax)?,
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
                    return Err(DecodeError::Syntax);
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
    use crate::tagvalue::{ConfigFastDefault, Configurable};

    // Use http://www.validfix.com/fix-analyzer.html for testing.

    fn encoder() -> Codec<slr::Message, impl Config> {
        let mut config = Configurable::default();
        config.set_separator(b'|');
        Codec::new(config)
    }

    fn encoder_with_soh() -> Codec<slr::Message, impl Config> {
        Codec::new(ConfigFastDefault)
    }

    fn encoder_slash_no_verify() -> Codec<slr::Message, impl Config> {
        let mut config = Configurable::default();
        config.set_separator(b'|');
        config.set_verify_checksum(false);
        Codec::new(config)
    }

    fn with_soh(msg: &str) -> String {
        msg.split("|").collect::<Vec<&str>>().join("\x01")
    }

    #[test]
    fn can_parse_simple_message() {
        let message = "8=FIX.4.2|9=40|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=091|";
        let mut config = Configurable::default();
        config.set_separator(b'|');
        let mut codec = Codec::<slr::Message>::new(config);
        let result = codec.decode(message.as_bytes());
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
        let mut config = Configurable::default();
        config.set_separator(b'|');
        config.set_verify_checksum(false);
        let mut codec = Codec::<slr::Message>::new(config);
        let result = codec.decode(message.as_bytes());
        assert!(result.is_ok());
    }

    #[test]
    fn no_skip_checksum_verification() {
        let message = "8=FIX.FOOBAR|9=5|35=0|10=000|";
        let mut config = Configurable::default();
        config.set_separator(b'|');
        config.set_verify_checksum(true);
        let mut codec = Codec::<slr::Message>::new(config);
        let result = codec.decode(message.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn assortment_of_random_messages_is_ok() {
        for msg_with_vertical_bar in RANDOM_MESSAGES {
            let message = with_soh(msg_with_vertical_bar);
            let mut codec = encoder_with_soh();
            let result = codec.decode(message.as_bytes());
            result.unwrap();
        }
    }

    #[test]
    fn heartbeat_message_fields_are_ok() {
        let mut codec = encoder_slash_no_verify();
        let message = codec.decode(&mut RANDOM_MESSAGES[0].as_bytes()).unwrap();
        assert_eq!(
            message.get_field(8),
            Some(&FixFieldValue::string(b"FIX.4.2").unwrap())
        );
        assert_eq!(
            message.get_field(35),
            Some(&FixFieldValue::string(b"0").unwrap())
        );
    }

    #[test]
    fn message_without_final_separator() {
        let message = "8=FIX.4.4|9=122|35=D|34=215|49=CLIENT12|52=20100225-19:41:57.316|56=B|1=Marcel|11=13346|21=1|40=2|44=5|54=1|59=0|60=20100225-19:39:52.020|10=072";
        let mut config = Configurable::default();
        config.set_separator(b'|');
        let mut codec = Codec::<slr::Message>::new(config);
        let result = codec.decode(message.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn message_must_end_with_separator() {
        let msg = "8=FIX.4.2|9=41|35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|10=127";
        let mut codec = encoder();
        let result = codec.decode(&mut msg.as_bytes());
        assert_eq!(result, Err(DecodeError::Syntax));
    }

    #[test]
    fn message_without_checksum() {
        let msg = "8=FIX.4.4|9=37|35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|";
        let mut codec = encoder();
        let result = codec.decode(&mut msg.as_bytes());
        assert_eq!(result, Err(DecodeError::Syntax));
    }

    #[test]
    fn message_without_standard_header() {
        let msg = "35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|10=000|";
        let mut codec = encoder();
        let result = codec.decode(&mut msg.as_bytes());
        assert_eq!(result, Err(DecodeError::Syntax));
    }

    #[test]
    fn detect_incorrect_checksum() {
        let msg = "8=FIX.4.2|9=43|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=146|";
        let mut codec = encoder();
        let result = codec.decode(&mut msg.as_bytes());
        assert_eq!(result, Err(DecodeError::Syntax));
    }
}
