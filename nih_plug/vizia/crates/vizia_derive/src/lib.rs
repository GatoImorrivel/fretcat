#![allow(dead_code)]

// Adapted from Druid

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

mod attr;
mod data;
mod lens;
mod model;
mod ray;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(Data, attributes(data))]
pub fn derive_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    data::derive_data_impl(input).unwrap_or_else(|err| err.to_compile_error()).into()
}

#[proc_macro_derive(Lens, attributes(lens))]
pub fn derive_lens(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    lens::derive_lens_impl(input).unwrap_or_else(|err| err.to_compile_error()).into()
}

#[proc_macro_derive(Setter, attributes(setter))]
pub fn derive_ray(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    ray::derive_ray_impl(input).unwrap_or_else(|err| err.to_compile_error()).into()
}

#[proc_macro_derive(Model, attributes(model))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    model::derive_model_impl(input).unwrap_or_else(|err| err.to_compile_error()).into()
}
