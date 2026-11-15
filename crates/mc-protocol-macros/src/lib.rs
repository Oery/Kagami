use proc_macro::TokenStream;
use proc_macro2::TokenStream as Tokens;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Generics, Lifetime, Type, parse_macro_input};

mod attrs;
mod format;

use attrs::PacketAttributes;
use format::Format;

enum Origin {
    Client,
    Server,
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
    let PacketAttributes { id, state } = attributes;
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
    let mut decodes = Vec::new();
    let mut writes = Vec::new();
    let mut lengths = Vec::new();
    for field in &fields {
        let ident = &field.ident;
        let ty = &field.ty;
        let (decode, write, length) = match Format::parse(&field.attrs)? {
            Format::Standard => (
                quote!(let #ident = <#ty>::read(buf)?;),
                quote!(self.#ident.write(w).await?;),
                quote!(self.#ident.body_len()),
            ),
            Format::VarInt => {
                // i32 needs no fallible conversion; other integer fields retain
                // their checked conversion from the signed wire value.
                let value = match ty {
                    Type::Path(path) if path.path.segments.last().is_some_and(|segment| segment.ident == "i32") => {
                        quote!(VarInt::read(buf)?.0)
                    }
                    _ => quote!(VarInt::read(buf)?.0.try_into()?),
                };
                (
                    quote!(let #ident: #ty = #value;),
                    quote!(VarInt(self.#ident as i32).write(w).await?;),
                    quote!(VarInt(self.#ident as i32).body_len()),
                )
            }
            Format::Json => (
                quote! {
                    let json: &str = <&str>::read(buf)?;
                    let #ident: #ty = serde_json::from_str(json)?;
                },
                quote! {
                    let json = serde_json::to_string(&self.#ident)?;
                    json.as_str().write(w).await?;
                },
                quote!(serde_json::to_string(&self.#ident)?.as_str().body_len()),
            ),
        };
        decodes.push(decode);
        writes.push(write);
        lengths.push(length);
    }
    let names = fields.iter().map(|field| &field.ident);
    let construct = if matches!(data.fields, Fields::Unit) {
        quote!(Self)
    } else {
        quote!(Self { #(#names),* })
    };
    Ok(quote! {
        #[derive(Payload, Debug)]
        #item

        impl #impl_generics Payload<#lifetime> for #name #ty_generics #where_clause {
            const ID: VarInt = VarInt(#id);
            const STATE: State = State::#state;

            fn payload_len(&self) -> EncodeResult<usize> {
                Ok(0 #(+ #lengths)*)
            }

            fn decode_payload(buf: &mut &#lifetime [u8]) -> DecodeResult<Self> {
                #(#decodes)*
                Ok(#construct)
            }

            async fn write_payload<W: AsyncWrite + Unpin>(&self, w: &mut W) -> EncodeResult {
                #(#writes)*
                Ok(())
            }
        }
    })
}

/// Registers the field helper attribute emitted by `packet`.
#[proc_macro_derive(Payload, attributes(format))]
pub fn derive_payload(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro_derive(InnerPacket)]
pub fn derive_inner_packet(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    expand_inner_packet(&item).unwrap_or_else(syn::Error::into_compile_error).into()
}

fn expand_inner_packet(item: &DeriveInput) -> syn::Result<Tokens> {
    let Data::Enum(data) = &item.data else {
        return Err(syn::Error::new_spanned(item, "InnerPacket requires an enum"));
    };
    let ident = &item.ident;
    let (implementation, lifetime) = codec_generics(&item.generics)?;
    let (impl_generics, _, where_clause) = implementation.split_for_impl();
    let (_, ty_generics, _) = item.generics.split_for_impl();
    let mut id_cases = Vec::new();
    let mut state_cases = Vec::new();
    for variant in &data.variants {
        let Fields::Unnamed(fields) = &variant.fields else {
            return Err(syn::Error::new_spanned(variant, "variants require exactly one packet field"));
        };
        if fields.unnamed.len() != 1 {
            return Err(syn::Error::new_spanned(variant, "variants require exactly one packet field"));
        }
        let name = &variant.ident;
        let ty = &fields.unnamed[0].ty;
        id_cases.push(quote!(Self::#name(_) => <#ty as Payload<#lifetime>>::ID.0));
        state_cases.push(quote!(Self::#name(_) => <#ty as Payload<#lifetime>>::STATE));
    }
    Ok(quote! {
        impl #impl_generics InnerPacket for #ident #ty_generics #where_clause {
            fn id(&self) -> i32 {
                match self { #(#id_cases),* }
            }

            fn state(&self) -> State {
                match self { #(#state_cases),* }
            }
        }
    })
}

#[cfg(test)]
#[path = "tests/tests_packet.rs"]
mod tests;
