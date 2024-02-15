use crate::{Buffer, StreamingDecoder};
use bytes::BytesMut;
use tokio_util::codec;

#[derive(Debug)]
pub struct TokioDecoderAdapter<D> {
    decoder: D,
}

impl<D> codec::Decoder for TokioDecoderAdapter<D>
where
    D: StreamingDecoder,
    D::Error: From<std::io::Error>,
{
    type Item = D::Item;
    type Error = D::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        self.decoder.clear();
        // FIXME: this is quite inefficient because it copies the entire buffer.
        self.decoder.buffer_mut().extend_from_slice(&src);
        self.decoder.try_parse().map_err(Into::into)
    }
}
