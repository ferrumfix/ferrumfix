mod fix_message;

use chrono::NaiveDateTime;
pub use fix_message::{FieldsIter, FixMessage};

pub trait FixMessageReadGroup {
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait FixMessageRead {
    fn field_raw(&self, tag: u32) -> Option<&[u8]>;

    fn field_char(&self, tag: u32) -> Option<char>;

    fn field_bool(&self, tag: u32) -> Option<bool>;

    fn field_data(&self, tag: u32) -> Option<&[u8]>;

    fn field_i64(&self, tag: u32) -> Option<i64>;

    fn field_u64(&self, tag: u32) -> Option<u64>;

    fn field_str(&self, tag: u32) -> Option<&str>;

    //fn group(&self, tag: u32) -> Option<Group>;

    //fn field_utc_timestamp(&self, tag: u32) -> Option<UTCTimestamp> {
    //    let s = self.field_data(tag)?;
    //    let len = s.len();
    //    if len < 17 {
    //        return None;
    //    }
    //    // YYYYMMDD-HH:MM:SS.sss
    //    let timestamp = UTCTimestamp {
    //        year: digit(s[0]) as u32 * 1000
    //            + digit(s[1]) as u32 * 100
    //            + digit(s[2]) as u32 * 10
    //            + digit(s[3]) as u32,
    //        month: digit(s[4]) * 10 + digit(s[5]),
    //        day: digit(s[6]) * 10 + digit(s[7]),
    //        hour: digit(s[9]) * 10 + digit(s[10]),
    //        minute: digit(s[12]) * 10 + digit(s[13]),
    //        second: digit(s[15]) * 10 + digit(s[16]),
    //    };
    //    if len == 17 {
    //        Some(timestamp)
    //    } else if len == 21 {
    //        Some(timestamp)
    //    } else {
    //        None
    //    }
    //}

    fn field_chrono_dt(&self, tag: u32) -> Option<chrono::DateTime<chrono::Utc>> {
        use chrono::DateTime;
        let s = self.field_str(tag)?;
        let naive = NaiveDateTime::parse_from_str(s, "").ok()?;
        Some(DateTime::<chrono::Utc>::from_utc(
            naive,
            chrono::offset::Utc {},
        ))
    }
}

/// This trait provides random access to any tag within a FIX message.
pub trait FixFieldAccess {
    fn field_char(&self, tag: u32) -> Option<char>;

    fn field(&self, tag: u32) -> Option<&[u8]> {
        self.field_data(tag)
    }

    fn field_data(&self, tag: u32) -> Option<&[u8]>;

    fn field_bool(&self, tag: u32) -> Option<bool>;

    fn field_i64(&self, tag: u32) -> Option<i64>;

    fn field_str(&self, tag: u32) -> Option<&str>;

    //fn field_utc_timestamp(&self, tag: u32) -> Option<UTCTimestamp> {
    //    let s = self.field_data(tag)?;
    //    let len = s.len();
    //    if len < 17 {
    //        return None;
    //    }
    //    // YYYYMMDD-HH:MM:SS.sss
    //    let timestamp = UTCTimestamp {
    //        year: digit(s[0]) as u32 * 1000
    //            + digit(s[1]) as u32 * 100
    //            + digit(s[2]) as u32 * 10
    //            + digit(s[3]) as u32,
    //        month: digit(s[4]) * 10 + digit(s[5]),
    //        day: digit(s[6]) * 10 + digit(s[7]),
    //        hour: digit(s[9]) * 10 + digit(s[10]),
    //        minute: digit(s[12]) * 10 + digit(s[13]),
    //        second: digit(s[15]) * 10 + digit(s[16]),
    //    };
    //    if len == 17 {
    //        Some(timestamp)
    //    } else if len == 21 {
    //        Some(timestamp)
    //    } else {
    //        None
    //    }
    //}

    fn field_chrono_dt(&self, tag: u32) -> Option<chrono::DateTime<chrono::Utc>> {
        use chrono::DateTime;
        let s = self.field_str(tag)?;
        let naive = NaiveDateTime::parse_from_str(s, "").ok()?;
        Some(DateTime::<chrono::Utc>::from_utc(
            naive,
            chrono::offset::Utc {},
        ))
    }
}

/// This trait allows implementors to sequentially iterate over FIX fields in a
/// message.
pub trait FixFieldsIter<T> {
    type FieldsIter: Iterator<Item = (u32, T)>;
    type FieldsIterStdHeader: Iterator<Item = (u32, T)>;
    type FieldsIterBody: Iterator<Item = (u32, T)>;

    fn iter_fields(&self) -> Self::FieldsIter;

    fn iter_fields_in_std_header(&self) -> Self::FieldsIterStdHeader;

    fn iter_fields_in_body(&self) -> Self::FieldsIterBody;
}

#[derive(Debug, Clone)]
pub enum Error {
    Duplicate,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_message_has_no_fields() {
        let msg = FixMessage::new();
        assert_eq!((&msg).iter_fields().count(), 0);
    }
}
