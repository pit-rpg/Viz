
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

// use quote::*;
use proc_macro::TokenStream;
// use proc_macro::*;
use syn::DeriveInput;


#[proc_macro_derive(HelloWorld)]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    // let s = input.to_string();

    // Parse the string representation
    // let ast = syn::parse_derive_input(&s).unwrap();
    let input: DeriveInput = syn::parse(input).unwrap();

    // Build the impl
    let gen = impl_hello_world(&input);

    // Return the generated impl
    // gen.
    TokenStream::from(gen)
    // gen.parse().unwrap()
}

fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl HelloWorld for #name {
            fn hello_world() {
                println!("Hello, World! My name is {}", stringify!(#name));
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
