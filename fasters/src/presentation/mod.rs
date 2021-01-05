//! Support for FIX-related encoding types (OSI Layer 6).
use std::io;
use std::error::Error;

mod fast;
mod json;
mod tagvalue;
pub mod sofh;

pub use fast::Fast;
pub use tagvalue::TagValue;

/// Capabilities to decode and encode FIX messages according to a FIX dictionary.
pub trait Encoding<M> {
    type DecodeErr: Error;
    type EncodeErr: Error;

    fn decode(&self, source: &mut impl io::BufRead) -> Result<M, Self::DecodeErr>;
    fn encode(&self, message: M) -> Result<Vec<u8>, Self::EncodeErr>;
}
