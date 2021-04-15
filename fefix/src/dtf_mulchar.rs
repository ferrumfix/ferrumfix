/// An [`Iterator`] over space-delimited bytes in a
/// [`DataType::MultipleCharValue`](crate::DataType::MultipleCharValue) field.
#[derive(Debug, Clone)]
pub struct DtfMulCharIter<'a> {
    data: &'a [u8],
    i: usize,
}

impl<'a> DtfMulCharIter<'a> {
    /// Creates a new [`DtfMulCharIter`] from `data`.
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, i: 0 }
    }
}

impl<'a> Iterator for DtfMulCharIter<'a> {
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
        let dtf = &mut DtfMulCharIter::new(b"");
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn one_char() {
        let dtf = &mut DtfMulCharIter::new(b"t");
        assert_eq!(dtf.next(), Some(b't'));
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn one_char_trailing_space() {
        let dtf = &mut DtfMulCharIter::new(b": ");
        assert_eq!(dtf.next(), Some(b':'));
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn leading_space_not_detected() {
        let dtf = &mut DtfMulCharIter::new(b" f o o b a r");
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), None);
    }
}
