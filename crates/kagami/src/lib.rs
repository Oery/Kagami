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

use std::io;
use std::marker::PhantomData;
use std::net::SocketAddr;

use mc_protocol::packets::{Clientbound, PacketKind, Serverbound};
use mc_protocol::{Direction, State};

pub use kagami_proxy::{Account, Action, Client, Context, Handlers, Interest, Server};
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

#[cfg(test)]
#[path = "../tests/tests_chain.rs"]
mod tests;
