//! Types for holding [`DataType`](crate::DataType) values.
//!
//! This module introduces reusable, allocation-free data structures that can be
//! used to store [`DataType`](crate::DataType) values. This is done via the
//! [`DataType`] trait, which allows both serialization and deserialization.
//!
//! FerrumFIX maps FIX date types to the following [`DataType`] implementors:
//!
//! - `int`: any Rust primitive integer type;
//! - `Length`: [`usize`];
//! - `NumInGroup`: [`u16`];
//! - `SeqNum`: [`u64`];
//! - `TagNum`: [`u32`];
//! - `DayOfMonth`: [`u32`];
//! - `float`, `Qty`, `Price`, `PriceOffset`, `Amt`, `Percentage`:
//! `rust_decimal::Decimal`, `decimal::d128` or other custom types;
//! - `char`: [`u8`] (FIX mandates the use of a single-byte encoding, so `u8` is
//! a better fit than `char`);
//! - `Boolean`: [`bool`];
//! - `String`, `data`: `&[u8]`;
//! - `MultipleCharValue`: [`MultipleChars`];
//! - `MultipleValueString`: [`MultipleStrings`];
//! - `Country`: [`Country`];
//! - `Currency`: [`Currency`];
//! - `Exchange`: [`Exchange`];
//! - `month-year`: [`MonthYear`];
//! - `UTCTimestamp`, `LocalMktDate`: [`Timestamp`];
//! - `UTCTimeOnly`: [`Time`];
//! - `UTCDateOnly`: [`Date`];
//! - `TZTimeOnly`: [`TzTime`];
//! - `TZTimestamp`: [`TzTimestamp`];
//!
//! # Quick tour of [`FixValue`]
//!
//! ```
//! use fefix::FixValue;
//! use fefix::tagvalue::datatypes::Timestamp;
//!
//! let bytes = b"20130422-12:30:00.000";
//!
//! // You can use `FixValue::deserialize` to parse data fields.
//! let timestamp = Timestamp::deserialize(bytes).unwrap();
//! assert_eq!(timestamp.date().year(), 2013);
//!
//! // `FixValue::deserialize_lossy` is like `FixValue::deserialize`, but it's
//! // allowed to skip some format verification for the sake of speed.
//! assert!(u32::deserialize(b"invalid integer").is_err());
//! assert!(u32::deserialize_lossy(b"invalid integer").is_ok());
//!
//! let buffer: &mut Vec<u8> = &mut vec![];
//! // Use `DataType::serialize` to write values to buffers.
//! 1337u32.serialize(buffer);
//! assert_eq!(&buffer[..], b"1337" as &[u8]);
//! ```

mod checksum;
mod date;
pub mod error;
mod monthyear;
mod multiple_chars;
mod multiple_strings;
mod time;
mod timestamp;
mod tz;
mod tz_time;
mod tz_timestamp;

use crate::FixValue;

pub use checksum::CheckSum;
pub use date::Date;
pub use monthyear::MonthYear;
pub use multiple_chars::MultipleChars;
pub use multiple_strings::MultipleStrings;
pub use time::Time;
pub use timestamp::Timestamp;
pub use tz::Tz;
pub use tz_time::TzTime;
pub use tz_timestamp::TzTimestamp;

pub type Country = [u8; 2];
pub type Currency = [u8; 3];
pub type Exchange = [u8; 4];

#[cfg(test)]
pub fn verify_serialization_behavior<T>(item: T) -> bool
where
    T: for<'a> FixValue<'a> + PartialEq,
{
    let serialized = item.to_bytes();
    let bytes = &serialized[..];
    let deserialized = T::deserialize(bytes).ok().unwrap();
    let deserialized_lossy = T::deserialize_lossy(bytes).ok().unwrap();
    deserialized == item && deserialized_lossy == item
}
