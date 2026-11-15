//! Protocol 777 presentation team creation and removal.
use std::borrow::Cow;

use mc_protocol_macros::{IntoOwned, packet};

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::error::DecodeError;
use crate::packets::PacketWriter;
use crate::types::{Nbt, VarInt};

#[derive(Debug, IntoOwned)]
pub struct TeamProperties<'p> {
    pub display_name: Nbt<'p>,
    pub prefix: Nbt<'p>,
    pub suffix: Nbt<'p>,
    pub name_tag_visibility: i32,
    pub collision_rule: i32,
    /// 26.3 encodes a presence boolean before the color ID (0..=15).
    pub color: Option<VarInt>,
    pub friendly_flags: u8,
    pub entities: Vec<Cow<'p, str>>,
}

#[derive(Debug, IntoOwned)]
pub enum TeamAction<'p> {
    Create(TeamProperties<'p>),
    Remove,
}

impl<'p> PacketWriter<'p> for TeamAction<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        match u8::read(input)? {
            0 => Ok(Self::Create(TeamProperties {
                display_name: Nbt::read(input)?,
                prefix: Nbt::read(input)?,
                suffix: Nbt::read(input)?,
                name_tag_visibility: VarInt::read(input)?.0,
                collision_rule: VarInt::read(input)?.0,
                color: Option::read(input)?,
                friendly_flags: u8::read(input)?,
                entities: Vec::read(input)?,
            })),
            1 => Ok(Self::Remove),
            _ => Err(DecodeError::EnumOutOfRange),
        }
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        let Self::Create(properties) = self else {
            return 1u8.encode(out);
        };
        0u8.encode(out)?;
        properties.display_name.encode(out)?;
        properties.prefix.encode(out)?;
        properties.suffix.encode(out)?;
        VarInt(properties.name_tag_visibility).encode(out)?;
        VarInt(properties.collision_rule).encode(out)?;
        properties.color.encode(out)?;
        properties.friendly_flags.encode(out)?;
        properties.entities.encode(out)
    }

    fn body_len(&self) -> usize {
        1 + match self {
            Self::Remove => 0,
            Self::Create(properties) => {
                properties.display_name.body_len()
                    + properties.prefix.body_len()
                    + properties.suffix.body_len()
                    + VarInt(properties.name_tag_visibility).body_len()
                    + VarInt(properties.collision_rule).body_len()
                    + properties.color.body_len()
                    + 1
                    + properties.entities.body_len()
            }
        }
    }
}

#[packet(Play, 0x6F, Clientbound)]
pub struct SetPlayerTeam<'p> {
    pub name: Cow<'p, str>,
    pub action: TeamAction<'p>,
}

#[cfg(test)]
#[path = "../../../tests/tests_packets_team.rs"]
mod tests;
