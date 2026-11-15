//! `#[derive(CommandValue)]`: a fieldless enum read from one word, each
//! variant under its name and aliases.

use std::collections::HashSet;

use proc_macro2::TokenStream as Tokens;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr};

use crate::attrs;

pub fn expand(input: &DeriveInput) -> syn::Result<Tokens> {
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new_spanned(input, "CommandValue is derived for enums"));
    };
    if !input.generics.params.is_empty() || data.variants.is_empty() {
        return Err(syn::Error::new_spanned(input, "CommandValue needs variants and no generics"));
    }

    let enum_attrs = attrs::parse(&input.attrs, "value", &["into"])?;

    let mut choices = Vec::new();
    let mut arms = Vec::new();
    let mut seen = HashSet::new();

    for variant in &data.variants {
        if !matches!(variant.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(variant, "CommandValue variants have no fields"));
        }

        let attrs = attrs::parse(&variant.attrs, "value", &["name", "alias"])?;
        let mut words = vec![attrs::name(&attrs, &variant.ident)?];
        for alias in &attrs.aliases {
            attrs::check_word(alias)?;
            words.push(alias.clone());
        }

        for word in &words {
            attrs::unique(word, &mut seen)?;
        }

        let ident = &variant.ident;
        arms.push(quote!(#(#words)|* => ::core::result::Result::Ok(Self::#ident)));
        choices.extend(words);
    }

    let ident = &input.ident;
    let listed: Vec<String> = choices.iter().map(LitStr::value).collect();
    let message = format!("Expected one of: {}.", listed.join(", "));

    // Matching Rust variant names, not the words players type.
    let conversion = enum_attrs.into.map(|target| {
        let variants = data.variants.iter().map(|variant| &variant.ident);

        quote! {
            impl ::core::convert::From<#ident> for #target {
                fn from(value: #ident) -> Self {
                    match value {
                        #(#ident::#variants => Self::#variants),*
                    }
                }
            }
        }
    });

    Ok(quote! {
        impl<'__command> ::kagami::command::CommandArgument<'__command> for #ident {
            const KIND: ::kagami::command::ArgumentKind = ::kagami::command::ArgumentKind::Choices(&[#(#choices),*]);

            fn parse(word: &'__command str) -> ::core::result::Result<Self, &'static str> {
                match word {
                    #(#arms,)*
                    _ => ::core::result::Result::Err(#message),
                }
            }
        }

        #conversion
    })
}
