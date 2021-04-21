//! Code generation for FerrumFIX.

#![deny(missing_debug_implementations, clippy::useless_conversion)]

mod derive_data_field;
mod derive_read_fields;

use proc_macro::{Literal, TokenStream, TokenTree};

#[proc_macro_derive(DataField, attributes(fefix))]
pub fn derive_data_field(input: TokenStream) -> TokenStream {
    derive_data_field::derive_data_field(input)
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
