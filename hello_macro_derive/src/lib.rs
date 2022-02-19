extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}


#[proc_macro_derive(ByeMacro)]
pub fn bye_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_bye_macro(&ast)
}

fn impl_bye_macro(ast: &syn::DeriveInput) -> TokenStream {
    // println!("CONTENT OF AST {}", ast);

    let name = &ast.ident;
    let gen = quote!{
        impl ByeMacro for #name {
            fn bye_macro() {
                println!("See you soo, it's only goodbye for now {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}