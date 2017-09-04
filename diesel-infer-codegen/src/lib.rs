extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{Tokens, ToTokens};
use syn::{MetaItem, Lit};

mod utils;

#[proc_macro_derive(InferDBFields, attributes(table_name))]
pub fn derive_infer_fields(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    let ast = syn::parse_derive_input(&source).unwrap();

    let expanded = expand_fields_and_table_name(&ast);

    expanded.parse().unwrap()
}

fn expand_fields_and_table_name(ast: &syn::DeriveInput) -> quote::Tokens {
    let data_fields = match ast.body {
        syn::Body::Struct(ref data) => data.fields(),
        syn::Body::Enum(_) => panic!("#[derive(Fields)] can only be used with structs"),
    };

    let mut fields = Vec::new();
    let mut tys = Vec::new();
    for field in data_fields {
        fields.push(field.ident.clone().unwrap().to_string());
        tys.push(field.ty.clone());
    }
    let mut tokens = Tokens::new();
    tys.to_tokens(&mut tokens);

    // Used in the quasi-quotation below as `#name`
    let name = &ast.ident;
    let table_name = match ast.attrs.clone().into_iter().find(|attr| attr.name() == "table_name").unwrap().value {
        MetaItem::NameValue(_, Lit::Str(ref value, _)) => &*value,
        _ => panic!(r#"`table` must be in the form `#[table="something"]`"#),
    }.clone();

    let mut lifetimes = Tokens::new();
    ast.generics.to_tokens(&mut lifetimes);

    let values = ::utils::get_values(&fields, &tys);
    let fields = ::utils::get_fields_vec(&fields);

    quote! {
        // The generated impl
        impl#lifetimes ::diesel_infer::InferDBFields for #name#lifetimes {
            fn get_fields() -> Vec<String> {
                #fields
            }

            fn get_table_name() -> &'static str {
                #table_name
            }

            fn get_values(&self) -> Vec<String> {
                #values
            }
        }
    }
}
