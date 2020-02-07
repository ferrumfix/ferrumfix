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
