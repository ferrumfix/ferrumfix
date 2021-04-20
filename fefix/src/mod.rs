//! Module definition for build-time features.

#![allow(unused, missing_debug_implementations, clippy::all)]

mod utils;

pub mod app_version;
pub mod codegen;
pub mod data_type;
pub mod dictionary;
pub mod quickfix_specs;

pub use app_version::AppVersion;
pub use data_type::DataType;
pub use quickfix_specs::quickfix_spec;
