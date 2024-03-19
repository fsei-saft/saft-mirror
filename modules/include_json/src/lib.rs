#![feature(proc_macro_span)]

extern crate proc_macro;

mod processor;
mod cache;

use crate::processor::process_value;

use proc_macro2::{Ident, TokenStream};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use quote::{format_ident, quote};
use serde_json::{Map, Value};
use syn::{parse2, parse_macro_input, Data, DeriveInput, Fields, LitStr, Token, parse};
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
    let input = parse::<Input>(input).unwrap();
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
    File::open(path).unwrap().read_to_string(&mut buf).unwrap();

    parse_json_impl(ty, buf)
}

#[proc_macro]
pub fn parse_json(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse::<Input>(input).unwrap();
    parse_json_impl(input.ty, input.st.value()).into()
}

fn parse_json_impl(ty: Ident, data: String) -> TokenStream {
    let json: Value = serde_json::from_str(data.as_str()).unwrap();

    process_value(json, ty)
}


#[proc_macro_derive(IncludeJson)]
pub fn include_json_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    include_json_derive_impl(input).into()
}

fn include_json_derive_impl(input: DeriveInput) -> TokenStream {
    let _ident = input.ident;

    if let Data::Struct(input) = input.data {
        if let Fields::Named(input) = input.fields {
            let mut fields: Vec<TokenStream> = Vec::new();

            for field in input.named.into_iter() {
                let name = format_ident!("IncludeJsonType_{}", field.ident.unwrap());
                let ty = field.ty;
                fields.push(quote!{type #name = #ty;});
            }

            return quote!{
                #(#fields)*
            }.into();
        }
    }
    
    panic!("IncludeJson only supports named structs");
}
