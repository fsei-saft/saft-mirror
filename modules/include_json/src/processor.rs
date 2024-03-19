use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use serde_json::{Map, Value};
use syn::{parse2, parse_macro_input, Data, DeriveInput, Fields, LitStr, Token};
use syn::parse::{Parse, ParseStream};

pub fn process_value(json: Value, ty: Ident) -> TokenStream {
    match json {
        Value::Null => { quote!{None} }
        Value::Array(_v) => { quote!{vec![]} }
        Value::Bool(v) => { quote!{#v} }
        Value::Number(v) => { 
            if v.is_i64() {
                let v = v.as_i64().unwrap();
                quote!{#v}
            } else if v.is_u64() {
                let v = v.as_u64().unwrap();
                quote!{#v}
            } else if v.is_f64() {
                let v = v.as_f64().unwrap();
                quote!{#v}
            } else {
                panic!("invalid number");
            }
        }
        Value::String(v) => { quote!{#v.to_string()} }
        Value::Object(v) => {
            process_struct(v, ty)
        }
    }
}

pub fn process_struct(json: Map<String, Value>, ty: Ident) -> TokenStream {
    let mut fields = Vec::new();

    for (k, v) in json {
        let ident = format_ident!("{}", k);
        let content = process_value(v, ty.clone());
        fields.push(quote!{#ident: #content,});
    }

    let fields = fields.into_iter().fold(quote!{}, |sum, e| { quote!{#sum #e} });
    quote!{#ty {#fields}}
}
