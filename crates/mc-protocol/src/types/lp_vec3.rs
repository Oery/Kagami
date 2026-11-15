//! Protocol 777's quantized vector, checked against the official LpVec3 codec.
use super::VarInt;
use crate::{decode::Result as DecodeResult, encode::Result as EncodeResult, packets::PacketWriter};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LpVec3(pub [f64; 3]);

impl LpVec3 {
    fn packed(self) -> Option<(u64, u64)> {
        let values = self.0.map(|v| {
            if v.is_nan() {
                0.0
            } else {
                v.clamp(-17_179_869_183.0, 17_179_869_183.0)
            }
        });
        let max = values.into_iter().map(f64::abs).fold(0.0, f64::max);
        if max < 1.0 / 32766.0 {
            return None;
        }
        let scale = max.ceil() as u64;
        let mut packed = (scale & 3) | if scale > 3 { 4 } else { 0 };
        for (value, offset) in values.into_iter().zip([3, 18, 33]) {
            let component = ((value / scale as f64 * 0.5 + 0.5) * 32766.0 + 0.5).floor() as u64;
            packed |= component << offset;
        }
        Some((packed, scale))
    }
}

impl PacketWriter<'_> for LpVec3 {
    fn read(input: &mut &[u8]) -> DecodeResult<Self> {
        let first = u8::read(input)?;
        if first == 0 {
            return Ok(Self::default());
        }
        let packed = u64::from(first) | (u64::from(u8::read(input)?) << 8) | ((i32::read(input)? as u32 as u64) << 16);
        let mut scale = u64::from(first & 3);
        if first & 4 != 0 {
            scale |= (VarInt::read(input)?.0 as u32 as u64) << 2;
        }
        Ok(Self([3, 18, 33].map(|offset| {
            (((packed >> offset) & 32767).min(32766) as f64 * 2.0 / 32766.0 - 1.0) * scale as f64
        })))
    }
    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        let Some((packed, scale)) = self.packed() else {
            return 0u8.encode(out);
        };
        out.extend_from_slice(&[packed as u8, (packed >> 8) as u8]);
        ((packed >> 16) as i32).encode(out)?;
        if scale > 3 {
            VarInt((scale >> 2) as i32).encode(out)?;
        }
        Ok(())
    }
    fn body_len(&self) -> usize {
        self.packed().map_or(1, |(_, scale)| {
            6 + if scale > 3 {
                VarInt((scale >> 2) as i32).body_len()
            } else {
                0
            }
        })
    }
}

#[cfg(test)]
#[path = "../../tests/tests_types_lp_vec3.rs"]
mod tests;
