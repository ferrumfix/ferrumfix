//! Error types for SBE operations

use thiserror::Error;

/// Errors that can occur during SBE encoding and decoding operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum SbeError {
    /// Buffer is too small for the requested operation
    #[error("Buffer too small: need {need} bytes, have {have}")]
    BufferTooSmall { need: usize, have: usize },

    /// Invalid template ID in message header
    #[error("Invalid template ID: expected {expected}, found {found}")]
    InvalidTemplateId { expected: u16, found: u16 },

    /// Invalid schema version
    #[error("Invalid schema version: expected {expected}, found {found}")]
    InvalidSchemaVersion { expected: u16, found: u16 },

    /// Message length exceeds maximum allowed size
    #[error("Message too large: {length} bytes exceeds maximum {max}")]
    MessageTooLarge { length: usize, max: usize },

    /// Invalid message length in header
    #[error("Invalid message length: {length}")]
    InvalidMessageLength { length: u16 },

    /// Field offset is out of bounds
    #[error("Field offset out of bounds: offset {offset}, message length {length}")]
    FieldOffsetOutOfBounds { offset: usize, length: usize },

    /// Group count exceeds reasonable limits
    #[error("Group count too large: {count}")]
    GroupCountTooLarge { count: u32 },

    /// Invalid group block length
    #[error("Invalid group block length: {length}")]
    InvalidGroupBlockLength { length: u16 },

    /// Variable-length data offset is invalid
    #[error("Invalid variable data offset: {offset}")]
    InvalidVariableDataOffset { offset: usize },

    /// String data is not valid UTF-8
    #[error("Invalid UTF-8 string data")]
    InvalidUtf8String,

    /// Integer overflow during calculations
    #[error("Integer overflow in calculation")]
    IntegerOverflow,

    /// Buffer alignment requirements not met
    #[error("Buffer alignment requirement not met: required {required}, actual {actual}")]
    AlignmentRequirement { required: usize, actual: usize },

    /// Custom error for application-specific cases
    #[error("Custom error: {message}")]
    Custom { message: String },
}

/// Result type for SBE operations
pub type SbeResult<T> = Result<T, SbeError>;

impl SbeError {
    /// Create a custom error with a message
    pub fn custom(message: impl Into<String>) -> Self {
        Self::Custom {
            message: message.into(),
        }
    }

    /// Check if this error indicates a fatal condition that should stop processing
    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            Self::IntegerOverflow
                | Self::InvalidTemplateId { .. }
                | Self::InvalidSchemaVersion { .. }
                | Self::AlignmentRequirement { .. }
        )
    }

    /// Check if this error might be recoverable with more data
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::BufferTooSmall { .. }
                | Self::InvalidMessageLength { .. }
                | Self::FieldOffsetOutOfBounds { .. }
        )
    }
}
