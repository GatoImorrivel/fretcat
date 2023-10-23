use quote::quote;
use syn::{spanned::Spanned, Data, Fields, DataEnum};

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

    let mut from_ifs = vec![];
    let mut into_ifs = vec![];
    for variant in e.variants.iter() {
        let fields = &variant.fields;

        if let Fields::Unnamed(field) = fields {
            if fields.len() > 1 {
                return Err(syn::Error::new_spanned(variant, "Must have only one named field"));
            }
            let variant_ident = &variant.ident;
            let ty = &field.unnamed.first().unwrap().ty;

            from_ifs.push(quote! {
                if value.is::<#ty>() {
                    return Ok(#enum_name::#variant_ident(value.downcast_ref::<#ty>().unwrap().clone()));
                }
            });

            into_ifs.push(quote! {
                if let #enum_name::#variant_ident(val) = self {
                    return Ok(Arc::new(val));
                }
            });
        } else {
            return Err(syn::Error::new_spanned(variant, "There are no unnamed fields in the enum"));
        }
    }

    let res = quote! {
        impl TryFrom<Arc<dyn AudioEffect>> for #enum_name {
            type Error = MapperError;

            fn try_from(value: Arc<dyn AudioEffect>) -> Result<Self, Self::Error> {
                #(#from_ifs)*

                Err(Self::Error::NotFound)
            }
        }

        impl TryInto<Arc<dyn AudioEffect>> for Mapper {
            type Error = MapperError;

            fn try_into(self) -> Result<Arc<dyn AudioEffect>, Self::Error> {
                #(#into_ifs)*

                Err(Self::Error::NotFound)
            }
        }
    };

    Ok(res)
}
