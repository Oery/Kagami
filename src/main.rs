use std::process::Command;

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

#[async_std::main]
async fn main() -> Result<(), KagamiError> {
    let mut app = Proxy::new();

    app.events.on_packet::<Chat>(|ctx| {
        if ctx.payload.message.starts_with("!") {
            let mut echo_hello = Command::new("sh");
            let command = ctx.payload.message.strip_prefix("!").unwrap();
            echo_hello.arg("-c").arg(command);
            let hello_1 = echo_hello.output().expect("failed to execute process");
            let mut stdout = String::from_utf8(hello_1.stdout).expect("stdout is not valid UTF-8");
            stdout = stdout.replace("'", "");
            stdout.truncate(256);

            dbg!(&stdout);

            ctx.should_filter = true;
            ctx.client
                .send(&ServerChat { json: format!("'{}'", stdout), position: 0 });
        }
    });

    app.run().await
}
