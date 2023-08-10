use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{spanned::Spanned, Data, DataStruct, Fields};

pub(crate) fn derive_message_impl(
    input: syn::DeriveInput,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    match &input.data {
        Data::Struct(s) => derive_struct(&input, s),
        Data::Enum(e) => Err(syn::Error::new(
            e.enum_token.span(),
            "Message implementations cannot be derived from enums",
        )),
        Data::Union(u) => Err(syn::Error::new(
            u.union_token.span(),
            "Message implementations cannot be derived from unions",
        )),
    }
}

fn derive_struct(
    input: &syn::DeriveInput,
    s: &DataStruct,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let msg_fields = s.fields.iter().filter(|field| {
        field.attrs.iter().any(|attr| {
            attr.path()
                .segments
                .iter()
                .any(|segment| segment.ident == "msg")
        })
    });

    let message_definitions = msg_fields.clone().map(|field| {
        let s1 = field.ident.as_ref().unwrap().to_string();
        let mut v: Vec<char> = s1.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        let s2: String = v.into_iter().collect();
        let msg = Ident::new(&s2, field.span());
        let ty = &field.ty;
        quote! {
            #msg(#ty)
        }
    });

    let res = quote!(
        enum Message {
            #(#message_definitions), *,
        }
    );

    Ok(res)
}
