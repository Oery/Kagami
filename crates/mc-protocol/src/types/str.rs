use futures_lite::AsyncWriteExt;

use crate::decode::{MAX_COLLECTION_LEN, read_length, take};
use crate::error::DecodeError;
use crate::packets::PacketWriter;
use crate::types::VarInt;

impl<'p> PacketWriter<'p> for &'p str {
    fn read(input: &mut &'p [u8]) -> Result<&'p str, DecodeError> {
        let len = read_length(input, 32_767 * 3)?;
        let bytes = take(input, len)?;
        let str = str::from_utf8(bytes)?;
        if str.encode_utf16().count() > 32_767 {
            return Err(DecodeError::InvalidLength);
        }
        Ok(str)
    }

    async fn write<T: AsyncWriteExt + Unpin>(&self, w: &mut T) -> crate::encode::Result {
        VarInt(self.len() as i32).write(w).await?;
        w.write_all(self.as_bytes()).await?;

        Ok(())
    }

    fn body_len(&self) -> usize {
        self.len() + VarInt(self.len() as i32).body_len()
    }
}

impl<'p, T: PacketWriter<'p>> PacketWriter<'p> for Vec<T> {
    fn read(buf: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let length = read_length(buf, MAX_COLLECTION_LEN)?;
        // Do not reserve from an untrusted count. Successful elements alone
        // grow storage; truncated input fails before a large reservation.
        let mut elements = Vec::new();

        for _ in 0..length {
            let element = T::read(buf)?;
            elements.push(element);
        }

        Ok(elements)
    }

    async fn write<W: AsyncWriteExt + Unpin>(&self, w: &mut W) -> crate::encode::Result {
        VarInt(self.len() as i32).write(w).await?;

        for e in self {
            e.write(w).await?;
        }

        Ok(())
    }

    fn body_len(&self) -> usize {
        let mut length = 0;

        length += VarInt(self.len() as i32).body_len();
        length += self.iter().fold(0, |len, e| len + e.body_len());

        length
    }
}

impl<'p, T: PacketWriter<'p>> PacketWriter<'p> for Option<T> {
    fn read(buf: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let is_some = bool::read(buf)?;

        let data = match is_some {
            true => Some(T::read(buf)?),
            false => None,
        };

        Ok(data)
    }

    async fn write<W: AsyncWriteExt + Unpin>(&self, w: &mut W) -> crate::encode::Result {
        self.is_some().write(w).await?;

        if let Some(data) = self {
            data.write(w).await?;
        }

        Ok(())
    }

    fn body_len(&self) -> usize {
        let payload = match self {
            Some(data) => data.body_len(),
            None => 0,
        };

        payload + 1
    }
}

#[cfg(test)]
#[path = "../tests/tests_types_str.rs"]
mod tests;
