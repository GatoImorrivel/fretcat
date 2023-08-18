use proc_macro2::{Ident, Span};
use quote::{quote, __private::ext::RepToTokensExt};
use syn::{spanned::Spanned, Data, DataStruct, Fields, DataEnum};

pub(crate) fn derive_mapper_impl(
    input: syn::DeriveInput,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    match &input.data {
        Data::Enum(e) => derive_enum(&input, e),
        Data::Struct(s) => Err(syn::Error::new(
            s.struct_token.span(),
            "Getters implementations cannot be derived from structs",
        )),
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
    let enum_name = &input.ident;

    let mut ifs = vec![];
    for variant in e.variants.iter() {
        let fields = &variant.fields;

        if let Fields::Unnamed(field) = fields {
            if fields.len() > 1 {
                return Err(syn::Error::new_spanned(variant, "Must have only one named field"));
            }
            let variant_ident = &variant.ident;
            let ty = &field.unnamed.first().unwrap().ty;

            ifs.push(quote! {
                if value.is::<#ty>() {
                    return Ok(#enum_name::#variant_ident(value.downcast_ref::<#ty>().unwrap().clone()));
                }
            });
        } else {
            return Err(syn::Error::new_spanned(variant, "There are no unnamed fields in the enum"));
        }
    }

    let res = quote! {
        impl TryFrom<Box<dyn AudioEffect>> for #enum_name {
            type Error = MapperError;

            fn try_from(value: Box<dyn AudioEffect>) -> Result<Self, Self::Error> {
                #(#ifs)*

                Err(Self::Error::NotFound)
            }
        }
    };

    Ok(res)
}
