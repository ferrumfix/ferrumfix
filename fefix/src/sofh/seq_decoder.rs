use super::err::Error;
use super::frame::Frame;
use std::io;

/// A parser for SOFH-enclosed messages.
///
/// SOFH stands for Simple Open Framing Header and it's an encoding-agnostic
/// framing mechanism for variable-length messages. It was developed by the FIX
/// High Performance Group to allow message processors and communication gateways
/// to determine the length and the data format of incoming messages.
#[derive(Debug)]
pub struct SeqDecoder {
    buffer: Vec<u8>,
    buffer_actual_len: usize,
}

impl Default for SeqDecoder {
    fn default() -> Self {
        Self::with_capacity(128)
    }
}

impl SeqDecoder {
    /// Creates a new [`SeqDecoder`] with a buffer large enough to
    /// hold `capacity` amounts of bytes without reallocating.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            buffer_actual_len: 0,
        }
    }

    /// Returns the current buffer capacity of this [`SeqDecoder`]. This value is
    /// subject to change after every incoming message.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::sofh::SeqDecoder;
    ///
    /// let parser = SeqDecoder::with_capacity(8192);
    /// assert_eq!(parser.capacity(), 8192);
    /// ```
    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    /// Provides a buffer that MUST be filled before re-attempting to deserialize
    /// the next [`Frame`].
    pub fn supply_buffer(&mut self) -> &mut [u8] {
        let decode_result = Frame::<&[u8]>::deserialize(self.buffer.as_slice());
        match decode_result {
            Ok(_) => &mut [],
            Err(Error::Io(_)) => panic!("Impossible IO error"),
            Err(Error::Incomplete { needed }) => {
                self.buffer.resize(self.buffer.as_slice().len() + needed, 0);
                &mut self.buffer.as_mut_slice()[self.buffer_actual_len..]
            }
            Err(Error::InvalidMessageLength) => panic!("Invalid stream"),
        }
    }

    /// Attempts decoding. Returns `Ok(())` if a [`Frame`] is ready, otherwise an `Err`.
    pub fn attempt_decoding(&mut self) -> Result<(), Error> {
        let slice = &self.buffer.as_slice()[..self.buffer_actual_len];
        let decode_result = Frame::<&[u8]>::deserialize(slice);
        decode_result.map(|_| ())
    }

    /// Returns the current [`Frame`]
    pub fn current_frame(&self) -> Frame<&[u8]> {
        let slice = &self.buffer.as_slice()[..self.buffer_actual_len];
        let decode_result = Frame::<&[u8]>::deserialize(slice);
        decode_result.unwrap()
    }

    pub fn read_frames<R>(self, reader: R) -> Frames<R>
    where
        R: io::Read,
    {
        Frames {
            decoder: self,
            reader,
        }
    }
}

#[derive(Debug)]
pub struct Frames<R> {
    decoder: SeqDecoder,
    reader: R,
}

impl<R> Frames<R>
where
    R: std::io::Read,
{
    pub fn next(&mut self) -> Result<Option<Frame<&[u8]>>, Error> {
        loop {
            let buffer = &mut self.decoder.supply_buffer();
            match self.reader.read(buffer) {
                Err(e) => {
                    return Err(Error::Io(e));
                }
                Ok(_) => {}
            }
            match self.decoder.attempt_decoding() {
                Ok(()) => {
                    return Ok(Some(self.decoder.current_frame()));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    //use super::*;

    //fn _frames_with_increasing_length() -> impl Iterator<Item = Vec<u8>> {
    //    std::iter::once(()).enumerate().map(|(i, ())| {
    //        let header = encode_header(i as u32 + 6, 0);
    //        let mut buffer = Vec::new();
    //        buffer.extend_from_slice(&header[..]);
    //        for _ in 0..i {
    //            buffer.extend_from_slice(&[0]);
    //        }
    //        buffer
    //    })
    //}

    //struct Reader<T> {
    //    source: T,
    //}

    //impl<T> std::io::Read for Reader<T>
    //where
    //    T: Iterator<Item = u8>,
    //{
    //    fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
    //        for i in 0..buffer.len() {
    //            buffer[i] = self.source.next().unwrap();
    //        }
    //        Ok(buffer.len())
    //    }
    //}

    //fn _increasing_frames_as_read() -> impl std::io::Read {
    //    let stream = _frames_with_increasing_length()
    //        .map(|vec| vec.into_iter())
    //        .flatten();
    //    Reader { source: stream }
    //}

    //#[test]
    //fn frameless_decoder_returns_error_when_frame_has_len_lt_6() {
    //    for len in 0..6 {
    //        let header = encode_header(len, 0x4324);
    //        let parser = Decoder::new();
    //        let mut frames = parser.read_frames(&header[..]);
    //        let frame = frames.next();
    //        match frame {
    //            Err(DecodeError::InvalidMessageLength) => (),
    //            _ => panic!(),
    //        }
    //    }
    //}
}
