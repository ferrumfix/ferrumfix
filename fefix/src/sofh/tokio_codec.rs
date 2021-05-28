use super::{Error, Frame};
use crate::MemorySlice;
use bytes::{Bytes, BytesMut};
use std::io;
use tokio_util::codec;

/// A dummy decoder and encoder for SOFH-enclosed messages that have
/// [`bytes::Bytes`] as payload. It supports [`tokio_util::codec`] utilities.
///
/// # Examples
///
/// ```
/// use bytes::{Bytes, BytesMut};
/// use fefix::sofh;
/// use tokio_util::codec::{Decoder, Encoder};
///
/// let payload = Bytes::from_static(b"payload");
/// let codec = &mut sofh::TokioCodec::default();
/// let destination = &mut BytesMut::new();
/// codec.encode(sofh::Frame::new(0x1337, payload.clone()), destination);
///
/// assert_eq!(
///     codec.decode(destination).unwrap(),
///     Some(sofh::Frame::new(0x1337, payload))
/// );
/// ```
#[derive(Debug)]
pub struct TokioCodec {}

impl Default for TokioCodec {
    fn default() -> Self {
        Self {}
    }
}

impl codec::Decoder for TokioCodec {
    type Item = Frame<Bytes>;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        use std::convert::TryInto;
        if src.len() >= 6 {
            let bytes_04 = (&src[0..4]).try_into().unwrap();
            let nominal_len = u32::from_be_bytes(bytes_04) as usize;
            if src.len() >= nominal_len {
                let mut frame = src.split_to(nominal_len);
                let bytes_46 = (&frame[4..6]).try_into().unwrap();
                let encoding_type = u16::from_be_bytes(bytes_46);
                let payload = frame.split_off(6).freeze();
                Ok(Some(Frame::new(encoding_type, payload)))
            } else {
                src.reserve(nominal_len - src.len());
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

impl<T> codec::Encoder<Frame<T>> for TokioCodec
where
    T: MemorySlice,
{
    type Error = io::Error;

    fn encode(&mut self, frame: Frame<T>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let frame_len = frame.payload().len() + 6;
        dst.reserve(frame_len);
        dst.extend_from_slice(&u32::to_be_bytes(frame_len as u32)[..]);
        dst.extend_from_slice(&u16::to_be_bytes(frame.encoding_type())[..]);
        dst.extend_from_slice(frame.payload().as_ref());
        Ok(())
    }
}
