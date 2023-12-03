extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Name)]
pub fn derive(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	let name = &ast.ident;
	let gen = quote! {
        impl #name {
            pub fn name() -> String {
                stringify!(#name).trim().to_lowercase()
            }
        }
    };
	gen.into()
}
