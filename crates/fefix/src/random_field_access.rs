use crate::FieldType;
use std::iter::FusedIterator;
use std::ops::Range;

/// Provides random (i.e. non-sequential) access to FIX fields and groups within
/// messages.
///
/// # Methods
///
/// [`RandomFieldAccess`] provides two kinds of methods:
///
/// 1. Group getters: [`RandomFieldAccess::group`] and
/// [`RandomFieldAccess::group_opt`].
///
/// 2. Field getters: [`RandomFieldAccess::fv_raw`], [`RandomFieldAccess::fv`],
/// etc..
///
/// The most basic form of field access is done via
/// [`RandomFieldAccess::fv_raw`], which performs no deserialization at all: it
/// simply returns the bytes contents associated with a FIX field, if found.
///
/// Building upon [`RandomFieldAccess::fv_raw`] and [`FieldType`], the other
/// field access methods all provide some utility deserialization logic. These
/// methods all have the `fv` prefix, with the following considerations:
///
/// - `fvl` methods perform "lossy" deserialization via
/// [`FieldType::deserialize_lossy`]. Unlike lossless deserialization, these
/// methods may skip some error checking logic and thus prove to be faster.
/// Memory-safety is still guaranteed, but malformed FIX fields won't be
/// detected 100% of the time.
/// - `_opt` methods work exactly like their non-`_opt` counterparties, but they
/// have a different return type: instead of returning [`Err(None)`] for missing
/// fields, these methods return [`None`] for missing fields and
/// [`Some(Ok(field))`] for existing fields.
///
/// # Type parameters
///
/// This trait is generic over a type `F`, which must univocally identify FIX
/// fields (besides FIX repeating groups, which allow repetitions).
pub trait RandomFieldAccess<F> {
    /// The type returned by [`RandomFieldAccess::group`] and
    /// [`RandomFieldAccess::group_opt`].
    type Group: RepeatingGroup<Entry = Self>;

    /// Looks for a `field` within `self` and then returns its raw byte
    /// contents, if it exists.
    fn fv_raw(&self, field: F) -> Option<&[u8]>;

    /// Like [`RandomFieldAccess::group`], but doesn't return an [`Err`] if the
    /// group is missing.
    fn group_opt(&self, field: F) -> Option<Result<Self::Group, <usize as FieldType>::Error>>;

    /// Looks for a group that starts with `field` within `self`.
    #[inline]
    fn group(&self, field: F) -> Result<Self::Group, Option<<usize as FieldType>::Error>> {
        match self.group_opt(field) {
            Some(Ok(group)) => Ok(group),
            Some(Err(e)) => Err(Some(e)),
            None => Err(None),
        }
    }

    /// Looks for a `field` within `self` and then decodes its raw byte contents
    /// via [`FieldType::deserialize`], if found.
    #[inline]
    fn fv<'a, V>(&'a self, field: F) -> Result<V, Option<V::Error>>
    where
        V: FieldType<'a>,
    {
        match self.fv_opt(field) {
            Some(Ok(x)) => Ok(x),
            Some(Err(err)) => Err(Some(err)),
            None => Err(None),
        }
    }

    /// Like [`RandomFieldAccess::fv`], but with lossy deserialization.
    #[inline]
    fn fvl<'a, V>(&'a self, field: F) -> Result<V, Option<V::Error>>
    where
        V: FieldType<'a>,
    {
        match self.fvl_opt(field) {
            Some(Ok(x)) => Ok(x),
            Some(Err(err)) => Err(Some(err)),
            None => Err(None),
        }
    }

    /// Like [`RandomFieldAccess::fv`], but doesn't return an [`Err`] if `field`
    /// is missing.
    #[inline]
    fn fv_opt<'a, V>(&'a self, field: F) -> Option<Result<V, V::Error>>
    where
        V: FieldType<'a>,
    {
        self.fv_raw(field).map(|raw| match V::deserialize(raw) {
            Ok(value) => Ok(value),
            Err(err) => Err(err.into()),
        })
    }

    /// Like [`RandomFieldAccess::fv_opt`], but with lossy deserialization.
    #[inline]
    fn fvl_opt<'a, V>(&'a self, field: F) -> Option<Result<V, V::Error>>
    where
        V: FieldType<'a>,
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
    /// [`RandomFieldAccess`].
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

/// An [`Iterator`] over the entries of a FIX repeating group.
///
/// This `struct` is created by the method [`RepeatingGroup::entries`]. It
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
impl<'a, G> ExactSizeIterator for GroupEntries<'a, G> where G: RepeatingGroup {}

impl<'a, G> DoubleEndedIterator for GroupEntries<'a, G>
where
    G: RepeatingGroup,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let i = self.range.next_back()?;
        Some(self.group.entry(i))
    }
}
