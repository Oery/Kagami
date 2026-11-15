use futures_lite::AsyncWriteExt;

use crate::decode::{MAX_FRAME_LEN, read_length, take};
use crate::error::DecodeError;
use crate::packets::PacketWriter;
use crate::types::VarInt;

impl<'p> PacketWriter<'p> for &'p [u8] {
    fn read(input: &mut &'p [u8]) -> Result<&'p [u8], DecodeError> {
        let len = read_length(input, MAX_FRAME_LEN)?;
        let bytes = take(input, len)?;
        Ok(bytes)
    }

    async fn write<T: AsyncWriteExt + Unpin>(&self, w: &mut T) -> crate::encode::Result {
        VarInt(self.len() as i32).write(w).await?;
        w.write_all(self).await?;

        Ok(())
    }

    fn body_len(&self) -> usize {
        self.len() + VarInt(self.len() as i32).body_len()
    }
}

#[derive(Debug)]
pub struct Remainder<'p>(pub &'p [u8]);

impl<'p> PacketWriter<'p> for Remainder<'p> {
    fn read(input: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let rest = take(input, input.len())?;
        Ok(Remainder(rest))
    }

    async fn write<T: AsyncWriteExt + Unpin>(&self, w: &mut T) -> crate::encode::Result {
        w.write_all(self.0).await?;

        Ok(())
    }

    fn body_len(&self) -> usize {
        self.0.len()
    }
}

impl<'p> PacketWriter<'p> for String {
    fn read(input: &mut &'p [u8]) -> Result<Self, DecodeError> {
        Ok(<&str>::read(input)?.to_owned())
    }

    async fn write<T: AsyncWriteExt + Unpin>(&self, w: &mut T) -> crate::encode::Result {
        self.as_str().write(w).await?;

        Ok(())
    }

    fn body_len(&self) -> usize {
        self.as_str().body_len()
    }
}
