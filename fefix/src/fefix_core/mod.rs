/// FerrumFIX code generation utilities.

pub mod codegen;
pub mod dict;

/// Type alias for FIX tags: 16-bit unsigned integers, strictly positive.
pub type TagU16 = std::num::NonZeroU16;
