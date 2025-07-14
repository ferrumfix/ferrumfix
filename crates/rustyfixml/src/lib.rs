//! # RustyFIXML - XML-based FIX Message Encoding
//!
//! High-performance FIXML (Financial Information eXchange Markup Language)
//! encoding/decoding for FIX messages optimized for trading systems.
//!
//! ## Features
//!
//! - Complete FIXML 1.1 specification support
//! - High-performance XML parsing with quick-xml
//! - Zero-copy string processing where possible
//! - Streaming XML processing for large messages
//! - Comprehensive message validation
//! - Schema-aware encoding/decoding
//!
//! ## Example
//!
//! ```rust
//! use rustyfixml::{FixmlEncoder, FixmlDecoder, FixmlMessage};
//!
//! let encoder = FixmlEncoder::new();
//! let message = FixmlMessage::new_order_single(/* fields */);
//! let xml = encoder.encode(&message)?;
//!
//! let decoder = FixmlDecoder::new();
//! let decoded = decoder.decode(&xml)?;
//! ```

#![doc(html_root_url = "https://docs.rs/rustyfixml/")]
#![warn(rustdoc::missing_doc_code_examples)]
#![allow(unused, missing_docs, dead_code)]
#![deny(
    unsafe_op_in_unsafe_fn,
    rustdoc::broken_intra_doc_links,
    unconditional_recursion,
    unstable_name_collisions,
    clippy::useless_conversion,
    clippy::missing_panics_doc,
    clippy::mixed_case_hex_literals,
    clippy::needless_bool,
    clippy::needless_lifetimes
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod decoder;
mod encoder;
mod error;
mod message;
mod schema;

pub use decoder::{DecodeConfig, FixmlDecoder};
pub use encoder::{EncodeConfig, FixmlEncoder};
pub use error::{DecodeError, EncodeError, FixmlError};
pub use message::{FieldValue, FixmlMessage, MessageType};
pub use schema::{FixmlSchema, SchemaVersion};

// Re-export quick-xml types for convenience
pub use quick_xml::{events::Event, Reader, Writer};

/// Prelude module for common imports
pub mod prelude {
    pub use crate::{
        DecodeError, EncodeError, FieldValue, FixmlDecoder, FixmlEncoder, FixmlError, FixmlMessage,
        FixmlSchema, MessageType, SchemaVersion,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // TDD test - this will fail until we implement the types
        let encoder = FixmlEncoder::new();
        let decoder = FixmlDecoder::new();

        let message = FixmlMessage::new_order_single("BTCUSD".into(), 50000.0, 1.5, "1".into());

        let xml = encoder.encode(&message).expect("encoding should work");
        let decoded = decoder.decode(&xml).expect("decoding should work");

        assert_eq!(message, decoded);
    }

    #[test]
    fn test_xml_format_compliance() {
        let encoder = FixmlEncoder::new();
        let message = FixmlMessage::new_order_single("ETHUSD".into(), 3000.0, 2.0, "2".into());

        let xml = encoder.encode(&message).expect("encoding should work");

        // Verify XML structure
        assert!(xml.contains("<?xml"));
        assert!(xml.contains("<FIXML"));
        assert!(xml.contains("</FIXML>"));
        assert!(xml.contains("xmlns"));
    }

    #[test]
    fn test_message_validation() {
        let mut message = FixmlMessage::new(MessageType::NewOrderSingle);

        // Should fail validation - missing required fields
        assert!(message.validate().is_err());

        // Add required fields
        message.set_field("Symbol", FieldValue::String("BTCUSD".to_string()));
        message.set_field("Side", FieldValue::String("1".to_string()));
        message.set_field("OrderQty", FieldValue::Decimal(1.5));
        message.set_field("OrdType", FieldValue::String("2".to_string()));

        // Should pass validation
        assert!(message.validate().is_ok());
    }
}
