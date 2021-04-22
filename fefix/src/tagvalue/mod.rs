//! FIX tag-value encoding support.
//!
//! This is the original encoding used for FIX messages and also the encoding
//! currently used by the FIX session layer.

use super::dtf;
use super::FieldDef;
use crate::{DataField, OptError, OptResult};
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
pub use decoder::{Decoder, DecoderBuffered, Message, MessageGroup, MessageGroupEntry};
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
        T: DataField<'b>;

    fn set_fv<'b, T, S>(&'b mut self, field: &FieldDef<'b, T>, value: S)
    where
        T: DataField<'b>,
        S: DataField<'b>;
}

/// A trait to retrieve field values in a FIX message.
pub trait Fv<'a> {
    type Key;

    fn fv_raw_with_key<'b>(&'b self, key: &Self::Key) -> Option<&'b [u8]>;

    fn fv_raw<'b, T>(&'b self, field: &FieldDef<'b, T>) -> Option<&'b [u8]>
    where
        'b: 'a,
        T: dtf::DataField<'b>;

    fn fv_opt<'b, T, S>(&'b self, field: &FieldDef<'b, T>) -> Option<Result<S, S::Error>>
    where
        'b: 'a,
        T: dtf::DataField<'b>,
        S: dtf::SubDataField<'b, T>,
    {
        self.fv_raw(field).map(|raw| match T::deserialize(raw) {
            Ok(value) => S::convert(value),
            Err(err) => Err(err.into()),
        })
    }

    fn fv<'b, T, S>(&'b self, field: &FieldDef<'b, T>) -> OptResult<S, S::Error>
    where
        'b: 'a,
        T: dtf::DataField<'b>,
        S: dtf::SubDataField<'b, T>,
    {
        match self.fv_opt(field) {
            Some(Ok(x)) => Ok(x),
            Some(Err(err)) => Err(OptError::Other(err)),
            None => Err(OptError::None),
        }
    }

    fn fvl_opt<'b, T, S>(&'b self, field: &FieldDef<'b, T>) -> Option<Result<S, S::Error>>
    where
        'b: 'a,
        T: dtf::DataField<'b>,
        S: dtf::SubDataField<'b, T>,
    {
        self.fv_raw(field)
            .map(|raw| match T::deserialize_lossy(raw) {
                Ok(value) => S::convert(value),
                Err(err) => Err(err.into()),
            })
    }

    fn fvl<'b, T, S>(&'b self, field: &FieldDef<'b, T>) -> Result<S, OptError<S::Error>>
    where
        'b: 'a,
        T: dtf::DataField<'b>,
        S: dtf::SubDataField<'b, T>,
    {
        match self.fvl_opt(field) {
            Some(Ok(x)) => Ok(x),
            Some(Err(err)) => Err(OptError::Other(err)),
            None => Err(OptError::None),
        }
    }
}
