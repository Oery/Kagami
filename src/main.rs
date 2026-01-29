use crate::error::KagamiError;
use crate::events::Context;
use crate::packets::client::Chat;
use crate::proxy::Proxy;

mod context;
mod error;
mod events;
pub mod packet;
mod packets;
mod proxy;
mod state;
mod varint;

fn handle_ping(ctx: &mut Context<Chat>) {
    ctx.cancel_packet(); // Cancel the packet so that it is not sent to the server
    ctx.client.chat("Pong!"); // Send a chat message to the client
}

fn handle_hello_world(ctx: &mut Context<Chat>) {
    ctx.cancel_packet();
    ctx.client.chat("Hello, World!");
}

#[async_std::main]
async fn main() -> Result<(), KagamiError> {
    let mut app = Proxy::new();

    app.events.on_packet::<Chat>(|ctx| {
        match ctx.payload.message.as_ref() {
            "/ping" => handle_ping(ctx),
            "/hw" => handle_hello_world(ctx),
            _ => {}
        };
    });

    app.run().await
}
