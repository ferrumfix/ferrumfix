//! RustySBE - High-performance Simple Binary Encoding (SBE) for Rust
//!
//! This crate provides zero-copy, high-performance SBE encoding and decoding
//! capabilities optimized for financial trading systems and other low-latency
//! applications.
//!
//! # Features
//!
//! - **Zero-copy decoding** - Read directly from network buffers
//! - **SIMD-aligned buffers** - Optimized memory access patterns
//! - **Type-safe message handling** - Compile-time message validation
//! - **Group iteration** - Efficient handling of repeating groups
//! - **Variable-length data** - Support for varchar fields
//! - **High performance** - Minimal allocations and overhead
//!
//! # Example
//!
//! ```rust,ignore
//! use rustysbe::{SbeDecoder, SbeEncoder, SbeMessage};
//!
//! // Encode a message
//! let mut encoder = SbeEncoder::new(1, 0, 64);
//! encoder.write_u64(0, 12345)?;
//! encoder.write_string(8, 16, "BTCUSDT")?;
//! let message = encoder.finalize()?;
//!
//! // Decode the message
//! let decoder = SbeDecoder::new(&message)?;
//! let value = decoder.read_u64(0)?;
//! let symbol = decoder.read_string(8, 16)?;
//! ```

pub mod buffer;
pub mod codegen;
pub mod decoder;
pub mod encoder;
pub mod error;
pub mod message;

// Re-export commonly used types for convenience
pub use buffer::{SbeBuffer, SbeReader};
pub use decoder::{SbeDecoder, SbeGroupElement, SbeGroupIterator, SbeHeader, SbeVariableData};
pub use encoder::{GroupElementEncoder, GroupEncoderBuilder, SbeEncoder};
pub use error::{SbeError, SbeResult};
pub use message::{
    SbeMessage, SbeMessageDecoder, SbeMessageEncoder, SbeMessageHeader, SbeMessageMetadata,
    SbeMessageRegistry,
};

/// Generated SBE message types from schema
pub mod generated {
    #![allow(clippy::all)]
    #![allow(missing_docs)]
    #![allow(non_snake_case)]
    #![allow(unused)]

    include!(concat!(env!("OUT_DIR"), "/sbe.rs"));
}

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::{
        SbeBuffer, SbeDecoder, SbeEncoder, SbeError, SbeMessage, SbeMessageDecoder,
        SbeMessageEncoder, SbeMessageHeader, SbeMessageRegistry, SbeReader, SbeResult,
    };
}

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// SBE specification version supported
pub const SBE_VERSION: &str = "2.0";

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_basic_round_trip() {
        // Test basic encoding/decoding functionality
        let mut encoder = SbeEncoder::new(1, 0, 32);

        // Write test data
        encoder.write_u64(0, 1234567890).unwrap();
        encoder.write_u32(8, 42).unwrap();
        encoder.write_string(12, 16, "TEST_STRING").unwrap();
        encoder.write_f32(28, std::f32::consts::PI).unwrap();

        let message = encoder.finalize().unwrap();

        // Decode and verify
        let decoder = SbeDecoder::new(&message).unwrap();
        assert_eq!(decoder.template_id(), 1);
        assert_eq!(decoder.schema_version(), 0);
        assert_eq!(decoder.read_u64(0).unwrap(), 1234567890);
        assert_eq!(decoder.read_u32(8).unwrap(), 42);
        assert_eq!(
            decoder.read_string(12, 16).unwrap().trim_end_matches('\0'),
            "TEST_STRING"
        );
        assert!((decoder.read_f32(28).unwrap() - std::f32::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_variable_data() {
        let mut encoder = SbeEncoder::new(2, 0, 8);

        // Fixed field
        encoder.write_u64(0, 999).unwrap();

        // Variable data
        encoder.write_variable_string("Hello").unwrap();
        encoder.write_variable_string("World").unwrap();
        encoder.write_variable_bytes(b"Binary data").unwrap();

        let message = encoder.finalize().unwrap();

        // Verify fixed field
        let decoder = SbeDecoder::new(&message).unwrap();
        assert_eq!(decoder.read_u64(0).unwrap(), 999);

        // Variable data would be processed by generated code
        assert!(message.len() > 8 + 8); // Header + fixed field + variable data
    }

    #[test]
    fn test_header_utilities() {
        let mut encoder = SbeEncoder::new(123, 5, 16);
        encoder.write_u64(0, 42).unwrap();
        encoder.write_u64(8, 84).unwrap();
        let message = encoder.finalize().unwrap();

        // Test header extraction
        let template_id = SbeMessageHeader::extract_template_id(&message).unwrap();
        let schema_version = SbeMessageHeader::extract_schema_version(&message).unwrap();
        let length = SbeMessageHeader::extract_message_length(&message).unwrap();

        assert_eq!(template_id, 123);
        assert_eq!(schema_version, 5);
        assert_eq!(length, message.len() as u32);

        // Test validation
        let (len, tid, sv) = SbeMessageHeader::validate_basic(&message).unwrap();
        assert_eq!(len, message.len() as u32);
        assert_eq!(tid, 123);
        assert_eq!(sv, 5);
    }

    #[test]
    fn test_error_handling() {
        // Test buffer too small
        let small_buffer = [1, 2, 3];
        assert!(SbeDecoder::new(&small_buffer).is_err());

        // Test invalid template ID extraction
        let invalid_header = [0, 0, 0, 0]; // Too small
        assert!(SbeMessageHeader::extract_template_id(&invalid_header).is_err());

        // Test field offset out of bounds
        let mut encoder = SbeEncoder::new(1, 0, 8);
        assert!(encoder.write_u64(4, 123).is_err()); // Would overlap boundary
    }
}
