use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput};

pub fn props(input: DeriveInput) -> syn::Result<TokenStream> {
    let Data::Struct(data_struct) = &input.data else {
        return Err(syn::Error::new(
            input.span(),
            "#[derive(Props)] may only be used on structs",
        ));
    };

    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut setters = Vec::new();
    let mut impl_froms = Vec::new();

    for field in &data_struct.fields {
        if let Some(name) = field.ident.as_ref()
            && let Some(prop_attr) = field.attrs.iter().find(|attr| attr.path().is_ident("prop"))
        {
            let ty = &field.ty;
            let mut is_setters_exist = false;
            let mut is_from_exist = false;

            prop_attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("setters") {
                    is_setters_exist = true;
                    return Ok(());
                }

                if meta.path.is_ident("from") {
                    is_from_exist = true;
                    return Ok(());
                }

                Err(meta.error("unrecognized prop"))
            })?;

            if is_setters_exist {
                let setter_name = format_ident!("set_{}", name);
                setters.push(quote! {
                    #[must_use]
                    pub fn #name(mut self, #name: #ty) -> Self {
                        self.#name = #name;
                        self
                    }

                    pub fn #setter_name(&mut self, #name: #ty) -> &mut Self {
                        self.#name = #name;
                        self
                    }
                });
            }

            if is_from_exist {
                let body = if is_setters_exist {
                    quote! {
                        Self::default().#name(#name)
                    }
                } else {
                    quote! {
                        let mut this = Self::default();
                        this.#name = #name;
                        this
                    }
                };

                impl_froms.push(quote! {
                    #[automatically_derived]
                    impl #impl_generics From<#ty> for #struct_name #ty_generics #where_clause {
                        fn from(#name: #ty) -> Self {
                            #body
                        }
                    }
                });
            }
        }
    }

    let output = if !setters.is_empty() {
        quote! {
            #[automatically_derived]
            impl #impl_generics #struct_name #ty_generics #where_clause {
                #(#setters)*
            }

            #(#impl_froms)*
        }
    } else {
        quote! {
            #(#impl_froms)*
        }
    };

    Ok(output)
}
