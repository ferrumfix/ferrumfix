use crate::dtf::CheckSum;
use crate::tagvalue::DecodeError;
use std::convert::TryInto;

// A tag-value message can't possibly be shorter than this.
//
//   8=?|9=?|35=?|10=???|
pub const MIN_FIX_MESSAGE_LEN_IN_BYTES: usize = 20;

/// The checksum field is composed of:
///  - `10=`       (3 characters)
///  - `XYZ`       (checksum value, always 3 characters)
///  - separator   (1 character)
/// Total: 7 characters.
pub const FIELD_CHECKSUM_LEN_IN_BYTES: usize = 7;

/// Parses an `u8` from `digits` and returns the result.
///
/// No error detection is performed. Values less than 100 must be zero-padded.
pub fn parse_u8_from_decimal(digits: [u8; 3]) -> u8 {
    digits[0]
        .wrapping_sub(b'0')
        .wrapping_mul(100)
        .wrapping_add(digits[1].wrapping_sub(b'0').wrapping_mul(10))
        .wrapping_add(digits[2].wrapping_sub(b'0'))
}

/// Returns a copy of the `CheckSum <10>` digits of `message`.
pub fn checksum_digits(message: &[u8]) -> [u8; 3] {
    debug_assert!(message.len() >= MIN_FIX_MESSAGE_LEN_IN_BYTES);
    message[message.len() - 4..message.len() - 1]
        .try_into()
        .unwrap()
}

pub fn verify_checksum(message: &[u8]) -> Result<(), DecodeError> {
    let nominal_checksum = parse_u8_from_decimal(checksum_digits(message));
    let actual_checksum =
        CheckSum::compute(&message[..message.len() - FIELD_CHECKSUM_LEN_IN_BYTES]).0;
    if nominal_checksum != actual_checksum {
        Err(DecodeError::CheckSum)
    } else {
        Ok(())
    }
}

/// Verifies the `BodyLength(9)` field of the FIX message in `data`.
pub fn verify_body_length(
    data: &[u8],
    start_of_body: usize,
    nominal_body_length: usize,
) -> Result<(), DecodeError> {
    let body_length = data
        .len()
        .wrapping_sub(FIELD_CHECKSUM_LEN_IN_BYTES)
        .wrapping_sub(start_of_body);
    let end_of_body = data.len() - FIELD_CHECKSUM_LEN_IN_BYTES;
    if start_of_body > end_of_body || nominal_body_length != body_length {
        dbglog!(
            "BodyLength mismatch: expected {} but is {}.",
            body_length,
            nominal_body_length,
        );
        Err(DecodeError::Invalid)
    } else {
        debug_assert!(body_length < data.len());
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_retrieval_of_checksum_digits() {
        assert_eq!(
            &checksum_digits(b"8=FIX.4.4|9=1337|35=?|...|10=000|"),
            b"000"
        );
        assert_eq!(
            &checksum_digits(b"8=FIX.4.4|9=1337|35=?|...|10=ABC|"),
            b"ABC"
        );
    }
}
