//! Simple Open Framing Header
//! ([SOFH](https://www.fixtrading.org/standards/fix-sofh-online)) support.
//!
//! SOFH provides encoding- and content-agnostic message framing over wire. SOFH
//! mandates that each message is preceded by a six (6) byte header, which
//! contains two pieces of information:
//!
//! 1. The total length of the current message, in bytes.
//! 2. The "encoding type" of such message, i.e. a numeric code used to pass
//! information to the receiver(s) as to the kind of payload. [`EncodingType`]
//! comes in handy when dealing with standard encoding types.
//!
//! The preferred way to send and receive SOFH-enclosed messages over wire is with
//! [`TokioCodec`].

#![doc(html_root_url = "https://docs.rs/fesofh/")]
#![warn(missing_docs, rustdoc::missing_doc_code_examples)]
#![deny(
    unused,
    missing_debug_implementations,
    unsafe_op_in_unsafe_fn,
    rustdoc::broken_intra_doc_links,
    //missing_docs,
    unconditional_recursion,
    unstable_name_collisions,
    clippy::useless_conversion,
    clippy::missing_panics_doc,
    clippy::mixed_case_hex_literals,
    clippy::needless_bool,
    clippy::needless_lifetimes
)]
// Only enables the `doc_cfg` feature when its feature is defined.
#![cfg_attr(doc_cfg, feature(doc_cfg))]

mod encoding_type;
mod frame;
mod seq_decoder;

#[cfg(feature = "utils-tokio")]
mod tokio_codec;

pub use encoding_type::EncodingType;
pub use frame::Frame;
pub use seq_decoder::{Frames, SeqDecoder};
use std::convert::TryInto;
use std::io;
use thiserror::Error;

#[cfg(feature = "utils-tokio")]
pub use tokio_codec::TokioCodec;

/// The type returned in the event of an error when decoding SOFH-enclosed
/// messages.
#[derive(Debug, Error)]
pub enum Error {
    /// The SOFH-enclosed message's length is outside the legal range.
    #[error("The SOFH-enclosed message's length is outside the legal range.")]
    InvalidMessageLength,
    /// The SOFH-enclosed message is incomplete. More bytes are needed.
    #[error("The SOFH-enclosed message is incomplete. {needed} more bytes are needed.")]
    Incomplete {
        /// The number of missing bytes to complete the SOFH-enclosed message.
        needed: usize,
    },
    /// I/O-related error.
    #[error("I/O related error.")]
    Io(#[from] io::Error),
}

/// The header of a SOFH-enclosed message.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Header {
    /// The length of the message payload, in bytes. This does *not* include the
    /// length of the header itself.
    pub nominal_message_length_in_bytes: usize,
    /// The "encoding type" of the message payload.
    pub encoding_type: u16,
}

impl Header {
    const LENGTH_IN_BYTES: usize = 6;

    fn to_bytes(&self) -> [u8; Self::LENGTH_IN_BYTES] {
        let mut bytes = [0u8; Self::LENGTH_IN_BYTES];
        let (a, b) = bytes.split_at_mut(4);
        a.copy_from_slice(&(self.nominal_message_length_in_bytes as u32).to_be_bytes());
        b.copy_from_slice(&self.encoding_type.to_be_bytes());
        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let err_incomplete = || Error::Incomplete {
            needed: Self::LENGTH_IN_BYTES - bytes.len(),
        };
        let header = bytes
            .get(..Self::LENGTH_IN_BYTES)
            .ok_or_else(err_incomplete)?
            .try_into()
            .map(|header_bytes: &[u8; Self::LENGTH_IN_BYTES]| {
                let nominal_message_length_in_bytes =
                    u32::from_be_bytes(header_bytes[0..4].try_into().unwrap()) as usize;
                let encoding_type = u16::from_be_bytes(header_bytes[4..6].try_into().unwrap());
                Self {
                    nominal_message_length_in_bytes,
                    encoding_type,
                }
            })
            .map_err(|_| err_incomplete())?;
        if header.nominal_message_length_in_bytes < Self::LENGTH_IN_BYTES {
            Err(Error::InvalidMessageLength)
        } else {
            Ok(header)
        }
    }
}
