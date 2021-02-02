//! Code generation for Fasters.

use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[proc_macro_derive(TsrMessage, attributes(fasters))]
pub fn derive_tsr_message(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let darling_context = MessageStructure::from_derive_input(&ast).unwrap();
    let context = GeneratorContext {
        crate_path: CratePath::Relative,
        message_structure: darling_context,
    };
    let gen = context.gen_get_field();
    //gen.extend(context.gen_set_field());
    //gen.extend(context.gen_transforms());
    gen.into()
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(fasters))]
struct MessageStructureField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    tag: u32,
    opt: bool,
    rust_type: String,
}

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(fasters), supports(struct_named))]
struct MessageStructure {
    ident: syn::Ident,
    data: darling::ast::Data<darling::util::Ignored, MessageStructureField>,
    msg_type: String,
}

enum CratePath {
    Absolute,
    Relative,
}

struct GeneratorContext {
    crate_path: CratePath,
    message_structure: MessageStructure,
}

impl GeneratorContext {
    fn crate_path(&self) -> TokenStream2 {
        match self.crate_path {
            CratePath::Absolute => quote! { ::fasters },
            CratePath::Relative => quote! { crate },
        }
    }

    fn gen_transforms(&self) -> TokenStream2 {
        let name = &self.message_structure.ident;
        let crate_name = self.crate_path();
        let gen = quote! {
            impl ::std::convert::TryFrom<#crate_name::slr::Message> for #name {
                type Error = ();

                fn try_from(msg: #crate_name::slr::Message) -> Result<Self, Self::Error> {
                    todo!();
                }
            }

            impl ::std::convert::From<#name> for #crate_name::slr::Message {
                fn from(msg: #(self.message_name)) -> Self {
                    let message = #crate_name::slr::Message::new();
                    message.fields.
                }
            }
        };
        gen.into()
    }

    fn gen_set_field(&self) -> TokenStream2 {
        let name = &self.message_structure.ident;
        let crate_name = self.crate_path();
        let gen = quote! {
            impl #name {
                fn set_field(&mut self, tag: u32, value: ::std::option::Option<#crate_name::slr::FixValue>) {
                    match (tag, value) {
                        _ => todo!(),
                    }
                }
            }
        };
        gen.into()
    }

    fn gen_get_field(&self) -> TokenStream2 {
        let crate_name = self.crate_path();
        let match_body = self
            .message_structure
            .data
            .clone()
            .map_struct_fields(|field| {
                let field_name = &field.ident.as_ref().unwrap();
                let field_tag = field.tag;
                if field.opt {
                    quote! {
                        #field_tag => self.#field_name.clone().map(|v| v.into())
                    }
                } else {
                    quote! {
                        #field_tag => ::std::option::Option::Some(self.#field_name.clone().into())
                    }
                }
            })
            .take_struct()
            .unwrap()
            .fields;
        let name = &self.message_structure.ident;
        let gen = quote! {
            impl #name {
                fn get_field(&self, tag: u32) -> ::std::option::Option<#crate_name::app::slr::FixFieldValue> {
                    match tag {
                        #(#match_body),*,
                        _ => None,
                    }
                }
            }
        };
        gen.into()
    }
}

#[derive(Debug, Clone)]
enum Error {
    NotANamedStruct(String),
}
