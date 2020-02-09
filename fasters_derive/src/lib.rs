//! Fasters is a standard-compliant FIX & FAST (FIX Adapted for STreaming)
//! implementation in pure Rust.
//!
//! FIX and FAST functionality is kept isolated in the
//! [`fasters::fix`](fasters::fix) and [`fasters::fast`](fasters::fast) modules,
//! respectively.

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn derive(tokens: TokenStream) -> TokenStream {
    unimplemented!()
}
