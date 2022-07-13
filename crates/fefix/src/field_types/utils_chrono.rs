use super::{Date, Tz, ERR_TIME, ERR_UTF8};
use crate::{Buffer, BufferWriter, FieldType};
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Offset;
use chrono::TimeZone;
use chrono::Timelike;
use std::fmt::Write;

/// Specifies whether a timestamp should have millisecond or second precision;
/// see [`FieldType::SerializeSettings`].
///
/// Enabled by [`Default`].
#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-chrono")))]
#[derive(Debug, Copy, Clone)]
pub struct WithMilliseconds(pub bool);

impl Default for WithMilliseconds {
    fn default() -> Self {
        Self(true)
    }
}

#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-chrono")))]
impl<'a> FieldType<'a> for chrono::NaiveDateTime {
    type Error = &'static str;
    type SerializeSettings = WithMilliseconds;

    fn serialize_with<B>(&self, buffer: &mut B, with_millis: Self::SerializeSettings) -> usize
    where
        B: Buffer,
    {
        self.date().serialize(buffer)
            + b"-".serialize(buffer)
            + self.time().serialize_with(buffer, with_millis)
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 10 {
            return Err("Datetime field too short.");
        }

        let date = chrono::NaiveDate::deserialize(&data[..8])?;
        let hyphen = <&[u8]>::deserialize(&data[8..9]).unwrap();
        if hyphen != b"-" {
            return Err("Hyphen in datetime not found.");
        }
        let time = chrono::NaiveTime::deserialize(&data[9..])?;

        Ok(NaiveDateTime::new(date, time))
    }
}

#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-chrono")))]
impl<'a> FieldType<'a> for chrono::NaiveDate {
    type Error = &'static str;
    type SerializeSettings = ();

    #[inline]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
    where
        B: Buffer,
    {
        use chrono::Datelike;
        write!(
            BufferWriter(buffer),
            "{:04}{:02}{:02}",
            self.year(),
            self.month(),
            self.day(),
        )
        .unwrap();
        8
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        Self::parse_from_str(s, "%Y%m%d").map_err(|_| ERR_TIME)
    }
}

#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-chrono")))]
impl<'a> FieldType<'a> for chrono::NaiveTime {
    type Error = &'static str;
    type SerializeSettings = WithMilliseconds;

    fn serialize_with<B>(
        &self,
        buffer: &mut B,
        WithMilliseconds(with_millis): Self::SerializeSettings,
    ) -> usize
    where
        B: Buffer,
    {
        write!(
            BufferWriter(buffer),
            "{:02}:{:02}:{:02}",
            self.hour(),
            self.minute(),
            self.second()
        )
        .unwrap();
        if with_millis {
            write!(
                BufferWriter(buffer),
                ".{:03}",
                self.nanosecond() / 1_000_000
            )
            .unwrap();
            12
        } else {
            8
        }
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        if data.len() == 8 {
            Self::parse_from_str(s, "%H:%M:%S").map_err(|_| ERR_TIME)
        } else {
            Self::parse_from_str(s, "%H:%M:%S.%3f").map_err(|_| ERR_TIME)
        }
    }
}

#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-chrono")))]
impl<'a> FieldType<'a> for chrono::FixedOffset {
    type Error = &'static str;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _: ()) -> usize
    where
        B: Buffer,
    {
        Tz::from_chrono_offset(*self).serialize(buffer)
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        Tz::deserialize(data).map(|tz| tz.to_chrono_offset())
    }
}

#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-chrono")))]
impl<'a> FieldType<'a> for chrono::DateTime<chrono::FixedOffset> {
    type Error = &'static str;
    type SerializeSettings = ();

    fn serialize_with<B>(&self, buffer: &mut B, _: ()) -> usize
    where
        B: Buffer,
    {
        self.date().naive_local().serialize(buffer) + self.timezone().serialize(buffer)
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() < 17 {
            return Err("Datetime too short.");
        }
        let utc_naive_datetime = NaiveDateTime::deserialize(&data[..17])?;
        let tz = chrono::FixedOffset::deserialize(&data[17..])?;
        Ok(DateTime::<chrono::Utc>::from_utc(utc_naive_datetime, chrono::Utc).with_timezone(&tz))
    }
}
