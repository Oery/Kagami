//! One client's session as handlers and tasks see it: its sockets, its
//! tasks and subscriptions, and the state every session shares.

use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;

use mc_protocol::State;
use mc_protocol::packets::PacketKind;

use crate::handlers::{Context, Interest};
use crate::outlet::{Client, Server};
use crate::subscription::{Subscription, Subscriptions};
use crate::tasks::{TaskHandle, Tasks};

/// What every session of one proxy shares.
pub struct Shared<S> {
    /// The state given to the proxy.
    pub state: RefCell<S>,
    /// Packet kinds whose handlers are turned off.
    pub disabled: RefCell<Interest>,
}

impl<S> Shared<S> {
    pub fn new(state: S) -> Self {
        Self { state: RefCell::new(state), disabled: RefCell::new(Interest::NONE) }
    }
}

/// One client's session. Cloning it is cheap; move a clone into a task to
/// send packets, read the state or subscribe from there.
///
/// Tasks spawned through it are cancelled, and its subscriptions end, when
/// the session does.
pub struct Session<S> {
    pub client: Client,
    pub server: Server,
    shared: Rc<Shared<S>>,
    tasks: Rc<Tasks>,
    subscriptions: Rc<Subscriptions>,
}

impl<S> Clone for Session<S> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            server: self.server.clone(),
            shared: self.shared.clone(),
            tasks: self.tasks.clone(),
            subscriptions: self.subscriptions.clone(),
        }
    }
}

impl<S> Session<S> {
    pub(crate) fn new(client: Client, server: Server, shared: Rc<Shared<S>>) -> Self {
        Self { client, server, shared, tasks: Rc::default(), subscriptions: Rc::default() }
    }

    /// Runs `f` on the state given to the proxy. For tasks: handlers have
    /// it as `cx.state`, and calling this from a handler panics.
    pub fn with<R>(&self, f: impl FnOnce(&mut S) -> R) -> R {
        let mut state = self.shared.state.try_borrow_mut().expect("handlers use cx.state rather than Session::with");

        f(&mut state)
    }

    /// Runs `task` next to the pumps, until it finishes, is stopped through
    /// the returned handle, or the session ends.
    ///
    /// The task never blocks packets, but it runs on the proxy's only
    /// thread: blocking calls belong in `glommio::executor().spawn_blocking`.
    pub fn spawn(&self, task: impl Future<Output = ()> + 'static) -> TaskHandle {
        self.tasks.spawn(task)
    }

    /// Copies every `K` packet of this session to the returned subscription,
    /// as it arrives and before handlers see it, until it is dropped.
    pub fn subscribe<K: PacketKind>(&self) -> Subscription<K> {
        const {
            assert!(
                matches!(K::STATE, State::Config | State::Play),
                "subscriptions only see configuration and play packets"
            );
        }

        self.subscriptions.subscribe::<K>()
    }

    /// Turns the handlers of `K` packets back on.
    pub fn enable<K: PacketKind>(&self) {
        let mut disabled = self.shared.disabled.borrow_mut();

        *disabled = disabled.without(K::DIRECTION, K::STATE, K::ID);
    }

    /// Turns off every handler of `K` packets, in every session, until
    /// [`enable`](Self::enable). `K` packets are then forwarded without
    /// being decoded. A packet being handled still reaches the rest of its
    /// handlers.
    pub fn disable<K: PacketKind>(&self) {
        let mut disabled = self.shared.disabled.borrow_mut();

        *disabled = disabled.with(K::DIRECTION, K::STATE, K::ID);
    }

    /// Whether the handlers of `K` packets are on.
    pub fn is_enabled<K: PacketKind>(&self) -> bool {
        !self.shared.disabled.borrow().contains(K::DIRECTION, K::STATE, K::ID)
    }

    /// The packet kinds whose handlers are turned off.
    pub(crate) fn disabled(&self) -> Interest {
        *self.shared.disabled.borrow()
    }

    pub(crate) fn subscriptions(&self) -> &Subscriptions {
        &self.subscriptions
    }

    /// Runs `f` with the context handlers get.
    pub(crate) fn with_context<R>(&self, f: impl FnOnce(&mut Context<'_, S>) -> R) -> R {
        let mut state = self.shared.state.borrow_mut();
        let mut cx = Context { state: &mut *state, client: &self.client, server: &self.server, session: self };

        f(&mut cx)
    }

    /// Ends the session's tasks and subscriptions. Packets sent afterwards
    /// are discarded.
    pub(crate) fn close(&self) {
        self.tasks.close();
        self.subscriptions.close();
        self.client.0.close();
        self.server.0.close();
    }
}
