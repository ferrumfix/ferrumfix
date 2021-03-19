use super::Error;
use std::convert::TryInto;
use std::io;

const HEADER_SIZE_IN_BYTES: usize = 6;
const MAX_MESSAGE_SIZE_IN_BYTES: usize = u32::MAX as usize - HEADER_SIZE_IN_BYTES;

/// An immutable view into a SOFH-enclosed message, complete with its
/// encoding type tag and message.
///
/// This type is returned in a borrowed form during decoding and
/// there is no owned version of this type.
#[derive(Debug, Copy, Clone)]
pub struct Frame<'a> {
    encoding_type: u16,
    message: &'a [u8],
}

impl<'a> Frame<'a> {
    /// Creates a new [`Frame`] with the given `encoding_type` and `message`.
    ///
    /// # Panics
    ///
    /// This function panics if `message.len() > u32::MAX as usize - 6`. Permitting larger
    /// messages would cause encoding issues.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::sofh::Frame;
    ///
    /// let frame = Frame::new(0xF500, b"{}");
    /// assert_eq!(frame.message().len(), 2);
    /// ```
    pub fn new(encoding_type: u16, message: &[u8]) -> Frame {
        assert!(message.len() <= MAX_MESSAGE_SIZE_IN_BYTES);
        Frame {
            encoding_type,
            message,
        }
    }

    /// Returns the 16-bits encoding type of `self`. You may want to
    /// convert this value to an [`EncodingType`](super::EncodingType), which allows
    /// for nice pattern matching.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::sofh::{EncodingType, Frame};
    ///
    /// let frame = Frame::new(0xF500, &[]);
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
    /// use fefix::sofh::Frame;
    ///
    /// let frame = Frame::new(0x0, &[42]);
    /// assert_eq!(frame.message(), &[42]);
    /// ```
    pub fn message(&self) -> &[u8] {
        self.message
    }

    /// Deserializes a [`Frame`] from `data`. Returns an `Err` if invalid. Zero-copy.
    ///
    /// This function ignores trailing bytes that are not part of the message.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::sofh::Frame;
    ///
    /// // Message_Length:          -----------
    /// // Encoding_Type:                       ---------
    /// // Message:                                       --
    /// let frame = Frame::decode(&[0, 0, 0, 7, 0x0, 0x0, 42]).unwrap();
    /// assert_eq!(frame.message(), &[42]);
    /// ```
    pub fn decode(data: &[u8]) -> Result<Frame, Error> {
        // The buffer doesn't contain enough data to even meaningfully reason
        // about it, let alone decode it.
        if data.len() < HEADER_SIZE_IN_BYTES {
            return Err(Error::Incomplete {
                needed: HEADER_SIZE_IN_BYTES - data.len(),
            });
        }
        let message_len = field_message_length(data) as usize;
        if message_len < HEADER_SIZE_IN_BYTES {
            // We have enough data to decode the header, but the Message_Length
            // field is invalid.
            Err(Error::InvalidMessageLength)
        } else if data.len() < message_len {
            // The header is fine, we just need to wait for the whole message.
            Err(Error::Incomplete {
                needed: message_len - data.len(),
            })
        } else {
            Ok(Self::new(
                field_encoding_type(data),
                &data[HEADER_SIZE_IN_BYTES..],
            ))
        }
    }

    /// Serializes `self` to a `Writer`. This requires copying and thus is
    /// potentially expensive.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::sofh::Frame;
    ///
    /// // Message_Length:
    /// //            -----------
    /// // Encoding_Type:         ---------
    /// // Message:                         --
    /// let bytes = &[0, 0, 0, 7, 0x0, 0x0, 42];
    /// let frame = Frame::decode(bytes).unwrap();
    /// let buffer = &mut Vec::new();
    /// frame.encode(buffer).unwrap();
    /// assert_eq!(&buffer[..], bytes);
    /// ```
    pub fn encode<W>(&self, writer: &mut W) -> io::Result<usize>
    where
        W: io::Write,
    {
        let len = self.message().len();
        writer.write_all(&((len + HEADER_SIZE_IN_BYTES) as u32).to_be_bytes())?;
        writer.write_all(&self.encoding_type().to_be_bytes())?;
        writer.write_all(self.message())?;
        Ok(HEADER_SIZE_IN_BYTES + len)
    }
}

fn field_message_length(data: &[u8]) -> u32 {
    u32::from_be_bytes(data[0..4].try_into().unwrap())
}

fn field_encoding_type(data: &[u8]) -> u16 {
    u16::from_be_bytes(data[4..6].try_into().unwrap())
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::QuickCheck;

    #[test]
    fn information_retrieval_is_consistent_with_new() {
        let frame = Frame::new(0x0, &[]);
        assert_eq!(frame.encoding_type(), 0x0);
        assert_eq!(frame.message(), [] as [u8; 0]);
        let frame = Frame::new(0x1122, b"foobar");
        assert_eq!(frame.encoding_type(), 0x1122);
        assert_eq!(frame.message(), b"foobar");
        let frame = Frame::new(u16::MAX, &[0]);
        assert_eq!(frame.encoding_type(), u16::MAX);
        assert_eq!(frame.message(), &[0]);
    }

    #[test]
    #[should_panic]
    fn new_with_size_u32_max() {
        let data = vec![0; u32::MAX as usize];
        Frame::new(0x0, &data[..]);
    }

    #[test]
    #[should_panic]
    fn new_with_size_u32_max_minus_5() {
        let data = vec![0; u32::MAX as usize - 5];
        Frame::new(0x0, &data[..]);
    }

    #[test]
    fn new_with_size_u32_max_minus_6() {
        let data = vec![0; u32::MAX as usize - 6];
        Frame::new(0x0, &data[..]);
    }

    #[test]
    fn new_with_size_u32_max_minus_7() {
        let data = vec![0; u32::MAX as usize - 7];
        Frame::new(0x0, &data[..]);
    }

    #[test]
    fn decode_incomplete_header() {
        assert!(matches!(
            Frame::decode(&[]),
            Err(Error::Incomplete { needed: 6 })
        ));
        assert!(matches!(
            Frame::decode(&[0, 0, 0]),
            Err(Error::Incomplete { needed: 3 })
        ));
        assert!(matches!(
            Frame::decode(&[0, 0, 0, 0, 0]),
            Err(Error::Incomplete { needed: 1 })
        ));
    }

    #[test]
    fn decode_empty_message() {
        let frame = Frame::decode(&[0, 0, 0, 6, 0, 0]).unwrap();
        assert_eq!(frame.encoding_type(), 0);
        assert_eq!(frame.message(), [] as [u8; 0]);
    }

    #[test]
    fn encode_then_decode_should_have_no_effect() {
        fn prop(encoding_type: u16, data: Vec<u8>) -> bool {
            let frame = Frame::new(encoding_type, &data[..]);
            let buffer = &mut vec![];
            frame.encode(buffer).unwrap();
            let frame_decoded = Frame::decode(&buffer[..]).unwrap();
            frame_decoded.encoding_type() == encoding_type && frame_decoded.message() == &data[..]
        }
        QuickCheck::new()
            .tests(1000)
            .quickcheck(prop as fn(u16, Vec<u8>) -> bool)
    }
}
