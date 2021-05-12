use super::error;
use crate::Buffer;
use crate::FixValue;

const LEN_IN_BYTES_NO_MILLI: usize = 8;
const LEN_IN_BYTES_WITH_MILLI: usize = 12;

const MAX_HOUR: u32 = 23;
const MAX_MINUTE: u32 = 59;
const MAX_SECOND: u32 = 60; // Leap seconds.
const MAX_MILLISECOND: u32 = 999;

const MIN_HOUR: u32 = 0;
const MIN_MINUTE: u32 = 0;
const MIN_SECOND: u32 = 0;
const MIN_MILLISECOND: u32 = 0;

/// Canonical data field (DTF) for
/// [`FixDataType::UtcTimeOnly`](crate::dict::FixDataType::UtcTimeOnly).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    hour: u32,
    minute: u32,
    second: u32,
    milli: u32,
}

impl Time {
    /// Creates a new time value from its components, with milliseconds.
    pub fn from_hmsm(hour: u32, minute: u32, second: u32, milli: u32) -> Option<Self> {
        if hour >= MIN_HOUR
            && hour <= MAX_HOUR
            && minute >= MIN_MINUTE
            && minute <= MAX_MINUTE
            && second >= MIN_SECOND
            && second <= MAX_SECOND
            && milli >= MIN_MILLISECOND
            && milli <= MAX_MILLISECOND
        {
            Some(Self {
                hour,
                minute,
                second,
                milli,
            })
        } else {
            None
        }
    }

    pub const fn to_bytes(&self) -> [u8; LEN_IN_BYTES_WITH_MILLI] {
        [
            (self.hour() / 10) as u8 + b'0',
            (self.hour() % 10) as u8 + b'0',
            b':',
            (self.minute() / 10) as u8 + b'0',
            (self.minute() % 10) as u8 + b'0',
            b':',
            (self.second() / 10) as u8 + b'0',
            (self.second() % 10) as u8 + b'0',
            b'.',
            (self.milli() / 100) as u8 + b'0',
            ((self.milli() / 10) % 10) as u8 % 10 + b'0',
            (self.milli() % 10) as u8 + b'0',
        ]
    }

    /// Returns the hour of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::prelude::*;
    /// use fefix::tagvalue::datatypes::Time;
    ///
    /// let dtf = Time::deserialize(b"12:45:00").unwrap();
    /// assert_eq!(dtf.hour(), 12)
    /// ```
    pub const fn hour(&self) -> u32 {
        self.hour
    }

    /// Returns the hour of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::prelude::*;
    /// use fefix::tagvalue::datatypes::Time;
    ///
    /// let dtf = Time::deserialize(b"12:45:00").unwrap();
    /// assert_eq!(dtf.minute(), 45)
    /// ```
    pub const fn minute(&self) -> u32 {
        self.minute
    }

    /// Returns the second of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::prelude::*;
    /// use fefix::tagvalue::datatypes::Time;
    ///
    /// let dtf = Time::deserialize(b"12:45:00").unwrap();
    /// assert_eq!(dtf.minute(), 45)
    /// ```
    ///
    /// Leap second:
    ///
    /// ```
    /// use fefix::prelude::*;
    /// use fefix::tagvalue::datatypes::Time;
    ///
    /// let dtf = Time::deserialize(b"23:59:60").unwrap();
    /// assert_eq!(dtf.second(), 60)
    /// ```
    pub const fn second(&self) -> u32 {
        self.second
    }

    /// Returns the milliecond of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::prelude::*;
    /// use fefix::tagvalue::datatypes::Time;
    ///
    /// let dtf = Time::deserialize(b"12:45:00.328").unwrap();
    /// assert_eq!(dtf.milli(), 328)
    /// ```
    pub const fn milli(&self) -> u32 {
        self.milli
    }

    #[cfg(feature = "utils-chrono")]
    pub fn to_chrono_naive(&self) -> Option<chrono::NaiveTime> {
        chrono::NaiveTime::from_hms_milli_opt(
            self.hour(),
            self.minute(),
            self.second(),
            self.milli(),
        )
    }
}

impl<'a> FixValue<'a> for Time {
    type Error = error::Time;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let bytes = self.to_bytes();
        buffer.extend_from_slice(&bytes[..]);
        bytes.len()
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let mut milli = 0;
        if data.len() == LEN_IN_BYTES_WITH_MILLI {
            milli = ascii_digit_to_u32(data[9], 100)
                + ascii_digit_to_u32(data[10], 10)
                + ascii_digit_to_u32(data[11], 1);
            if data[8] != b'.' {
                return Err(Self::Error::Other);
            }
        } else if data.len() != LEN_IN_BYTES_NO_MILLI {
            return Err(Self::Error::Other);
        }
        let digits_are_ok = data[2] == b':'
            && data[5] == b':'
            && is_ascii_digit(data[0])
            && is_ascii_digit(data[1])
            && is_ascii_digit(data[3])
            && is_ascii_digit(data[4])
            && is_ascii_digit(data[6])
            && is_ascii_digit(data[7]);
        if !digits_are_ok {
            return Err(Self::Error::Other);
        }
        let hour = ascii_digit_to_u32(data[0], 10) + ascii_digit_to_u32(data[1], 1);
        let minute = ascii_digit_to_u32(data[3], 10) + ascii_digit_to_u32(data[4], 1);
        let second = ascii_digit_to_u32(data[6], 10) + ascii_digit_to_u32(data[7], 1);
        Self::from_hmsm(hour, minute, second, milli).ok_or(Self::Error::Other)
    }
}

const fn is_ascii_digit(byte: u8) -> bool {
    byte >= b'0' && byte <= b'9'
}

const fn ascii_digit_to_u32(digit: u8, multiplier: u32) -> u32 {
    (digit as u32).wrapping_sub(b'0' as u32) * multiplier
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    impl Arbitrary for Time {
        fn arbitrary(g: &mut Gen) -> Self {
            let hour = u32::arbitrary(g) % 24;
            let minute = u32::arbitrary(g) % 60;
            let second = u32::arbitrary(g) % 60;
            let millisecond = if bool::arbitrary(g) {
                format!(".{:03}", u32::arbitrary(g) % 1000)
            } else {
                String::new()
            };
            let s = format!("{:02}:{:02}:{:02}{}", hour, minute, second, millisecond);
            Self::deserialize(s.as_bytes()).unwrap()
        }
    }

    struct TestCase {
        bytes: &'static [u8],
        hour: u32,
        minute: u32,
        second: u32,
        milli: u32,
    }

    impl TestCase {
        const fn new(
            bytes: &'static [u8],
            hour: u32,
            minute: u32,
            second: u32,
            milli: u32,
        ) -> Self {
            Self {
                bytes,
                hour,
                minute,
                second,
                milli,
            }
        }
    }

    const VALID_TEST_CASES: &[TestCase] = &[
        TestCase::new(b"00:00:00", 0, 0, 0, 0),
        TestCase::new(b"00:00:00.123", 0, 0, 0, 123),
        TestCase::new(b"12:00:00", 12, 0, 0, 0),
        TestCase::new(b"23:59:60", 23, 59, 60, 0),
    ];

    #[test]
    fn valid_test_cases() {
        for test_case in VALID_TEST_CASES {
            let dtf = Time::deserialize(test_case.bytes).unwrap();
            assert_eq!(dtf.hour(), test_case.hour);
            assert_eq!(dtf.minute(), test_case.minute);
            assert_eq!(dtf.second(), test_case.second);
            assert_eq!(dtf.milli(), test_case.milli);
        }
    }

    #[quickcheck]
    fn verify_serialization_behavior(time: Time) -> bool {
        super::super::verify_serialization_behavior(time)
    }
}
