use kagami_macros::Serializable;
use kagami_macros::packet;

use std::borrow::Cow;
use std::io::Write;
use std::time::Duration;

use crate::events::*;
use crate::packet::*;
use crate::packets::data::*;

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
// FIXME: Dimension is a byte here instead of an int
#[packet(Play, 0x01, Server)]
pub struct JoinGame<'a> {
    #[format = "varint"]
    pub entity_id: i32,
    pub gamemode: u8,
    pub dimension: u8,
    #[from = "u8"]
    pub difficulty: Difficulty,
    pub max_players: u8,
    pub level_type: Cow<'a, str>,
    pub reduced_debug_info: bool,
}

// TODO: json is a JSON (obviously)
#[packet(Play, 0x02, Server)]
pub struct Chat<'a> {
    pub json: Cow<'a, str>,
    #[from = "u8"]
    pub position: ChatPosition,
}

#[packet(Play, 0x07, Server)]
pub struct Respawn<'a> {
    #[from = "i32"]
    pub dimension: Dimension,
    #[from = "u8"]
    pub difficulty: Difficulty,
    #[from = "u8"]
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
    #[from = "u8"]
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
    #[from = "u8"]
    pub effect: PotionEffect,
    pub amplifier: u8,
    pub duration: Duration,
    pub hide_particles: bool,
}

#[packet(Play, 0x1E, Server)]
pub struct RemoveEntityEffect {
    #[format = "varint"]
    pub entity_id: i32,
    #[from = "u8"]
    pub effect: PotionEffect,
}

#[packet(Play, 0x2E, Server)]
pub struct CloseWindow {
    pub window_id: u8,
}

// TODO: Reason is a JSON
#[packet(Play, 0x2E, Server)]
pub struct Disconnect<'a> {
    pub reason: Cow<'a, str>,
}
