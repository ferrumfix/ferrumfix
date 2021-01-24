//! Simple Open Framing Header (SOFH) support.
//!
//! SOFH provides encoding-agnostic message framing. By SOFH rules, each payload
//! is preceded by a header that consists of six (6) bytes, which contain
//! information regarding both
//! - payload's encoding type
//! - payload's total length
//!
//! For more information read the official documentation.
//!
//! https://www.fixtrading.org/standards/fix-sofh-online/

use std::convert::TryInto;
use std::io;
use std::num::NonZeroU8;

/// Enumeration type mapped from the 16-bit raw space.
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum EncodingType {
    /// Value in the user-reserved space.
    Private(NonZeroU8),
    /// Simple Binary Encoding (SBE) v1.0, big-endian mode.
    /// https://www.fixtrading.org/standards/sbe/
    SimpleBinaryEncodingV10BE,
    /// Simple Binary Encoding (SBE) v1.0, little-endian mode.
    /// https://www.fixtrading.org/standards/sbe/
    SimpleBinaryEncodingV10LE,
    /// Google's "Protobuf".
    /// https://www.fixtrading.org/standards/gpb/
    Protobuf,
    /// ASN.1 with Packed Encoding Rules (PER).
    /// https://www.fixtrading.org/standards/asn1/
    Asn1PER,
    /// ASN.1 with Basic Encoding Rules (BER).
    /// https://www.fixtrading.org/standards/asn1/
    Asn1BER,
    /// ASN.1 with Octet Encoding Rules (OER).
    /// https://www.fixtrading.org/standards/asn1/
    Asn1OER,
    /// Tag-value (classic) encoding.
    /// https://www.fixtrading.org/standards/tagvalue/
    TagValue,
    /// FIXML.
    /// https://www.fixtrading.org/standards/fixml/
    FixmlSchema,
    /// FAST.
    /// https://www.fixtrading.org/standards/fast/
    Fast(NonZeroU8),
    /// JSON.
    /// https://www.fixtrading.org/standards/json/
    Json,
    /// BSON.
    /// https://www.fixtrading.org/standards/bson/
    Bson,
    /// Unknown value.
    Other(u16),
}

impl From<u16> for EncodingType {
    fn from(encoding_type: u16) -> Self {
        // https://www.fixtrading.org/standards/fix-sofh-online/#encoding_type-field
        match encoding_type {
            0x1..=0xFF => EncodingType::Private(u16_as_nonzero_u8(encoding_type, 0).unwrap()),
            0x4700 => EncodingType::Protobuf,
            0x5BE0 => EncodingType::SimpleBinaryEncodingV10BE,
            0xA500 => EncodingType::Asn1PER,
            0xA501 => EncodingType::Asn1BER,
            0xA502 => EncodingType::Asn1OER,
            0xEB50 => EncodingType::SimpleBinaryEncodingV10LE,
            0xF000 => EncodingType::TagValue,
            0xF100 => EncodingType::FixmlSchema,
            0xF500 => EncodingType::Json,
            0xFA01..=0xFAFF => {
                EncodingType::Fast(u16_as_nonzero_u8(encoding_type, 0xFA00).unwrap())
            }
            0xFB00 => EncodingType::Bson,
            _ => EncodingType::Other(encoding_type),
        }
    }
}

fn u16_as_nonzero_u8(value: u16, offset: u16) -> Option<NonZeroU8> {
    NonZeroU8::new((value - offset).try_into().ok()?)
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
    message_length: u32,
    encoding_type: u16,
    payload: Vec<u8>,
}

impl Frame {
    pub fn encoding_type(&self) -> u16 {
        self.encoding_type
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload[..]
    }
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
        match OpenFrameIter::new(&bytes[..]).next() {
            Some(Err(Error::InvalidMessageLength)) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn frame_with_only_header_is_valid() {
        let bytes = vec![0u8, 0, 0, 6, 13, 37];
        match OpenFrameIter::new(&bytes[..]).next() {
            Some(Ok(_)) => (),
            _ => panic!(),
        }
    }
}
