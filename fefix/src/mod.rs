//! Module definition for build-time features.

#![allow(unused, missing_debug_implementations, clippy::all)]

mod utils;

pub mod app_version;
pub mod dictionary;
pub mod dt;
pub mod fix_codegen;
pub mod quickfix_specs;

pub use app_version::AppVersion;
pub use dt::DataType;
pub use quickfix_specs::quickfix_spec;
