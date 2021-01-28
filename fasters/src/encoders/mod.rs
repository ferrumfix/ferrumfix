//! Support for FIX-related encoding types (OSI Layer 6).
use std::error::Error;
use std::io;

pub mod fast;
mod json;
pub mod sofh;
mod tagvalue;

pub use fast::Fast;
pub use json::Json;
pub use tagvalue::TagValue;

/// Capabilities to decode and encode FIX messages according to a FIX dictionary.
pub trait Encoding<M> {
    type DecodeError;
    type EncodeError;

    fn decode(&self, source: &mut impl io::BufRead) -> Result<M, Self::DecodeError>;

    fn encode(&mut self, message: M) -> Result<Vec<u8>, Self::EncodeError>;
}

/// A decoder and encoder device with an internal buffer to minimize allocations.
pub trait Codec<'a, 's, M>
where
    Self: 's,
    's: 'a,
    M: 'a,
{
    type DecodeError;
    type EncodeError;

    fn erase_buffer(&mut self);
    fn supply_buffer(&mut self) -> Result<&mut [u8], Self::DecodeError>;
    fn poll(&self, num_bytes: usize) -> Poll;

    fn decode(&mut self) -> Result<(), Self::DecodeError>;
    fn get_item(&'s self) -> M;
    fn encode(&'s mut self, data: M) -> Result<&'s [u8], Self::EncodeError>;
}

pub enum Poll {
    KeepThemComing,
    StopNow,
}
