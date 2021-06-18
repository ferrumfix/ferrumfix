//! Core FIX utilities, as well as encoding and decoding of FIX messages using
//! the standard `tag=value|` syntax.
//!
//! This is the original encoding used for FIX messages and also the encoding
//! currently used by the FIX session layer.
//!
//! # When should you use each decoder?
//!
//! This module offers several FIX decoders.
//!
//! ## Use a [`RawDecoder`] when:
//!
//! - You operate on byte slices.
//! - The only features you want are `BodyLength` and `CheckSum` verification.
//!
//! ## Use a [`RawDecoderBuffered`] when:
//!
//! - You operate on a stream of bytes, i.e. multiple FIX messages.
//! - You want to delegate FIX message framing logic (i.e. the start and end of
//! every FIX message) while maintaing as much control over parsing logic as
//! possible.
//!
//! ## Use a [`Decoder`] when:
//!
//! - You operate on byte slices.
//! - You want both sequential and random access to FIX fields and groups.
//!
//! ## Use a [`DecoderBuffered`] when:
//!
//! - You operate on a stream of bytes.
//! - You want to delegate FIX message framing logic.
//! - You need pre-build field parsing logic and basic features.
//!
//! ## Summary
//!
//! |**Decoder type**      |Operates on              |Produces    |
//! |----------------------|-------------------------|------------|
//! |[`RawDecoder`]        |`&[u8]`                  |[`RawFrame`]|
//! |[`Decoder`]           |`&[u8]`                  |[`Message`] |
//! |[`RawDecoderBuffered`]|`Vec<u8>` internal buffer|[`RawFrame`]|
//! |[`DecoderBuffered`]   |`Vec<u8>` internal buffer|[`Message`] |

use crate::dict::IsFieldDefinition;
use crate::FixValue;
use std::fmt;
use std::fmt::Debug;
use std::io;

mod config;
mod decoder;
mod encoder;
mod field_access;
mod raw_decoder;
#[cfg(feature = "utils-tokio")]
mod tokio_decoder;
mod utils;

pub use config::{Config, Configure};
pub use decoder::{Decoder, DecoderBuffered, Fields, Message, MessageGroup, MessageGroupEntry};
pub use encoder::{Encoder, EncoderHandle};
pub use field_access::{FieldAccess, RepeatingGroup};
pub use raw_decoder::{RawDecoder, RawDecoderBuffered, RawFrame};
#[cfg(feature = "utils-tokio")]
pub use tokio_decoder::TokioDecoder;

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

pub trait FvWrite<'a> {
    type Key;

    fn set_fv_with_key<'b, T>(&'b mut self, key: &Self::Key, value: T)
    where
        T: FixValue<'b>;

    fn set_fv<'b, V, F>(&'b mut self, field: &F, value: V)
    where
        V: FixValue<'b>,
        F: IsFieldDefinition;
}
