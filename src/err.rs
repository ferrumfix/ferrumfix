use failure::Fail;
use std::{io, str};

/// Shorthand for a `Result` with the error type `serde_json::Error`.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "I/O error.")]
    Io { err: io::Error },
    #[fail(display = "Utf8 encoding error.")]
    Utf8 { err: str::Utf8Error },
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io { err }
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        Error::Utf8 { err }
    }
}
