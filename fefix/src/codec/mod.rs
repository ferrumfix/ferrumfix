//! Support for FIX-related encoding types (OSI Layer 6).
//!
//! Encoders need to implement [`Encoder`], while decoders can choose to
//! implement [`Decoder`] or [`StreamingDecoder`] (or both). Implementors of
//! [`Decoder`] decode messages with a pre-defined length. Implementors of
//! [`StreamingDecoder`] decode *streams* of messages, without delimiters.
//!
//! - FIX tag-value: [`tagvalue::Codec`].
//! - FAST: [`fast::Fast`].
//! - JSON: [`json::Codec`].
//! - SOFH: [`sofh::Codec`].
//!
//! Most encoding types support configuration options via the *transmuter
//! pattern*. Transmuters are traits that define all configurable options for a
//! specific encoding.
use crate::buffering::*;
use std::io;
use std::marker::PhantomData;

pub mod fast;
pub mod json;
pub mod sofh;
pub mod tagvalue;

/// A device that can parse a stream of bytes into messages.
///
/// A [`StreamingDecoder`]
pub trait StreamingDecoder<M>
where
    Self: Sized,
{
    /// The type returned in the event of a deserialization error.
    type Error;

    /// Returns a mutable slice of bytes to accomodate for new input.
    ///
    /// The slice
    /// can have any non-zero length, depending on how many bytes `self` believes
    /// is a good guess. All bytes should be set to 0.
    fn supply_buffer(&mut self) -> &mut [u8];

    /// Validates the contents of the internal buffer and possibly caches the
    /// resulting message. When successful, this method will return a [`Poll`] to
    /// let the caller know whether more bytes are needed or not.
    fn attempt_decoding(&mut self) -> Result<Option<&M>, Self::Error>;

    fn get(&self) -> &M {
        unimplemented!()
    }

    fn frames_streamiter<R>(self, reader: R) -> Frames<Self, R, M, Self::Error>
    where
        R: io::Read,
    {
        Frames {
            decoder: self,
            source: reader,
            message: PhantomData::default(),
            err: None,
        }
    }
}

/// A device that can parse arbitrary sequences of bytes into messages.
pub trait Decoder<M> {
    /// The type returned in the event of a deserialization error.
    type Error;

    /// Parses the contents of `data` into a message `M`, which is then returned
    /// in the form of an immutable reference. In the event of failure, this
    /// method will return `Self::Error`.
    fn decode(&mut self, data: &[u8]) -> Result<&M, Self::Error>;
}

/// A device that writes messages to a [`Buffer`].
pub trait Encoder<M> {
    /// The type returned in the event of a serialization error.
    type Error;

    /// Encodes `message` to `buffer`. It then returns the number of bytes
    /// written to `buffer`.
    fn encode(&mut self, buffer: impl Buffer, message: &M) -> Result<usize, Self::Error>;

    /// Allocates a buffer on the heap and encodes `message` to it.
    fn encode_to_vec(&mut self, message: &M) -> Result<Vec<u8>, Self::Error> {
        let mut buffer = Vec::<u8>::new();
        self.encode(&mut buffer, message)?;
        Ok(buffer.as_slice().iter().cloned().collect())
    }
}

/// A [`StreamIterator`] that iterates over all the messages that come from a
/// [reader](std::io::Read).
///
/// This `struct` is created by the [`frames`](StreamingDecoder::frames) method
/// on [`StreamingDecoder`].
/// See its documentation for more.
#[derive(Debug)]
pub struct Frames<D, R, M, E> {
    decoder: D,
    source: R,
    message: PhantomData<M>,
    err: Option<FramelessError<E>>,
}

impl<D, R, M, E> Frames<D, R, M, E>
where
    M: Sized,
    D: StreamingDecoder<M, Error = E>,
    R: io::Read,
{
    pub fn next(&mut self) -> Result<Option<&M>, &FramelessError<E>> {
        loop {
            let mut buffer = self.decoder.supply_buffer();
            if let Err(e) = self.source.read(&mut buffer) {
                self.err = Some(e.into());
                break;
            }
            match self.decoder.attempt_decoding() {
                Ok(Some(_)) => break,
                Ok(None) => (),
                Err(e) => {
                    self.err = Some(FramelessError::Decoder(e));
                    break;
                }
            }
        }
        match self.err {
            Some(ref err) => Err(err),
            None => Ok(Some(self.decoder.get())),
        }
    }
}

#[derive(Debug)]
pub enum FramelessError<E> {
    Decoder(E),
    Io(io::Error),
}

impl<T> From<io::Error> for FramelessError<T> {
    fn from(err: io::Error) -> Self {
        FramelessError::Io(err)
    }
}

/// Represents the progress that a codec device has made in regard to the current
/// message.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Poll {
    /// The message has been fully decoded and is available in the internal buffer.
    Ready,
    /// The message is incomplete and more bytes are needed for completing
    /// decoding.
    Incomplete,
}
