//! Fasters is a standard-compliant FIX & FAST (FIX Adapted for STreaming)
//! implementation in pure Rust.
//!
//! FIX and FAST functionality is kept isolated in the
//! [`fasters::fix`](fasters::fix) and [`fasters::fast`](fasters::fast) modules,
//! respectively.

pub mod codegen;
mod dictionary;
pub mod engines;
pub mod presentation;
#[deprecated]
pub mod internals;
pub mod ir;
pub mod session;
pub mod sofh;
pub mod transport;

pub use dictionary::Dictionary;

#[cfg(expose_openssl)]
pub extern crate openssl;

#[cfg(not(expose_openssl))]
pub(crate) extern crate openssl;