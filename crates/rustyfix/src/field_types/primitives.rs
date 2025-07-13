use crate::{Buffer, FieldType};
use smartstring::alias::String as SmartString;
use std::str;

// Helper function to serialize integers
fn serialize_int<B: Buffer, T: std::fmt::Display>(value: T, buffer: &mut B) -> usize {
    use std::fmt::Write;
    let start_len = buffer.len();
    write!(crate::BufferWriter(buffer), "{value}").unwrap();
    buffer.len() - start_len
}

// Helper function to serialize floats
fn serialize_float<B: Buffer, T: std::fmt::Display>(value: T, buffer: &mut B) -> usize {
    use std::fmt::Write;
    let start_len = buffer.len();
    write!(crate::BufferWriter(buffer), "{value}").unwrap();
    buffer.len() - start_len
}

/// Error type for integer parsing failures.
///
/// Returned when attempting to deserialize invalid integer data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("Invalid integer")]
pub struct InvalidInt;

/// Error type for floating point parsing failures.
///
/// Returned when attempting to deserialize invalid float data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("Invalid float")]
pub struct InvalidFloat;

/// Error type for boolean parsing failures.
///
/// Returned when attempting to deserialize invalid boolean data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("Invalid boolean")]
pub struct InvalidBool;

// Macro to implement FieldType for integer types
macro_rules! impl_fieldtype_for_int {
    ($($t:ty),*) => {
        $(
            impl<'a> FieldType<'a> for $t {
                type Error = InvalidInt;
                type SerializeSettings = ();

                fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
                where
                    B: Buffer,
                {
                    serialize_int(*self, buffer)
                }

                fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
                    str::from_utf8(data)
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .ok_or(InvalidInt)
                }

                fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
                    // For lossy parsing, try to parse as much as possible
                    let mut result = 0;
                    let mut sign = 1;
                    let mut idx = 0;

                    if data.is_empty() {
                        return Ok(0);
                    }

                    // Handle sign
                    if data[0] == b'-' {
                        sign = -1;
                        idx = 1;
                    } else if data[0] == b'+' {
                        idx = 1;
                    }

                    // Parse digits
                    while idx < data.len() && data[idx].is_ascii_digit() {
                        result = result * 10 + (data[idx] - b'0') as Self;
                        idx += 1;
                    }

                    Ok(result * sign as Self)
                }
            }
        )*
    };
}

// Implement for all integer types
impl_fieldtype_for_int!(u32, u64, u128, i32, i64, i128, usize, isize);

// Implement for u8 and u16 separately (can't use multiply with sign directly)
impl<'a> FieldType<'a> for u8 {
    type Error = InvalidInt;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
    where
        B: Buffer,
    {
        serialize_int(*self, buffer)
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        str::from_utf8(data)
            .ok()
            .and_then(|s| s.parse().ok())
            .ok_or(InvalidInt)
    }
}

impl<'a> FieldType<'a> for u16 {
    type Error = InvalidInt;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
    where
        B: Buffer,
    {
        serialize_int(*self, buffer)
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        str::from_utf8(data)
            .ok()
            .and_then(|s| s.parse().ok())
            .ok_or(InvalidInt)
    }
}

impl<'a> FieldType<'a> for i8 {
    type Error = InvalidInt;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
    where
        B: Buffer,
    {
        serialize_int(*self, buffer)
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        str::from_utf8(data)
            .ok()
            .and_then(|s| s.parse().ok())
            .ok_or(InvalidInt)
    }
}

impl<'a> FieldType<'a> for i16 {
    type Error = InvalidInt;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
    where
        B: Buffer,
    {
        serialize_int(*self, buffer)
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        str::from_utf8(data)
            .ok()
            .and_then(|s| s.parse().ok())
            .ok_or(InvalidInt)
    }
}

// Implement for floating point types
impl<'a> FieldType<'a> for f32 {
    type Error = InvalidFloat;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
    where
        B: Buffer,
    {
        serialize_float(*self, buffer)
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        str::from_utf8(data)
            .ok()
            .and_then(|s| s.parse().ok())
            .ok_or(InvalidFloat)
    }
}

impl<'a> FieldType<'a> for f64 {
    type Error = InvalidFloat;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
    where
        B: Buffer,
    {
        serialize_float(*self, buffer)
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        str::from_utf8(data)
            .ok()
            .and_then(|s| s.parse().ok())
            .ok_or(InvalidFloat)
    }
}

// Implement for bool
impl<'a> FieldType<'a> for bool {
    type Error = InvalidBool;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
    where
        B: Buffer,
    {
        buffer.extend_from_slice(if *self { b"Y" } else { b"N" });
        1
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        match data {
            b"Y" | b"y" | b"1" | b"true" | b"TRUE" | b"True" => Ok(true),
            b"N" | b"n" | b"0" | b"false" | b"FALSE" | b"False" => Ok(false),
            _ => Err(InvalidBool),
        }
    }
}

// Implement for &[u8]
impl<'a> FieldType<'a> for &'a [u8] {
    type Error = std::convert::Infallible;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
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

// Implement for Vec<u8>
impl<'a> FieldType<'a> for Vec<u8> {
    type Error = std::convert::Infallible;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
    where
        B: Buffer,
    {
        buffer.extend_from_slice(self);
        self.len()
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(data.to_vec())
    }
}

// Implement for String
impl<'a> FieldType<'a> for String {
    type Error = std::str::Utf8Error;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
    where
        B: Buffer,
    {
        buffer.extend_from_slice(self.as_bytes());
        self.len()
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        str::from_utf8(data).map(|s| s.to_string())
    }
}

// Implement for SmartString
impl<'a> FieldType<'a> for SmartString {
    type Error = std::str::Utf8Error;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
    where
        B: Buffer,
    {
        buffer.extend_from_slice(self.as_bytes());
        self.len()
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        str::from_utf8(data).map(|s| s.into())
    }
}

// Implement for &str
impl<'a> FieldType<'a> for &'a str {
    type Error = std::str::Utf8Error;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
    where
        B: Buffer,
    {
        buffer.extend_from_slice(self.as_bytes());
        self.len()
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        str::from_utf8(data)
    }
}

// Implement for fixed-size byte arrays
macro_rules! impl_fieldtype_for_array {
    ($($n:expr),*) => {
        $(
            impl<'a> FieldType<'a> for [u8; $n] {
                type Error = std::array::TryFromSliceError;
                type SerializeSettings = ();

                fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
                where
                    B: Buffer,
                {
                    buffer.extend_from_slice(self);
                    $n
                }

                fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
                    data.try_into()
                }
            }

            impl<'a> FieldType<'a> for &'a [u8; $n] {
                type Error = std::array::TryFromSliceError;
                type SerializeSettings = ();

                fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
                where
                    B: Buffer,
                {
                    buffer.extend_from_slice(*self);
                    $n
                }

                fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
                    data.try_into().map(|arr: &[u8; $n]| arr)
                }
            }
        )*
    };
}

// Implement for common array sizes
impl_fieldtype_for_array!(1, 2, 3, 4, 8, 16, 32, 64, 128, 256);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_serialization() {
        let mut buffer = Vec::new();
        assert_eq!(42u32.serialize(&mut buffer), 2);
        assert_eq!(&buffer[..], b"42");

        buffer.clear();
        assert_eq!((-123i32).serialize(&mut buffer), 4);
        assert_eq!(&buffer[..], b"-123");
    }

    #[test]
    fn test_bool_serialization() {
        let mut buffer = Vec::new();
        assert_eq!(true.serialize(&mut buffer), 1);
        assert_eq!(&buffer[..], b"Y");

        buffer.clear();
        assert_eq!(false.serialize(&mut buffer), 1);
        assert_eq!(&buffer[..], b"N");
    }

    #[test]
    fn test_byte_slice_serialization() {
        let mut buffer = Vec::new();
        let data: &[u8] = b"hello";
        assert_eq!(data.serialize(&mut buffer), 5);
        assert_eq!(&buffer[..], b"hello");
    }
}
