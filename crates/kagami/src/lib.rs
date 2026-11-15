//! Kagami: client-side Minecraft plugins, run in a local proxy between the
//! game and the server.
//!
//! Handlers are registered per packet type and run inline, in registration
//! order, as packets pass through. Packets without a handler are forwarded
//! without being decoded.
//!
//! ```no_run
//! use kagami::protocol::packets::client::SystemChatMessage;
//! use kagami::protocol::packets::server::ChatCommand;
//! use kagami::protocol::types::Nbt;
//! use kagami::{Action, Kagami};
//!
//! #[derive(Default)]
//! struct State {
//!     commands: u32,
//! }
//!
//! Kagami::new(State::default())
//!     .on_packet::<ChatCommand>(|command, cx| {
//!         cx.state.commands += 1;
//!         if command.command != "ping" {
//!             return Action::Forward;
//!         }
//!
//!         cx.client.send(&SystemChatMessage { content: Nbt::Str("pong".into()), overlay: false });
//!         Action::Drop
//!     })
//!     .run("127.0.0.1:25566", "127.0.0.1:25565")?;
//! # Ok::<(), std::io::Error>(())
//! ```
//!
//! A plugin made of several handlers is a function over the builder,
//! applied with [`Kagami::with`]:
//!
//! ```
//! use kagami::protocol::packets::client::SetHealth;
//! use kagami::{Action, Handlers, Kagami};
//!
//! fn low_health_warning<S>(proxy: Kagami<S, impl Handlers<S>>) -> Kagami<S, impl Handlers<S>> {
//!     let mut warned = false;
//!     proxy.on_packet::<SetHealth>(move |health, _| {
//!         warned = health.health < 6.0;
//!         Action::Forward
//!     })
//! }
//!
//! let proxy = Kagami::new(()).with(low_health_warning);
//! ```
//!
//! Handlers never wait. Anything that takes time runs in a task spawned in
//! the client's [`Session`]: it sends packets through the same handles,
//! reads the state with [`Session::with`], and is cancelled when the client
//! disconnects.
//!
//! ```
//! use std::time::Duration;
//!
//! use kagami::protocol::packets::server::ChatCommand;
//! use kagami::{Action, Kagami};
//!
//! #[derive(Default)]
//! struct State {
//!     spam: Option<kagami::TaskHandle>,
//! }
//!
//! let proxy = Kagami::new(State::default()).on_packet::<ChatCommand>(|command, cx| {
//!     if command.command != "spam" {
//!         return Action::Forward;
//!     }
//!
//!     // The second `/spam` stops the first.
//!     if let Some(spam) = cx.state.spam.take() {
//!         spam.stop();
//!         return Action::Drop;
//!     }
//!
//!     let server = cx.server.clone();
//!     let spam = cx.spawn(async move {
//!         loop {
//!             server.send(&ChatCommand { command: "say hello".into() });
//!             kagami::sleep(Duration::from_secs(2)).await;
//!         }
//!     });
//!     cx.state.spam = Some(spam);
//!
//!     Action::Drop
//! });
//! ```

// The command derives name `::kagami`, which inside this crate is itself.
extern crate self as kagami;

pub mod command;

use std::io;
use std::marker::PhantomData;
use std::net::SocketAddr;

use command::Command;
use mc_protocol::packets::client::{Commands, SystemChatMessage};
use mc_protocol::packets::server::ChatCommand;
use mc_protocol::packets::{Clientbound, PacketKind, Serverbound};
use mc_protocol::types::text::Text;
use mc_protocol::{Direction, State};

pub use glommio::future::timeout;
pub use glommio::timer::sleep;
pub use kagami_proxy::{Account, Action, Client, Context, Handlers, Interest, Server, Session, Subscription, TaskHandle};
pub use mc_protocol as protocol;

/// A proxy and its handlers, built by chaining.
pub struct Kagami<S, H> {
    state: S,
    handlers: H,
    account: Option<Account>,
}

impl<S> Kagami<S, ()> {
    /// A proxy without handlers. `state` is shared by every handler, as
    /// `cx.state`.
    pub fn new(state: S) -> Self {
        Self { state, handlers: (), account: None }
    }
}

impl<S, H: Handlers<S>> Kagami<S, H> {
    /// Runs `handler` on every `K` packet, after the handlers registered
    /// before it, which see and may edit the same decoded packet. A handler
    /// that drops the packet skips the ones after it.
    pub fn on_packet<K: PacketKind>(
        self, handler: impl for<'p> FnMut(&mut K::Packet<'p>, &mut Context<'_, S>) -> Action,
    ) -> Kagami<S, impl Handlers<S>> {
        const {
            assert!(matches!(K::STATE, State::Config | State::Play), "handlers only see configuration and play packets");
        }

        let handlers = OnPacket { handler, rest: self.handlers, kind: PhantomData::<fn() -> K> };

        Kagami { state: self.state, handlers, account: self.account }
    }

    /// Runs `handler` on every `/` command named [`C::NAME`](Command::NAME),
    /// parsed. The command is never sent to the server, and the client
    /// completes it like its own. A command that does not parse shows the
    /// player its usage instead of running the handler.
    ///
    /// Commands run among the packet handlers, in registration order: a
    /// [`ChatCommand`] handler before this one may still drop it.
    pub fn on_command<C: Command>(self, handler: impl for<'c> FnMut(C::Parsed<'c>, &mut Context<'_, S>)) -> Kagami<S, impl Handlers<S>> {
        let handlers = OnCommand { handler, rest: self.handlers, command: PhantomData::<fn() -> C> };

        Kagami { state: self.state, handlers, account: self.account }
    }

    /// Applies a bundle of handlers: a function over the builder.
    pub fn with<Bundled>(self, bundle: impl FnOnce(Self) -> Kagami<S, Bundled>) -> Kagami<S, Bundled> {
        bundle(self)
    }

    /// Logs in to online-mode servers as `account`.
    pub fn account(self, account: Account) -> Self {
        Self { account: Some(account), ..self }
    }

    /// Listens on `listen` and proxies every client to `server` (`host` or
    /// `host:port`), forever. Blocks the calling thread.
    pub fn run(self, listen: &str, server: &str) -> io::Result<()>
    where
        S: 'static,
        H: 'static,
    {
        let listen: SocketAddr = listen.parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid listen address"))?;
        let upstream = kagami_proxy::Upstream::parse(server)?;
        let config = kagami_proxy::Config { listen, upstream, account: self.account };

        kagami_proxy::run(config, self.state, self.handlers)
    }
}

/// One handler for packets of kind `K`, after the handlers in `rest`.
struct OnPacket<K, F, Rest> {
    handler: F,
    rest: Rest,
    kind: PhantomData<fn() -> K>,
}

impl<S, K, F, Rest> Handlers<S> for OnPacket<K, F, Rest>
where
    K: PacketKind,
    F: for<'p> FnMut(&mut K::Packet<'p>, &mut Context<'_, S>) -> Action,
    Rest: Handlers<S>,
{
    const INTEREST: Interest = Rest::INTEREST.with(K::DIRECTION, K::STATE, K::ID);

    fn serverbound(&mut self, packet: &mut Serverbound<'_>, cx: &mut Context<'_, S>) -> Action {
        let action = self.rest.serverbound(packet, cx);
        if action == Action::Drop || K::DIRECTION != Direction::Serverbound {
            return action;
        }

        let Some(packet) = K::from_serverbound(packet) else {
            return action;
        };

        action.then((self.handler)(packet, cx))
    }

    fn clientbound(&mut self, packet: &mut Clientbound<'_>, cx: &mut Context<'_, S>) -> Action {
        let action = self.rest.clientbound(packet, cx);
        if action == Action::Drop || K::DIRECTION != Direction::Clientbound {
            return action;
        }

        let Some(packet) = K::from_clientbound(packet) else {
            return action;
        };

        action.then((self.handler)(packet, cx))
    }
}

/// The handler of command `C`, after the handlers in `rest`.
struct OnCommand<C, F, Rest> {
    handler: F,
    rest: Rest,
    command: PhantomData<fn() -> C>,
}

impl<S, C, F, Rest> Handlers<S> for OnCommand<C, F, Rest>
where
    C: Command,
    F: for<'c> FnMut(C::Parsed<'c>, &mut Context<'_, S>),
    Rest: Handlers<S>,
{
    const INTEREST: Interest = Rest::INTEREST.with(Direction::Serverbound, State::Play, <ChatCommand as PacketKind>::ID).with(
        Direction::Clientbound,
        State::Play,
        <Commands as PacketKind>::ID,
    );

    fn serverbound(&mut self, packet: &mut Serverbound<'_>, cx: &mut Context<'_, S>) -> Action {
        let action = self.rest.serverbound(packet, cx);
        if action == Action::Drop {
            return action;
        }

        let Serverbound::ChatCommand(chat) = packet else {
            return action;
        };
        let Some(parsed) = command::parse::<C>(&chat.command) else {
            return action;
        };

        match parsed {
            Ok(parsed) => (self.handler)(parsed, cx),
            Err(error) => {
                let usage = Text::new().red(error.to_string());
                cx.client.send(&SystemChatMessage { content: usage.nbt(), overlay: false });
            }
        }

        Action::Drop
    }

    fn clientbound(&mut self, packet: &mut Clientbound<'_>, cx: &mut Context<'_, S>) -> Action {
        let action = self.rest.clientbound(packet, cx);
        if action == Action::Drop {
            return action;
        }

        let Clientbound::Commands(tree) = packet else {
            return action;
        };
        command::splice(tree, C::NAME, C::SYNTAX);

        action.then(Action::Modified)
    }
}

#[cfg(test)]
#[path = "../tests/tests_chain.rs"]
mod tests;
