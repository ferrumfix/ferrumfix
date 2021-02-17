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

use super::{Encoding, StreamingDecoder};
use crate::buffering::Buffer;
use std::convert::TryInto;
use std::default::Default;
use std::fmt;
use std::io;

const HEADER_SIZE_IN_BYTES: usize = 6;

/// An immutable view into a SOFH-enclosed message, complete with its
/// encoding type tag and payload.
///
/// This type is returned in a borrowed form from
/// [`sofh::Codec::decode`](sofh::Codec::decode) and
/// there is no owned version of this type.
#[derive(Debug)]
pub struct Frame {
    encoding_type: u16,
    buffer: *const u8,
    buffer_len: usize,
}

impl Frame {
    /// Creates a new [`Frame`](Frame) that points to `data`.
    ///
    /// # Safety
    /// This function is marked `unsafe` because it returns an owned `Frame`,
    /// which should **never** be possible to an end user ([`Frame`](Frame)s
    /// should only
    /// be borrowed for as long as the underlying buffer lives).
    unsafe fn new(encoding_type: u16, data: &[u8]) -> Self {
        Self {
            encoding_type,
            buffer: data.as_ptr(),
            buffer_len: data.len(),
        }
    }

    /// Returns the 16-bits encoding type of `self`. You may want to
    /// convert this value to an [`EncodingType`](EncodingType), which allows
    /// for nice pattern matching.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::codec::Decoder;
    /// use fefix::codec::sofh::Codec;
    ///
    /// let frame = Codec::decode(&[0, 0, 0, 0, 0, 1, 42]).unwrap();
    /// let encoding_type = frame.encoding_type().into();
    /// assert_eq!(encoding_type, EncodingType::Json);
    /// ```
    pub fn encoding_type(&self) -> u16 {
        self.encoding_type
    }

    /// Returns an immutable reference to the actual contents of `self`, i.e.
    /// without its header.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::codec::Decoder;
    /// use fefix::codec::sofh::Codec;
    ///
    /// let frame = Codec::decode(&[0, 0, 0, 1, 0, 0, 42]).unwrap();
    /// let payload = frame.payload();
    /// assert_eq!(payload, &[42]);
    /// ```
    pub fn payload(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.buffer, self.buffer_len) }
    }
}

/// A codec for SOFH-enclosed payloads.
#[derive(Debug)]
pub struct Codec {
    frame: Frame,
}

impl Codec {
    /// Turns `self` into a [`StreamingDecoder`](StreamingDecoder) -enabled codec
    /// by allocating an internal buffer.
    pub fn buffered(self) -> BufferedCodec {
        BufferedCodec {
            buffer: Vec::new(),
            buffer_relevant_len: 0,
            buffer_additional_len: 0,
            codec: self,
        }
    }

    /// Turns `self` into a [`StreamingDecoder`](StreamingDecoder) -enabled codec
    /// by allocating an internal buffer. The allocated buffer will have an
    /// initial capacity of `capacity` in bytes.
    pub fn buffered_with_capacity(self, capacity: usize) -> BufferedCodec {
        BufferedCodec {
            buffer: vec![0; capacity],
            buffer_relevant_len: 0,
            buffer_additional_len: 0,
            codec: self,
        }
    }
}

impl Default for Codec {
    fn default() -> Self {
        Self {
            frame: unsafe { Frame::new(0, &[]) },
        }
    }
}

impl Encoding<Frame> for Codec {
    type DecodeError = DecodeError;
    type EncodeError = EncodeError;

    fn decode(&mut self, data: &[u8]) -> Result<&Frame, Self::DecodeError> {
        if data.len() < HEADER_SIZE_IN_BYTES {
            return Err(Self::DecodeError::InvalidMessageLength);
        }
        // Note that the message length field also includes the header.
        if data.len() != get_field_message_length(data) as usize {
            return Err(Self::DecodeError::InvalidMessageLength);
        }
        let encoding_type = get_field_encoding_type(data);
        self.frame = Frame {
            encoding_type,
            buffer: data.as_ptr(),
            buffer_len: data.len(),
        };
        Ok(&self.frame)
    }

    fn encode<B>(
        &mut self,
        buffer: &mut B,
        frame: &Frame,
    ) -> std::result::Result<usize, Self::EncodeError>
    where
        B: Buffer,
    {
        let len = frame.payload().len();
        let body_len: u32 = len.try_into().map_err(|_| EncodeError::TooLong)?;
        let field_message_length = body_len.to_be_bytes();
        let field_encoding_type = frame.encoding_type().to_be_bytes();
        buffer.extend_from_slice(&field_message_length[..]);
        buffer.extend_from_slice(&field_encoding_type[..]);
        buffer.extend_from_slice(frame.payload());
        Ok(buffer.as_slice().len())
    }
}

fn get_field_message_length(data: &[u8]) -> u32 {
    u32::from_be_bytes(data[0..4].try_into().unwrap())
}

fn get_field_encoding_type(data: &[u8]) -> u16 {
    u16::from_be_bytes(data[4..6].try_into().unwrap())
}

/// A parser for SOFH-enclosed messages.
///
/// SOFH stands for Simple Open Framing Header and it's an encoding-agnostic
/// framing mechanism for variable-length messages. It was developed by the FIX
/// High Performance Group to allow message processors and communication gateways
/// to determine the length and the data format of incoming messages.
#[derive(Debug)]
pub struct BufferedCodec {
    buffer: Vec<u8>,
    buffer_relevant_len: usize,
    buffer_additional_len: usize,
    codec: Codec,
}

impl BufferedCodec {
    /// Creates a new SOFH parser with default buffer size.
    pub fn new() -> Self {
        Self::with_capacity(1024)
    }

    /// Creates a new [`Codec`](Codec) with a buffer large enough to
    /// hold `capacity` amounts of bytes without reallocating.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: vec![0; capacity],
            buffer_relevant_len: 0,
            buffer_additional_len: 0,
            codec: Codec::default(),
        }
    }

    /// Returns the current buffer capacity of this [`Codec`]. This value is
    /// subject to change after every incoming message.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::codec::sofh::Codec;
    ///
    /// let parser = Codec::with_capacity(8192);
    /// assert_eq!(parser.capacity(), 8192);
    /// ```
    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }
}

impl Encoding<Frame> for BufferedCodec {
    type DecodeError = DecodeError;
    type EncodeError = EncodeError;

    fn decode(&mut self, data: &[u8]) -> Result<&Frame, Self::DecodeError> {
        self.codec.decode(data)
    }

    fn encode<B>(
        &mut self,
        buffer: &mut B,
        frame: &Frame,
    ) -> std::result::Result<usize, Self::EncodeError>
    where
        B: Buffer,
    {
        self.codec.encode(buffer, frame)
    }
}

impl StreamingDecoder<Frame> for BufferedCodec {
    type Error = DecodeError;

    fn supply_buffer(&mut self) -> (&mut usize, &mut [u8]) {
        let len = if self.buffer_relevant_len < 4 {
            // A good default.
            512
        } else {
            // We can calculate how many additional bytes we need.
            let payload_size = get_field_message_length(&self.buffer[..]);
            payload_size as usize - self.buffer_relevant_len
        };
        self.buffer.resize(self.buffer_relevant_len + len, 0);
        (
            &mut self.buffer_additional_len,
            &mut self.buffer[self.buffer_relevant_len..self.buffer_relevant_len + len],
        )
    }

    fn attempt_decoding(&mut self) -> Result<Option<&Frame>, Self::Error> {
        self.buffer_relevant_len += self.buffer_additional_len;
        assert!(self.buffer_relevant_len <= self.buffer.len());
        // We don't even have a full header, so don't bother.
        if self.buffer_relevant_len < HEADER_SIZE_IN_BYTES {
            return Ok(None);
        }
        let expected_payload_size = get_field_message_length(&self.buffer[..]);
        if self.buffer_relevant_len < expected_payload_size as usize {
            // The payload is incomplete.
            Ok(None)
        } else {
            self.codec
                .decode(&self.buffer[..self.buffer_relevant_len])
                .map(|frame| Some(frame))
        }
    }

    fn get(&self) -> &Frame {
        &self.codec.frame
    }
}

/// The error type that can be returned if some error occurs during SOFH parsing.
#[derive(Debug)]
pub enum DecodeError {
    /// The provided message length is outside the legal range.
    InvalidMessageLength,
    /// I/O-related error.
    Io(io::Error),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::Io(err) => {
                writeln!(f, "I/O error while reading the message.")?;
                (*err).fmt(f)
            }
            DecodeError::InvalidMessageLength => {
                writeln!(f, "Message length must be greater than or equal to 6.",)
            }
        }
    }
}

impl From<io::Error> for DecodeError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

/// The error type that can be returned when attempting to serialize a SOFH-enclosed
/// payload.
#[derive(Debug, Clone)]
pub enum EncodeError {
    /// The assigned payload is too big to fit in a single SOFH-enclosed
    /// message.
    TooLong,
}

/// Sum type that can be converted [`Into`](Into) and [`From`](From) a
/// `u16` encoding type value.
///
/// Each variant has a predetermined value or range of values, as specified by
/// the official guidelines. This type is marked with `#[non_exhaustive]` to
/// support new encoding types without breaking compatibility;
/// [`EncodingType::Unknown`](EncodingType::Unknown) contains all values that are
/// not included in the official guidelines; this way conversion is infallible
/// and doesn't lose any information.
///
/// # Equality
///
/// It's important to note that the behavior of [`Eq`](Eq) and
/// [`PartialEq`](PartialEq) for this type always falls back to equality on
/// `u16`. This may cause unusual behavior e.g.:
///
/// ```
/// use fefix::codec::sofh::EncodingType;
///
/// let e_type = EncodingType::Unknown(0xF500);
/// assert_eq!(e_type, EncodingType::Json);
/// ```
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum EncodingType {
    /// User-specified encoding type. Legal values and their respective semantics
    /// ought to be agreed upon out-of-band by counterparties.
    ///
    /// Please note that `0x0` is *not* a valid [`EncodingType::Private`] value.
    Private(u8),
    /// Simple Binary Encoding (SBE) v1.0, big-endian mode.
    /// Please refer to <https://www.fixtrading.org/standards/sbe/> for more
    /// information.
    SimpleBinaryEncodingV10BE,
    /// Simple Binary Encoding (SBE) v1.0, little-endian mode.
    /// Please refer to <https://www.fixtrading.org/standards/sbe/> for more
    /// information.
    SimpleBinaryEncodingV10LE,
    /// Google's "Protobuf".
    /// Please refer to <https://www.fixtrading.org/standards/gpb/> for more
    /// information.
    Protobuf,
    /// ASN.1 with Packed Encoding Rules (PER).
    /// Please refer to <https://www.fixtrading.org/standards/asn1/> for more
    /// information.
    Asn1PER,
    /// ASN.1 with Basic Encoding Rules (BER).
    /// Please refer to <https://www.fixtrading.org/standards/asn1/> for more
    /// information.
    Asn1BER,
    /// ASN.1 with Octet Encoding Rules (OER).
    /// Please refer to <https://www.fixtrading.org/standards/asn1/> for more
    /// information.
    Asn1OER,
    /// Tag-value (classic) encoding.
    /// Please refer to <https://www.fixtrading.org/standards/tagvalue/> for more
    /// information.
    TagValue,
    /// FIXML encoding.
    /// Please refer to <https://www.fixtrading.org/standards/fixml/> for more
    /// information.
    FixmlSchema,
    /// FAST encoding.
    /// Please refer to <https://www.fixtrading.org/standards/fast/> for more
    /// information.
    ///
    /// Please note that `0xFA00` is *not* a valid [`EncodingType::Fast`] value.
    Fast(u8),
    /// JSON encoding.
    /// Please refer to <https://www.fixtrading.org/standards/json/> for more
    /// information.
    Json,
    /// BSON encoding.
    /// Please refer to <https://www.fixtrading.org/standards/bson/> for more
    /// information.
    Bson,
    /// Unknown value.
    Unknown(u16),
}

const ENCODING_TYPE_FAST_OFFSET: u16 = 0xFA00;

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
            0xFA01..=0xFAFF => {
                EncodingType::Fast((encoding_type - ENCODING_TYPE_FAST_OFFSET) as u8)
            }
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
            EncodingType::Fast(x) => ENCODING_TYPE_FAST_OFFSET + (x as u16),
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

impl std::hash::Hash for EncodingType {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        u16::from(*self).hash(state)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::codec::FramelessError;
    use crate::codec::StreamingDecoder;

    fn _frames_with_increasing_length() -> impl Iterator<Item = Vec<u8>> {
        std::iter::once(()).enumerate().map(|(i, ())| {
            let header = encode_header(i as u32 + 6, 0);
            let mut buffer = Vec::new();
            buffer.extend_from_slice(&header[..]);
            for _ in 0..i {
                buffer.extend_from_slice(&[0]);
            }
            buffer
        })
    }

    struct Reader<T> {
        source: T,
    }

    impl<T> std::io::Read for Reader<T>
    where
        T: Iterator<Item = u8>,
    {
        fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
            for i in 0..buffer.len() {
                buffer[i] = self.source.next().unwrap();
            }
            Ok(buffer.len())
        }
    }

    fn _increasing_frames_as_read() -> impl std::io::Read {
        let stream = _frames_with_increasing_length()
            .map(|vec| vec.into_iter())
            .flatten();
        Reader { source: stream }
    }

    fn encode_header(len: u32, encoding_type: u16) -> [u8; 6] {
        let a = len.to_be_bytes();
        let b = encoding_type.to_be_bytes();
        let mut bytes = [0u8; 6];
        bytes[0..4].copy_from_slice(&a);
        bytes[4..6].copy_from_slice(&b);
        bytes
    }

    #[test]
    fn encoding_type_conversion_is_correct() {
        let mut value = 0u16;
        loop {
            let encoding_type = EncodingType::from(value);
            assert_eq!(value, u16::from(encoding_type));
            if value == u16::MAX {
                return;
            }
            value += 1;
        }
    }

    #[test]
    fn low_values_correspond_to_private_encoding_types() {
        for value in &[0x1, 0x82, 0xff] {
            let encoding_type = EncodingType::from(*value);
            match encoding_type {
                EncodingType::Private(x) if x as u16 == *value => (),
                _ => panic!(),
            };
        }
    }

    #[test]
    fn every_encoding_type_is_equal_to_itself() {
        let mut value = 0u16;
        loop {
            let encoding_type = EncodingType::from(value);
            assert_eq!(encoding_type, encoding_type);
            if value == u16::MAX {
                return;
            }
            value += 1;
        }
    }

    #[test]
    fn value_0x100u16_is_not_a_private_encoding_type() {
        let encoding_type = EncodingType::from(0x100);
        if let EncodingType::Private(_) = encoding_type {
            panic!();
        }
    }

    #[test]
    fn frameless_decoder_returns_error_when_frame_has_len_lt_6() {
        for len in 0..6 {
            let header = encode_header(len, 0x4324);
            let parser = BufferedCodec::new();
            let mut frames = parser.frames_streamiter(&header[..]);
            let frame = frames.next();
            match frame {
                Err(FramelessError::Decoder(DecodeError::InvalidMessageLength)) => (),
                _ => panic!(),
            }
        }
    }

    #[test]
    fn decoder_returns_error_when_frame_has_len_lt_6() {
        for len in 0..6 {
            let header = encode_header(len, 0x4324);
            let mut parser = Codec::default();
            let frame = parser.decode(&header[..]);
            match frame {
                Err(DecodeError::InvalidMessageLength) => (),
                _ => panic!(),
            }
        }
    }

    #[test]
    fn decoder_accepts_frame_with_len_6() {
        let header = encode_header(6, 0x4324);
        let mut parser = Codec::default();
        let frame = parser.decode(&header[..]);
        if frame.is_err() {
            panic!();
        }
    }
}
