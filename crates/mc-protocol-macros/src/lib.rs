use proc_macro::TokenStream;
use proc_macro2::TokenStream as Tokens;
use quote::quote;
use syn::{Data, DeriveInput, Generics, Lifetime, parse_macro_input};

mod attrs;
mod fields;
mod format;
mod owned;
mod packet_enum;

use attrs::PacketAttributes;
use fields::{FieldCodecs, field_codecs};
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

/// Owned types still implement their codec for any input lifetime. Borrowed
/// types use their declared lifetime, without assuming it is named `'p`.
fn codec_generics(generics: &Generics, what: &str) -> syn::Result<(Generics, Lifetime)> {
    if generics.params.iter().any(|param| !matches!(param, syn::GenericParam::Lifetime(_))) || generics.lifetimes().count() > 1 {
        return Err(syn::Error::new_spanned(
            generics,
            format!("{what} support at most one lifetime and no type or const parameters"),
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
    let FieldCodecs { decodes, encodes, lengths, construct } = field_codecs(data, "packets")?;

    let name = &item.ident;
    let (implementation, lifetime) = codec_generics(&item.generics, "packets")?;
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

/// Implements `PacketWriter` for a struct by coding its named fields in
/// declaration order. Fields accept `#[format = "varint"]` and
/// `#[format = "varlong"]`, as in `#[packet]`.
#[proc_macro_derive(PacketWriter, attributes(format))]
pub fn derive_packet_writer(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    expand_packet_writer(&item).unwrap_or_else(syn::Error::into_compile_error).into()
}

fn expand_packet_writer(item: &DeriveInput) -> syn::Result<Tokens> {
    let Data::Struct(data) = &item.data else {
        return Err(syn::Error::new_spanned(item, "PacketWriter can only be derived for structs"));
    };
    let FieldCodecs { decodes, encodes, lengths, construct } = field_codecs(data, "field codecs")?;

    let name = &item.ident;
    let (implementation, lifetime) = codec_generics(&item.generics, "field codecs")?;
    let (impl_generics, _, where_clause) = implementation.split_for_impl();
    let (_, ty_generics, _) = item.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics ::mc_protocol::PacketWriter<#lifetime> for #name #ty_generics #where_clause {
            fn read(buf: &mut &#lifetime [u8]) -> ::mc_protocol::decode::Result<Self> {
                #(#decodes)*
                Ok(#construct)
            }

            fn encode(&self, out: &mut Vec<u8>) -> ::mc_protocol::encode::Result {
                #(#encodes)*
                Ok(())
            }

            fn body_len(&self) -> usize {
                0 #(+ #lengths)*
            }
        }
    })
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
