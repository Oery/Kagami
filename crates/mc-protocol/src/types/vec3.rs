use mc_protocol_macros::IntoOwned;

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::packets::PacketWriter;

/// Three doubles: a position, a size or a velocity.
#[derive(Clone, Copy, Debug, Default, PartialEq, IntoOwned)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PacketWriter<'_> for Vec3 {
    fn read(input: &mut &[u8]) -> DecodeResult<Self> {
        let x = f64::read(input)?;
        let y = f64::read(input)?;
        let z = f64::read(input)?;

        Ok(Self { x, y, z })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.x.encode(out)?;
        self.y.encode(out)?;

        self.z.encode(out)
    }

    fn body_len(&self) -> usize {
        24
    }
}
