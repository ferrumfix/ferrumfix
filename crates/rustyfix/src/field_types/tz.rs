#![cfg_attr(docsrs, feature(doc_cfg))]
use crate::{Buffer, FieldType};
use std::hash::Hash;
use quanta::Duration;

const ERR_INVALID: &str = "Invalid timezone.";

const HOUR: u32 = 3600;
const MINUTE: u32 = 60;

/// Timezone indicator.
///
/// # Examples
///
/// ```
/// use rustyfix::FieldType;
/// use rustyfix::field_types::Tz;
/// use quanta::Duration;
///
/// let timezone = Tz::deserialize(b"Z").unwrap();
/// assert_eq!(timezone, Tz::UTC);
/// assert_eq!(timezone.offset().1, Duration::from_secs(0));
///
/// let timezone = Tz::deserialize(b"+03").unwrap();
/// assert_eq!(timezone.offset(), (1, Duration::from_secs(3 * 3600)));
///
/// let timezone = Tz::deserialize(b"-01").unwrap();
/// assert_eq!(timezone.offset(), (-1, Duration::from_secs(1 * 3600)));
///
/// let timezone = Tz::deserialize(b"+04:30").unwrap();
/// assert_eq!(timezone.offset(), (1, Duration::from_secs(4 * 3600 + 30 * 60)));
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Tz {
    offset_from_utc_in_seconds: i32,
}

impl Tz {
    /// The UTC timezone.
    pub const UTC: Self = Self {
        offset_from_utc_in_seconds: 0,
    };

    /// Calculates the offset information of `self` as compared to UTC. The
    /// return value is in the form of a sign (-1, 0, or +1) and a [`Duration`].
    pub fn offset(&self) -> (i32, Duration) {
        (
            self.offset_from_utc_in_seconds.signum(),
            Duration::from_secs(self.offset_from_utc_in_seconds.unsigned_abs() as u64),
        )
    }

    /// Returns the raw offset from UTC of `self` measured in seconds.
    pub fn offset_as_secs(&self) -> i32 {
        self.offset_from_utc_in_seconds
    }

    /// Converts `self` into a [`chrono::FixedOffset`].
    ///
    /// # Panics
    /// This function will panic if the offset is out of bounds. This should
    /// never happen, as the offset is validated during deserialization.
    #[cfg(feature = "utils-chrono")]
    pub fn to_chrono_offset(&self) -> chrono::FixedOffset {
        // unwrap(): we already verified that the offset is within bounds during
        // deserialization
        chrono::FixedOffset::east_opt(self.offset().1.as_secs() as i32).unwrap()
    }

    /// Creates a [`Tz`] from a [`chrono::FixedOffset`].
    #[cfg(feature = "utils-chrono")]
    pub fn from_chrono_offset(offset: chrono::FixedOffset) -> Self {
        Self {
            offset_from_utc_in_seconds: offset.local_minus_utc(),
        }
    }
}

impl<'a> FieldType<'a> for Tz {
    type Error = &'static str;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        if self.offset_from_utc_in_seconds == 0 {
            buffer.extend_from_slice(b"Z");
            1
        } else {
            let sign = if self.offset_from_utc_in_seconds > 0 {
                b'+'
            } else {
                b'-'
            };
            let hour = self.offset().1.as_secs() as u32 / HOUR;
            buffer.extend_from_slice(&[
                sign,
                u32_digit_to_ascii(hour / 10),
                u32_digit_to_ascii(hour % 10),
            ]);
            let minutes = (self.offset().1.as_secs() as u32 % 3600) / 60;
            if minutes != 0 {
                buffer.extend_from_slice(&[
                    b':',
                    u32_digit_to_ascii(minutes / 10),
                    u32_digit_to_ascii(minutes % 10),
                ]);
                6
            } else {
                3
            }
        }
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.is_empty() {
            return Err(ERR_INVALID);
        }
        let sign = match data[0] {
            b'Z' => {
                return Ok(Self::UTC);
            }
            b'+' => 1,
            b'-' => -1,
            _ => return Err(ERR_INVALID),
        };
        match data.len() {
            3 => {
                let hour = ascii_digit_to_u32(data[1], 10) + ascii_digit_to_u32(data[2], 1);
                Ok(Self {
                    offset_from_utc_in_seconds: sign * (hour * HOUR) as i32,
                })
            }
            6 => {
                let hour = ascii_digit_to_u32(data[1], 10) + ascii_digit_to_u32(data[2], 1);
                let minute = ascii_digit_to_u32(data[4], 10) + ascii_digit_to_u32(data[5], 1);
                Ok(Self {
                    offset_from_utc_in_seconds: sign * (hour * HOUR + minute * MINUTE) as i32,
                })
            }
            _ => Err(ERR_INVALID),
        }
    }

    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        match data.len() {
            1 => Ok(Self::UTC),
            3 => {
                let sign = if data[0] == b'+' { 1 } else { -1 };
                let hour = ascii_digit_to_u32(data[1], 10) + ascii_digit_to_u32(data[2], 1);
                Ok(Self {
                    offset_from_utc_in_seconds: sign * (hour * HOUR) as i32,
                })
            }
            6 => {
                let sign = if data[0] == b'+' { 1 } else { -1 };
                let hour = ascii_digit_to_u32(data[1], 10) + ascii_digit_to_u32(data[2], 1);
                let minute = ascii_digit_to_u32(data[4], 10) + ascii_digit_to_u32(data[5], 1);
                Ok(Self {
                    offset_from_utc_in_seconds: sign * (hour * HOUR + minute * MINUTE) as i32,
                })
            }
            _ => Err(ERR_INVALID),
        }
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
    fn utc() {
        assert_eq!(Tz::deserialize(b"Z").unwrap(), Tz::UTC);
        assert_eq!(&Tz::UTC.to_bytes()[..], "Z".as_bytes());
        assert_eq!(Tz::UTC.offset_as_secs(), 0);
    }

    #[test]
    fn negative_with_minutes() {
        let tz = Tz::deserialize(b"-03:30").unwrap();
        assert_eq!(&tz.to_bytes()[..], "-03:30".as_bytes());
    }

    #[test]
    fn negative_without_minutes() {
        let tz = Tz::deserialize(b"-01").unwrap();
        assert_eq!(&tz.to_bytes()[..], "-01".as_bytes());
        assert_eq!(tz.offset_as_secs(), -3600);
    }
}
