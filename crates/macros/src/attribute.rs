use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, Ident, ItemStruct, Token};

struct ConstStr {
    ident: Ident,
    value: Expr,
}

impl Parse for ConstStr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let value = input.parse()?;

        Ok(Self { ident, value })
    }
}

impl ToTokens for ConstStr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let value = &self.value;

        tokens.extend(quote! {
            pub const #ident: &str = #value;
        });
    }
}

struct ConstStrFn<'a>(&'a ConstStr);

impl ToTokens for ConstStrFn<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.0.ident.to_string().to_lowercase());
        let value = &self.0.value;

        tokens.extend(quote! {
            pub const fn #name() -> &'static str {
                #value
            }
        });
    }
}

pub struct Args {
    consts: Vec<ConstStr>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let consts = Punctuated::<ConstStr, Token![,]>::parse_separated_nonempty(input)?;

        Ok(Self {
            consts: consts.into_iter().collect(),
        })
    }
}

impl ToTokens for Args {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let consts = &self.consts;

        tokens.extend(quote! {
            #(#consts)*
        });
    }
}

pub fn const_str(args: Args, input: ItemStruct) -> syn::Result<TokenStream> {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut impl_consts = None;
    let mut impl_const_fns = None;

    if !args.consts.is_empty() {
        impl_consts = Some(quote! {
            #[automatically_derived]
            impl #impl_generics #struct_name #ty_generics #where_clause {
                #args
            }
        });

        if input.generics.type_params().all(|param| param.default.is_some()) {
            let params = input.generics.type_params().map(|param| {
                if let Some(default) = &param.default {
                    quote!(#default)
                } else {
                    quote!(#param)
                }
            });
            let const_fns = args.consts.iter().map(ConstStrFn);
            impl_const_fns = Some(quote! {
                #[automatically_derived]
                impl #struct_name <#(#params,)*> {
                    #(#const_fns)*
                }
            });
        }
    }

    Ok(quote! {
        #input
        #impl_consts
        #impl_const_fns
    })
}
