use super::{Error, Header};
use std::io;

const MAX_MESSAGE_SIZE_IN_BYTES: usize = u32::MAX as usize - Header::LENGTH_IN_BYTES;

/// An immutable view into a SOFH-enclosed message, complete with its
/// encoding type tag and message payload.
///
/// # Type parameters
///
/// [`Frame`] is generic over `T`, which must implement `AsRef<[u8]>` and is the
/// *payload type*. Common choices for `T` are `&'a [u8]` for some lifetime `'a`,
/// [`bytes::Bytes`], and [`bytes::BytesMut`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Frame<T> {
    encoding_type: u16,
    payload: T,
}

impl<T> Frame<T>
where
    T: AsRef<[u8]>,
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
    /// use rustysofh::Frame;
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
    /// use rustysofh::{EncodingType, Frame};
    ///
    /// let frame = Frame::new(0xF500, &[] as &[u8]);
    /// let encoding_type = EncodingType::new(frame.encoding_type());
    /// assert_eq!(encoding_type, Some(EncodingType::Json));
    /// ```
    pub fn encoding_type(&self) -> u16 {
        self.encoding_type
    }

    /// Returns an immutable reference to the payload of `self`, i.e.
    /// without its header.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustysofh::Frame;
    ///
    /// let frame = Frame::new(0x0, &[42u8] as &[u8]);
    /// assert_eq!(frame.payload(), &[42]);
    /// ```
    pub fn payload(&self) -> &T {
        &self.payload
    }

    /// Returns a mutable reference to the payload of `self`.
    pub fn payload_mut(&mut self) -> &mut T {
        &mut self.payload
    }

    /// Serializes `self` to a `Writer`. This requires copying and thus is
    /// potentially expensive.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustysofh::Frame;
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
        let nominal_message_length_in_bytes =
            self.payload().as_ref().len() + Header::LENGTH_IN_BYTES;
        let header = Header {
            nominal_message_length_in_bytes,
            encoding_type: self.encoding_type,
        };
        writer.write_all(&header.to_bytes())?;
        writer.write_all(self.payload().as_ref())?;
        Ok(nominal_message_length_in_bytes)
    }
}

impl<'a> Frame<&'a [u8]> {
    /// Tries to deserialize a [`Frame`] from `data`. Returns an `Err` if
    /// invalid. Zero-copy.
    ///
    /// This function ignores trailing bytes that are not part of the message.
    ///
    /// # Examples
    ///
    /// ```
    /// use rustysofh::Frame;
    ///
    /// // Message_Length:                        ~~~~~~~~~~~
    /// // Encoding_Type:                                     ~~~~~~~~~
    /// // Message:                                                     ~~
    /// let frame = Frame::<&[u8]>::deserialize(&[0, 0, 0, 7, 0x0, 0x0, 42]).unwrap();
    /// assert_eq!(frame.payload(), &[42]);
    /// ```
    pub fn deserialize(data: &'a [u8]) -> Result<Self, Error> {
        let header = Header::from_bytes(data)?;
        Ok(Frame::new(
            header.encoding_type,
            &data[Header::LENGTH_IN_BYTES..header.nominal_message_length_in_bytes],
        ))
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
        assert_eq!(frame.payload(), &[]);
        let frame = Frame::new(0x1122, b"foobar" as &[u8]);
        assert_eq!(frame.encoding_type(), 0x1122);
        assert_eq!(frame.payload(), &&b"foobar"[..]);
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
        assert_eq!(frame.payload(), &[]);
    }

    #[test]
    fn encode_then_decode_should_have_no_effect() {
        fn prop(encoding_type: u16, payload: Vec<u8>) -> bool {
            let frame = Frame::<&[u8]>::new(encoding_type, &payload[..]);
            let mut buffer = vec![];
            frame.serialize(&mut buffer).unwrap();
            let frame_decoded = Frame::<&[u8]>::deserialize(&buffer[..]).unwrap();
            frame_decoded.encoding_type() == encoding_type
                && frame_decoded.payload() == &&payload[..]
        }
        QuickCheck::new().quickcheck(prop as fn(u16, Vec<u8>) -> bool)
    }
}
