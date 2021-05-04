//! Module definition for build-time features.

#![allow(unused, missing_debug_implementations, clippy::all)]

mod utils;

pub mod app_version;
pub mod codegen;
pub mod dict;

pub use app_version::AppVersion;

pub type TagU16 = std::num::NonZeroU16;
