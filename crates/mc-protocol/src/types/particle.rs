//! Particles with their options, as in entity metadata and particle packets.
use mc_protocol_macros::IntoOwned;

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::error::DecodeError;
use crate::packets::PacketWriter;
use crate::types::{ItemStackTemplate, VarInt, Vec3};

/// `minecraft:particle_type` IDs of the particles that carry options, from
/// 26.3 registration order. Every other particle type carries none.
pub mod particle_type {
    pub const BLOCK: i32 = 1;
    pub const BLOCK_MARKER: i32 = 2;
    pub const GEYSER: i32 = 7;
    pub const GEYSER_BASE: i32 = 8;
    pub const GEYSER_POOF: i32 = 9;
    pub const GEYSER_PLUME: i32 = 10;
    pub const DRAGON_BREATH: i32 = 15;
    pub const DUST: i32 = 21;
    pub const DUST_COLOR_TRANSITION: i32 = 22;
    pub const EFFECT: i32 = 23;
    pub const ENTITY_EFFECT: i32 = 28;
    pub const FALLING_DUST: i32 = 36;
    pub const TINTED_LEAVES: i32 = 46;
    pub const SCULK_CHARGE: i32 = 48;
    pub const FLASH: i32 = 52;
    pub const INSTANT_EFFECT: i32 = 56;
    pub const ITEM: i32 = 57;
    pub const VIBRATION: i32 = 58;
    pub const TRAIL: i32 = 59;
    pub const SHRIEK: i32 = 115;
    pub const DUST_PILLAR: i32 = 121;
    pub const BLOCK_CRUMBLE: i32 = 125;
}

/// A particle type and its options, sent as the type's ID then the options
/// that type defines. Encoding trusts `options` to match `kind`.
#[derive(Clone, Debug, PartialEq, IntoOwned)]
pub struct Particle<'p> {
    /// A `minecraft:particle_type` ID, see [`particle_type`].
    pub kind: i32,
    pub options: ParticleOptions<'p>,
}

impl Particle<'_> {
    /// A particle type that carries no options.
    pub fn simple(kind: i32) -> Self {
        Self { kind, options: ParticleOptions::None }
    }
}

/// The options of each particle type. Colors are packed into an `i32`.
#[derive(Clone, Debug, PartialEq, IntoOwned)]
pub enum ParticleOptions<'p> {
    None,
    /// Block, block marker, falling dust, dust pillar and block crumble.
    Block {
        state: i32,
    },
    /// Geyser and geyser plume.
    Geyser {
        water_blocks: i32,
    },
    /// Geyser base and geyser poof.
    GeyserBase {
        water_blocks: i32,
        burst_impulse_base: f32,
    },
    /// Dragon breath.
    Power {
        power: f32,
    },
    Dust {
        color: i32,
        scale: f32,
    },
    DustColorTransition {
        from_color: i32,
        to_color: i32,
        scale: f32,
    },
    /// Effect and instant effect.
    Spell {
        color: i32,
        power: f32,
    },
    /// Entity effect, tinted leaves and flash.
    Color {
        color: i32,
    },
    SculkCharge {
        roll: f32,
    },
    Item(ItemStackTemplate<'p>),
    Vibration {
        destination: PositionSource,
        arrival_in_ticks: i32,
    },
    Trail {
        target: Vec3,
        color: i32,
        duration: i32,
    },
    Shriek {
        delay: i32,
    },
}

/// Where a vibration particle travels to.
#[derive(Clone, Debug, PartialEq, IntoOwned)]
pub enum PositionSource {
    /// A packed block position.
    Block(i64),
    Entity {
        entity_id: i32,
        y_offset: f32,
    },
}

impl<'p> PacketWriter<'p> for Particle<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        use particle_type::*;

        let kind = VarInt::read(input)?.0;

        let options = match kind {
            BLOCK | BLOCK_MARKER | FALLING_DUST | DUST_PILLAR | BLOCK_CRUMBLE => ParticleOptions::Block { state: VarInt::read(input)?.0 },
            GEYSER | GEYSER_PLUME => ParticleOptions::Geyser { water_blocks: i32::read(input)? },
            GEYSER_BASE | GEYSER_POOF => {
                let water_blocks = i32::read(input)?;
                let burst_impulse_base = f32::read(input)?;

                ParticleOptions::GeyserBase { water_blocks, burst_impulse_base }
            }
            DRAGON_BREATH => ParticleOptions::Power { power: f32::read(input)? },
            DUST => {
                let color = i32::read(input)?;
                let scale = f32::read(input)?;

                ParticleOptions::Dust { color, scale }
            }
            DUST_COLOR_TRANSITION => {
                let from_color = i32::read(input)?;
                let to_color = i32::read(input)?;
                let scale = f32::read(input)?;

                ParticleOptions::DustColorTransition { from_color, to_color, scale }
            }
            EFFECT | INSTANT_EFFECT => {
                let color = i32::read(input)?;
                let power = f32::read(input)?;

                ParticleOptions::Spell { color, power }
            }
            ENTITY_EFFECT | TINTED_LEAVES | FLASH => ParticleOptions::Color { color: i32::read(input)? },
            SCULK_CHARGE => ParticleOptions::SculkCharge { roll: f32::read(input)? },
            ITEM => ParticleOptions::Item(ItemStackTemplate::read(input)?),
            VIBRATION => {
                let destination = PositionSource::read(input)?;
                let arrival_in_ticks = VarInt::read(input)?.0;

                ParticleOptions::Vibration { destination, arrival_in_ticks }
            }
            TRAIL => {
                let target = Vec3::read(input)?;
                let color = i32::read(input)?;
                let duration = VarInt::read(input)?.0;

                ParticleOptions::Trail { target, color, duration }
            }
            SHRIEK => ParticleOptions::Shriek { delay: VarInt::read(input)?.0 },
            _ => ParticleOptions::None,
        };

        Ok(Self { kind, options })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        VarInt(self.kind).encode(out)?;

        match &self.options {
            ParticleOptions::None => Ok(()),
            ParticleOptions::Block { state } => VarInt(*state).encode(out),
            ParticleOptions::Geyser { water_blocks } => water_blocks.encode(out),
            ParticleOptions::GeyserBase { water_blocks, burst_impulse_base } => {
                water_blocks.encode(out)?;
                burst_impulse_base.encode(out)
            }
            ParticleOptions::Power { power } => power.encode(out),
            ParticleOptions::Dust { color, scale } => {
                color.encode(out)?;
                scale.encode(out)
            }
            ParticleOptions::DustColorTransition { from_color, to_color, scale } => {
                from_color.encode(out)?;
                to_color.encode(out)?;
                scale.encode(out)
            }
            ParticleOptions::Spell { color, power } => {
                color.encode(out)?;
                power.encode(out)
            }
            ParticleOptions::Color { color } => color.encode(out),
            ParticleOptions::SculkCharge { roll } => roll.encode(out),
            ParticleOptions::Item(item) => item.encode(out),
            ParticleOptions::Vibration { destination, arrival_in_ticks } => {
                destination.encode(out)?;
                VarInt(*arrival_in_ticks).encode(out)
            }
            ParticleOptions::Trail { target, color, duration } => {
                target.encode(out)?;
                color.encode(out)?;
                VarInt(*duration).encode(out)
            }
            ParticleOptions::Shriek { delay } => VarInt(*delay).encode(out),
        }
    }

    fn body_len(&self) -> usize {
        let options = match &self.options {
            ParticleOptions::None => 0,
            ParticleOptions::Block { state } => VarInt(*state).body_len(),
            ParticleOptions::Geyser { .. } => 4,
            ParticleOptions::GeyserBase { .. } => 4 + 4,
            ParticleOptions::Power { .. } => 4,
            ParticleOptions::Dust { .. } => 4 + 4,
            ParticleOptions::DustColorTransition { .. } => 4 + 4 + 4,
            ParticleOptions::Spell { .. } => 4 + 4,
            ParticleOptions::Color { .. } => 4,
            ParticleOptions::SculkCharge { .. } => 4,
            ParticleOptions::Item(item) => item.body_len(),
            ParticleOptions::Vibration { destination, arrival_in_ticks } => destination.body_len() + VarInt(*arrival_in_ticks).body_len(),
            ParticleOptions::Trail { duration, .. } => 24 + 4 + VarInt(*duration).body_len(),
            ParticleOptions::Shriek { delay } => VarInt(*delay).body_len(),
        };

        VarInt(self.kind).body_len() + options
    }
}

/// Sent as its `minecraft:position_source_type` ID, then its data.
impl<'p> PacketWriter<'p> for PositionSource {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let source = match VarInt::read(input)?.0 {
            0 => Self::Block(i64::read(input)?),
            1 => {
                let entity_id = VarInt::read(input)?.0;
                let y_offset = f32::read(input)?;

                Self::Entity { entity_id, y_offset }
            }
            _ => return Err(DecodeError::EnumOutOfRange),
        };

        Ok(source)
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        match self {
            Self::Block(position) => {
                VarInt(0).encode(out)?;
                position.encode(out)
            }
            Self::Entity { entity_id, y_offset } => {
                VarInt(1).encode(out)?;
                VarInt(*entity_id).encode(out)?;
                y_offset.encode(out)
            }
        }
    }

    fn body_len(&self) -> usize {
        match self {
            Self::Block(_) => 1 + 8,
            Self::Entity { entity_id, .. } => 1 + VarInt(*entity_id).body_len() + 4,
        }
    }
}
