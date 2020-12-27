//! Support for FIX-related encoding types (OSI Layer 6).
use crate::ir;
use std::io;

mod fast;
mod tagvalue;

pub use fast::Fast;
pub use tagvalue::TagValue;

/// Capabilities to decode and encode FIX messages according to a FIX dictionary.
pub trait Encoding {
    type DecodeErr;
    type EncodeErr;

    fn decode(&self, source: &mut impl io::BufRead) -> Result<ir::Message, Self::DecodeErr>;
    fn encode(&self, message: ir::Message) -> Result<Vec<u8>, Self::EncodeErr>;
}
