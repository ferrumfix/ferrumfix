use super::error;
use super::FixFieldValue;
use crate::Buffer;
use std::time::Duration;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TzTime {
    hour: u32,
    minute: u32,
    second: u32,
    second_is_explicit: bool,
    tz_is_explicit: bool,
    tz_offset_hour: i32,
    tz_offset_minute: i32,
    tz_offset_minute_is_explicit: bool,
}

impl TzTime {
    /// Returns the hour of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::datatypes::{DataType, TzTime};
    ///
    /// let dtf = TzTime::deserialize(b"12:45:00Z").unwrap();
    /// assert_eq!(dtf.minute(), 45);
    /// ```
    pub fn hour(&self) -> u32 {
        self.hour
    }

    pub fn minute(&self) -> u32 {
        self.minute
    }

    pub fn second(&self) -> u32 {
        self.second
    }

    pub fn tz_offset(&self) -> Duration {
        Duration::from_secs((self.tz_offset_hour * 3600 + self.tz_offset_minute * 60) as u64)
    }
}

impl<'a> FixFieldValue<'a> for TzTime {
    type Error = error::Time;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    fn serialize<B>(&self, buffer: &mut B) -> usize
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
            b'+',
            self.tz_offset_hour as u8 / 10,
            self.tz_offset_hour as u8 % 10,
            b':',
            self.tz_offset_minute as u8 / 10,
            self.tz_offset_minute as u8 % 10,
        ];
        buffer.extend_from_slice(bytes);
        bytes.len()
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        fn parse_timezone(data: &[u8]) -> Option<(bool, i32, i32)> {
            match data[0] {
                // FIXME
                b'Z' => Some((true, 0, 0)),
                b'+' => Some((false, 0, 0)),
                b'-' => Some((false, 0, 0)),
                _ => None,
            }
        }
        if data.len() < 6 || data[2] != b':' {
            return Err(Self::Error::Other);
        }
        let hour = ascii_digit_to_u32(data[0], 10) + ascii_digit_to_u32(data[1], 1);
        let minute = ascii_digit_to_u32(data[3], 10) + ascii_digit_to_u32(data[4], 1);
        match data[5] {
            b':' => {
                let second = ascii_digit_to_u32(data[6], 10) + ascii_digit_to_u32(data[7], 1);
                let (is_utc, tz_offset_hour, tz_offset_minute) =
                    parse_timezone(&data[8..]).ok_or(Self::Error::Other)?;
                Ok(TzTime {
                    hour,
                    minute,
                    second,
                    second_is_explicit: true,
                    tz_is_explicit: !is_utc,
                    tz_offset_hour,
                    tz_offset_minute,
                    tz_offset_minute_is_explicit: false,
                })
            }
            _ => {
                let (is_utc, tz_offset_hour, tz_offset_minute) =
                    parse_timezone(&data[5..]).ok_or(Self::Error::Other)?;
                Ok(TzTime {
                    hour,
                    minute,
                    second: 0,
                    second_is_explicit: false,
                    tz_is_explicit: !is_utc,
                    tz_offset_hour,
                    tz_offset_minute,
                    tz_offset_minute_is_explicit: false,
                })
            }
        }
    }
}

const fn ascii_digit_to_u32(digit: u8, multiplier: u32) -> u32 {
    (digit as u32).wrapping_sub(b'0' as u32) * multiplier
}

#[cfg(test)]
mod test {
    use super::FixFieldValue;
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
