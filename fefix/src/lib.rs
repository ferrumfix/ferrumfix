//! A Financial Information eXchange
//! ([FIX](https://www.fixtrading.org/standards/)) protocol implementation in Rust.
//!
//! FerrumFIX is a collection of reusable components to produce and consume
//! FIX-compliant data. It is *not* a FIX engine, although you can very easily
//! build one with FerrumFIX. FerrumFIX is:
//!
//!  - **Unopinionated**. FerrumFIX takes care of every little detail of the FIX
//!  specification, but no configurations or decisions are mandated to the user
//!  (as much as practically feasible).
//!  - **Comprehensive**. Most standards adopted by the FIX Community are
//!  available, from [transport] and [session] layers to [encodings](encoders) and
//!  dictionary-related [application](backend) logic.
//!  - **Foundational**. FerrumFIX is foundational in the sense that it exposes a
//!  large amount of primitives in its public interface, so that users can
//!  easily build upon them to implement custom solutions tailored for their
//!  needs. Multiple FIX message data structures are available.
//!  - **Fast**. We favor configuration via trait interfaces directly in code rather
//!  than files. This results in much faster code at the cost of compilation speed
//!  and code size.
//!  
//! Please check out the [README](https://github.com/neysofu/fefix/) for more
//! general information regarding FerrumFIX.

#![deny(
    unused,
    missing_debug_implementations,
    clippy::useless_conversion,
    clippy::missing_panics_docs,
    clippy::mixed_case_hex_literals,
    clippy::needless_bool,
    clippy::needless_lifetimes
)]

mod utils;

mod app_version;
pub mod backend;
pub mod buffering;
mod dictionary;
mod dt;
pub mod engines;
pub mod fast;
mod fix_codegen;
pub mod json;
mod quickfix_specs;
pub mod session;
pub mod sofh;
mod stream_iterator;
pub mod tagvalue;
pub mod transport;

pub use app_version::AppVersion;
pub use dictionary::{Dictionary, MsgType};
pub use dt::DataType;
pub use fefix_derive::*;
pub use fix_codegen::codegen;
pub use quickfix_specs::quickfix_spec;
pub use stream_iterator::StreamIterator;

#[cfg(expose_openssl)]
pub extern crate openssl;

#[cfg(not(expose_openssl))]
pub(crate) extern crate openssl;

use crate::buffering::*;
use std::io;
use std::marker::PhantomData;

/// A device that can parse a stream of bytes into messages.
pub trait StreamingDecoder<M>: Sized {
    /// The type returned in the event of a deserialization error.
    type Error;

    /// Returns a mutable slice of bytes to accomodate for new input.
    ///
    /// The slice
    /// can have any non-zero length, depending on how many bytes `self` believes
    /// is a good guess. All bytes should be set to 0.
    fn supply_buffer(&mut self) -> (&mut usize, &mut [u8]);

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

/// A device that can consume and produce message instances.
pub trait Encoding<T> {
    /// The type returned in the event of a deserialization error.
    type DecodeError;
    /// The type returned in the event of a serialization error.
    type EncodeError;

    /// Parses the contents of `data` into a message `M`, which is then returned
    /// in the form of an immutable reference. In the event of failure, this
    /// method will return `Self::Error`.
    fn decode(&mut self, data: &[u8]) -> Result<&T, Self::DecodeError>;

    /// Encodes `message` to `buffer`. It then returns the number of bytes
    /// written to `buffer`.
    fn encode<B>(&mut self, buffer: &mut B, message: &T) -> Result<usize, Self::EncodeError>
    where
        B: Buffer;

    /// Allocates a buffer on the heap and encodes `message` to it.
    fn encode_to_vec(&mut self, message: &T) -> Result<Vec<u8>, Self::EncodeError> {
        let mut buffer = Vec::<u8>::new();
        self.encode(&mut buffer, message)?;
        Ok(buffer.as_slice().iter().cloned().collect())
    }
}

/// A device that writes messages to a [`Buffer`].
//pub trait Encoder<M> {
//    /// The type returned in the event of a serialization error.
//    type Error;
//
//    fn message(&self) -> &M;
//
//    fn message_mut(&mut self) -> &mut M;
//
//    /// Encodes `message` to `buffer`. It then returns the number of bytes
//    /// written to `buffer`.
//    fn encode(&mut self, buffer: impl Buffer) -> Result<usize, Self::Error>;
//
//    /// Allocates a buffer on the heap and encodes `message` to it.
//    fn encode_to_vec(&mut self) -> Result<Vec<u8>, Self::Error> {
//        let mut buffer = Vec::<u8>::new();
//        self.encode(&mut buffer)?;
//        Ok(buffer.as_slice().iter().cloned().collect())
//    }
//}

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
            let (len, mut buffer) = self.decoder.supply_buffer();
            match self.source.read(&mut buffer) {
                Err(e) => {
                    self.err = Some(e.into());
                    break;
                }
                Ok(count) => {
                    *len = count;
                }
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
