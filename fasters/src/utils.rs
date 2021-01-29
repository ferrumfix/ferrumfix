use std::io;

pub struct GrowableVecU8<'a> {
    pub bytes: &'a mut Vec<u8>,
}

impl<'a> io::Write for GrowableVecU8<'a> {
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.bytes.extend_from_slice(buf);
        Ok(buf.len())
    }
}
