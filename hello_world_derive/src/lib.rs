extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use syn::Ident;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloWorld, attributes(HelloWorldName))]
pub fn hello_world(input: TokenStream) -> TokenStream {

    let input: proc_macro2::TokenStream = input.into();
    // Construct a string representation of the type definition
    let s = input.to_string();
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_hello_world(&ast);

    // Return the generated impl
    let output: proc_macro2::TokenStream = gen.parse().unwrap();
    output.into()
}

fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
    let struct_name = &ast.ident;
    let mut name: Option<Ident> = None;
    for attr in ast.attrs.iter().as_ref() {
        use syn::{MetaItem, Lit};
        match attr.value {
            MetaItem::NameValue(ref ident, Lit::Str(ref lit, _)) if ident == "HelloWorldName" => {
                name = Some(lit.as_str().into());
                break;
            }
            _ => (),
        }
    }
    let name = &name.as_ref();
    let name = name.unwrap_or(struct_name);

    if let syn::Body::Struct(_) = ast.body {
        quote! {
            impl HelloWorld for #struct_name {
                fn hello_world() {
                    println!("Hello, World! My name is {}", stringify!(#name));
                }
            }
        }
    } else {
        panic!("#[derive(HelloWorld)] is only defined for structs, not for enums!");
    }
}