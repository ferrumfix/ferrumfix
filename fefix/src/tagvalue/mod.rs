//! FIX tag-value encoding support.
//!
//! This is the original encoding used for FIX messages and also the encoding
//! currently used by the FIX session layer.

use std::fmt;
use std::fmt::Debug;
use std::io;

mod config;
mod decoder;
mod encoder;
mod field_access;
mod raw_decoder;
mod utils;

pub use config::{Config, Configure};
pub use decoder::{
    Decoder, DecoderBuffered, GroupRef, GroupRefIter, Message, MessageBuilder, MessageGroup,
    MessageGroupEntry,
};
pub use encoder::{Encoder, EncoderHandle};
pub use field_access::FieldAccess;
pub use raw_decoder::{RawDecoder, RawDecoderBuffered, RawFrame};

/// The type returned in the event of an error during message decoding.
#[derive(Clone, Debug, PartialEq)]
pub enum DecodeError {
    FieldPresence,
    /// Invalid FIX message syntax.
    Invalid,
    CheckSum,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OptionalFieldError<E> {
    Missing,
    Invalid(E),
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
        Self::Invalid // FIXME
    }
}
