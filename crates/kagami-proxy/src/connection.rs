//! One socket during handshake, status and login, where the proxy reads and
//! sends whole packets.

use std::io;

use futures_lite::AsyncWriteExt;
use glommio::net::TcpStream;
use mc_protocol::State;
use mc_protocol::packets::{Clientbound, Serverbound};

use crate::frame::FrameBuffer;
use crate::invalid_data;
use crate::split::{ReadHalf, WriteHalf, split};

pub struct Connection {
    stream: TcpStream,
    buffer: FrameBuffer,
    /// Reused for encoding outgoing packets.
    outgoing: Vec<u8>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        stream.set_nodelay(true)?;
        Ok(Self { stream, buffer: FrameBuffer::new(), outgoing: Vec::new() })
    }

    /// Reads the next frame body, or `None` once the peer closed the stream.
    pub async fn read_frame(&mut self) -> io::Result<Option<&[u8]>> {
        loop {
            if let Some(body) = self.buffer.take_frame()? {
                return Ok(Some(self.buffer.slice(body)));
            }
            if !self.buffer.fill(&mut self.stream).await? {
                return Ok(None);
            }
        }
    }

    pub async fn read_serverbound(&mut self, state: State) -> io::Result<Option<Serverbound<'_>>> {
        let Some(body) = self.read_frame().await? else {
            return Ok(None);
        };
        let packet = Serverbound::decode(state, body).map_err(invalid_data)?;
        Ok(Some(packet))
    }

    pub async fn read_clientbound(&mut self, state: State) -> io::Result<Option<Clientbound<'_>>> {
        let Some(body) = self.read_frame().await? else {
            return Ok(None);
        };
        let packet = Clientbound::decode(state, body).map_err(invalid_data)?;
        Ok(Some(packet))
    }

    /// Encodes a frame with `encode`, then sends it.
    pub async fn send(&mut self, encode: impl FnOnce(&mut Vec<u8>) -> mc_protocol::encode::Result) -> io::Result<()> {
        self.outgoing.clear();
        encode(&mut self.outgoing).map_err(invalid_data)?;
        self.stream.write_all(&self.outgoing).await
    }

    /// Hands the socket to the pumps, with the bytes already read but not
    /// consumed: the peer may have sent its first packets of the next state
    /// together with its last login packet.
    pub fn into_parts(self) -> (ReadHalf, WriteHalf, FrameBuffer) {
        let (reader, writer) = split(self.stream);
        (reader, writer, self.buffer)
    }
}
