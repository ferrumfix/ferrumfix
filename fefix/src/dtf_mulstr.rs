/// An [`Iterator`] over space-delimited byte sequences in a
/// [`DataType::MultipleStringValue`](crate::DataType::MultipleStringValue) field.
#[derive(Debug, Clone)]
pub struct DtfMulStrIter<'a, const SEP: u8> {
    data: &'a [u8],
    i: usize,
}

impl<'a, const SEP: u8> DtfMulStrIter<'a, SEP> {
    /// Creates a new [`DtfMulStrIter`] from `data`.
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, i: 0 }
    }
}

impl<'a, const SEP: u8> Iterator for DtfMulStrIter<'a, SEP> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.data.len() {
            return None;
        }
        let i_start = self.i + self.data[self.i..].iter().position(|byte| *byte != SEP)?;
        let i_end = i_start
            + self.data[i_start..]
                .iter()
                .position(|byte| *byte == SEP)
                .unwrap_or(self.data.len() - i_start);
        self.i = i_end;
        Some(&self.data[i_start..i_end])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_data_has_no_values() {
        let dtf = &mut DtfMulStrIter::<b' '>::new(&[]);
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn whitespace_has_no_values() {
        let dtf = &mut DtfMulStrIter::<b' '>::new(b"     ");
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn a_single_word_has_one_value() {
        let dtf = &mut DtfMulStrIter::<b' '>::new(b"foobar");
        assert_eq!(dtf.next(), Some(b"foobar" as &[u8]));
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn words_separated_by_space() {
        let dtf = &mut DtfMulStrIter::<b' '>::new(b"foo bar spam");
        assert_eq!(dtf.next(), Some(b"foo" as &[u8]));
        assert_eq!(dtf.next(), Some(b"bar" as &[u8]));
        assert_eq!(dtf.next(), Some(b"spam" as &[u8]));
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn word_with_trailing_space() {
        let dtf = &mut DtfMulStrIter::<b' '>::new(b"foo ");
        assert_eq!(dtf.next(), Some(b"foo" as &[u8]));
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn words_with_trailing_spaces() {
        let dtf = &mut DtfMulStrIter::<b' '>::new(b"foo  bar    ");
        assert_eq!(dtf.next(), Some(b"foo" as &[u8]));
        assert_eq!(dtf.next(), Some(b"bar" as &[u8]));
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn words_with_leading_spaces() {
        let dtf = &mut DtfMulStrIter::<b' '>::new(b"  \t  bar spam");
        assert_eq!(dtf.next(), Some(b"\t" as &[u8]));
        assert_eq!(dtf.next(), Some(b"bar" as &[u8]));
        assert_eq!(dtf.next(), Some(b"spam" as &[u8]));
        assert_eq!(dtf.next(), None);
    }
}
