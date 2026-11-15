use std::borrow::Cow;

use futures_lite::{AsyncWrite, AsyncWriteExt};
use mc_protocol_macros::IntoOwned;

use crate::decode::Result as DecodeResult;
use crate::direction::Direction;
use crate::encode::Result as EncodeResult;
use crate::error::EncodeError;
use crate::state::State;
use crate::types::VarInt;

pub mod client;
mod clientbound;
pub mod server;
mod serverbound;

pub use clientbound::Clientbound;
pub use serverbound::Serverbound;

/// A field codec. Decoding borrows from the input; encoding appends to `out`.
pub trait PacketWriter<'p>: Sized {
    fn read(buf: &mut &'p [u8]) -> DecodeResult<Self>;

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult;

    fn body_len(&self) -> usize;
}

// Deliberately supports local, non-Send transports (Glommio).
#[allow(async_fn_in_trait)]
pub trait Payload<'p>: Sized {
    const DIRECTION: Direction;
    const ID: VarInt;
    const STATE: State;

    fn payload_len(&self) -> EncodeResult<usize>;

    fn decode_payload(buf: &mut &'p [u8]) -> DecodeResult<Self>;

    fn encode_payload(&self, out: &mut Vec<u8>) -> EncodeResult;

    /// Appends the uncompressed frame: length, packet id, then the payload.
    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        let length = Self::ID.body_len() + self.payload_len()?;
        let length = i32::try_from(length).map_err(|_| EncodeError::InvalidLength)?;
        VarInt(length).encode(out)?;
        Self::ID.encode(out)?;

        self.encode_payload(out)
    }

    async fn write_packet<W>(&self, w: &mut W) -> EncodeResult
    where
        W: AsyncWrite + Unpin,
    {
        let mut frame = Vec::new();
        self.encode(&mut frame)?;
        w.write_all(&frame).await?;

        Ok(())
    }
}

/// Names one packet type independently of the buffer it is decoded from.
///
/// Implemented on the `'static` form of every packet listed in
/// [`Serverbound`] or [`Clientbound`]: `ChatCommand` names the kind, while
/// [`pick`](Self::pick) hands out a `ChatCommand<'p>` borrowing the buffer.
pub trait PacketKind: 'static {
    const DIRECTION: Direction;
    const STATE: State;
    const ID: u8;

    /// This packet, borrowing a buffer that lives for `'p`.
    type Packet<'p>: Payload<'p>;

    /// The packet enum of [`DIRECTION`](Self::DIRECTION).
    type Enum<'p>;

    /// Returns the packet if `packet` holds this kind.
    fn pick<'a, 'p>(packet: &'a mut Self::Enum<'p>) -> Option<&'a mut Self::Packet<'p>>;
}

/// A packet with no definition for its state and id. Proxied as is.
#[derive(Debug, IntoOwned)]
pub struct RawPacket<'p> {
    pub id: i32,
    pub payload: Cow<'p, [u8]>,
}

impl RawPacket<'_> {
    /// Appends the uncompressed frame: length, packet id, then the payload.
    pub fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        let id = VarInt(self.id);
        let length = id.body_len() + self.payload.len();
        let length = i32::try_from(length).map_err(|_| EncodeError::InvalidLength)?;
        VarInt(length).encode(out)?;
        id.encode(out)?;
        out.extend_from_slice(&self.payload);

        Ok(())
    }
}

#[cfg(test)]
#[path = "../../tests/tests_packets.rs"]
mod tests;
