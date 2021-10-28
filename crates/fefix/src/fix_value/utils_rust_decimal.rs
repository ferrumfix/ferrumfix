use super::{ERR_DECIMAL, ERR_UTF8};
use crate::prelude::*;
use crate::BufferWriter;
use rust_decimal::Decimal;
use std::fmt::Write;
use std::str::FromStr;

#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-rust-decimal")))]
impl<'a> FixValue<'a> for Decimal {
    type Error = &'static str;
    type SerializeSettings = ();

    #[inline]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        let initial_len = buffer.len();
        write!(BufferWriter(buffer), "{}", self).unwrap();
        buffer.len() - initial_len
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(data).map_err(|_| ERR_UTF8)?;
        Decimal::from_str(s).map_err(|_| ERR_DECIMAL)
    }
}
