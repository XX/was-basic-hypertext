use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod derive;

#[proc_macro_derive(Params, attributes(param))]
pub fn derive_params(input: TokenStream) -> TokenStream {
    derive::params(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
