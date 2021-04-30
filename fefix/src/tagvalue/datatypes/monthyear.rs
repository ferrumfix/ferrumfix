use super::error;
use crate::FixFieldValue;
use crate::Buffer;

const LEN_IN_BYTES: usize = 8;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DayOrWeek {
    Day(u32),
    Week(u32),
}

/// Canonical data field (DTF) for
/// [`DataType::MonthYear`](crate::DataType::MonthYear).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MonthYear {
    year: u32,
    month: u32,
    day_or_week: DayOrWeek,
}

impl MonthYear {
    /// Converts `self` to a byte array.
    pub fn to_bytes(&self) -> [u8; LEN_IN_BYTES] {
        let day_or_week_1 = match self.day_or_week {
            DayOrWeek::Day(day) => (day / 10) as u8 + b'0',
            DayOrWeek::Week(_) => b'w',
        };
        let day_or_week_2 = match self.day_or_week {
            DayOrWeek::Day(day) => (day % 10) as u8 + b'0',
            DayOrWeek::Week(week) => week as u8 + b'0',
        };
        [
            (self.year() / 1000) as u8 + b'0',
            ((self.year() / 100) % 10) as u8 + b'0',
            ((self.year() / 10) % 10) as u8 + b'0',
            (self.year() % 10) as u8 + b'0',
            (self.month() / 10) as u8 + b'0',
            (self.month() % 10) as u8 + b'0',
            day_or_week_1,
            day_or_week_2,
        ]
    }

    /// Returns the year of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::datatypes::MonthYear;
    ///
    /// let dtf = MonthYear::parse(b"19390901").unwrap();
    /// assert_eq!(dtf.year(), 1939)
    /// ```
    pub fn year(&self) -> u32 {
        self.year
    }

    /// Returns the month of `self`, starting from January as 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::datatypes::MonthYear;
    ///
    /// let dtf = MonthYear::parse(b"20000101").unwrap();
    /// assert_eq!(dtf.month(), 1)
    /// ```
    pub fn month(&self) -> u32 {
        self.month
    }

    /// Returns the day of `self`, if defined.
    ///
    /// # Examples
    ///
    /// Day included in the definition:
    ///
    /// ```
    /// use fefix::datatypes::MonthYear;
    ///
    /// let dtf = MonthYear::parse(b"20191225").unwrap();
    /// assert_eq!(dtf.day(), Some(25))
    /// ```
    ///
    /// Day not included:
    ///
    /// ```
    /// use fefix::datatypes::MonthYear;
    ///
    /// let dtf = MonthYear::parse(b"201801w3").unwrap();
    /// assert_eq!(dtf.day(), None)
    /// ```
    pub fn day(&self) -> Option<u32> {
        if let DayOrWeek::Day(day) = self.day_or_week {
            Some(day)
        } else {
            None
        }
    }

    /// Returns the intra-month week code of `self`, if defined. Note that it is
    /// 1-indexed.
    ///
    /// # Examples
    ///
    /// Present week code:
    ///
    /// ```
    /// use fefix::datatypes::MonthYear;
    ///
    /// let dtf = MonthYear::parse(b"201912w1").unwrap();
    /// assert_eq!(dtf.week(), Some(1))
    /// ```
    ///
    /// Absent week code:
    ///
    /// ```
    /// use fefix::datatypes::MonthYear;
    ///
    /// let dtf = MonthYear::parse(b"20191225").unwrap();
    /// assert_eq!(dtf.week(), None)
    /// ```
    pub fn week(&self) -> Option<u32> {
        if let DayOrWeek::Week(week) = self.day_or_week {
            Some(week)
        } else {
            None
        }
    }
}

impl<'a> FixFieldValue<'a> for MonthYear {
    type Error = error::MonthYear;
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
        if validate(data) {
            Self::deserialize_lossy(data)
        } else {
            Err(Self::Error::Other)
        }
    }

    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        let year = from_digit(data[0]) as u32 * 1000
            + from_digit(data[1]) as u32 * 100
            + from_digit(data[2]) as u32 * 10
            + from_digit(data[3]) as u32;
        let month = from_digit(data[4]) as u32 * 10 + from_digit(data[5]) as u32;
        let day_or_week = if data[6] == b'w' {
            DayOrWeek::Week(from_digit(data[7]) as u32)
        } else {
            DayOrWeek::Day(from_digit(data[6]) as u32 * 10 + from_digit(data[7]) as u32)
        };
        Ok(Self {
            year,
            month,
            day_or_week,
        })
    }
}

fn is_digit(byte: u8, min_digit: u8, max_digit: u8) -> bool {
    byte >= (min_digit + b'0') && byte <= (max_digit + b'0')
}

fn from_digit(digit: u8) -> u8 {
    digit.wrapping_sub(b'0')
}

fn validate(data: &[u8]) -> bool {
    if data.len() != 8 {
        return false;
    }
    if !validate_year(data) || !validate_month(data) {
        return false;
    }
    validate_week(data) || validate_day(data)
}

fn validate_year(data: &[u8]) -> bool {
    is_digit(data[0], 0, 9)
        && is_digit(data[1], 0, 9)
        && is_digit(data[2], 0, 9)
        && is_digit(data[3], 0, 9)
}

fn validate_month(data: &[u8]) -> bool {
    ((data[4] == b'0' && data[5] <= b'9') || (data[4] == b'1' && data[5] <= b'2'))
        && data[5] >= b'0'
}

fn validate_week(data: &[u8]) -> bool {
    data[6] == b'w' && is_digit(data[7], 1, 5)
}

fn validate_day(data: &[u8]) -> bool {
    (data[6] == b'0' && data[7] >= b'0' && data[7] <= b'9')
        || (data[6] == b'1' && data[7] >= b'0' && data[7] <= b'9')
        || (data[6] == b'2' && data[7] >= b'0' && data[7] <= b'9')
        || (data[6] == b'3' && data[7] >= b'0' && data[7] <= b'1')
}

#[cfg(test)]
mod test {

    #[test]
    fn parse_speedy() {}
}
