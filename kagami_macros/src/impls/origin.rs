use quote::quote;
use syn::{Generics, Ident};

pub enum Origin {
    Client,
    Server,
}

impl std::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let origin = match self {
            Origin::Client => "client",
            Origin::Server => "server",
        };
        write!(f, "{origin}")
    }
}

pub fn get_origin_impl(name: &Ident, origin: &Origin, generics: &Generics) -> proc_macro2::TokenStream {
    let item_type = match generics.lifetimes().next().is_some() {
        true => quote! { #name<'a> },
        false => quote! { #name },
    };

    match origin {
        Origin::Client => quote! { impl<'a> ClientPacket<'a> for #item_type {} },
        Origin::Server => quote! { impl<'a> ServerPacket<'a> for #item_type {} },
    }
}
