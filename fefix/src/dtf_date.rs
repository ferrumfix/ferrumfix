const LEN_IN_BYTES: usize = 8;

/// Concrete value for [`DataType::LocalMktDate`](crate::DataType::LocalMktDate)
/// and [`DataType::UTCDateOnly`](crate::DataType::UtcDateOnly) fields.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DtfDate {
    year: u32,
    month: u32,
    day: u32,
}

impl DtfDate {
    /// Parses from a `"YYYYMMDD"` format.
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() != LEN_IN_BYTES {
            return None;
        }
        for byte in data.iter().copied() {
            if byte < b'0' || byte > b'9' {
                return None;
            }
        }
        let year = from_digit(data[0]) * 1000
            + from_digit(data[1]) * 100
            + from_digit(data[2]) * 10
            + from_digit(data[3]);
        let month = from_digit(data[4]) * 10 + from_digit(data[5]);
        let day = from_digit(data[6]) * 10 + from_digit(data[7]);
        if month >= 1 && month <= 12 && day >= 1 && day <= 31 {
            Some(Self { year, month, day })
        } else {
            None
        }
    }

    pub fn to_bytes(&self) -> [u8; LEN_IN_BYTES] {
        fn to_digit(n: u32) -> u8 {
            (n - (b'0' as u32)) as u8
        }
        [
            to_digit(self.year() / 1000),
            to_digit((self.year() / 100) % 10),
            to_digit((self.year() / 10) % 10),
            to_digit(self.year() % 10),
            to_digit(self.month() / 10),
            to_digit(self.month() % 10),
            to_digit(self.day() / 10),
            to_digit(self.day() % 10),
        ]
    }

    /// Returns the `year` of `self`.
    pub fn year(&self) -> u32 {
        self.year
    }

    /// Returns the `month` of `self`.
    pub fn month(&self) -> u32 {
        self.month
    }

    /// Returns the `day` of `self`.
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

fn from_digit(digit: u8) -> u32 {
    digit as u32 - (b'0' as u32)
}

#[cfg(test)]
mod test {
    use super::*;

    const VALID_DATES: &[&[u8; LEN_IN_BYTES]] = &[
        b"00000101",
        b"00010101",
        b"99991231",
        b"20191225",
        b"20190231",
    ];

    const INVALID_DATES: &[&[u8; LEN_IN_BYTES]] = &[
        b"00000001", // Invalid month.
        b"00000100", // Invalid day.
        b"19801301", // Invalid month.
        b"19800001", // Invalid month.
        b"19801233", // Invalid day.
        b"19801232", // Invalid day.
        b"-9801232", // Invalid character.
        b"29801232", // Invalid day.
        b"1980010a", // Invalid character.
        b"1980010:", // Invalid character.
        b"19800:01", // Invalid character.
        b"19800:00", // Invalid character and invalid day.
    ];

    #[test]
    fn parse_then_serialize() {
        for date in VALID_DATES {
            let dtf = DtfDate::parse(*date).unwrap();
            let as_bytes = dtf.to_bytes();
            assert_eq!(**date, as_bytes);
        }
    }

    #[test]
    fn detect_errors_in_edge_cases() {
        for date in INVALID_DATES {
            dbglog!(
                "Parsing date `{}` ({:?})",
                std::str::from_utf8(*date).unwrap_or(""),
                date
            );
            assert!(DtfDate::parse(*date).is_none());
        }
    }

    #[test]
    fn get_year() {
        assert_eq!(DtfDate::parse(b"00000101").unwrap().year(), 0);
        assert_eq!(DtfDate::parse(b"13370101").unwrap().year(), 1337);
        assert_eq!(DtfDate::parse(b"99990101").unwrap().year(), 9999);
    }

    #[test]
    fn get_month() {
        assert_eq!(DtfDate::parse(b"19700111").unwrap().month(), 1);
        assert_eq!(DtfDate::parse(b"19700201").unwrap().month(), 2);
        assert_eq!(DtfDate::parse(b"19701030").unwrap().month(), 10);
        assert_eq!(DtfDate::parse(b"19701131").unwrap().month(), 11);
        assert_eq!(DtfDate::parse(b"19701201").unwrap().month(), 12);
    }

    #[test]
    fn get_day() {
        assert_eq!(DtfDate::parse(b"19700201").unwrap().day(), 1);
        assert_eq!(DtfDate::parse(b"19700211").unwrap().day(), 11);
        assert_eq!(DtfDate::parse(b"19700230").unwrap().day(), 30);
        assert_eq!(DtfDate::parse(b"19700231").unwrap().day(), 31);
    }
}
