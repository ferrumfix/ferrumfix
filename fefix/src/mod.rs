//! Module definition for build-time features.

#![allow(unused, missing_debug_implementations, clippy::all)]

mod utils;

pub mod app_version;
pub mod codegen;
pub mod dictionary;
pub mod fix_data_type;
pub mod quickfix_specs;

pub use app_version::AppVersion;
pub use fix_data_type::FixDataType;
pub use quickfix_specs::quickfix_spec;

use std::num::NonZeroU16;

pub type TagU16 = NonZeroU16;
