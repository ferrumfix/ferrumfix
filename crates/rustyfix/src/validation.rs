use crate::tagvalue::Message;
use rustyfix_dictionary::Dictionary;

pub trait Validator {
    fn validate<T>(&self, msg: &Message<T>, dict: &Dictionary) -> Result<(), ValidationError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("dummy")]
    Dummy,
}
