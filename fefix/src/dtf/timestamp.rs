use super::error;
use crate::{dtf::DataField, dtf::Date, dtf::Time, Buffer};

/// Canonical data field (DTF) for
/// [DataType::UtcTimestamp](super::DataType::UtcTimeStamp).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Timestamp {
    date: Date,
    time: Time,
}

impl Timestamp {
    pub fn new(date: Date, time: Time) -> Self {
        Self { date, time }
    }

    /// Parses from a `"YYYYMMDD"` format.
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 12 {
            return None;
        }
        if data[8] != b'-' {
            return None;
        }
        let date = Date::deserialize(&data[0..8]).ok()?;
        let time = Time::deserialize(&data[9..]).ok()?;
        Some(Self::new(date, time))
    }

    pub fn utc_now() -> Self {
        use chrono::{Datelike, Timelike};
        let utc: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
        let date = Date::new(utc.year() as u32, utc.month() as u32, utc.day() as u32);
        let time = Time::from_hmsm(
            utc.hour() as u32,
            utc.minute() as u32,
            utc.second() as u32,
            utc.nanosecond() as u32 / 1_000_000,
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
}

impl<'a> DataField<'a> for Timestamp {
    type Error = error::Timestamp;
    type SerializeSettings = ();

    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        self.date().serialize(buffer) + b"-".serialize(buffer) + self.time().serialize(buffer)
    }

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        Self::parse(data).ok_or(Self::Error::Other)
    }
}
