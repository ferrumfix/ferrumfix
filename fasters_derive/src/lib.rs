//! Fasters is a standard-compliant FIX & FAST (FIX Adapted for STreaming)
//! implementation in pure Rust.
//!
//! FIX and FAST functionality is kept isolated in the
//! [`fasters::fix`](fasters::fix) and [`fasters::fast`](fasters::fast) modules,
//! respectively.

extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

#[proc_macro_derive(Fix)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    let output = impl_fix(&ast);
    proc_macro::TokenStream::from(output)
}

fn impl_fix(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let des = generate_des(&ast.data);
    let gen = quote! {
        impl Fix for #name {
            fn des(bytes: &[u8]) -> Result<Self> {
                #des
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
    TokenStream::from(gen)
}

fn generate_des(data: &syn::Data) -> TokenStream {
    (match *data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => {
                let quoted = fields.named.iter().map(|f| "");
                quote! {
                    unimplemented()
                }
            }
            syn::Fields::Unit | syn::Fields::Unnamed(_) => panic!("struct must be named"),
        },
        syn::Data::Enum(_) | syn::Data::Union(_) => panic!("Fix messages must be structs"),
    })
    .into()
}
