use std::borrow::Cow;

use mc_protocol_macros::IntoOwned;

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::packets::PacketWriter;
use crate::types::{Nbt, VarInt};

#[derive(Debug, IntoOwned)]
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

#[derive(Debug, IntoOwned)]
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

/// The masks are BitSets: bit `i` of the whole set is bit `i % 64` of
/// `mask[i / 64]`, one bit per section.
#[derive(Debug, IntoOwned)]
pub struct LightData<'p> {
    pub sky_mask: Vec<i64>,
    pub block_mask: Vec<i64>,
    pub empty_sky_mask: Vec<i64>,
    pub empty_block_mask: Vec<i64>,
    pub sky_light: Vec<Cow<'p, [u8]>>,
    pub block_light: Vec<Cow<'p, [u8]>>,
}

/// Sections per column on the wire: 24 world sections plus one below min
/// and one above max height.
pub const LIGHT_SECTIONS: usize = 26;
/// One section of full-bright light: half-byte per block, every nibble 0xF.
/// Shared by every chunk of every connection; never allocated per send.
static FULLBRIGHT_SECTION: [u8; 2048] = [0xFF; 2048];
/// 26 bits set: every section carries explicit light data.
const FULL_MASK: i64 = (1 << LIGHT_SECTIONS) - 1;

impl LightData<'_> {
    pub fn fullbright() -> LightData<'static> {
        let section: &'static [u8] = &FULLBRIGHT_SECTION;

        // An empty BitSet has no longs at all: no section is all-zero.
        LightData {
            sky_mask: vec![FULL_MASK],
            block_mask: vec![FULL_MASK],
            empty_sky_mask: Vec::new(),
            empty_block_mask: Vec::new(),
            sky_light: vec![Cow::Borrowed(section); LIGHT_SECTIONS],
            block_light: vec![Cow::Borrowed(section); LIGHT_SECTIONS],
        }
    }
}

impl<'p> PacketWriter<'p> for LightData<'p> {
    fn read(buf: &mut &'p [u8]) -> DecodeResult<Self> {
        let sky_mask = Vec::read(buf)?;
        let block_mask = Vec::read(buf)?;
        let empty_sky_mask = Vec::read(buf)?;
        let empty_block_mask = Vec::read(buf)?;
        let sky_light = Vec::read(buf)?;
        let block_light = Vec::read(buf)?;

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
