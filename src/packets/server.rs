use kagami_macros::Serializable;
use kagami_macros::SerializablePacket;
use kagami_macros::packet;

use std::borrow::Cow;
use std::io::Write;
use std::time::Duration;

use crate::events::*;
use crate::packet::*;
use crate::packets::data::*;
use crate::packets::json::*;
use crate::traits::*;

// TODO: Check if response can be zero copied
#[packet(Status, 0x00, Server)]
pub struct StatusResponse {
    #[format = "json"]
    pub response: Response,
}

#[packet(Login, 0x02, Server)]
pub struct LoginSuccess<'a> {
    pub uuid: Cow<'a, str>,
    pub username: Cow<'a, str>,
}

#[packet(Login, 0x03, Server)]
pub struct SetCompression {
    #[format = "varint"]
    pub threshold: i32,
}

#[packet(Play, 0x00, Server)]
pub struct KeepAlive {
    #[format = "varint"]
    pub id: i32,
}

// TODO: gamemode has a special bit for hardcore mode
// TODO: level_type should be an enum
#[packet(Play, 0x01, Server)]
pub struct JoinGame<'a> {
    pub entity_id: i32,
    #[enum_as = "u8"]
    pub gamemode: GameMode,
    #[enum_as = "i8"]
    pub dimension: Dimension,
    #[enum_as = "u8"]
    pub difficulty: Difficulty,
    pub max_players: u8,
    pub level_type: Cow<'a, str>,
    pub reduced_debug_info: bool,
}

// TODO: json is a JSON (obviously)
#[packet(Play, 0x02, Server)]
pub struct Chat {
    #[format = "json"]
    pub json: ChatComponent,
    #[enum_as = "u8"]
    pub position: ChatPosition,
}

#[packet(Play, 0x05, Server)]
pub struct SpawnPosition {
    pub location: Position,
}

#[packet(Play, 0x07, Server)]
pub struct Respawn<'a> {
    #[enum_as = "i32"]
    pub dimension: Dimension,
    #[enum_as = "u8"]
    pub difficulty: Difficulty,
    #[enum_as = "u8"]
    pub game_mode: GameMode,
    pub level_type: Cow<'a, str>,
}

#[packet(Play, 0x09, Server)]
pub struct HeldItemChange {
    pub slot: u8,
}

#[packet(Play, 0x0B, Server)]
pub struct Animation {
    #[format = "varint"]
    pub entity_id: i32,
    #[enum_as = "u8"]
    pub kind: AnimationKind,
}

#[packet(Play, 0x14, Server)]
pub struct Entity {
    #[format = "varint"]
    pub entity_id: i32,
}

#[packet(Play, 0x1D, Server)]
pub struct EntityEffect {
    #[format = "varint"]
    pub entity_id: i32,
    #[enum_as = "u8"]
    pub effect: PotionEffect,
    pub amplifier: u8,
    pub duration: Duration,
    pub hide_particles: bool,
}

#[packet(Play, 0x1E, Server)]
pub struct RemoveEntityEffect {
    #[format = "varint"]
    pub entity_id: i32,
    #[enum_as = "u8"]
    pub effect: PotionEffect,
}

#[packet(Play, 0x1F, Server)]
pub struct SetExperience {
    pub experience_bar: f32,
    #[format = "varint"]
    pub level: i32,
    #[format = "varint"]
    pub total_experience: i32,
}

#[packet(Play, 0x2B, Server)]
pub struct ChangeGameState {
    #[enum_as = "u8"]
    pub reason: GameStateReason,
}

#[packet(Play, 0x2E, Server)]
pub struct CloseWindow {
    pub window_id: u8,
}

// TODO: Reason is a JSON
#[packet(Play, 0x40, Server)]
pub struct Disconnect<'a> {
    pub reason: Cow<'a, str>,
}

#[packet(Play, 0x42, Server)]
pub struct CombatEvent<'a> {
    #[format = "varint"]
    pub event: EventKind<'a>,
}
