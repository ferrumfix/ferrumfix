//! Zero-copy buffering utilities.

use std::io;

/// Operations on a growable in-memory buffer. Superset of [`std::io::Write`].
///
/// [`Buffer`] allows common data operations on in-memory data buffers. All
/// implementors also must infallibly implement [`std::io::Write`]. While
/// [`std::io::Write`] only allows sequential write operations, [`Bufer`] allows
/// arbitrary data manipulation over the whole buffer.
///
/// Please note that calls to [`std::io::Write::flush`] on [`Buffer`]
/// implementors should have **no** effect.
pub trait Buffer
where
    Self: io::Write,
{
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

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::QuickCheck;

    #[test]
    fn vec_slice_operations_are_consistent() {
        fn prop(mut vec: Vec<u8>) -> bool {
            let buf_as_slice = Vec::from(Buffer::as_mut_slice(&mut vec));
            let buf_as_mut_slice = Vec::from(Buffer::as_slice(&vec));
            let buf_len = Buffer::len(&vec);
            buf_as_slice == buf_as_mut_slice && buf_as_slice.len() == buf_len
        }
        QuickCheck::new()
            .tests(1000)
            .quickcheck(prop as fn(Vec<u8>) -> bool)
    }
}
