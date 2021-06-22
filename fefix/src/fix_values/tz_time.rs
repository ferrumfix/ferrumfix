use super::{FixValue, Tz};
use crate::Buffer;

const ERR_INVALID: &str = "Invalid time.";

/// Timezone-aware intra-day timestamp.
///
/// # Examples
///
/// ```
/// use fefix::FixValue;
/// use fefix::fix_values::{Tz, TzTime};
///
/// let tztime = TzTime::deserialize(b"07:39:20Z").unwrap();
/// assert_eq!(tztime.hour(), 7);
/// assert_eq!(tztime.minute(), 39);
/// assert_eq!(tztime.second(), 20);
/// assert_eq!(tztime.timezone(), Tz::UTC);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TzTime {
    hour: u32,
    minute: u32,
    second: u32,
    second_is_explicit: bool,
    tz: Tz,
}

impl TzTime {
    /// Returns the hour of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::prelude::*;
    /// use fefix::fix_values::TzTime;
    ///
    /// let dtf = TzTime::deserialize(b"12:45:00Z").unwrap();
    /// assert_eq!(dtf.minute(), 45);
    /// ```
    pub fn hour(&self) -> u32 {
        self.hour
    }

    /// Returns the minutes' portion of `self`. This goes from 0 to 59
    /// (including), but can go up to 60 for leap seconds.
    pub fn minute(&self) -> u32 {
        self.minute
    }

    /// Returns the seconds' portion of `self`. This goes from 0 to 59
    /// (including), but can go up to 60 for leap seconds.
    pub fn second(&self) -> u32 {
        self.second
    }

    /// Returns the timezone ([`Tz`]) of `self`.
    pub fn timezone(&self) -> Tz {
        self.tz
    }
}

impl<'a> FixValue<'a> for TzTime {
    type Error = &'static str;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let bytes = &[
            self.hour() as u8 / 10,
            self.hour() as u8 % 10,
            b':',
            self.minute() as u8 / 10,
            self.minute() as u8 % 10,
            b':',
            self.second() as u8 / 10,
            self.second() as u8 % 10,
        ];
        buffer.extend_from_slice(bytes);
        bytes.len() + self.timezone().serialize(buffer)
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 6 || data[2] != b':' {
            return Err(ERR_INVALID);
        }
        let hour = ascii_digit_to_u32(data[0], 10) + ascii_digit_to_u32(data[1], 1);
        let minute = ascii_digit_to_u32(data[3], 10) + ascii_digit_to_u32(data[4], 1);
        match data[5] {
            b':' => {
                let second = ascii_digit_to_u32(data[6], 10) + ascii_digit_to_u32(data[7], 1);
                Ok(TzTime {
                    hour,
                    minute,
                    second,
                    second_is_explicit: true,
                    tz: Tz::deserialize(&data[8..]).map_err(|_| ERR_INVALID)?,
                })
            }
            _ => Ok(TzTime {
                hour,
                minute,
                second: 0,
                second_is_explicit: false,
                tz: Tz::deserialize(&data[5..]).map_err(|_| ERR_INVALID)?,
            }),
        }
    }
}

const fn ascii_digit_to_u32(digit: u8, multiplier: u32) -> u32 {
    (digit as u32).wrapping_sub(b'0' as u32) * multiplier
}

#[cfg(test)]
mod test {
    use super::FixValue;
    use super::*;

    struct TestCase {
        bytes: &'static [u8],
        hour: u32,
        minute: u32,
        second: u32,
    }

    impl TestCase {
        const fn new(s: &'static str, hour: u32, minute: u32, second: u32) -> Self {
            Self {
                bytes: s.as_bytes(),
                hour,
                minute,
                second,
            }
        }
    }

    const TEST_CASES: &[TestCase] = &[
        TestCase::new("07:39Z", 7, 39, 0),
        TestCase::new("02:39-05", 2, 39, 0),
        TestCase::new("15:39+08", 15, 39, 0),
        TestCase::new("13:09+05:30", 13, 9, 0),
    ];

    #[test]
    fn valid_test_cases() {
        for test_case in TEST_CASES.iter() {
            let tz_time = TzTime::deserialize(test_case.bytes).unwrap();
            assert_eq!(tz_time.hour(), test_case.hour);
            assert_eq!(tz_time.minute(), test_case.minute);
            assert_eq!(tz_time.second(), test_case.second);
        }
    }
}
