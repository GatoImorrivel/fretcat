use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Data, spanned::Spanned, DataStruct, Fields};

pub(crate) fn derive_control_impl(
    input: syn::DeriveInput,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    match &input.data {
        Data::Struct(s) => derive_struct(&input, s),
        Data::Enum(e) => Err(syn::Error::new(
            e.enum_token.span(),
            "Control implementations cannot be derived from enums",
        )),
        Data::Union(u) => Err(syn::Error::new(
            u.union_token.span(),
            "Control implementations cannot be derived from unions",
        )),
    }
}

fn derive_struct(
    input: &syn::DeriveInput,
    s: &DataStruct,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let ident = &input.ident; 
    let struct_name = format!("{}Control", ident.to_string());
    let new_ident = Ident::new(&struct_name, Span::call_site());

    let res = quote!(
        pub struct #new_ident {

        }

        impl #new_ident {
            fn print() {
                println!("{}", #struct_name);
            }
        }
    );

    Ok(res)
}