use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{spanned::Spanned, Data, DataStruct, Fields, DataEnum};

pub(crate) fn derive_mapper_impl(
    input: syn::DeriveInput,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    match &input.data {
        Data::Enum(e) => derive_enum(&input, e),
        Data::Struct(s) => Err(syn::Error::new(
            s.struct_token.span(),
            "Getters implementations cannot be derived from structs",
        ))
        Data::Union(u) => Err(syn::Error::new(
            u.union_token.span(),
            "Getters implementations cannot be derived from unions",
        ))
    }
}

fn derive_enum(
    input: &syn::DeriveInput,
    e: &DataEnum,
) -> Result<proc_macro2::TokenStream, syn::Error> {

    let ifs = vec![];
    for variant in e.variants.iter() {
        let fields = variant.fields;

        if let Fields::Named(field) = fields {
            
        } else {
            return Err(syn::Error::new_spanned(variant, "Must have one named field"));
        }
    }

    Ok(())
}
