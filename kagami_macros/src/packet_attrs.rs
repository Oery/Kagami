use syn::parse::{Parse, ParseStream};

use crate::Origin;

pub struct PacketAttributes {
    pub id: i32,
    pub origin: Origin,
}

impl Parse for PacketAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Ident>()?;
        input.parse::<syn::Token![,]>()?;

        let id_lit = input.parse::<syn::LitInt>()?;
        let id = id_lit.base10_parse::<i32>()?;

        input.parse::<syn::Token![,]>()?;

        let origin_ident = input.parse::<syn::Ident>()?;
        let origin = match origin_ident.to_string().as_str() {
            "Client" => Origin::Client,
            "Server" => Origin::Server,
            _ => panic!("Invalid Origin"),
        };

        Ok(PacketAttributes { id, origin })
    }
}
