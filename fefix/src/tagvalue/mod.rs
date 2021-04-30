//! Encoding and decoding of FIX messages with standard `tag=value` syntax.
//!
//! This is the original encoding used for FIX messages and also the encoding
//! currently used by the FIX session layer.

use crate::dict::IsTypedFieldDefinition;
use crate::{datatypes::SuperDataType, DataType};
use std::fmt;
use std::fmt::Debug;
use std::io;

mod config;
mod decoder;
mod encoder;
mod field_getters;
mod raw_decoder;
mod utils;

pub use config::{Config, Configure};
pub use decoder::{Decoder, DecoderBuffered, Message, MessageGroup, MessageGroupEntry};
pub use encoder::{Encoder, EncoderHandle};
pub use field_getters::FieldGetter as Fv;
pub use raw_decoder::{RawDecoder, RawDecoderBuffered, RawFrame};

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

pub trait MapFields<'a>
where
    Self: Fv<'a>,
{
    type Group: MapGroup<'a>;

    fn group(&'a self) -> Self::Group;
}

pub trait MapGroup<'a> {
    type Entry: MapFields<'a>;

    fn len(&self) -> usize;

    fn entry(&self) -> Self::Entry;
}

pub trait FvWrite<'a> {
    type Key;

    fn set_fv_with_key<'b, T>(&'b mut self, key: &Self::Key, value: T)
    where
        T: DataType<'b>;

    fn set_fv<'b, V, T, F>(&'b mut self, field: &F, value: V)
    where
        V: DataType<'b>,
        T: SuperDataType<'b, V>,
        F: IsTypedFieldDefinition<T>;
}
