use crate::error::DecodeError;
use crate::packets::PacketWriter;

/// A 64-bit integer in at most ten 7-bit groups, least significant first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VarLong(pub i64);

impl<'p> PacketWriter<'p> for VarLong {
    fn read(input: &mut &'p [u8]) -> Result<Self, DecodeError> {
        let mut value = 0u64;

        for shift in (0..70).step_by(7) {
            let byte = u8::read(input)?;
            if shift == 63 && byte & 0xfe != 0 {
                return Err(DecodeError::InvalidVarInt);
            }

            value |= u64::from(byte & 0x7f) << shift;

            if byte & 0x80 == 0 {
                return Ok(VarLong(value as i64));
            }
        }

        Err(DecodeError::InvalidVarInt)
    }

    fn encode(&self, out: &mut Vec<u8>) -> crate::encode::Result {
        let mut value = self.0 as u64;

        while (value & !0x7F) != 0 {
            out.push(((value & 0x7F) | 0x80) as u8);
            value >>= 7;
        }

        out.push(value as u8);

        Ok(())
    }

    fn body_len(&self) -> usize {
        let mut value = self.0 as u64;
        let mut len = 1;

        while (value & !0x7F) != 0 {
            len += 1;
            value >>= 7;
        }

        len
    }
}

#[cfg(test)]
#[path = "../../tests/tests_types_varlong.rs"]
mod tests;
