extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn telegram_api_method(token_stream: TokenStream) -> TokenStream {
    (quote! {}).into()
}
