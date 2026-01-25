use crate::events::Context;
use error::KagamiError;

use proxy::Proxy;

use crate::packets::*;

mod context;
mod error;
mod events;
pub mod packet;
mod packets;
mod proxy;
mod state;
mod varint;

fn handle_ping(ctx: &mut Context<Chat>) {
    ctx.cancel();
    ctx.client.chat("Pong!");
}

fn handle_hello_world(ctx: &mut Context<Chat>) {
    ctx.cancel();
    ctx.client.chat("Hello, World!");
}

#[async_std::main]
async fn main() -> Result<(), KagamiError> {
    let mut app = Proxy::new();

    app.events.on_packet::<LoginSuccess>(|ctx| {
        dbg!(&ctx.payload);
    });

    app.events.on_packet::<Chat>(|ctx| {
        match ctx.payload.message.as_ref() {
            "/ping" => handle_ping(ctx),
            "/hw" => handle_hello_world(ctx),
            _ => {}
        };
    });

    app.run().await
}
