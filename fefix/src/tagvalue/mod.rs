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
mod group_delimiter;
mod message;
mod raw_decoder;
mod taglookup;
mod utils;

pub use config::{Config, Configure};
pub use decoder::{Decoder, DecoderBuffered};
pub use encoder::{Encoder, EncoderHandle};
pub use field_access::FieldAccess;
pub use group_delimiter::GroupDelimiter;
pub use message::{
    GroupRef, GroupRefIter, Message, MessageBuilder, MessageGroup, MessageGroupEntry,
};
pub use raw_decoder::{RawDecoder, RawDecoderBuffered, RawFrame};
pub use taglookup::{TagLookup, TagLookupSingleAppVersion};

/// The type returned in the event of an error during message decoding.
#[derive(Clone, Debug, PartialEq)]
pub enum DecodeError {
    FieldPresence,
    /// Invalid FIX message syntax.
    Invalid,
    CheckSum,
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
