use proc_macro::TokenStream;
use proc_macro2::TokenStream as Tokens;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Generics, Lifetime, Type, parse_macro_input};

mod attrs;
mod format;
mod owned;
mod packet_enum;

use attrs::PacketAttributes;
use format::Format;
use owned::expand_into_owned;
use packet_enum::expand_packet_enum;

enum Direction {
    Serverbound,
    Clientbound,
}

enum State {
    Handshake,
    Status,
    Login,
    Config,
    Play,
}

/// Owned packets still implement Payload for any input lifetime. Borrowed
/// packets use their declared lifetime, without assuming it is named `'p`.
fn codec_generics(generics: &Generics) -> syn::Result<(Generics, Lifetime)> {
    if generics.params.iter().any(|param| !matches!(param, syn::GenericParam::Lifetime(_))) || generics.lifetimes().count() > 1 {
        return Err(syn::Error::new_spanned(
            generics,
            "packets support at most one lifetime and no type or const parameters",
        ));
    }
    let mut implementation = generics.clone();
    let lifetime = match generics.lifetimes().next() {
        Some(param) => param.lifetime.clone(),
        None => {
            implementation.params.push(syn::parse_quote!('p));
            syn::parse_quote!('p)
        }
    };
    Ok((implementation, lifetime))
}

#[proc_macro_attribute]
pub fn packet(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attributes = parse_macro_input!(attr as PacketAttributes);
    let item = parse_macro_input!(input as DeriveInput);
    expand_packet(attributes, &item).unwrap_or_else(syn::Error::into_compile_error).into()
}

fn expand_packet(attributes: PacketAttributes, item: &DeriveInput) -> syn::Result<Tokens> {
    let PacketAttributes { id, state, direction } = attributes;
    let Data::Struct(data) = &item.data else {
        return Err(syn::Error::new_spanned(item, "packets must be structs"));
    };
    let fields: Vec<_> = match &data.fields {
        Fields::Named(fields) => fields.named.iter().collect(),
        Fields::Unit => Vec::new(),
        Fields::Unnamed(_) => {
            return Err(syn::Error::new_spanned(&data.fields, "packets require named fields"));
        }
    };
    let name = &item.ident;
    let (implementation, lifetime) = codec_generics(&item.generics)?;
    let (impl_generics, _, where_clause) = implementation.split_for_impl();
    let (_, ty_generics, _) = item.generics.split_for_impl();
    let state = match state {
        State::Handshake => quote!(Handshake),
        State::Status => quote!(Status),
        State::Login => quote!(Login),
        State::Config => quote!(Config),
        State::Play => quote!(Play),
    };
    let direction = match direction {
        Direction::Serverbound => quote!(Serverbound),
        Direction::Clientbound => quote!(Clientbound),
    };
    let id = i32::from(id);

    let mut decodes = Vec::new();
    let mut encodes = Vec::new();
    let mut lengths = Vec::new();
    for field in &fields {
        let ident = &field.ident;
        let ty = &field.ty;
        let (decode, encode, length) = match Format::parse(&field.attrs)? {
            Format::Standard => (
                quote!(let #ident = <#ty as ::mc_protocol::PacketWriter>::read(buf)?;),
                quote!(::mc_protocol::PacketWriter::encode(&self.#ident, out)?;),
                quote!(::mc_protocol::PacketWriter::body_len(&self.#ident)),
            ),
            Format::VarInt => {
                // i32 needs no fallible conversion; other integer fields retain
                // their checked conversion from the signed wire value.
                let read = quote!(<::mc_protocol::VarInt as ::mc_protocol::PacketWriter>::read(buf)?.0);
                let value = match ty {
                    Type::Path(path) if path.path.segments.last().is_some_and(|segment| segment.ident == "i32") => read,
                    _ => quote!(#read.try_into()?),
                };
                let varint = quote!(::mc_protocol::VarInt(self.#ident as i32));
                (
                    quote!(let #ident: #ty = #value;),
                    quote!(::mc_protocol::PacketWriter::encode(&#varint, out)?;),
                    quote!(::mc_protocol::PacketWriter::body_len(&#varint)),
                )
            }
        };
        decodes.push(decode);
        encodes.push(encode);
        lengths.push(length);
    }

    let names = fields.iter().map(|field| &field.ident);
    let construct = if matches!(data.fields, Fields::Unit) {
        quote!(Self)
    } else {
        quote!(Self { #(#names),* })
    };
    let item = without_format_attributes(item);
    let into_owned = expand_into_owned(&item)?;

    Ok(quote! {
        #[derive(Debug)]
        #item

        impl #impl_generics ::mc_protocol::Payload<#lifetime> for #name #ty_generics #where_clause {
            const DIRECTION: ::mc_protocol::Direction = ::mc_protocol::Direction::#direction;
            const ID: ::mc_protocol::VarInt = ::mc_protocol::VarInt(#id);
            const STATE: ::mc_protocol::State = ::mc_protocol::State::#state;

            fn payload_len(&self) -> ::mc_protocol::encode::Result<usize> {
                Ok(0 #(+ #lengths)*)
            }

            fn decode_payload(buf: &mut &#lifetime [u8]) -> ::mc_protocol::decode::Result<Self> {
                #(#decodes)*
                Ok(#construct)
            }

            fn encode_payload(&self, out: &mut Vec<u8>) -> ::mc_protocol::encode::Result {
                #(#encodes)*
                Ok(())
            }
        }

        #into_owned
    })
}

/// `#[format]` only steers the generated codec; the struct itself must not keep it.
fn without_format_attributes(item: &DeriveInput) -> DeriveInput {
    let mut item = item.clone();
    let Data::Struct(data) = &mut item.data else {
        return item;
    };
    for field in data.fields.iter_mut() {
        field.attrs.retain(|attr| !attr.path().is_ident("format"));
    }
    item
}

#[proc_macro_derive(IntoOwned)]
pub fn derive_into_owned(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    expand_into_owned(&item).unwrap_or_else(syn::Error::into_compile_error).into()
}

/// Lists a direction's packets: `#[packet_enum(Serverbound)] pub enum Serverbound<'p> { ChatCommand(ChatCommand<'p>), .. }`.
///
/// Appends a `Raw` variant for packets without a definition, and generates
/// `decode(state, body)`, `encode(out)` and a `PacketKind` for each packet.
#[proc_macro_attribute]
pub fn packet_enum(attr: TokenStream, input: TokenStream) -> TokenStream {
    let direction = parse_macro_input!(attr as syn::Ident);
    let item = parse_macro_input!(input as syn::ItemEnum);
    expand_packet_enum(direction, &item).unwrap_or_else(syn::Error::into_compile_error).into()
}

#[cfg(test)]
#[path = "../tests/tests_packet.rs"]
mod tests;
