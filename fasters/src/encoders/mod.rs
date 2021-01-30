//! Support for FIX-related encoding types (OSI Layer 6).
//!
//! Encoders (*aka* codecs) must implement [`Encoding`](Encoding) or
//! [`Codec`](Codec).
//!
//! - FIX tag-value: [`encoders::tagvalue::TagValue`](encoders::tagvalue::TagValue).
//! - FAST: [`encoders::fast::Fast`](encoders::fast::Fast).
//! - JSON: [`encoders::json::Json`](encoders::json::Json).
//! - SOFH: [`encoders::sofh::SofhParser`](encoders::sofh::SofhParser).
//!
//! Most encoding types support configuration options via the *transmuter
//! pattern*. Transmuters are traits that define all configurable options for a
//! specific encoding.
use std::io;

pub mod fast;
pub mod json;
pub mod sofh;
pub mod tagvalue;

/// Capabilities to decode and encode FIX messages according to a FIX dictionary.
pub trait Encoding<M> {
    type DecodeError;
    type EncodeError;

    fn decode(&self, source: &mut impl io::BufRead) -> Result<M, Self::DecodeError>;

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
    Self: 's,
{
    /// The error type that can arise during message deserialization.
    type DecodeError;
    /// The error type that can arise during message serialization.
    type EncodeError;

    /// Writes a slice of bytes into the internal buffer and attempts
    /// deserialization. Three scenarios are then possible:
    ///
    /// 1. Deserialization is complete, and the user can fetch the item:
    ///    `Ok(Poll::Ready)`.
    /// 2. No errors are detected so far, but the message is complete. The
    ///    message is not yet available for reading: `Ok(Poll::Incomplete)`.
    /// 3. An error in the data has been detected: `Err(Self::DecodeError)`.
    fn decode(&mut self, data: &[u8]) -> Result<Poll, Self::DecodeError>;

    /// Returns the last message.
    fn get_item(&'s self) -> M;

    /// Serializes `data` into the internal buffer and finally returns an
    /// immutable reference to the internal buffer.
    ///
    /// Please note that even though serialization errors are way less common
    /// than deserialization errors, one should still be careful to manage them
    /// when they arise.
    fn encode(&mut self, data: M) -> Result<&[u8], Self::EncodeError>;
}

/// Represents the progress that a codec device has made in regard to the current
/// message.
pub enum Poll {
    /// The message has been fully deserialized and is available in the internal buffer.
    Ready,
    /// The message is incomplete and more bytes are needed for completing
    /// deserialization.
    Incomplete,
}

pub trait TransmuterPattern: Clone {}
