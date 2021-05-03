//! Module definition for build-time features.

#![allow(unused, missing_debug_implementations, clippy::all)]

mod utils;

pub mod app_version;
pub mod codegen;
pub mod dict;

pub use app_version::AppVersion;

use std::num::NonZeroU16;

pub type TagU16 = NonZeroU16;
