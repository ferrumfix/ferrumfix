//! Types for holding [`DataType`](crate::DataType) values.
//!
//! This module introduces reusable, allocation-free data structures that can be
//! used to store [`DataType`](crate::DataType) values. This is done via the
//! [`DataField`] trait, which allows both serialization and deserialization.
//!
//! FerrumFIX maps FIX date types to the following [`DataField`] implementors:
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
//! The module name (`dtf`) is short for **D**a**T**a **F**ields.
//!
//! # Quick tour of [`DataField`]
//!
//! ```
//! use fefix::dtf::{DataField, Timestamp};
//!
//! let bytes = b"20130422-12:30:00.000";
//!
//! // You can use `DataField::deserialize` to parse data fields.
//! let timestamp = Timestamp::deserialize(bytes).unwrap();
//! assert_eq!(timestamp.date().year(), 2013);
//!
//! // `DataField::deserialize_lossy` is like `DataField::deserialize`, but it's
//! // allowed to skip some format verification for the sake of speed.
//! assert!(u32::deserialize(b"invalid integer").is_err());
//! assert!(u32::deserialize_lossy(b"invalid integer").is_ok());
//!
//! let buffer: &mut Vec<u8> = &mut vec![];
//! // Use `DataField::serialize` to write values to buffers.
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

use crate::Buffer;
use rust_decimal::Decimal;
use std::convert::TryInto;
use std::str::FromStr;

pub trait SubDataField<'a, T>
where
    Self: Sized,
    T: DataField<'a>,
{
    type Error: From<T::Error>;

    fn convert(dtf: T) -> Result<Self, Self::Error>;
}

impl<'a, T> SubDataField<'a, T> for T
where
    T: DataField<'a>,
{
    type Error = <T as DataField<'a>>::Error;

    fn convert(dtf: T) -> Result<Self, Self::Error> {
        Ok(dtf)
    }
}

/// A trait for serializing data directly into a [`Buffer`].
pub trait DataField<'a>
where
    Self: Sized,
{
    type Error;
    type SerializeSettings;

    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer;

    fn serialize_custom<B>(&self, buffer: &mut B, _settings: &Self::SerializeSettings) -> usize
    where
        B: Buffer,
    {
        self.serialize(buffer)
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error>;

    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        Self::deserialize(data)
    }
}

impl<'a> DataField<'a> for Decimal {
    type Error = error::Decimal;
    type SerializeSettings = ();

    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        // TODO: Remove allocations.
        let s = self.to_string();
        buffer.extend_from_slice(s.as_bytes());
        s.as_bytes().len()
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| Self::Error::NotUtf8)?;
        Decimal::from_str(s).map_err(|err| Self::Error::Other(err.to_string()))
    }
}

impl<'a> DataField<'a> for bool {
    type Error = error::Bool;
    type SerializeSettings = ();

    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        let byte = if *self { b'Y' } else { b'N' };
        buffer.extend_from_slice(&[byte]);
        1
    }

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

    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(data == b"Y")
    }
}

impl<'a> DataField<'a> for u8 {
    type Error = error::Int;
    type SerializeSettings = ();

    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        buffer.extend_from_slice(&[*self]);
        1
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(data[0])
    }
}

impl<'a> DataField<'a> for &'a [u8] {
    type Error = ();
    type SerializeSettings = ();

    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        buffer.extend_from_slice(self);
        self.len()
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(data)
    }
}

impl<'a> SubDataField<'a, &'a [u8]> for &'a str {
    type Error = ();

    fn convert(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(bytes).map_err(|_| ())
    }
}

impl<'a, const N: usize> DataField<'a> for &'a [u8; N] {
    type Error = ();
    type SerializeSettings = ();

    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        buffer.extend_from_slice(&self[..]);
        self.len()
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        data.try_into().map_err(|_| ())
    }
}

impl<'a> DataField<'a> for u32 {
    type Error = error::Int;
    type SerializeSettings = ();

    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        let s = self.to_string();
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| Self::Error::InvalidUtf8)?;
        s.parse().map_err(|_| Self::Error::Other)
    }

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

impl<'a> DataField<'a> for i32 {
    type Error = error::Int;
    type SerializeSettings = ();

    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        let s = self.to_string();
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| Self::Error::InvalidUtf8)?;
        s.parse().map_err(|_| Self::Error::Other)
    }

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

impl<'a> DataField<'a> for u64 {
    type Error = error::Int;
    type SerializeSettings = ();

    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        let s = self.to_string();
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| Self::Error::InvalidUtf8)?;
        s.parse().map_err(|_| Self::Error::Other)
    }

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

impl<'a> DataField<'a> for i64 {
    type Error = error::Int;
    type SerializeSettings = ();

    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        let s = self.to_string();
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| Self::Error::InvalidUtf8)?;
        s.parse().map_err(|_| Self::Error::Other)
    }

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

impl<'a> DataField<'a> for usize {
    type Error = error::Int;
    type SerializeSettings = ();

    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        let s = self.to_string();
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| Self::Error::InvalidUtf8)?;
        s.parse().map_err(|_| Self::Error::Other)
    }

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_bools() {
        let mut buffer = Vec::new();
        assert_eq!(true.serialize(&mut buffer), 1);
        assert_eq!(false.serialize(&mut buffer), 1);
        assert_eq!(&buffer[..], b"YN" as &[u8]);
    }

    #[test]
    fn serialize_bytes() {
        let data: &[&[u8]] = &[b"hello", b"", b" ", b"foo"];
        let mut buffer = Vec::new();
        for slice in data {
            assert_eq!(slice.serialize(&mut buffer), slice.len());
        }
        assert_eq!(&buffer[..], b"hello foo" as &[u8]);
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
