use darling::{FromDeriveInput, FromVariant};
use proc_macro::TokenStream;
use proc_macro2::{Literal, TokenTree};
use quote::quote;

pub fn derive_field_value(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let darling_context = DataFieldWithVariants::from_derive_input(&ast).unwrap();
    let identifier = darling_context.ident;
    let matching_cases = darling_context
        .data
        .clone()
        .map_enum_variants(|enum_variant| {
            let enum_discriminant = enum_variant.variant.as_str();
            let enum_discriminant_len = enum_variant.variant.as_bytes().len();
            let enum_variant = enum_variant.ident;
            quote! {
                Self::#enum_variant => {
                    buffer.extend_from_slice(#enum_discriminant.as_bytes());
                    #enum_discriminant_len
                },
            }
        })
        .take_enum()
        .expect("Invalid enum");
    let deserialize_matching_cases = darling_context
        .data
        .clone()
        .map_enum_variants(|enum_variant| {
            let enum_discriminant = enum_variant.variant.as_str();
            let enum_variant = enum_variant.ident;
            let bstring: proc_macro2::TokenStream =
                TokenTree::from(Literal::byte_string(enum_discriminant.as_bytes()))
                    .into();
            quote! {
                #bstring => Ok(Self::#enum_variant)
            }
        })
        .take_enum()
        .expect("Invalid enum");
    let gen = quote! {
        impl<'a> FixFieldValue<'a> for #identifier {
            type Error = ();
            type SerializeSettings = ();

            const IS_ASCII: bool = true;

            fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
            where
                B: Buffer,
            {
                match self {
                    #(#matching_cases)*
                }
            }

            fn deserialize(data: &'a [u8]) -> Result<Self, <Self as FixFieldValue<'a>>::Error> {
                match data {
                    #(#deserialize_matching_cases),*,
                    _ => Err(())
                }
            }
        }
    };
    gen.into()
}

#[derive(Debug, Clone, FromVariant)]
#[darling(attributes(fefix))]
struct EnumVariantInfo {
    ident: syn::Ident,
    variant: String,
}

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(fefix))]
struct DataFieldWithVariants {
    ident: syn::Ident,
    data: darling::ast::Data<EnumVariantInfo, darling::util::Ignored>,
}
