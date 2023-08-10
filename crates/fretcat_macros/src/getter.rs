use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{spanned::Spanned, Data, DataStruct, Fields};

pub(crate) fn derive_getters_impl(
    input: syn::DeriveInput,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    match &input.data {
        Data::Struct(s) => derive_struct(&input, s),
        Data::Enum(e) => Err(syn::Error::new(
            e.enum_token.span(),
            "Getters implementations cannot be derived from enums",
        )),
        Data::Union(u) => Err(syn::Error::new(
            u.union_token.span(),
            "Getters implementations cannot be derived from unions",
        )),
    }
}

fn derive_struct(
    input: &syn::DeriveInput,
    s: &DataStruct,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let struct_name = &input.ident;

    let get_fields = s.fields.iter().filter(|field| {
        field.attrs.iter().any(|attr| {
            attr.path()
                .segments
                .iter()
                .any(|segment| segment.ident == "get")
        })
    });

    let fn_definitions = get_fields.clone().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        let fn_name = Ident::new(format!("get_{}", name.to_string()).as_str(), field.span());
        let ret_ident = Ident::new(format!("{}_getter", name.to_string()).as_str(), field.span());
        quote! {
            type #ret_ident = Then<Wrapper<crate::chain::chain_data_derived_lenses::chain>, nih_plug_vizia::vizia::binding::Map<impl Fn(&std::sync::Arc<atomic_refcell::AtomicRefCell<crate::Chain>>) -> #ty, std::sync::Arc<atomic_refcell::AtomicRefCell<crate::Chain>>, #ty>>;
            fn #fn_name(effect: Effect) ->  Then<Wrapper<crate::chain::chain_data_derived_lenses::chain>, nih_plug_vizia::vizia::binding::Map<impl Fn(&std::sync::Arc<atomic_refcell::AtomicRefCell<crate::Chain>>) -> #ty, std::sync::Arc<atomic_refcell::AtomicRefCell<crate::Chain>>, #ty>> {
                ChainData::chain.map(move |chain| chain.borrow().query_cast::<#struct_name>(&effect).unwrap().#name)
            }
        }
    });

    let message_definitions = get_fields.clone().map(|field| {
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

        #(#fn_definitions)*
    );

    Ok(res)
}
