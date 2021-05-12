/// FerrumFIX code generation utilities.

#[cfg(feature = "codegen")]
pub mod codegen;
pub mod dict;

/// Type alias for FIX tags: 16-bit unsigned integers, strictly positive.
pub type TagU16 = std::num::NonZeroU16;
