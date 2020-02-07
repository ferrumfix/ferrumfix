//! Fasters is a standard-compliant FIX & FAST (FIX Adapted for STreaming)
//! implementation in pure Rust.
//!
//! FIX and FAST functionality is kept isolated in the
//! [`fasters::fix`](fasters::fix) and [`fasters::fast`](fasters::fast) modules,
//! respectively.

#![allow(dead_code)]

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
    fn sbe(&self) -> String;
}

/// Available versions of the FIX standard.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum Version {
    /// Unsupported.
    #[serde(rename = "FIX.2.7")]
    Fix27,
    /// Unsupported.
    #[serde(rename = "FIX.3.0")]
    Fix30,
    /// Unsupported.
    #[serde(rename = "FIX.4.0")]
    Fix40,
    /// Unsupported.
    #[serde(rename = "FIX.4.1")]
    Fix41,
    /// Currently supported.
    #[serde(rename = "FIX.4.2")]
    Fix42,
    /// Unsupported.
    #[serde(rename = "FIX.4.3")]
    Fix43,
    /// Currently supported.
    #[serde(rename = "FIX.4.4")]
    Fix44,
    /// Unsupported.
    #[serde(rename = "FIX.5.0")]
    Fix50,
    /// Unsupported.
    #[serde(rename = "FIX.5.0SP1")]
    Fix50SP1,
    /// Currently supported.
    #[serde(rename = "FIX.5.0SP2")]
    Fix50SP2,
    /// Currently supported.
    #[serde(rename = "FIXT.1.1")]
    Fixt11,
}

impl Version {
    pub fn all() -> impl Iterator<Item = &'static &'static str> {
        [
            "FIX.4.0",
            "FIX.4.1",
            "FIX.4.2",
            "FIX.4.3",
            "FIX.4.4",
            "FIX.5.0",
            "FIX.5.0SP1",
            "FIX.5.0SP2",
            "FIXT.1.1",
        ]
        .iter()
    }

    pub fn to_str(&self) -> &'static str {
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
}
