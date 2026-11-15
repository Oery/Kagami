//! Protocol 777 scoreboard objectives, display slots and score entries.
use mc_protocol_macros::packet;

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::error::DecodeError;
use crate::packets::PacketWriter;
use crate::types::{Nbt, VarInt};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScoreRender {
    #[default]
    Integer,
    Hearts,
}

impl PacketWriter<'_> for ScoreRender {
    fn read(input: &mut &[u8]) -> DecodeResult<Self> {
        match VarInt::read(input)?.0 {
            0 => Ok(Self::Integer),
            1 => Ok(Self::Hearts),
            _ => Err(DecodeError::EnumOutOfRange),
        }
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        VarInt(*self as i32).encode(out)
    }

    fn body_len(&self) -> usize {
        1
    }
}

/// Styled carries a style compound; Fixed carries a text component.
#[derive(Clone, Debug)]
pub enum ScoreNumberFormat<'p> {
    Blank,
    Styled(Nbt<'p>),
    Fixed(Nbt<'p>),
}

impl<'p> PacketWriter<'p> for ScoreNumberFormat<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        match VarInt::read(input)?.0 {
            0 => Ok(Self::Blank),
            1 => Ok(Self::Styled(Nbt::read(input)?)),
            2 => Ok(Self::Fixed(Nbt::read(input)?)),
            _ => Err(DecodeError::EnumOutOfRange),
        }
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        match self {
            Self::Blank => VarInt(0).encode(out),
            Self::Styled(style) => {
                VarInt(1).encode(out)?;
                style.encode(out)
            }
            Self::Fixed(text) => {
                VarInt(2).encode(out)?;
                text.encode(out)
            }
        }
    }

    fn body_len(&self) -> usize {
        1 + match self {
            Self::Blank => 0,
            Self::Styled(value) | Self::Fixed(value) => value.body_len(),
        }
    }
}

#[derive(Debug)]
pub struct ObjectiveProperties<'p> {
    pub title: Nbt<'p>,
    pub render: ScoreRender,
    pub number_format: Option<ScoreNumberFormat<'p>>,
}

#[derive(Debug)]
pub enum ObjectiveAction<'p> {
    Create(ObjectiveProperties<'p>),
    Remove,
    Update(ObjectiveProperties<'p>),
}

impl<'p> PacketWriter<'p> for ObjectiveAction<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let mode = u8::read(input)?;
        if mode == 1 {
            return Ok(Self::Remove);
        }
        if mode != 0 && mode != 2 {
            return Err(DecodeError::EnumOutOfRange);
        }
        let properties =
            ObjectiveProperties { title: Nbt::read(input)?, render: ScoreRender::read(input)?, number_format: Option::read(input)? };
        Ok(if mode == 0 {
            Self::Create(properties)
        } else {
            Self::Update(properties)
        })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        let (mode, properties) = match self {
            Self::Create(properties) => (0u8, properties),
            Self::Update(properties) => (2u8, properties),
            Self::Remove => return 1u8.encode(out),
        };
        mode.encode(out)?;
        properties.title.encode(out)?;
        properties.render.encode(out)?;
        properties.number_format.encode(out)
    }

    fn body_len(&self) -> usize {
        1 + match self {
            Self::Remove => 0,
            Self::Create(properties) | Self::Update(properties) => {
                properties.title.body_len() + properties.render.body_len() + properties.number_format.body_len()
            }
        }
    }
}

#[packet(Play, 0x6C, Clientbound)]
pub struct SetObjective<'p> {
    pub name: &'p str,
    pub action: ObjectiveAction<'p>,
}

/// Slots: 0 = Tab list, 1 = sidebar, 2 = below name; 3..=18 are team sidebars.
/// An empty objective name clears the slot.
#[packet(Play, 0x64, Clientbound)]
pub struct SetDisplayObjective<'p> {
    #[format = "varint"]
    pub slot: i32,
    pub name: &'p str,
}

#[packet(Play, 0x70, Clientbound)]
pub struct SetScore<'p> {
    pub holder: &'p str,
    pub objective: &'p str,
    #[format = "varint"]
    pub value: i32,
    pub display: Option<Nbt<'p>>,
    pub number_format: Option<ScoreNumberFormat<'p>>,
}

/// `None` removes this holder's scores from every objective.
#[packet(Play, 0x50, Clientbound)]
pub struct ResetScore<'p> {
    pub holder: &'p str,
    pub objective: Option<&'p str>,
}

#[cfg(test)]
#[path = "../../tests/tests_packets_scoreboard.rs"]
mod tests;
