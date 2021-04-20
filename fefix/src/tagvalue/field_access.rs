use crate::dtf;

/// A trait to retrieve field values in a FIX message.
pub trait FieldAccess<E> {
    fn raw(&self) -> Result<&[u8], E>;

    fn as_char(&self) -> Result<u8, E>;

    fn as_chars(&self) -> Result<dtf::MultipleChars, E>;

    fn as_bool(&self) -> Result<bool, E>;

    fn as_i64(&self) -> Result<i64, E>;

    fn as_u64(&self) -> Result<u64, E>;

    fn as_timestamp(&self) -> Result<i64, E>;

    fn as_date(&self) -> Result<dtf::Date, E>;

    fn as_time(&self) -> Result<dtf::Time, E>;

    fn as_float(&self) -> Result<(), E>;

    fn as_month_year(&self) -> Result<dtf::MonthYear, E>;
}
