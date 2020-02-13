use crate::err::Result;
use std::io::Write;

pub trait Fix: Sized {
    fn des(bytes: &[u8]) -> Result<Self>;
    fn ser(w: impl Write) -> Result<usize>;
    fn des_fixml(bytes: &[u8]) -> Result<Self>;
    fn ser_fixml(w: impl Write) -> Result<usize>;
}
