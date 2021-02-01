//! Support for FIX-related encoding types (OSI Layer 6).
//!
//! Encoders (*aka* codecs) must implement [`Encoding`](Encoding) or
//! [`Codec`](Codec).
//!
//! - FIX tag-value: [`tagvalue::TagValue`].
//! - FAST: [`fast::Fast`].
//! - JSON: [`json::Json`].
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

/// Capabilities to decode and encode FIX messages according to a FIX dictionary.
pub trait Encoding<M> {
    /// The type of any error that can arise during message decoding.
    type DecodeError;
    /// The type of any error that can arise during message encoding.
    type EncodeError;

    /// Reads a single message from `source` and then returns it if successful.
    ///
    /// When called successfully, this method will most likely result in one or
    /// more allocations.
    fn decode(&self, source: &mut impl io::BufRead) -> Result<M, Self::DecodeError>;

    /// Serializes `message` into a [`Vec<u8>`](Vec) and then returns it if
    /// successful.
    ///
    /// When called successfully, this method requires at minimum one allocation
    /// for the returned [`Vec<u8>`](Vec).
    fn encode(&mut self, message: M) -> Result<Vec<u8>, Self::EncodeError>;
}

/// A decoder and encoder device with an internal buffer to minimize allocations.
///
/// # Type parameters and generics
/// This trait requires one lifetime generic and one type parameter, `M`,
/// which represents the message type. The lifetime generic always refers to
/// `Self` and allows `M` to hold references to the internal buffer, e.g.:
///
/// ```
/// impl<'a, 's> Codec<'a, 's, MessageType<'s>>
/// ```
///
/// There are several cases which require this specific pattern and thus it is
/// supported; in other cases `'s` will be mostly useless, notably codecs that give
/// away ownership of messages.
pub trait Codec<'s, M>
where
    Self: Sized + 's,
{
    /// The type of any error that can arise during message decoding.
    type DecodeError;
    /// The type of any error that can arise during message encoding.
    type EncodeError;

    /// Returns a mutable slice of bytes to accomodate for new input.
    ///
    /// The slice
    /// can have any non-zero length, depending on how many bytes `self` believes
    /// is a good guess. All bytes should be set to 0.
    fn supply_buffer(&mut self) -> &mut [u8] {
        unimplemented!();
    }

    /// Validates the contents of the internal buffer and possibly caches the
    /// resulting message. When successful, this method will return a [`Poll`] to
    /// let the caller know whether more bytes are needed or not.
    fn poll_decoding(&mut self) -> Result<Poll, Self::DecodeError> {
        unimplemented!();
    }

    fn decode_next_item(&'s mut self) -> Result<Option<M>, Self::DecodeError> {
        self.poll_decoding().map(move |t| match t {
            Poll::Ready => Some(self.get_item()),
            Poll::Incomplete => None,
        })
    }

    /// Returns the last message.
    fn get_item(&'s self) -> M;

    /// Writes a slice of bytes into the internal buffer and attempts
    /// decoding. Three scenarios are then possible:
    ///
    /// 1. Decoding is complete, and the user can fetch the item:
    ///    `Ok(Poll::Ready)`.
    /// 2. No errors are detected so far, but the message is complete. The
    ///    message is not yet available for reading: `Ok(Poll::Incomplete)`.
    /// 3. An error in the data has been detected: `Err(Self::DecodeError)`.
    fn decode(&mut self, data: &[u8]) -> Result<Poll, Self::DecodeError>;

    fn encode_to_buffer(&self, data: M, buffer: &mut [u8]) -> Result<usize, Self::EncodeError> {
        unimplemented!()
    }

    /// Encodes `data` into the internal buffer and finally returns an
    /// immutable reference to the internal buffer.
    ///
    /// Please note that even though encoding errors are way less common
    /// than decoding errors, one should still be careful to manage them
    /// when they arise.
    fn encode(&mut self, data: M) -> Result<&[u8], Self::EncodeError>;

    fn items<R>(self, source: R) -> ItemsIter<Self, R, M>
    where
        R: io::Read,
    {
        ItemsIter {
            codec: self,
            source,
            phantom: PhantomData::default(),
        }
    }
}

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

pub trait Decoder<'s, M> {
    type Error;

    fn decode(&'s mut self, data: &'s [u8]) -> Result<M, Self::Error>;
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

impl<'s, M, C, R> StreamIterator<'s> for ItemsIter<C, R, M>
where
    C: Codec<'s, M>,
    R: io::Read,
{
    type Item = M;

    fn advance(&mut self) {
        loop {
            let buffer = self.codec.supply_buffer();
            self.source.read(buffer).unwrap();
            let status = self.codec.poll_decoding().ok().unwrap();
            match status {
                Poll::Incomplete => (),
                Poll::Ready => break,
            }
        }
    }

    fn get(&'s self) -> Option<Self::Item> {
        Some(self.codec.get_item())
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
