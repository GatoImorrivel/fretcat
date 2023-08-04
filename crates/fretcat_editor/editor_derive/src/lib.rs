#[allow(dead_code, unused_imports)]

mod control;

use proc_macro::TokenStream;
use syn::parse_macro_input;


#[proc_macro_derive(Control)]
pub fn derive_control(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    control::derive_control_impl(input).unwrap_or_else(|err| err.to_compile_error()).into()
}