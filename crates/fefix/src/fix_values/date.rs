use crate::Buffer;
use crate::FixValue;
use std::convert::{TryFrom, TryInto};

const LEN_IN_BYTES: usize = 8;

const MAX_YEAR: u32 = 9999;
const MAX_MONTH: u32 = 12;
const MAX_DAY: u32 = 31;

const MIN_YEAR: u32 = 0;
const MIN_MONTH: u32 = 1;
const MIN_DAY: u32 = 1;

const ERR_NOT_ASCII_DIGITS: &str = "Invalid characters, expected ASCII digits.";
const ERR_LENGTH: &str = "Invalid length, expected 8 bytes (YYYYMMDD format).";
const ERR_BOUNDS: &str = "Values outside legal bounds.";

/// Representation for `LocalMktDate` and and `UTCDateOnly` in `YYYYMMDD` format.
///
/// # Examples
///
/// - `19411208`
/// - `16201211`
/// - `18630101`
/// - `20170526`
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    year: u32,
    month: u32,
    day: u32,
}

impl Date {
    /// Creates a new date from its components. It returns `None` if any of the
    /// three components is outside the legal range.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::fix_values::Date;
    ///
    /// assert!(Date::new(2021, 4, 16).is_some());
    /// assert!(Date::new(2021, 13, 32).is_none());
    ///
    /// // Support from January 1, year zero (which doesn't actually exist) to
    /// // December 31, 9999.
    /// assert!(Date::new(0, 1, 1).is_some());
    /// assert!(Date::new(9999, 12, 31).is_some());
    ///
    /// // We don't check month-aware day boundaries, i.e. go ahead and assume
    /// // every month has 31 days.
    /// assert!(Date::new(2021, 2, 31).is_some());
    /// ```
    pub fn new(year: u32, month: u32, day: u32) -> Option<Self> {
        if year >= MIN_YEAR
            && year <= MAX_YEAR
            && month >= MIN_MONTH
            && month <= MAX_MONTH
            && day >= MIN_DAY
            && day <= MAX_DAY
        {
            Some(Self { year, month, day })
        } else {
            None
        }
    }

    /// Converts `self` to `YYYYMMDD` format.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::fix_values::Date;
    ///
    /// assert_eq!(&Date::new(2021, 01, 01).unwrap().to_bytes(), b"20210101");
    /// ```
    pub fn to_yyyymmdd(&self) -> [u8; LEN_IN_BYTES] {
        fn digit_to_ascii(n: u32) -> u8 {
            (n + b'0' as u32) as u8
        }
        [
            digit_to_ascii(self.year() / 1000),
            digit_to_ascii((self.year() / 100) % 10),
            digit_to_ascii((self.year() / 10) % 10),
            digit_to_ascii(self.year() % 10),
            digit_to_ascii(self.month() / 10),
            digit_to_ascii(self.month() % 10),
            digit_to_ascii(self.day() / 10),
            digit_to_ascii(self.day() % 10),
        ]
    }

    /// Returns the `year` of `self`.
    pub fn year(&self) -> u32 {
        self.year
    }

    /// Returns the `month` of `self` (1-indexing, i.e. 1-12).
    pub fn month(&self) -> u32 {
        self.month
    }

    /// Returns the `day` of `self` (1-indexing, i.e. 1-31).
    pub fn day(&self) -> u32 {
        self.day
    }

    /// Converts `self` to a [`chrono`] UTC date. [`chrono`] might impose
    /// additional constraints and checks on date components (e.g. leap year,
    /// day 31 in 30-day months); this function will return `None` for invalid
    /// dates.
    #[cfg(feature = "utils-chrono")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "utils-chrono")))]
    pub fn to_chrono_utc(&self) -> Option<chrono::Date<chrono::Utc>> {
        let naive = self.to_chrono_naive()?;
        Some(chrono::Date::from_utc(naive, chrono::Utc))
    }

    /// Converts `self` to [`chrono::NaiveDate`]. [`chrono`] might impose
    /// additional constraints and checks on date components (e.g. leap year,
    /// day 31 in 30-day months); this function will return `None` for invalid
    /// dates.
    #[cfg(feature = "utils-chrono")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "utils-chrono")))]
    pub fn to_chrono_naive(&self) -> Option<chrono::NaiveDate> {
        chrono::NaiveDate::from_ymd_opt(self.year() as i32, self.month(), self.day())
    }
}

impl<'a> FixValue<'a> for Date {
    type Error = &'static str;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let bytes = self.to_bytes();
        buffer.extend_from_slice(&bytes[..]);
        bytes.len()
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        if let Ok(bytes) = <[u8; LEN_IN_BYTES]>::try_from(data) {
            for byte in bytes.iter().copied() {
                if !is_digit(byte) {
                    return Err(ERR_NOT_ASCII_DIGITS);
                }
            }
            deserialize(bytes)
        } else {
            Err(ERR_LENGTH)
        }
    }

    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        if let Ok(bytes) = data.try_into() {
            deserialize(bytes)
        } else {
            Err(ERR_LENGTH)
        }
    }
}

fn deserialize(data: [u8; LEN_IN_BYTES]) -> Result<Date, &'static str> {
    let year = ascii_digit_to_u32(data[0], 1000)
        + ascii_digit_to_u32(data[1], 100)
        + ascii_digit_to_u32(data[2], 10)
        + ascii_digit_to_u32(data[3], 1);
    let month = ascii_digit_to_u32(data[4], 10) + ascii_digit_to_u32(data[5], 1);
    let day = ascii_digit_to_u32(data[6], 10) + ascii_digit_to_u32(data[7], 1);
    Date::new(year, month, day).ok_or(ERR_BOUNDS)
}

const fn is_digit(byte: u8) -> bool {
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

    impl Arbitrary for Date {
        fn arbitrary(g: &mut Gen) -> Self {
            let year = u32::arbitrary(g) % 10000;
            let month = (u32::arbitrary(g) % 12) + 1;
            let day = (u32::arbitrary(g) % 31) + 1;
            Date::new(year, month, day).unwrap()
        }
    }

    #[quickcheck]
    fn verify_serialization_behavior(date: Date) -> bool {
        super::super::verify_serialization_behavior(date)
    }

    const VALID_DATES: &[&[u8]] = &[
        b"00000101",
        b"00010101",
        b"99991231",
        b"99990101",
        b"20191225",
        b"20190231",
    ];

    const INVALID_DATES: &[&[u8]] = &[
        b"",          // Empty string.
        b"2013011",   // String too short.
        b"201301120", // String too long.
        b"00000001",  // Invalid month.
        b"00000100",  // Invalid day.
        b"19801301",  // Invalid month.
        b"19800001",  // Invalid month.
        b"19801233",  // Invalid day.
        b"19801232",  // Invalid day.
        b"-9801232",  // Invalid character.
        b"29801232",  // Invalid day.
        b"1980010a",  // Invalid character.
        b"1980010:",  // Invalid character.
        b"19800:01",  // Invalid character.
        b"19800:00",  // Invalid character and invalid day.
    ];

    #[quickcheck]
    fn serialize_and_to_bytes_are_the_same(date: Date) -> bool {
        date.to_bytes() == &FixValue::to_bytes(&date)[..]
    }

    #[test]
    fn lossy_and_lossless_are_equivalent() {
        // Lossy and losseless deserialization can only be meaningfully compared
        // on legal inputs.
        for bytes in VALID_DATES {
            let date = Date::deserialize(*bytes).unwrap();
            let date_lossy = Date::deserialize_lossy(*bytes).unwrap();
            assert_eq!(date, date_lossy);
        }
    }

    #[quickcheck]
    fn new_via_getters(date: Date) -> bool {
        let date_via_new = Date::new(date.year(), date.month(), date.day()).unwrap();
        date == date_via_new
    }

    #[test]
    fn lossless_deserialization_detects_errors() {
        for bytes in INVALID_DATES {
            assert!(Date::deserialize(*bytes).is_err());
        }
    }

    #[test]
    fn serialize_and_deserialize_are_consistent_with_each_other() {
        for bytes in VALID_DATES {
            let date = Date::deserialize(*bytes).unwrap();
            let serialized = date.to_bytes();
            assert_eq!(**bytes, serialized);
        }
    }
}
