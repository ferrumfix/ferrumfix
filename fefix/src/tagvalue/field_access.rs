use std::iter::FusedIterator;

use crate::dict::IsFieldDefinition;
use crate::FixValue;
use crate::{OptError, OptResult};

/// Provides access to entries within a FIX repeating group.
pub trait RepeatingGroup: Sized {
    /// The type of entries in this FIX repeating group. Must implement
    /// [`FieldAccess`].
    type Entry: FieldAccess;

    /// Returns the number of FIX group entries in `self`.
    fn len(&self) -> usize;

    /// Returns the `i` -th entry in `self`.
    ///
    /// # Panics
    ///
    /// This method will panic if and only if `i` is outside the legal range of
    /// `self`.
    fn entry(&self, i: usize) -> Self::Entry;

    /// Creates and returns an [`Iterator`] over the entries in `self`.
    /// Iteration MUST be done in sequential order, i.e. in which they appear in
    /// the original FIX message.
    fn entries(&self) -> Entries<Self> {
        Entries {
            group: self,
            i: 0,
            max_i_plus_one: self.len(),
        }
    }
}

/// An [`Iterator`] that runs over the entries of a FIX [`RepeatingGroup`].
///
/// This `struct` is created by the method [`RepeatingGroup::entries()`]. It
/// also implements [`FusedIterator`], [`DoubleEndedIterator`], and
/// [`ExactSizeIterator`].
#[derive(Debug, Clone)]
pub struct Entries<'a, G> {
    group: &'a G,
    i: usize,
    max_i_plus_one: usize,
}

impl<'a, G> Iterator for Entries<'a, G>
where
    G: RepeatingGroup,
{
    type Item = G::Entry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.max_i_plus_one {
            None
        } else {
            let entry = self.group.entry(self.i);
            self.i += 1;
            Some(entry)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.group.len() - self.i;
        (n, Some(n))
    }
}

impl<'a, G> FusedIterator for Entries<'a, G> where G: RepeatingGroup {}

impl<'a, G> DoubleEndedIterator for Entries<'a, G>
where
    G: RepeatingGroup,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.i == self.max_i_plus_one {
            None
        } else {
            self.max_i_plus_one -= 1;
            let entry = self.group.entry(self.max_i_plus_one);
            Some(entry)
        }
    }
}

impl<'a, G> ExactSizeIterator for Entries<'a, G>
where
    G: RepeatingGroup,
{
    fn len(&self) -> usize {
        self.group.len()
    }
}

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
pub trait FieldAccess {
    /// The type returned by [`FieldAccess::group()`] and [`FieldAccess::group_opt()`].
    type Group: RepeatingGroup;

    /// Queries `self` for a group tagged with `key`. An unsuccessful query
    /// results in [`OptError::None`].
    fn group<F>(&self, field: &F) -> OptResult<Self::Group, <usize as FixValue>::Error>
    where
        F: IsFieldDefinition,
    {
        match self.group_opt(field) {
            Some(Ok(group)) => Ok(group),
            Some(Err(e)) => Err(OptError::Other(e)),
            None => Err(OptError::None),
        }
    }

    /// Queries `self` for a group tagged with `key` which may or may not be
    /// present in `self`. This differs from
    /// [`FieldAccess::group()`] as missing groups result in [`None`] rather than
    /// [`Err`].
    fn group_opt<F>(&self, field: &F) -> Option<Result<Self::Group, <usize as FixValue>::Error>>
    where
        F: IsFieldDefinition;

    /// Queries `self` for `field` and returns its raw contents, which can then
    fn fv_raw<F>(&self, field: &F) -> Option<&[u8]>
    where
        F: IsFieldDefinition;

    /// Queries `self` for `field` and deserializes it.
    #[inline]
    fn fv<'a, V, F>(&'a self, field: &F) -> OptResult<V, V::Error>
    where
        V: FixValue<'a>,
        F: IsFieldDefinition,
    {
        match self.fv_opt(field) {
            Some(Ok(x)) => Ok(x),
            Some(Err(err)) => Err(OptError::Other(err)),
            None => Err(OptError::None),
        }
    }

    /// Like [`FieldAccess::fv()`], but with lossy deserialization.
    #[inline]
    fn fvl<'a, V, F>(&'a self, field: &F) -> OptResult<V, V::Error>
    where
        V: FixValue<'a>,
        F: IsFieldDefinition,
    {
        match self.fvl_opt(field) {
            Some(Ok(x)) => Ok(x),
            Some(Err(err)) => Err(OptError::Other(err)),
            None => Err(OptError::None),
        }
    }

    /// Queries `self` for `field` and deserializes it. This
    /// differs from [`FieldAccess::fv()`] as missing fields result in [`None`]
    /// rather than [`Err`].
    #[inline]
    fn fv_opt<'a, V, F>(&'a self, field: &F) -> Option<Result<V, V::Error>>
    where
        V: FixValue<'a>,
        F: IsFieldDefinition,
    {
        self.fv_raw(field).map(|raw| match V::deserialize(raw) {
            Ok(value) => Ok(value),
            Err(err) => Err(err.into()),
        })
    }

    /// Like [`FieldAccess::fv_opt()`], but with lossy deserialization.
    #[inline]
    fn fvl_opt<'a, V, F>(&'a self, field: &F) -> Option<Result<V, V::Error>>
    where
        V: FixValue<'a>,
        F: IsFieldDefinition,
    {
        self.fv_raw(field)
            .map(|raw| match V::deserialize_lossy(raw) {
                Ok(value) => Ok(value),
                Err(err) => Err(err.into()),
            })
    }
}
