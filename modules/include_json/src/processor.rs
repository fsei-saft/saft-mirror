use crate::cache::{StructDescription, get_struct};

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use serde_json::{Map, Value};
use syn::{parse_str, GenericArgument, PathArguments, Type};

pub fn process_value(json: Value, ty: String) -> TokenStream {
    match json {
        Value::Null => { quote!{None} }
        Value::Array(v) => {
            let ty = parse_str::<Type>(&ty).expect(&format!("\"{}\" is not a type", ty));

            if let Type::Path(ty) = &ty {
                let ty = ty.path.segments.first().expect(&format!("array type \"{}\" is not compatible", ty.into_token_stream().to_string()));
                if ty.ident.to_string() == "Vec" {
                    if let PathArguments::AngleBracketed(args) = &ty.arguments {
                        if let GenericArgument::Type(arg) = args.args.first().expect(&format!("array type \"{}\" is not compatible", ty.into_token_stream().to_string())) {
                            let ty = arg.into_token_stream().to_string();
                            let elements = v.into_iter().map(|e| { process_value(e, ty.clone()) });
                            return quote!{vec![#(#elements,)*]};
                        }
                    }
                }
            }

            panic!("array type \"{}\" is not compatible", ty.into_token_stream().to_string());
        }
        Value::Bool(v) => { quote!{#v} }
        Value::Number(v) => { 
            let ty = format_ident!("{}", ty);
            if v.is_i64() {
                let v = v.as_i64().unwrap();
                quote!{#v as #ty}
            } else if v.is_u64() {
                let v = v.as_u64().unwrap();
                quote!{#v as #ty}
            } else if v.is_f64() {
                let v = v.as_f64().unwrap();
                quote!{#v as #ty}
            } else {
                panic!("invalid number");
            }
        }
        Value::String(v) => { quote!{#v.to_string()} }
        Value::Object(v) => {
            process_struct(v, get_struct(&ty).expect(&format!("struct \"{}\" not cached", &ty)))
        }
    }
}

pub fn process_struct(json: Map<String, Value>, ty: StructDescription) -> TokenStream {
    let mut fields = Vec::new();

    for (k, v) in json {
        let ident = format_ident!("{}", k);
        let content = process_value(v, ty.fields.iter().find(|f| f.name == k).expect("field not cached").ty.clone());
        fields.push(quote!{#ident: #content,});
    }

    let fields = fields.into_iter().fold(quote!{}, |sum, e| { quote!{#sum #e} });
    let ident = format_ident!("{}", ty.name);
    quote!{#ident {#fields}}
}
