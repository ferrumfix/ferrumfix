use crate::app::slr;
use crate::dictionary::{BaseType, Dictionary};
use crate::encoders::{Codec, Encoding, Poll};
use std::fmt;
use std::io;
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
pub struct TagValue<Z: Transmuter> {
    dict: Dictionary,
    transmuter: Z,
}

impl<Z: Transmuter> TagValue<Z> {
    /// Builds a new `TagValue` encoding device with an empty FIX dictionary.
    pub fn new(transmuter: Z) -> Self {
        Self::with_dict(transmuter, Dictionary::empty())
    }

    /// Creates a new codec for the tag-value format. `transmuter` specifies its
    /// settings and `dict` is used to parse messages.
    pub fn with_dict(transmuter: Z, dict: Dictionary) -> Self {
        TagValue {
            dict,
            transmuter,
        }
    }
}

impl<'s, Z> Codec<'s, &'s slr::Message> for TagValue<Z>
where
    Z: Transmuter + 's,
{
    type DecodeError = DecodeError;
    type EncodeError = EncodeError;

    fn decode(&mut self, _data: &[u8]) -> ResultDecode<Poll> {
        unimplemented!()
    }

    fn get_item(&self) -> &slr::Message {
        unimplemented!()
    }

    fn encode(&mut self, _data: &slr::Message) -> ResultEncode<&[u8]> {
        unimplemented!()
    }
}

impl<Z> Encoding<slr::Message> for TagValue<Z>
where
    Z: Transmuter,
{
    type EncodeError = EncodeError;
    type DecodeError = DecodeError;

    fn decode(
        &self,
        source: &mut impl io::BufRead,
    ) -> ResultDecode<slr::Message> {
        let mut field_iter = FieldIter {
            handle: source,
            checksum: Z::ChecksumCalculator::default(),
            designator: StandardTagLookup::new(&self.dict),
            length: std::u32::MAX,
            is_last: false,
            data_length: 0,
            transmuter: self.transmuter.clone(),
        };
        let mut message = slr::Message::new();
        {
            // `BeginString(8)`.
            let f = field_iter.next().ok_or(Error::Eof)??;
            if f.tag == 8 {
                message.fields.insert(f.tag, f.value);
            } else {
                return Err(Error::InvalidStandardHeader);
            }
        };
        {
            // `BodyLength(9)`.
            let f = field_iter.next().ok_or(Error::InvalidStandardHeader)??;
            if f.tag == 9 {
                message.fields.insert(f.tag, f.value);
            } else {
                return Err(Error::InvalidStandardHeader);
            }
        };
        {
            // `MsgType(35)`.
            let f = field_iter.next().ok_or(Error::InvalidStandardHeader)??;
            if f.tag == 35 {
                message.fields.insert(f.tag, f.value);
            } else {
                return Err(Error::InvalidStandardHeader);
            }
        };
        let mut last_tag = 35;
        for f_result in field_iter {
            let f = f_result?;
            message.fields.insert(f.tag, f.value);
            last_tag = f.tag;
        }
        if last_tag == 10 {
            Ok(message)
        } else {
            Err(Error::InvalidStandardTrailer)
        }
    }

    fn encode(&mut self, message: slr::Message) -> ResultEncode<Vec<u8>> {
        let mut target = Vec::new();
        for (tag, value) in message.fields {
            let field = slr::Field {
                tag,
                value,
                checksum: 0,
                len: 0,
            };
            field.encode(&mut target)?;
        }
        Ok(target)
    }
}

type DecodeError = Error;
type EncodeError = Error;

type ResultDecode<T> = Result<T, DecodeError>;
type ResultEncode<T> = Result<T, EncodeError>;

impl From<io::Error> for Error {
    fn from(_err: io::Error) -> Self {
        Error::Eof // FIXME
    }
}

trait TagLookup {
    fn lookup(&mut self, tag: u32) -> BaseType;
}

struct StandardTagLookup<'d> {
    dictionary: &'d Dictionary,
    data_length: usize,
}

impl<'d> StandardTagLookup<'d> {
    fn new(dict: &'d Dictionary) -> Self {
        StandardTagLookup {
            dictionary: dict,
            data_length: 0,
        }
    }
}

impl<'d> TagLookup for StandardTagLookup<'d> {
    fn lookup(&mut self, tag: u32) -> BaseType {
        self.dictionary
            .get_field(tag)
            .map(|f| f.basetype())
            .unwrap_or(BaseType::String)
    }
}

pub enum TypeInfo {
    Int,
    Float,
    Char,
    String,
    Data(usize),
}

struct FieldIter<'d, R: io::Read, D: TagLookup, Z: Transmuter> {
    handle: &'d mut R,
    checksum: Z::ChecksumCalculator,
    designator: D,
    length: u32,
    is_last: bool,
    data_length: u32,
    transmuter: Z,
}

impl<'d, R, D, Z> Iterator for FieldIter<'d, R, D, Z>
where
    R: io::BufRead,
    D: TagLookup,
    Z: Transmuter,
{
    type Item = Result<slr::Field, DecodeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_last {
            return None;
        }
        let mut buffer: Vec<u8> = Vec::new();
        self.handle.read_until(b'=', &mut buffer).unwrap();
        if let None = buffer.pop() {
            return None;
        }
        let tag = std::str::from_utf8(&buffer[..])
            .unwrap()
            .parse::<i64>()
            .unwrap();
        if tag == 10 {
            self.is_last = true;
        }
        let datatype = self.designator.lookup(tag as u32);
        if let BaseType::Data = datatype {
            buffer = vec![0u8; self.data_length as usize];
            self.handle.read_exact(&mut buffer).unwrap();
            self.checksum.roll(&buffer[..]);
            self.checksum.roll_byte(Z::SOH_SEPARATOR);
            self.handle.read_exact(&mut buffer[0..1]).unwrap();
        } else {
            buffer = vec![];
            self.handle
                .read_until(Z::SOH_SEPARATOR, &mut buffer)
                .unwrap();
            match buffer.last() {
                Some(b) if *b == Z::SOH_SEPARATOR => buffer.pop(),
                _ => return Some(Err(Error::Eof)),
            };
            self.checksum.roll(&buffer[..]);
        }
        let field_value = field_value(datatype, &buffer[..]).unwrap();
        if let slr::FixFieldValue::Int(l) = field_value {
            self.data_length = l as u32;
        }
        Some(Ok(slr::Field {
            tag,
            value: field_value,
            checksum: self.checksum.clone().result(),
            len: self.checksum.window_length(),
        }))
    }
}

fn field_value(datatype: BaseType, buf: &[u8]) -> Result<slr::FixFieldValue, Error> {
    Ok(match datatype {
        BaseType::Char => slr::FixFieldValue::Char(buf[0] as char),
        BaseType::String => {
            slr::FixFieldValue::String(str::from_utf8(buf).map_err(|_| Error::Syntax)?.to_string())
        }
        BaseType::Data => slr::FixFieldValue::Data(buf.to_vec()),
        BaseType::Float => slr::FixFieldValue::Float(
            str::from_utf8(buf)
                .map_err(|_| Error::Syntax)?
                .parse::<f64>()
                .map_err(|_| Error::Syntax)?,
        ),
        BaseType::Int => slr::FixFieldValue::Int(
            str::from_utf8(buf)
                .map_err(|_| Error::Syntax)?
                .parse::<i64>()
                .map_err(|_| Error::Syntax)?,
        ),
    })
}

/// The [`Transmuter`](Transmuter) pattern allows deep customization of encoding
/// and decoding behavior without relying on runtime settings. By using this
/// trait and specializing the behavior of particular methods, users can change
/// the behavior of the FIX encoder without incurring in performance loss.
pub trait Transmuter: Clone {
    type ChecksumCalculator: ChecksumCalculator;

    /// The delimiter character, which terminates every tag-value pair including
    /// the last one. It is the ASCII control character SOH (0x1) by default.
    const SOH_SEPARATOR: u8 = 0x1;

    /// Determines if checksum should be calculated or not.
    const VALIDATE_CHECKSUM: bool = true;
}

#[derive(Clone)]
pub struct TransVerticalSlash;

impl Transmuter for TransVerticalSlash {
    type ChecksumCalculator = RollingChecksumCalculator;

    const SOH_SEPARATOR: u8 = '|' as u8;
}

pub trait ChecksumCalculator: Default + Clone {
    /// Calculates the checksum of `window` and compounds it with `self`.
    fn roll(&mut self, window: &[u8]);

    /// Calculates the checksum of `byte` and compounds it with `self`.
    fn roll_byte(&mut self, byte: u8);

    /// Returns the amount of bytes that were processed calculating for this
    /// checksum.
    fn window_length(&self) -> usize;

    /// Returns the final checksum value.
    fn result(self) -> u8;

    /// Checks that the calculated checksum of `self` matches `checksum`.
    fn verify(&self, checksum: u8) -> bool;
}

/// A rolling checksum over a byte array. Sums over each byte wrapping around at
/// 256.
#[derive(Copy, Clone, Debug, Default)]
pub struct RollingChecksumCalculator {
    checksum: u8,
    len: usize,
}

impl ChecksumCalculator for RollingChecksumCalculator {
    fn roll(&mut self, window: &[u8]) {
        for byte in window {
            self.roll_byte(*byte);
        }
    }

    fn roll_byte(&mut self, byte: u8) {
        self.checksum = self.checksum.wrapping_add(byte);
        self.len += 1;
    }

    fn window_length(&self) -> usize {
        self.len
    }

    fn result(self) -> u8 {
        self.checksum
    }

    fn verify(&self, checksum: u8) -> bool {
        self.checksum == checksum
    }
}

/// A non-verifying checksum calculator.
#[derive(Copy, Clone, Debug, Default)]
pub struct FakeChecksumCalculator {
    len: usize,
}

impl ChecksumCalculator for FakeChecksumCalculator {
    fn roll(&mut self, window: &[u8]) {
        self.len += window.len();
    }

    fn roll_byte(&mut self, _byte: u8) {
        self.len += 1;
    }

    fn window_length(&self) -> usize {
        self.len
    }

    fn result(self) -> u8 {
        0
    }

    fn verify(&self, _checksum: u8) -> bool {
        true
    }
}

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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InvalidChecksum {
    pub expected: u8,
    pub actual: u8,
}

#[cfg(test)]
mod test {
    use super::*;

    fn encoder() -> TagValue<TransVerticalSlash> {
        TagValue::new(TransVerticalSlash)
    }

    #[test]
    fn can_parse_simple_message() {
        let msg = "8=FIX.4.2|9=251|35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|10=127|";
        let result = encoder().decode(&mut msg.as_bytes());
        assert!(result.is_ok());
    }

    #[test]
    fn message_must_end_with_separator() {
        let msg = "8=FIX.4.2|9=251|35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|10=127";
        let result = encoder().decode(&mut msg.as_bytes());
        assert_eq!(result, Err(Error::Eof));
    }

    #[test]
    fn message_without_checksum() {
        let msg = "8=FIX.4.4|9=251|35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|";
        let result = encoder().decode(&mut msg.as_bytes());
        assert_eq!(result, Err(Error::InvalidStandardTrailer));
    }

    #[test]
    fn message_without_standard_header() {
        let msg = "35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|10=000|";
        let result = encoder().decode(&mut msg.as_bytes());
        assert_eq!(result, Err(Error::InvalidStandardHeader));
    }

    #[test]
    fn detect_incorrect_checksum() {
        let msg = "8=FIX.4.2|9=251|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=126|";
        let _result = encoder().decode(&mut msg.as_bytes());
    }
}
