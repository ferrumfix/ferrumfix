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
//! ## Use a [`RawDecoderStreaming`] when:
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
//! ## Use a [`DecoderStreaming`] when:
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
//! |[`RawDecoderStreaming`]|byte streams             |[`RawFrame`]|
//! |[`Decoder`]           |`&[u8]`                  |[`Message`] |
//! |[`DecoderStreaming`]   |data streams             |[`Message`] |

use std::fmt::Debug;

mod config;
mod decoder;
mod encoder;
mod raw_decoder;
mod utils;

pub use config::Config;
pub use decoder::{Decoder, DecoderStreaming, Fields, Message, MessageGroup};
pub use encoder::{Encoder, EncoderHandle};
pub use raw_decoder::{RawDecoder, RawDecoderStreaming, RawFrame};

#[cfg(feature = "utils-tokio")]
mod tokio_decoder_adapter;
#[cfg(feature = "utils-tokio")]
pub use tokio_decoder_adapter::TokioDecoderAdapter;

/// The type returned in the event of an error during message decoding.
#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    /// Mandatory field not found.
    #[error("Field not found.")]
    FieldPresence,
    /// Invalid FIX message syntax, `BodyLength <9>` value mismatch, or similar errors.
    #[error("Invalid FIX message syntax.")]
    Invalid,
    /// Invalid `CheckSum <10>` FIX field value.
    #[error("Invalid `CheckSum <10>` FIX field value.")]
    CheckSum,
    /// I/O error.
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),
}
