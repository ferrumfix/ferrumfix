//! Fasters is a standard-compliant FIX & FAST (FIX Adapted for STreaming)
//! implementation in pure Rust.
//!
//! FIX and FAST functionality is kept isolated in the
//! [`fasters::fix`](fasters::fix) and [`fasters::fast`](fasters::fast) modules,
//! respectively.

#![allow(dead_code)]

mod err;
/// FAST (FIX Adapted for STreaming)-related functionality.
pub mod fast;
/// FIX-related functionality.
pub mod fix;
/// Interface with the FIX repository.
pub mod repo;
mod settings;

pub use crate::err::{Error, Result};

use std::io;

pub trait Codec<R, W> {
    type Message;

    fn new_stdio() -> Self;

    fn write_message(&mut self, msg: Self::Message) -> io::Result<()>;

    fn read_message(&self) -> Result<Option<Self::Message>>;
}
