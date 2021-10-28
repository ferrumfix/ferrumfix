//! A wide collection of [`FixValue`](crate::FixValue) implementors.
//!
//! | FIX datatype               | Relevant [`FixValue`] implementors                                                 |
//! |----------------------------|------------------------------------------------------------------------------------|
//! | `int`                      | [`u32`], [`i32`], [`u64`], [`i64`]
//! | `Length`                   | [`usize`]                                                                          |
//! | `NumInGroup`               | [`usize`]                                                                          |
//! | `SeqNum`                   | [`u64`]                                                                            |
//! | `TagNum`                   | [`TagU16`](crate::TagU16)                                                          |
//! | `DayOfMonth`               | [`u32`]                                                                            |
//! | `float` and `float` -like  | [`f32`], [`f64`]. [`rust_decimal::Decimal`](https://docs.rs/rust_decimal/1.16.0/rust_decimal/struct.Decimal.html) and [`decimal::d128`](https://docs.rs/decimal/2.1.0/decimal/struct.d128.html) are also supported. |
//! | `Boolean`                  | [`bool`]                                                                           |
//! | `char`                     | [`u8`] [^1]                                                                        |
//! | `String`                   | [`Vec<u8>`], `&[u8]`.[^1]                                                            |
//! | `data`                     | [`Vec<u8>`], `&[u8]` (also [`String`], [`str`] for UTF-8 content).                   |
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
//! use fefix::fix_value::Timestamp;
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
//! let mut buffer: Vec<u8> = vec![];
//! // Use `FixValue::serialize` to write values to buffers.
//! 1337u32.serialize(&mut buffer);
//! assert_eq!(&buffer[..], b"1337" as &[u8]);
//! ```
//!
//! [^1]: With the exception of datatype `data`, FIX mandates a single-byte
//! encoding (Latin alphabet No. 1 by default), while Rust strings are UTF-8,
//! which is a multibyte. These are *not* compatible. Watch out!

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

use crate::Buffer;

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

/// Provides (de)serialization logic for a Rust type as FIX field values.
///
/// See the [`fix_value`](crate::fix_value) module for more information.
pub trait FixValue<'a>
where
    Self: Sized,
{
    /// The error type that can arise during deserialization.
    type Error;
    /// A type with values that customize the serialization algorithm, e.g.
    /// padding information.
    type SerializeSettings: Default;

    /// Writes `self` to `buffer` using default settings.
    #[inline]
    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        self.serialize_with(buffer, Self::SerializeSettings::default())
    }

    /// Writes `self` to `buffer` using custom serialization `settings`.
    fn serialize_with<B>(&self, buffer: &mut B, settings: Self::SerializeSettings) -> usize
    where
        B: Buffer;

    /// Parses and deserializes from `data`.
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error>;

    /// Like [`FixValue::deserialize`], but it's allowed to skip *some* amount of
    /// input checking. Invalid inputs might not trigger errors and instead be
    /// deserialized as random values.
    ///
    /// # Safety
    ///
    /// This method remains 100% safe even on malformed inputs.
    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        Self::deserialize(data)
    }

    /// Serializes `self` to a [`Vec`] of bytes, allocated on the fly.
    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.serialize(&mut buffer);
        buffer
    }

    /// Allocates a [`String`] representation of `self`, using [`FixValue::to_bytes`].
    ///
    /// # Panics
    ///
    /// This function will panic if the underlying byte representation is not
    /// valid UTF-8. As such, you should only *ever* use this function for
    /// [`FixValue`] implementors that are guaranteed to be representable with
    /// valid UTF-8 (like numbers with ASCII digits).
    fn to_string(&self) -> String {
        String::from_utf8(self.to_bytes()).expect("Invalid UTF-8 representation of FIX field.")
    }
}

/// Zero-padding for integers; see [`FixValue::SerializeSettings`].
#[derive(Debug, Copy, Clone, Default)]
pub struct ZeroPadding(pub usize);

/// Tries to [`FixValue::serialize`] an `item`, then to
/// [`FixValue::deserialize`] it, and finally checks for equality with the
/// initial data. [`FixValue::deserialize_lossy`] is then tested in the same
/// manner.
pub fn test_utility_verify_serialization_behavior<T>(item: T) -> bool
where
    T: for<'a> FixValue<'a> + PartialEq,
{
    let serialized = item.to_bytes();
    let bytes = &serialized[..];
    let deserialized = T::deserialize(bytes).ok().unwrap();
    let deserialized_lossy = T::deserialize_lossy(bytes).ok().unwrap();
    deserialized == item && deserialized_lossy == item
}

#[cfg(test)]
mod test {
    use super::*;
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
        let s = FixValue::to_string(&n);
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
