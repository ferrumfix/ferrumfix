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
    #[error("Field {0} is required but not present.")]
    RequiredFieldMissing(TagU32),
    /// A field has an invalid value.
    #[error("Invalid value for field {0}.")]
    InvalidFieldValue(TagU32),
    /// The message is malformed.
    #[error("Invalid message structure.")]
    InvalidMessage,
}

/// A simple [`Validator`] that checks for field presence and correctness.
#[derive(Debug, Default, Copy, Clone)]
pub struct SimpleValidator {}

impl Validator for SimpleValidator {
    fn validate<T>(&self, msg: &Message<T>, dict: &Dictionary) -> Result<(), ValidationError> {
        let msg_type = msg
            .msg_type()
            .map_err(|_| ValidationError::InvalidMessage)?;
        let message_spec = dict
            .message_by_msgtype(msg_type.as_str())
            .ok_or(ValidationError::InvalidMessage)?;

        for item in message_spec.layout() {
            if item.required() {
                if let crate::dict::LayoutItemKind::Field(field_spec) = item.kind() {
                    if msg.get_raw(field_spec.tag().into()).is_none() {
                        return Err(ValidationError::RequiredFieldMissing(field_spec.tag()));
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::fix44;
    use crate::tagvalue::Decoder;

    #[test]
    fn test_missing_required_field() {
        let validator = SimpleValidator::default();
        let dict = Dictionary::fix44().unwrap();
        let mut decoder = Decoder::new(dict.clone());

        // A valid NewOrderSingle message.
        let msg = "8=FIX.4.4|9=76|35=D|49=A|56=B|34=12|52=20100304-07:59:30|11=13346|21=1|38=100|40=2|54=1|59=0|60=20100304-07:59:30|10=139|";
        let mut message = decoder.decode(msg.as_bytes()).unwrap();
        assert!(validator.validate(&message, &dict).is_ok());

        // NewOrderSingle missing HandlInst (21).
        message.remove(21);
        let validation_result = validator.validate(&message, &dict);
        assert_eq!(
            validation_result,
            Err(ValidationError::RequiredFieldMissing(
                TagU32::new(fix44::HANDL_INST.tag).unwrap()
            ))
        );
    }
}
