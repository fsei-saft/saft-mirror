#![feature(proc_macro_span)]

extern crate proc_macro;

mod processor;
mod cache;

use crate::processor::process_value;
use crate::cache::{register_struct, StructDescription, StructFieldDescription};

use proc_macro2::{Ident, TokenStream};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use quote::{quote, ToTokens};
use serde_json::Value;
use syn::{parse_macro_input, Data, DeriveInput, Fields, LitStr, Token, parse};
use syn::parse::{Parse, ParseStream};

struct Input {
    ty: Ident,
    st: LitStr
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty = input.parse()?;
        input.parse::<Token![,]>()?;
        let st = input.parse()?;
        Ok(Self {
            ty,
            st
        })
    }
}

#[proc_macro]
pub fn include_json(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let span = input.clone().into_iter().next().unwrap().span();
    let input = parse::<Input>(input).expect("macro expects a type and a path");
    include_json_impl(input.ty, input.st.value(), span.source_file().path()).into()
}

fn include_json_impl(ty: Ident, path: String, mut source_path: PathBuf) -> TokenStream {
    let path: PathBuf = path.into();
    let path = if path.is_absolute() {
        path
    } else {
        source_path.pop();
        source_path.push(path);
        source_path
    };

    let mut buf = String::new();
    File::open(path).expect("file not found").read_to_string(&mut buf).expect("file not readable");

    parse_json_impl(ty, buf)
}

#[proc_macro]
pub fn parse_json(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse::<Input>(input).expect("json not parseable");
    parse_json_impl(input.ty, input.st.value()).into()
}

fn parse_json_impl(ty: Ident, data: String) -> TokenStream {
    let json: Value = serde_json::from_str(data.as_str()).expect("json not parseable");

    process_value(json, ty.to_string())
}

#[proc_macro_derive(IncludeJson)]
pub fn include_json_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    include_json_derive_impl(input).into()
}

fn include_json_derive_impl(input: DeriveInput) -> TokenStream {
    let ident = input.ident;

    if let Data::Struct(input) = input.data {
        if let Fields::Named(input) = input.fields {
            let fields: Vec<_> = input.named.into_iter().map(|f| {
                StructFieldDescription {
                    name: f.ident.expect("only named fields supported").to_string(),
                    ty: f.ty.into_token_stream().to_string()
                }
            }).collect();
            register_struct(StructDescription {
                name: ident.to_string(),
                fields
            });
            return quote!{};
        }
    }
    
    panic!("only named structs supported");
}
