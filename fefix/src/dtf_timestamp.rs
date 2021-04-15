use crate::{tagvalue::SerializeField, Buffer, DtfDate, DtfTime};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DtfTimestamp {
    date: DtfDate,
    time: DtfTime,
}

impl DtfTimestamp {
    pub fn new(date: DtfDate, time: DtfTime) -> Self {
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
        let date = DtfDate::parse(&data[0..8])?;
        let time = DtfTime::parse(&data[9..])?;
        Some(Self::new(date, time))
    }

    pub fn utc_now() -> Self {
        use chrono::prelude::*;
        let utc: DateTime<Utc> = Utc::now();
        let date = DtfDate::new(utc.year() as u32, utc.month() as u32, utc.day() as u32);
        let time = DtfTime::from_hmsm(
            utc.hour() as u32,
            utc.minute() as u32,
            utc.second() as u32,
            utc.nanosecond() as u32 / 1_000_000,
        );
        Self::new(date.unwrap(), time)
    }

    /// Returns the date of `self`.
    pub fn date(&self) -> DtfDate {
        self.date
    }

    /// Returns the time of `self`.
    pub fn time(&self) -> DtfTime {
        self.time.clone()
    }
}

impl SerializeField for DtfTimestamp {
    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        self.date().serialize(buffer) + b"-".serialize(buffer) + self.time().serialize(buffer)
    }
}
