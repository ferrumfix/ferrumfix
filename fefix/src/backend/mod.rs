//! FIX message in-memory representations, each tuned for specific operations
//! and performance characteristics.

use crate::tagvalue::FixFieldValue;
use crate::StreamIterator;
use std::fmt;

pub mod field_value;

pub use field_value::FieldValue;

pub trait FieldRef<U> {
    fn tag(&self) -> u32;
    fn value(&self) -> &U;
}

/// A [`Dictionary`](crate::Dictionary) -agnostic FIX message representation.
pub trait Backend<U = FixFieldValue> {
    /// The type returned in the event of a fatal error when defining new fields.
    type Error: fmt::Debug;
    type Iter: StreamIterator<Item = Self::IterItem>;
    type IterItem: FieldRef<U>;

    /// Returns an immutable reference to a specific field by `tag`, if present.
    fn field(&self, tag: u32) -> Option<&U>;

    /// Removes all fields and resets `self` to its empty state.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::backend::Backend;
    /// use fefix::backend::MessageSeq;
    ///
    /// let mut message = MessageSeq::default();
    /// assert_eq!(message.len(), 0);
    /// message.insert(35, FixFieldValue::String("A")).unwrap();
    /// assert_eq!(message.len(), 1);
    /// message.clear();
    /// assert_eq!(message.len(), 0);
    /// ```
    fn clear(&mut self);

    /// Returns the number of fields set in `self`. This counter gets incremented
    /// by one at each call of [`Backend::insert`](Backend::insert).
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::backend::Backend;
    /// use fefix::backend::MessageSeq;
    ///
    /// let mut message = MessageSeq::default();
    /// assert_eq!(message.len(), 0);
    /// message.insert(8, FixFieldValue::String("FIX.4.4")).unwrap();
    /// message.insert(35, FixFieldValue::String("D")).unwrap();
    /// assert_eq!(message.len(), 2);
    /// ```
    fn len(&self) -> usize;

    /// Defines a new field with value `value` and tag `tag`.
    fn insert(&mut self, tag: u32, value: U) -> Result<(), Self::Error>;

    /// Calls a function `f` for every field in `self`.
    fn for_each<E, F>(&self, f: F) -> Result<(), E>
    where
        F: FnMut(u32, &U) -> Result<(), E>;

    /// Calls a function `f` for every field in `self`.
    fn iter_fields(&mut self) -> &mut Self::Iter;
}
