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
    let queried_init = Ident::new("data", Span::call_site());

    let control_fields = s.fields.iter().filter(|field| {
        field.attrs.iter().any(|attr| {
            attr.path()
                .segments
                .iter()
                .any(|segment| segment.ident == "control")
        })
    });

    let control_fields_definitions = control_fields.clone().map(|elem| {
        let ident = elem.ident.as_ref().unwrap();
        let ty = &elem.ty;

        quote! {
            pub #ident: #ty
        }
    });

    let control_fields_inits = control_fields.clone().map(|elem| {
        let ident = elem.ident.as_ref().unwrap();
        let source = queried_init.clone();

        quote! {
            #ident: #source.#ident
        }
    });

    let message_definitions = control_fields.clone().map(|elem| {
        let name = elem.ident.as_ref().unwrap().to_string();
        let mut v: Vec<char> = name.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        let s2: String = v.into_iter().collect();
        let ident = Ident::new(&s2, Span::call_site());
        let ty = &elem.ty;

        quote! {
            #ident(#ty)
        }
    });

    let arms_definitions = control_fields.clone().map(|elem| {
        let original_field_ident = elem.ident.as_ref().unwrap();
        let name = elem.ident.as_ref().unwrap().to_string();
        let mut v: Vec<char> = name.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        let s2: String = v.into_iter().collect();
        let arm_ident = Ident::new(&s2, Span::call_site());

        quote! {
            Message::#arm_ident(val) => {
                self.#original_field_ident = *val;
                let mut chain = ChainHandle::root.get(_cx);

                let e = chain.query_cast_mut::<#ident>(&self.handle).unwrap();
                e.#original_field_ident = *val;
            }
        }
    });

    let new_ident_string = new_ident.clone().to_string();

    let res = quote!(
        #[derive(Lens)]
        pub struct #new_ident {
            #(#control_fields_definitions), *,
            pub handle: Effect
        }

        fn create_control<F: FnOnce(&mut Context)>(cx: &mut Context, handle: &Effect, content: F) {
            let chain = ChainHandle::root.get(cx);
            let #queried_init = chain.query_cast::<#ident>(handle).unwrap();

            #new_ident {
                #(#control_fields_inits), *,
                handle: handle.clone()
            }.build(cx, content);
        }

        enum Message {
            #(#message_definitions), *,
        }

        impl View for #new_ident {
            fn element(&self) -> Option<&'static str> {
                Some(#new_ident_string)
            }

            fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
                event.map(|event, _| match event {
                    #(#arms_definitions), *,
                });
            }
        }
    );

    Ok(res)
}
