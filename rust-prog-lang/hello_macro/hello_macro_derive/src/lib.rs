extern crate pro_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream{
    //将rust代码转换为我们能够进行处理的语法树
    let ast = syn::parse(input).unwrap();

    //构建对于的trait实现
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello,Macro! My name is {}",stringify!(#name));
            }
        }
    };
    gen.into();
}