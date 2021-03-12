//! Simple Open Framing Header (SOFH) support.
//!
//! SOFH provides encoding-agnostic message framing. By SOFH rules, each payload
//! is preceded by a header that consists of six (6) bytes, which contain
//! information regarding both
//! - payload's encoding type
//! - payload's total length
//!
//! Please refer to https://www.fixtrading.org/standards/fix-sofh/ for more
//! information.

mod decoder;
mod encoding_type;
mod err;
mod frame;

pub use decoder::{Decoder, Frames};
pub use encoding_type::EncodingType;
pub use err::Error;
pub use frame::Frame;