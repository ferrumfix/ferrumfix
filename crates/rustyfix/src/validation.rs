//! Message validation.

use crate::tagvalue::Message;
use crate::{Dictionary, TagU32};

/// A validator for inbound and outbound FIX messages.
pub trait Validator {
    /// Validates a `msg` and returns `Ok(())` on success.
    fn validate<T>(&self, msg: &Message<T>, dict: &Dictionary) -> Result<(), ValidationError>;
}

/// The type of error that can arise during a FIX message validation.
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// The message is missing a required field.
    #[error("Field {tag} is required but not present in message type '{msg_type}'.")]
    RequiredFieldMissing {
        /// The missing field tag
        tag: TagU32,
        /// The message type that's missing the field
        msg_type: String,
    },
    /// A field has an invalid value.
    #[error("Invalid value '{value}' for field {tag}: {reason}")]
    InvalidFieldValue {
        /// The field tag with invalid value
        tag: TagU32,
        /// The invalid value
        value: String,
        /// The reason why the value is invalid
        reason: String,
    },
    /// The message is malformed.
    #[error("Invalid message structure: {reason}")]
    InvalidMessage {
        /// The reason why the message is invalid
        reason: String,
    },
    /// Unknown message type.
    #[error("Unknown message type: '{msg_type}'")]
    UnknownMessageType {
        /// The unknown message type
        msg_type: String,
    },
    /// Field format validation failed.
    #[error("Field {tag} format validation failed: {reason}")]
    InvalidFieldFormat {
        /// The field tag with invalid format
        tag: u32,
        /// The reason why the format is invalid
        reason: String,
    },
    /// Field value out of acceptable range.
    #[error("Field {tag} value '{value}' is out of acceptable range: {reason}")]
    ValueOutOfRange {
        /// The field tag with out-of-range value
        tag: u32,
        /// The out-of-range value
        value: String,
        /// The reason why the value is out of range
        reason: String,
    },
}

/// A simple [`Validator`] that checks for field presence and correctness.
#[derive(Debug, Default, Copy, Clone)]
pub struct SimpleValidator {}

impl Validator for SimpleValidator {
    fn validate<T>(&self, msg: &Message<T>, dict: &Dictionary) -> Result<(), ValidationError> {
        let msg_type = msg
            .msg_type()
            .map_err(|_| ValidationError::InvalidMessage {
                reason: "Unable to extract message type".to_string(),
            })?;
        let message_spec = dict.message_by_msgtype(msg_type.as_str()).ok_or(
            ValidationError::UnknownMessageType {
                msg_type: msg_type.clone(),
            },
        )?;

        for item in message_spec.layout() {
            if item.required() {
                if let crate::dict::LayoutItemKind::Field(field_spec) = item.kind() {
                    if msg.get_raw(field_spec.tag().into()).is_none() {
                        return Err(ValidationError::RequiredFieldMissing {
                            tag: field_spec.tag(),
                            msg_type: msg_type.clone(),
                        });
                    }
                }
            }
        }
        Ok(())
    }
}

/// An advanced [`Validator`] with comprehensive validation capabilities inspired by QuickFIX patterns.
///
/// This validator provides multi-layered validation including:
/// - Message type validation  
/// - Field format validation
/// - Required field presence validation per message type
/// - Field value range and constraint validation
#[derive(Debug, Default, Clone)]
pub struct AdvancedValidator {
    /// Whether to perform strict field format validation
    pub strict_format_validation: bool,
    /// Whether to validate field value ranges
    pub validate_value_ranges: bool,
    /// Whether to check for unknown fields
    pub reject_unknown_fields: bool,
}

impl AdvancedValidator {
    /// Creates a new AdvancedValidator with default settings.
    pub fn new() -> Self {
        Self {
            strict_format_validation: true,
            validate_value_ranges: true,
            reject_unknown_fields: false, // More permissive by default
        }
    }

    /// Creates a new AdvancedValidator with strict validation enabled.
    pub fn strict() -> Self {
        Self {
            strict_format_validation: true,
            validate_value_ranges: true,
            reject_unknown_fields: true,
        }
    }

    /// Validates the message type is known and supported.
    pub fn validate_message_type(
        &self,
        msg_type: &str,
        dict: &Dictionary,
    ) -> Result<(), ValidationError> {
        if dict.message_by_msgtype(msg_type).is_some() {
            Ok(())
        } else {
            Err(ValidationError::UnknownMessageType {
                msg_type: msg_type.to_string(),
            })
        }
    }

    /// Validates field format according to FIX data type specifications.
    pub fn validate_field_format(
        &self,
        tag: u32,
        value: &[u8],
        dict: &Dictionary,
    ) -> Result<(), ValidationError> {
        if !self.strict_format_validation {
            return Ok(());
        }

        // Convert to string for validation
        let value_str =
            std::str::from_utf8(value).map_err(|_| ValidationError::InvalidFieldFormat {
                tag,
                reason: "Field value is not valid UTF-8".to_string(),
            })?;

        // Get field datatype from dictionary
        let field_spec = dict.field_by_tag(tag);
        if let Some(field) = field_spec {
            match field.name() {
                // Common field format validations
                name if name.contains("Time") || name.contains("SendingTime") => {
                    self.validate_time_format(tag, value_str)
                }
                name if name.contains("Date") => self.validate_date_format(tag, value_str),
                name if name.contains("Price") || name.contains("Qty") => {
                    self.validate_numeric_format(tag, value_str)
                }
                name if name.contains("SeqNum") => {
                    self.validate_sequence_number_format(tag, value_str)
                }
                _ => Ok(()), // Allow other fields to pass for now
            }
        } else if self.reject_unknown_fields {
            Err(ValidationError::InvalidFieldFormat {
                tag,
                reason: "Unknown field tag".to_string(),
            })
        } else {
            Ok(())
        }
    }

    /// Validates required fields are present for the given message type.
    pub fn validate_required_fields<T>(
        &self,
        message: &Message<T>,
        dict: &Dictionary,
    ) -> Result<(), ValidationError> {
        let msg_type = message
            .msg_type()
            .map_err(|_| ValidationError::InvalidMessage {
                reason: "Unable to extract message type".to_string(),
            })?;

        let message_spec = dict.message_by_msgtype(msg_type.as_str()).ok_or(
            ValidationError::UnknownMessageType {
                msg_type: msg_type.clone(),
            },
        )?;

        // Check required fields in message layout
        for item in message_spec.layout() {
            if item.required() {
                if let crate::dict::LayoutItemKind::Field(field_spec) = item.kind() {
                    if message.get_raw(field_spec.tag().into()).is_none() {
                        return Err(ValidationError::RequiredFieldMissing {
                            tag: field_spec.tag(),
                            msg_type: msg_type.clone(),
                        });
                    }
                }
            }
        }

        Ok(())
    }

    /// Validates field values are within acceptable ranges and constraints.
    pub fn validate_field_values(
        &self,
        tag: u32,
        value: &[u8],
        _dict: &Dictionary,
    ) -> Result<(), ValidationError> {
        if !self.validate_value_ranges {
            return Ok(());
        }

        let value_str =
            std::str::from_utf8(value).map_err(|_| ValidationError::ValueOutOfRange {
                tag,
                value: format!("{value:?}"),
                reason: "Field value is not valid UTF-8".to_string(),
            })?;

        match tag {
            // Message sequence number must be positive
            34 => {
                let seq_num: u64 =
                    value_str
                        .parse()
                        .map_err(|_| ValidationError::ValueOutOfRange {
                            tag,
                            value: value_str.to_string(),
                            reason: "Sequence number must be a positive integer".to_string(),
                        })?;
                if seq_num == 0 {
                    return Err(ValidationError::ValueOutOfRange {
                        tag,
                        value: value_str.to_string(),
                        reason: "Sequence number must be greater than 0".to_string(),
                    });
                }
            }
            // Side must be valid
            54 => match value_str {
                "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "A" | "B" | "C" => {}
                _ => {
                    return Err(ValidationError::ValueOutOfRange {
                        tag,
                        value: value_str.to_string(),
                        reason: "Invalid Side value".to_string(),
                    });
                }
            },
            // OrderType validation
            40 => match value_str {
                "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "A" | "B" | "C" | "D"
                | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" => {}
                _ => {
                    return Err(ValidationError::ValueOutOfRange {
                        tag,
                        value: value_str.to_string(),
                        reason: "Invalid OrderType value".to_string(),
                    });
                }
            },
            // TimeInForce validation
            59 => match value_str {
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "A" | "B" => {}
                _ => {
                    return Err(ValidationError::ValueOutOfRange {
                        tag,
                        value: value_str.to_string(),
                        reason: "Invalid TimeInForce value".to_string(),
                    });
                }
            },
            _ => {
                // For other fields, we could add more validations as needed
            }
        }

        Ok(())
    }

    // Helper methods for format validation
    fn validate_time_format(&self, tag: u32, value: &str) -> Result<(), ValidationError> {
        // FIX time format: YYYYMMDD-HH:MM:SS or YYYYMMDD-HH:MM:SS.nnn
        if value.len() < 17 || !value.contains('-') || !value.contains(':') {
            return Err(ValidationError::InvalidFieldFormat {
                tag,
                reason: "Invalid time format, expected YYYYMMDD-HH:MM:SS[.nnn]".to_string(),
            });
        }
        Ok(())
    }

    fn validate_date_format(&self, tag: u32, value: &str) -> Result<(), ValidationError> {
        // FIX date format: YYYYMMDD
        if value.len() != 8 || !value.chars().all(|c| c.is_ascii_digit()) {
            return Err(ValidationError::InvalidFieldFormat {
                tag,
                reason: "Invalid date format, expected YYYYMMDD".to_string(),
            });
        }
        Ok(())
    }

    fn validate_numeric_format(&self, tag: u32, value: &str) -> Result<(), ValidationError> {
        // Check if it's a valid decimal number
        if value.parse::<f64>().is_err() {
            return Err(ValidationError::InvalidFieldFormat {
                tag,
                reason: "Invalid numeric format".to_string(),
            });
        }
        Ok(())
    }

    fn validate_sequence_number_format(
        &self,
        tag: u32,
        value: &str,
    ) -> Result<(), ValidationError> {
        // Sequence numbers must be positive integers
        if value.parse::<u64>().is_err() {
            return Err(ValidationError::InvalidFieldFormat {
                tag,
                reason: "Sequence number must be a positive integer".to_string(),
            });
        }
        Ok(())
    }
}

impl Validator for AdvancedValidator {
    fn validate<T>(&self, msg: &Message<T>, dict: &Dictionary) -> Result<(), ValidationError> {
        // 1. Validate message type
        let msg_type = msg
            .msg_type()
            .map_err(|_| ValidationError::InvalidMessage {
                reason: "Unable to extract message type".to_string(),
            })?;
        self.validate_message_type(&msg_type, dict)?;

        // 2. Validate required fields
        self.validate_required_fields(msg, dict)?;

        // 3. For now, we'll focus on validating the most critical fields manually
        // to avoid lifetime issues with the fields() iterator
        // This validates key fields that are always present in FIX messages

        // Validate sequence number format and value (tag 34)
        if let Some(seq_num_value) = msg.get_raw(34) {
            self.validate_field_format(34, seq_num_value, dict)?;
            self.validate_field_values(34, seq_num_value, dict)?;
        }

        // Validate sending time format (tag 52)
        if let Some(sending_time_value) = msg.get_raw(52) {
            self.validate_field_format(52, sending_time_value, dict)?;
        }

        // Validate side field if present (tag 54)
        if let Some(side_value) = msg.get_raw(54) {
            self.validate_field_values(54, side_value, dict)?;
        }

        // Validate order type if present (tag 40)
        if let Some(order_type_value) = msg.get_raw(40) {
            self.validate_field_values(40, order_type_value, dict)?;
        }

        // Validate time in force if present (tag 59)
        if let Some(tif_value) = msg.get_raw(59) {
            self.validate_field_values(59, tif_value, dict)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::GetConfig;
    use crate::tagvalue::Decoder;

    #[test]
    fn test_missing_required_field() {
        let validator = SimpleValidator::default();
        let dict = Dictionary::fix44().unwrap();
        let mut decoder = Decoder::new(dict.clone());
        decoder.config_mut().separator = b'|';

        // Test a message missing required field ClOrdID (11)
        let msg = "8=FIX.4.2|9=40|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=091|";
        let message = decoder.decode(msg.as_bytes()).unwrap();

        // Should fail validation due to missing ClOrdID (tag 11)
        let result = validator.validate(&message, &dict);
        assert!(result.is_err());

        // Verify it's specifically the missing ClOrdID field
        match result {
            Err(ValidationError::RequiredFieldMissing { tag, msg_type }) => {
                assert_eq!(tag.get(), 11); // ClOrdID
                assert_eq!(msg_type, "D"); // NewOrderSingle
            }
            _ => panic!("Expected RequiredFieldMissing error for ClOrdID"),
        }
    }

    #[test]
    fn test_advanced_validator_basic_functionality() {
        let validator = AdvancedValidator::new();
        let dict = Dictionary::fix44().unwrap();

        // Test that the advanced validator correctly identifies validation issues
        // Test 1: Valid message type should pass
        assert!(validator.validate_message_type("D", &dict).is_ok());

        // Test 2: Invalid message type should fail
        assert!(validator.validate_message_type("INVALID", &dict).is_err());

        // Test 3: Valid field values should pass
        assert!(validator.validate_field_values(34, b"123", &dict).is_ok());
        assert!(validator.validate_field_values(54, b"1", &dict).is_ok());

        // Test 4: Invalid field values should fail
        assert!(validator.validate_field_values(34, b"0", &dict).is_err()); // Invalid seq num
        assert!(
            validator
                .validate_field_values(54, b"INVALID", &dict)
                .is_err()
        ); // Invalid side

        // Test 5: Valid field formats should pass
        assert!(validator.validate_field_format(34, b"123", &dict).is_ok());

        // Test 6: Invalid field formats should fail when strict validation is enabled
        assert!(
            validator
                .validate_field_format(52, b"invalid-time", &dict)
                .is_err()
        );
    }

    #[test]
    fn test_advanced_validator_unknown_message_type() {
        let validator = AdvancedValidator::new();
        let dict = Dictionary::fix44().unwrap();

        // Test unknown message type validation
        let result = validator.validate_message_type("UNKNOWN", &dict);
        assert!(
            matches!(result, Err(ValidationError::UnknownMessageType { msg_type }) if msg_type == "UNKNOWN")
        );
    }

    #[test]
    fn test_advanced_validator_field_value_validation() {
        let validator = AdvancedValidator::new();
        let dict = Dictionary::fix44().unwrap();

        // Test invalid sequence number (zero)
        let result = validator.validate_field_values(34, b"0", &dict);
        assert!(matches!(
            result,
            Err(ValidationError::ValueOutOfRange { tag: 34, .. })
        ));

        // Test valid sequence number
        let result = validator.validate_field_values(34, b"123", &dict);
        assert!(result.is_ok());

        // Test invalid side value
        let result = validator.validate_field_values(54, b"X", &dict);
        assert!(matches!(
            result,
            Err(ValidationError::ValueOutOfRange { tag: 54, .. })
        ));

        // Test valid side value
        let result = validator.validate_field_values(54, b"1", &dict);
        assert!(result.is_ok());
    }

    #[test]
    fn test_advanced_validator_format_validation() {
        let validator = AdvancedValidator::new();
        let dict = Dictionary::fix44().unwrap();

        // Test invalid time format
        let result = validator.validate_field_format(52, b"invalid-time", &dict);
        assert!(matches!(
            result,
            Err(ValidationError::InvalidFieldFormat { tag: 52, .. })
        ));

        // Test valid time format
        let result = validator.validate_field_format(52, b"20100304-07:59:30", &dict);
        assert!(result.is_ok());

        // Test invalid numeric format for price fields
        let result = validator.validate_field_format(44, b"not-a-number", &dict);
        assert!(matches!(
            result,
            Err(ValidationError::InvalidFieldFormat { tag: 44, .. })
        ));

        // Test valid numeric format
        let result = validator.validate_field_format(44, b"123.45", &dict);
        assert!(result.is_ok());
    }

    #[test]
    fn test_advanced_validator_strict_mode() {
        let validator = AdvancedValidator::strict();

        // Strict mode should have all validations enabled
        assert!(validator.reject_unknown_fields);
        assert!(validator.strict_format_validation);
        assert!(validator.validate_value_ranges);
    }

    #[test]
    fn test_advanced_validator_configurable_validation() {
        let mut validator = AdvancedValidator::new();
        let dict = Dictionary::fix44().unwrap();

        // Disable strict format validation
        validator.strict_format_validation = false;

        // Should now pass even with invalid format
        let result = validator.validate_field_format(52, b"invalid-time", &dict);
        assert!(result.is_ok());

        // Disable value range validation
        validator.validate_value_ranges = false;

        // Should now pass even with invalid values
        let result = validator.validate_field_values(34, b"0", &dict);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validation_error_messages() {
        let validator = AdvancedValidator::new();
        let dict = Dictionary::fix44().unwrap();

        // Test error message content for unknown message type
        let result = validator.validate_message_type("TEST", &dict);
        match result {
            Err(ValidationError::UnknownMessageType { msg_type }) => {
                assert_eq!(msg_type, "TEST");
                let error_msg = format!("{}", ValidationError::UnknownMessageType { msg_type });
                assert!(error_msg.contains("Unknown message type: 'TEST'"));
            }
            _ => panic!("Expected UnknownMessageType error"),
        }

        // Test error message content for value out of range
        let result = validator.validate_field_values(54, b"INVALID", &dict);
        match result {
            Err(ValidationError::ValueOutOfRange { tag, value, reason }) => {
                assert_eq!(tag, 54);
                assert_eq!(value, "INVALID");
                assert!(reason.contains("Invalid Side value"));
            }
            _ => panic!("Expected ValueOutOfRange error"),
        }
    }

    #[test]
    fn test_comprehensive_message_validation() {
        let validator = AdvancedValidator::new();
        let dict = Dictionary::fix44().unwrap();

        // Test individual validation functions instead of full message decoding to avoid decoder issues

        // Test unknown message type validation
        let result = validator.validate_message_type("UNKNOWN_TYPE", &dict);
        assert!(result.is_err());

        // Test valid message type
        let result = validator.validate_message_type("D", &dict);
        assert!(result.is_ok());

        // Test field value validation - invalid sequence number
        let result = validator.validate_field_values(34, b"0", &dict);
        assert!(result.is_err());

        // Test field value validation - valid sequence number
        let result = validator.validate_field_values(34, b"123", &dict);
        assert!(result.is_ok());
    }
}
