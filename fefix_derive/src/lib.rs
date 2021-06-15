//! Code generation for FerrumFIX.

#![deny(missing_debug_implementations, clippy::useless_conversion)]

mod derive_field_value;
mod derive_read_fields;

use proc_macro::TokenStream;

#[proc_macro_derive(FixValue, attributes(fefix))]
pub fn derive_field_value(input: TokenStream) -> TokenStream {
    derive_field_value::derive_field_value(input)
}

#[proc_macro_derive(ReadFields, attributes(fefix))]
pub fn derive_read_fields(input: TokenStream) -> TokenStream {
    derive_read_fields::derive_tsr_message(input)
}
