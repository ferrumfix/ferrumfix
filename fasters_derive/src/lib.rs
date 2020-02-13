//! Fasters is a standard-compliant FIX & FAST (FIX Adapted for STreaming)
//! implementation in pure Rust.
//!
//! FIX and FAST functionality is kept isolated in the
//! [`fasters::fix`](fasters::fix) and [`fasters::fast`](fasters::fast) modules,
//! respectively.

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Fix)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_fix(&ast)
}

fn impl_fix(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Fix for #name {
            fn des(bytes: &[u8]) -> Result<Self> {
                unimplemented!()
            }

            fn ser(w: impl Write) -> Result<usize> {
                unimplemented!()
            }

            fn des_fixml(bytes: &[u8]) -> Result<Self> {
                unimplemented!()
            }

            fn ser_fixml(w: impl Write) -> Result<usize> {
                unimplemented!()
            }
        }
    };
    gen.into()
}
