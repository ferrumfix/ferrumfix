use super::FixValue;
use crate::Buffer;
use std::hash::{Hash, Hasher};
use std::time::Duration;

const ERR_INVALID: &str = "Invalid timezone.";

const HOUR: u32 = 3600;
const MINUTE: u32 = 60;

/// Timezone indicator.
///
/// # Examples
///
/// ```
/// use fefix::FixValue;
/// use fefix::fix_values::Tz;
/// use std::time::Duration;
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
#[derive(Debug, Clone)]
pub struct Tz {
    sign: i32,
    offset: Duration,
    offset_has_minutes: bool,
    is_utc: bool,
}

impl Tz {
    /// The UTC timezone.
    pub const UTC: Self = Self {
        sign: 0,
        offset: Duration::from_secs(0),
        offset_has_minutes: false,
        is_utc: true,
    };

    /// Calculates the offset information of `self` as compared to UTC. The
    /// return value is in the form of a sign (-1, 0, or +1) and a [`Duration`].
    pub fn offset(&self) -> (i32, Duration) {
        (self.sign, self.offset)
    }

    #[cfg(feature = "utils-chrono")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "utils-chrono")))]
    pub fn to_chrono_offset(&self) -> chrono::FixedOffset {
        chrono::FixedOffset::east(self.offset().1.as_secs() as i32)
    }

    #[cfg(feature = "utils-chrono")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "utils-chrono")))]
    pub fn from_chrono_offset(offset: chrono::FixedOffset) -> Self {
        let local_minus_utc = offset.local_minus_utc();
        Self {
            sign: local_minus_utc.signum(),
            offset: Duration::from_secs(local_minus_utc.abs() as u64),
            offset_has_minutes: true,
            is_utc: local_minus_utc == 0,
        }
    }
}

impl<'a> FixValue<'a> for Tz {
    type Error = &'static str;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
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
            return Err(ERR_INVALID);
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
            _ => return Err(ERR_INVALID),
        }
        match data.len() {
            3 => {
                let hour = ascii_digit_to_u32(data[1], 10) + ascii_digit_to_u32(data[2], 1);
                Ok(Self {
                    sign,
                    offset: Duration::from_secs((hour * HOUR) as u64),
                    is_utc: false,
                    offset_has_minutes: false,
                })
            }
            6 => {
                let hour = ascii_digit_to_u32(data[1], 10) + ascii_digit_to_u32(data[2], 1);
                let minute = ascii_digit_to_u32(data[4], 10) + ascii_digit_to_u32(data[5], 1);
                Ok(Self {
                    sign,
                    offset: Duration::from_secs((hour * HOUR + minute * MINUTE) as u64),
                    is_utc: false,
                    offset_has_minutes: false,
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
            _ => Err(ERR_INVALID),
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
