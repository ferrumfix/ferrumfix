use crate::prelude::*;

#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-decimal")))]
impl<'a> FixValue<'a> for decimal::d128 {
    type Error = decimal::Status;
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

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        decimal::d128::set_status(decimal::Status::empty());
        let s = std::str::from_utf8(data).unwrap_or(super::ERR_UTF8);
        let number =
            decimal::d128::from_str(s).expect("decimal::d128 should always parse without errors");
        let status = decimal::d128::get_status();
        if status.is_empty() {
            Ok(number)
        } else {
            Err(status)
        }
    }
}
