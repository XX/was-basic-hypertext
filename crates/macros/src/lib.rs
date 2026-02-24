use proc_macro::TokenStream;
use syn::{DeriveInput, ItemStruct, parse_macro_input};

mod attribute;
mod derive;

#[proc_macro_derive(Params, attributes(param))]
pub fn derive_params(input: TokenStream) -> TokenStream {
    derive::params(parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn const_str(args: TokenStream, input: TokenStream) -> TokenStream {
    attribute::const_str(
        parse_macro_input!(args as attribute::Args),
        parse_macro_input!(input as ItemStruct),
    )
    .unwrap_or_else(|err| err.to_compile_error())
    .into()
}
