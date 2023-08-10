#[allow(dead_code, unused_imports)]
mod message;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Path, Ident, parse::{Parse, ParseStream}, Token};

#[proc_macro_derive(Message, attributes(msg))]
pub fn derive_message(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    message::derive_message_impl(input).unwrap_or_else(|err| err.to_compile_error()).into()
}

struct GetterInput {
    ident: Ident,
    ty_path: Path,
}

impl Parse for GetterInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let ty_path: Path = input.parse()?;
        Ok(GetterInput { ident, ty_path })
    }
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