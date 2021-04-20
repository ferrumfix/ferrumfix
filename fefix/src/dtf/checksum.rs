use super::error;
use crate::dtf::DataField;
use crate::Buffer;

const LEN_IN_BYTES: usize = 3;

/// A [`CheckSum`] contains the result of a FIX checksum calculation.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CheckSum(pub u8);

impl CheckSum {
    /// Returns the [`CheckSum`] of `data`.
    pub fn compute(data: &[u8]) -> Self {
        let mut value = 0u8;
        for byte in data {
            value = value.wrapping_add(*byte);
        }
        Self(value)
    }
}

impl<'a> DataField<'a> for CheckSum {
    type Error = error::CheckSum;

    fn serialize<B>(&self, buffer: &mut B) -> usize
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
        if data.len() != LEN_IN_BYTES {
            Err(Self::Error::WrongLength)
        } else {
            Ok(checksum_from_digits(data))
        }
    }

    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != LEN_IN_BYTES {
            Err(Self::Error::WrongLength)
        } else if !is_ascii_digit(data[0]) || !is_ascii_digit(data[1]) || !is_ascii_digit(data[2]) {
            Err(Self::Error::NotAsciiDigits)
        } else {
            Ok(checksum_from_digits(data))
        }
    }
}

fn checksum_from_digits(data: &[u8]) -> CheckSum {
    CheckSum(
        ascii_digit_to_u8(data[0], 100)
            .wrapping_add(ascii_digit_to_u8(data[1], 10))
            .wrapping_add(ascii_digit_to_u8(data[2], 1)),
    )
}

fn is_ascii_digit(byte: u8) -> bool {
    byte >= b'0' && byte <= b'9'
}

fn digit_to_ascii(byte: u8) -> u8 {
    byte + b'0'
}

fn ascii_digit_to_u8(digit: u8, multiplier: u8) -> u8 {
    (digit.wrapping_sub(b'0')) * multiplier
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn edges_cases() {
        assert_eq!(CheckSum::compute(&[]).0, 0);
        assert_eq!(CheckSum::compute(&[1]).0, 1);
        assert_eq!(CheckSum::compute(&[128, 127]).0, 255);
        assert_eq!(CheckSum::compute(&[128, 128]).0, 0);
    }
}
