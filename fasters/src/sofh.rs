//! Simple Open Framing Header (SOFH) support.
//!
//! SOFH provides encoding-agnostic message framing. By SOFH rules, each payload
//! is preceded by a header that consists of six (6) bytes, which contain
//! information regarding both
//! - payload's encoding type
//! - payload's total length
//!
//! For more information read the official documentation.

use std::convert::TryInto;
use std::io;

/// The FIX Trading Community has reserved some encodings.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EncodingType {
    Private(u8),
    SimpleBinaryEncodingV10BE,
    SimpleBinaryEncodingV10LE,
    Gcb,
    ASN1PER,
    ASN1BER,
    ASN1OER,
    TagValue,
    FixmlSchema,
    Fast,
    Json,
    Bson,
    Other(u16),
}

impl From<u16> for EncodingType {
    fn from(val: u16) -> Self {
        // https://www.fixtrading.org/standards/fix-sofh-online/#encoding_type-field
        match val {
            0x5be0 => EncodingType::SimpleBinaryEncodingV10BE,
            0xeb50 => EncodingType::SimpleBinaryEncodingV10LE,
            0x4700 => EncodingType::Gcb,
            0xa500 => EncodingType::ASN1PER,
            0xa501 => EncodingType::ASN1BER,
            0xa502 => EncodingType::ASN1OER,
            0xf000 => EncodingType::TagValue,
            0xf100 => EncodingType::FixmlSchema,
            0xfa01 | 0xfaff => EncodingType::Fast,
            0xf500 => EncodingType::Json,
            0xfb00 => EncodingType::Bson,
            x if (0x1..=0xff).contains(&x) => EncodingType::Private(x as u8),
            _ => EncodingType::Other(val),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Header {
    message_length: u32,
    encoding_type: u16,
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
pub struct Frame {
    pub message_length: u32,
    pub encoding_type: u16,
    pub payload: Vec<u8>,
}

pub struct OpenFrameIter<R: io::Read> {
    reader: R,
}

impl<R: io::Read> OpenFrameIter<R> {
    pub fn new(reader: R) -> Self {
        OpenFrameIter { reader }
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

impl<R: io::Read> Iterator for OpenFrameIter<R> {
    type Item = Result<Frame>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut header_buffer = [0u8; 6];
        if let Err(e) = self.reader.read_exact(&mut header_buffer) {
            return Some(Err(e.into()));
        }
        let header = Header::from(&header_buffer);
        if header.message_length < 6 {
            return Some(Err(Error::InvalidMessageLength));
        }
        let mut payload = vec![0u8; header.message_length as usize - 6];
        if let Err(e) = self.reader.read_exact(&mut payload) {
            return Some(Err(e.into()));
        }
        assert_eq!(payload.len(), header.message_length as usize - 6);
        let frame = Frame {
            message_length: header.message_length,
            encoding_type: header.encoding_type,
            payload,
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
    fn test_encode_than_decode_random_header(header: Header) -> bool {
        header == (&(<[u8; 6]>::from(header))).into()
    }

    #[test]
    fn test_private_encodings() {
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
    fn test_frame_too_short() {
        let bytes = vec![0u8, 0, 0, 4, 13, 37, 42];
        match OpenFrameIter::new(&bytes[..]).next() {
            Some(Err(Error::InvalidMessageLength)) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn test_frame_with_only_header() {
        let bytes = vec![0u8, 0, 0, 6, 13, 37];
        match OpenFrameIter::new(&bytes[..]).next() {
            Some(Ok(_)) => (),
            _ => panic!(),
        }
    }
}
