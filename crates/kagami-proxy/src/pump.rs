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
use crate::handlers::{Action, Context, Handlers, Interest};
use crate::invalid_data;
use crate::outlet::{Client, Outlet, Server};
use crate::transition::next_state;

/// The handlers and their state, shared by every pump of every session.
pub struct Shared<S, H> {
    pub state: RefCell<S>,
    pub handlers: RefCell<H>,
}

pub struct Pump<S, H> {
    direction: Direction,
    state: State,
    /// Set when both legs use the compressed framing.
    inflater: Option<Inflater>,
    /// A runtime copy of the handlers' interest.
    interest: Interest,
    shared: Rc<Shared<S, H>>,
    client: Client,
    server: Server,
    /// Holds a handled packet that arrived compressed.
    inflated: Vec<u8>,
    /// Holds an edited packet, encoded again.
    replacement: Vec<u8>,
}

/// What to write for the frames at the start of the buffer.
#[derive(Debug, PartialEq, Eq)]
struct Step {
    /// Bytes to write as they were read.
    run: usize,
    /// Bytes after the run that are consumed without being written: a frame
    /// that was dropped, or replaced by `Pump::replacement`.
    skipped: usize,
}

impl<S, H: Handlers<S>> Pump<S, H> {
    pub fn new(direction: Direction, compressed: bool, shared: Rc<Shared<S, H>>, client: Client, server: Server) -> Self {
        Self {
            direction,
            state: State::Config,
            inflater: compressed.then(Inflater::new),
            interest: H::INTEREST,
            shared,
            client,
            server,
            inflated: Vec::new(),
            replacement: Vec::new(),
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

            let target = match self.direction {
                Direction::Serverbound => &self.server.0,
                Direction::Clientbound => &self.client.0,
            };
            let run = &mut buffer.pending_mut()[..step.run];
            target.write(run, &mut self.replacement).await?;

            buffer.consume(step.run + step.skipped);
            self.replacement.clear();
        }
    }

    /// The socket this pump writes to.
    pub fn target(&self) -> &Outlet {
        match self.direction {
            Direction::Serverbound => &self.server.0,
            Direction::Clientbound => &self.client.0,
        }
    }

    /// The socket the other pump writes to.
    fn other(&self) -> &Outlet {
        match self.direction {
            Direction::Serverbound => &self.client.0,
            Direction::Clientbound => &self.server.0,
        }
    }

    /// Walks the complete frames at the start of `bytes`, running handlers on
    /// the frames they want, and stops where something must be written:
    /// after a frame whose handlers changed its fate or sent packets.
    fn scan(&mut self, bytes: &[u8]) -> io::Result<Step> {
        let mut run = 0;

        while let Some(header) = FrameHeader::parse(&bytes[run..])? {
            let frame_end = run + header.frame_len();
            if frame_end > bytes.len() {
                break;
            }

            let body = &bytes[run + header.header_len..frame_end];
            let id = match &mut self.inflater {
                Some(inflater) => inflater.packet_id(body)?,
                None => VarInt::read(&mut &body[..]).map_err(invalid_data)?.0,
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

            let action = self.handle(body)?;

            // Packets sent to the other socket are written by its flusher.
            if self.other().has_outbox() {
                self.other().wake_flusher();
            }

            if action != Action::Drop {
                self.state = next_state(self.direction, self.state, id);
            }

            match action {
                Action::Forward if !self.target().has_outbox() => run = frame_end,
                Action::Forward => return Ok(Step { run: frame_end, skipped: 0 }),
                Action::Modified | Action::Drop => return Ok(Step { run, skipped: frame_end - run }),
            }
        }

        Ok(Step { run, skipped: 0 })
    }

    /// Decodes one frame body and runs the handlers on it. An edited packet
    /// is encoded into `replacement`. A packet that does not decode is
    /// forwarded as it is.
    fn handle(&mut self, body: &[u8]) -> io::Result<Action> {
        let packet = match &mut self.inflater {
            Some(inflater) => inflater.packet(body, &mut self.inflated)?,
            None => body,
        };

        let mut state = self.shared.state.borrow_mut();
        let mut handlers = self.shared.handlers.borrow_mut();
        let mut cx = Context { state: &mut *state, client: &self.client, server: &self.server };

        let action = match self.direction {
            Direction::Serverbound => {
                let Ok(mut packet) = Serverbound::decode(self.state, packet) else {
                    return Ok(Action::Forward);
                };
                let action = handlers.serverbound(&mut packet, &mut cx);

                if action == Action::Modified {
                    packet.encode(&mut self.replacement).map_err(invalid_data)?;
                }

                action
            }
            Direction::Clientbound => {
                let Ok(mut packet) = Clientbound::decode(self.state, packet) else {
                    return Ok(Action::Forward);
                };
                let action = handlers.clientbound(&mut packet, &mut cx);

                if action == Action::Modified {
                    packet.encode(&mut self.replacement).map_err(invalid_data)?;
                }

                action
            }
        };

        if action == Action::Modified && self.inflater.is_some() {
            add_zero_data_length(&mut self.replacement, 0)?;
        }

        Ok(action)
    }
}

#[cfg(test)]
#[path = "../tests/tests_pump.rs"]
mod tests;
