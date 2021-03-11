//! JSON encoding for FIX support.

use std::fmt;

mod codec;
mod config;

pub use codec::Codec;
pub use config::{Configure, ConfigPrettyPrint, Config};

/// The type returned in the event of an error when encoding a FIX JSON message.
#[derive(Copy, Clone, Debug)]
pub enum EncoderError {
    /// The type returned in case there is an inconsistency between
    /// `BeginString`, `MsgType`, fields presence and other encoding rules as
    /// establised by the dictionary.
    Dictionary,
}

/// The type returned in the event of an error when decoding a FIX JSON message.
#[derive(Copy, Clone, Debug)]
pub enum DecodeError {
    /// Bad JSON syntax.
    Syntax,
    /// The message is valid JSON, but not a valid FIX message.
    Schema,
    /// Unrecognized message type.
    InvalidMsgType,
    /// The data does not conform to the specified message type.
    InvalidData,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FIX JSON decoding error.")
    }
}
