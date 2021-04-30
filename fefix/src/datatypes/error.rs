//! Error types for [`DataType`](super::DataType) implementors.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decimal {
    NotUtf8,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bool {
    WrongLength,
    InvalidCharacter,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Int {
    InvalidUtf8,
    Other,
}

/// Type error for [`CheckSum`](super::CheckSum).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckSum {
    WrongLength,
    NotAsciiDigits,
}

/// Error type for [`MonthYear`](super::MonthYear).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MonthYear {
    Other,
}

/// Error type for [`Time`](super::Time).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Time {
    Other,
}

/// Error type for [`Timestamp`](super::Timestamp).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Timestamp {
    Other,
}

/// Error type for [`Date`](super::Date).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Date {
    WrongLength,
    NotAsciiDigits,
    OutsideBounds,
}
