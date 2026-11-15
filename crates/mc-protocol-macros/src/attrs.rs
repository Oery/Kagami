use syn::parse::{Parse, ParseStream};

use crate::Origin;
use crate::State;

pub struct PacketAttributes {
    pub id: i32,
    pub state: State,
}

impl Parse for PacketAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let state_ident = input.parse::<syn::Ident>()?;
        let state = match state_ident.to_string().as_str() {
            "Handshake" => State::Handshake,
            "Status" => State::Status,
            "Login" => State::Login,
            "Config" => State::Config,
            "Play" => State::Play,
            _ => return Err(syn::Error::new_spanned(state_ident, "unknown packet state")),
        };

        input.parse::<syn::Token![,]>()?;

        let id_lit = input.parse::<syn::LitInt>()?;
        let id = id_lit.base10_parse::<i32>()?;

        input.parse::<syn::Token![,]>()?;

        let origin_ident = input.parse::<syn::Ident>()?;
        let _origin = match origin_ident.to_string().as_str() {
            "Client" => Origin::Client,
            "Server" => Origin::Server,
            _ => return Err(syn::Error::new_spanned(origin_ident, "expected Client or Server")),
        };

        Ok(PacketAttributes { id, state })
    }
}
