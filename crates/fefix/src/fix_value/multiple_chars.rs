use std::iter::FusedIterator;

/// An [`Iterator`] over space-delimited bytes in a
/// `MultipleCharValue` FIX field.
///
/// # Example
///
/// ```
/// use fefix::fix_value::MultipleChars;
///
/// let mut chars = MultipleChars::new(b"a b c");
/// assert_eq!(chars.next(), Some(b'a'));
/// assert_eq!(chars.next(), Some(b'b'));
/// assert_eq!(chars.next(), Some(b'c'));
/// assert_eq!(chars.next(), None);
/// ```
#[derive(Debug, Clone)]
pub struct MultipleChars<'a> {
    data: &'a [u8],
}

impl<'a> MultipleChars<'a> {
    /// Creates a new [`MultipleChars`] from raw `data`.
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
}

impl<'a> Iterator for MultipleChars<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(byte) = self.data.get(0).copied() {
            self.data = &self.data.get(2..).unwrap_or(&[]);
            Some(byte)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let num_elements = (self.data.len() + 1) / 2;
        (num_elements, Some(num_elements))
    }
}

impl<'a> ExactSizeIterator for MultipleChars<'a> {
    fn len(&self) -> usize {
        (self.data.len() + 1) / 2
    }
}

impl<'a> DoubleEndedIterator for MultipleChars<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(byte) = self.data.last().copied() {
            self.data = &self.data.get(..self.data.len() - 2).unwrap_or(&[]);
            Some(byte)
        } else {
            None
        }
    }
}

impl<'a> FusedIterator for MultipleChars<'a> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let mut dtf = MultipleChars::new(b"");
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn one_char() {
        let mut dtf = MultipleChars::new(b"t");
        assert_eq!(dtf.next(), Some(b't'));
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn one_char_trailing_space() {
        let mut dtf = MultipleChars::new(b": ");
        assert_eq!(dtf.next(), Some(b':'));
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn leading_space_not_detected() {
        let mut dtf = MultipleChars::new(b" f o o b a r");
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), None);
    }
}
