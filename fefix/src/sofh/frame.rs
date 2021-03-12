use super::Error;
use std::convert::TryInto;
use std::io;

const HEADER_SIZE_IN_BYTES: usize = 6;

/// An immutable view into a SOFH-enclosed message, complete with its
/// encoding type tag and message.
///
/// This type is returned in a borrowed form from
/// [`sofh::Codec::decode`](sofh::Codec::decode) and
/// there is no owned version of this type.
#[derive(Debug, Copy, Clone)]
pub struct Frame<'a> {
    encoding_type: u16,
    message: &'a [u8],
}

impl<'a> Frame<'a> {
    /// Creates a new [`Frame`] with the given `encoding_type` and `message`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::sofh::Frame;
    ///
    /// let frame = Frame::new(0xF500, b"{}");
    /// assert_eq!(frame.message().len, 2);
    /// ```
    pub fn new(encoding_type: u16, message: &[u8]) -> Frame {
        Frame {
            encoding_type,
            message,
        }
    }

    /// Returns the 16-bits encoding type of `self`. You may want to
    /// convert this value to an [`EncodingType`], which allows
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

    /// Deserializes [`Self`] from `data`. Returns `None` if invalid. Zero-copy.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::sofh::Frame;
    ///
    /// // Message_Length ->            -----------
    /// // Encoding_Type ->                         ---------
    /// // Message ->                                         --
    /// let frame = Frame::from_bytes(&[0, 0, 0, 7, 0x0, 0x0, 42]).unwrap();
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
        let message_len = get_field_message_length(data) as usize;
        if message_len < HEADER_SIZE_IN_BYTES {
            // We have enough data to decode the header, but the Message_Length
            // field is invalid.
            Err(Error::InvalidMessageLength)
        } else if data.len() < message_len {
            // The header is fine, we just need to wait for the whole message.
            Err(Error::Incomplete {
                needed: message_len - data.len()
            })
        } else {
            Ok(Self::new(
                get_field_encoding_type(data),
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
    /// // Message_Length ->            -----------
    /// // Encoding_Type ->                         ---------
    /// // Message ->                                         --
    /// let bytes = &[0, 0, 0, 7, 0x0, 0x0, 42];
    /// let frame = Frame::from_bytes(bytes).unwrap();
    /// let buffer = &mut Vec::new();
    /// frame.encode(buffer).unwrap();
    /// assert_eq!(&buffer[..], bytes);
    /// ```
    pub fn encode<W>(&self, writer: &mut W) -> io::Result<usize>
    where
        W: io::Write,
    {
        let len = self.message().len();
        writer.write_all(&(len as u32).to_be_bytes())?;
        writer.write_all(&self.encoding_type().to_be_bytes())?;
        writer.write_all(self.message())?;
        Ok(HEADER_SIZE_IN_BYTES + len)
    }
}

fn get_field_message_length(data: &[u8]) -> u32 {
    u32::from_be_bytes(data[0..4].try_into().unwrap())
}

fn get_field_encoding_type(data: &[u8]) -> u16 {
    u16::from_be_bytes(data[4..6].try_into().unwrap())
}

#[cfg(test)]
mod test {
    //use super::*;

    //#[test]
    //fn decoder_returns_error_when_frame_has_len_lt_6() {
    //    for len in 0..6 {
    //        let header = encode_header(len, 0x4324);
    //        let frame = Frame::from_bytes(&header[..]);
    //        assert!(frame.is_none());
    //    }
    //}

    //#[test]
    //fn decoder_accepts_frame_with_len_6() {
    //    let header = encode_header(6, 0x4324);
    //    for len in 0..6 {
    //        let frame = Frame::from_bytes(&header[..]);
    //        assert!(frame.is_some());
    //    }
    //}
}
