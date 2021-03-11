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

mod encoding_type;
mod errors;

pub use encoding_type::EncodingType;
pub use errors::{DecodeError, EncodeError};

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
    /// convert this value to an [`EncodingType`], which allows
    /// for nice pattern matching.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::Encoding;
    /// use fefix::sofh::{Codec, EncodingType};
    ///
    /// let codec = &mut Codec::default();
    /// // Message_Length ->       -----------
    /// // Encoding_Type ->                    ----------
    /// let frame = codec.decode(&[0, 0, 0, 6, 0xF5, 0x00]).unwrap();
    /// let encoding_type = EncodingType::from(frame.encoding_type());
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
    /// use fefix::Encoding;
    /// use fefix::sofh::Codec;
    ///
    /// let codec = &mut Codec::default();
    /// // Message_Length ->       -----------
    /// // Encoding_Type ->                    ---------
    /// // Message ->                                    --
    /// let frame = codec.decode(&[0, 0, 0, 7, 0x0, 0x0, 42]).unwrap();
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
    pub fn buffered(self) -> CodecBuffered {
        CodecBuffered {
            buffer: Vec::new(),
            buffer_relevant_len: 0,
            buffer_additional_len: 0,
            codec: self,
        }
    }

    /// Turns `self` into a [`StreamingDecoder`](StreamingDecoder) -enabled codec
    /// by allocating an internal buffer. The allocated buffer will have an
    /// initial capacity of `capacity` in bytes.
    pub fn buffered_with_capacity(self, capacity: usize) -> CodecBuffered {
        CodecBuffered {
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
            buffer: (&data[HEADER_SIZE_IN_BYTES..]).as_ptr(),
            buffer_len: data.len() - HEADER_SIZE_IN_BYTES,
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
pub struct CodecBuffered {
    buffer: Vec<u8>,
    buffer_relevant_len: usize,
    buffer_additional_len: usize,
    codec: Codec,
}

impl CodecBuffered {
    /// Creates a new SOFH parser with default buffer size.
    pub fn new() -> Self {
        Self::with_capacity(1024)
    }

    /// Creates a new [`Codec`] with a buffer large enough to
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
    /// use fefix::sofh::CodecBuffered;
    ///
    /// let parser = CodecBuffered::with_capacity(8192);
    /// assert_eq!(parser.capacity(), 8192);
    /// ```
    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }
}

impl Encoding<Frame> for CodecBuffered {
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

impl StreamingDecoder<Frame> for CodecBuffered {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::FramelessError;
    use crate::StreamingDecoder;

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
    fn frameless_decoder_returns_error_when_frame_has_len_lt_6() {
        for len in 0..6 {
            let header = encode_header(len, 0x4324);
            let parser = CodecBuffered::new();
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
