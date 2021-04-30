use super::error;
use crate::FixFieldValue;
use crate::Buffer;

const LEN_IN_BYTES: usize = 8;

const MAX_YEAR: u32 = 9999;
const MAX_MONTH: u32 = 12;
const MAX_DAY: u32 = 31;

const MIN_YEAR: u32 = 0;
const MIN_MONTH: u32 = 1;
const MIN_DAY: u32 = 1;

/// Canonical data field (DTF) for
/// [`DataType::LocalMktDate`](crate::DataType::LocalMktDate)
/// and [`DataType::UTCDateOnly`](crate::DataType::UtcDateOnly).
///
/// Date in format `YYYYMMDD`.
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
    /// Crates a new date from its components. It returns `None` if any of the
    /// three components is outside the legal range.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::datatypes::Date;
    ///
    /// assert!(Date::new(2021, 4, 16).is_ok());
    /// assert!(Date::new(2021, 13, 32).is_err());
    ///
    /// // Support from January 1, year zero (which doesn't actually exist) to
    /// // December 31, 9999.
    /// assert!(Date::new(0, 1, 1).is_ok());
    /// assert!(Date::new(9999, 12, 31).is_ok());
    ///
    /// // We don't check month-aware day boundaries, i.e. go ahead and assume
    /// // every month has 31 days.
    /// assert!(Date::new(2021, 2, 31).is_ok());
    /// ```
    pub fn new(year: u32, month: u32, day: u32) -> Result<Self, error::Date> {
        if year >= MIN_YEAR
            && year <= MAX_YEAR
            && month >= MIN_MONTH
            && month <= MAX_MONTH
            && day >= MIN_DAY
            && day <= MAX_DAY
        {
            Ok(Self { year, month, day })
        } else {
            Err(error::Date::OutsideBounds)
        }
    }

    /// Converts `self` to `YYYYMMDD` format.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::datatypes::Date;
    ///
    /// assert_eq!(&Date::new(2021, 01, 01).unwrap().to_bytes(), b"20210101");
    /// ```
    pub fn to_bytes(&self) -> [u8; LEN_IN_BYTES] {
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

    #[cfg(feature = "chrono-time")]
    pub fn to_chrono_utc_date(&self) -> chrono::Date {
        let naive = self.to_chrono_naivedate();
        chrono::Date::from_utc(naive, chrono::Utc)
    }

    #[cfg(feature = "chrono-time")]
    pub fn to_chrono_naivedate(&self) -> chrono::NaiveDate {
        chrono::NaiveDate::from_ymd_opt(self.year(), self.month(), self.day())
    }
}

impl<'a> FixFieldValue<'a> for Date {
    type Error = error::Date;
    type SerializeSettings = ();

    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        let bytes = self.to_bytes();
        buffer.extend_from_slice(&bytes[..]);
        bytes.len()
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != LEN_IN_BYTES {
            return Err(Self::Error::WrongLength);
        }
        for byte in data.iter().copied() {
            if !is_digit(byte) {
                return Err(Self::Error::NotAsciiDigits);
            }
        }
        deserialize(data)
    }

    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != LEN_IN_BYTES {
            return Err(Self::Error::WrongLength);
        }
        deserialize(data)
    }
}

fn deserialize(data: &[u8]) -> Result<Date, error::Date> {
    debug_assert_eq!(data.len(), LEN_IN_BYTES);
    let year = ascii_digit_to_u32(data[0], 1000)
        + ascii_digit_to_u32(data[1], 100)
        + ascii_digit_to_u32(data[2], 10)
        + ascii_digit_to_u32(data[3], 1);
    let month = ascii_digit_to_u32(data[4], 10) + ascii_digit_to_u32(data[5], 1);
    let day = ascii_digit_to_u32(data[6], 10) + ascii_digit_to_u32(data[7], 1);
    Date::new(year, month, day)
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

    #[test]
    fn new_is_consistent_with_deserialize() {
        for bytes in VALID_DATES {
            let date = Date::deserialize(*bytes).unwrap();
            let date_via_new = Date::new(date.year(), date.month(), date.day()).unwrap();
            assert_eq!(date, date_via_new);
        }
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
