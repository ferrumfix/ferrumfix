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

use crate::FixValue;
use crate::{Buffer, TagU16};
use std::convert::TryInto;
use std::string::ToString;

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

const ERR_BOOL_LENGTH: &str = "Invalid length; a boolean is Y or N (1 char).";
const ERR_BOOL_CHAR: &str = "Invalid character for boolean. Only Y and N are valid.";
const ERR_INT_INVALID: &str = "Invalid integer digits.";
const ERR_DECIMAL: &str = "Invalid decimal number.";
pub(crate) const ERR_UTF8: &str = "Invalid byte sequence; expected UTF-8 valid bytes.";

/// Zero-padding for integers; see [`FixValue::SerializeSettings`].
#[derive(Debug, Copy, Clone, Default)]
pub struct ZeroPadding(pub usize);

impl<'a> FixValue<'a> for bool {
    type Error = &'static str;
    type SerializeSettings = ();

    #[inline]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let byte = if *self { b'Y' } else { b'N' };
        buffer.extend_from_slice(&[byte]);
        1
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != 1 {
            Err(ERR_BOOL_LENGTH)
        } else if data[0] == b'Y' {
            Ok(true)
        } else if data[0] == b'N' {
            Ok(false)
        } else {
            Err(ERR_BOOL_CHAR)
        }
    }

    #[inline]
    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != 1 {
            Err(ERR_BOOL_LENGTH)
        } else {
            Ok(data[0] == b'Y')
        }
    }
}

impl<'a> FixValue<'a> for &'a str {
    type Error = std::str::Utf8Error;
    type SerializeSettings = ();

    #[inline]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        buffer.extend_from_slice(self.as_bytes());
        self.as_bytes().len()
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(data)
    }
}

impl<'a> FixValue<'a> for &'a [u8] {
    type Error = ();
    type SerializeSettings = ();

    #[inline]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        buffer.extend_from_slice(self);
        self.len()
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(data)
    }
}

impl<'a, const N: usize> FixValue<'a> for [u8; N] {
    type Error = ();
    type SerializeSettings = ();

    #[inline]
    fn serialize_with<B>(&self, buffer: &mut B, settings: ()) -> usize
    where
        B: Buffer,
    {
        (&self).serialize_with(buffer, settings)
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        data.try_into().map_err(|_| ())
    }
}

impl<'a, const N: usize> FixValue<'a> for &'a [u8; N] {
    type Error = ();
    type SerializeSettings = ();

    #[inline]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        buffer.extend_from_slice(&self[..]);
        self.len()
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        data.try_into().map_err(|_| ())
    }
}

impl<'a> FixValue<'a> for TagU16 {
    type Error = &'static str;
    type SerializeSettings = ();

    #[inline]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let s = ToString::to_string(self);
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        s.parse().map_err(|_| ERR_INT_INVALID)
    }

    #[inline]
    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        fn ascii_digit_to_u16(digit: u8) -> u16 {
            (digit as u16).wrapping_sub(b'0' as u16)
        }
        let mut n = 0u16;
        for byte in data.iter().copied() {
            n = n.wrapping_mul(10).wrapping_add(ascii_digit_to_u16(byte));
        }
        TagU16::new(n).ok_or(ERR_INT_INVALID)
    }
}

impl<'a> FixValue<'a> for u32 {
    type Error = &'static str;
    type SerializeSettings = ZeroPadding;

    #[inline]
    fn serialize_with<B>(&self, buffer: &mut B, padding: Self::SerializeSettings) -> usize
    where
        B: Buffer,
    {
        if padding.0 == 0 {
            let s = ToString::to_string(self);
            buffer.extend_from_slice(s.as_bytes());
            return s.len();
        }
        let initial_len = buffer.len();
        buffer.resize(buffer.len() + padding.0, b'0');
        let bytes = buffer.as_mut_slice();
        let mut multiplier = 1;
        for i in (0..padding.0).rev() {
            bytes[i + initial_len] = ((self / multiplier) % 10).wrapping_add(b'0' as u32) as u8;
            multiplier *= 10;
        }
        padding.0
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        s.parse().map_err(|_| ERR_INT_INVALID)
    }

    #[inline]
    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        fn ascii_digit_to_u32(digit: u8) -> u32 {
            (digit as u32).wrapping_sub(b'0' as u32)
        }
        let mut n = 0u32;
        for byte in data.iter().copied() {
            n = n.wrapping_mul(10).wrapping_add(ascii_digit_to_u32(byte));
        }
        Ok(n)
    }
}

impl<'a> FixValue<'a> for i32 {
    type Error = &'static str;
    type SerializeSettings = ();

    #[inline]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let s = ToString::to_string(self);
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        s.parse().map_err(|_| ERR_INT_INVALID)
    }

    #[inline]
    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        fn ascii_digit_to_i32(digit: u8) -> i32 {
            digit as i32 - b'0' as i32
        }
        let mut n = 0;
        for byte in data.iter().copied() {
            n = n * 10 + ascii_digit_to_i32(byte);
        }
        let sign = if data[0] == b'-' { -1 } else { 1 };
        Ok(n * sign)
    }
}

impl<'a> FixValue<'a> for u64 {
    type Error = &'static str;
    type SerializeSettings = ();

    #[inline]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let s = ToString::to_string(self);
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        s.parse().map_err(|_| ERR_INT_INVALID)
    }

    #[inline]
    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        fn ascii_digit_to_u64(digit: u8) -> u64 {
            digit as u64 - b'0' as u64
        }
        let mut n = 0;
        for byte in data.iter().copied() {
            n = n * 10 + ascii_digit_to_u64(byte);
        }
        Ok(n)
    }
}

impl<'a> FixValue<'a> for i64 {
    type Error = &'static str;
    type SerializeSettings = ();

    #[inline]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let s = ToString::to_string(self);
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        s.parse().map_err(|_| ERR_INT_INVALID)
    }

    #[inline]
    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        fn ascii_digit_to_i64(digit: u8) -> i64 {
            digit as i64 - b'0' as i64
        }
        let mut n = 0;
        for byte in data.iter().copied() {
            n = n * 10 + ascii_digit_to_i64(byte);
        }
        let sign = if data[0] == b'-' { -1 } else { 1 };
        Ok(n * sign)
    }
}

impl<'a> FixValue<'a> for usize {
    type Error = &'static str;
    type SerializeSettings = ();

    #[inline]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let s = ToString::to_string(self);
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        s.parse().map_err(|_| ERR_INT_INVALID)
    }

    #[inline]
    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        fn ascii_digit_to_usize(digit: u8) -> usize {
            digit as usize - b'0' as usize
        }
        let mut n = 0;
        for byte in data.iter().copied() {
            n = n * 10 + ascii_digit_to_usize(byte);
        }
        Ok(n)
    }
}

impl<'a> FixValue<'a> for f32 {
    type Error = &'static str;
    type SerializeSettings = ();

    #[inline]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let s = std::string::ToString::to_string(self);
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        str::parse(s).map_err(|_| ERR_DECIMAL)
    }
}

impl<'a> FixValue<'a> for f64 {
    type Error = &'static str;
    type SerializeSettings = ();

    #[inline]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let s = std::string::ToString::to_string(self);
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        str::parse(s).map_err(|_| ERR_DECIMAL)
    }
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
