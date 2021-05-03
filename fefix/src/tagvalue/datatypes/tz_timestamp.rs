use std::time::Duration;

use super::error;
use super::{FixFieldValue, Timestamp};
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

impl<'a> FixFieldValue<'a> for TzTimestamp {
    type Error = error::Timestamp;
    type SerializeSettings = ();

    const IS_ASCII: bool = true;

    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        self.timestamp().serialize(buffer)
    }

    fn deserialize(_data: &'a [u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
