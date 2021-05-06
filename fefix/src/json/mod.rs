//! Encoding and decoding of FIX messages using JSON.

use std::error::Error;
use std::fmt;

mod config;
mod decoder;
mod encoder;

pub use config::{Config, Configure};
pub use decoder::{Decoder, Message, MessageGroup, MessageGroupEntry};
pub use encoder::Encoder;

#[doc(inline)]
pub use encoder::encoder_states;

/// The type returned in the event of an error when encoding a FIX JSON message.
#[derive(Copy, Clone, Debug)]
pub enum EncodeError {
    /// The type returned in case there is an inconsistency between
    /// `BeginString`, `MsgType`, fields presence and other encoding rules as
    /// establised by the dictionary.
    Dictionary,
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Incosistency between the FIX message and encoding rules as established by the dictionary.")
    }
}

impl Error for EncodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
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
        let err = match self {
            Self::Schema => "The message is valid JSON, but not a valid FIX message.",
            Self::Syntax => "Bad JSON syntax.",
            Self::InvalidMsgType => "Unrecognized message type.",
            Self::InvalidData => "The data does not conform to the specified message type.",
        };
        write!(f, "{}", err)
    }
}

impl Error for DecodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
