//! Where packets for one socket go: the socket's writer, shared by the
//! pump that forwards to it and by handlers that send to it.

use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use glommio::sync::{Mutex, Semaphore};
use mc_protocol::{Direction, Payload};

use crate::compression::add_zero_data_length;
use crate::split::WriteHalf;
use crate::stream::Writer;

/// One socket's writing side. Whoever writes holds the lock for whole
/// frames, so frames never interleave and the cipher stream follows the
/// order bytes reach the socket.
pub struct Outlet {
    writer: Mutex<Writer<WriteHalf>>,
    /// Frames sent by handlers, already in this leg's framing.
    outbox: RefCell<Vec<u8>>,
    compressed: bool,
    /// A permit means the outbox has frames for the flusher.
    pending: Semaphore,
}

impl Outlet {
    pub fn new(writer: Writer<WriteHalf>, compressed: bool) -> Self {
        Self { writer: Mutex::new(writer), outbox: RefCell::new(Vec::new()), compressed, pending: Semaphore::new(0) }
    }

    /// Encodes `packet` into the outbox. It is written after the frame being
    /// handled when this socket's pump runs the handler, or by the flusher.
    fn send<'p>(&self, packet: &impl Payload<'p>) {
        let mut outbox = self.outbox.borrow_mut();
        let start = outbox.len();

        let mut encoded = packet.encode(&mut outbox).map_err(io::Error::other);

        if encoded.is_ok() && self.compressed {
            encoded = add_zero_data_length(&mut outbox, start);
        }

        // An invalid packet is a bug in the handler; it is left out rather
        // than ending the session.
        if encoded.is_err() {
            debug_assert!(false, "a handler sent a packet that cannot be encoded");
            outbox.truncate(start);
        }
    }

    pub fn has_outbox(&self) -> bool {
        !self.outbox.borrow().is_empty()
    }

    /// Lets the flusher write the outbox, for packets sent to this socket
    /// while its own pump may be waiting on a read.
    pub fn wake_flusher(&self) {
        if self.pending.available() == 0 {
            self.pending.signal(1);
        }
    }

    /// Writes `run` (bytes from the pump's read buffer), then `replacement`,
    /// then the outbox, in one hold of the lock.
    pub async fn write(&self, run: &mut [u8], replacement: &mut [u8]) -> io::Result<()> {
        let mut writer = self.writer.lock().await.map_err(io::Error::other)?;
        writer.write_all_in_place(run).await?;
        writer.write_all_in_place(replacement).await?;

        Self::write_outbox(&self.outbox, &mut writer).await
    }

    /// Writes whatever the outbox holds each time the flusher is woken, until
    /// the session closes the outlet.
    pub async fn flush_when_woken(&self) -> io::Result<()> {
        while self.pending.acquire(1).await.is_ok() {
            let mut writer = self.writer.lock().await.map_err(io::Error::other)?;
            Self::write_outbox(&self.outbox, &mut writer).await?;
        }

        Ok(())
    }

    /// Stops the flusher once the session is over.
    pub fn close(&self) {
        self.pending.close();
    }

    pub async fn shutdown(&self) -> io::Result<()> {
        let writer = self.writer.lock().await.map_err(io::Error::other)?;

        writer.shutdown().await
    }

    /// Writes the outbox, including frames handlers add while it is written.
    /// Its buffer is taken out for the write and given back for reuse.
    async fn write_outbox(outbox: &RefCell<Vec<u8>>, writer: &mut Writer<WriteHalf>) -> io::Result<()> {
        loop {
            let mut frames = std::mem::take(&mut *outbox.borrow_mut());
            if frames.is_empty() {
                return Ok(());
            }

            writer.write_all_in_place(&mut frames).await?;

            // Kept for reuse unless handlers already started a new outbox.
            frames.clear();

            if outbox.borrow().is_empty() {
                *outbox.borrow_mut() = frames;
            }
        }
    }
}

/// Sends packets to the client. Cloning it is cheap.
#[derive(Clone)]
pub struct Client(pub(crate) Rc<Outlet>);

/// Sends packets to the server. Cloning it is cheap.
#[derive(Clone)]
pub struct Server(pub(crate) Rc<Outlet>);

impl Client {
    /// Queues a clientbound packet. Sent while handling a clientbound packet,
    /// it goes out right after it, or in its place when it was dropped.
    pub fn send<'p, P: Payload<'p>>(&self, packet: &P) {
        const { assert!(matches!(P::DIRECTION, Direction::Clientbound), "only clientbound packets can be sent to the client") };

        self.0.send(packet);
    }
}

impl Server {
    /// Queues a serverbound packet. Sent while handling a serverbound packet,
    /// it goes out right after it, or in its place when it was dropped.
    pub fn send<'p, P: Payload<'p>>(&self, packet: &P) {
        const { assert!(matches!(P::DIRECTION, Direction::Serverbound), "only serverbound packets can be sent to the server") };

        self.0.send(packet);
    }
}
