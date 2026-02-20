use quote::quote;
use syn::{Generics, Ident};

pub enum Origin {
    Client,
    Server,
}

impl Origin {
    pub fn to_string(&self) -> String {
        match self {
            Origin::Client => "client",
            Origin::Server => "server",
        }
        .into()
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
