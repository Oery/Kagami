use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    println!("cargo::rerun-if-changed=src/packets/client.rs");
    println!("cargo::rerun-if-changed=src/packets/server.rs");
    println!("cargo::rerun-if-changed=build.rs");

    let out_dir = std::env::var("OUT_DIR").unwrap();

    let handle_fn = quote::quote! {
        async fn handle_packet(ctx: &mut ProxyContext<'_>, packet: &Packet<'_>) -> Result<(), PacketError> {
            let state = ctx.state.mc_state.load(Ordering::Relaxed);

            use crate::context::Source::*;

            match (packet.id, &ctx.source, state) {
                (0x00, Client, McState::Handshake) => handle_payload::<Handshake>(ctx, packet).await,
                (0xFE, Client, McState::Handshake) => handle_payload::<LegacyPing>(ctx, packet).await,
                (0x01, Client, McState::Play) => handle_payload::<client::Chat>(ctx, packet).await,
                (0x02, Server, McState::Login) => handle_payload::<LoginSuccess>(ctx, packet).await,
                (0x03, Server, McState::Login) => handle_payload::<SetCompression>(ctx, packet).await,
                (0x00, Server, McState::Play) => handle_payload::<server::KeepAlive>(ctx, packet).await,
                (0x02, Server, McState::Play) => handle_payload::<server::Chat>(ctx, packet).await,
                (0x46, Server, McState::Play) => handle_payload::<SetCompression>(ctx, packet).await,

                _ => Err(PacketError::UnknownPacket),
            }
        }
    };

    File::create(Path::new(&out_dir).join("handle_packet.rs"))
        .unwrap()
        .write_all(handle_fn.to_string().as_bytes())
        .unwrap();
}
