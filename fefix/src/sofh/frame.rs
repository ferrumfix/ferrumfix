use super::{field_encoding_type, field_message_length, Error};
use crate::buffer::MemorySlice;
use std::io;

const HEADER_SIZE_IN_BYTES: usize = 6;
const MAX_MESSAGE_SIZE_IN_BYTES: usize = u32::MAX as usize - HEADER_SIZE_IN_BYTES;

/// An immutable view into a SOFH-enclosed message, complete with its
/// encoding type tag and message.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Frame<T>
where
    T: MemorySlice,
{
    encoding_type: u16,
    payload: T,
}

impl<T> Frame<T>
where
    T: MemorySlice,
{
    /// Creates a new [`Frame`] with the given `encoding_type` and `payload`.
    ///
    /// # Panics
    ///
    /// This function panics if `message.len() > u32::MAX as usize - 6`, which
    /// would cause encoding issues.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::sofh::Frame;
    ///
    /// let frame = Frame::new(0xF500, b"{}" as &[u8]);
    /// assert_eq!(frame.payload().len(), 2);
    /// ```
    pub fn new(encoding_type: u16, payload: T) -> Self {
        assert!(payload.as_ref().len() <= MAX_MESSAGE_SIZE_IN_BYTES);
        Frame {
            encoding_type,
            payload,
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
    /// let frame = Frame::new(0xF500, &[] as &[u8]);
    /// let encoding_type = EncodingType::from(frame.encoding_type());
    /// assert_eq!(encoding_type, EncodingType::Json);
    /// ```
    pub fn encoding_type(&self) -> u16 {
        self.encoding_type
    }

    /// Returns an immutable reference to the payload bytes of `self`, i.e.
    /// without its header.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::sofh::Frame;
    ///
    /// let frame = Frame::new(0x0, &[42u8] as &[u8]);
    /// assert_eq!(frame.payload(), &[42]);
    /// ```
    pub fn payload(&self) -> &[u8] {
        self.payload.as_ref()
    }

    /// Deserializes a [`Frame<&[u8]>`] from `data`. Returns an `Err` if
    /// invalid. Zero-copy.
    ///
    /// This function ignores trailing bytes that are not part of the message.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::sofh::Frame;
    ///
    /// // Message_Length:                        ~~~~~~~~~~~
    /// // Encoding_Type:                                     ~~~~~~~~~
    /// // Message:                                                     ~~
    /// let frame = Frame::<&[u8]>::deserialize(&[0, 0, 0, 7, 0x0, 0x0, 42]).unwrap();
    /// assert_eq!(frame.payload(), &[42]);
    /// ```
    pub fn deserialize(data: &[u8]) -> Result<Frame<&[u8]>, Error> {
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
            Ok(Frame::new(
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
    /// //            ~~~~~~~~~~~~~
    /// // Encoding_Type:           ~~~~~~~~~
    /// // Message:                           ~~
    /// let bytes = &[0u8, 0, 0, 7, 0x0, 0x0, 42] as &[u8];
    /// let frame = Frame::<&[u8]>::deserialize(bytes).unwrap();
    /// let buffer = &mut Vec::new();
    /// frame.serialize(buffer).unwrap();
    /// assert_eq!(&buffer[..], bytes);
    /// ```
    pub fn serialize<W>(&self, writer: &mut W) -> io::Result<usize>
    where
        W: io::Write,
    {
        let len = self.payload().len() + HEADER_SIZE_IN_BYTES;
        writer.write_all(&(len as u32).to_be_bytes())?;
        writer.write_all(&self.encoding_type().to_be_bytes())?;
        writer.write_all(self.payload())?;
        Ok(len)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::QuickCheck;

    #[test]
    fn information_retrieval_is_consistent_with_new() {
        let frame = Frame::new(0x0, b"" as &[u8]);
        assert_eq!(frame.encoding_type(), 0x0);
        assert_eq!(frame.payload(), [] as [u8; 0]);
        let frame = Frame::new(0x1122, b"foobar" as &[u8]);
        assert_eq!(frame.encoding_type(), 0x1122);
        assert_eq!(frame.payload(), b"foobar" as &[u8]);
        let frame = Frame::new(u16::MAX, &[0u8] as &[u8]);
        assert_eq!(frame.encoding_type(), u16::MAX);
        assert_eq!(frame.payload(), &[0]);
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
            Frame::<&[u8]>::deserialize(&[]),
            Err(Error::Incomplete { needed: 6 })
        ));
        assert!(matches!(
            Frame::<&[u8]>::deserialize(&[0, 0, 0]),
            Err(Error::Incomplete { needed: 3 })
        ));
        assert!(matches!(
            Frame::<&[u8]>::deserialize(&[0, 0, 0, 0, 0]),
            Err(Error::Incomplete { needed: 1 })
        ));
    }

    #[test]
    fn decode_empty_message() {
        let frame = Frame::<&[u8]>::deserialize(&[0, 0, 0, 6, 0, 0]).unwrap();
        assert_eq!(frame.encoding_type(), 0);
        assert_eq!(frame.payload(), [] as [u8; 0]);
    }

    #[test]
    fn encode_then_decode_should_have_no_effect() {
        fn prop(encoding_type: u16, data: Vec<u8>) -> bool {
            let frame = Frame::<&[u8]>::new(encoding_type, &data[..]);
            let buffer = &mut vec![];
            frame.serialize(buffer).unwrap();
            let frame_decoded = Frame::<&[u8]>::deserialize(&buffer[..]).unwrap();
            frame_decoded.encoding_type() == encoding_type && frame_decoded.payload() == &data[..]
        }
        QuickCheck::new()
            .tests(1000)
            .quickcheck(prop as fn(u16, Vec<u8>) -> bool)
    }
}
