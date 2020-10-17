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
    for variant in &data.variants {
        let mut path = syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>::new();
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
    }
    let gen = quote! {
        impl fmt::Display for #name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Debug::fmt(self, f)
            }
        }
        
        impl Key for #name {
            fn parse(cmdline: &mut str) -> ArgumentIterator<Self> {
                miniarg::parse(cmdline, &[#variants])
            }
        }
    };
    gen.into()
}
