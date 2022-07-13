use super::{ERR_DECIMAL, ERR_UTF8};
use crate::prelude::*;
use crate::BufferWriter;
use decimal::d128;
use std::fmt::Write;
use std::str::FromStr;

#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-decimal")))]
impl<'a> FieldType<'a> for d128 {
    type Error = decimal::Status;
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

    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error> {
        d128::set_status(decimal::Status::empty());

        let s = std::str::from_utf8(data).unwrap_or(ERR_UTF8);
        let number = d128::from_str(s).expect(ERR_DECIMAL);

        let status = d128::get_status();
        if status.is_empty() {
            Ok(number)
        } else {
            Err(status)
        }
    }
}
