//! Fasters is a standard-compliant FIX & FAST (FIX Adapted for STreaming)
//! implementation in pure Rust.
//!
//! FIX and FAST functionality is kept isolated in the
//! [`fasters::fix`](fasters::fix) and [`fasters::fast`](fasters::fast) modules,
//! respectively.

extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};

#[proc_macro_derive(FixMessage)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    let output = impl_fix(&ast);
    proc_macro::TokenStream::from(output)
}

fn impl_fix(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let des = generate_des(&ast.data);
    let gen = quote! {
        impl ::std::convert::TryFrom<::fasters::slr::Message> for #name {
            type Error = ();

            fn try_from(msg: ::fasters::slr::Message) -> Result<Self, Self::Error> {
                todo!();
            }
        }

        impl ::std::convert::From<#name> for ::fasters::slr::Message {
            fn from(msg: #name) -> Self {
                let message = ::fasters::slr::Message::new();
                message.fields.
            }
        }
    };
    gen
}

fn generate_des(data: &syn::Data) -> TokenStream {
    (match *data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => {
                let quoted = fields.named.iter().map(|f| "");
                quote! {
                    unimplemented!()
                }
            }
            syn::Fields::Unit | syn::Fields::Unnamed(_) => panic!("struct must be named"),
        },
        syn::Data::Enum(_) | syn::Data::Union(_) => panic!("Fix messages must be structs"),
    })
    .into()
}
