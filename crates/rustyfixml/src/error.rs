//! Error types for FIXML encoding/decoding.

use std::string::FromUtf8Error;
use thiserror::Error;

/// Main error type for FIXML operations.
#[derive(Error, Debug)]
pub enum FixmlError {
    /// Encoding operation failed
    #[error("Encoding failed: {0}")]
    Encode(#[from] EncodeError),

    /// Decoding operation failed
    #[error("Decoding failed: {0}")]
    Decode(#[from] DecodeError),

    /// XML processing error
    #[error("XML error: {0}")]
    Xml(#[from] quick_xml::Error),

    /// UTF-8 conversion error
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    /// String conversion error
    #[error("String conversion error: {0}")]
    StringConversion(#[from] FromUtf8Error),

    /// Attribute error
    #[error("Attribute error: {0}")]
    Attribute(#[from] quick_xml::events::attributes::AttrError),
}

/// Errors during FIXML encoding.
#[derive(Error, Debug)]
pub enum EncodeError {
    /// Required field is missing
    #[error("Missing required field: {field_name}")]
    MissingRequiredField { field_name: String },

    /// Invalid field value
    #[error("Invalid field value: {field_name} = {value}")]
    InvalidFieldValue { field_name: String, value: String },

    /// Message type not supported
    #[error("Unsupported message type: {message_type}")]
    UnsupportedMessageType { message_type: String },

    /// XML writing error
    #[error("XML write error: {0}")]
    XmlWrite(String),
}

/// Errors during FIXML decoding.
#[derive(Error, Debug)]
pub enum DecodeError {
    /// Invalid XML structure
    #[error("Invalid XML structure: {reason}")]
    InvalidXmlStructure { reason: String },

    /// Unknown message type
    #[error("Unknown message type: {message_type}")]
    UnknownMessageType { message_type: String },

    /// Field validation failed
    #[error("Field validation failed: {field_name}")]
    FieldValidationFailed { field_name: String },

    /// XML parsing error
    #[error("XML parse error: {0}")]
    XmlParse(String),
}
