use crate::ir;
use crate::presentation::Encoding;
use crate::dictionary::{BaseType, Dictionary};
use std::io;
use std::str;

/// "Start of heading" (SOH) control character (ASCII `0x1`). Each tag-value pair
/// MUST be followed by this control character.
const SOH_SEPARATOR: char = 1u8 as char;

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
pub struct TagValue {
    dict: Dictionary,
}

type DecodeResult<T> = Result<T, <TagValue as Encoding>::DecodeErr>;
type EncodeResult<T> = Result<T, <TagValue as Encoding>::EncodeErr>;

impl TagValue {
    /// Builds a new `TagValue` encoding device with an empty FIX dictionary.
    pub fn new() -> Self {
        Self::default()
    }

    //fn decode_checksum(
    //    &self,
    //    source: &mut impl io::BufRead,
    //    message: &mut ir::Message,
    //) -> DecodeResult<u8> {
    //    let field = parse_field(source, self.separator, &|_: i64| BaseType::Int)?;
    //    if let ir::FixFieldValue::Int(checksum) = field.value {
    //        message.fields.insert(field.tag, field.value);
    //        Ok(checksum as u8)
    //    } else {
    //        Err(Error::Syntax)
    //    }
    //}

    fn decode_ws(
        &self,
        source: &mut impl io::BufRead,
        separator: char,
    ) -> Result<ir::Message, <TagValue as Encoding>::DecodeErr> {
        let tag_lookup = StandardTagLookup::new(&self.dict);
        let mut checksum = Checksum::new();
        let mut field_iter = FieldIter {
            handle: source,
            separator: separator as u8,
            checksum,
            designator: tag_lookup,
            length: std::u32::MAX,
            is_last: false,
            data_length: 0,
        };
        let mut message = ir::Message::new();
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
}

impl Default for TagValue {
    fn default() -> Self {
        TagValue {
            dict: Dictionary::empty(),
        }
    }
}

impl Encoding for TagValue {
    type EncodeErr = Error;
    type DecodeErr = Error;

    fn decode(&self, source: &mut impl io::BufRead) -> Result<ir::Message, Self::DecodeErr> {
        self.decode_ws(source, SOH_SEPARATOR)
    }

    fn encode(&self, message: ir::Message) -> Result<Vec<u8>, Self::EncodeErr> {
        let mut target = Vec::new();
        for (tag, value) in message.fields {
            let field = ir::Field {
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

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Eof // FIXME
    }
}

/// A rolling checksum over a byte array. Sums over each byte wrapping around at
/// 256.
#[derive(Copy, Clone, Debug)]
struct Checksum(u8, usize);

impl Checksum {
    fn new() -> Self {
        Checksum(0, 0)
    }

    fn roll(&mut self, window: &[u8]) {
        for byte in window {
            self.roll_byte(*byte);
        }
    }

    fn roll_byte(&mut self, byte: u8) {
        self.0 = self.0.wrapping_add(byte);
        self.1 += 1;
    }

    fn window_length(&self) -> usize {
        self.1
    }

    fn result(self) -> u8 {
        self.0
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

struct FieldIter<'d, R: io::Read, D: TagLookup> {
    handle: &'d mut R,
    separator: u8,
    checksum: Checksum,
    designator: D,
    length: u32,
    is_last: bool,
    data_length: u32,
}

impl<'d, R: io::BufRead, D: TagLookup> Iterator for FieldIter<'d, R, D> {
    type Item = DecodeResult<ir::Field>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_last {
            return None;
        }
        let mut buffer: Vec<u8> = Vec::new();
        self.handle.read_until(b'=', &mut buffer).unwrap();
        if let None = buffer.pop() {
            return None;
        }
        //println!("{:?}", std::str::from_utf8(&buffer[..]).unwrap());
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
            self.checksum.roll_byte(self.separator);
            self.handle.read_exact(&mut buffer[0..1]).unwrap();
        } else {
            buffer = vec![];
            self.handle.read_until(self.separator, &mut buffer).unwrap();
            match buffer.last() {
                Some(b) if *b == self.separator => buffer.pop(),
                _ => return Some(Err(Error::Eof)),
            };
            self.checksum.roll(&buffer[..]);
        }
        let field_value = field_value(datatype, &buffer[..]).unwrap();
        if let ir::FixFieldValue::Int(l) = field_value {
            self.data_length = l as u32;
        }
        Some(Ok(ir::Field {
            tag,
            value: field_value,
            checksum: self.checksum.0,
            len: self.checksum.window_length(),
        }))
    }
}

fn field_value(datatype: BaseType, buf: &[u8]) -> Result<ir::FixFieldValue, Error> {
    Ok(match datatype {
        BaseType::Char => ir::FixFieldValue::Char(buf[0] as char),
        BaseType::String => {
            ir::FixFieldValue::String(str::from_utf8(buf).map_err(|_| Error::Syntax)?.to_string())
        }
        BaseType::Data => ir::FixFieldValue::Data(buf.to_vec()),
        BaseType::Float => ir::FixFieldValue::Float(
            str::from_utf8(buf)
                .map_err(|_| Error::Syntax)?
                .parse::<f64>()
                .map_err(|_| Error::Syntax)?,
        ),
        BaseType::Int => ir::FixFieldValue::Int(
            str::from_utf8(buf)
                .map_err(|_| Error::Syntax)?
                .parse::<i64>()
                .map_err(|_| Error::Syntax)?,
        ),
    })
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InvalidChecksum {
    pub expected: u8,
    pub actual: u8,
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

#[cfg(test)]
mod test {
    use super::*;

    fn simple_dict() -> Dictionary {
        Dictionary::empty()
    }

    #[test]
    fn can_parse_simple_message() {
        let msg = "8=FIX.4.2|9=251|35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|10=127|";
        let result = TagValue::new().decode_ws(&mut msg.as_bytes(), '|');
        assert!(result.is_ok());
    }

    #[test]
    fn message_must_end_with_separator() {
        let msg = "8=FIX.4.2|9=251|35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|10=127";
        let result = TagValue::new().decode_ws(&mut msg.as_bytes(), '|');
        assert_eq!(result, Err(Error::Eof));
    }

    #[test]
    fn message_without_checksum() {
        let msg = "8=FIX.4.4|9=251|35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|";
        let result = TagValue::new().decode_ws(&mut msg.as_bytes(), '|');
        assert_eq!(result, Err(Error::InvalidStandardTrailer));
    }

    #[test]
    fn message_without_standard_header() {
        let msg = "35=D|49=AFUNDMGR|56=ABROKERt|15=USD|59=0|10=000|";
        let result = TagValue::new().decode_ws(&mut msg.as_bytes(), '|');
        assert_eq!(result, Err(Error::InvalidStandardHeader));
    }

    #[test]
    fn detect_incorrect_checksum() {
        let msg = "8=FIX.4.2|9=251|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=126|";
        let result = TagValue::new().decode_ws(&mut msg.as_bytes(), '|');
    }
}
