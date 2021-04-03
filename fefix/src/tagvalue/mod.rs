//! FIX tag-value encoding support.
//!
//! This is the original encoding used for FIX messages and also the encoding
//! currently used by the FIX session layer.

use crate::tagvalue::field_value as val;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Debug;
use std::io;
use std::time::SystemTime;

mod config;
mod decoder;
mod encoder;
mod field_setter;
pub mod field_value;
mod fix_message_ref;
mod group_delimiter;
mod raw_decoder;
mod raw_encoder;
mod taglookup;
mod utils;

pub use config::{Config, Configure};
pub use decoder::{Decoder, DecoderBuffered};
pub use encoder::Encoder;
pub use field_setter::MessageAccumulator;
pub use fix_message_ref::{FixMessageRef, FixMessageRefBuilder, GroupRef, GroupRefIter};
pub use group_delimiter::GroupDelimiter;
pub use raw_decoder::{RawDecoder, RawDecoderBuffered, RawFrame};
pub use raw_encoder::RawEncoder;
pub use taglookup::{TagLookup, TagLookupSingleAppVersion};
pub use utils::{checksum_10, encode_raw};

/// The type returned in the event of an error during message encoding.
type EncodeError = ();

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

/// An owned value of a FIX field.
#[derive(Clone, Debug, PartialEq)]
pub enum FixFieldValue {
    Atom(val::FieldValue<'static>),
    Group(Vec<BTreeMap<i64, FixFieldValue>>),
}

impl FixFieldValue {
    pub fn string(data: &[u8]) -> Option<Self> {
        std::str::from_utf8(data)
            .ok()
            .map(|s| Self::Atom(val::FieldValue::string(s.to_string())))
    }

    pub fn as_length(&self) -> Option<usize> {
        if let Self::Atom(val::FieldValue::Length(length)) = self {
            Some((*length).into())
        } else {
            None
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        if let Self::Atom(val::FieldValue::Int(x)) = self {
            Some((*x).into())
        } else {
            None
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        if let Self::Atom(val::FieldValue::String(s)) = self {
            Some(s.as_str())
        } else {
            None
        }
    }
}

impl From<i64> for FixFieldValue {
    fn from(v: i64) -> Self {
        FixFieldValue::Atom(val::FieldValue::int(v as i64))
    }
}

impl From<String> for FixFieldValue {
    fn from(v: String) -> Self {
        FixFieldValue::Atom(val::FieldValue::string(v))
    }
}

impl From<f64> for FixFieldValue {
    fn from(v: f64) -> Self {
        FixFieldValue::Atom(val::FieldValue::float(v as f32))
    }
}

impl From<(u8, u16)> for FixFieldValue {
    fn from(v: (u8, u16)) -> Self {
        FixFieldValue::from(((v.0 as i64) << 16) + (v.1 as i64))
    }
}

impl From<char> for FixFieldValue {
    fn from(v: char) -> Self {
        FixFieldValue::Atom(val::FieldValue::char(v))
    }
}

impl From<usize> for FixFieldValue {
    fn from(v: usize) -> Self {
        FixFieldValue::from(v as i64)
    }
}

impl From<Vec<u8>> for FixFieldValue {
    fn from(v: Vec<u8>) -> Self {
        FixFieldValue::Atom(val::FieldValue::Data(v))
    }
}

impl From<bool> for FixFieldValue {
    fn from(v: bool) -> Self {
        FixFieldValue::from(if v { 't' } else { 'f' })
    }
}

impl From<u8> for FixFieldValue {
    fn from(v: u8) -> Self {
        FixFieldValue::from(i64::from(v))
    }
}

impl From<SystemTime> for FixFieldValue {
    fn from(v: SystemTime) -> Self {
        FixFieldValue::from(v.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64)
    }
}
