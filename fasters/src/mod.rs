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
pub use fasters_derive::derive;
pub use settings::Settings;
pub use version::Version;
