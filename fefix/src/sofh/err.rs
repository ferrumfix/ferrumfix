use std::fmt;
use std::io;

/// The type returned in the event of an error when decoding SOFH-enclosed
/// messages.
#[derive(Debug)]
pub enum Error {
    /// The provided message length is outside the legal range.
    InvalidMessageLength,
    /// The given message is incomplete.
    Incomplete {
        /// The number of missing bytes to complete the SOFH-enclosed message.
        needed: usize,
    },
    /// I/O-related error.
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => {
                writeln!(f, "I/O error while reading the message.")?;
                (*err).fmt(f)
            }
            Error::Incomplete { needed } => {
                writeln!(
                    f,
                    "The message is incomplete. {} more bytes are needed.",
                    needed
                )
            }
            Error::InvalidMessageLength => {
                writeln!(f, "Message length must be greater than or equal to 6.",)
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
