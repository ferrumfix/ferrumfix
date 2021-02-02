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
use std::io;

pub mod fast;
pub mod json;
pub mod sofh;
pub mod tagvalue;

pub trait FramelessDecoder<'s, M>
where
    Self: Sized,
{
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

    /// Returns the last message.
    fn get_item(&'s self) -> M;

    fn decode_next_item(&'s mut self) -> Result<Option<M>, Self::Error> {
        self.attempt_decoding().map(move |t| match t {
            Poll::Ready => Some(self.get_item()),
            Poll::Incomplete => None,
        })
    }
}

pub trait FramelessRefDecoder<M>
where
    Self: Sized,
{
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
}

pub trait Decoder<'a, M> {
    type Error;

    fn decode(&mut self, data: &'a [u8]) -> Result<M, Self::Error>;
}

pub trait RefDecoder<M> {
    type Error;

    fn decode(&mut self, data: &[u8]) -> Result<&M, Self::Error>;
}

pub trait Encoder<M> {
    type Error;

    /// Encodes `message` on `buffer`.
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
/// This `struct` is created by the [`frames`](FramelessDecoder::frames) method
/// on [`FramelessDecoder`].
/// See its documentation for more.
#[derive(Debug)]
pub struct Frames<C, R, M, T> {
    codec: C,
    source: R,
    message: Option<M>,
    err: Option<FramelessError<T>>,
}

#[derive(Debug)]
pub enum FramelessError<E> {
    Decoder(E),
    Io(io::Error),
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
