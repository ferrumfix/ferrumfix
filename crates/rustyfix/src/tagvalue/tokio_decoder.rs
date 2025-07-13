use super::{Config, DecodeError, Decoder, Message, RawDecoder, RawFrame};
use crate::GetConfig;
use bytes::{Bytes, BytesMut};
use tokio_util::codec;

/// A [`tokio_util::codec::Decoder`] for raw FIX frames.
#[derive(Debug)]
pub struct TokioRawDecoder {
    raw_decoder: RawDecoder,
}

impl codec::Decoder for TokioRawDecoder {
    type Item = RawFrame<Bytes>;
    type Error = DecodeError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let split = src.split();
        let result = self.raw_decoder.decode(split);
        match result {
            Ok(raw_frame) => Ok(Some(RawFrame {
                data: raw_frame.data.freeze(),
                begin_string: raw_frame.begin_string,
                payload: raw_frame.payload,
            })),
            Err(DecodeError::Invalid) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

impl GetConfig for TokioRawDecoder {
    type Config = Config;

    fn config(&self) -> &Self::Config {
        self.raw_decoder.config()
    }

    fn config_mut(&mut self) -> &mut Self::Config {
        self.raw_decoder.config_mut()
    }
}

/// A [`tokio_util::codec::Decoder`] for FIX messages.
#[derive(Debug)]
#[cfg_attr(docsrs, doc(cfg(feature = "utils-tokio")))]
pub struct TokioDecoder {
    raw_decoder: Decoder,
}

impl codec::Decoder for TokioDecoder {
    type Item = Message<'static, Bytes>;
    type Error = DecodeError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let split = src.split();
        let result = self.raw_decoder.decode(split);
        match result {
            Ok(_raw_frame) => {
                // This operation is currently unsupported because the `Message` type does
                // not own its underlying buffer, which is a requirement for Tokio
                // codecs.
                Err(DecodeError::Unsupported(
                    "The Message type does not support owning its buffer, which is required for a Tokio codec. This needs a redesign of the Message struct.".to_string()
                ))
            }
            Err(DecodeError::Invalid) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
