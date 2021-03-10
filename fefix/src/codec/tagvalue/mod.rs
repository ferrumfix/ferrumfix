//! FIX tag-value encoding support.
//!
//! This is the original encoding used for FIX messages and also the encoding
//! currently used by the FIX session layer.

use std::fmt;
use std::fmt::Debug;
use std::io;

mod agnostic;
mod codec;
mod config;
mod taglookup;
mod utils;

pub use agnostic::{AgnosticMessage, CodecAgnostic};
pub use codec::{Codec, CodecBuffered};
pub use config::{Config, ConfigFastDefault, Configurable};
pub use taglookup::{TagLookup, TagLookupPredetermined};

/// The type returned in the event of an error during message encoding.
type EncodeError = ();

/// The type returned in the event of an error during message decoding.
#[derive(Clone, Debug, PartialEq)]
pub enum DecodeError {
    FieldPresence,
    Syntax,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<io::Error> for DecodeError {
    fn from(_err: io::Error) -> Self {
        Self::Syntax // FIXME
    }
}
