/// An [`Iterator`] over space-delimited bytes in a
/// [`DataType::MultipleCharValue`](crate::DataType::MultipleCharValue) field.
#[derive(Debug, Clone)]
pub struct DtfMulCharIter<'a, const SEP: u8> {
    data: &'a [u8],
    i: usize,
}

impl<'a, const SEP: u8> DtfMulCharIter<'a, SEP> {
    /// Creates a new [`DtfMulCharIter`] from `data`.
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, i: 0 }
    }
}

impl<'a, const SEP: u8> Iterator for DtfMulCharIter<'a, SEP> {
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
        let dtf = &mut DtfMulCharIter::<b' '>::new(b"");
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn one_char() {
        let dtf = &mut DtfMulCharIter::<b' '>::new(b"t");
        assert_eq!(dtf.next(), Some(b't'));
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn one_char_trailing_space() {
        let dtf = &mut DtfMulCharIter::<b' '>::new(b": ");
        assert_eq!(dtf.next(), Some(b':'));
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn leading_space_not_detected() {
        let dtf = &mut DtfMulCharIter::<b' '>::new(b" f o o b a r");
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), Some(b' '));
        assert_eq!(dtf.next(), None);
    }
}
