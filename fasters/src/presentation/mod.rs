//! Support for FIX-related encoding types (OSI Layer 6).
use crate::app::slr;
use std::io;
use std::error::Error;

mod fast;
mod tagvalue;
pub mod sofh;

pub use fast::Fast;
pub use tagvalue::TagValue;

/// Capabilities to decode and encode FIX messages according to a FIX dictionary.
pub trait Encoding {
    type DecodeErr: Error;
    type EncodeErr: Error;

    fn decode(&self, source: &mut impl io::BufRead) -> Result<slr::Message, Self::DecodeErr>;
    fn encode(&self, message: slr::Message) -> Result<Vec<u8>, Self::EncodeErr>;
}
