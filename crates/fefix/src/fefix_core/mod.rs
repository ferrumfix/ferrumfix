//! FerrumFIX code generation utilities.

#[cfg(feature = "codegen")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "codegen")))]
pub mod codegen;
pub mod dict;

/// Type alias for FIX tags: 32-bit unsigned integers, strictly positive.
pub type TagU32 = std::num::NonZeroU32;
