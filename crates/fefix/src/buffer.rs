//! Zero-copy buffering utilities.

/// Operations on a growable in-memory buffer.
///
/// [`Buffer`] allows common data operations on in-memory data buffers. While
/// [`std::io::Write`] only allows sequential write operations, [`Buffer`] allows
/// arbitrary data manipulation over the whole buffer.
pub trait Buffer {
    /// Returns an immutable reference to the contents of the buffer.
    fn as_slice(&self) -> &[u8];

    /// Returns a mutable reference to the contents of the buffer.
    fn as_mut_slice(&mut self) -> &mut [u8];

    /// Returns the length of the contents of the buffer.
    fn len(&self) -> usize {
        self.as_slice().len()
    }

    /// Returns the number of bytes that `self` can hold without reallocating.
    fn capacity(&self) -> usize;

    /// Erases the contents of `self`.
    fn clear(&mut self);

    /// Appends the contents of `extend` onto `self`, growing the buffer if
    /// necessary.
    fn extend_from_slice(&mut self, extend: &[u8]);

    /// Resizes this [`Buffer`] in-place so that its new `len()` is equal to
    /// `new_len`.
    ///
    /// If `new_len` is greater than `len()`, `self` is extended by the difference,
    /// with each additional slot filled with `filler`. If `new_len` is less than `len()`,
    /// `self` is simply truncated.
    fn resize(&mut self, new_len: usize, filler: u8) {
        for _ in 0..new_len - self.as_slice().len() {
            self.extend_from_slice(&[filler]);
        }
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

    fn resize(&mut self, new_len: usize, filler: u8) {
        self.resize(new_len, filler)
    }
}

#[cfg(feature = "utils-bytes")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "utils-bytes")))]
impl Buffer for bytes::BytesMut {
    fn as_slice(&self) -> &[u8] {
        &self[..]
    }

    fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self[..]
    }

    fn capacity(&self) -> usize {
        bytes::BytesMut::capacity(self)
    }

    fn clear(&mut self) {
        bytes::BytesMut::clear(self)
    }

    fn extend_from_slice(&mut self, extend: &[u8]) {
        bytes::BytesMut::extend_from_slice(self, extend)
    }

    fn resize(&mut self, new_len: usize, filler: u8) {
        bytes::BytesMut::resize(self, new_len, filler)
    }
}

/// A [`Buffer`] wrapper that implements [`std::fmt::Write`].
#[allow(missing_debug_implementations)]
pub struct BufferWriter<'a, B>(pub &'a mut B);

impl<'a, B> std::fmt::Write for BufferWriter<'a, B>
where
    B: Buffer,
{
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.extend_from_slice(s.as_bytes());
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn vec_slicing_is_consistent(mut vec: Vec<u8>) -> bool {
        let buf_as_slice = Vec::from(Buffer::as_mut_slice(&mut vec));
        let buf_as_mut_slice = Vec::from(Buffer::as_slice(&vec));
        let buf_len = Buffer::len(&vec);
        buf_as_slice == buf_as_mut_slice && buf_as_slice.len() == buf_len
    }

    #[quickcheck]
    fn vec_clear_always_removes_content(mut vec: Vec<u8>) -> bool {
        Buffer::clear(&mut vec);
        vec.len() == 0
    }
}
