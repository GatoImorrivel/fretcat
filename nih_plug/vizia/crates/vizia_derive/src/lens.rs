// Adapted from Druid lens.rs

// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// use proc_macro2::{Ident, Span};
use quote::quote;
// use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::{Data, GenericParam, Ident, Token, TypeParam, VisRestricted, Visibility};

use super::attr::{FieldKind, Fields, LensAttrs};

pub(crate) fn derive_lens_impl(
    input: syn::DeriveInput,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    match &input.data {
        Data::Struct(_) => derive_struct(&input),
        Data::Enum(_) => derive_enum(&input),
        Data::Union(u) => Err(syn::Error::new(
            u.union_token.span(),
            "Lens implementations cannot be derived from unions",
        )),
    }
}

fn derive_struct(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    let struct_type = &input.ident;

    // The generated module should have the same visibility as the struct. If the struct is private
    // then the generated structs within the new module should be visible only to the module the
    // original struct was in.
    let module_vis = &input.vis;
    let struct_vis = increase_visibility(module_vis);

    let fields = if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &input.data {
        Fields::<LensAttrs>::parse_ast(fields)?
    } else {
        return Err(syn::Error::new(
            input.span(),
            "Lens implementations can only be derived from structs with named fields",
        ));
    };

    if fields.kind != FieldKind::Named {
        return Err(syn::Error::new(
            input.span(),
            "Lens implementations can only be derived from structs with named fields",
        ));
    }

    let twizzled_name = if is_camel_case(&struct_type.to_string()) {
        let temp_name = format!("{}_derived_lenses", to_snake_case(&struct_type.to_string()));
        proc_macro2::Ident::new(&temp_name, proc_macro2::Span::call_site())
    } else {
        return Err(syn::Error::new(
            struct_type.span(),
            "Lens implementations can only be derived from CamelCase types",
        ));
    };
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut lens_ty_idents = Vec::new();
    let mut phantom_decls = Vec::new();
    let mut phantom_inits = Vec::new();
    let mut lens_ty_decls = Vec::new();

    for gp in input.generics.params.iter() {
        if let GenericParam::Type(TypeParam { ident, .. }) = gp {
            lens_ty_idents.push(quote! {#ident});
            lens_ty_decls.push(quote! {#ident: 'static});
            phantom_decls.push(quote! {std::marker::PhantomData<*const #ident>});
            phantom_inits.push(quote! {std::marker::PhantomData});
        }
    }

    let lens_ty_generics = quote! {
        <#(#lens_ty_idents),*>
    };

    let lens_ty_generics_decls = quote! {
        <#(#lens_ty_decls),*>
    };

    // Define lens types for each field
    let defs = fields.iter().filter(|f| !f.attrs.ignore).map(|f| {
        let field_name = &f.ident.unwrap_named();
        let struct_docs = format!(
            "Lens for the field `{field}` on [`{ty}`](super::{ty}).",
            field = field_name,
            ty = struct_type,
        );

        let fn_docs = format!(
            "Creates a new lens for the field `{field}` on [`{ty}`](super::{ty}). \
            Use [`{ty}::{field}`](super::{ty}::{field}) instead.",
            field = field_name,
            ty = struct_type,
        );

        quote! {
            #[doc = #struct_docs]
            #[allow(non_camel_case_types)]
            #[derive(PartialEq, Eq)]
            #struct_vis struct #field_name#lens_ty_generics(#(#phantom_decls),*);

            impl #lens_ty_generics #field_name#lens_ty_generics{
                #[doc = #fn_docs]
                pub const fn new()->Self{
                    Self(#(#phantom_inits),*)
                }
            }

            impl #lens_ty_generics_decls std::hash::Hash for #field_name#lens_ty_generics {
                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    std::any::TypeId::of::<Self>().hash(state);
                }
            }

            impl #lens_ty_generics std::fmt::Debug for #field_name#lens_ty_generics {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f,"{}:{}",stringify!(#struct_type), stringify!(#field_name))
                }
            }

            impl #lens_ty_generics Clone for #field_name#lens_ty_generics  {
                fn clone(&self) -> #field_name#lens_ty_generics {
                    *self
                }
            }

            impl #lens_ty_generics Copy for #field_name#lens_ty_generics {}


        }
    });

    // let used_params: HashSet<String> = input
    //     .generics
    //     .params
    //     .iter()
    //     .flat_map(|gp: &GenericParam| match gp {
    //         GenericParam::Type(TypeParam { ident, .. }) => Some(ident.to_string()),
    //         _ => None,
    //     })
    //     .collect();

    // let gen_new_param = |name: &str| {
    //     let mut candidate: String = name.into();
    //     let mut count = 1usize;
    //     while used_params.contains(&candidate) {
    //         candidate = format!("{}_{}", name, count);
    //         count += 1;
    //     }
    //     Ident::new(&candidate, Span::call_site())
    // };

    //let func_ty_par = gen_new_param("F");
    //let val_ty_par = gen_new_param("V");

    let impls = fields.iter().filter(|f| !f.attrs.ignore).map(|f| {
        let field_name = &f.ident.unwrap_named();
        let field_ty = &f.ty;
        quote! {

            impl #impl_generics Lens for #twizzled_name::#field_name#lens_ty_generics #where_clause {

                type Source = #struct_type#ty_generics;
                type Target = #field_ty;

                fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &#struct_type#ty_generics, map: F) -> O {
                    map(Some(&source.#field_name))
                }
            }
        }
    });

    let associated_items = fields.iter().filter(|f| !f.attrs.ignore).map(|f| {
        let field_name = &f.ident.unwrap_named();
        let lens_field_name = f.attrs.lens_name_override.as_ref().unwrap_or(field_name);
        let field_vis = &f.vis;

        quote! {
            /// Lens for the corresponding field.
            #field_vis const #lens_field_name: Wrapper<#twizzled_name::#field_name#lens_ty_generics> = Wrapper(#twizzled_name::#field_name::new());
        }
    });

    let mod_docs = format!("Derived lenses for [`{}`].", struct_type);
    let root_docs = format!("Lens for the whole [`{ty}`](super::{ty}) struct.", ty = struct_type);
    //let lens_docs = format!("# Lenses for [`{ty}`](super::{ty})", ty = struct_type);

    let expanded = quote! {
        #[doc = #mod_docs]
        #module_vis mod #twizzled_name {
            #(#defs)*
            #[derive(PartialEq, Eq)]
            #[doc = #root_docs]
            #[allow(non_camel_case_types)]
            #struct_vis struct root#lens_ty_generics(#(#phantom_decls),*);

            impl #lens_ty_generics root#lens_ty_generics{
                pub const fn new()->Self{
                    Self(#(#phantom_inits),*)
                }
            }

            impl #lens_ty_generics_decls std::hash::Hash for root#lens_ty_generics {
                fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    std::any::TypeId::of::<Self>().hash(state);
                }
            }

            impl #lens_ty_generics std::fmt::Debug for root#lens_ty_generics {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f,"{}",stringify!(#struct_type))
                }
            }

            impl #lens_ty_generics Clone for root#lens_ty_generics  {
                fn clone(&self) -> root#lens_ty_generics {
                    *self
                }
            }

            impl #lens_ty_generics Copy for root#lens_ty_generics {}
        }

        #(#impls)*

        impl #impl_generics Lens for #twizzled_name::root#lens_ty_generics {
            type Source = #struct_type#ty_generics;
            type Target = #struct_type#ty_generics;

            fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &Self::Source, map: F) -> O {
                map(Some(source))
            }
        }

        #[allow(non_upper_case_globals)]
        #[doc(hidden)]
        impl #impl_generics #struct_type #ty_generics #where_clause {
            #(#associated_items)*

            pub const root: Wrapper<#twizzled_name::root#lens_ty_generics> = Wrapper(#twizzled_name::root::new());
        }
    };

    Ok(expanded)
}

//I stole these from rustc!
pub(crate) fn char_has_case(c: char) -> bool {
    c.is_lowercase() || c.is_uppercase()
}

fn is_camel_case(name: &str) -> bool {
    let name = name.trim_matches('_');
    if name.is_empty() {
        return true;
    }

    // start with a non-lowercase letter rather than non-uppercase
    // ones (some scripts don't have a concept of upper/lowercase)
    !name.chars().next().unwrap().is_lowercase()
        && !name.contains("__")
        && !name.chars().collect::<Vec<_>>().windows(2).any(|pair| {
            // contains a capitalisable character followed by, or preceded by, an underscore
            char_has_case(pair[0]) && pair[1] == '_' || char_has_case(pair[1]) && pair[0] == '_'
        })
}

fn to_snake_case(mut str: &str) -> String {
    let mut words = vec![];
    // Preserve leading underscores
    str = str.trim_start_matches(|c: char| {
        if c == '_' {
            words.push(String::new());
            true
        } else {
            false
        }
    });
    for s in str.split('_') {
        let mut last_upper = false;
        let mut buf = String::new();
        if s.is_empty() {
            continue;
        }
        for ch in s.chars() {
            if !buf.is_empty() && buf != "'" && ch.is_uppercase() && !last_upper {
                words.push(buf);
                buf = String::new();
            }
            last_upper = ch.is_uppercase();
            buf.extend(ch.to_lowercase());
        }
        words.push(buf);
    }
    words.join("_")
}

fn derive_enum(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    let enum_type = &input.ident;

    // See `derive_struct`
    let module_vis = &input.vis;
    let struct_vis = increase_visibility(module_vis);

    let variants = if let syn::Data::Enum(syn::DataEnum { variants, .. }) = &input.data {
        variants
    } else {
        panic!("I don't know what this case being triggered means. Please open an issue!")
    };

    let usable_variants = variants
        .iter()
        .filter_map(|v| match &v.fields {
            syn::Fields::Unnamed(f) => {
                if f.unnamed.len() == 1 {
                    Some((&v.ident, &f.unnamed.first().unwrap().ty))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    if usable_variants.is_empty() {
        panic!("This enum has no variants which can have Lenses built. A valid variant has exactly one unnamed field. If you think this is unreasonable, please work on https://github.com/rust-lang/rfcs/pull/2593")
    }

    let twizzled_name = if is_camel_case(&enum_type.to_string()) {
        let temp_name = format!("{}_derived_lenses", to_snake_case(&enum_type.to_string()));
        proc_macro2::Ident::new(&temp_name, proc_macro2::Span::call_site())
    } else {
        return Err(syn::Error::new(
            enum_type.span(),
            "Lens implementations can only be derived from CamelCase types",
        ));
    };

    if !input.generics.params.is_empty() {
        panic!("Lens implementations can only be derived from non-generic enums (for now)");
    }

    let defs = usable_variants.iter().map(|(variant_name, _)| {
        quote! {
            #[allow(non_camel_case_types)]
            #[derive(Copy, Clone, Hash, PartialEq, Eq)]
            #struct_vis struct #variant_name();

            impl #variant_name {
                pub const fn new() -> Self {
                    Self()
                }
            }

            impl std::fmt::Debug for #variant_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f,"{}:{}",stringify!(#enum_type), stringify!(#variant_name))
                }
            }
        }
    });

    let impls = usable_variants.iter().map(|(variant_name, variant_type)| {
        let name = format!("{}:{}", enum_type, variant_name);
        quote! {
            impl Lens for #twizzled_name::#variant_name {
                type Source = #enum_type;
                type Target = #variant_type;

                fn view<O, F: FnOnce(Option<&Self::Target>) -> O>(&self, source: &#enum_type, map: F) -> O {
                    map(if let #enum_type::#variant_name(inner_value) = source {
                        Some(inner_value)
                    } else {
                        None
                    })
                }
            }

            impl std::fmt::Debug for #twizzled_name::#variant_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(#name)
                }
            }
        }
    });

    let associated_items = usable_variants.iter().map(|(variant_name, _)| {
        let variant_const_name = to_snake_case(&variant_name.to_string());
        let variant_const_name = proc_macro2::Ident::new(&variant_const_name, proc_macro2::Span::call_site());
        quote! {
            pub const #variant_const_name: #twizzled_name::#variant_name = #twizzled_name::#variant_name::new();
        }
    });

    let expanded = quote! {
        #module_vis mod #twizzled_name {
            #(#defs)*
        }

        #(#impls)*

        #[allow(non_upper_case_globals)]
        impl #enum_type {
            #(#associated_items)*
        }
    };

    Ok(expanded)
}

/// Increase privite/inherited visiblity to `pub(super)`, `pub(super)` or anything else relative to
/// `super` to one module higher than that, and leave everything else as is.
pub(crate) fn increase_visibility(vis: &Visibility) -> Visibility {
    match vis {
        // Private structs are promoted to `pub(super)`
        Visibility::Inherited => Visibility::Restricted(VisRestricted {
            pub_token: Token![pub](vis.span()),
            paren_token: syn::token::Paren(vis.span()),
            in_token: None,
            path: Box::new(syn::Path::from(Token![super](vis.span()))),
        }),
        // `pub(super(::...))` should be promoted to `pub(super::super(:...))`. Checking for this
        // looks a bit funky.
        Visibility::Restricted(vis @ VisRestricted { path, .. })
            if path.segments.first().map(|segment| &segment.ident)
                == Some(&Ident::from(Token![super](vis.span()))) =>
        {
            let mut new_path = syn::Path::from(Token![super](vis.span()));
            for segment in &path.segments {
                new_path.segments.push(segment.clone());
            }

            Visibility::Restricted(VisRestricted {
                path: Box::new(new_path),
                // Anything other than `crate` or `super` always needs to be prefixed with `in`
                in_token: Some(Token![in](vis.span())),
                ..*vis
            })
        }
        // Everything else stays the same
        vis => vis.clone(),
    }
}
