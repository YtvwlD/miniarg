//! custom derives for [miniarg]
//!
//! [miniarg]: https://github.com/YtvwlD/miniarg
#![doc(html_root_url = "https://docs.rs/miniarg_derive/0.3.0")]

use proc_macro::TokenStream;
use quote::quote;
use syn;

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
    let mut help_strings = Vec::new();
    for variant in &data.variants {
        let mut path = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::PathSep>::new();
        path.push(syn::PathSegment {
            ident: syn::token::SelfType{span: proc_macro2::Span::call_site()}.into(),
            arguments: syn::PathArguments::None,
        });
        path.push(syn::PathSegment {
            ident: variant.ident.clone(),
            arguments: syn::PathArguments::None,
        });
        variants.push(syn::Path {
            leading_colon: None,
            segments: path,
        });
        let mut doc = "".to_string();
        for attr in &variant.attrs {
            if let syn::Meta::NameValue(mnv) = &attr.meta {
                if mnv.path.is_ident("doc") {
                    if let syn::Expr::Lit(l) = &mnv.value {
                        if let syn::Lit::Str(s) = &l.lit {
                            doc = s.value();
                            break;
                        } else {
                            panic!("failed to parse {l:?}");
                        }
                    } else {
                        panic!("failed to parse {mnv:?}");
                    }
                }
            }
        };
        help_strings.push(format!("-{}\t{}", first_lower(&variant.ident.to_string()), doc));
    }
    let help_text = help_strings.join("\n");
    let gen = quote! {
        impl fmt::Display for #name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Debug::fmt(self, f)
            }
        }
        
        impl Key for #name {
            fn parse(cmdline: &str) -> ArgumentIterator<Self, miniarg::split_args::SplitArgs> {
                miniarg::parse(cmdline, &[#variants])
            }
            
            fn help_text() -> &'static str {
                #help_text
            }
        }
    };
    gen.into()
}

/// Turn the first character into lowercase.
// This has to be duplicated because of proc_macro.
fn first_lower(input: &str) -> String {
    // taken from https://stackoverflow.com/a/38406885/2192464
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
    }
}
