use super::{FixValue, ZeroPadding, ERR_DECIMAL, ERR_INT_INVALID, ERR_UTF8};
use crate::{Buffer, BufferWriter, TagU16};
use std::fmt::Write;

const ERR_BOOL_LENGTH: &str = "Invalid length; a boolean is Y or N (1 char).";
const ERR_BOOL_CHAR: &str = "Invalid character for boolean. Only Y and N are valid.";

macro_rules! lossy_deserialize_unsigned {
    ($data:ident, $ty:ty) => {{
        fn ascii_digit_to_num(digit: u8) -> $ty {
            (digit as $ty).wrapping_sub(b'0' as $ty)
        }
        let mut n: $ty = 0;
        for byte in $data.iter().copied() {
            n = n.wrapping_mul(10).wrapping_add(ascii_digit_to_num(byte));
        }
        Ok(n)
    }};
}

macro_rules! lossy_deserialize_signed {
    ($data:ident, $ty:ty) => {{
        lossy_deserialize_unsigned!($data, $ty).map(|n| {
            if $data.get(0).copied() == Some(b'-' as u8) {
                n.wrapping_neg()
            } else {
                n
            }
        })
    }};
}

macro_rules! impl_unsigned {
    ($ty:ty) => {
        impl<'a> FixValue<'a> for $ty {
            type Error = &'static str;
            type SerializeSettings = ZeroPadding;

            #[inline]
            fn serialize_with<B>(&self, buffer: &mut B, padding: ZeroPadding) -> usize
            where
                B: Buffer,
            {
                let initial_len = buffer.len();
                write!(BufferWriter(buffer), "{:0width$}", self, width = padding.0).unwrap();
                buffer.len() - initial_len
            }

            #[inline]
            fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
                std::str::from_utf8(data)
                    .map_err(|_| ERR_UTF8)?
                    .parse()
                    .map_err(|_| ERR_INT_INVALID)
            }

            #[inline]
            fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
                lossy_deserialize_unsigned!(data, $ty)
            }
        }
    };
}

macro_rules! impl_signed {
    ($ty:ty) => {
        impl<'a> FixValue<'a> for $ty {
            type Error = &'static str;
            type SerializeSettings = ();

            #[inline]
            fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
            where
                B: Buffer,
            {
                let initial_len = buffer.len();
                write!(BufferWriter(buffer), "{}", self).unwrap();
                buffer.len() - initial_len
            }

            #[inline]
            fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
                std::str::from_utf8(data)
                    .map_err(|_| ERR_UTF8)?
                    .parse()
                    .map_err(|_| ERR_INT_INVALID)
            }

            #[inline]
            fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
                lossy_deserialize_signed!(data, $ty)
            }
        }
    };
}

macro_rules! impl_float {
    ($ty:ty) => {
        impl<'a> FixValue<'a> for $ty {
            type Error = &'static str;
            type SerializeSettings = ();

            #[inline]
            fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
            where
                B: Buffer,
            {
                let initial_len = buffer.len();
                write!(BufferWriter(buffer), "{}", self).unwrap();
                buffer.len() - initial_len
            }

            #[inline]
            fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
                std::str::from_utf8(data)
                    .map_err(|_| ERR_UTF8)?
                    .parse()
                    .map_err(|_| ERR_INT_INVALID)
            }
        }
    };
}

impl_unsigned!(u32);
impl_unsigned!(u64);
impl_unsigned!(u128);
impl_unsigned!(usize);

impl_signed!(i32);
impl_signed!(i64);
impl_signed!(i128);

impl_float!(f32);
impl_float!(f64);

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
        match data {
            b"Y" => Ok(true),
            b"N" => Ok(false),
            _ if data.len() == 1 => Err(ERR_BOOL_CHAR),
            _ => Err(ERR_BOOL_LENGTH),
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
