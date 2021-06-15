use std::time::Duration;

use super::{FixValue, Timestamp};
use crate::Buffer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TzTimestamp {
    timestamp: Timestamp,
    is_utc: bool,
    tz_offset: Duration,
}

impl TzTimestamp {
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp.clone()
    }

    pub fn is_utc(&self) -> bool {
        self.is_utc
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
