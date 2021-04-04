use crate::{DtfDate, DtfMonthYear, DtfMulCharIter, DtfTime};

/// A trait to retrieve field values in a FIX message.
pub trait FieldAccess<E> {
    fn raw(&self) -> Result<&[u8], E>;

    fn as_char(&self) -> Result<u8, E>;

    fn as_chars(&self) -> Result<DtfMulCharIter<b' '>, E>;

    fn as_bool(&self) -> Result<bool, E>;

    fn as_i64(&self) -> Result<i64, E>;

    fn as_u64(&self) -> Result<u64, E>;

    fn as_timestamp(&self) -> Result<i64, E>;

    fn as_date(&self) -> Result<DtfDate, E>;

    fn as_time(&self) -> Result<DtfTime, E>;

    fn as_float(&self) -> Result<(), E>;

    fn as_month_year(&self) -> Result<DtfMonthYear, E>;
}
