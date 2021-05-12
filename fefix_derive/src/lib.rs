//! Code generation for FerrumFIX.

#![deny(missing_debug_implementations, clippy::useless_conversion)]

mod derive_field_value;
mod derive_read_fields;

use proc_macro::{Literal, TokenStream, TokenTree};

#[proc_macro_derive(FixValue, attributes(fefix))]
pub fn derive_field_value(input: TokenStream) -> TokenStream {
    derive_field_value::derive_field_value(input)
}

#[proc_macro_derive(ReadFields, attributes(fefix))]
pub fn derive_read_fields(input: TokenStream) -> TokenStream {
    derive_read_fields::derive_tsr_message(input)
}

// https://docs.rs/bstringify/0.1.2/src/bstringify/lib.rs.html#1-19
#[proc_macro]
pub fn bstringify(input: TokenStream) -> TokenStream {
    TokenTree::from(Literal::byte_string(input.to_string().as_bytes())).into()
}
