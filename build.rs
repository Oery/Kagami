use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::{ItemStruct, parse_file};

use std::fs;
use std::io::Write;
use std::path::Path;

enum Origin {
    Client,
    Server,
}

impl ToTokens for Origin {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::Client => quote! { Source::Client },
            Self::Server => quote! { Source::Server },
        });
    }
}

enum State {
    Handshake = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}

impl ToTokens for State {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            State::Handshake => quote! { McState::Handshake },
            State::Status => quote! { McState::Status },
            State::Login => quote! { McState::Login },
            State::Play => quote! { McState::Play },
        });
    }
}

fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        match c.is_uppercase() {
            true => {
                if i != 0 {
                    out.push('_');
                }
                out.push(c.to_ascii_lowercase());
            }
            false => out.push(c),
        };
    }
    out
}

struct PacketAttributes {
    state: State,
    id: i32,
    origin: Origin,
}

impl Parse for PacketAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let state_ident = input.parse::<syn::Ident>()?;
        let state = match state_ident.to_string().as_str() {
            "Handshake" => State::Handshake,
            "Status" => State::Status,
            "Login" => State::Login,
            "Play" => State::Play,
            _ => panic!("Invalid State"),
        };

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

        Ok(PacketAttributes { state, id, origin })
    }
}

fn load_packets(packets: &mut Vec<ItemStruct>, path: &str) {
    let src = fs::read_to_string(path).expect("failed to read file");
    let file = parse_file(&src).expect("failed to parse Rust file");

    file.items.iter().for_each(|item| {
        if let syn::Item::Struct(s) = item {
            packets.push(s.clone())
        }
    });
}

fn get_match_case(packet: &ItemStruct) -> TokenStream {
    let name = &packet.ident;

    let PacketAttributes { id, state, origin } = packet.attrs[0]
        .parse_args::<PacketAttributes>()
        .expect("invalid attribute");

    let src = match origin {
        Origin::Client => quote! { client },
        Origin::Server => quote! { server },
    };

    quote! {
        (#id, #origin, #state) => handle_payload::<#src::#name>(ctx, packet).await,
    }
}

fn get_event_field(packet: &ItemStruct) -> TokenStream {
    let name = &packet.ident;

    let PacketAttributes { id: _, state: _, origin } = packet.attrs[0]
        .parse_args::<PacketAttributes>()
        .expect("invalid attribute");

    let src = match origin {
        Origin::Client => quote! { client },
        Origin::Server => quote! { server },
    };

    let (_, ty_generics, _) = &packet.generics.split_for_impl();

    let snake_name = to_snake_case(name.to_string().as_str());
    let field_name = format!("{src}_{snake_name}");
    let field_ident = syn::Ident::new(&field_name, name.span());

    quote! {
        pub #field_ident: Vec<Box<dyn for<'a> Fn(&mut Context<#src::#name #ty_generics>) + Send + Sync + 'static>>,
    }
}

fn main() {
    println!("cargo::rerun-if-changed=src/packets/client.rs");
    println!("cargo::rerun-if-changed=src/packets/server.rs");
    println!("cargo::rerun-if-changed=build.rs");

    let out_dir = std::env::var("OUT_DIR").unwrap();

    let mut packets = Vec::new();
    load_packets(&mut packets, "src/packets/client.rs");
    load_packets(&mut packets, "src/packets/server.rs");

    let cases = packets.iter().map(get_match_case);

    let handle_fn = quote! {
        async fn handle_packet(ctx: &mut ProxyContext<'_>, packet: &Packet<'_>) -> Result<(), PacketError> {
            let state = ctx.state.mc_state.load(Ordering::Relaxed);

            match (packet.id, &ctx.source, state) {
                #( #cases )*
                _ => Err(PacketError::UnknownPacket),
            }
        }
    };

    fs::File::create(Path::new(&out_dir).join("handle_packet.rs"))
        .unwrap()
        .write_all(handle_fn.to_string().as_bytes())
        .unwrap();

    let fields = packets.iter().map(get_event_field);

    let events_struct = quote! {
        #[derive(Default)]
        pub struct PacketEvents {
                #( #fields )*
        }
    };

    fs::File::create(Path::new(&out_dir).join("packet_events.rs"))
        .unwrap()
        .write_all(events_struct.to_string().as_bytes())
        .unwrap();
}
