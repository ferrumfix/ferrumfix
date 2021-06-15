use crate::fix_values::*;
use crate::{Buffer, TagU16};
use std::convert::TryInto;
use std::string::ToString;

const ERR_BOOL_LENGTH: &str = "Invalid length; a boolean is Y or N (1 char).";
const ERR_BOOL_CHAR: &str = "Invalid character for boolean. Only Y and N are valid.";
const ERR_UTF8: &str = "Invalid byte sequence; expected UTF-8 valid bytes.";
const ERR_INT_INVALID: &str = "Invalid integer digits.";
const ERR_DECIMAL_INVALID: &str = "Invalid decimal number.";

/// Allows (de)serialization as a FIX value (e.g. `tag=value|`).
pub trait FixValue<'a>
where
    Self: Sized,
{
    /// The error type that can arise during deserialization.
    type Error;
    /// A type with values that customize the serialization algorithm, e.g.
    /// padding information.
    type SerializeSettings: Default;

    /// Flag that is enabled if and only if the byte representation of `Self` is
    /// ALWAYS valid ASCII.
    ///
    /// This flag is currently not used, but it might be once Rust supports
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
    fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
    where
        B: Buffer;

    /// Parses and deserializes from `data`.
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error>;

    /// Like [`FixValue::deserialize`], but it's allowed to skip *some* amount of
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

    /// Allocates a [`String`] representation of `self`.
    ///
    /// # Panics
    /// This function will panic if the underlying byte representation is not
    /// valid UTF-8. As such, you should only *ever* use this function for
    /// [`FixValue`] implementors that are guaranteed to be representable with
    /// valid UTF-8 (like numbers with ASCII digits).
    fn to_string(&self) -> String {
        String::from_utf8(self.to_bytes()).expect("Invalid UTF-8 representation of FIX field.")
    }
}

/// Zero-padding instructions for integers.
#[derive(Debug, Copy, Clone, Default)]
pub struct ZeroPadding(pub usize);

/// Specifies whether a timestamp should have millisecond or second precision.
///
/// Enabled by [`Default`].
#[derive(Debug, Copy, Clone)]
pub struct WithMilliseconds(pub bool);

impl Default for WithMilliseconds {
    fn default() -> Self {
        Self(true)
    }
}

#[cfg(feature = "utils-chrono")]
impl<'a> FixValue<'a> for chrono::DateTime<chrono::Utc> {
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
    fn serialize_with<B>(&self, buffer: &mut B, settings: Self::SerializeSettings) -> usize
    where
        B: Buffer,
    {
        use chrono::{Datelike, Timelike};
        (self.year() as u32).serialize_with(buffer, ZeroPadding(4));
        (self.month() as u32).serialize_with(buffer, ZeroPadding(2));
        (self.day() as u32).serialize_with(buffer, ZeroPadding(2));
        buffer.extend_from_slice(b"-");
        (self.hour() as u32).serialize_with(buffer, ZeroPadding(2));
        buffer.extend_from_slice(b":");
        (self.minute() as u32).serialize_with(buffer, ZeroPadding(2));
        buffer.extend_from_slice(b":");
        (self.second() as u32).serialize_with(buffer, ZeroPadding(2));
        if settings.0 {
            buffer.extend_from_slice(b".");
            (self.nanosecond() / 10E6 as u32).serialize_with(buffer, ZeroPadding(3));
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

#[cfg(feature = "utils-chrono")]
impl<'a> FixValue<'a> for chrono::NaiveDate {
    type Error = &'static str;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
    where
        B: Buffer,
    {
        use chrono::Datelike;
        (self.year() as u32).serialize_with(buffer, ZeroPadding(4));
        (self.month() as u32).serialize_with(buffer, ZeroPadding(2));
        (self.day() as u32).serialize_with(buffer, ZeroPadding(2));
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

#[cfg(feature = "utils-rust-decimal")]
impl<'a> FixValue<'a> for rust_decimal::Decimal {
    type Error = &'static str;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        // TODO: Remove allocations.
        let s = ToString::to_string(self);
        buffer.extend_from_slice(s.as_bytes());
        s.as_bytes().len()
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        rust_decimal::Decimal::from_str(s).map_err(|_| ERR_DECIMAL_INVALID)
    }
}

#[cfg(feature = "utils-decimal")]
impl<'a> FixValue<'a> for decimal::d128 {
    type Error = decimal::Status;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        // TODO: Remove allocations.
        let s = ToString::to_string(self);
        buffer.extend_from_slice(s.as_bytes());
        s.as_bytes().len()
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        decimal::d128::set_status(decimal::Status::empty());
        let s = std::str::from_utf8(data).unwrap_or("invalid UTF-8");
        let number =
            decimal::d128::from_str(s).expect("decimal::d128 should always parse without errors");
        let status = decimal::d128::get_status();
        if status.is_empty() {
            Ok(number)
        } else {
            Err(status)
        }
    }
}

impl<'a> FixValue<'a> for bool {
    type Error = &'static str;
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
            Err(ERR_BOOL_LENGTH)
        } else if data[0] == b'Y' {
            Ok(true)
        } else if data[0] == b'N' {
            Ok(false)
        } else {
            Err(ERR_BOOL_CHAR)
        }
    }

    #[inline(always)]
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

impl<'a> FixValue<'a> for u8 {
    type Error = &'static str;
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

impl<'a> FixValue<'a> for &'a [u8] {
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

impl<'a, const N: usize> FixValue<'a> for [u8; N] {
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

impl<'a, const N: usize> FixValue<'a> for &'a [u8; N] {
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

impl<'a> FixValue<'a> for TagU16 {
    type Error = &'static str;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let s = ToString::to_string(self);
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        s.parse().map_err(|_| ERR_INT_INVALID)
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
        TagU16::new(n).ok_or(ERR_INT_INVALID)
    }
}

impl<'a> FixValue<'a> for u32 {
    type Error = &'static str;
    type SerializeSettings = ZeroPadding;

    const IS_ASCII: bool = true;

    #[inline(always)]
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

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        s.parse().map_err(|_| ERR_INT_INVALID)
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

impl<'a> FixValue<'a> for i32 {
    type Error = &'static str;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let s = ToString::to_string(self);
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        s.parse().map_err(|_| ERR_INT_INVALID)
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

impl<'a> FixValue<'a> for u64 {
    type Error = &'static str;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let s = ToString::to_string(self);
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        s.parse().map_err(|_| ERR_INT_INVALID)
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

impl<'a> FixValue<'a> for i64 {
    type Error = &'static str;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let s = ToString::to_string(self);
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        s.parse().map_err(|_| ERR_INT_INVALID)
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

impl<'a> FixValue<'a> for usize {
    type Error = &'static str;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    #[inline(always)]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let s = ToString::to_string(self);
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }

    #[inline(always)]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        s.parse().map_err(|_| ERR_INT_INVALID)
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
        let s = FixValue::to_string(&n);
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
