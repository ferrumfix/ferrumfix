/// An [`Iterator`] over space-delimited bytes in a
/// `MultipleCharValue` field.
///
/// # Example
///
/// ```
/// use fefix::fix_values::MultipleChars;
///
/// let chars = &mut MultipleChars::new(b"a b c");
/// assert_eq!(chars.next(), Some(b'a'));
/// assert_eq!(chars.next(), Some(b'b'));
/// assert_eq!(chars.next(), Some(b'c'));
/// assert_eq!(chars.next(), None);
/// ```
#[derive(Debug, Clone)]
pub struct MultipleChars<'a> {
    data: &'a [u8],
    i: usize,
}

impl<'a> MultipleChars<'a> {
    /// Creates a new [`MultipleChars`] from raw `data`.
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, i: 0 }
    }
}

impl<'a> Iterator for MultipleChars<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.data.len() {
            None
        } else {
            let byte = self.data[self.i];
            self.i += 2;
            Some(byte)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let dtf = &mut MultipleChars::new(b"");
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn one_char() {
        let dtf = &mut MultipleChars::new(b"t");
        assert_eq!(dtf.next(), Some(b't'));
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn one_char_trailing_space() {
        let dtf = &mut MultipleChars::new(b": ");
        assert_eq!(dtf.next(), Some(b':'));
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn leading_space_not_detected() {
        let dtf = &mut MultipleChars::new(b" f o o b a r");
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), None);
    }
}
