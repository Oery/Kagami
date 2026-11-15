//! Entity spawning, movement, removal, metadata, equipment and vehicles for protocol 777.

use mc_protocol_macros::{IntoOwned, packet};
use uuid::Uuid;

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::error::EncodeError;
use crate::packets::PacketWriter;
use crate::types::{EntityMetadata, LpVec3, Slot, VarInt, Vec3};

/// `data` is type-specific: falling blocks carry their block state; items and
/// players carry zero.
#[packet(Play, 0x01, Clientbound)]
pub struct AddEntity {
    #[format = "varint"]
    pub entity_id: i32,
    pub uuid: Uuid,
    #[format = "varint"]
    pub entity_type: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub velocity: LpVec3,
    pub pitch: u8,
    pub yaw: u8,
    pub head_yaw: u8,
    #[format = "varint"]
    pub data: i32,
}

#[packet(Play, 0x67, Clientbound)]
pub struct SetEntityVelocity {
    #[format = "varint"]
    pub entity_id: i32,
    pub velocity: LpVec3,
}

#[packet(Play, 0x08, Clientbound)]
pub struct BlockUpdate {
    pub location: i64,
    #[format = "varint"]
    pub block_state: i32,
}

#[packet(Play, 0x25, Clientbound)]
pub struct AddTransientBlock {
    pub location: i64,
    #[format = "varint"]
    pub block_state: i32,
}

/// The player entry in the Minecraft Java 26.3 entity-type registry.
pub const PLAYER_ENTITY_TYPE: i32 = 159;
/// Falling sand, red sand and gravel share this 26.3 entity type.
pub const FALLING_BLOCK_ENTITY_TYPE: i32 = 52;
/// Dropped item stacks in the 26.3 entity-type registry.
pub const ITEM_ENTITY_TYPE: i32 = 72;

/// The collect animation: `amount` items fly from the item entity to the collector.
#[packet(Play, 0x7F, Clientbound)]
pub struct TakeItemEntity {
    #[format = "varint"]
    pub item_id: i32,
    #[format = "varint"]
    pub collector_id: i32,
    #[format = "varint"]
    pub amount: i32,
}

#[packet(Play, 0x22, Clientbound)]
pub struct EntityStatus {
    pub entity_id: i32,
    pub status: u8,
}

/// Converts degrees to the protocol's unsigned, wrapping 1/256-turn angle.
pub fn encode_angle(degrees: f32) -> u8 {
    ((f64::from(degrees).rem_euclid(360.0) * 256.0 / 360.0).floor() as u16) as u8
}

/// Converts a protocol 1/256-turn angle to degrees in `0.0..360.0`.
pub fn decode_angle(angle: u8) -> f32 {
    f32::from(angle) * 360.0 / 256.0
}

/// One step of a stepped movement. Deltas are fixed-point with 12
/// fractional bits: 4096 is one block.
#[derive(Clone, Copy, Debug, PartialEq, Eq, IntoOwned)]
pub struct DeltaStep {
    /// The tick of this step, counted from the packet.
    pub ticks: i32,
    pub dx: i16,
    pub dy: i16,
    pub dz: i16,
}

/// How far an entity moved. Deltas are fixed-point with 12 fractional
/// bits: 4096 is one block.
#[derive(Clone, Debug, PartialEq, Eq, IntoOwned)]
pub enum PositionDelta {
    Linear {
        dx: i16,
        dy: i16,
        dz: i16,
    },
    /// Never empty: zero steps is how the wire says `Linear`.
    Stepped(Vec<DeltaStep>),
}

/// The on-ground flag and the position delta of a movement packet.
///
/// On the wire both start in one VarInt: bit 0 is on ground, the bits
/// above count the steps.
#[derive(Clone, Debug, PartialEq, Eq, IntoOwned)]
pub struct EntityMovement {
    pub on_ground: bool,
    pub delta: PositionDelta,
}

impl EntityMovement {
    fn step_count(&self) -> usize {
        match &self.delta {
            PositionDelta::Linear { .. } => 0,
            PositionDelta::Stepped(steps) => steps.len(),
        }
    }

    /// Fails for an empty step list or one too long for the VarInt.
    fn properties(&self) -> EncodeResult<i32> {
        let step_count = self.step_count();
        if matches!(self.delta, PositionDelta::Stepped(_)) && step_count == 0 {
            return Err(EncodeError::InvalidLength);
        }

        let step_count = i32::try_from(step_count).map_err(|_| EncodeError::InvalidLength)?;
        let shifted = step_count.checked_mul(2).ok_or(EncodeError::InvalidLength)?;

        Ok(shifted | i32::from(self.on_ground))
    }
}

impl PacketWriter<'_> for EntityMovement {
    fn read(input: &mut &[u8]) -> DecodeResult<Self> {
        let properties = VarInt::read(input)?.0 as u32;
        let on_ground = properties & 1 != 0;
        let step_count = properties >> 1;

        if step_count == 0 {
            let dx = i16::read(input)?;
            let dy = i16::read(input)?;
            let dz = i16::read(input)?;

            return Ok(Self { on_ground, delta: PositionDelta::Linear { dx, dy, dz } });
        }

        // Each step takes at least seven bytes, so a short input fails
        // before the list grows far.
        let mut steps = Vec::new();

        for _ in 0..step_count {
            let ticks = VarInt::read(input)?.0;
            let dx = i16::read(input)?;
            let dy = i16::read(input)?;
            let dz = i16::read(input)?;
            steps.push(DeltaStep { ticks, dx, dy, dz });
        }

        Ok(Self { on_ground, delta: PositionDelta::Stepped(steps) })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        VarInt(self.properties()?).encode(out)?;

        match &self.delta {
            PositionDelta::Linear { dx, dy, dz } => {
                dx.encode(out)?;
                dy.encode(out)?;
                dz.encode(out)?;
            }
            PositionDelta::Stepped(steps) => {
                for step in steps {
                    VarInt(step.ticks).encode(out)?;
                    step.dx.encode(out)?;
                    step.dy.encode(out)?;
                    step.dz.encode(out)?;
                }
            }
        }

        Ok(())
    }

    fn body_len(&self) -> usize {
        // Movement that cannot be encoded fails in `encode`; its length is moot.
        let properties = self.properties().unwrap_or(0);
        let mut length = VarInt(properties).body_len();

        match &self.delta {
            PositionDelta::Linear { .. } => length += 6,
            PositionDelta::Stepped(steps) => {
                for step in steps {
                    length += VarInt(step.ticks).body_len() + 6;
                }
            }
        }

        length
    }
}

#[packet(Play, 0x36, Clientbound)]
pub struct MoveEntityPosition {
    #[format = "varint"]
    pub entity_id: i32,
    pub movement: EntityMovement,
}

/// Angles are 1/256 turns; see [`encode_angle`] and [`decode_angle`].
#[packet(Play, 0x37, Clientbound)]
pub struct MoveEntityPositionRotation {
    #[format = "varint"]
    pub entity_id: i32,
    pub movement: EntityMovement,
    pub yaw: u8,
    pub pitch: u8,
}

#[packet(Play, 0x39, Clientbound)]
pub struct MoveEntityRotation {
    #[format = "varint"]
    pub entity_id: i32,
    pub on_ground: bool,
    pub yaw: u8,
    pub pitch: u8,
}

#[packet(Play, 0x23, Clientbound)]
pub struct EntityPositionSync {
    #[format = "varint"]
    pub entity_id: i32,
    #[format = "varint"]
    pub path_type: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

#[packet(Play, 0x55, Clientbound)]
pub struct SetHeadRotation {
    #[format = "varint"]
    pub entity_id: i32,
    pub yaw: u8,
}

#[packet(Play, 0x4E, Clientbound)]
pub struct RemoveEntities {
    pub entity_ids: Vec<VarInt>,
}

#[packet(Play, 0x46, Clientbound)]
pub struct PlayerInfoRemove {
    pub uuids: Vec<Uuid>,
}

#[packet(Play, 0x65, Clientbound)]
pub struct SetEntityMetadata<'p> {
    #[format = "varint"]
    pub entity_id: i32,
    pub metadata: EntityMetadata<'p>,
}

/// `EquipmentSlot` ordinals in Set Equipment.
pub mod equipment_slot {
    pub const MAIN_HAND: u8 = 0;
    pub const OFF_HAND: u8 = 1;
    pub const FEET: u8 = 2;
    pub const LEGS: u8 = 3;
    pub const CHEST: u8 = 4;
    pub const HEAD: u8 = 5;
    pub const BODY: u8 = 6;
    pub const SADDLE: u8 = 7;
}

#[packet(Play, 0x68, Clientbound)]
pub struct SetEquipment<'p> {
    #[format = "varint"]
    pub entity_id: i32,
    pub equipment: EquipmentList<'p>,
}

#[derive(Debug, IntoOwned)]
pub struct Equipment<'p> {
    /// An [`equipment_slot`].
    pub slot: u8,
    pub item: Slot<'p>,
}

/// At least one piece of equipment. On the wire, each slot byte has its top
/// bit set when another piece follows.
#[derive(Debug, IntoOwned)]
pub struct EquipmentList<'p>(pub Vec<Equipment<'p>>);

const ANOTHER_PIECE_FOLLOWS: u8 = 0x80;

impl<'p> PacketWriter<'p> for EquipmentList<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let mut pieces = Vec::new();

        loop {
            let slot_byte = u8::read(input)?;
            let item = Slot::read(input)?;

            pieces.push(Equipment { slot: slot_byte & !ANOTHER_PIECE_FOLLOWS, item });

            if slot_byte & ANOTHER_PIECE_FOLLOWS == 0 {
                break;
            }
        }

        Ok(Self(pieces))
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        let Some(last) = self.0.len().checked_sub(1) else {
            return Err(EncodeError::InvalidLength);
        };

        for (index, piece) in self.0.iter().enumerate() {
            let mut slot_byte = piece.slot;
            if index != last {
                slot_byte |= ANOTHER_PIECE_FOLLOWS;
            }

            slot_byte.encode(out)?;
            piece.item.encode(out)?;
        }

        Ok(())
    }

    fn body_len(&self) -> usize {
        self.0.iter().map(|piece| 1 + piece.item.body_len()).sum()
    }
}

/// A lead between two entities; a holder of -1 detaches it.
#[packet(Play, 0x66, Clientbound)]
pub struct LinkEntities {
    pub attached_id: i32,
    pub holder_id: i32,
}

#[packet(Play, 0x6D, Clientbound)]
pub struct SetPassengers {
    #[format = "varint"]
    pub vehicle_id: i32,
    pub passenger_ids: Vec<VarInt>,
}

/// `yaw` is the direction of the damage, relative to the entity.
#[packet(Play, 0x2B, Clientbound)]
pub struct HurtAnimation {
    #[format = "varint"]
    pub entity_id: i32,
    pub yaw: f32,
}

#[derive(Debug, IntoOwned)]
pub struct MinecartStep {
    pub position: Vec3,
    pub velocity: Vec3,
    pub yaw: u8,
    pub pitch: u8,
    pub weight: f32,
}

impl PacketWriter<'_> for MinecartStep {
    fn read(input: &mut &[u8]) -> DecodeResult<Self> {
        let position = Vec3::read(input)?;
        let velocity = Vec3::read(input)?;
        let yaw = u8::read(input)?;
        let pitch = u8::read(input)?;
        let weight = f32::read(input)?;

        Ok(Self { position, velocity, yaw, pitch, weight })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.position.encode(out)?;
        self.velocity.encode(out)?;
        self.yaw.encode(out)?;
        self.pitch.encode(out)?;

        self.weight.encode(out)
    }

    fn body_len(&self) -> usize {
        24 + 24 + 1 + 1 + 4
    }
}

/// Minecart movement under the experimental minecart physics.
#[packet(Play, 0x38, Clientbound)]
pub struct MoveMinecartAlongTrack {
    #[format = "varint"]
    pub entity_id: i32,
    pub steps: Vec<MinecartStep>,
}

/// Moves the vehicle this client controls; degrees.
#[packet(Play, 0x3A, Clientbound)]
pub struct MoveVehicle {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

/// Teleports an entity; each axis may be relative, see `flags`.
#[packet(Play, 0x80, Clientbound)]
pub struct SyncVehiclePosition {
    #[format = "varint"]
    pub entity_id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub velocity_z: f64,
    pub yaw: f32,
    pub pitch: f32,
    /// Teleport flags, as in Synchronize Player Position.
    pub flags: i32,
    pub on_ground: bool,
}

/// The acceleration of a wind charge or fireball.
#[packet(Play, 0x8A, Clientbound)]
pub struct ProjectilePower {
    #[format = "varint"]
    pub entity_id: i32,
    pub power: f64,
}

#[cfg(test)]
#[path = "../../../tests/tests_packets_client_entity.rs"]
mod tests;
