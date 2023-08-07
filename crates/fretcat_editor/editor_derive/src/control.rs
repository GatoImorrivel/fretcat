use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{spanned::Spanned, Data, DataStruct, Fields};

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
    let control_name = input.ident.clone();

    let message_arms_definitions = s.fields.iter().map(|elem| {
        let ident = elem.ident.as_ref().unwrap();
        let s1 = ident.to_string();
        let mut v: Vec<char> = s1.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        let s2: String = v.into_iter().collect();

        let arm_ident = Ident::new(&s2, ident.span());
        let ty = &elem.ty;

        quote! {
            #arm_ident(#ty)
        }
    });

    let update_arms_definitions = s.fields.iter().map(|elem| {
        let ident = elem.ident.as_ref().unwrap();
        let s1 = ident.to_string();
        let mut v: Vec<char> = s1.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        let s2: String = v.into_iter().collect();

        let arm_ident = Ident::new(&s2, ident.span());

        quote! {
            Message::#arm_ident(val) => {
                nih_log!("{:#?}", event);
                self.#ident *= val;
            }
        }
    });

    let control_name_string = control_name.clone().to_string().to_lowercase();

    let res = quote!(
        #[derive(Debug)]
        pub enum Message {
            #(#message_arms_definitions), *,
        }
    );

    Ok(res)
}
