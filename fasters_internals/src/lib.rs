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
mod tagvalue;
mod version;

pub use crate::err::{Error, Result};
pub use dict::{codegen, Dictionary};
pub use fasters_derive::*;
pub use fix::Fix;
pub use settings::Settings;
pub use version::Version;
