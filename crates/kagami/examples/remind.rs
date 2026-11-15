//! Reminders in chat: `/remind <seconds> <text>` shows `text` once the time
//! is up, and `/remind clear` cancels the pending ones. The command is a
//! derived type, which the client completes; the server never sees it. Each
//! reminder is a task that sleeps, and reminders end with the session.
//!
//! cargo run --release -p kagami --example remind -- 127.0.0.1:25566 play.example.net
//!
//! Online-mode servers need the account the client plays with, from the
//! launcher: `KAGAMI_ACCESS_TOKEN`, `KAGAMI_UUID` and `KAGAMI_NAME`.

use std::io;
use std::time::Duration;

use kagami::command::{Command, GreedyText};
use kagami::protocol::packets::client::SystemChatMessage;
use kagami::protocol::types::text::{Style, Text};
use kagami::{Account, Client, Context, Handlers, Kagami, TaskHandle};

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

/// `/remind clear` or `/remind <seconds> <text>`.
#[derive(Command)]
enum Remind<'c> {
    Clear,
    #[command(default)]
    Later {
        seconds: u32,
        text: GreedyText<'c>,
    },
}

fn reminders<H: Handlers<State>>(proxy: Kagami<State, H>) -> Kagami<State, impl Handlers<State>> {
    proxy.on_command::<Remind>(|remind, cx| {
        // Finished and stopped reminders are no longer pending.
        cx.state.pending.retain(TaskHandle::is_running);

        match remind {
            Remind::Clear => clear(cx),
            Remind::Later { seconds, text } => remind_later(cx, Duration::from_secs(seconds.into()), text.0.to_owned()),
        }
    })
}

fn clear(cx: &mut Context<'_, State>) {
    let cleared = cx.state.pending.len();
    for reminder in cx.state.pending.drain(..) {
        reminder.stop();
    }

    tell(cx.client, Text::new().plain(format!("Cleared {cleared} reminders")));
}

fn remind_later(cx: &mut Context<'_, State>, delay: Duration, text: String) {
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
}

fn tell(client: &Client, text: Text) {
    client.send(&SystemChatMessage { content: text.nbt(), overlay: false });
}
