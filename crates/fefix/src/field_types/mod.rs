//! A wide collection of [`FieldType`](crate::FieldType) implementors.
//!
//! | FIX datatype               | Suggested [`FieldType`] implementors                                                 |
//! |----------------------------|------------------------------------------------------------------------------------|
//! | `int`                      | [`u32`], [`i32`], [`u64`], [`i64`], [`u128`], [`i128`].                            |
//! | `Length`                   | [`usize`].                                                                         |
//! | `NumInGroup`               | [`usize`].                                                                         |
//! | `SeqNum`                   | [`u64`].                                                                           |
//! | `TagNum`                   | [`TagU16`](crate::TagU16).                                                         |
//! | `DayOfMonth`               | [`u32`].                                                                           |
//! | `float`, `Price`, etc.     | [`f32`], [`f64`], [`struct@rust_decimal::Decimal`], [`struct@decimal::d128`]. |
//! | `Boolean`                  | [`bool`].                                                                          |
//! | `char`                     | [`u8`] [^1].                                                                      |
//! | `String`                   | [`Vec<u8>`], `&[u8]`.[^1]                                                          |
//! | `data`                     | [`Vec<u8>`], `&[u8]` (also [`String`], [`str`] for UTF-8 content).                 |
//! | `MultipleCharValue`        | [`MultipleChars`] [^1].                                                            |
//! | `MultipleValueString`      | [`MultipleStrings`] [^1].                                                          |
//! | `Country`                  | [`Country`].                                                                       |
//! | `Currency`                 | [`Currency`].                                                                      |
//! | `Exchange`                 | [`Exchange`].                                                                      |
//! | `month-year`               | [`MonthYear`].                                                                     |
//! | `UTCTimeOnly`              | [`Time`], [`chrono::NaiveTime`].                                                                          |
//! | `UTCDateOnly`              | [`Date`], [`chrono::NaiveDate`].                                                                          |
//! | `UTCTimestamp`             | [`Timestamp`], [`chrono::NaiveDateTime`].                                                                     |
//! | `TZTimestamp`              | [`TzTimestamp`], [`chrono::DateTime<chrono::FixedOffset>`].                                                                   |
//! | `LocalMktDate`             | [`Date`], [`chrono::NaiveDate`].                                                                     |
//! | `TZTimeOnly`               | [`TzTime`],  |
//!
//! The above table provides some useful guidelines that work for the vast
//! majority of use cases.
//!
//! # Quick tour of [`FieldType`]
//!
//! ```
//! use fefix::FieldType;
//! use fefix::field_types::Timestamp;
//!
//! let bytes = b"20130422-12:30:00.000";
//!
//! // You can use `FieldType::deserialize` to parse data fields.
//! let timestamp = Timestamp::deserialize(bytes).unwrap();
//! assert_eq!(timestamp.date().year(), 2013);
//!
//! // `FieldType::deserialize_lossy` is like `FieldType::deserialize`, but it's
//! // allowed to skip some format verification for the sake of speed.
//! assert!(u32::deserialize(b"invalid integer").is_err());
//! assert!(u32::deserialize_lossy(b"invalid integer").is_ok());
//!
//! let mut buffer: Vec<u8> = vec![];
//! // Use `FieldType::serialize` to write values to buffers.
//! 1337u32.serialize(&mut buffer);
//! assert_eq!(&buffer[..], b"1337" as &[u8]);
//! ```
//!
//! [^1]: With the exception of datatype `data`, FIX mandates a single-byte
//! encoding (Latin alphabet No. 1 by default), while Rust strings are UTF-8,
//! which is a multibyte encoding. These are *not* compatible. Watch out!

mod checksum;
mod date;
mod monthyear;
mod multiple_chars;
mod multiple_strings;
mod primitives;
mod tagu16;
mod time;
mod timestamp;
mod tz;
mod tz_time;
mod tz_timestamp;

#[cfg(feature = "utils-chrono")]
mod utils_chrono;
#[cfg(feature = "utils-decimal")]
mod utils_decimal;
#[cfg(feature = "utils-rust-decimal")]
mod utils_rust_decimal;

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

use crate::{Buffer, FieldType};

/// Type alias for ISO 3166-1 alpha-2 strings (two-letter country codes).
pub type Country = [u8; 2];
/// Type alias for ISO 4217 alpha codes (three-letter currency codes).
pub type Currency = [u8; 3];
/// Type alias for four-letter *Market Identifier Codes* (MICs) as defined by
/// ISO 10383.
pub type Exchange = [u8; 4];

pub(crate) const ERR_UTF8: &str = "Invalid byte sequence; expected UTF-8 valid bytes.";
pub(crate) const ERR_INT_INVALID: &str = "Invalid integer digits.";
pub(crate) const ERR_TIME: &str = "Invalid time.";
pub(crate) const ERR_DECIMAL: &str = "Invalid decimal number.";

/// Zero-padding for integers; see [`FieldType::SerializeSettings`].
#[derive(Debug, Copy, Clone, Default)]
pub struct ZeroPadding(pub usize);

/// Tries to [`FieldType::serialize`] an `item`, then to
/// [`FieldType::deserialize`] it, and finally checks for equality with the
/// initial data. [`FieldType::deserialize_lossy`] is then
/// tested in the same manner.
pub fn test_utility_verify_serialization_behavior<T>(item: T) -> bool
where
    T: for<'a> FieldType<'a> + PartialEq,
{
    let serialized = item.to_bytes();
    let bytes = &serialized[..];
    let deserialized = T::deserialize(bytes).ok().unwrap();
    let deserialized_lossy = T::deserialize_lossy(bytes).ok().unwrap();
    deserialized == item && deserialized_lossy == item
}

#[cfg(test)]
mod test {
    use crate::FieldType;
    use quickcheck_macros::quickcheck;

    #[test]
    fn serialize_bools() {
        let mut buffer = Vec::new();
        assert_eq!(true.serialize(&mut buffer), 1);
        assert_eq!(false.serialize(&mut buffer), 1);
        assert_eq!(&buffer[..], b"YN" as &[u8]);
    }

    #[quickcheck]
    fn serialize_bytes(data: Vec<Vec<u8>>) -> bool {
        let mut buffer = Vec::new();
        for slice in data.iter() {
            assert_eq!((&slice[..]).serialize(&mut buffer), slice.len());
        }
        &buffer[..] == &data.iter().flatten().copied().collect::<Vec<u8>>()[..]
    }

    #[quickcheck]
    fn u32_serialize(n: u32) -> bool {
        let mut buffer = Vec::new();
        let s = FieldType::to_string(&n);
        let bytes = s.as_bytes();
        let len = n.serialize(&mut buffer);
        bytes == buffer.as_slice() && len == bytes.len()
    }

    #[test]
    fn serialize_country() {
        let mut buffer = Vec::new();
        assert_eq!(b"IT".serialize(&mut buffer), 2);
        assert_eq!(&buffer[..], b"IT" as &[u8]);
    }

    #[test]
    fn serialize_currency() {
        let mut buffer = Vec::new();
        assert_eq!(b"USD".serialize(&mut buffer), 3);
        assert_eq!(&buffer[..], b"USD" as &[u8]);
    }
}
