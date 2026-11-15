use proc_macro2::{Span, TokenStream as Tokens};
use quote::quote;
use syn::visit_mut::VisitMut;
use syn::{Fields, Ident, ItemEnum, Lifetime, Type};

/// One packet of the direction: `Name(Type<'p>)`.
struct Variant<'a> {
    name: &'a Ident,
    attrs: &'a [syn::Attribute],
    /// The type at the enum's lifetime, as written.
    ty: &'a Type,
    /// The same type at `'static`, which names its `PacketKind`.
    kind: Type,
}

/// Expands a direction enum: adds the `Raw` variant, `decode` by state and
/// id, `encode`, and a `PacketKind` for every listed packet.
pub fn expand_packet_enum(direction: Ident, item: &ItemEnum) -> syn::Result<Tokens> {
    if direction != "Serverbound" && direction != "Clientbound" {
        return Err(syn::Error::new_spanned(direction, "expected Serverbound or Clientbound"));
    }

    let lifetimes: Vec<_> = item.generics.lifetimes().collect();
    if lifetimes.len() != 1 || item.generics.params.len() != 1 {
        return Err(syn::Error::new_spanned(&item.generics, "packet enums take exactly one lifetime"));
    }

    let lifetime = &lifetimes[0].lifetime;

    let mut variants = Vec::new();

    for variant in &item.variants {
        let Fields::Unnamed(fields) = &variant.fields else {
            return Err(syn::Error::new_spanned(variant, "variants hold exactly one packet"));
        };

        if fields.unnamed.len() != 1 {
            return Err(syn::Error::new_spanned(variant, "variants hold exactly one packet"));
        }

        let ty = &fields.unnamed[0].ty;
        let mut kind = ty.clone();
        StaticLifetime { replaced: lifetime }.visit_type_mut(&mut kind);
        variants.push(Variant { name: &variant.ident, attrs: &variant.attrs, ty, kind });
    }

    let enum_definition = expand_definition(item, lifetime, &variants);
    let methods = expand_methods(item, lifetime, &variants);
    let kinds = variants.iter().map(|variant| expand_kind(item, &direction, lifetime, variant));
    let uniqueness = expand_uniqueness_check(item, &variants);

    Ok(quote! {
        #enum_definition
        #methods
        #(#kinds)*
        #uniqueness
    })
}

/// Fails compilation when two packets share a state and id, which would make
/// one of them unreachable in `decode`.
fn expand_uniqueness_check(item: &ItemEnum, variants: &[Variant]) -> Tokens {
    let count = variants.len();
    let keys = variants.iter().map(|Variant { kind, .. }| {
        let kind = quote!(<#kind as ::mc_protocol::packets::PacketKind>);
        quote!((#kind::STATE as u8, #kind::ID))
    });
    let message = format!("{} lists two packets with the same state and id", item.ident);

    quote! {
        const _: () = {
            let keys: [(u8, u8); #count] = [#(#keys),*];
            let mut first = 0;

            while first < keys.len() {
                let mut second = first + 1;

                while second < keys.len() {
                    let same_state = keys[first].0 == keys[second].0;
                    let same_id = keys[first].1 == keys[second].1;
                    assert!(!(same_state && same_id), #message);
                    second += 1;
                }

                first += 1;
            }
        };
    }
}

fn expand_definition(item: &ItemEnum, lifetime: &Lifetime, variants: &[Variant]) -> Tokens {
    let ItemEnum { attrs, vis, ident, generics, .. } = item;
    let declarations = variants.iter().map(|Variant { name, attrs, ty, .. }| quote!(#(#attrs)* #name(#ty)));

    quote! {
        #(#attrs)*
        #[derive(Debug)]
        #vis enum #ident #generics {
            #(#declarations,)*
            /// A packet with no definition for its state and id, kept as bytes.
            Raw(::mc_protocol::packets::RawPacket<#lifetime>),
        }
    }
}

fn expand_methods(item: &ItemEnum, lifetime: &Lifetime, variants: &[Variant]) -> Tokens {
    let ident = &item.ident;
    let decode_arms = variants.iter().map(|Variant { name, ty, kind, .. }| {
        let state = quote!(<#kind as ::mc_protocol::packets::PacketKind>::STATE);
        let id = quote!(<#kind as ::mc_protocol::packets::PacketKind>::ID);
        quote! {
            (#state, #id) => Self::#name(<#ty as ::mc_protocol::Payload<#lifetime>>::decode_payload(&mut input)?)
        }
    });
    let encode_arms =
        variants.iter().map(|Variant { name, .. }| quote!(Self::#name(packet) => ::mc_protocol::Payload::encode(packet, out)));

    quote! {
        impl<#lifetime> #ident<#lifetime> {
            /// Decodes a frame body (packet id, then payload) received in
            /// `state`. Ids without a definition decode as `Raw`; a known
            /// packet must consume the whole body.
            pub fn decode(state: ::mc_protocol::State, body: &#lifetime [u8]) -> ::mc_protocol::decode::Result<Self> {
                let mut input = body;
                let id = <::mc_protocol::VarInt as ::mc_protocol::PacketWriter>::read(&mut input)?.0;
                let raw = ::mc_protocol::packets::RawPacket { id, payload: ::std::borrow::Cow::Borrowed(input) };
                let Ok(byte_id) = u8::try_from(id) else {
                    return Ok(Self::Raw(raw));
                };

                let packet = match (state, byte_id) {
                    #(#decode_arms,)*
                    _ => return Ok(Self::Raw(raw)),
                };

                if !input.is_empty() {
                    return Err(::mc_protocol::error::DecodeError::TrailingData);
                }

                Ok(packet)
            }

            /// Appends the uncompressed frame: length, packet id, then the payload.
            pub fn encode(&self, out: &mut Vec<u8>) -> ::mc_protocol::encode::Result {
                match self {
                    #(#encode_arms,)*
                    Self::Raw(packet) => packet.encode(out),
                }
            }
        }
    }
}

fn expand_kind(item: &ItemEnum, direction: &Ident, lifetime: &Lifetime, variant: &Variant) -> Tokens {
    let ident = &item.ident;
    let Variant { name, ty, kind, .. } = variant;
    let payload = quote!(<#kind as ::mc_protocol::Payload<'static>>);
    let wrong_direction = format!("{name} is not {direction}");
    let id_too_large = format!("the id of {name} does not fit in a byte");

    quote! {
        const _: () = {
            assert!(matches!(#payload::DIRECTION, ::mc_protocol::Direction::#direction), #wrong_direction);
            assert!(#payload::ID.0 >= 0 && #payload::ID.0 <= 255, #id_too_large);
        };

        impl ::mc_protocol::packets::PacketKind for #kind {
            const DIRECTION: ::mc_protocol::Direction = ::mc_protocol::Direction::#direction;
            const STATE: ::mc_protocol::State = #payload::STATE;
            const ID: u8 = #payload::ID.0 as u8;

            type Packet<#lifetime> = #ty;
            type Enum<#lifetime> = #ident<#lifetime>;

            // Spelled through the associated types, as in the trait, so the
            // lifetimes bind the same way.
            fn pick<'enum_borrow, #lifetime>(
                packet: &'enum_borrow mut Self::Enum<#lifetime>,
            ) -> Option<&'enum_borrow mut Self::Packet<#lifetime>> {
                match packet {
                    #ident::#name(packet) => Some(packet),
                    _ => None,
                }
            }
        }
    }
}

/// Rewrites one lifetime to `'static` throughout a type.
struct StaticLifetime<'a> {
    replaced: &'a Lifetime,
}

impl VisitMut for StaticLifetime<'_> {
    fn visit_lifetime_mut(&mut self, lifetime: &mut Lifetime) {
        if lifetime.ident == self.replaced.ident {
            *lifetime = Lifetime::new("'static", Span::call_site());
        }
    }
}
