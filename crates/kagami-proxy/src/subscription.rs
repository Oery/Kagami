//! Packets copied to tasks as they pass through the pumps.
//!
//! A subscription only observes: the packet still goes on to its handlers
//! and then out, whatever the subscriber does with its copy.

use std::cell::{Cell, RefCell};
use std::future::poll_fn;
use std::marker::PhantomData;
use std::rc::{Rc, Weak};
use std::task::{Context, Poll, Waker};

use mc_protocol::packets::PacketKind;
use mc_protocol::{Direction, IntoOwned, PacketWriter, Payload, State, VarInt};

use crate::handlers::Interest;

/// The subscriptions of one session.
#[derive(Default)]
pub struct Subscriptions {
    /// The packets at least one queue wants.
    interest: RefCell<Interest>,
    queues: RefCell<Vec<Rc<Queue>>>,
    /// Set once the session ended: queues are closed and new ones start closed.
    closed: Cell<bool>,
    /// Counts changes to `interest`, so pumps notice them without comparing
    /// whole sets.
    changes: Cell<u64>,
}

impl Subscriptions {
    pub fn subscribe<K: PacketKind>(self: &Rc<Self>) -> Subscription<K> {
        let queue = Rc::new(Queue {
            direction: K::DIRECTION,
            state: K::STATE,
            id: K::ID,
            packets: RefCell::new(Packets::default()),
            waker: Cell::new(None),
            closed: Cell::new(self.closed.get()),
        });

        if !self.closed.get() {
            self.queues.borrow_mut().push(queue.clone());

            let mut interest = self.interest.borrow_mut();
            *interest = interest.with(K::DIRECTION, K::STATE, K::ID);
            self.changed();
        }

        Subscription { queue, subscriptions: Rc::downgrade(self), kind: PhantomData }
    }

    /// The packets at least one subscription wants.
    pub fn interest(&self) -> Interest {
        *self.interest.borrow()
    }

    pub fn contains(&self, direction: Direction, state: State, id: u8) -> bool {
        self.interest.borrow().contains(direction, state, id)
    }

    pub fn changes(&self) -> u64 {
        self.changes.get()
    }

    fn changed(&self) {
        self.changes.set(self.changes.get() + 1);
    }

    /// Copies `packet` (its id, then its payload) to every queue that wants
    /// it and wakes their tasks.
    pub fn publish(&self, direction: Direction, state: State, id: u8, packet: &[u8]) {
        for queue in self.queues.borrow().iter() {
            if !queue.wants(direction, state, id) {
                continue;
            }

            queue.packets.borrow_mut().push(packet);
            queue.wake();
        }
    }

    /// Ends every subscription: their tasks read what is queued, then `None`.
    pub fn close(&self) {
        self.closed.set(true);
        *self.interest.borrow_mut() = Interest::NONE;
        self.changed();

        let queues = std::mem::take(&mut *self.queues.borrow_mut());
        for queue in queues {
            queue.closed.set(true);
            queue.wake();
        }
    }

    fn unsubscribe(&self, queue: &Rc<Queue>) {
        let mut queues = self.queues.borrow_mut();
        queues.retain(|other| !Rc::ptr_eq(other, queue));

        // Another subscription to the same packet keeps it wanted.
        let still_wanted = queues.iter().any(|other| other.wants(queue.direction, queue.state, queue.id));
        if still_wanted {
            return;
        }

        let mut interest = self.interest.borrow_mut();
        *interest = interest.without(queue.direction, queue.state, queue.id);
        self.changed();
    }
}

/// Packets of one kind waiting for a task.
struct Queue {
    direction: Direction,
    state: State,
    id: u8,
    packets: RefCell<Packets>,
    waker: Cell<Option<Waker>>,
    closed: Cell<bool>,
}

impl Queue {
    fn wants(&self, direction: Direction, state: State, id: u8) -> bool {
        self.direction == direction && self.state == state && self.id == id
    }

    fn wake(&self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }
}

/// Queued packets in one buffer, each behind its length, so a busy
/// subscription does not allocate per packet.
#[derive(Default)]
struct Packets {
    bytes: Vec<u8>,
    /// Where the oldest unread packet starts.
    start: usize,
}

const LENGTH_PREFIX: usize = size_of::<u32>();

impl Packets {
    fn push(&mut self, packet: &[u8]) {
        // Frames are capped at 2 MiB by the protocol, far below `u32::MAX`.
        let length = packet.len() as u32;

        self.bytes.extend_from_slice(&length.to_le_bytes());
        self.bytes.extend_from_slice(packet);
    }

    /// Removes the oldest packet and returns its bytes.
    fn pop(&mut self) -> Option<&[u8]> {
        // Everything was read: reuse the buffer from its start.
        if self.start == self.bytes.len() {
            self.bytes.clear();
            self.start = 0;

            return None;
        }

        let body_start = self.start + LENGTH_PREFIX;
        let length = u32::from_le_bytes(self.bytes[self.start..body_start].try_into().expect("a length is four bytes"));
        let body_end = body_start + length as usize;
        self.start = body_end;

        Some(&self.bytes[body_start..body_end])
    }
}

/// Packets of kind `K` copied from the session as they arrive, before
/// handlers see them. Dropping it ends the subscription.
pub struct Subscription<K: PacketKind> {
    queue: Rc<Queue>,
    subscriptions: Weak<Subscriptions>,
    kind: PhantomData<fn() -> K>,
}

impl<K: PacketKind> Subscription<K> {
    /// Waits for the next `K` packet. Returns `None` once the session ended
    /// and every queued packet was read.
    ///
    /// Packets queue up until they are read, so a subscription that is no
    /// longer read should be dropped.
    pub async fn next(&mut self) -> Option<K::Packet<'static>> {
        poll_fn(|cx| self.poll_next(cx)).await
    }

    fn poll_next(&self, cx: &mut Context<'_>) -> Poll<Option<K::Packet<'static>>> {
        let mut packets = self.queue.packets.borrow_mut();

        loop {
            let Some(packet) = packets.pop() else {
                if self.queue.closed.get() {
                    return Poll::Ready(None);
                }

                self.queue.waker.set(Some(cx.waker().clone()));
                return Poll::Pending;
            };

            // A packet that does not decode is skipped, as the pump forwards
            // it without running handlers.
            if let Some(packet) = decode::<K>(packet) {
                return Poll::Ready(Some(packet));
            }
        }
    }
}

impl<K: PacketKind> Drop for Subscription<K> {
    fn drop(&mut self) {
        let Some(subscriptions) = self.subscriptions.upgrade() else {
            return;
        };

        subscriptions.unsubscribe(&self.queue);
    }
}

/// Decodes a queued packet, its id then its payload, into an owned `K`.
fn decode<K: PacketKind>(packet: &[u8]) -> Option<K::Packet<'static>> {
    let mut payload = packet;
    VarInt::read(&mut payload).ok()?;

    let packet = K::Packet::decode_payload(&mut payload).ok()?;

    Some(packet.into_owned())
}
