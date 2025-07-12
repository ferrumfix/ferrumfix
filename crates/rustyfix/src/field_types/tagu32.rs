use super::{ERR_INT_INVALID, ERR_UTF8};
use crate::{Buffer, BufferWriter, FieldType, TagU32};
use std::fmt::Write;

impl<'a> FieldType<'a> for TagU32 {
    type Error = &'static str;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let initial_len = buffer.len();
        write!(BufferWriter(buffer), "{self}").unwrap();
        buffer.len() - initial_len
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(data)
            .map_err(|_| ERR_UTF8)?
            .parse()
            .map_err(|_| ERR_INT_INVALID)
    }

    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        // Parse number loosely, accepting partial valid digits
        let mut result = 0u32;
        let mut found_digit = false;

        for &byte in data {
            if byte.is_ascii_digit() {
                result = result
                    .saturating_mul(10)
                    .saturating_add((byte - b'0') as u32);
                found_digit = true;
            } else if found_digit {
                // Stop at first non-digit after finding digits
                break;
            }
        }

        if !found_digit {
            return Err(ERR_INT_INVALID);
        }

        Ok(TagU32::new(result.max(1)).unwrap())
    }
}
