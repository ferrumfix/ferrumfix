//! FIX message in-memory representations, each tuned for specific operations
//! and performance characteristics.

use crate::StreamIterator;
use std::collections::BTreeMap;
use std::fmt;
use std::str;
use std::time::SystemTime;

pub mod fix42;
mod seqdump;
pub mod slr;
pub mod field_value;

pub use seqdump::PushyMessage;
pub use field_value::FieldValue;
use field_value as val;

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
    /// use fefix::backend::PushyMessage;
    ///
    /// let mut message = PushyMessage::default();
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
    /// use fefix::backend::PushyMessage;
    ///
    /// let mut message = PushyMessage::default();
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

/// An owned value of a FIX field.
#[derive(Clone, Debug, PartialEq)]
pub enum FixFieldValue {
    Atom(val::FieldValue<'static>),
    Group(Vec<BTreeMap<i64, FixFieldValue>>),
}

impl FixFieldValue {
    pub fn string(data: &[u8]) -> Option<Self> {
        str::from_utf8(data)
            .ok()
            .map(|s| Self::Atom(val::FieldValue::string(s.to_string())))
    }

    pub fn as_length(&self) -> Option<usize> {
        if let Self::Atom(val::FieldValue::Length(length)) = self {
            Some((*length).into())
        } else {
            None
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        if let Self::Atom(val::FieldValue::Int(x)) = self {
            Some((*x).into())
        } else {
            None
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        if let Self::Atom(val::FieldValue::String(s)) = self {
            Some(s.as_str())
        } else {
            None
        }
    }
}

impl From<i64> for FixFieldValue {
    fn from(v: i64) -> Self {
        FixFieldValue::Atom(val::FieldValue::int(v as i64))
    }
}

impl From<String> for FixFieldValue {
    fn from(v: String) -> Self {
        FixFieldValue::Atom(val::FieldValue::string(v))
    }
}

impl From<f64> for FixFieldValue {
    fn from(v: f64) -> Self {
        FixFieldValue::Atom(val::FieldValue::float(v as f32))
    }
}

impl From<(u8, u16)> for FixFieldValue {
    fn from(v: (u8, u16)) -> Self {
        FixFieldValue::from(((v.0 as i64) << 16) + (v.1 as i64))
    }
}

impl From<char> for FixFieldValue {
    fn from(v: char) -> Self {
        FixFieldValue::Atom(val::FieldValue::char(v))
    }
}

impl From<usize> for FixFieldValue {
    fn from(v: usize) -> Self {
        FixFieldValue::from(v as i64)
    }
}

impl From<Vec<u8>> for FixFieldValue {
    fn from(v: Vec<u8>) -> Self {
        FixFieldValue::Atom(val::FieldValue::Data(v))
    }
}

impl From<bool> for FixFieldValue {
    fn from(v: bool) -> Self {
        FixFieldValue::from(if v { 't' } else { 'f' })
    }
}

impl From<u8> for FixFieldValue {
    fn from(v: u8) -> Self {
        FixFieldValue::from(i64::from(v))
    }
}

impl From<SystemTime> for FixFieldValue {
    fn from(v: SystemTime) -> Self {
        FixFieldValue::from(v.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64)
    }
}