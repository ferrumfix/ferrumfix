//! Module definition for build-time features.

#![allow(unused, missing_debug_implementations, clippy::all)]

mod utils;

pub mod app_version;
pub mod codegen;
pub mod dict;
pub mod fix_data_type;

pub use app_version::AppVersion;
pub use fix_data_type::FixDataType;

use std::num::NonZeroU16;

pub type TagU16 = NonZeroU16;
