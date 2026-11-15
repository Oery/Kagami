use mc_protocol_macros::IntoOwned;

use crate::error::DecodeError;
use crate::packets::PacketWriter;

#[derive(Debug, IntoOwned)]
pub struct SkinParts(pub u8);

impl PacketWriter<'_> for SkinParts {
    fn read(buf: &mut &[u8]) -> Result<Self, DecodeError> {
        Ok(SkinParts(u8::read(buf)?))
    }

    fn encode(&self, out: &mut Vec<u8>) -> crate::encode::Result {
        self.0.encode(out)?;
        Ok(())
    }

    fn body_len(&self) -> usize {
        1
    }
}

impl SkinParts {
    pub fn cape(self) -> bool {
        self.0 & 0x01 != 0
    }

    pub fn jacket(self) -> bool {
        self.0 & 0x02 != 0
    }

    pub fn left_sleeve(self) -> bool {
        self.0 & 0x04 != 0
    }

    pub fn right_sleeve(self) -> bool {
        self.0 & 0x08 != 0
    }

    pub fn left_pants(self) -> bool {
        self.0 & 0x10 != 0
    }

    pub fn right_pants(self) -> bool {
        self.0 & 0x20 != 0
    }

    pub fn hat(self) -> bool {
        self.0 & 0x40 != 0
    }
}
