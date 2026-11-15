use proc_macro2::{Span, TokenStream as Tokens};
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, Ident, Lifetime};

/// Implements `IntoOwned` by converting every field. A type with a lifetime
/// becomes the same type at `'static`; a type without one is returned as is.
pub fn expand_into_owned(item: &DeriveInput) -> syn::Result<Tokens> {
    let generics = &item.generics;
    let has_type_parameters = generics.params.iter().any(|param| !matches!(param, syn::GenericParam::Lifetime(_)));

    if has_type_parameters || generics.lifetimes().count() > 1 {
        return Err(syn::Error::new_spanned(
            generics,
            "IntoOwned supports at most one lifetime and no type or const parameters",
        ));
    }

    let name = &item.ident;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    if generics.lifetimes().next().is_none() {
        return Ok(quote! {
            impl ::mc_protocol::IntoOwned for #name #where_clause {
                type Owned = Self;

                fn into_owned(self) -> Self {
                    self
                }
            }
        });
    }

    let conversion = match &item.data {
        Data::Struct(data) => {
            let (pattern, construct) = convert_fields(quote!(#name), &data.fields);
            quote!(let #pattern = self; #construct)
        }
        Data::Enum(data) => {
            let mut arms = Vec::new();

            for variant in &data.variants {
                let variant_name = &variant.ident;
                let (pattern, construct) = convert_fields(quote!(#name::#variant_name), &variant.fields);
                arms.push(quote!(#pattern => #construct));
            }

            quote!(match self { #(#arms),* })
        }
        Data::Union(_) => return Err(syn::Error::new_spanned(item, "IntoOwned does not support unions")),
    };

    let owned_lifetime = Lifetime::new("'static", Span::call_site());

    Ok(quote! {
        impl #impl_generics ::mc_protocol::IntoOwned for #name #ty_generics #where_clause {
            type Owned = #name<#owned_lifetime>;

            fn into_owned(self) -> #name<#owned_lifetime> {
                #conversion
            }
        }
    })
}

/// A pattern binding every field of `path`, and the same shape rebuilt from
/// the owned fields.
fn convert_fields(path: Tokens, fields: &Fields) -> (Tokens, Tokens) {
    match fields {
        Fields::Named(fields) => {
            let names: Vec<&Ident> = fields.named.iter().filter_map(|field| field.ident.as_ref()).collect();
            let pattern = quote!(#path { #(#names),* });
            let construct = quote!(#path { #(#names: ::mc_protocol::IntoOwned::into_owned(#names)),* });
            (pattern, construct)
        }
        Fields::Unnamed(fields) => {
            let names: Vec<Ident> = (0..fields.unnamed.len()).map(|index| format_ident!("field_{index}")).collect();
            let pattern = quote!(#path(#(#names),*));
            let construct = quote!(#path(#(::mc_protocol::IntoOwned::into_owned(#names)),*));
            (pattern, construct)
        }
        Fields::Unit => (path.clone(), path),
    }
}
