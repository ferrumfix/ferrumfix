//! Encoding and decoding of FIX messages using JSON.

mod config;
mod decoder;
mod encoder;

pub use config::Config;
pub use decoder::{Decoder, FieldOrGroup, Message, MessageFieldsIter, MessageGroup};
pub use encoder::Encoder;

/// The type returned in the event of an error when encoding a FIX JSON message.
#[derive(Copy, Clone, Debug, thiserror::Error)]
pub enum EncodeError {
    /// The type returned in case there is an inconsistency between
    /// `BeginString`, `MsgType`, fields presence and other encoding rules as
    /// establised by the dictionary.
    #[error(
        "Inconsistency between the FIX message and encoding rules as established by the dictionary."
    )]
    Dictionary,
}

/// The type returned in the event of an error when decoding a FIX JSON message.
#[derive(Copy, Clone, Debug, thiserror::Error)]
pub enum DecodeError {
    /// Bad JSON syntax.
    #[error("Bad JSON syntax.")]
    Syntax,
    /// The message is valid JSON, but not a valid FIX message.
    #[error("The message is valid JSON, but not a valid FIX message.")]
    Schema,
    /// Unrecognized message type.
    #[error("Unrecognized message type.")]
    InvalidMsgType,
    /// The data does not conform to the specified message type.
    #[error("The data does not conform to the specified message type.")]
    InvalidData,
}
