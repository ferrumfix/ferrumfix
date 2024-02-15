//! Derive macros for FerrumFIX.

#![deny(missing_debug_implementations, clippy::useless_conversion)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

mod derive_fix_value;

use proc_macro::TokenStream;

/// A *derive macro* for the `FieldType` trait on `enum`'s.
#[proc_macro_derive(FieldType, attributes(fefix))]
pub fn derive_fix_value(input: TokenStream) -> TokenStream {
    derive_fix_value::derive_fix_value(input)
}
