//! Entity spawning, movement and removal for protocol 777.

use mc_protocol_macros::packet;
use uuid::Uuid;

use crate::types::{LpVec3, VarInt};

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

#[cfg(test)]
#[path = "../../../tests/tests_packets_client_entity.rs"]
mod tests;
