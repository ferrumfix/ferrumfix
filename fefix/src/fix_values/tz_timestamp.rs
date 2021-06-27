use super::{FixValue, Timestamp, Tz};
use crate::Buffer;

/// A time and date combination representing local time with an offset from UTC.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TzTimestamp {
    timestamp: Timestamp,
    tz: Tz,
}

impl TzTimestamp {
    /// Returns the [`Timestamp`] (without timezone information) of `self`.
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp.clone()
    }

    /// Returns the [`Tz`] timezone information of `self`.
    pub fn timezone(&self) -> Tz {
        self.tz
    }
}

impl<'a> FixValue<'a> for TzTimestamp {
    type Error = &'static str;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        self.timestamp().serialize(buffer)
    }

    fn deserialize(_data: &'a [u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
