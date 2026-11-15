//! Forwards one direction of a session after login, frame by frame.
//!
//! Frames without handlers join a run that is written straight from the
//! read buffer. A frame with handlers is decoded once, borrowing the
//! buffer, and the handlers decide whether it goes out as read, edited
//! or not at all. Packets the handlers send follow it.

use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use futures_lite::AsyncRead;
use mc_protocol::packets::{Clientbound, Serverbound};
use mc_protocol::{Direction, PacketWriter, State, VarInt};

use crate::compression::{Inflater, add_zero_data_length};
use crate::frame::{FrameBuffer, FrameHeader};
use crate::handlers::{Action, Handlers, Interest};
use crate::invalid_data;
use crate::outlet::Outlet;
use crate::session::Session;
use crate::transition::next_state;

pub struct Pump<S, H> {
    direction: Direction,
    state: State,
    /// Set when both legs use the compressed framing.
    inflater: Option<Inflater>,
    /// The packets wanted by handlers that are on, or by subscriptions. Only
    /// handlers and tasks change it, so it is refreshed before each scan and
    /// after handlers run, when the session counted a change.
    interest: Interest,
    /// The session's count of interest changes `interest` was computed at.
    interest_changes: u64,
    /// Shared by every session.
    handlers: Rc<RefCell<H>>,
    session: Session<S>,
    /// Holds a handled packet that arrived compressed.
    inflated: Vec<u8>,
    /// Written after the run: the handled packet when it was edited, then
    /// the packets its handlers sent to this pump's socket.
    tail: Vec<u8>,
}

/// What to write for the frames at the start of the buffer.
#[derive(Debug, PartialEq, Eq)]
pub struct Step {
    /// Bytes to write as they were read.
    run: usize,
    /// Bytes after the run that are consumed without being written: a frame
    /// that was dropped, or replaced by an edit in `Pump::tail`.
    skipped: usize,
}

/// Who wants a frame besides the socket it goes to.
#[derive(Clone, Copy)]
struct Wanted {
    handlers: bool,
    subscriptions: bool,
}

impl<S, H: Handlers<S>> Pump<S, H> {
    pub fn new(direction: Direction, compressed: bool, handlers: Rc<RefCell<H>>, session: Session<S>) -> Self {
        Self {
            direction,
            state: State::Config,
            inflater: compressed.then(Inflater::new),
            // Right until anything is turned off or subscribed to.
            interest: H::INTEREST,
            interest_changes: 0,
            handlers,
            session,
            inflated: Vec::new(),
            tail: Vec::new(),
        }
    }

    /// Forwards frames from `reader` until it ends.
    pub async fn run<R: AsyncRead + Unpin>(&mut self, buffer: &mut FrameBuffer, reader: &mut R) -> io::Result<()> {
        loop {
            self.forward(buffer).await?;

            if !buffer.fill(reader).await? {
                return Ok(());
            }
        }
    }

    /// Forwards the complete frames in `buffer`.
    async fn forward(&mut self, buffer: &mut FrameBuffer) -> io::Result<()> {
        loop {
            let step = self.scan(buffer.pending())?;
            if step.run == 0 && step.skipped == 0 {
                return Ok(());
            }

            let target = target(self.direction, &self.session);
            let run = &mut buffer.pending_mut()[..step.run];
            target.write(run, &mut self.tail).await?;

            buffer.consume(step.run + step.skipped);
            self.tail.clear();
        }
    }

    /// Walks every complete frame of `bytes` as `forward` does, discarding
    /// what it would write. For benchmarks.
    pub fn scan_without_writing(&mut self, bytes: &[u8]) -> io::Result<()> {
        let mut start = 0;

        loop {
            let step = self.scan(&bytes[start..])?;
            if step.run == 0 && step.skipped == 0 {
                return Ok(());
            }

            start += step.run + step.skipped;
            self.tail.clear();
        }
    }

    /// The socket this pump writes to.
    pub fn target(&self) -> &Outlet {
        target(self.direction, &self.session)
    }

    /// Walks the complete frames at the start of `bytes`, running handlers on
    /// the frames they want, and stops where something must be written:
    /// after a frame whose handlers changed its fate or sent packets.
    pub fn scan(&mut self, bytes: &[u8]) -> io::Result<Step> {
        self.refresh_interest();
        let mut run = 0;

        while let Some(header) = FrameHeader::parse(&bytes[run..])? {
            let frame_end = run + header.frame_len();
            if frame_end > bytes.len() {
                break;
            }

            let body = &bytes[run + header.header_len..frame_end];
            let id = match &mut self.inflater {
                Some(inflater) => {
                    let wanted = self.interest.any(self.direction, self.state);
                    inflater.packet_id(body, wanted)?
                }
                None => Some(VarInt::read(&mut &body[..]).map_err(invalid_data)?.0),
            };
            let Some(id) = id else {
                run = frame_end;
                continue;
            };
            // Ids past a byte belong to no known packet: no handler, no state change.
            let Ok(id) = u8::try_from(id) else {
                run = frame_end;
                continue;
            };

            if !self.interest.contains(self.direction, self.state, id) {
                self.state = next_state(self.direction, self.state, id);
                run = frame_end;
                continue;
            }

            let action = self.handle(id, body)?;
            self.refresh_interest();

            if action != Action::Drop {
                self.state = next_state(self.direction, self.state, id);
            }

            // Packets the handlers sent to this socket go out right after
            // this frame, ahead of anything a task sends later.
            target(self.direction, &self.session).take_outbox(&mut self.tail);

            match action {
                Action::Forward if self.tail.is_empty() => run = frame_end,
                Action::Forward => return Ok(Step { run: frame_end, skipped: 0 }),
                Action::Modified | Action::Drop => return Ok(Step { run, skipped: frame_end - run }),
            }
        }

        Ok(Step { run, skipped: 0 })
    }

    fn refresh_interest(&mut self) {
        let changes = self.session.interest_changes();
        if changes == self.interest_changes {
            return;
        }

        let handled = H::INTEREST.difference(self.session.disabled());
        self.interest = handled.union(self.session.subscriptions().interest());
        self.interest_changes = changes;
    }

    fn wanted(&self, id: u8) -> Wanted {
        let (direction, state) = (self.direction, self.state);

        let registered = H::INTEREST.contains(direction, state, id);
        let handlers = registered && !self.session.is_disabled(direction, state, id);
        let subscriptions = self.session.subscriptions().contains(direction, state, id);

        Wanted { handlers, subscriptions }
    }

    /// Copies one frame body to the subscriptions that want it, then decodes
    /// it and runs the handlers on it. An edited packet is encoded into
    /// `tail`. A packet that does not decode is forwarded as it is.
    fn handle(&mut self, id: u8, body: &[u8]) -> io::Result<Action> {
        let wanted = self.wanted(id);

        let packet = match &mut self.inflater {
            Some(inflater) => inflater.packet(body, &mut self.inflated)?,
            None => body,
        };

        if wanted.subscriptions {
            self.session.subscriptions().publish(self.direction, self.state, id, packet);
        }

        if !wanted.handlers {
            return Ok(Action::Forward);
        }

        let mut handlers = self.handlers.borrow_mut();

        let action = match self.direction {
            Direction::Serverbound => {
                let Ok(mut packet) = Serverbound::decode(self.state, packet) else {
                    return Ok(Action::Forward);
                };
                let action = self.session.with_context(|cx| handlers.serverbound(&mut packet, cx));

                if action == Action::Modified {
                    packet.encode(&mut self.tail).map_err(invalid_data)?;
                }

                action
            }
            Direction::Clientbound => {
                let Ok(mut packet) = Clientbound::decode(self.state, packet) else {
                    return Ok(Action::Forward);
                };
                let action = self.session.with_context(|cx| handlers.clientbound(&mut packet, cx));

                if action == Action::Modified {
                    packet.encode(&mut self.tail).map_err(invalid_data)?;
                }

                action
            }
        };

        if action == Action::Modified && self.inflater.is_some() {
            add_zero_data_length(&mut self.tail, 0)?;
        }

        Ok(action)
    }
}

/// The socket a pump going in `direction` writes to.
fn target<S>(direction: Direction, session: &Session<S>) -> &Outlet {
    match direction {
        Direction::Serverbound => &session.server.0,
        Direction::Clientbound => &session.client.0,
    }
}

#[cfg(test)]
#[path = "../tests/tests_pump.rs"]
mod tests;
