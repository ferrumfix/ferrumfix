use super::error;
use super::DataType;
use crate::Buffer;
use std::{
    hash::{Hash, Hasher},
    time::Duration,
};

const HOUR: u32 = 3600;
const MINUTE: u32 = 60;

/// Timezone indicator.
#[derive(Debug, Clone)]
pub struct Tz {
    sign: i32,
    offset: Duration,
    offset_has_minutes: bool,
    is_utc: bool,
}

impl Tz {
    pub const UTC: Self = Self {
        sign: 1,
        offset: Duration::from_secs(0),
        offset_has_minutes: false,
        is_utc: true,
    };

    pub fn offset(&self) -> (i32, Duration) {
        (self.sign, self.offset)
    }

    pub fn to_chrono_offset(&self) -> chrono::FixedOffset {
        chrono::FixedOffset::east(self.offset().1.as_secs() as i32)
    }
}

impl<'a> DataType<'a> for Tz {
    type Error = error::Timestamp;
    type SerializeSettings = ();

    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        if self.is_utc {
            buffer.extend_from_slice(b"Z");
            1
        } else {
            let sign = if self.offset().0 > 0 { b'+' } else { b'-' };
            let hour = self.offset().1.as_secs() as u32 / HOUR;
            buffer.extend_from_slice(&[
                sign,
                u32_digit_to_ascii(hour / 10),
                u32_digit_to_ascii(hour % 10),
            ]);
            if self.offset_has_minutes {
                let minute = self.offset().1.as_secs() as u32 / MINUTE;
                buffer.extend_from_slice(&[
                    b':',
                    u32_digit_to_ascii(minute / 10),
                    u32_digit_to_ascii(minute % 10),
                ]);
                6
            } else {
                3
            }
        }
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() == 0 {
            return Err(Self::Error::Other);
        }
        let sign: i32;
        match data[0] {
            b'Z' => {
                return Ok(Self::UTC);
            }
            b'+' => {
                sign = 1;
            }
            b'-' => {
                sign = -1;
            }
            _ => return Err(Self::Error::Other),
        }
        let hour = ascii_digit_to_u32(data[1], 10) + ascii_digit_to_u32(data[2], 1);
        let minute = ascii_digit_to_u32(data[4], 10) + ascii_digit_to_u32(data[5], 1);
        Ok(Self {
            sign,
            offset: Duration::from_secs((hour * HOUR + minute * MINUTE) as u64),
            is_utc: false,
            offset_has_minutes: false,
        })
    }

    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        match data.len() {
            1 => Ok(Self::UTC),
            3 => {
                let sign = if data[0] == b'+' { 1 } else { -1 };
                let hour = ascii_digit_to_u32(data[1], 10) + ascii_digit_to_u32(data[2], 1);
                Ok(Self {
                    sign,
                    offset: Duration::from_secs((hour * HOUR) as u64),
                    is_utc: false,
                    offset_has_minutes: false,
                })
            }
            6 => {
                let sign = if data[0] == b'+' { 1 } else { -1 };
                let hour = ascii_digit_to_u32(data[1], 10) + ascii_digit_to_u32(data[2], 1);
                let minute = ascii_digit_to_u32(data[4], 10) + ascii_digit_to_u32(data[5], 1);
                Ok(Self {
                    sign,
                    offset: Duration::from_secs((hour * HOUR + minute * MINUTE) as u64),
                    is_utc: false,
                    offset_has_minutes: false,
                })
            }
            _ => Err(Self::Error::Other),
        }
    }
}

impl PartialEq for Tz {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
    }
}

impl Eq for Tz {}

impl Hash for Tz {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.offset.hash(state);
    }
}

const fn u32_digit_to_ascii(digit: u32) -> u8 {
    digit as u8 + b'0'
}

const fn ascii_digit_to_u32(digit: u8, multiplier: u32) -> u32 {
    (digit as u32).wrapping_sub(b'0' as u32) * multiplier
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize_empty_is_err() {
        Tz::deserialize(b"").unwrap_err();
        Tz::deserialize_lossy(b"").unwrap_err();
    }

    #[test]
    fn utc_deserialize() {
        assert_eq!(Tz::deserialize(b"Z").unwrap(), Tz::UTC);
    }

    #[test]
    fn serialize_utc() {
        let buffer = &mut Vec::new();
        assert_eq!(Tz::UTC.serialize(buffer), 1);
        assert_eq!(&buffer[..], b"Z");
    }
}
