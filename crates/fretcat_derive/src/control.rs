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
    let ident = &input.ident;
    let struct_name = format!("{}Control", ident.to_string());
    let new_ident = Ident::new(&struct_name, Span::call_site());

    let control_fields = s.fields.iter().filter(|field| {
        field.attrs.iter().any(|attr| {
            attr.path()
                .segments
                .iter()
                .any(|segment| segment.ident == "control")
        })
    });

    let control_fields_definitions = control_fields.map(|elem| {
        let ident = elem.ident.as_ref().unwrap();
        let ty = &elem.ty;

        quote! {
            pub #ident: #ty
        }
    });

    let res = quote!(
        #[derive(Lens)]
        pub struct #new_ident {
            #(#control_fields_definitions), *,
            pub handle: EffectHandle
        }

        impl #new_ident {
            pub fn downcast_mut_handle(&mut self) -> &mut #ident {
                self.handle.as_mut_any().downcast_mut::<#ident>().unwrap()
            }

            pub fn downcast_handle(&self) -> &#ident {
                self.handle.as_any().downcast_ref::<#ident>().unwrap()
            }
        }
    );

    Ok(res)
}