use crate::encodings::Encoding;
use crate::ir;
use crate::spec::{BaseType, Dictionary};
use std::io;
use std::str;

/// "Start of heading" (SOH) control character (ASCII `0x1`). Each tag-value pair
/// MUST be followed by this control character.
const SOH_SEPARATOR: char = 1u8 as char;

/// The FIX TagValue Encoding is the original and most widely used encoding. It's
/// simple ASCII string format.
pub struct TagValue {
    dict: Dictionary,
    separator: char,
}

type DecodeResult<T> = Result<T, <TagValue as Encoding>::DecodeErr>;
type EncodeResult<T> = Result<T, <TagValue as Encoding>::EncodeErr>;

impl TagValue {
    /// Builds a new `TagValue` encoding device with an empty FIX dictionary.
    pub fn new() -> Self {
        TagValue {
            dict: Dictionary::empty(),
            separator: SOH_SEPARATOR,
        }
    }

    fn decode_checksum(
        &self,
        source: &mut impl io::BufRead,
        message: &mut ir::Message,
    ) -> DecodeResult<u8> {
        let field = parse_field(source, self.separator, &|_: i64| BaseType::Int)?;
        if let ir::FixFieldValue::Int(checksum) = field.value {
            message.fields.insert(field.tag, field.value);
            Ok(checksum as u8)
        } else {
            Err(Error::Syntax)
        }
    }

    fn decode_ws(
        &self,
        source: &mut impl io::BufRead,
        separator: char,
    ) -> Result<ir::Message, <TagValue as Encoding>::DecodeErr> {
        let tag_datatype = |t| self.dict.fields.get(&t).unwrap();
        let typer = &|t| str_to_basetype(tag_datatype(t as usize).data_type.as_str());
        let (mut checksum, f0, f1, f2) = parse_standard_header(source, separator)?;
        let mut fields = vec![f0, f1, f2];
        let mut message = ir::Message::new();
        let body_length = 11; //f1.value;
        let mut body_length_remaining = body_length;
        let mut expect_data_field = false;
        while body_length_remaining > 0 {
            let field = parse_field(source, separator, typer)?;
            checksum.roll(&[field.checksum]);
            body_length_remaining -= field.len;
            fields.push(field);
            expect_data_field = false;
        }
        // `CheckSum(10)` doesn't count towards `BodyLength(9)` quota, so we
        // must read it outside of the main loop.
        let actual_checksum = self.decode_checksum(source, &mut message)?;
        if checksum.0 != actual_checksum {
            Err(Error::InvalidChecksum(InvalidChecksum {
                expected: checksum.0.into(),
                actual: actual_checksum,
            }))
        } else {
            let mut message = ir::Message::new();
            for f in fields {
                message.fields.insert(f.tag, f.value);
            }
            Ok(message)
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
        for (key, value) in message.fields {
            let field = ir::Field {
                tag: key,
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

impl ir::Field {
    fn encode(&self, write: &mut impl io::Write) -> EncodeResult<()> {
        write.write(self.tag.to_string().as_bytes())?;
        write.write(&['=' as u8])?;
        match &self.value {
            ir::FixFieldValue::Char(c) => write.write(&[*c as u8])?,
            ir::FixFieldValue::String(s) => write.write(s.as_bytes())?,
            ir::FixFieldValue::Int(int) => write.write(int.to_string().as_bytes())?,
            ir::FixFieldValue::Float(float) => write.write(float.to_string().as_bytes())?,
            ir::FixFieldValue::Data(raw_data) => write.write(&raw_data)?,
        };
        write.write(&[1u8])?;
        Ok(())
    }
}

fn str_to_basetype(s: &str) -> BaseType {
    match s {
        "string" => BaseType::String,
        "char" => BaseType::Char,
        "int" => BaseType::Int,
        "float" => BaseType::Float,
        "data" => BaseType::Data,
        _ => BaseType::Char, // FIXME
    }
}

/// A rolling checksum over a byte array. Sums over each byte wrapping around at
/// 256.
struct Checksum(u8, usize);

impl Checksum {
    fn new() -> Self {
        Checksum(0, 0)
    }

    fn roll(&mut self, window: &[u8]) {
        self.0 = self.0.wrapping_add(window.iter().sum());
        self.1 += window.len();
    }

    fn window_length(&self) -> usize {
        self.1
    }
}

impl From<&Checksum> for u8 {
    fn from(val: &Checksum) -> Self {
        val.0
    }
}

fn parse_standard_header(
    mut source: &mut impl io::BufRead,
    separator: char,
) -> Result<(Checksum, ir::Field, ir::Field, ir::Field), <TagValue as Encoding>::DecodeErr> {
    let field_0 = parse_field(source, separator, &|t| BaseType::String)?;
    let field_1 = parse_field(source, separator, &|t| BaseType::Int)?;
    let field_2 = parse_field(source, separator, &|t| BaseType::Int)?;
    let mut checksum = Checksum::new();
    checksum.roll(&[field_0.checksum, field_1.checksum, field_2.checksum]);
    Ok((checksum, field_0, field_1, field_2))
}

fn parse_data_field(
    source: &mut impl io::BufRead,
    separator: char,
    length: usize,
) -> Result<ir::Field, <TagValue as Encoding>::DecodeErr> {
    let mut buffer = Vec::new();
    let mut checksum = Checksum::new();
    source
        .read_until('=' as u8, &mut buffer)
        .map_err(|_| Error::Syntax)?;
    checksum.roll(&buffer[..]);
    let tag = std::str::from_utf8(&buffer[..])
        .unwrap()
        .parse::<i64>()
        .unwrap();
    buffer = Vec::with_capacity(length);
    checksum.roll(&buffer[..]);
    checksum.roll(&[separator as u8]);
    source
        .read_exact(&mut buffer[..])
        .map_err(|_| Error::Syntax)?;
    Ok(ir::Field {
        tag,
        value: ir::FixFieldValue::Data(buffer),
        checksum: checksum.0,
        len: checksum.window_length(),
    })
}

fn parse_field(
    source: &mut impl io::BufRead,
    separator: char,
    typer: &dyn Fn(i64) -> BaseType,
) -> Result<ir::Field, <TagValue as Encoding>::DecodeErr> {
    let mut buffer = Vec::new();
    let mut checksum = Checksum::new();
    source
        .read_until('=' as u8, &mut buffer)
        .map_err(|_| Error::Syntax)?;
    checksum.roll(&buffer[..]);
    let tag = str::from_utf8(&buffer[..])
        .map_err(|_| Error::Syntax)?
        .parse::<i64>()
        .map_err(|_| Error::Syntax)?;
    let datatype = typer(tag);
    source.read_until(separator as u8, &mut buffer);
    checksum.roll(&buffer[..]);
    let field_value = field_value(datatype, &buffer[..])?;
    Ok(ir::Field {
        tag,
        value: field_value,
        checksum: checksum.0,
        len: checksum.window_length(),
    })
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

fn compute_checksum(buf: &[u8]) -> u8 {
    buf.iter().sum()
}

pub struct InvalidChecksum {
    pub expected: u8,
    pub actual: u8,
}

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
        Dictionary {
            version: "version-foobar".to_string(),
            data_types: HashMap::default(),
            fields: HashMap::default(),
            components: HashMap::default(),
            messages: HashMap::default(),
            msg_contents: HashMap::default(),
        }
    }

    #[test]
    fn test_simple_message() {
        let msg = "8=FIX.4.2^9=251^35=D^49=AFUNDMGR^56=ABROKERt^15=USD^59=0^10=127";
        let result = TagValue::new().decode(&mut msg.as_bytes());
    }

    #[test]
    fn test_detect_incorrect_checksum() {
        let msg = "8=FIX.4.2^9=251^35=D^49=AFUNDMGR^56=ABROKERt^15=USD^59=0^10=126";
        let result = TagValue::new().decode(&mut msg.as_bytes());
    }
}
