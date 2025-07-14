//! # RustyGPB - Protocol Buffers for FIX Messages
//!
//! High-performance Protocol Buffers encoding/decoding for FIX messages optimized for
//! ultra-low latency trading systems.
//!
//! ## Features
//!
//! - Zero-copy deserialization where possible
//! - SIMD-aligned buffers for optimal performance  
//! - Support for standard FIX message types
//! - Streaming message processing
//! - Comprehensive error handling
//!
//! ## Example
//!
//! ```rust
//! use rustygpb::{GpbEncoder, GpbDecoder, FixMessage};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut encoder = GpbEncoder::new();
//!     let message = FixMessage::new_order_single(
//!         "BTCUSD".to_string(),
//!         50000.0,
//!         1.0,
//!         "1".to_string()
//!     );
//!     let bytes = encoder.encode(&message)?;
//!
//!     let decoder = GpbDecoder::new();
//!     let decoded = decoder.decode(bytes)?;
//!     Ok(())
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/rustygpb/")]
#![deny(
    unused,
    missing_debug_implementations,
    unsafe_op_in_unsafe_fn,
    rustdoc::broken_intra_doc_links,
    missing_docs,
    unconditional_recursion,
    unstable_name_collisions,
    clippy::useless_conversion,
    clippy::missing_panics_doc,
    clippy::mixed_case_hex_literals,
    clippy::needless_bool,
    clippy::needless_lifetimes
)]
#![allow(
    clippy::uninlined_format_args,
    clippy::should_implement_trait,
    clippy::match_like_matches_macro,
    clippy::too_many_arguments,
    clippy::for_kv_map,
    clippy::identity_op,
    clippy::io_other_error,
    clippy::manual_div_ceil,
    clippy::needless_borrow
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod buffer;
mod decoder;
mod encoder;
mod error;
mod message;

pub use buffer::{GpbBuffer, GpbReader, GpbWriter};
pub use decoder::{DecodeConfig, GpbDecoder};
pub use encoder::{EncodeConfig, GpbEncoder};
pub use error::{DecodeError, EncodeError, GpbError};
pub use message::{FieldValue, FixMessage, MessageType};

// Re-export protobuf types for convenience
pub use prost::Message;
pub use prost_types::Timestamp;

/// Prelude module for common imports
pub mod prelude {
    pub use crate::{
        DecodeError, EncodeError, FieldValue, FixMessage, GpbDecoder, GpbEncoder, GpbError,
        Message, MessageType,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // This will fail until we implement the types
        // But it defines our TDD target
        let mut encoder = GpbEncoder::new();
        let decoder = GpbDecoder::new();

        let message = FixMessage::new_order_single("BTCUSD".into(), 1000.0, 100.0, "BUY".into());

        let encoded = encoder.encode(&message).expect("encoding should work");
        let decoded = decoder.decode(&encoded).expect("decoding should work");

        assert_eq!(message, decoded);
    }
}
