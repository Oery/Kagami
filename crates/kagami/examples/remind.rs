//! Reminders in chat: `/remind <seconds> <text>` shows `text` once the time
//! is up, and `/remind clear` cancels the pending ones. Each reminder is a
//! task that sleeps; the server never sees the commands, and reminders end
//! with the session.
//!
//! cargo run --release -p kagami --example remind -- 127.0.0.1:25566 play.example.net
//!
//! Online-mode servers need the account the client plays with, from the
//! launcher: `KAGAMI_ACCESS_TOKEN`, `KAGAMI_UUID` and `KAGAMI_NAME`.

use std::io;
use std::time::Duration;

use kagami::protocol::packets::client::SystemChatMessage;
use kagami::protocol::packets::server::ChatCommand;
use kagami::protocol::types::text::{Style, Text};
use kagami::{Account, Action, Client, Handlers, Kagami, TaskHandle};

#[derive(Default)]
struct State {
    pending: Vec<TaskHandle>,
    /// Reminders shown so far, counted by the tasks.
    shown: u32,
}

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let (Some(listen), Some(server)) = (args.next(), args.next()) else {
        eprintln!("usage: remind <listen address> <server[:port]>");
        std::process::exit(2);
    };

    let mut proxy = Kagami::new(State::default()).with(reminders);

    if let Some(account) = Account::from_env()? {
        proxy = proxy.account(account);
    }

    proxy.run(&listen, &server)
}

fn reminders<H: Handlers<State>>(proxy: Kagami<State, H>) -> Kagami<State, impl Handlers<State>> {
    proxy.on_packet::<ChatCommand>(|command, cx| {
        let (name, arguments) = command.command.split_once(' ').unwrap_or((&command.command, ""));
        if name != "remind" {
            return Action::Forward;
        }

        // Finished and stopped reminders are no longer pending.
        cx.state.pending.retain(TaskHandle::is_running);

        if arguments == "clear" {
            let cleared = cx.state.pending.len();
            for reminder in cx.state.pending.drain(..) {
                reminder.stop();
            }

            tell(cx.client, Text::new().plain(format!("Cleared {cleared} reminders")));
            return Action::Drop;
        }

        let Some((delay, text)) = parse(arguments) else {
            tell(cx.client, Text::new().push("Usage: /remind <seconds> <text> or /remind clear", Style::color("red")));
            return Action::Drop;
        };

        let session = cx.session.clone();
        let reminder = cx.spawn(async move {
            kagami::sleep(delay).await;

            let number = session.with(|state| {
                state.shown += 1;
                state.shown
            });
            let message = Text::new().push(format!("Reminder #{number}: "), Style::color("gold")).plain(text);
            tell(&session.client, message);
        });
        cx.state.pending.push(reminder);

        tell(cx.client, Text::new().plain(format!("Reminding you in {} s", delay.as_secs())));

        Action::Drop
    })
}

/// Splits `<seconds> <text>`.
fn parse(arguments: &str) -> Option<(Duration, String)> {
    let (seconds, text) = arguments.split_once(' ')?;
    let seconds = seconds.parse().ok()?;

    Some((Duration::from_secs(seconds), text.to_owned()))
}

fn tell(client: &Client, text: Text) {
    client.send(&SystemChatMessage { content: text.nbt(), overlay: false });
}
