use proc_macro2::{Ident, TokenStream};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use quote::{format_ident, quote};
use serde_json::{Map, Value};
use syn::{parse2, parse_macro_input, Data, DeriveInput, Fields, LitStr, Token};
use syn::parse::{Parse, ParseStream};
