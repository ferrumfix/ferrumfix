#![cfg_attr(docsrs, feature(doc_cfg))]
use crate::field_types::{Date, Time};
use crate::{Buffer, FieldType};

/// Representation for `UtcTimestamp`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timestamp {
    date: Date,
    time: Time,
}

impl Timestamp {
    /// Combines `date` and `time` into a [`Timestamp`].
    pub fn new(date: Date, time: Time) -> Self {
        Self { date, time }
    }

    /// Parses from a `"YYYYMMDD"` format.
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 12 || data[8] != b'-' {
            return None;
        }
        let date = Date::deserialize(&data[0..8]).ok()?;
        let time = Time::deserialize(&data[9..]).ok()?;
        Some(Self::new(date, time))
    }

    /// Returns the current UTC system time with millisecond precision.
    ///
    /// # Panics
    /// This function will panic if the current system time is not a valid
    /// FIX timestamp. This should never happen.
    pub fn utc_now() -> Self {
        use chrono::{Datelike, Timelike};
        let utc: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let date = Date::new(utc.year() as u32, utc.month(), utc.day());
        let time = Time::from_hmsm(
            utc.hour(),
            utc.minute(),
            utc.second(),
            utc.nanosecond() / 1_000_000,
        )
        .unwrap();
        Self::new(date.unwrap(), time)
    }

    /// Returns the date of `self`.
    pub fn date(&self) -> Date {
        self.date
    }

    /// Returns the time of `self`.
    pub fn time(&self) -> Time {
        self.time
    }

    /// Attempts to convert `self` to a UTC [`chrono::DateTime`]. As `chrono`
    /// might perform additional checks that make such conversion impossible,
    /// the return value of this function might be `None`.
    #[cfg(feature = "utils-chrono")]
    pub fn to_chrono_utc(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.to_chrono_naive()
            .map(|naive| chrono::DateTime::from_naive_utc_and_offset(naive, chrono::Utc))
    }

    /// Attempts to convert `self` to a [`chrono::NaiveDateTime`]. As `chrono`
    /// might perform additional checks that make such conversion impossible,
    /// the return value of this function might be `None`.
    #[cfg(feature = "utils-chrono")]
    pub fn to_chrono_naive(&self) -> Option<chrono::NaiveDateTime> {
        let date = self.date().to_chrono_naive()?;
        let time = self.time().to_chrono_naive()?;
        Some(chrono::NaiveDateTime::new(date, time))
    }
}

impl<'a> FieldType<'a> for Timestamp {
    type Error = &'static str;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        self.date().serialize(buffer) + b"-".serialize(buffer) + self.time().serialize(buffer)
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        Self::parse(data).ok_or("Invalid timestamp format")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    impl Arbitrary for Timestamp {
        fn arbitrary(g: &mut Gen) -> Self {
            let date = Date::arbitrary(g).to_string();
            let time = Time::arbitrary(g).to_string();
            let s = format!("{date}-{time}");
            Self::deserialize(s.as_bytes()).unwrap()
        }
    }

    #[quickcheck]
    fn verify_serialization_behavior(timestamp: Timestamp) -> bool {
        crate::field_types::test_utility_verify_serialization_behavior(timestamp)
    }
}
