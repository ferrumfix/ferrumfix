use std::iter::FusedIterator;

const SEPARATOR: u8 = b' ';

/// An [`Iterator`] over space-delimited byte sequences in a
/// `MultipleStringValue` FIX field.
///
/// # Example
///
/// ```
/// use fefix::fix_values::MultipleStrings;
///
/// let words = &mut MultipleStrings::new(b"foo bar qwerty");
/// assert_eq!(words.next(), Some("foo".as_bytes()));
/// assert_eq!(words.next(), Some("bar".as_bytes()));
/// assert_eq!(words.next(), Some("qwerty".as_bytes()));
/// assert_eq!(words.next(), None);
/// ```
#[derive(Debug, Clone)]
pub struct MultipleStrings<'a> {
    data: &'a [u8],
    i: usize,
}

impl<'a> MultipleStrings<'a> {
    /// Creates a new [`MultipleStrings`] from `data`.
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, i: 0 }
    }
}

impl<'a> Iterator for MultipleStrings<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.data.len() {
            return None;
        }
        let i_start = self.i
            + self.data[self.i..]
                .iter()
                .position(|byte| *byte != SEPARATOR)?;
        let i_end = i_start
            + self.data[i_start..]
                .iter()
                .position(|byte| *byte == SEPARATOR)
                .unwrap_or(self.data.len() - i_start);
        self.i = i_end;
        Some(&self.data[i_start..i_end])
    }
}

impl<'a> FusedIterator for MultipleStrings<'a> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_data_has_no_values() {
        let dtf = &mut MultipleStrings::new(&[]);
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn whitespace_has_no_values() {
        let dtf = &mut MultipleStrings::new(b"     ");
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn a_single_word_has_one_value() {
        let dtf = &mut MultipleStrings::new(b"foobar");
        assert_eq!(dtf.next(), Some(b"foobar" as &[u8]));
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn words_separated_by_space() {
        let dtf = &mut MultipleStrings::new(b"foo bar spam");
        assert_eq!(dtf.next(), Some(b"foo" as &[u8]));
        assert_eq!(dtf.next(), Some(b"bar" as &[u8]));
        assert_eq!(dtf.next(), Some(b"spam" as &[u8]));
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn word_with_trailing_space() {
        let dtf = &mut MultipleStrings::new(b"foo ");
        assert_eq!(dtf.next(), Some(b"foo" as &[u8]));
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn words_with_trailing_spaces() {
        let dtf = &mut MultipleStrings::new(b"foo  bar    ");
        assert_eq!(dtf.next(), Some(b"foo" as &[u8]));
        assert_eq!(dtf.next(), Some(b"bar" as &[u8]));
        assert_eq!(dtf.next(), None);
    }

    #[test]
    fn words_with_leading_spaces() {
        let dtf = &mut MultipleStrings::new(b"  \t  bar spam");
        assert_eq!(dtf.next(), Some(b"\t" as &[u8]));
        assert_eq!(dtf.next(), Some(b"bar" as &[u8]));
        assert_eq!(dtf.next(), Some(b"spam" as &[u8]));
        assert_eq!(dtf.next(), None);
    }
}
