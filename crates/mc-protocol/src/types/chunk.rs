use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::packets::PacketWriter;
use crate::types::{Nbt, VarInt};

#[derive(Debug)]
pub struct Heightmap {
    pub kind: u8,
    pub data: Vec<i64>,
}

impl<'p> PacketWriter<'p> for Heightmap {
    fn read(buf: &mut &'p [u8]) -> DecodeResult<Self> {
        let kind = u8::read(buf)?;
        let data = <Vec<i64>>::read(buf)?;
        Ok(Self { kind, data })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.kind.encode(out)?;
        self.data.encode(out)?;
        Ok(())
    }

    fn body_len(&self) -> usize {
        self.kind.body_len() + self.data.body_len()
    }
}

#[derive(Debug)]
pub struct BlockEntity<'p> {
    pub packed_xz: u8,
    pub y: i16,
    pub kind: i32,
    pub data: Nbt<'p>,
}

impl<'p> PacketWriter<'p> for BlockEntity<'p> {
    fn read(buf: &mut &'p [u8]) -> DecodeResult<Self> {
        let packed_xz = u8::read(buf)?;
        let y = i16::read(buf)?;
        let kind = VarInt::read(buf)?.0;
        let data = Nbt::read(buf)?;
        Ok(Self { packed_xz, y, kind, data })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.packed_xz.encode(out)?;
        self.y.encode(out)?;
        VarInt(self.kind).encode(out)?;
        self.data.encode(out)?;
        Ok(())
    }

    fn body_len(&self) -> usize {
        self.packed_xz.body_len() + self.y.body_len() + VarInt(self.kind).body_len() + self.data.body_len()
    }
}

#[derive(Debug)]
pub struct LightData<'p> {
    pub sky_mask: &'p [u8],
    pub block_mask: &'p [u8],
    pub empty_sky_mask: &'p [u8],
    pub empty_block_mask: &'p [u8],
    pub sky_light: Vec<&'p [u8]>,
    pub block_light: Vec<&'p [u8]>,
}

/// Sections per column on the wire: 24 world sections plus one below min
/// and one above max height.
pub const LIGHT_SECTIONS: usize = 26;
/// One section of full-bright light: half-byte per block, every nibble 0xF.
/// Shared by every chunk of every connection; never allocated per send.
static FULLBRIGHT_SECTION: [u8; 2048] = [0xFF; 2048];
/// 26 bits set: every section carries explicit light data.
static FULL_MASK: [u8; 4] = [0xFF, 0xFF, 0xFF, 0x03];
/// No section is all-zero.
static EMPTY_MASK: [u8; 4] = [0x00, 0x00, 0x00, 0x00];

impl LightData<'_> {
    pub fn fullbright() -> LightData<'static> {
        LightData {
            sky_mask: &FULL_MASK,
            block_mask: &FULL_MASK,
            empty_sky_mask: &EMPTY_MASK,
            empty_block_mask: &EMPTY_MASK,
            sky_light: vec![&FULLBRIGHT_SECTION; LIGHT_SECTIONS],
            block_light: vec![&FULLBRIGHT_SECTION; LIGHT_SECTIONS],
        }
    }
}

impl<'p> PacketWriter<'p> for LightData<'p> {
    fn read(buf: &mut &'p [u8]) -> DecodeResult<Self> {
        let sky_mask = <&[u8]>::read(buf)?;
        let block_mask = <&[u8]>::read(buf)?;
        let empty_sky_mask = <&[u8]>::read(buf)?;
        let empty_block_mask = <&[u8]>::read(buf)?;
        let sky_light = <Vec<&[u8]>>::read(buf)?;
        let block_light = <Vec<&[u8]>>::read(buf)?;
        Ok(Self { sky_mask, block_mask, empty_sky_mask, empty_block_mask, sky_light, block_light })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.sky_mask.encode(out)?;
        self.block_mask.encode(out)?;
        self.empty_sky_mask.encode(out)?;
        self.empty_block_mask.encode(out)?;
        self.sky_light.encode(out)?;
        self.block_light.encode(out)?;
        Ok(())
    }

    fn body_len(&self) -> usize {
        self.sky_mask.body_len()
            + self.block_mask.body_len()
            + self.empty_sky_mask.body_len()
            + self.empty_block_mask.body_len()
            + self.sky_light.body_len()
            + self.block_light.body_len()
    }
}
