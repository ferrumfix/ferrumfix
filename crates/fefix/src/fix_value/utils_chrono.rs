use super::{Date, ERR_TIME, ERR_UTF8};
use crate::{Buffer, BufferWriter, FixValue};
use chrono::NaiveDateTime;
use std::fmt::Write;

/// Specifies whether a timestamp should have millisecond or second precision;
/// see [`FixValue::SerializeSettings`].
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
impl<'a> FixValue<'a> for chrono::DateTime<chrono::Utc> {
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
        use chrono::Timelike;
        write!(
            BufferWriter(buffer),
            "{}-{:02}:{:02}:{:02}",
            self.date().naive_utc(),
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
            21
        } else {
            17
        }
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        let naive;
        if data.len() == 21 {
            naive =
                NaiveDateTime::parse_from_str(s, "%Y%m%d-%H:%M:%S.%3f").map_err(|_| ERR_TIME)?;
        } else {
            naive = NaiveDateTime::parse_from_str(s, "%Y%m%d-%H:%M:%S").map_err(|_| ERR_TIME)?;
        }
        Ok(Self::from_utc(naive, chrono::Utc))
    }
}

#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-chrono")))]
impl<'a> FixValue<'a> for chrono::NaiveDate {
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
        let date = Date::deserialize(data).map_err(|_| "Invalid date format.")?;
        date.to_chrono_naive().ok_or("Invalid date range.")
    }

    #[inline]
    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        let date = Date::deserialize_lossy(data).map_err(|_| "Invalid date format.")?;
        date.to_chrono_naive().ok_or("Invalid date range.")
    }
}
