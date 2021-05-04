//! Types for holding [`DataType`](crate::DataType) values.
//!
//! This module introduces reusable, allocation-free data structures that can be
//! used to store [`DataType`](crate::DataType) values. This is done via the
//! [`DataType`] trait, which allows both serialization and deserialization.
//!
//! FerrumFIX maps FIX date types to the following [`DataType`] implementors:
//!
//! - `int`: any Rust primitive integer type;
//! - `Length`: [`usize`];
//! - `NumInGroup`: [`u16`];
//! - `SeqNum`: [`u64`];
//! - `TagNum`: [`u32`];
//! - `DayOfMonth`: [`u32`];
//! - `float`, `Qty`, `Price`, `PriceOffset`, `Amt`, `Percentage`: [`rust_decimal::Decimal`];
//! - `char`: [`u8`] (FIX mandates the use of a single-byte encoding, so `u8` is
//! a better fit than `char`);
//! - `Boolean`: [`bool`];
//! - `String`, `data`: `&[u8]`;
//! - `MultipleCharValue`: [`MultipleChars`];
//! - `MultipleValueString`: [`MultipleStrings`];
//! - `Country`: `[u8; 2]`;
//! - `Currency`: `[u8; 3]`;
//! - `Exchange`: `[u8; 4]`;
//! - `month-year`: [`MonthYear`];
//! - `UTCTimestamp`, `LocalMktDate`: [`Timestamp`];
//! - `UTCTimeOnly`: [`Time`];
//! - `UTCDateOnly`: [`Date`];
//! - `TZTimeOnly`: [`TzTime`];
//! - `TZTimestamp`: [`TzTimestamp`];
//!
//! # Quick tour of [`DataType`]
//!
//! ```
//! use fefix::datatypes::{DataType, Timestamp};
//!
//! let bytes = b"20130422-12:30:00.000";
//!
//! // You can use `DataType::deserialize` to parse data fields.
//! let timestamp = Timestamp::deserialize(bytes).unwrap();
//! assert_eq!(timestamp.date().year(), 2013);
//!
//! // `DataType::deserialize_lossy` is like `DataType::deserialize`, but it's
//! // allowed to skip some format verification for the sake of speed.
//! assert!(u32::deserialize(b"invalid integer").is_err());
//! assert!(u32::deserialize_lossy(b"invalid integer").is_ok());
//!
//! let buffer: &mut Vec<u8> = &mut vec![];
//! // Use `DataType::serialize` to write values to buffers.
//! 1337u32.serialize(buffer);
//! assert_eq!(&buffer[..], b"1337" as &[u8]);
//! ```

mod checksum;
mod date;
pub mod error;
mod monthyear;
mod multiple_chars;
mod multiple_strings;
mod time;
mod timestamp;
mod tz;
mod tz_time;
mod tz_timestamp;

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

use crate::{Buffer, TagU16};
use rust_decimal::Decimal;
use std::convert::TryInto;
use std::str::FromStr;

/// A trait for (de)serializing data directly into a [`Buffer`].
pub trait FixFieldValue<'a>
where
    Self: Sized,
{
    type Error;
    type SerializeSettings: Default;

    /// Flag that is enabled if and only if the byte representation of `Self` is
    /// always valid ASCII.
    ///
    /// This flag is currently not used, but it will be once Rust supports
    /// fully-fledged `const` generics.
    const IS_ASCII: bool;

    /// Writes `self` to `buffer` using default settings.
    #[inline(always)]
    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        self.serialize_with(buffer, Self::SerializeSettings::default())
    }

    /// Writes `self` to `buffer` using custom serialization `settings`.
    fn serialize_with<B>(
        &self,
        buffer: &mut B,
        _settings: Self::SerializeSettings,
    ) -> usize
    where
        B: Buffer;

    /// Parses and deserializes from `data`.
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error>;

    /// Like [`Self::deserialize`], but it's allowed to skip *some* amount of
    /// input checking. Invalid inputs might not trigger errors and instead be
    /// deserialized as random values.
    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        Self::deserialize(data)
    }

    /// Serializes `self` to a [`Vec`] of bytes, allocated on the fly.
    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.serialize(&mut buffer);
        buffer
    }

    fn to_string_opt(&self) -> Option<String> {
        String::from_utf8(self.to_bytes()).ok()
    }
}

/// Byte-padding instructions for byte strings.
#[derive(Debug, Copy, Clone)]
pub struct Padding {
    pub len: usize,
    pub byte: u8,
}

impl Default for Padding {
    #[inline(always)]
    fn default() -> Self {
        Self { len: 0, byte: 0 }
    }
}

impl Padding {
    #[inline(always)]
    pub fn zeros(len: usize) -> Self {
        Self { len, byte: b'0' }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct WithMilliseconds(pub bool);

impl Default for WithMilliseconds {
    fn default() -> Self {
        Self(true)
    }
}

#[cfg(feature = "chrono_time")]
impl<'a> FixFieldValue<'a> for chrono::DateTime<chrono::Utc> {
    type Error = &'static str;
    type SerializeSettings = WithMilliseconds;

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        // Serialize with milliseconds by default.
        self.serialize_with(buffer, WithMilliseconds(true))
    }

    #[inline(always)]
    fn serialize_with<B>(
        &self,
        buffer: &mut B,
        settings: Self::SerializeSettings,
    ) -> usize
    where
        B: Buffer,
    {
        use chrono::{Datelike, Timelike};
        (self.year() as u32).serialize_with(buffer, Padding::zeros(4));
        (self.month() as u32).serialize_with(buffer, Padding::zeros(2));
        (self.day() as u32).serialize_with(buffer, Padding::zeros(2));
        buffer.extend_from_slice(b"-");
        (self.hour() as u32).serialize_with(buffer, Padding::zeros(2));
        buffer.extend_from_slice(b":");
        (self.minute() as u32).serialize_with(buffer, Padding::zeros(2));
        buffer.extend_from_slice(b":");
        (self.second() as u32).serialize_with(buffer, Padding::zeros(2));
        if settings.0 {
            buffer.extend_from_slice(b".");
            (self.nanosecond() / 10E6 as u32).serialize_with(buffer, Padding::zeros(3));
            21
        } else {
            17
        }
    }

    #[inline(always)]
    fn deserialize(_data: &'a [u8]) -> Result<Self, Self::Error> {
        Err("TODO")
    }
}

#[cfg(feature = "chrono_time")]
impl<'a> FixFieldValue<'a> for chrono::NaiveDate {
    type Error = &'static str;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize_with<B>(
        &self,
        buffer: &mut B,
        _settings: Self::SerializeSettings,
    ) -> usize
    where
        B: Buffer,
    {
        use chrono::Datelike;
        (self.year() as u32).serialize_with(buffer, Padding::zeros(4));
        (self.month() as u32).serialize_with(buffer, Padding::zeros(2));
        (self.day() as u32).serialize_with(buffer, Padding::zeros(2));
        8
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let date = Date::deserialize(data).map_err(|_| "Invalid date format.")?;
        date.to_chrono_naive().ok_or("Invalid date range.")
    }

    #[inline(always)]
    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        let date = Date::deserialize_lossy(data).map_err(|_| "Invalid date format.")?;
        date.to_chrono_naive().ok_or("Invalid date range.")
    }
}

impl<'a> FixFieldValue<'a> for Decimal {
    type Error = error::Decimal;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        // TODO: Remove allocations.
        let s = self.to_string();
        buffer.extend_from_slice(s.as_bytes());
        s.as_bytes().len()
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| Self::Error::NotUtf8)?;
        Decimal::from_str(s).map_err(|err| Self::Error::Other(err.to_string()))
    }
}

impl<'a> FixFieldValue<'a> for bool {
    type Error = error::Bool;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let byte = if *self { b'Y' } else { b'N' };
        buffer.extend_from_slice(&[byte]);
        1
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != 1 {
            Err(Self::Error::WrongLength)
        } else if data[0] == b'Y' {
            Ok(true)
        } else if data[0] == b'N' {
            Ok(false)
        } else {
            Err(Self::Error::InvalidCharacter)
        }
    }

    #[inline(always)]
    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != 1 {
            Err(Self::Error::WrongLength)
        } else {
            Ok(data[0] == b'Y')
        }
    }
}

impl<'a> FixFieldValue<'a> for &'a str {
    type Error = std::str::Utf8Error;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        buffer.extend_from_slice(self.as_bytes());
        self.as_bytes().len()
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(data)
    }
}

impl<'a> FixFieldValue<'a> for u8 {
    type Error = error::Int;
    type SerializeSettings = ();

    const IS_ASCII: bool = false;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        buffer.extend_from_slice(&[*self]);
        1
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(data[0])
    }
}

impl<'a> FixFieldValue<'a> for &'a [u8] {
    type Error = ();
    type SerializeSettings = ();

    const IS_ASCII: bool = false;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        buffer.extend_from_slice(self);
        self.len()
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(data)
    }
}

impl<'a, const N: usize> FixFieldValue<'a> for [u8; N] {
    type Error = ();
    type SerializeSettings = ();

    const IS_ASCII: bool = false;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, settings: ()) -> usize
    where
        B: Buffer,
    {
        (&self).serialize_with(buffer, settings)
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        data.try_into().map_err(|_| ())
    }
}

impl<'a, const N: usize> FixFieldValue<'a> for &'a [u8; N] {
    type Error = ();
    type SerializeSettings = ();

    const IS_ASCII: bool = false;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        buffer.extend_from_slice(&self[..]);
        self.len()
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        data.try_into().map_err(|_| ())
    }
}

impl<'a> FixFieldValue<'a> for TagU16 {
    type Error = error::Int;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let s = self.to_string();
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| Self::Error::InvalidUtf8)?;
        s.parse().map_err(|_| Self::Error::Other)
    }

    #[inline(always)]
    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        fn ascii_digit_to_u16(digit: u8) -> u16 {
            (digit as u16).wrapping_sub(b'0' as u16)
        }
        let mut n = 0u16;
        for byte in data.iter().copied() {
            n = n.wrapping_mul(10).wrapping_add(ascii_digit_to_u16(byte));
        }
        TagU16::new(n).ok_or(Self::Error::Other)
    }
}

impl<'a> FixFieldValue<'a> for u32 {
    type Error = error::Int;
    type SerializeSettings = Padding;

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize_with<B>(
        &self,
        buffer: &mut B,
        padding: Self::SerializeSettings,
    ) -> usize
    where
        B: Buffer,
    {
        if padding.len == 0 {
            let s = self.to_string();
            buffer.extend_from_slice(s.as_bytes());
            return s.len();
        }
        let initial_len = buffer.len();
        buffer.resize(buffer.len() + padding.len, padding.byte);
        let bytes = buffer.as_mut_slice();
        let mut multiplier = 1;
        for i in (0..padding.len).rev() {
            bytes[i + initial_len] =
                ((self / multiplier) % 10).wrapping_add(b'0' as u32) as u8;
            multiplier *= 10;
        }
        padding.len
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| Self::Error::InvalidUtf8)?;
        s.parse().map_err(|_| Self::Error::Other)
    }

    #[inline(always)]
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

impl<'a> FixFieldValue<'a> for i32 {
    type Error = error::Int;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let s = self.to_string();
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| Self::Error::InvalidUtf8)?;
        s.parse().map_err(|_| Self::Error::Other)
    }

    #[inline(always)]
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

impl<'a> FixFieldValue<'a> for u64 {
    type Error = error::Int;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let s = self.to_string();
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| Self::Error::InvalidUtf8)?;
        s.parse().map_err(|_| Self::Error::Other)
    }

    #[inline(always)]
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

impl<'a> FixFieldValue<'a> for i64 {
    type Error = error::Int;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let s = self.to_string();
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| Self::Error::InvalidUtf8)?;
        s.parse().map_err(|_| Self::Error::Other)
    }

    #[inline(always)]
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

impl<'a> FixFieldValue<'a> for usize {
    type Error = error::Int;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let s = self.to_string();
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| Self::Error::InvalidUtf8)?;
        s.parse().map_err(|_| Self::Error::Other)
    }

    #[inline(always)]
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

pub trait SuperDataType<'a, T>
where
    Self: FixFieldValue<'a>,
    T: FixFieldValue<'a>,
{
}

/// A [`DataType`] is always a [`SuperDataType`] of itself.
impl<'a, T> SuperDataType<'a, T> for T where T: FixFieldValue<'a> {}

impl<'a> SuperDataType<'a, &'a str> for &'a [u8] {}
impl<'a> SuperDataType<'a, i64> for &'a [u8] {}
impl<'a> SuperDataType<'a, u32> for u64 {}
impl<'a> SuperDataType<'a, i32> for i64 {}

#[cfg(test)]
pub fn verify_serialization_behavior<T>(item: T) -> bool
where
    T: for<'a> FixFieldValue<'a> + PartialEq,
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
        let buffer = &mut Vec::new();
        let s = n.to_string();
        let bytes = s.as_bytes();
        let len = n.serialize(buffer);
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
