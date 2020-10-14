use std::string::{String, ToString};

use proc_macro::TokenStream;
use quote::quote;
use syn;

use crate::first_lower;

// taken in parts from
// https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro

#[proc_macro_derive(Key)]
pub fn key_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_key(&ast)
}

fn impl_key(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = match &ast.data {
        syn::Data::Enum(d) => d,
        _ => panic!("only enums are supported"),
    };
    let mut variants = syn::punctuated::Punctuated::<_, syn::token::Comma>::new();
    for variant in &data.variants {
        variants.push(first_lower(&variant.ident.to_string()));
    }
    let gen = quote! {
        impl Key for #name {
            fn parse(cmdline: &mut str) -> Result<Vec<(&str, &str)>, miniarg::ParseError> {
                miniarg::parse(cmdline, &[#variants])
            }
        }
    };
    gen.into()
}

/// Turn the first character into lowercase.
fn first_lower(input: &str) -> String {
    // taken from https://stackoverflow.com/a/38406885/2192464
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
    }
}
