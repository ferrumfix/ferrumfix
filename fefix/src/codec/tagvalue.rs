//! FIX tag-value encoding support.
//!
//! This is the original encoding used for FIX messages and also the encoding
//! currently used by the FIX session layer.

use crate::app::{slr, TsrMessageRef, Version};
use crate::codec::{Decoder, Encoder, StreamingDecoder};
use crate::dt;
use crate::dt::DataType;
use crate::dictionary::Dictionary;
use crate::utils::{Buffer, BufferWriter};
use std::fmt;
use std::fmt::Debug;
use std::io;
use std::rc::Rc;
use std::str;

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
pub struct Codec<T, Z> {
    dict: Dictionary,
    buffer: Vec<u8>,
    state: DecoderState,
    message: T,
    body: Body,
    config: Z,
}

impl<T, Z> Codec<T, Z>
where
    T: TsrMessageRef,
    Z: Config,
{
    /// Builds a new `Codec` encoding device with a FIX 4.4 dictionary.
    pub fn new(config: Z) -> Self {
        Self::with_dict(Dictionary::from_version(Version::Fix44), config)
    }

    /// Creates a new codec for the tag-value format. `dict` is used to parse messages.
    pub fn with_dict(dict: Dictionary, config: Z) -> Self {
        Self {
            dict,
            buffer: Vec::new(),
            state: DecoderState::Header,
            message: T::default(),
            body: Body::new(&[]),
            config,
        }
    }
}

#[derive(Debug)]
enum DecoderState {
    Header,
    Body(usize),
    Trailer,
}

#[derive(Debug)]
pub struct Body {
    len: usize,
}

impl Body {
    fn new(data: &[u8]) -> Self {
        Self {
            len: data.len(),
        }
    }
}

impl<Z> StreamingDecoder<Body> for Codec<slr::Message, Z>
where
    Z: Config,
{
    type Error = DecodeError;

    fn supply_buffer(&mut self) -> &mut [u8] {
        let buffer_len = self.buffer.len();
        let additional_capacity = match self.state {
            DecoderState::Header => 50,
            DecoderState::Body(n) => n,
            DecoderState::Trailer => 7,
        };
        for _ in 0..additional_capacity {
            self.buffer.push(0);
        }
        &mut self.buffer[buffer_len..]
    }

    fn attempt_decoding(&mut self) -> Result<Option<&Body>, Self::Error> {
        let mut field_iter: &mut FieldIter<_, Z> = &mut FieldIter {
            handle: &mut &self.buffer[..],
            designator: Z::TagLookup::from_dict(&self.dict),
            is_last: false,
            data_length: 0,
        };
        let mut message = slr::Message::new();
        {
            // `BeginString(8)`.
            let f = field_iter.next().ok_or(Error::Eof)??;
            if f.tag() == 8 {
                message.set_field(f.tag() as u32, f.value().clone());
            } else {
                return Err(Error::InvalidStandardHeader);
            }
        };
        {
            // `BodyLength(9)`.
            let f = field_iter.next().ok_or(Error::InvalidStandardHeader)??;
            if f.tag() == 9 {
                message.set_field(f.tag() as u32, f.value().clone());
            } else {
                return Err(Error::InvalidStandardHeader);
            }
        };
        {
            // `MsgType(35)`.
            let f = field_iter.next().ok_or(Error::InvalidStandardHeader)??;
            if f.tag() == 35 {
                message.set_field(f.tag() as u32, f.value().clone());
            } else {
                return Err(Error::InvalidStandardHeader);
            }
        };
        self.state = DecoderState::Body(0);
        self.state = DecoderState::Trailer;
        Ok(Some(&self.body))
    }

    fn get(&self) -> &Body {
        &self.body
    }
}

impl<Z, T> Decoder<T> for Codec<T, Z>
where
    T: TsrMessageRef,
    Z: Config,
{
    type Error = DecodeError;

    fn decode(&mut self, mut data: &[u8]) -> Result<&T, Self::Error> {
        let mut checksum = Z::ChecksumAlgo::default();
        checksum.roll(&data[..data.len() - 7]);
        let mut field_iter: &mut FieldIter<_, Z> = &mut FieldIter {
            handle: &mut data,
            designator: Z::TagLookup::from_dict(&self.dict),
            is_last: false,
            data_length: 0,
        };
        let mut message = T::default();
        {
            // `BeginString(8)`.
            let f = field_iter.next().ok_or(Error::Eof)??;
            if f.tag() == 8 {
                message.set_field(f.tag() as u32, f.value().clone());
            } else {
                return Err(Error::InvalidStandardHeader);
            }
        };
        {
            // `BodyLength(9)`.
            let f = field_iter.next().ok_or(Error::InvalidStandardHeader)??;
            if f.tag() == 9 {
                message.set_field(f.tag() as u32, f.value().clone());
            } else {
                return Err(Error::InvalidStandardHeader);
            }
        };
        {
            // `MsgType(35)`.
            let f = field_iter.next().ok_or(Error::InvalidStandardHeader)??;
            if f.tag() == 35 {
                message.set_field(f.tag() as u32, f.value().clone());
            } else {
                return Err(Error::InvalidStandardHeader);
            }
        };
        let mut last_tag = 35;
        for f_result in &mut field_iter {
            let f = f_result?;
            message.set_field(f.tag() as u32, f.value().clone());
            last_tag = f.tag();
        }
        let chesksum_field = message.get_field(10);
        if let Some(slr::FixFieldValue::String(s)) = chesksum_field {
            let n = s.as_str().parse::<u8>().unwrap();
            if !checksum.verify(n) {
                let checksum_error = InvalidChecksum {
                    actual: n,
                    expected: checksum.result(),
                };
                return Err(Error::InvalidChecksum(checksum_error));
            }
        }
        if last_tag == 10 {
            self.message = message;
            Ok(&self.message)
        } else {
            Err(Error::InvalidStandardTrailer)
        }
    }
}

impl<Z> Encoder<slr::Message> for Codec<slr::Message, Z>
where
    Z: Config,
{
    type Error = EncodeError;

    fn encode(
        &mut self,
        mut buffer: impl Buffer,
        message: &slr::Message,
    ) -> Result<usize, Self::Error> {
        let mut writer = BufferWriter::new(&mut buffer);
        // First, write `BeginString(8)`.
        encode_field(
            8.into(),
            message.get_field(8).unwrap(),
            &mut writer,
            Z::SOH_SEPARATOR,
        );
        // The second field is supposed to be `BodyLength(9)`, but obviously
        // the length of the message is unknow until later in the
        // serialization phase. This alone would usually require to
        //
        //  1. Serialize the rest of the message into an external buffer.
        //  2. Calculate the length of the message.
        //  3. Serialize `BodyLength(9)`.
        //  4. Copy the contents of the external buffer into `buffer`.
        //  5. ... go on with the serialization process.
        //
        // Luckily, FIX allows for zero-padded integer values and we can
        // leverage this to reserve some space for the value. We might waste
        // some bytes but the benefits largely outweight the costs.
        //
        // Six digits (~1MB) ought to be enough for every message.
        writer.extend_from_slice(b"9=000000");
        writer.extend_from_slice(&[Z::SOH_SEPARATOR]);
        let body_length_range = writer.as_slice().len() - 7..writer.as_slice().len() - 2;
        // We now must start to calculate the message length.
        let mut len = writer.as_slice().len();
        // Third field: `MsgType(35)`.
        encode_field(
            35.into(),
            message.get_field(35).unwrap(),
            &mut writer,
            Z::SOH_SEPARATOR,
        );
        // Now all the other fields.
        for (tag, value) in message.fields.iter() {
            if *tag != 35 {
                encode_field((*tag as u16).into(), value, &mut writer, Z::SOH_SEPARATOR);
            }
        }
        len = writer.as_slice().len() - len;
        // Finally, we need to serialize the `Checksum(10)` field.
        //encode_field(9.into(), &slr::FixFieldValue::Int(len as i64), &mut writer)?;
        for i in body_length_range.rev() {
            writer.as_mut_slice()[i] = (len % 10) as u8 + '0' as u8;
            len /= 10;
        }
        let mut checksum = Z::ChecksumAlgo::default();
        checksum.roll(writer.as_slice());
        encode_field(
            10.into(),
            &slr::FixFieldValue::from(checksum.result() as i64),
            &mut writer,
            Z::SOH_SEPARATOR,
        );
        Ok(writer.as_slice().len())
    }
}

fn encode_field(
    tag: dt::TagNum,
    value: &slr::FixFieldValue,
    write: &mut impl Buffer,
    separator: u8,
) {
    write.extend_from_slice(tag.to_string().as_bytes());
    write.extend_from_slice(&[b'=']);
    match &value {
        slr::FixFieldValue::String(s) => write.extend_from_slice(s.as_bytes()),
        slr::FixFieldValue::Data(raw_data) => write.extend_from_slice(&raw_data),
        slr::FixFieldValue::Group(_) => panic!("Can't encode a group!"),
        slr::FixFieldValue::Value(field) => write.extend_from_slice(field.to_string().as_bytes()),
    };
    write.extend_from_slice(&[separator]);
}

/// This trait describes dynamic tag lookup logic.
///
/// In this context, "tag lookup"
/// means to search in the dictionary the data type associated with a specific
/// tag number. This may seem trivial at best, but it can actually be quite
/// convoluted and require internal state (thus it is "dynamic" tag lookup). In
/// particular, several fields affect the internal state of a
/// [`TagLookup`](TagLookup):
///
///  - `ApplVerID <1128>`
///  - `ApplExtID <1156>`
///  - `CstmApplVerID <1129>`
///  - `DefaultApplVerID <1137>`
///  - `DefaultApplExtID <1407>`
///  - `DefaultCstmApplVerID <1408>`
///
/// Each of these fields affects the internal state and thus changes how
/// subsequent fields (and messages) are interpreted.
///
/// # Naming conventions
/// Implementors of this trait should start with `TagLookup`.
pub trait TagLookup {
    type Error: Debug;

    fn from_dict(dict: &Dictionary) -> Self;

    /// Returns the [`BaseType`] of the tag number `tag`.
    fn lookup(&mut self, tag: u32) -> Result<dt::DataType, Self::Error>;
}

/// A [`TagLookup`] that only allows a specific revision of the standard, like
/// most venues do.
#[derive(Debug)]
pub struct TagLookupPredetermined {
    current_dict: Rc<Dictionary>,
}

impl TagLookup for TagLookupPredetermined {
    type Error = TagLookupPredeterminedError;

    fn from_dict(dict: &Dictionary) -> Self {
        Self {
            current_dict: Rc::new(dict.clone()),
        }
    }

    fn lookup(&mut self, tag: u32) -> Result<dt::DataType, Self::Error> {
        // TODO
        match tag {
            // `ApplVerID <1128>`
            1128 => {}
            // `ApplExtID <1156>`
            1156 => {
                return Err(Self::Error::InvalidApplExtID);
            }
            // `CstmApplVerID <1129>`
            1129 => {
                return Err(Self::Error::InvalidCstmApplVerID);
            }
            // `DefaultApplVerID <1137>`
            1137 => {
                return Err(Self::Error::InvalidApplExtID);
            }
            // `DefaultApplExtID <1407>`
            1407 => {
                return Err(Self::Error::InvalidApplExtID);
            }
            // `DefaultCstmApplVerID <1408>`
            1408 => {
                return Err(Self::Error::InvalidCstmApplVerID);
            }
            _ => (),
        };
        Ok(self
            .current_dict
            .field_by_tag(tag)
            .map(|f| f.basetype())
            .unwrap_or(DataType::String))
    }
}

#[derive(Debug)]
pub enum TagLookupPredeterminedError {
    InvalidApplVerID,
    InvalidApplExtID,
    InvalidCstmApplVerID,
}

#[derive(Debug)]
pub enum TypeInfo {
    Int,
    Float,
    Char,
    String,
    Data(usize),
}

struct FieldIter<R, Z: Config> {
    handle: R,
    is_last: bool,
    data_length: u32,
    designator: Z::TagLookup,
}

impl<'d, R, Z> Iterator for &mut FieldIter<&'d mut R, Z>
where
    R: io::Read,
    Z: Config,
{
    type Item = Result<slr::Field, DecodeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_last {
            return None;
        }
        let mut buffer: Vec<u8> = Vec::new();
        let mut tag: u32 = 0;
        let mut buf = [0];
        loop {
            if self.handle.read(&mut buf).unwrap() == 0 {
                break;
            }
            let byte = buf[0];
            if byte == b'=' {
                break;
            }
            tag = tag * 10 + (byte as char).to_digit(10).unwrap();
        }
        if tag == 10 {
            self.is_last = true;
        } else if tag == 0 {
            return None;
        }
        let datatype = self.designator.lookup(tag as u32);
        match datatype {
            Ok(DataType::Data) => {
                buffer = vec![0u8; self.data_length as usize];
                self.handle.read_exact(&mut buffer).unwrap();
                self.handle.read_exact(&mut buffer[0..1]).unwrap();
            }
            Ok(_basetype) => {
                buffer = vec![];
                loop {
                    if self.handle.read(&mut buf).unwrap() == 0 {
                        return Some(Err(Error::Eof));
                    }
                    let byte = buf[0];
                    if byte == Z::SOH_SEPARATOR {
                        break;
                    } else {
                        buffer.push(byte);
                    }
                }
            }
            Err(_) => (),
        };
        let datatype = datatype.unwrap();
        let field_value = field_value(datatype, &buffer[..]).unwrap();
        if let slr::FixFieldValue::Value(dt::DataTypeValue::Int(dt::Int(l))) = field_value {
            self.data_length = l as u32;
        }
        Some(Ok(slr::Field::new(tag, field_value)))
    }
}

fn field_value(datatype: DataType, buf: &[u8]) -> Result<slr::FixFieldValue, Error> {
    debug_assert!(!buf.is_empty());
    Ok(match datatype {
        DataType::Char => slr::FixFieldValue::from(buf[0] as char),
        DataType::String => {
            slr::FixFieldValue::String(str::from_utf8(buf).map_err(|_| Error::Syntax)?.to_string())
        }
        DataType::Data => slr::FixFieldValue::Data(buf.to_vec()),
        DataType::Float => slr::FixFieldValue::Value(dt::DataTypeValue::Float(dt::Float::from(
            str::from_utf8(buf)
                .map_err(|_| Error::Syntax)?
                .parse::<f32>()
                .map_err(|_| Error::Syntax)?,
        ))),
        DataType::Int => {
            let mut n: i64 = 0;
            for byte in buf {
                if *byte >= '0' as u8 && *byte <= '9' as u8 {
                    let digit = byte - '0' as u8;
                    n = n * 10 + digit as i64;
                } else if *byte == '-' as u8 {
                    n *= -1;
                } else if *byte != '+' as u8 {
                    return Err(Error::Syntax);
                }
            }
            slr::FixFieldValue::from(n)
        }
        _ => return Err(Error::Syntax),
    })
}

/// The [`Config`](Config) pattern allows deep customization of encoding
/// and decoding behavior without relying on runtime settings. By using this
/// trait and specializing the behavior of particular methods, users can change
/// the behavior of the FIX encoder without incurring in performance loss.
///
/// # Naming conventions
/// Implementors of this trait should start with `Trans`.
pub trait Config: Clone {
    type ChecksumAlgo: ChecksumAlgo;
    type TagLookup: TagLookup;

    /// The delimiter character, which terminates every tag-value pair including
    /// the last one.
    ///
    /// ASCII 0x1 is the default SOH separator character.
    const SOH_SEPARATOR: u8 = 0x1;
}

/// A [`Config`] for [`Codec`] with default configuration
/// options.
///
/// This configurator uses [`ChecksumAlgoDefault`] as a checksum algorithm and
/// [`TagLookupPredetermined`] for its dynamic tag lookup logic.
#[derive(Debug, Clone)]
pub struct ConfigDefault;

impl Config for ConfigDefault {
    type ChecksumAlgo = ChecksumAlgoDefault;
    type TagLookup = TagLookupPredetermined;
}

/// A [`Config`](Config) for [`Codec`] with `|` (ASCII 0x7C)
/// as a field separator.
#[derive(Debug, Clone)]
pub struct ConfigVerticalSlash;

impl Config for ConfigVerticalSlash {
    type ChecksumAlgo = ChecksumAlgoDefault;
    type TagLookup = TagLookupPredetermined;

    const SOH_SEPARATOR: u8 = '|' as u8;
}

/// A [`Config`](Config) for [`Codec`] with `^` (ASCII 0x5F)
/// as a field separator.
#[derive(Debug, Clone)]
pub struct ConfigCaret;

impl Config for ConfigCaret {
    type ChecksumAlgo = ChecksumAlgoDefault;
    type TagLookup = TagLookupPredetermined;

    const SOH_SEPARATOR: u8 = '^' as u8;
}

/// Checksum calculation & verification algorithm. The API is designed to work
/// only with so-called "rolling" checksum algorithms, much like the one used by
/// the FIX tag-value encoding.
///
/// # Naming conventions
/// Implementors of this trait should start with `ChecksumAlgo`.
pub trait ChecksumAlgo: Default + Clone {
    /// Calculates the checksum of `window` and compounds it with `self`.
    fn roll(&mut self, window: &[u8]);

    /// Adds a partial checksum to `self`.
    fn add(&mut self, sum: u8);

    /// Returns the amount of bytes that were processed calculating for this
    /// checksum.
    fn window_length(&self) -> usize;

    /// Returns the final checksum value.
    fn result(&self) -> u8;

    /// Checks that the calculated checksum of `self` matches `checksum`.
    fn verify(&self, checksum: u8) -> bool;
}

/// A rolling checksum over a byte array. Sums over each byte wrapping around at
/// 256.
#[derive(Copy, Clone, Debug, Default)]
pub struct ChecksumAlgoDefault {
    checksum: u8,
    len: usize,
}

impl ChecksumAlgo for ChecksumAlgoDefault {
    fn roll(&mut self, window: &[u8]) {
        for byte in window {
            self.checksum = self.checksum.wrapping_add(*byte);
        }
        self.len += window.len();
    }

    fn add(&mut self, sum: u8) {
        self.checksum = self.checksum.wrapping_add(sum);
    }

    fn window_length(&self) -> usize {
        self.len
    }

    fn result(&self) -> u8 {
        self.checksum
    }

    fn verify(&self, checksum: u8) -> bool {
        self.checksum == checksum
    }
}

/// A non-verifying checksum calculator.
#[derive(Copy, Clone, Debug, Default)]
pub struct ChecksumAlgoLazy {
    len: usize,
}

impl ChecksumAlgo for ChecksumAlgoLazy {
    fn roll(&mut self, window: &[u8]) {
        self.len += window.len();
    }

    fn add(&mut self, _sum: u8) {}

    fn window_length(&self) -> usize {
        self.len
    }

    fn result(&self) -> u8 {
        0
    }

    fn verify(&self, _checksum: u8) -> bool {
        true
    }
}

type DecodeError = Error;
type EncodeError = Error;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    FieldWithoutValue(u32),
    RepeatedTag(u32),
    Eof,
    InvalidStandardHeader,
    InvalidStandardTrailer,
    InvalidChecksum(InvalidChecksum),
    Syntax,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<io::Error> for Error {
    fn from(_err: io::Error) -> Self {
        Error::Eof // FIXME
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InvalidChecksum {
    pub expected: u8,
    pub actual: u8,
}

#[cfg(test)]
mod test {
    use super::*;

    // Use http://www.validfix.com/fix-analyzer.html for testing.

    fn encoder() -> Codec<slr::Message, impl Config> {
        Codec::new(ConfigVerticalSlash)
    }

    fn encoder_with_soh() -> Codec<slr::Message, impl Config> {
        Codec::new(ConfigDefault)
    }

    #[derive(Clone, Debug)]
    struct ConfigVerticalSlashNoVerify;

    impl Config for ConfigVerticalSlashNoVerify {
        type ChecksumAlgo = ChecksumAlgoLazy;
        type TagLookup = TagLookupPredetermined;

        const SOH_SEPARATOR: u8 = '|' as u8;
    }

    fn encoder_slash_no_verify() -> Codec<slr::Message, impl Config> {
        Codec::new(ConfigVerticalSlashNoVerify)
    }

    fn with_soh(msg: &str) -> String {
        msg.split("|").collect::<Vec<&str>>().join("\x01")
    }

    #[test]
    fn can_parse_simple_message() {
        let msg = with_soh("8=FIX.4.2|9=251|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=050|");
        let mut codec = encoder_with_soh();
        let result = codec.decode(&mut msg.as_bytes());
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
    fn assortment_of_random_messages_is_ok() {
        for msg_with_vertical_bar in RANDOM_MESSAGES {
            let msg = with_soh(msg_with_vertical_bar);
            let mut codec = encoder_with_soh();
            let result = codec.decode(&mut msg.as_bytes());
            assert!(result.is_ok());
        }
    }

    #[test]
    fn heartbeat_message_fields_are_ok() {
        let mut codec = encoder_slash_no_verify();
        let message = codec.decode(&mut RANDOM_MESSAGES[0].as_bytes()).unwrap();
        assert_eq!(
            message.get_field(8),
            Some(&slr::FixFieldValue::String("FIX.4.2".to_string()))
        );
        assert_eq!(message.get_field(9), Some(&slr::FixFieldValue::from(42i64)));
        assert_eq!(
            message.get_field(35),
            Some(&slr::FixFieldValue::String("0".to_string()))
        );
    }

    #[test]
    fn new_order_single_without_final_separator() {
        let msg = "8=FIX.4.4|9=122|35=D|34=215|49=CLIENT12|52=20100225-19:41:57.316|56=B|1=Marcel|11=13346|21=1|40=2|44=5|54=1|59=0|60=20100225-19:39:52.020|10=072";
        let mut codec = encoder();
        let result = codec.decode(&mut msg.as_bytes());
        assert_eq!(result, Err(Error::Eof));
    }

    #[test]
    fn message_must_end_with_separator() {
        let msg = "8=FIX.4.2|9=251|35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|10=127";
        let mut codec = encoder();
        let result = codec.decode(&mut msg.as_bytes());
        assert_eq!(result, Err(Error::Eof));
    }

    #[test]
    fn message_without_checksum() {
        let msg = "8=FIX.4.4|9=251|35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|";
        let mut codec = encoder();
        let result = codec.decode(&mut msg.as_bytes());
        assert_eq!(result, Err(Error::InvalidStandardTrailer));
    }

    #[test]
    fn message_without_standard_header() {
        let msg = "35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|10=000|";
        let mut codec = encoder();
        let result = codec.decode(&mut msg.as_bytes());
        assert_eq!(result, Err(Error::InvalidStandardHeader));
    }

    #[test]
    fn detect_incorrect_checksum() {
        let msg = "8=FIX.4.2|9=251|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=146|";
        let mut codec = encoder();
        let result = codec.decode(&mut msg.as_bytes());
        match result {
            Err(DecodeError::InvalidChecksum(_)) => (),
            _ => panic!(),
        }
    }
}
