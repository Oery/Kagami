//! One socket during handshake, status and login, where the proxy reads and
//! sends whole packets.

use std::io;
use std::ops::Range;

use futures_lite::AsyncWriteExt;
use glommio::net::TcpStream;
use mc_protocol::State;
use mc_protocol::packets::{Clientbound, Serverbound};

use crate::compression::{Inflater, add_zero_data_length};
use crate::frame::FrameBuffer;
use crate::invalid_data;
use crate::split::{ReadHalf, WriteHalf, split};

pub struct Connection {
    reader: ReadHalf,
    writer: WriteHalf,
    buffer: FrameBuffer,
    /// Set once the server enabled compression for this leg.
    compression: Option<Inflater>,
    /// Holds the last packet that arrived compressed.
    inflated: Vec<u8>,
    /// Reused for encoding outgoing packets.
    outgoing: Vec<u8>,
}

/// A connection handed to the pumps once login is over.
pub struct Parts {
    pub reader: ReadHalf,
    pub writer: WriteHalf,
    /// Bytes already read but not consumed: the peer may have sent its first
    /// packets of the next state together with its last login packet.
    pub buffer: FrameBuffer,
    pub compressed: bool,
}

impl Connection {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        stream.set_nodelay(true)?;
        let (reader, writer) = split(stream);

        Ok(Self { reader, writer, buffer: FrameBuffer::new(), compression: None, inflated: Vec::new(), outgoing: Vec::new() })
    }

    /// Switches to the compressed framing, or back when `threshold` is negative.
    pub fn set_compression(&mut self, threshold: i32) {
        self.compression = (threshold >= 0).then(Inflater::new);
    }

    pub async fn read_serverbound(&mut self, state: State) -> io::Result<Option<Serverbound<'_>>> {
        let Some(packet) = self.read_packet().await? else {
            return Ok(None);
        };
        let packet = Serverbound::decode(state, packet).map_err(invalid_data)?;

        Ok(Some(packet))
    }

    pub async fn read_clientbound(&mut self, state: State) -> io::Result<Option<Clientbound<'_>>> {
        let Some(packet) = self.read_packet().await? else {
            return Ok(None);
        };
        let packet = Clientbound::decode(state, packet).map_err(invalid_data)?;

        Ok(Some(packet))
    }

    /// Encodes a frame with `encode`, then sends it in this leg's framing.
    pub async fn send(&mut self, encode: impl FnOnce(&mut Vec<u8>) -> mc_protocol::encode::Result) -> io::Result<()> {
        self.outgoing.clear();
        encode(&mut self.outgoing).map_err(invalid_data)?;

        if self.compression.is_some() {
            add_zero_data_length(&mut self.outgoing)?;
        }

        self.writer.write_all(&self.outgoing).await
    }

    pub fn into_parts(self) -> Parts {
        let compressed = self.compression.is_some();

        Parts { reader: self.reader, writer: self.writer, buffer: self.buffer, compressed }
    }

    /// Reads the next packet, id then payload, or `None` once the peer closed the stream.
    async fn read_packet(&mut self) -> io::Result<Option<&[u8]>> {
        let Some(body) = self.read_frame().await? else {
            return Ok(None);
        };
        let body = self.buffer.slice(body);

        let Some(inflater) = &mut self.compression else {
            return Ok(Some(body));
        };
        let packet = inflater.packet(body, &mut self.inflated)?;

        Ok(Some(packet))
    }

    /// Reads the next frame and returns where its body is in the buffer.
    async fn read_frame(&mut self) -> io::Result<Option<Range<usize>>> {
        loop {
            if let Some(body) = self.buffer.take_frame()? {
                return Ok(Some(body));
            }

            if !self.buffer.fill(&mut self.reader).await? {
                return Ok(None);
            }
        }
    }
}
