//! Cosmetic protocol 777 feedback: swings, entity animations, sounds and particles.
use std::borrow::Cow;

use mc_protocol_macros::{IntoOwned, packet};

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::packets::{PacketWriter, Payload};
use crate::types::VarInt;
use crate::{Direction, State};

/// `InteractionHand` ordinals; distinct from the configured dominant arm.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InteractionHand {
    #[default]
    Main = 0,
    Off = 1,
}

/// `SwingAnimationType` ordinals.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwingKind {
    None = 0,
    #[default]
    Whack = 1,
    Stab = 2,
}

/// The default `SwingAnimation` component duration, in client ticks.
pub const SWING_TICKS: i32 = 6;

#[packet(Play, 0x7B, Clientbound)]
pub struct SwingAnimation {
    #[format = "varint"]
    pub entity_id: i32,
    #[format = "varint"]
    pub hand: i32,
    #[format = "varint"]
    pub kind: i32,
    #[format = "varint"]
    pub duration: i32,
}

/// Entity Animation IDs remaining in protocol 777.
pub mod animation {
    pub const CRITICAL_HIT: u8 = 1;
    pub const MAGIC_CRITICAL_HIT: u8 = 2;
}

#[packet(Play, 0x02, Clientbound)]
pub struct EntityAnimation {
    #[format = "varint"]
    pub entity_id: i32,
    pub animation: u8,
}

/// An inline `Holder<SoundEvent>`: zero instead of `registry ID + 1`, then the
/// direct definition. This avoids shipping the sound-event registry table.
#[derive(Clone, Debug, IntoOwned)]
pub struct InlineSound<'p> {
    pub name: Cow<'p, str>,
    pub fixed_range: Option<f32>,
}

impl<'p> PacketWriter<'p> for InlineSound<'p> {
    fn read(buf: &mut &'p [u8]) -> DecodeResult<Self> {
        if VarInt::read(buf)?.0 != 0 {
            return Err(crate::error::DecodeError::EnumOutOfRange);
        }
        Ok(Self { name: Cow::read(buf)?, fixed_range: Option::read(buf)? })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        VarInt(0).encode(out)?;
        self.name.encode(out)?;
        self.fixed_range.encode(out)
    }

    fn body_len(&self) -> usize {
        VarInt(0).body_len() + self.name.body_len() + self.fixed_range.body_len()
    }
}

/// `SoundSource` ordinals.
pub mod sound_category {
    pub const MASTER: i32 = 0;
    pub const BLOCK: i32 = 4;
    pub const HOSTILE: i32 = 5;
    pub const NEUTRAL: i32 = 6;
    pub const PLAYER: i32 = 7;
    pub const AMBIENT: i32 = 8;
}

/// Positions are fixed-point eighths of a block.
#[packet(Play, 0x77, Clientbound)]
pub struct SoundEffect<'p> {
    pub sound: InlineSound<'p>,
    #[format = "varint"]
    pub category: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub volume: f32,
    pub pitch: f32,
    pub seed: i64,
}

#[packet(Play, 0x76, Clientbound)]
pub struct EntitySoundEffect<'p> {
    pub sound: InlineSound<'p>,
    #[format = "varint"]
    pub category: i32,
    #[format = "varint"]
    pub entity_id: i32,
    pub volume: f32,
    pub pitch: f32,
    pub seed: i64,
}

/// An absent category or identifier matches every category or sound respectively.
#[derive(Clone, Debug, PartialEq, Eq, IntoOwned)]
pub struct StopSound<'p> {
    pub category: Option<i32>,
    pub name: Option<Cow<'p, str>>,
}

impl<'p> Payload<'p> for StopSound<'p> {
    const DIRECTION: Direction = Direction::Clientbound;
    const ID: VarInt = VarInt(0x79);
    const STATE: State = State::Play;

    fn payload_len(&self) -> EncodeResult<usize> {
        Ok(1 + self.category.map_or(0, |category| VarInt(category).body_len()) + self.name.as_ref().map_or(0, |name| name.body_len()))
    }

    fn decode_payload(buf: &mut &'p [u8]) -> DecodeResult<Self> {
        let flags = u8::read(buf)?;
        if flags & !3 != 0 {
            return Err(crate::error::DecodeError::EnumOutOfRange);
        }
        let category = if flags & 1 != 0 {
            Some(VarInt::read(buf)?.0)
        } else {
            None
        };
        let name = if flags & 2 != 0 { Some(Cow::read(buf)?) } else { None };
        Ok(Self { category, name })
    }

    fn encode_payload(&self, out: &mut Vec<u8>) -> EncodeResult {
        let flags = u8::from(self.category.is_some()) | (u8::from(self.name.is_some()) << 1);
        flags.encode(out)?;
        if let Some(category) = self.category {
            VarInt(category).encode(out)?;
        }
        if let Some(name) = &self.name {
            name.encode(out)?;
        }
        Ok(())
    }
}

/// `minecraft:particle_type` IDs of data-less particles, from 26.3 registration order.
pub mod particle {
    /// Verified against the official 26.3 PARTICLE_TYPE registry.
    pub const NOTE: i32 = 68;
    pub const CRIT: i32 = 13;
    pub const DAMAGE_INDICATOR: i32 = 14;
    pub const ENCHANTED_HIT: i32 = 25;
    pub const SWEEP_ATTACK: i32 = 77;
    /// Takes an ARGB color; used by status effect swirls.
    pub const ENTITY_EFFECT: i32 = 28;
}

/// Level Particles for particle types without options; their data is empty.
#[packet(Play, 0x30, Clientbound)]
pub struct LevelParticles {
    #[format = "varint"]
    pub particle: i32,
    pub override_limiter: bool,
    pub always_visible: bool,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_z: f32,
    pub speed_x: f32,
    pub speed_y: f32,
    pub speed_z: f32,
    #[format = "varint"]
    pub count: i32,
    #[format = "varint"]
    pub randomization: i32,
}

/// `MobEffectInstance` presentation flags in Update Mob Effect.
pub mod mob_effect_flags {
    pub const AMBIENT: u8 = 0x01;
    pub const PARTICLES: u8 = 0x02;
    pub const ICON: u8 = 0x04;
}

/// Adds or replaces a status effect; a duration of -1 never expires.
#[packet(Play, 0x87, Clientbound)]
pub struct UpdateMobEffect {
    #[format = "varint"]
    pub entity_id: i32,
    /// Index in the built-in `minecraft:mob_effect` registry.
    #[format = "varint"]
    pub effect_id: i32,
    #[format = "varint"]
    pub amplifier: i32,
    #[format = "varint"]
    pub duration: i32,
    pub flags: u8,
}

#[packet(Play, 0x4F, Clientbound)]
pub struct RemoveMobEffect {
    #[format = "varint"]
    pub entity_id: i32,
    #[format = "varint"]
    pub effect_id: i32,
}

/// `AttributeModifier.Operation` ordinals.
pub mod attribute_operation {
    pub const ADD_VALUE: u8 = 0;
    pub const ADD_MULTIPLIED_TOTAL: u8 = 2;
}

/// `minecraft:attribute` registry IDs, from 26.3 registration order.
pub mod attribute {
    pub const ATTACK_DAMAGE: i32 = 3;
    pub const MAX_ABSORPTION: i32 = 22;
    pub const MOVEMENT_SPEED: i32 = 26;
    pub const SAFE_FALL_DISTANCE: i32 = 29;
}

#[derive(Clone, Debug, PartialEq, IntoOwned)]
pub struct AttributeModifier<'p> {
    pub id: Cow<'p, str>,
    pub amount: f64,
    pub operation: u8,
}

impl<'p> PacketWriter<'p> for AttributeModifier<'p> {
    fn read(buf: &mut &'p [u8]) -> DecodeResult<Self> {
        Ok(Self { id: Cow::read(buf)?, amount: f64::read(buf)?, operation: u8::read(buf)? })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.id.encode(out)?;
        self.amount.encode(out)?;
        self.operation.encode(out)
    }

    fn body_len(&self) -> usize {
        self.id.body_len() + 8 + 1
    }
}

/// One attribute's base and complete modifier set. The client replaces all
/// of the attribute's modifiers, including ones it added itself.
#[derive(Clone, Debug, PartialEq, IntoOwned)]
pub struct AttributeSnapshot<'p> {
    pub attribute: i32,
    pub base: f64,
    pub modifiers: Vec<AttributeModifier<'p>>,
}

impl<'p> PacketWriter<'p> for AttributeSnapshot<'p> {
    fn read(buf: &mut &'p [u8]) -> DecodeResult<Self> {
        Ok(Self { attribute: VarInt::read(buf)?.0, base: f64::read(buf)?, modifiers: Vec::read(buf)? })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        VarInt(self.attribute).encode(out)?;
        self.base.encode(out)?;
        self.modifiers.encode(out)
    }

    fn body_len(&self) -> usize {
        VarInt(self.attribute).body_len() + 8 + self.modifiers.body_len()
    }
}

#[packet(Play, 0x86, Clientbound)]
pub struct UpdateAttributes<'p> {
    #[format = "varint"]
    pub entity_id: i32,
    pub attributes: Vec<AttributeSnapshot<'p>>,
}

#[cfg(test)]
#[path = "../../../tests/tests_packets_client_effect.rs"]
mod tests;
