use futures_lite::AsyncWriteExt;

use crate::decode::{take, take_array};
use crate::error::DecodeError;
use crate::packets::PacketWriter;

impl PacketWriter<'_> for u16 {
    fn read(input: &mut &[u8]) -> Result<Self, DecodeError> {
        Ok(u16::from_be_bytes(take_array(input)?))
    }

    async fn write<W: AsyncWriteExt + Unpin>(&self, w: &mut W) -> crate::encode::Result {
        let bytes: [u8; 2] = self.to_be_bytes();
        w.write_all(&bytes).await?;

        Ok(())
    }

    fn body_len(&self) -> usize {
        2
    }
}
impl PacketWriter<'_> for i32 {
    fn read(input: &mut &[u8]) -> Result<Self, DecodeError> {
        Ok(i32::from_be_bytes(take_array(input)?))
    }

    async fn write<W: AsyncWriteExt + Unpin>(&self, w: &mut W) -> crate::encode::Result {
        w.write_all(&self.to_be_bytes()).await?;
        Ok(())
    }

    fn body_len(&self) -> usize {
        4
    }
}
impl PacketWriter<'_> for i16 {
    fn read(input: &mut &[u8]) -> Result<Self, DecodeError> {
        Ok(i16::from_be_bytes(take_array(input)?))
    }

    async fn write<W: AsyncWriteExt + Unpin>(&self, w: &mut W) -> crate::encode::Result {
        w.write_all(&self.to_be_bytes()).await?;
        Ok(())
    }

    fn body_len(&self) -> usize {
        2
    }
}

impl PacketWriter<'_> for f32 {
    fn read(input: &mut &[u8]) -> Result<Self, DecodeError> {
        Ok(f32::from_be_bytes(take_array(input)?))
    }

    async fn write<W: AsyncWriteExt + Unpin>(&self, w: &mut W) -> crate::encode::Result {
        w.write_all(&self.to_be_bytes()).await?;
        Ok(())
    }

    fn body_len(&self) -> usize {
        4
    }
}

impl PacketWriter<'_> for f64 {
    fn read(input: &mut &[u8]) -> Result<Self, DecodeError> {
        Ok(f64::from_be_bytes(take_array(input)?))
    }

    async fn write<W: AsyncWriteExt + Unpin>(&self, w: &mut W) -> crate::encode::Result {
        w.write_all(&self.to_be_bytes()).await?;
        Ok(())
    }

    fn body_len(&self) -> usize {
        8
    }
}

impl PacketWriter<'_> for i64 {
    fn read(input: &mut &[u8]) -> Result<Self, DecodeError> {
        Ok(i64::from_be_bytes(take_array(input)?))
    }

    async fn write<W: AsyncWriteExt + Unpin>(&self, w: &mut W) -> crate::encode::Result {
        w.write_all(&self.to_be_bytes()).await?;
        Ok(())
    }

    fn body_len(&self) -> usize {
        8
    }
}

impl PacketWriter<'_> for bool {
    fn read(input: &mut &[u8]) -> Result<Self, DecodeError> {
        match take(input, 1)?[0] {
            0 => Ok(false),
            1 => Ok(true),
            b => Err(DecodeError::InvalidBool(b)),
        }
    }

    async fn write<W: AsyncWriteExt + Unpin>(&self, w: &mut W) -> crate::encode::Result {
        w.write_all(&[*self as u8]).await?;
        Ok(())
    }

    fn body_len(&self) -> usize {
        1
    }
}
