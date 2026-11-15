use syn::parse::{Parse, ParseStream};

use crate::Direction;
use crate::State;

pub struct PacketAttributes {
    pub id: u8,
    pub state: State,
    pub direction: Direction,
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

        // Ids index a 256-bit interest set, so they must fit in a byte.
        let id_lit = input.parse::<syn::LitInt>()?;
        let Ok(id) = id_lit.base10_parse::<u8>() else {
            return Err(syn::Error::new_spanned(id_lit, "packet ids must be in 0..=255"));
        };

        input.parse::<syn::Token![,]>()?;

        let direction_ident = input.parse::<syn::Ident>()?;
        let direction = match direction_ident.to_string().as_str() {
            "Serverbound" => Direction::Serverbound,
            "Clientbound" => Direction::Clientbound,
            _ => return Err(syn::Error::new_spanned(direction_ident, "expected Serverbound or Clientbound")),
        };

        Ok(PacketAttributes { id, state, direction })
    }
}
