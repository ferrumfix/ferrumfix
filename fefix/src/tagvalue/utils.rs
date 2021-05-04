use super::datatypes::CheckSum;
use crate::tagvalue::datatypes::FixFieldValue;
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

/// Returns a copy of the `CheckSum <10>` digits of `message`.
pub fn checksum_digits(message: &[u8]) -> [u8; 3] {
    debug_assert!(message.len() >= MIN_FIX_MESSAGE_LEN_IN_BYTES);
    message[message.len() - 4..message.len() - 1]
        .try_into()
        .unwrap()
}

pub fn verify_checksum(headerless_msg: &[u8]) -> Result<(), DecodeError> {
    let msg_contents =
        &headerless_msg[..headerless_msg.len() - FIELD_CHECKSUM_LEN_IN_BYTES];
    let nominal_checksum =
        CheckSum::deserialize_lossy(&checksum_digits(headerless_msg)[..])
            .map_err(|_| DecodeError::CheckSum)?;
    let actual_checksum = CheckSum::compute(msg_contents);
    if nominal_checksum == actual_checksum {
        Ok(())
    } else {
        Err(DecodeError::CheckSum)
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
