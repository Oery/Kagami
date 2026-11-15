//! Who is online: `/players` lists the players the server knows, and join
//! and leave messages are hidden until `/joins` turns them back on.
//!
//! The list comes from the server's own tab completion of `/msg `: a task
//! asks for it and waits for the answer through a subscription. Hiding the
//! join messages is a handler that `/joins` turns off and on; while it is
//! off, chat is not even decoded.
//!
//! cargo run --release -p kagami --example players -- 127.0.0.1:25566 play.example.net
//!
//! Online-mode servers need the account the client plays with, from the
//! launcher: `KAGAMI_ACCESS_TOKEN`, `KAGAMI_UUID` and `KAGAMI_NAME`.

use std::io;
use std::time::Duration;

use kagami::protocol::packets::client::{CommandSuggestionsResponse, SystemChatMessage};
use kagami::protocol::packets::server::{ChatCommand, CommandSuggestion};
use kagami::protocol::types::Nbt;
use kagami::protocol::types::text::{Style, Text};
use kagami::{Account, Action, Client, Handlers, Kagami, Session};

/// Marks the proxy's own completion requests. Clients count theirs up from
/// zero.
const TRANSACTION_ID: i32 = -1;

const ANSWER_TIMEOUT: Duration = Duration::from_secs(5);

const JOIN_KEYS: [&str; 2] = ["multiplayer.player.joined", "multiplayer.player.left"];

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let (Some(listen), Some(server)) = (args.next(), args.next()) else {
        eprintln!("usage: players <listen address> <server[:port]>");
        std::process::exit(2);
    };

    let mut proxy = Kagami::new(()).with(players);

    if let Some(account) = Account::from_env()? {
        proxy = proxy.account(account);
    }

    proxy.run(&listen, &server)
}

fn players<H: Handlers<()>>(proxy: Kagami<(), H>) -> Kagami<(), impl Handlers<()>> {
    proxy
        .on_packet::<ChatCommand>(|command, cx| match command.command.as_ref() {
            "players" => {
                cx.spawn(list_players(cx.session.clone()));
                Action::Drop
            }
            "joins" => {
                let hidden = cx.session.is_enabled::<SystemChatMessage>();

                if hidden {
                    cx.session.disable::<SystemChatMessage>();
                    tell(cx.client, Text::new().plain("Join and leave messages are shown"));
                } else {
                    cx.session.enable::<SystemChatMessage>();
                    tell(cx.client, Text::new().plain("Join and leave messages are hidden"));
                }

                Action::Drop
            }
            _ => Action::Forward,
        })
        .on_packet::<SystemChatMessage>(|message, _| {
            let Some(key) = translation_key(&message.content) else {
                return Action::Forward;
            };

            if JOIN_KEYS.contains(&key) {
                return Action::Drop;
            }

            Action::Forward
        })
        // The subscription saw the answer before this handler hides it from
        // the client.
        .on_packet::<CommandSuggestionsResponse>(|response, _| {
            if response.transaction_id == TRANSACTION_ID {
                return Action::Drop;
            }

            Action::Forward
        })
}

/// Asks the server to complete `/msg ` and lists the names it answers with.
async fn list_players(session: Session<()>) {
    let mut answers = session.subscribe::<CommandSuggestionsResponse>();
    session.server.send(&CommandSuggestion { transaction_id: TRANSACTION_ID, text: "/msg ".into() });

    let answer = async {
        while let Some(answer) = answers.next().await {
            if answer.transaction_id == TRANSACTION_ID {
                return Some(answer);
            }
        }

        None
    };

    let Ok(Some(answer)) = kagami::timeout(ANSWER_TIMEOUT, answer).await else {
        tell(&session.client, Text::new().push("The server did not list its players", Style::color("red")));
        return;
    };

    let names: Vec<&str> = answer.matches.iter().map(|name| name.text.as_ref()).collect();
    let message = Text::new().push(format!("Online ({}): ", names.len()), Style::color("gold")).plain(names.join(", "));
    tell(&session.client, message);
}

/// The `translate` key of a chat component, such as `multiplayer.player.joined`.
fn translation_key<'a>(content: &'a Nbt<'_>) -> Option<&'a str> {
    let Nbt::Compound(entries) = content else {
        return None;
    };

    for (name, value) in entries {
        if name != "translate" {
            continue;
        }

        let Nbt::Str(key) = value else {
            return None;
        };

        return Some(key);
    }

    None
}

fn tell(client: &Client, text: Text) {
    client.send(&SystemChatMessage { content: text.nbt(), overlay: false });
}
