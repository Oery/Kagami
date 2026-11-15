use futures_lite::AsyncWriteExt;

use crate::decode::take;
use crate::error::DecodeError;
use crate::packets::PacketWriter;

#[derive(Debug)]
pub struct VarInt(pub i32);

impl<'p> PacketWriter<'p> for VarInt {
    fn read(input: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let mut value = 0u32;

        for shift in (0..35).step_by(7) {
            let byte = u8::read(input)?;
            if shift == 28 && byte & 0xf0 != 0 {
                return Err(DecodeError::InvalidVarInt);
            }

            value |= ((byte & 0x7f) as u32) << shift;
            if byte & 0x80 == 0 {
                return Ok(VarInt(value as i32));
            }
        }

        Err(DecodeError::InvalidVarInt)
    }

    async fn write<W: AsyncWriteExt + Unpin>(&self, w: &mut W) -> crate::encode::Result {
        let mut value = self.0 as u32;

        while (value & !0x7F) != 0 {
            u8::write(&(((value & 0x7F) | 0x80) as u8), w).await?;
            value >>= 7;
        }

        u8::write(&(value as u8), w).await?;
        Ok(())
    }

    fn body_len(&self) -> usize {
        let mut val = self.0 as u32;
        let mut len = 1;

        while (val & !0x7F) != 0 {
            len += 1;
            val >>= 7;
        }

        len
    }
}

impl<'p> PacketWriter<'p> for u8 {
    fn read(input: &mut &'p [u8]) -> Result<u8, DecodeError> {
        let byte = take(input, 1)?;
        Ok(byte[0])
    }

    async fn write<W: AsyncWriteExt + Unpin>(&self, w: &mut W) -> crate::encode::Result {
        w.write_all(&[*self]).await?;
        Ok(())
    }

    fn body_len(&self) -> usize {
        1
    }
}
