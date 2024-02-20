use crate::StreamingDecoder;
use bytes::BytesMut;
use tokio_util::codec;

#[derive(Debug)]
pub struct TokioDecoderAdapter<D> {
    decoder: D,
    num_bytes_read: usize,
}

impl<D> TokioDecoderAdapter<D> {
    pub fn new(decoder: D) -> Self {
        Self {
            decoder,
            num_bytes_read: 0,
        }
    }
}

impl<D> codec::Decoder for TokioDecoderAdapter<D>
where
    D: StreamingDecoder<Buffer = BytesMut>,
    D::Error: From<std::io::Error>,
{
    type Item = D::Item;
    type Error = D::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() >= self.num_bytes_read + self.decoder.num_bytes_required() {
            let bytes_needed =
                src.split_to(self.num_bytes_read + self.decoder.num_bytes_required());
            self.decoder.buffer_mut().unsplit(bytes_needed);
            self.num_bytes_read = src.len();
        }
        self.decoder.try_parse().map_err(Into::into)
    }
}
