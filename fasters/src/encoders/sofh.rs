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

use super::Codec;
use super::Poll;
use crate::stream_iterator::StreamIterator;
use std::convert::TryInto;
use std::fmt;
use std::io;

pub use encoding_type::EncodingType;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Header {
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

/// A parser for Simple Open Framing Header (SOFH) -encoded messages.
pub struct SofhParser {
    buffer: Vec<u8>,
    next_size: usize,
    frame: Frame,
}

impl SofhParser {
    /// Creates a new SOFH parser with default buffer size.
    pub fn new() -> Self {
        Self::with_capacity(1024)
    }

    /// Creates a new [`SofhParser`](SofhParser) with a buffer large enough to
    /// hold `capacity` amounts of bytes without reallocating.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut parser = SofhParser {
            buffer: Vec::with_capacity(capacity),
            next_size: 1024,
            frame: Frame {
                encoding_type: 0,
                data: std::ptr::null(),
                len: 0,
            },
        };
        parser.frame.data = parser.buffer.as_ptr();
        parser.frame.len = parser.buffer.len();
        parser
    }

    /// Returns the current buffer capacity of this [`SofhParser`]. This value is
    /// subject to change after every incoming message.
    ///
    /// # Examples
    ///
    /// ```
    /// use fasters::encoders::sofh::SofhParser;
    ///
    /// let parser = SofhParser::with_capacity(8192);
    /// assert_eq!(parser.capacity(), 8192);
    /// ```
    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    /// Returns a [`StreamIterator`](StreamIterator) over the message frames
    /// produced by `source`.
    pub fn read_frames<'a, R: io::Read + 'a>(
        &'a mut self,
        source: R,
    ) -> impl StreamIterator<Item = Result<Frame>> + 'a {
        Frames {
            parser: self,
            source,
            // Dummy placeholder value.
            result: Err(Error::InvalidMessageLength(0)),
        }
    }
}

impl<'s> Codec<'s, Frame> for SofhParser
{
    type DecodeError = Error;
    type EncodeError = ();

    fn decode(&mut self, data: &[u8]) -> Result<Poll> {
        self.buffer.extend_from_slice(data);
        if self.next_size == 0 {
            self.next_size = u32::from_be_bytes(self.buffer[0..4].try_into().unwrap()) as usize;
        }
        match self.next_size {
            0 => Ok(Poll::Incomplete),
            len => {
                if self.buffer.len() >= len {
                    let frame = Frame {
                        encoding_type: 0,
                        data: self.buffer.as_ptr(),
                        len: self.buffer.len(),
                    };
                    Ok(Poll::Ready)
                } else {
                    Ok(Poll::Incomplete)
                }
            }
        }
    }

    fn get_item(&self) -> Frame {
        Frame {
            encoding_type: self.frame.encoding_type(),
            data: self.buffer.as_ptr(),
            len: self.buffer.len(),
        }
    }

    fn encode(&mut self, data: Frame) -> std::result::Result<&[u8], Self::EncodeError> {
        debug_assert!(self.buffer.is_empty());
        self.buffer
            .extend_from_slice(&(data.payload().len() as u32).to_be_bytes()[..]);
        self.buffer
            .extend_from_slice(&(data.encoding_type() as u16).to_be_bytes()[..]);
        self.buffer.extend_from_slice(data.payload());
        Ok(&self.buffer[..])
    }
}

/// A non-owning message frame, with an internal pointer to the buffer that
/// contains the raw data.
#[derive(Debug)]
pub struct Frame {
    encoding_type: u16,
    data: *const u8,
    len: usize,
}

impl Frame {
    /// Returns the encoding type for this message.
    pub fn encoding_type(&self) -> u16 {
        self.encoding_type
    }

    /// Returns an immutable reference to the internal buffer that contains the
    /// message payload.
    pub fn payload(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data, self.len) }
    }
}

/// A support structure to iterate over frames.
pub struct Frames<'a, R> {
    parser: &'a mut SofhParser,
    source: R,
    result: Result<Frame>,
}

impl<'a, R> StreamIterator for Frames<'a, R>
where
    R: io::Read,
{
    type Item = Result<Frame>;

    fn advance(&mut self) {
        let mut header_buffer = [0u8; 6];
        if let Err(e) = self.source.read_exact(&mut header_buffer) {
            self.result = Err(e.into());
            return;
        }
        let header = Header::from(&header_buffer);
        if header.message_length < 6 {
            self.result = Err(Error::InvalidMessageLength(header.message_length));
            return;
        }
        self.parser
            .buffer
            .resize(header.message_length as usize - 6, 0);
        if let Err(e) = self.source.read_exact(&mut self.parser.buffer[..]) {
            self.result = Err(e.into());
            return;
        }
        debug_assert_eq!(self.parser.buffer.len(), header.message_length as usize - 6);
        self.result = Ok(Frame {
            encoding_type: 0,
            data: self.parser.buffer.as_ptr(),
            len: self.parser.buffer.len(),
        });
    }

    fn get(&self) -> Option<&Self::Item> {
        Some(&self.result)
    }
}

/// A specialized [`Result`](std::result::Result) for SOFH decoding operations.
/// Encoding, on the other hand, is infallible and will never result in an error.
type Result<T> = std::result::Result<T, Error>;

/// The error type that can be returned if some error occurs during SOFH parsing.
#[derive(Debug)]
pub enum Error {
    InvalidMessageLength(u32),
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => {
                writeln!(f, "I/O error while reading the message.")?;
                (*err).fmt(f)
            }
            Error::InvalidMessageLength(len) => {
                writeln!(
                    f,
                    "Message length is {} but it must be greater than or equal to 6.",
                    len
                )
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

mod encoding_type {

    /// Enumeration type mapped from the 16-bit raw space.
    #[derive(Copy, Clone, Debug)]
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

    impl PartialEq for EncodingType {
        fn eq(&self, other: &Self) -> bool {
            u16::from(*self) == u16::from(*other)
        }
    }

    impl std::cmp::Eq for EncodingType {}
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::{Arbitrary, Gen, QuickCheck};
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

    impl Arbitrary for EncodingType {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            // Note: u8 intervals e.g. Private and Fast can't have 0.
            match u16::arbitrary(g) % 14 {
                0 => Self::Private(u8::arbitrary(g).max(1)),
                1 => Self::SimpleBinaryEncodingV10BE,
                2 => Self::SimpleBinaryEncodingV10LE,
                3 => Self::Protobuf,
                4 => Self::Asn1PER,
                5 => Self::Asn1BER,
                6 => Self::Asn1OER,
                7 => Self::TagValue,
                8 => Self::FixmlSchema,
                9 => Self::Fast(u8::arbitrary(g).max(1)),
                10 => Self::Json,
                11 => Self::Bson,
                n => n.into(),
            }
        }
    }

    #[quickcheck]
    fn encode_than_decode_random_header(header: Header) -> bool {
        header == (&(<[u8; 6]>::from(header))).into()
    }

    #[test]
    fn encoding_type_to_u16_then_back() {
        fn prop(val: EncodingType) -> bool {
            val == EncodingType::from(u16::from(val))
        }
        QuickCheck::new()
            .tests(1000)
            .quickcheck(prop as fn(EncodingType) -> bool)
    }

    #[test]
    fn u16_to_encoding_type_then_back() {
        fn prop(val: u16) -> bool {
            val == u16::from(EncodingType::from(val))
        }
        QuickCheck::new()
            .tests(1000)
            .quickcheck(prop as fn(u16) -> bool)
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
        let mut parser = SofhParser::new();
        let mut frames = parser.read_frames(&bytes[..]);
        match frames.next() {
            Some(Err(Error::InvalidMessageLength(_))) => (),
            _ => panic!(),
        }
    }

    #[test]
    fn frame_with_only_header_is_valid() {
        let bytes = vec![0u8, 0, 0, 6, 13, 37];
        let mut parser = SofhParser::new();
        let mut frames = parser.read_frames(&bytes[..]);
        match frames.next() {
            Some(Ok(_)) => (),
            _ => panic!(),
        }
    }
}
