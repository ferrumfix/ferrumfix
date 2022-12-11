use super::{Error, Frame, Header};
use bytes::{Bytes, BytesMut};
use std::io;
use tokio_util::codec;

/// A [`tokio_util`] [`Decoder`](tokio_util::codec::Decoder) and
/// [`Encoder`](tokio_util::codec::Encoder).
///
/// # Examples
///
/// ```
/// use bytes::{Bytes, BytesMut};
/// use tokio_util::codec::{Decoder, Encoder};
///
/// let payload = Bytes::from_static(b"payload");
/// let mut codec = fesofh::TokioCodec::default();
/// let mut destination = BytesMut::new();
/// codec.encode(fesofh::Frame::new(0x1337, payload.clone()), &mut destination);
///
/// assert_eq!(
///     codec.decode(&mut destination).unwrap(),
///     Some(fesofh::Frame::new(0x1337, payload))
/// );
/// ```
#[derive(Debug, Default)]
#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-tokio")))]
pub struct TokioCodec {}

impl TokioCodec {
    /// Creates a new [`TokioCodec`].
    pub fn new() -> Self {
        Self::default()
    }
}

impl codec::Decoder for TokioCodec {
    type Item = Frame<Bytes>;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match Header::from_bytes(src) {
            Ok(header) => {
                let len = header.nominal_message_length_in_bytes;
                if src.len() >= len {
                    let mut frame = src.split_to(len);
                    let payload = frame.split_off(Header::LENGTH_IN_BYTES).freeze();
                    Ok(Some(Frame::new(header.encoding_type, payload)))
                } else {
                    src.reserve(len - src.len());
                    Ok(None)
                }
            }
            Err(Error::InvalidMessageLength) => Err(Error::InvalidMessageLength),
            Err(Error::Incomplete { needed: _ }) => Ok(None),
            Err(Error::Io(_)) => panic!("Unexpected I/O error."),
        }
    }
}

impl<T> codec::Encoder<Frame<T>> for TokioCodec
where
    T: AsRef<[u8]>,
{
    type Error = io::Error;

    fn encode(&mut self, frame: Frame<T>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let nominal_message_length_in_bytes =
            frame.payload().as_ref().len() + Header::LENGTH_IN_BYTES;
        let header = Header {
            nominal_message_length_in_bytes,
            encoding_type: frame.encoding_type(),
        };
        dst.reserve(nominal_message_length_in_bytes);
        dst.extend_from_slice(&header.to_bytes());
        dst.extend_from_slice(frame.payload().as_ref());
        Ok(())
    }
}
