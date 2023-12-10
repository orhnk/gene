use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Name)]
pub fn derive_name(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	let name = ast.ident;
	let lovercase_name = name.to_string().to_lowercase();

	let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

	let gen = quote! {
		impl #impl_generics #name #ty_generics #where_clause {
			#[inline(always)]
            pub fn name() -> &'static str {
				#lovercase_name
            }
        }
    };

	gen.into()
}
/*

#[derive(Name)] // This derive macro
struct Foo<'a> {
	foo: &'a str,
}

// Expands to this code:

impl Foo<'_> {
	pub fn name() -> String {
		"foo"
	}
}

// This also works for non-lifetime variables:
#[derive(Name)]
struct Foo;

// Expands to this code:
impl Foo {
	pub fn name() -> String {
		"foo"
	}
}
 */
