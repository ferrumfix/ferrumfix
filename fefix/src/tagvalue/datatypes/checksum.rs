use crate::Buffer;
use crate::FixFieldValue;
use std::convert::TryInto;

const LEN_IN_BYTES: usize = 3;

const ERR_LENGTH: &str = "Expected exactly three bytes for CheckSum.";
const ERR_ASCII_DIGITS: &str = "Expected ASCII digits, found invalid characters.";

/// The result of a FIX checksum calculation.
///
/// [`CheckSum`] implements [`FixFieldValue`] as a zero-padded, unsigned integer
/// field of three bytes.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CheckSum(pub u8);

impl CheckSum {
    /// Returns the [`CheckSum`] of `data`. The result is always the sum of each
    /// byte in `data` wrapped at 0xFF, as per the FIX specification.
    pub fn compute(data: &[u8]) -> Self {
        let mut value = 0u8;
        for byte in data {
            value = value.wrapping_add(*byte);
        }
        Self(value)
    }
}

impl<'a> FixFieldValue<'a> for CheckSum {
    type Error = &'static str;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        buffer.extend_from_slice(&[
            digit_to_ascii(self.0 / 100),
            digit_to_ascii((self.0 / 10) % 10),
            digit_to_ascii(self.0 % 10),
        ]);
        LEN_IN_BYTES
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        if let Ok(digits) = data.try_into() {
            if is_ascii_digit(data[0]) & is_ascii_digit(data[1]) & is_ascii_digit(data[2]) {
                Ok(checksum_from_digits(digits))
            } else {
                Err(ERR_ASCII_DIGITS)
            }
        } else {
            Err(ERR_LENGTH)
        }
    }

    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        // Skip ASCII digits checking.
        if let Ok(digits) = data.try_into() {
            Ok(checksum_from_digits(digits))
        } else {
            Err(ERR_LENGTH)
        }
    }
}

fn checksum_from_digits(data: [u8; LEN_IN_BYTES]) -> CheckSum {
    CheckSum(
        ascii_digit_to_u8(data[0], 100)
            .wrapping_add(ascii_digit_to_u8(data[1], 10))
            .wrapping_add(ascii_digit_to_u8(data[2], 1)),
    )
}

#[inline(always)]
fn is_ascii_digit(byte: u8) -> bool {
    byte >= b'0' && byte <= b'9'
}

#[inline(always)]
fn digit_to_ascii(byte: u8) -> u8 {
    byte + b'0'
}

#[inline(always)]
fn ascii_digit_to_u8(digit: u8, multiplier: u8) -> u8 {
    digit.wrapping_sub(b'0').wrapping_mul(multiplier)
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    impl Arbitrary for CheckSum {
        fn arbitrary(g: &mut Gen) -> Self {
            Self(u8::arbitrary(g))
        }
    }

    #[test]
    fn edges_cases() {
        assert_eq!(CheckSum::compute(&[]).0, 0);
        assert_eq!(CheckSum::compute(&[1]).0, 1);
        assert_eq!(CheckSum::compute(&[128, 127]).0, 255);
        assert_eq!(CheckSum::compute(&[128, 128]).0, 0);
        assert_eq!(CheckSum::compute(&[128, 129]).0, 1);
    }

    #[quickcheck]
    fn serialized_takes_three_bytes(checksum: CheckSum) -> bool {
        checksum.to_bytes().len() == 3
    }

    #[quickcheck]
    fn serializes_as_integer(checksum: CheckSum) -> bool {
        let serialized_str = checksum.to_string();
        serialized_str.parse::<u8>().unwrap() == checksum.0
    }

    #[quickcheck]
    fn verify_serialization_behavior(checksum: CheckSum) -> bool {
        super::super::verify_serialization_behavior(checksum)
    }
}
