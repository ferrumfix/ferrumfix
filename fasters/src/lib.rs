//! Fasters is a standard-compliant FIX & FAST (FIX Adapted for STreaming)
//! implementation in pure Rust.
//!
//! FIX and FAST functionality is kept isolated in the
//! [`fasters::fix`](fasters::fix) and [`fasters::fast`](fasters::fast) modules,
//! respectively.

pub mod app;
mod dictionary;
pub mod encoders;
pub mod engines;
mod fix_codegen;
pub mod fix42;
pub mod prelude;
pub mod session;
pub mod transport;

pub use dictionary::Dictionary;
pub use fix_codegen::codegen;
pub use fasters_derive::*;

#[cfg(expose_openssl)]
pub extern crate openssl;

#[cfg(not(expose_openssl))]
pub(crate) extern crate openssl;
