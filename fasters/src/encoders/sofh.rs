//! Simple Open Framing Header (SOFH) support.
//!
//! SOFH provides encoding-agnostic message framing. By SOFH rules, each payload
//! is preceded by a header that consists of six (6) bytes, which contain
//! information regarding both
//! - payload's encoding type
//! - payload's total length
//!
//! Please refer to https://www.fixtrading.org/standards/fix-sofh/ for more
//! information.

use std::convert::TryInto;
use std::io;

/// Enumeration type mapped from the 16-bit raw space.
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum EncodingType {
    /// User-specified encoding type. Legal values and their respective semantics
    /// ought to be agreed upon out-of-band by counterparties.
    Private(u8),
    /// Simple Binary Encoding (SBE) v1.0, big-endian mode.
    /// Please refer to https://www.fixtrading.org/standards/sbe/ for more
    /// information.
    SimpleBinaryEncodingV10BE,
    /// Simple Binary Encoding (SBE) v1.0, little-endian mode.
    /// Please refer to https://www.fixtrading.org/standards/sbe/ for more
    /// information.
    SimpleBinaryEncodingV10LE,
    /// Google's "Protobuf".
    /// Please refer to https://www.fixtrading.org/standards/gpb/ for more
    /// information.
    Protobuf,
    /// ASN.1 with Packed Encoding Rules (PER).
    /// Please refer to https://www.fixtrading.org/standards/asn1/ for more
    /// information.
    Asn1PER,
    /// ASN.1 with Basic Encoding Rules (BER).
    /// Please refer to https://www.fixtrading.org/standards/asn1/ for more
    /// information.
    Asn1BER,
    /// ASN.1 with Octet Encoding Rules (OER).
    /// Please refer to https://www.fixtrading.org/standards/asn1/ for more
    /// information.
    Asn1OER,
    /// Tag-value (classic) encoding.
    /// Please refer to https://www.fixtrading.org/standards/tagvalue/ for more
    /// information.
    TagValue,
    /// FIXML encoding.
    /// Please refer to https://www.fixtrading.org/standards/fixml/ for more
    /// information.
    FixmlSchema,
    /// FAST encoding.
    /// Please refer to https://www.fixtrading.org/standards/fast/ for more
    /// information.
    Fast(u8),
    /// JSON encoding.
    /// Please refer to https://www.fixtrading.org/standards/json/ for more
    /// information.
    Json,
    /// BSON encoding.
    /// Please refer to https://www.fixtrading.org/standards/bson/ for more
    /// information.
    Bson,
    /// Unknown value.
    Unknown(u16),
}

impl From<u16> for EncodingType {
    fn from(encoding_type: u16) -> Self {
        // https://www.fixtrading.org/standards/fix-sofh-online/#encoding_type-field
        match encoding_type {
            0x1..=0xFF => EncodingType::Private(encoding_type as u8),
            0x4700 => EncodingType::Protobuf,
            0x5BE0 => EncodingType::SimpleBinaryEncodingV10BE,
            0xA500 => EncodingType::Asn1PER,
            0xA501 => EncodingType::Asn1BER,
            0xA502 => EncodingType::Asn1OER,
            0xEB50 => EncodingType::SimpleBinaryEncodingV10LE,
            0xF000 => EncodingType::TagValue,
            0xF100 => EncodingType::FixmlSchema,
            0xF500 => EncodingType::Json,
            0xFA01..=0xFAFF => EncodingType::Fast((encoding_type - 0xFA00) as u8),
            0xFB00 => EncodingType::Bson,
            _ => EncodingType::Unknown(encoding_type),
        }
    }
}

impl From<EncodingType> for u16 {
    fn from(encoding_type: EncodingType) -> Self {
        match encoding_type {
            EncodingType::Private(x) => x as u16,
            EncodingType::Protobuf => 0x4700,
            EncodingType::SimpleBinaryEncodingV10BE => 0x5BE0,
            EncodingType::Asn1PER => 0xA500,
            EncodingType::Asn1BER => 0xA501,
            EncodingType::Asn1OER => 0xA502,
            EncodingType::SimpleBinaryEncodingV10LE => 0xEB50,
            EncodingType::TagValue => 0xF000,
            EncodingType::FixmlSchema => 0xF100,
            EncodingType::Json => 0xF500,
            EncodingType::Fast(x) => 0xFA00u16 + (x as u16),
            EncodingType::Bson => 0xFB00,
            EncodingType::Unknown(x) => x,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Header {
    pub message_length: u32,
    pub encoding_type: u16,
}

impl From<Header> for [u8; 6] {
    fn from(header: Header) -> Self {
        let mut bytes = [0u8; 6];
        bytes[0..4].copy_from_slice(&header.message_length.to_be_bytes());
        bytes[4..6].copy_from_slice(&header.encoding_type.to_be_bytes());
        bytes
    }
}

impl From<&[u8; 6]> for Header {
    fn from(bytes: &[u8; 6]) -> Self {
        Header {
            message_length: u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
            encoding_type: u16::from_be_bytes(bytes[4..6].try_into().unwrap()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Frame<'a> {
    encoding_type: u16,
    payload: &'a [u8],
}

impl<'a> Frame<'a> {
    pub fn encoding_type(&self) -> u16 {
        self.encoding_type
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload[..]
    }
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    InvalidMessageLength,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

type Result<T> = std::result::Result<T, Error>;

/// A parser for Simple Open Framing Header (SOFH) -encoded messages.
pub struct SofhParser {
    buffer: Vec<u8>,
}

impl SofhParser {
    pub fn new() -> Self {
        Self::with_capacity(1024)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity)
        }
    }

    pub fn iter_frames<'a, R: io::Read + 'a>(
        &'a mut self,
        reader: R,
    ) -> FramesIter<'a, R> {
        FramesIter { reader, parser: self }
    }

    pub fn reserved(&self) -> usize {
        self.buffer.capacity()
    }
}

pub struct FramesIter<'a, R: io::Read> {
    reader: R,
    parser: &'a mut SofhParser,
}

impl<'a, R: io::Read> FramesIter<'a, R> {

    pub fn next<'b>(&'b mut self) -> Option<Result<Frame<'b>>>{
        let mut header_buffer = [0u8; 6];
        if let Err(e) = self.reader.read_exact(&mut header_buffer) {
            return Some(Err(e.into()));
        }
        let header = Header::from(&header_buffer);
        if header.message_length < 6 {
            return Some(Err(Error::InvalidMessageLength));
        }
        self.parser.buffer.resize(header.message_length as usize - 6, 0);
        if let Err(e) = self.reader.read_exact(&mut self.parser.buffer[..]) {
            return Some(Err(e.into()));
        }
        assert_eq!(self.parser.buffer.len(), header.message_length as usize - 6);
        let frame = Frame {
            encoding_type: header.encoding_type,
            payload: &self.parser.buffer[..],
        };
        Some(Ok(frame))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    // https://github.com/BurntSushi/quickcheck#generating-structs
    impl Arbitrary for Header {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            Header {
                message_length: u32::arbitrary(g),
                encoding_type: u16::arbitrary(g),
            }
        }
    }

    #[quickcheck]
    fn encode_than_decode_random_header(header: Header) -> bool {
        header == (&(<[u8; 6]>::from(header))).into()
    }

    #[test]
    fn private_encodings() {
        for val in &[0x1, 0x82, 0xff] {
            match EncodingType::from(*val) {
                EncodingType::Private(_) => (),
                _ => panic!(),
            };
        }
        for val in &[0x0, 0x100] {
            if let EncodingType::Private(_) = EncodingType::from(*val) {
                panic!();
            }
        }
    }

    #[test]
    fn frame_too_short() {
        let bytes = vec![0u8, 0, 0, 4, 13, 37, 42];
        let mut frames = SofhParser::new().iter_frames(&bytes[..]);
        match frames.next() {
            Some(Err(Error::InvalidMessageLength)) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn frame_with_only_header_is_valid() {
        let bytes = vec![0u8, 0, 0, 6, 13, 37];
        let mut frames = SofhParser::new().iter_frames(&bytes[..]);
        match frames.next() {
            Some(Ok(_)) => (),
            _ => panic!(),
        }
    }
}
