//! Error types for Protocol Buffers FIX encoding/decoding.

use std::io;
use thiserror::Error;

/// Main error type for GPB operations.
#[derive(Error, Debug)]
pub enum GpbError {
    /// Encoding operation failed
    #[error("Encoding failed: {0}")]
    Encode(#[from] EncodeError),

    /// Decoding operation failed
    #[error("Decoding failed: {0}")]
    Decode(#[from] DecodeError),

    /// I/O operation failed
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Invalid message format
    #[error("Invalid message format: {0}")]
    InvalidFormat(String),
}

/// Errors during Protocol Buffers encoding.
#[derive(Error, Debug)]
pub enum EncodeError {
    /// Buffer too small for encoded message
    #[error("Buffer too small: need {needed} bytes, have {available}")]
    BufferTooSmall {
        /// Bytes needed
        needed: usize,
        /// Bytes available
        available: usize,
    },

    /// Field value is invalid
    #[error("Invalid field value: field {field_id}, reason: {reason}")]
    InvalidFieldValue {
        /// FIX field ID
        field_id: u32,
        /// Reason for invalidity
        reason: String,
    },

    /// Required field is missing
    #[error("Missing required field: {field_id}")]
    MissingRequiredField {
        /// FIX field ID
        field_id: u32,
    },

    /// Message type not supported
    #[error("Unsupported message type: {message_type}")]
    UnsupportedMessageType {
        /// Message type identifier
        message_type: String,
    },

    /// Internal prost encoding error
    #[error("Prost encoding error: {0}")]
    Prost(#[from] prost::EncodeError),
}

/// Errors during Protocol Buffers decoding.
#[derive(Error, Debug)]
pub enum DecodeError {
    /// Input buffer is truncated
    #[error("Truncated buffer: expected at least {expected} bytes, got {actual}")]
    TruncatedBuffer {
        /// Expected minimum bytes
        expected: usize,
        /// Actual bytes available
        actual: usize,
    },

    /// Invalid wire format
    #[error("Invalid wire format: {reason}")]
    InvalidWireFormat {
        /// Reason for invalidity
        reason: String,
    },

    /// Unknown field encountered
    #[error("Unknown field: tag {tag}")]
    UnknownField {
        /// Field tag number
        tag: u32,
    },

    /// Field value validation failed
    #[error("Field validation failed: field {field_id}, value: {value:?}")]
    FieldValidationFailed {
        /// FIX field ID
        field_id: u32,
        /// Invalid value
        value: String,
    },

    /// Message checksum mismatch
    #[error("Checksum mismatch: expected {expected}, calculated {actual}")]
    ChecksumMismatch {
        /// Expected checksum
        expected: u32,
        /// Calculated checksum
        actual: u32,
    },

    /// Internal prost decoding error
    #[error("Prost decoding error: {0}")]
    Prost(#[from] prost::DecodeError),
}

impl GpbError {
    /// Check if error is recoverable (retryable)
    pub fn is_recoverable(&self) -> bool {
        match self {
            GpbError::Encode(EncodeError::BufferTooSmall { .. }) => true,
            GpbError::Decode(DecodeError::TruncatedBuffer { .. }) => true,
            GpbError::Io(_) => true,
            _ => false,
        }
    }

    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            GpbError::Encode(EncodeError::InvalidFieldValue { .. }) => ErrorSeverity::High,
            GpbError::Encode(EncodeError::MissingRequiredField { .. }) => ErrorSeverity::High,
            GpbError::Decode(DecodeError::ChecksumMismatch { .. }) => ErrorSeverity::Critical,
            GpbError::Decode(DecodeError::InvalidWireFormat { .. }) => ErrorSeverity::High,
            GpbError::Decode(DecodeError::TruncatedBuffer { .. }) => ErrorSeverity::Medium,
            GpbError::Encode(EncodeError::BufferTooSmall { .. }) => ErrorSeverity::Low,
            _ => ErrorSeverity::Medium,
        }
    }
}

/// Error severity levels for monitoring and alerting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Low severity - usually recoverable
    Low,
    /// Medium severity - requires attention
    Medium,
    /// High severity - significant issue
    High,
    /// Critical severity - immediate action required
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_recoverability() {
        let recoverable = GpbError::Encode(EncodeError::BufferTooSmall {
            needed: 100,
            available: 50,
        });
        assert!(recoverable.is_recoverable());

        let non_recoverable = GpbError::Encode(EncodeError::MissingRequiredField { field_id: 35 });
        assert!(!non_recoverable.is_recoverable());
    }

    #[test]
    fn test_error_severity() {
        let critical = GpbError::Decode(DecodeError::ChecksumMismatch {
            expected: 123,
            actual: 456,
        });
        assert_eq!(critical.severity(), ErrorSeverity::Critical);

        let low = GpbError::Encode(EncodeError::BufferTooSmall {
            needed: 100,
            available: 50,
        });
        assert_eq!(low.severity(), ErrorSeverity::Low);
    }
}
