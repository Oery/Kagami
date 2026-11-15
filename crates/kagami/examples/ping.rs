//! Answers `/ping` in the proxy itself: the server never sees the command.
//!
//! cargo run --release -p kagami --example ping -- 127.0.0.1:25566 play.example.net
//!
//! Online-mode servers need the account the client plays with, from the
//! launcher: `KAGAMI_ACCESS_TOKEN`, `KAGAMI_UUID` and `KAGAMI_NAME`.

use std::io;

use kagami::command::Command;
use kagami::protocol::packets::client::SystemChatMessage;
use kagami::protocol::types::text::{Style, Text};
use kagami::{Account, Kagami};

#[derive(Default)]
struct State {
    pings: u32,
}

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let (Some(listen), Some(server)) = (args.next(), args.next()) else {
        eprintln!("usage: ping <listen address> <server[:port]>");
        std::process::exit(2);
    };

    let mut proxy = Kagami::new(State::default()).with(ping);

    if let Some(account) = Account::from_env()? {
        proxy = proxy.account(account);
    }

    proxy.run(&listen, &server)
}

#[derive(Command)]
struct Ping;

fn ping<H: kagami::Handlers<State>>(proxy: Kagami<State, H>) -> Kagami<State, impl kagami::Handlers<State>> {
    proxy.on_command::<Ping>(|Ping, cx| {
        cx.state.pings += 1;

        let text = Text::new().push("Pong ", Style::color("gold")).plain(format!("#{}", cx.state.pings));
        cx.client.send(&SystemChatMessage { content: text.nbt(), overlay: false });
    })
}
