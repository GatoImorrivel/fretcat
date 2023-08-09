#[allow(dead_code, unused_imports)]

mod getter;

use proc_macro::TokenStream;
use syn::parse_macro_input;


#[proc_macro_derive(Getters, attributes(get))]
pub fn derive_control(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    getter::derive_getters_impl(input).unwrap_or_else(|err| err.to_compile_error()).into()
}