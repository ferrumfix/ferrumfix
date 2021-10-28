use crate::FixValue;
use std::iter::FusedIterator;
use std::ops::Range;

/// A trait to retrieve field values in a FIX message.
///
/// # Type parameters
///
/// This trait is generic over a lifetime `'a`, which
///
/// # Field getters naming scheme
///
/// All getters start with `fv`, which stands for Field Value.
/// - `l` stands for *lossy*, i.e. invalid field values might not be detected to
/// improve performance.
/// - `_opt` stands for *optional*, for better error reporting.
pub trait FieldAccess<F> {
    /// The type returned by [`FieldAccess::group()`] and [`FieldAccess::group_opt()`].
    type Group: RepeatingGroup<Entry = Self>;

    /// Queries `self` for a group tagged with `key`. An unsuccessful query
    /// results in [`Err(None)`].
    fn group(&self, field: &F) -> Result<Self::Group, Option<<usize as FixValue>::Error>> {
        match self.group_opt(field) {
            Some(Ok(group)) => Ok(group),
            Some(Err(e)) => Err(Some(e)),
            None => Err(None),
        }
    }

    /// Queries `self` for a group tagged with `key` which may or may not be
    /// present in `self`. This differs from
    /// [`FieldAccess::group()`] as missing groups result in [`None`] rather than
    /// [`Err`].
    fn group_opt(&self, field: &F) -> Option<Result<Self::Group, <usize as FixValue>::Error>>;

    /// Queries `self` for `field` and returns its raw contents.
    fn fv_raw(&self, field: &F) -> Option<&[u8]>;

    /// Queries `self` for `field` and deserializes it.
    #[inline]
    fn fv<'a, V>(&'a self, field: &F) -> Result<V, Option<V::Error>>
    where
        V: FixValue<'a>,
    {
        match self.fv_opt(field) {
            Some(Ok(x)) => Ok(x),
            Some(Err(err)) => Err(Some(err)),
            None => Err(None),
        }
    }

    /// Like [`FieldAccess::fv()`], but with lossy deserialization.
    #[inline]
    fn fvl<'a, V>(&'a self, field: &F) -> Result<V, Option<V::Error>>
    where
        V: FixValue<'a>,
    {
        match self.fvl_opt(field) {
            Some(Ok(x)) => Ok(x),
            Some(Err(err)) => Err(Some(err)),
            None => Err(None),
        }
    }

    /// Queries `self` for `field` and deserializes it. This
    /// differs from [`FieldAccess::fv()`] as missing fields result in [`None`]
    /// rather than [`Err`].
    #[inline]
    fn fv_opt<'a, V>(&'a self, field: &F) -> Option<Result<V, V::Error>>
    where
        V: FixValue<'a>,
    {
        self.fv_raw(field).map(|raw| match V::deserialize(raw) {
            Ok(value) => Ok(value),
            Err(err) => Err(err.into()),
        })
    }

    /// Like [`FieldAccess::fv_opt()`], but with lossy deserialization.
    #[inline]
    fn fvl_opt<'a, V>(&'a self, field: &F) -> Option<Result<V, V::Error>>
    where
        V: FixValue<'a>,
    {
        self.fv_raw(field)
            .map(|raw| match V::deserialize_lossy(raw) {
                Ok(value) => Ok(value),
                Err(err) => Err(err.into()),
            })
    }
}

/// Provides access to entries within a FIX repeating group.
pub trait RepeatingGroup: Sized {
    /// The type of entries in this FIX repeating group. Must implement
    /// [`FieldAccess`].
    type Entry;

    /// Returns the number of FIX group entries in `self`.
    fn len(&self) -> usize;

    /// Returns the `i` -th entry in `self`, if present.
    fn entry_opt(&self, i: usize) -> Option<Self::Entry>;

    /// Returns the `i` -th entry in `self`.
    ///
    /// # Panics
    ///
    /// Panics if `i` is outside the legal range of `self`.
    fn entry(&self, i: usize) -> Self::Entry {
        self.entry_opt(i)
            .expect("Index outside bounds of FIX repeating group.")
    }

    /// Creates and returns an [`Iterator`] over the entries in `self`.
    /// Iteration MUST be done in sequential order, i.e. in which they appear in
    /// the original FIX message.
    fn entries(&self) -> GroupEntries<Self> {
        GroupEntries {
            group: self,
            range: 0..self.len(),
        }
    }
}

/// An [`Iterator`] that runs over the entries of a FIX [`RepeatingGroup`].
///
/// This `struct` is created by the method [`RepeatingGroup::entries()`]. It
/// also implements [`FusedIterator`], [`DoubleEndedIterator`], and
/// [`ExactSizeIterator`].
#[derive(Debug, Clone)]
pub struct GroupEntries<'a, G> {
    group: &'a G,
    range: Range<usize>,
}

impl<'a, G> Iterator for GroupEntries<'a, G>
where
    G: RepeatingGroup,
{
    type Item = G::Entry;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.range.next()?;
        Some(self.group.entry(i))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<'a, G> FusedIterator for GroupEntries<'a, G> where G: RepeatingGroup {}

impl<'a, G> DoubleEndedIterator for GroupEntries<'a, G>
where
    G: RepeatingGroup,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let i = self.range.next_back()?;
        Some(self.group.entry(i))
    }
}

impl<'a, G> ExactSizeIterator for GroupEntries<'a, G> where G: RepeatingGroup {}
