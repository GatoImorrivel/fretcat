use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Control, attributes(control))]
pub fn control_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the struct name and fields
    let name = &input.ident;
    let control_name = syn::Ident::new(&(name.to_string() + "Control"), name.span());
    let fields = match input.data {
        Data::Struct(ref data_struct) => {
            if let Fields::Named(ref fields) = data_struct.fields {
                fields
            } else {
                panic!("The Control derive macro only supports named fields.");
            }
        }
        _ => panic!("The Control derive macro only supports structs."),
    };

    // Filter fields with the #[control] attribute
    let control_fields = fields.named.iter().filter(|field| {
        field.attrs.iter().any(|attr| {
            attr.path().segments.iter().any(|segment| segment.ident == "control")
        })
    });

    // Generate the field definitions for the new struct
    let control_field_definitions = control_fields.map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        quote! {
            pub(crate) #field_name: #field_type
        }
    });

    // Generate the final output tokens
    let expanded = quote! {
        pub struct #control_name {
            #(#control_field_definitions),*
        }
    };

    // Return the generated code as a TokenStream
    TokenStream::from(expanded)
}
