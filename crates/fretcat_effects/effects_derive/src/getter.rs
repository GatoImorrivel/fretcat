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
        quote! {
            fn #fn_name(effect: Effect) -> Then<Wrapper<crate::chain::chain_data_derived_lenses::chain>, nih_plug_vizia::vizia::binding::Map<impl Fn(&std::sync::Arc<atomic_refcell::AtomicRefCell<crate::Chain>>) -> f32, std::sync::Arc<atomic_refcell::AtomicRefCell<crate::Chain>>, #ty>> {
                ChainData::chain.map(move |chain| chain.borrow().query_cast::<#struct_name>(&effect).unwrap().#name)
            }
        }
    });

    let res = quote!(
        #(#fn_definitions), *,
    );

    Ok(res)
}
