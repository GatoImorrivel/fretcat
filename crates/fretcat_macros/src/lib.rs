#[allow(dead_code, unused_imports)]
mod message;
mod effect_mapper;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, Type};

#[proc_macro_derive(Message, attributes(msg))]
pub fn derive_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    message::derive_message_impl(input).unwrap_or_else(|err| err.to_compile_error()).into()
}

#[proc_macro_derive(EffectMapper)]
pub fn derive_mapper(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    effect_mapper::derive_mapper_impl(input).unwrap_or_else(|err| err.to_compile_error()).into()
}

#[proc_macro]
pub fn getter(input: TokenStream) -> TokenStream {
    // Parse the input identifier
    let ident = parse_macro_input!(input as Ident);

    // Generate the quoted code
    let expanded = quote! {
        ChainData::chain.map(move |chain| match chain.borrow().query_cast::<Self>(&effect) {
            Some(data) => data.#ident,
            None => 0.0
        })
    };

    expanded.into()
}

#[proc_macro]
pub fn mapper_match(input: TokenStream) -> TokenStream {
    // Parse the input identifier
    let ty = parse_macro_input!(input as Type);

    // Generate the quoted code
    let expanded = quote! {
        if value.is::<#ty>() {
            return Mapper::#ty(value.downcast_ref::<#ty>().unwrap().clone());
        }
    };

    expanded.into()
}