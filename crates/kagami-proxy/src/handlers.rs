//! The interface between the pumps and user code: which packets have
//! handlers, what the handlers decided, and what they may touch.

use mc_protocol::packets::{Clientbound, Serverbound};
use mc_protocol::{Direction, State};

use crate::outlet::{Client, Server};

/// What happens to a packet once its handlers ran.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// The original bytes go out untouched.
    Forward,
    /// The packet was edited and is encoded again.
    Modified,
    /// The packet is not sent on.
    Drop,
}

impl Action {
    /// Combines the outcome so far with the next handler's: a drop wins,
    /// then a modification, which stays even if later handlers only forward.
    pub fn then(self, next: Action) -> Action {
        match (self, next) {
            (Action::Drop, _) | (_, Action::Drop) => Action::Drop,
            (Action::Modified, _) | (_, Action::Modified) => Action::Modified,
            (Action::Forward, Action::Forward) => Action::Forward,
        }
    }
}

const STATES: usize = 5;

/// One bit per packet id, per direction and state: the packets that have
/// handlers. Every other packet is forwarded without being decoded.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Interest {
    bits: [[[u64; 4]; STATES]; 2],
}

impl Interest {
    pub const NONE: Interest = Interest { bits: [[[0; 4]; STATES]; 2] };

    pub const fn with(mut self, direction: Direction, state: State, id: u8) -> Interest {
        let (word, bit) = Self::position(id);
        self.bits[direction as usize][state as usize][word] |= bit;

        self
    }

    pub const fn contains(&self, direction: Direction, state: State, id: u8) -> bool {
        let (word, bit) = Self::position(id);

        self.bits[direction as usize][state as usize][word] & bit != 0
    }

    const fn position(id: u8) -> (usize, u64) {
        (id as usize / 64, 1 << (id % 64))
    }
}

/// What a handler may touch besides its packet.
pub struct Context<'a, S> {
    /// The state given to the proxy, shared by every handler.
    pub state: &'a mut S,
    pub client: &'a Client,
    pub server: &'a Server,
}

/// Runs the handlers registered for a packet. Users build it by chaining
/// `on_packet` in the `kagami` crate rather than implementing it.
///
/// Only configuration and play packets reach handlers; the proxy relays
/// handshake, status and login itself.
pub trait Handlers<S> {
    /// The packets with at least one handler.
    const INTEREST: Interest;

    fn serverbound(&mut self, packet: &mut Serverbound<'_>, cx: &mut Context<'_, S>) -> Action;

    fn clientbound(&mut self, packet: &mut Clientbound<'_>, cx: &mut Context<'_, S>) -> Action;
}

/// No handlers: every packet is forwarded without being decoded.
impl<S> Handlers<S> for () {
    const INTEREST: Interest = Interest::NONE;

    fn serverbound(&mut self, _: &mut Serverbound<'_>, _: &mut Context<'_, S>) -> Action {
        Action::Forward
    }

    fn clientbound(&mut self, _: &mut Clientbound<'_>, _: &mut Context<'_, S>) -> Action {
        Action::Forward
    }
}

#[cfg(test)]
#[path = "../tests/tests_handlers.rs"]
mod tests;
