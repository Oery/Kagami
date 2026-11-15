//! Bounded chat inputs. Signatures are consumed without claiming verification.
use crate::State;
use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use futures_lite::{AsyncWrite, AsyncWriteExt};
use mc_protocol_macros::{Payload, packet};

use crate::decode::{read_length, take};
use crate::error::{DecodeError, EncodeError};
use crate::packets::{PacketWriter, Payload};
use crate::types::VarInt;

#[derive(Debug)]
pub struct ChatText<'p>(pub &'p str);

impl<'p> PacketWriter<'p> for ChatText<'p> {
    fn read(input: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let length = read_length(input, 256 * 3)?;
        let text = str::from_utf8(take(input, length)?)?;
        if text.encode_utf16().count() > 256 {
            return Err(DecodeError::InvalidLength);
        }
        Ok(Self(text))
    }
    async fn write<W: AsyncWrite + Unpin>(&self, w: &mut W) -> Result<(), EncodeError> {
        if self.0.encode_utf16().count() > 256 {
            return Err(EncodeError::InvalidLength);
        }
        self.0.write(w).await
    }
    fn body_len(&self) -> usize {
        self.0.body_len()
    }
}

#[derive(Debug)]
pub struct Signature<'p>(pub &'p [u8]);

impl<'p> PacketWriter<'p> for Signature<'p> {
    fn read(input: &mut &'p [u8]) -> Result<Self, DecodeError> {
        Ok(Self(take(input, 256)?))
    }
    async fn write<W: AsyncWrite + Unpin>(&self, w: &mut W) -> Result<(), EncodeError> {
        if self.0.len() != 256 {
            return Err(EncodeError::InvalidLength);
        }
        w.write_all(self.0).await?;
        Ok(())
    }
    fn body_len(&self) -> usize {
        256
    }
}

#[derive(Debug)]
pub struct LastSeen {
    pub count: i32,
    pub acknowledged: [u8; 3],
    pub checksum: u8,
}

impl<'p> PacketWriter<'p> for LastSeen {
    fn read(input: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let count = VarInt::read(input)?.0;
        if count < 0 {
            return Err(DecodeError::InvalidLength);
        }
        let acknowledged = crate::decode::take_array(input)?;
        let checksum = u8::read(input)?;
        Ok(Self { count, acknowledged, checksum })
    }
    async fn write<W: AsyncWrite + Unpin>(&self, w: &mut W) -> Result<(), EncodeError> {
        VarInt(self.count).write(w).await?;
        w.write_all(&self.acknowledged).await?;
        self.checksum.write(w).await
    }
    fn body_len(&self) -> usize {
        VarInt(self.count).body_len() + 4
    }
}

#[derive(Debug)]
pub struct ArgumentSignatures<'p>(pub Vec<(&'p str, Signature<'p>)>);

impl<'p> PacketWriter<'p> for ArgumentSignatures<'p> {
    fn read(input: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let length = read_length(input, 8)?;
        let mut signatures = Vec::new();
        for _ in 0..length {
            let bytes = read_length(input, 16 * 3)?;
            let name = str::from_utf8(take(input, bytes)?)?;
            if name.encode_utf16().count() > 16 {
                return Err(DecodeError::InvalidLength);
            }
            signatures.push((name, Signature::read(input)?));
        }
        Ok(Self(signatures))
    }
    async fn write<W: AsyncWrite + Unpin>(&self, w: &mut W) -> Result<(), EncodeError> {
        if self.0.len() > 8 {
            return Err(EncodeError::InvalidLength);
        }
        VarInt(self.0.len() as i32).write(w).await?;
        for (name, signature) in &self.0 {
            name.write(w).await?;
            signature.write(w).await?;
        }
        Ok(())
    }
    fn body_len(&self) -> usize {
        VarInt(self.0.len() as i32).body_len()
            + self.0.iter().map(|(name, signature)| name.body_len() + signature.body_len()).sum::<usize>()
    }
}

#[packet(Play, 0x09, Server)]
pub struct ChatMessage<'p> {
    pub message: ChatText<'p>,
    pub timestamp: i64,
    pub salt: i64,
    pub signature: Option<Signature<'p>>,
    pub seen: LastSeen,
}

#[packet(Play, 0x08, Server)]
pub struct SignedChatCommand<'p> {
    pub command: &'p str,
    pub timestamp: i64,
    pub salt: i64,
    pub signatures: ArgumentSignatures<'p>,
    pub seen: LastSeen,
}

#[cfg(test)]
#[path = "../../tests/tests_packets_chat.rs"]
mod tests;
