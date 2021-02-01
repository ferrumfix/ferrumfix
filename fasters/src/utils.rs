use std::io;

pub trait Buffer {
    fn as_slice(&self) -> &[u8];

    fn as_mut_slice(&mut self) -> &mut [u8];

    /// Returns the number of bytes that `self` can hold without reallocating.
    fn capacity(&self) -> usize;

    fn clear(&mut self);

    fn extend_from_slice(&mut self, extend: &[u8]);

    fn len(&self) -> usize;
}

pub struct GrowableBuffer<'a> {
    pub bytes: &'a mut Vec<u8>,
}

impl<'a> Extend<u8> for GrowableBuffer<'a> {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = u8>,
    {
        for byte in iter.into_iter() {
            self.bytes.push(byte);
        }
    }
}

impl<'a> Buffer for GrowableBuffer<'a> {
    fn as_slice(&self) -> &[u8] {
        &self.bytes[..]
    }

    fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.bytes[..]
    }

    fn capacity(&self) -> usize {
        self.bytes.capacity()
    }

    fn clear(&mut self) {
        self.bytes.clear()
    }

    fn extend_from_slice(&mut self, extend: &[u8]) {
        self.bytes.extend_from_slice(extend)
    }

    fn len(&self) -> usize {
        self.bytes.len()
    }
}

impl Buffer for Vec<u8> {
    fn as_slice(&self) -> &[u8] {
        self.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }

    fn clear(&mut self) {
        self.clear()
    }

    fn extend_from_slice(&mut self, extend: &[u8]) {
        self.extend_from_slice(extend)
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<B> Buffer for &mut B
where
    B: Buffer,
{
    fn as_slice(&self) -> &[u8] {
        Buffer::as_slice(*self)
    }

    fn as_mut_slice(&mut self) -> &mut [u8] {
        Buffer::as_mut_slice(*self)
    }

    fn capacity(&self) -> usize {
        Buffer::capacity(*self)
    }

    fn clear(&mut self) {
        Buffer::clear(*self)
    }

    fn extend_from_slice(&mut self, extend: &[u8]) {
        Buffer::extend_from_slice(*self, extend)
    }

    fn len(&self) -> usize {
        Buffer::len(*self)
    }
}

pub struct BufferWriter<B: Buffer> {
    buffer: B,
}

impl<B> Extend<u8> for BufferWriter<B>
where
    B: Buffer,
{
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = u8>,
    {
        for byte in iter.into_iter() {
            self.extend_from_slice(&[byte]);
        }
    }
}

impl<B> BufferWriter<B>
where
    B: Buffer,
{
    pub fn new(buffer: B) -> Self {
        Self { buffer }
    }
}

impl<B> Buffer for BufferWriter<B>
where
    B: Buffer,
{
    fn as_slice(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [u8] {
        self.buffer.as_mut_slice()
    }

    fn capacity(&self) -> usize {
        self.buffer.capacity()
    }

    fn clear(&mut self) {
        self.buffer.clear()
    }

    fn extend_from_slice(&mut self, extend: &[u8]) {
        self.buffer.extend_from_slice(extend)
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }
}

impl<B> io::Write for BufferWriter<B>
where
    B: Buffer,
{
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer.extend_from_slice(buf);
        Ok(buf.len())
    }
}

impl<'a> GrowableBuffer<'a> {
    pub fn new(buffer: &'a mut Vec<u8>) -> Self {
        buffer.clear();
        Self { bytes: buffer }
    }
}

impl<'a> io::Write for GrowableBuffer<'a> {
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.bytes.extend_from_slice(buf);
        Ok(buf.len())
    }
}
