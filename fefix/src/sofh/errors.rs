use std::fmt;
use std::io;

/// The type returned in the event of an error when decoding SOFH-enclosed
/// messages.
#[derive(Debug)]
pub enum DecodeError {
    /// The provided message length is outside the legal range.
    InvalidMessageLength,
    /// I/O-related error.
    Io(io::Error),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::Io(err) => {
                writeln!(f, "I/O error while reading the message.")?;
                (*err).fmt(f)
            }
            DecodeError::InvalidMessageLength => {
                writeln!(f, "Message length must be greater than or equal to 6.",)
            }
        }
    }
}

impl From<io::Error> for DecodeError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

/// The type returned in the event of an error when encoding SOFH-enclosed
/// messages.
#[derive(Debug, Clone)]
pub enum EncodeError {
    /// The assigned payload is too big to fit in a single SOFH-enclosed
    /// message.
    TooLong,
}
