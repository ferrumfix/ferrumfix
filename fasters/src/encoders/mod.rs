//! Support for FIX-related encoding types (OSI Layer 6).
use std::error::Error;
use std::io;

mod fast;
mod json;
pub mod sofh;
mod tagvalue;

pub use fast::Fast;
pub use json::Json;
pub use tagvalue::TagValue;

/// Capabilities to decode and encode FIX messages according to a FIX dictionary.
pub trait Encoding<M> {
    type DecodeErr: Error;
    type EncodeErr: Error;

    fn decode(&self, source: &mut impl io::BufRead) -> Result<M, Self::DecodeErr>;
    fn encode(&self, message: M) -> Result<Vec<u8>, Self::EncodeErr>;
}
