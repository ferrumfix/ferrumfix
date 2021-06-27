//! Implementors of [`FixValue`](crate::FixValue).
//!
//! | FIX datatype               | Relevant [`FixValue`] implementors                                                 |
//! |----------------------------|------------------------------------------------------------------------------------|
//! | `int`                      | Any Rust primitive integer type.                                                   |
//! | `Length`                   | [`usize`]                                                                          |
//! | `NumInGroup`               | [`usize`]                                                                          |
//! | `SeqNum`                   | [`u64`]                                                                            |
//! | `TagNum`                   | [`TagU16`](crate::TagU16)                                                          |
//! | `DayOfMonth`               | [`u32`]                                                                            |
//! | `float` and `float` -like  | [`f32`], [`f64`], `rust_decimal::Decimal`, `decimal::d128`, or other custom types. |
//! | `Boolean`                  | [`bool`]                                                                           |
//! | `char`                     | [`u8`] [^1]                                                                        |
//! | `String`                   | `Vec<u8>`, `&[u8]`.[^1]                                                            |
//! | `data`                     | `Vec<u8>`, `&[u8]` (also [`String`], [`str`] for UTF-8 content).                   |
//! | `MultipleCharValue`        | [`MultipleChars`] [^1]                                                             |
//! | `MultipleValueString`      | [`MultipleStrings`] [^1]                                                           |
//! | `Country`                  | [`Country`]                                                                        |
//! | `Currency`                 | [`Currency`]                                                                       |
//! | `Exchange`                 | [`Exchange`]                                                                       |
//! | `month-year`               | [`MonthYear`]                                                                      |
//! | `UTCTimestamp`             | [`Timestamp`]                                                                      |
//! | `LocalMktDate`             | [`Timestamp`]                                                                      |
//! | `UTCTimeOnly`              | [`Time`]                                                                           |
//! | `TZTimestamp`              | [`TzTimestamp`]                                                                    |
//! | `TZTimeOnly`               | [`TzTime`]                                                                         |
//! | `UTCDateOnly`              | [`Date`]                                                                           |
//!
//! # Quick tour of [`FixValue`]
//!
//! ```
//! use fefix::FixValue;
//! use fefix::fix_values::Timestamp;
//!
//! let bytes = b"20130422-12:30:00.000";
//!
//! // You can use `FixValue::deserialize` to parse data fields.
//! let timestamp = Timestamp::deserialize(bytes).unwrap();
//! assert_eq!(timestamp.date().year(), 2013);
//!
//! // `FixValue::deserialize_lossy` is like `FixValue::deserialize`, but it's
//! // allowed to skip some format verification for the sake of speed.
//! assert!(u32::deserialize(b"invalid integer").is_err());
//! assert!(u32::deserialize_lossy(b"invalid integer").is_ok());
//!
//! let buffer: &mut Vec<u8> = &mut vec![];
//! // Use `FixValue::serialize` to write values to buffers.
//! 1337u32.serialize(buffer);
//! assert_eq!(&buffer[..], b"1337" as &[u8]);
//! ```
//!
//! [^1]: With the exception of datatype `data`, FIX mandates a single-byte
//! encoding (Latin alphabet No. 1 by default), while Rust strings are UTF-8,
//! which is a multibyte. Don't use Rust strings for anything other than `data`!

mod checksum;
mod date;
mod monthyear;
mod multiple_chars;
mod multiple_strings;
mod time;
mod timestamp;
mod tz;
mod tz_time;
mod tz_timestamp;

use crate::FixValue;

pub use checksum::CheckSum;
pub use date::Date;
pub use monthyear::MonthYear;
pub use multiple_chars::MultipleChars;
pub use multiple_strings::MultipleStrings;
pub use time::Time;
pub use timestamp::Timestamp;
pub use tz::Tz;
pub use tz_time::TzTime;
pub use tz_timestamp::TzTimestamp;

/// Type alias for ISO 3166-1 alpha-2 strings (two-letter country codes).
pub type Country = [u8; 2];
/// Type alias for ISO 4217 alpha codes (three-letter currency codes).
pub type Currency = [u8; 3];
/// Type alias for four-letter *Market Identifier Codes* (MICs) as defined by
/// ISO 10383.
pub type Exchange = [u8; 4];

#[cfg(test)]
pub fn verify_serialization_behavior<T>(item: T) -> bool
where
    T: for<'a> FixValue<'a> + PartialEq,
{
    let serialized = item.to_bytes();
    let bytes = &serialized[..];
    let deserialized = T::deserialize(bytes).ok().unwrap();
    let deserialized_lossy = T::deserialize_lossy(bytes).ok().unwrap();
    deserialized == item && deserialized_lossy == item
}
