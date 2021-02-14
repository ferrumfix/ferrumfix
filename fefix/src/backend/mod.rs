//! FIX message in-memory representations, each tuned for specific operations
//! and performance characteristics.

use rust_embed::RustEmbed;
use std::collections::BTreeMap;
use std::fmt;
use std::time::SystemTime;

pub mod fix42;
mod seqdump;
pub mod slr;
pub mod value;

use value as val;
pub use seqdump::PushyMessage;

/// Allows to iterate sequentially over FIX fields.
pub trait ReadFieldsSeq {
    fn next(&mut self) -> Option<(u32, &FixFieldValue)>;
}

/// Allows fast random lookup of FIX fields..
pub trait ReadFields {
    fn get_field(&self, msg_type: u32) -> Option<&FixFieldValue>;
}

/// Allows appending new FIX field values.
pub trait WriteFields {
    fn set_field(&mut self, msg_type: u32, val: FixFieldValue);
}

/// An owned value of a FIX field.
#[derive(Clone, Debug, PartialEq)]
pub enum FixFieldValue {
    String(String),
    Atom(val::Atomic),
    Group(Vec<BTreeMap<i64, FixFieldValue>>),
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
