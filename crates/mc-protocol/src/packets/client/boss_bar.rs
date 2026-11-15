//! Protocol 777 boss events (Minecraft Wiki, cross-checked with the 26.3 client).
use mc_protocol_macros::{IntoOwned, packet};
use uuid::Uuid;

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::error::DecodeError;
use crate::packets::PacketWriter;
use crate::types::{Nbt, VarInt};

macro_rules! ordinal {
    ($name:ident { $($variant:ident = $value:literal),* $(,)? }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, IntoOwned)]
        pub enum $name { $($variant = $value),* }
        impl PacketWriter<'_> for $name {
            fn read(input: &mut &[u8]) -> DecodeResult<Self> {
                match VarInt::read(input)?.0 {
                    $($value => Ok(Self::$variant),)*
                    _ => Err(DecodeError::EnumOutOfRange),
                }
            }
            fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
                VarInt(*self as i32).encode(out)
            }
            fn body_len(&self) -> usize { 1 }
        }
    };
}
ordinal!(BossBarColor { Pink = 0, Blue = 1, Red = 2, Green = 3, Yellow = 4, Purple = 5, White = 6 });
ordinal!(BossBarOverlay { Continuous = 0, Notched6 = 1, Notched10 = 2, Notched12 = 3, Notched20 = 4 });

#[derive(Debug, IntoOwned)]
pub enum BossBarAction<'p> {
    Add { title: Nbt<'p>, progress: f32, color: BossBarColor, overlay: BossBarOverlay, flags: u8 },
    Remove,
    Progress(f32),
    Title(Nbt<'p>),
    Style { color: BossBarColor, overlay: BossBarOverlay },
    Flags(u8),
}

impl<'p> PacketWriter<'p> for BossBarAction<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        Ok(match VarInt::read(input)?.0 {
            0 => Self::Add {
                title: Nbt::read(input)?,
                progress: f32::read(input)?,
                color: BossBarColor::read(input)?,
                overlay: BossBarOverlay::read(input)?,
                flags: u8::read(input)?,
            },
            1 => Self::Remove,
            2 => Self::Progress(f32::read(input)?),
            3 => Self::Title(Nbt::read(input)?),
            4 => Self::Style { color: BossBarColor::read(input)?, overlay: BossBarOverlay::read(input)? },
            5 => Self::Flags(u8::read(input)?),
            _ => return Err(DecodeError::EnumOutOfRange),
        })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        match self {
            Self::Add { title, progress, color, overlay, flags } => {
                VarInt(0).encode(out)?;
                title.encode(out)?;
                progress.encode(out)?;
                color.encode(out)?;
                overlay.encode(out)?;
                flags.encode(out)
            }
            Self::Remove => VarInt(1).encode(out),
            Self::Progress(value) => {
                VarInt(2).encode(out)?;
                value.encode(out)
            }
            Self::Title(value) => {
                VarInt(3).encode(out)?;
                value.encode(out)
            }
            Self::Style { color, overlay } => {
                VarInt(4).encode(out)?;
                color.encode(out)?;
                overlay.encode(out)
            }
            Self::Flags(value) => {
                VarInt(5).encode(out)?;
                value.encode(out)
            }
        }
    }

    fn body_len(&self) -> usize {
        1 + match self {
            Self::Add { title, .. } => title.body_len() + 7,
            Self::Remove => 0,
            Self::Progress(_) => 4,
            Self::Title(title) => title.body_len(),
            Self::Style { .. } => 2,
            Self::Flags(_) => 1,
        }
    }
}

#[packet(Play, 0x09, Clientbound)]
pub struct BossBar<'p> {
    pub id: Uuid,
    pub action: BossBarAction<'p>,
}

#[cfg(test)]
#[path = "../../../tests/tests_packets_boss_bar.rs"]
mod tests;
