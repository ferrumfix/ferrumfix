//! *Simple Open Framing Header*
//! ([SOFH](https://www.fixtrading.org/standards/fix-sofh-online)) support.
//!
//! SOFH provides encoding-agnostic message framing. By SOFH rules, each payload
//! is preceded by a header that consists of six (6) bytes, which contain
//! information regarding both
//! - payload's encoding type
//! - payload's total length

mod encoding_type;
mod err;
mod frame;
mod seq_decoder;
#[cfg(feature = "utils-tokio")]
mod tokio_codec;

pub use encoding_type::EncodingType;
pub use err::Error;
pub use frame::Frame;
pub use seq_decoder::{Frames, SeqDecoder};
#[cfg(feature = "utils-tokio")]
pub use tokio_codec::TokioCodec;

use std::convert::TryInto;

fn field_message_length(data: &[u8]) -> u32 {
    u32::from_be_bytes(data[0..4].try_into().unwrap())
}

fn field_encoding_type(data: &[u8]) -> u16 {
    u16::from_be_bytes(data[4..6].try_into().unwrap())
}
