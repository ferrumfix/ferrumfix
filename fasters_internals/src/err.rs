use failure::Fail;
use std::{io, str};

/// Shorthand for a `Result` with the error type [`Error`](enum.Error.html).
pub type Result<T> = std::result::Result<T, Error>;

/// Errors encountered by Fasters. Shared across FIX and FAST modules.
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "I/O error.")]
    Io { err: io::Error },
    #[fail(display = "Utf8 encoding error.")]
    Utf8 { err: str::Utf8Error },
    /// Invalid XML document.
    #[fail(display = "XML syntax error.")]
    XmlSyntax { err: quick_xml::Error },
    /// The given XML document is not valid FIXML.
    #[fail(display = "Malformed FIXML.")]
    BadFixml,
    /// Represents parsing errors for the compact notations of both FIX and FAST.
    #[fail(display = "Malformed compact notation.")]
    BadCompactNotation,
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

impl From<quick_xml::Error> for Error {
    fn from(err: quick_xml::Error) -> Self {
        Error::XmlSyntax { err }
    }
}
