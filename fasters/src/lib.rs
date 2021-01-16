//! Fasters is a standard-compliant FIX & FAST (FIX Adapted for STreaming)
//! implementation in pure Rust.
//!
//! FIX and FAST functionality is kept isolated in the
//! [`fasters::fix`](fasters::fix) and [`fasters::fast`](fasters::fast) modules,
//! respectively.

pub mod app;
mod fix_codegen;
mod dictionary;
pub mod encoders;
pub mod engines;
pub mod prelude;
pub mod session;
pub mod transport;

pub use dictionary::Dictionary;
pub use fix_codegen::codegen;

#[cfg(expose_openssl)]
pub extern crate openssl;

#[cfg(not(expose_openssl))]
pub(crate) extern crate openssl;
