use super::{Date, ZeroPadding};
use crate::{Buffer, FixValue};

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

    #[inline]
    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        // Serialize with milliseconds by default.
        self.serialize_with(buffer, WithMilliseconds(true))
    }

    #[inline]
    fn serialize_with<B>(&self, buffer: &mut B, settings: Self::SerializeSettings) -> usize
    where
        B: Buffer,
    {
        use chrono::{Datelike, Timelike};
        (self.year() as u32).serialize_with(buffer, ZeroPadding(4));
        (self.month() as u32).serialize_with(buffer, ZeroPadding(2));
        (self.day() as u32).serialize_with(buffer, ZeroPadding(2));
        buffer.extend_from_slice(b"-");
        (self.hour() as u32).serialize_with(buffer, ZeroPadding(2));
        buffer.extend_from_slice(b":");
        (self.minute() as u32).serialize_with(buffer, ZeroPadding(2));
        buffer.extend_from_slice(b":");
        (self.second() as u32).serialize_with(buffer, ZeroPadding(2));
        if settings.0 {
            buffer.extend_from_slice(b".");
            (self.nanosecond() / 10E6 as u32).serialize_with(buffer, ZeroPadding(3));
            21
        } else {
            17
        }
    }

    #[inline]
    fn deserialize(_data: &'a [u8]) -> Result<Self, Self::Error> {
        Err("TODO")
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
        (self.year() as u32).serialize_with(buffer, ZeroPadding(4));
        (self.month() as u32).serialize_with(buffer, ZeroPadding(2));
        (self.day() as u32).serialize_with(buffer, ZeroPadding(2));
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
