use crate::prelude::*;

const ERR_DECIMAL_INVALID: &str = "Invalid decimal number.";

#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-rust-decimal")))]
impl<'a> FixValue<'a> for rust_decimal::Decimal {
    type Error = &'static str;
    type SerializeSettings = ();

    #[inline]
    fn serialize_with<B>(&self, buffer: &mut B, _settings: ()) -> usize
    where
        B: Buffer,
    {
        // TODO: Remove allocations.
        let s = ToString::to_string(self);
        buffer.extend_from_slice(s.as_bytes());
        s.as_bytes().len()
    }

    #[inline]
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        let s = std::str::from_utf8(data).map_err(|_| super::ERR_UTF8)?;
        rust_decimal::Decimal::from_str(s).map_err(|_| ERR_DECIMAL_INVALID)
    }
}
