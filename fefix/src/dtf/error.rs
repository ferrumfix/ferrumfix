#[derive(Debug)]
pub enum Decimal {
    NotUtf8,
    Other(rust_decimal::Error),
}

#[derive(Debug)]
pub enum Bool {
    WrongLength,
    InvalidCharacter,
}

#[derive(Debug)]
pub enum Int {
    InvalidUtf8,
    Other,
}

/// Type error for [`CheckSum`](super::CheckSum).
#[derive(Debug)]
pub enum CheckSum {
    WrongLength,
    NotAsciiDigits,
}

/// Error type for [`MonthYear`](super::MonthYear).
#[derive(Debug)]
pub enum MonthYear {
    Other,
}

/// Error type for [`Time`](super::Time).
#[derive(Debug)]
pub enum Time {
    Other,
}

/// Error type for [`Timestamp`](super::Timestamp).
#[derive(Debug)]
pub enum Timestamp {
    Other,
}

/// Error type for [`Date`](super::Date).
#[derive(Debug)]
#[doc(hidden)]
pub enum Date {
    WrongLength,
    NotAsciiDigits,
    OutsideBounds,
}
