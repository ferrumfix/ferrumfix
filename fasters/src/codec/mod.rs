//! Support for FIX-related encoding types (OSI Layer 6).
//!
//! Encoders need to implement [`Encoder`], while decoders can choose to
//! implement [`Decoder`] or [`FramelessDecoder`] (or both). Implementors of
//! [`Decoder`] decode messages with a pre-defined length. Implementors of
//! [`FramelessDecoder`] decode *streams* of messages, without delimiters.
//!
//! - FIX tag-value: [`tagvalue::Codec`].
//! - FAST: [`fast::Fast`].
//! - JSON: [`json::Codec`].
//! - SOFH: [`sofh::Codec`].
//!
//! Most encoding types support configuration options via the *transmuter
//! pattern*. Transmuters are traits that define all configurable options for a
//! specific encoding.
use crate::utils::*;
use crate::StreamIterator;
use std::io;
use std::marker::PhantomData;

pub mod fast;
pub mod json;
pub mod sofh;
pub mod tagvalue;

pub trait FramelessDecoder<'s, M> where Self: Sized {
    type Error: 's;

    /// Returns a mutable slice of bytes to accomodate for new input.
    ///
    /// The slice
    /// can have any non-zero length, depending on how many bytes `self` believes
    /// is a good guess. All bytes should be set to 0.
    fn supply_buffer(&mut self) -> &mut [u8];

    /// Validates the contents of the internal buffer and possibly caches the
    /// resulting message. When successful, this method will return a [`Poll`] to
    /// let the caller know whether more bytes are needed or not.
    fn attempt_decoding(&mut self) -> Result<Poll, Self::Error>;

    fn decode_next_item(&'s mut self) -> Result<Option<M>, Self::Error> {
        self.attempt_decoding().map(move |t| match t {
            Poll::Ready => Some(self.get_item()),
            Poll::Incomplete => None,
        })
    }

    /// Returns the last message.
    fn get_item(&'s self) -> M;

    /// Returns a [`StreamIterator`] over the message frames
    /// produced by `source`.
    fn frames_streamiter<R>(self, reader: R) -> Frames<Self, R, M, Self::Error>
    where
        R: io::Read,
    {
        Frames {
            codec: self,
            source: reader,
            phantom: PhantomData::default(),
            err: None,
        }
    }
}

pub trait Decoder<'a, M> {
    type Error;

    fn decode(&'a mut self, data: &'a [u8]) -> Result<M, Self::Error>;
}

pub trait Encoder<M> {
    type Error;

    fn encode(&mut self, buffer: impl Buffer, message: &M) -> Result<usize, Self::Error>;

    fn encode_to_vec(&mut self, message: &M) -> Result<Vec<u8>, Self::Error> {
        let mut buffer = Vec::<u8>::new();
        self.encode(&mut buffer, message)?;
        Ok(buffer.as_slice().iter().cloned().collect())
    }
}

/// A [`StreamIterator`] that iterates over all the messages that come from a
/// [reader](std::io::Read).
///
/// This `struct` is created by the [`items`](Codec::items) method on [`Codec`].
/// See its documentation for more.
pub struct ItemsIter<C, R, M> {
    codec: C,
    source: R,
    phantom: PhantomData<M>,
}

pub struct Frames<C, R, M, T> {
    codec: C,
    source: R,
    phantom: PhantomData<M>,
    err: Option<FramelessError<T>>,
}

#[derive(Debug)]
pub enum FramelessError<E> {
    Decoder(E),
    Io(io::Error),
}

impl<'s, M, C, R> StreamIterator<'s> for Frames<C, R, M, C::Error>
where
    C: FramelessDecoder<'s, M>,
    R: io::Read,
{
    type Item = Result<M, &'s FramelessError<C::Error>>;

    fn advance(&mut self) {
        loop {
            let buffer = self.codec.supply_buffer();
            if let Err(e) = self.source.read(buffer) {
                self.err = Some(FramelessError::Io(e));
                break;
            }
            match self.codec.attempt_decoding() {
                Err(e) => {
                    self.err = Some(FramelessError::Decoder(e));
                    break;
                },
                Ok(Poll::Incomplete) => (),
                Ok(Poll::Ready) => break,
            }
        }
    }

    fn get(&'s self) -> Option<Self::Item> {
        match &self.err {
            Some(e) => {
                Some(Err(&e))
            }
            None => Some(Ok(self.codec.get_item())),
        }
    }
}

/// Represents the progress that a codec device has made in regard to the current
/// message.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Poll {
    /// The message has been fully decoded and is available in the internal buffer.
    Ready,
    /// The message is incomplete and more bytes are needed for completing
    /// decoding.
    Incomplete,
}

pub trait TransmuterPattern: Clone {}
