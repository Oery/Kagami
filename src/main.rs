use crate::packets::*;
use error::KagamiError;

use proxy::Proxy;

mod context;
mod error;
mod events;
pub mod packet;
mod packets;
mod proxy;
mod state;
mod varint;

#[async_std::main]
async fn main() -> Result<(), KagamiError> {
    let mut app = Proxy::new();

    app.events.on_packet::<Handshake>(|ctx| {
        // ctx.proxy.state.step = ctx.payload.next_state;
    });

    app.events.on_packet::<SetCompression>(|ctx| {
        // ctx.proxy.state.cmp_threshold = ctx.payload.threshold;
    });

    app.run().await
}
