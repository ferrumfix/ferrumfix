//! FIX message in-memory representations, each tuned for specific operations
//! and performance characteristics.

use crate::StreamIterator;
use rust_embed::RustEmbed;
use std::collections::BTreeMap;
use std::fmt;
use std::str;
use std::time::SystemTime;

pub mod fix42;
mod seqdump;
pub mod slr;
pub mod value;

pub use seqdump::PushyMessage;
use value as val;

/// Allows to iterate sequentially over FIX fields.
pub trait ReadFieldsSeq {
    fn next(&mut self) -> Option<(u32, &FixFieldValue)>;
}

/// An interface for backends that allow field lookup by tag.
pub trait ReadFields {
    /// Returns an immutable reference to the field tagged `tag`, if defined.
    fn get_field(&self, tag: u32) -> Option<&FixFieldValue>;
}

impl<'a, T> ReadFields for &'a T
where
    T: ReadFields,
{
    fn get_field(&self, tag: u32) -> Option<&FixFieldValue> {
        T::get_field(self, tag)
    }
}

pub trait FieldRef<U> {
    fn tag(&self) -> u32;
    fn value(&self) -> &U;
}

/// An interface for FIX backends.
pub trait Backend<U> {
    type Error: fmt::Debug;
    type Iter: StreamIterator<Item = Self::IterItem>;
    type IterItem: FieldRef<U>;

    /// Returns an immutable reference to the field tagged `tag`, if present.
    fn field(&self, tag: u32) -> Option<&U>;

    /// Removes all fields.
    fn clear(&mut self);

    /// Returns the number of fields set in `self`.
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Hbt {
    Header,
    Body,
    Trailer,
}

pub trait HbtBackend<U> {
    type Error: fmt::Debug;
    type Iter: StreamIterator<Item = U>;
}

pub fn fmt<B>(backend: B, f: &mut fmt::Formatter) -> fmt::Result
where
    B: Backend<FixFieldValue>,
{
    backend.for_each::<fmt::Error, _>(|tag, value| {
        write!(f, "{}=|", tag)?;
        match value {
            FixFieldValue::String(s) => {
                write!(f, "{}|", s)
            }
            FixFieldValue::Atom(a) => {
                write!(f, "{}|", a)
            }
            _ => Ok(()),
        }?;
        Ok(())
    })?;
    Ok(())
}

/// An owned value of a FIX field.
#[derive(Clone, Debug, PartialEq)]
pub enum FixFieldValue {
    String(String),
    Atom(val::Atomic),
    Group(Vec<BTreeMap<i64, FixFieldValue>>),
}

impl FixFieldValue {
    pub fn string(data: &[u8]) -> Option<Self> {
        str::from_utf8(data)
            .ok()
            .map(|s| Self::String(s.to_string()))
    }

    pub fn as_length(&self) -> Option<usize> {
        if let Self::Atom(val::Atomic::Length(length)) = self {
            Some((*length).into())
        } else {
            None
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        if let Self::Atom(val::Atomic::Int(x)) = self {
            Some((*x).into())
        } else {
            None
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        if let Self::String(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }
}

impl From<i64> for FixFieldValue {
    fn from(v: i64) -> Self {
        FixFieldValue::Atom(val::Atomic::int(v as i64))
    }
}

impl From<String> for FixFieldValue {
    fn from(v: String) -> Self {
        FixFieldValue::String(v)
    }
}

impl From<f64> for FixFieldValue {
    fn from(v: f64) -> Self {
        FixFieldValue::Atom(val::Atomic::float(v as f32))
    }
}

impl From<(u8, u16)> for FixFieldValue {
    fn from(v: (u8, u16)) -> Self {
        FixFieldValue::from(((v.0 as i64) << 16) + (v.1 as i64))
    }
}

impl From<char> for FixFieldValue {
    fn from(v: char) -> Self {
        FixFieldValue::Atom(val::Atomic::char(v))
    }
}

impl From<usize> for FixFieldValue {
    fn from(v: usize) -> Self {
        FixFieldValue::from(v as i64)
    }
}

impl From<Vec<u8>> for FixFieldValue {
    fn from(v: Vec<u8>) -> Self {
        FixFieldValue::Atom(val::Atomic::Data(v))
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

/// Which [`Dictionary`](fefix::Dictionary) version to use.
#[derive(Copy, Debug, Clone)]
#[non_exhaustive]
pub enum Version {
    Fix40,
    Fix41,
    Fix42,
    Fix43,
    Fix44,
    Fix50,
    Fix50SP1,
    Fix50SP2,
    Fixt11,
}

impl Version {
    /// Returns a [`String`](String) with the QuickFIX definition file for `self`
    /// as its
    /// content. The QuickFix definition files are extracted and decompressed
    /// from the binary without filesystem access.
    pub fn get_quickfix_spec(&self) -> String {
        let filename = match self {
            Version::Fix40 => "FIX-4.0.xml",
            Version::Fix41 => "FIX-4.1.xml",
            Version::Fix42 => "FIX-4.2.xml",
            Version::Fix43 => "FIX-4.3.xml",
            Version::Fix44 => "FIX-4.4.xml",
            Version::Fix50 => "FIX-5.0.xml",
            Version::Fix50SP1 => "FIX-5.0-SP1.xml",
            Version::Fix50SP2 => "FIX-5.0-SP2.xml",
            Version::Fixt11 => "FIXT-1.1.xml",
        };
        let xml_spec = QuickFixDicts::get(filename).expect(filename);
        std::str::from_utf8(&*xml_spec).unwrap().to_string()
    }

    #[cfg(test)]
    pub(crate) fn all() -> impl Iterator<Item = Self> {
        vec![
            Version::Fix40,
            Version::Fix41,
            Version::Fix42,
            Version::Fix43,
            Version::Fix44,
            Version::Fix50,
            Version::Fix50SP1,
            Version::Fix50SP2,
            Version::Fixt11,
        ]
        .into_iter()
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let as_str = match self {
            Version::Fix40 => "FIX-4.0",
            Version::Fix41 => "FIX-4.1",
            Version::Fix42 => "FIX-4.2",
            Version::Fix43 => "FIX-4.3",
            Version::Fix44 => "FIX-4.4",
            Version::Fix50 => "FIX-5.0",
            Version::Fix50SP1 => "FIX-5.0-SP1",
            Version::Fix50SP2 => "FIX-5.0-SP2",
            Version::Fixt11 => "FIXT-1.1",
        };
        write!(f, "{}", as_str)
    }
}

#[derive(RustEmbed)]
#[folder = "resources/quickfix/"]
struct QuickFixDicts;

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn all_versions_have_quickfix_spec() {
        assert!(Version::all()
            .map(|version| version.get_quickfix_spec())
            .all(|spec| spec.len() > 0));
    }

    #[test]
    fn all_versions_have_different_quickfix_spec() {
        let mut set: HashSet<String> = HashSet::default();
        Version::all()
            .map(|version| set.insert(version.get_quickfix_spec()))
            .count();
        assert_eq!(set.len(), Version::all().count());
    }

    #[test]
    fn all_versions_have_xml_valid_quickfix_spec() {
        assert!(Version::all()
            .map(|version| version.get_quickfix_spec())
            .all(|spec| roxmltree::Document::parse(spec.as_str()).is_ok()));
    }
}
