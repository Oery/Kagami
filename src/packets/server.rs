use kagami_macros::Serializable;
use kagami_macros::packet;

use std::borrow::Cow;
use std::io::Write;

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

#[packet(Play, 0x01, Server)]
pub struct JoinGame<'a> {
    #[format = "varint"]
    pub entity_id: i32,
    pub gamemode: u8,
    pub dimension: u8,
    pub difficulty: u8,
    pub max_players: u8,
    pub level_type: Cow<'a, str>,
    pub reduced_debug_info: u8,
}

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
