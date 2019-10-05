extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ReadymadeBuilder)]
pub fn readymade_builder(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl ReadymadeBuilder for #name {
            fn builder() -> T {
            }
        }
    };

    expanded.into()
}
