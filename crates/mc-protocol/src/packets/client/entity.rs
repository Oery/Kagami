//! Entity spawning, movement, removal, metadata, equipment and vehicles for protocol 777.

use mc_protocol_macros::{IntoOwned, packet};
use uuid::Uuid;

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::packets::PacketWriter;
use crate::types::bytes::Remainder;
use crate::types::{LpVec3, VarInt, Vec3};

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

#[packet(Play, 0x36, Clientbound)]
pub struct MoveEntityPosition {
    #[format = "varint"]
    pub entity_id: i32,
    /// Bit 0: on ground.
    #[format = "varint"]
    pub properties: i32,
    pub dx: i16,
    pub dy: i16,
    pub dz: i16,
}

/// Angles are 1/256 turns; see [`encode_angle`] and [`decode_angle`].
#[packet(Play, 0x37, Clientbound)]
pub struct MoveEntityPositionRotation {
    #[format = "varint"]
    pub entity_id: i32,
    /// Bit 0: on ground.
    #[format = "varint"]
    pub properties: i32,
    pub dx: i16,
    pub dy: i16,
    pub dz: i16,
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
    /// The metadata entries up to the 0xFF terminator; not decoded, since
    /// values include item stacks and particles.
    pub metadata: Remainder<'p>,
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
    /// Pairs of an [`equipment_slot`] byte, with the top bit set when
    /// another pair follows, and an item stack; not decoded.
    pub equipment: Remainder<'p>,
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
