//! Fasters is a standard-compliant FIX & FAST (FIX Adapted for STreaming)
//! implementation in pure Rust.
//!
//! FIX and FAST functionality is kept isolated in the
//! [`fasters::fix`](fasters::fix) and [`fasters::fast`](fasters::fast) modules,
//! respectively.

#![allow(dead_code)]

pub mod data_types;
/// Dictionaries for several versions of the FIX protocol.
mod dict;
mod err;
/// FAST (FIX Adapted for STreaming)-related functionality.
pub mod fast;
/// FIX-related functionality.
pub mod fix;
/// Provides programmatic access to the FIX Repository.
pub mod repo;
/// Code generation settings.
mod settings;

pub use crate::err::{Error, Result};
use crate::repo::types::Message;
use serde::Deserialize;
use std::io;

pub trait Codec<R, W> {
    type Message;

    fn new_stdio() -> Self;

    fn write_message(&mut self, msg: Self::Message) -> io::Result<()>;

    fn read_message(&self) -> Result<Option<Self::Message>>;
}

pub trait Fix {
    fn as_tagvalue(&self) -> String;
    fn as_fixml(&self) -> String;
    fn gpb(&self) -> String;
    fn sbe(&self) -> String;
    fn json(&self) -> String;
    fn asn1(&self) -> String;
    fn fast(&self) -> String;
}

pub struct Checksum {
    value: u8,
}

impl Checksum {
    fn feed(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.value = self.value.wrapping_add(*byte);
        }
    }

    fn final_value(self) -> u8 {
        self.value
    }
}

/// Available versions of the FIX standard.
#[derive(Copy, Clone, Debug, PartialEq, Deserialize, strum_macros::EnumIter)]
pub enum Version {
    /// Fix 2.7.
    #[serde(rename = "FIX.2.7")]
    Fix27,
    /// Fix 3.0.
    #[serde(rename = "FIX.3.0")]
    Fix30,
    /// Fix 4.0.
    #[serde(rename = "FIX.4.0")]
    Fix40,
    /// Fix 4.1.
    #[serde(rename = "FIX.4.1")]
    Fix41,
    /// Fix 4.2.
    #[serde(rename = "FIX.4.2")]
    Fix42,
    /// Fix 4.3.
    #[serde(rename = "FIX.4.3")]
    Fix43,
    /// Fix 4.4.
    #[serde(rename = "FIX.4.4")]
    Fix44,
    /// Fix 5.0.
    #[serde(rename = "FIX.5.0")]
    Fix50,
    /// Fix 5.0 SP1.
    #[serde(rename = "FIX.5.0SP1")]
    Fix50SP1,
    /// Fix 5.0 SP2.
    #[serde(rename = "FIX.5.0SP2")]
    Fix50SP2,
    /// FIXT 1.1.
    #[serde(rename = "FIXT.1.1")]
    Fixt11,
}

impl Version {
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }

    pub fn iter_supported() -> impl Iterator<Item = Self> {
        Self::iter().filter(|v| v.is_supported())
    }

    pub fn is_supported(self) -> bool {
        // From <https://www.fixtrading.org/standards/unsupported/>
        match self {
            Self::Fix27 => false,
            Self::Fix30 => false,
            Self::Fix40 => false,
            Self::Fix41 => false,
            Self::Fix42 => true,
            Self::Fix43 => false,
            Self::Fix44 => true,
            Self::Fix50 => false,
            Self::Fix50SP1 => false,
            Self::Fix50SP2 => true,
            Self::Fixt11 => true,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fix27 => "FIX.2.7",
            Self::Fix30 => "FIX.3.0",
            Self::Fix40 => "FIX.4.0",
            Self::Fix41 => "FIX.4.1",
            Self::Fix42 => "FIX.4.2",
            Self::Fix43 => "FIX.4.3",
            Self::Fix44 => "FIX.4.4",
            Self::Fix50 => "FIX.5.0",
            Self::Fix50SP1 => "FIX.5.0SP1",
            Self::Fix50SP2 => "FIX.5.0SP2",
            Self::Fixt11 => "FIXT.1.1",
        }
    }

    pub fn header(self) -> Message {
        unimplemented!()
    }

    pub fn trailer(self) -> Message {
        unimplemented!()
    }
}
