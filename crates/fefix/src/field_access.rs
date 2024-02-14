use crate::{FieldType, FieldValueError};
use std::iter::FusedIterator;
use std::ops::Range;

/// Provides random (i.e. non-sequential) access to FIX fields and groups within
/// messages.
///
/// # Methods
///
/// [`FieldMap`] provides two kinds of methods:
///
/// 1. Group getters: [`FieldMap::group`] and
/// [`FieldMap::group_opt`].
///
/// 2. Field getters: [`FieldMap::get_raw`], [`FieldMap::get`],
/// etc..
///
/// The most basic form of field access is done via
/// [`FieldMap::get_raw`], which performs no deserialization at all: it
/// simply returns the bytes contents associated with a FIX field, if found.
///
/// Building upon [`FieldMap::get_raw`] and [`FieldType`], the other
/// field access methods all provide some utility deserialization logic. These
/// methods all have the `get_` prefix, with the following considerations:
///
/// - `get_lossy` methods perform "lossy" deserialization via
/// [`FieldType::deserialize_lossy`]. Unlike lossless deserialization, these
/// methods may skip some error checking logic and thus prove to be faster.
/// Memory-safety is still guaranteed, but malformed FIX fields won't be
/// detected 100% of the time.
/// - `get_opt` methods work exactly like their non-`_opt` counterparties, but they
/// have a different return type: instead of returning [`Err(None)`] for missing
/// fields, these methods return [`None`] for missing fields and
/// [`Some(Ok(field))`] for existing fields.
///
/// # Type parameters
///
/// This trait is generic over a type `F`, which must univocally identify FIX
/// fields (besides FIX repeating groups, which allow repetitions).
pub trait FieldMap<F> {
    /// The type returned by [`FieldMap::group`] and
    /// [`FieldMap::group_opt`].
    type Group: RepeatingGroup<Entry = Self>;

    /// Looks for a `field` within `self` and then returns its raw byte
    /// contents, if it exists.
    fn get_raw(&self, field: F) -> Option<&[u8]>;

    /// Looks for a group that starts with `field` within `self`.
    fn group(&self, field: F) -> Result<Self::Group, FieldValueError<<usize as FieldType>::Error>>;

    /// Like [`FieldMap::group`], but doesn't return an [`Err`] if the
    /// group is missing.
    fn group_opt(&self, field: F) -> Result<Option<Self::Group>, <usize as FieldType>::Error> {
        match self.group(field) {
            Ok(group) => Ok(Some(group)),
            Err(FieldValueError::Missing) => Ok(None),
            Err(FieldValueError::Invalid(e)) => Err(e),
        }
    }

    /// Looks for a `field` within `self` and then decodes its raw byte contents
    /// via [`FieldType::deserialize`], if found.
    fn get<'a, V>(&'a self, field: F) -> Result<V, FieldValueError<V::Error>>
    where
        V: FieldType<'a>,
    {
        self.get_opt(field)
            .map_err(FieldValueError::Invalid)
            .and_then(|opt| opt.ok_or(FieldValueError::Missing))
    }

    /// Like [`FieldMap::get`], but with lossy deserialization.
    fn get_lossy<'a, V>(&'a self, field: F) -> Result<V, FieldValueError<V::Error>>
    where
        V: FieldType<'a>,
    {
        self.get_lossy_opt(field)
            .map_err(FieldValueError::Invalid)
            .and_then(|opt| opt.ok_or(FieldValueError::Missing))
    }

    /// Like [`FieldMap::get`], but doesn't return an [`Err`] if `field`
    /// is missing.
    fn get_opt<'a, V>(&'a self, field: F) -> Result<Option<V>, V::Error>
    where
        V: FieldType<'a>,
    {
        self.get_raw(field).map(V::deserialize).transpose()
    }

    /// Like [`FieldMap::get_opt`], but with lossy deserialization.
    fn get_lossy_opt<'a, V>(&'a self, field: F) -> Result<Option<V>, V::Error>
    where
        V: FieldType<'a>,
    {
        self.get_raw(field).map(V::deserialize_lossy).transpose()
    }
}

/// Provides access to entries within a FIX repeating group.
pub trait RepeatingGroup: Sized {
    /// The type of entries in this FIX repeating group. Must implement
    /// [`FieldMap`].
    type Entry;

    /// Returns the number of FIX group entries in `self`.
    fn len(&self) -> usize;

    /// Returns the `i` -th entry in `self`, if present.
    fn get(&self, i: usize) -> Option<Self::Entry>;

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
        self.group.get(i)
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
        self.group.get(i)
    }
}
