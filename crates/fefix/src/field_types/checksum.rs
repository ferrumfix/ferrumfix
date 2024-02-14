use crate::{Buffer, FieldType};
use std::convert::TryInto;

const LEN_IN_BYTES: usize = 3;

const ERR_LENGTH: &str = "Expected exactly three bytes for CheckSum.";
const ERR_ASCII_DIGITS: &str = "Expected ASCII digits, found invalid characters.";

/// The result of a FIX checksum calculation (0-255).
///
/// You generally shouldn't need to use [`CheckSum`] directly unless you're
/// building custom low-level FIX primitives (e.g. a new decoder).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CheckSum(pub u8);

impl CheckSum {
    /// Returns the [`CheckSum`] of `data`. The result is always the sum of each
    /// byte in `data` wrapped at `0xFF`, as per the FIX specification.
    pub fn compute(data: &[u8]) -> Self {
        let mut value = 0u8;
        for byte in data {
            value = value.wrapping_add(*byte);
        }
        Self(value)
    }
}

impl<'a> FieldType<'a> for CheckSum {
    type Error = &'static str;
    type SerializeSettings = ();

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
        let digits = data.try_into().map_err(|_| ERR_LENGTH)?;

        if is_ascii_digit(data[0]) & is_ascii_digit(data[1]) & is_ascii_digit(data[2]) {
            Ok(checksum_from_digits(digits))
        } else {
            Err(ERR_ASCII_DIGITS)
        }
    }

    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        let digits = data.try_into().map_err(|_| ERR_LENGTH)?;

        // Skip ASCII digits checking.
        Ok(checksum_from_digits(digits))
    }
}

fn checksum_from_digits(data: [u8; LEN_IN_BYTES]) -> CheckSum {
    let digit1 = ascii_digit_to_u8(data[0], 100);
    let digit2 = ascii_digit_to_u8(data[1], 10);
    let digit3 = ascii_digit_to_u8(data[2], 1);

    CheckSum(digit1.wrapping_add(digit2).wrapping_add(digit3))
}

fn is_ascii_digit(byte: u8) -> bool {
    byte.is_ascii_digit()
}

fn digit_to_ascii(byte: u8) -> u8 {
    byte + b'0'
}

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
        crate::field_types::test_utility_verify_serialization_behavior(checksum)
    }
}
