use kagami_macros::packet;

use std::borrow::Cow;
use std::io::Write;

use crate::events::*;
use crate::packet::*;

#[packet(Login, 0x02, Server)]
pub struct LoginSuccess<'a> {
    pub uuid: Cow<'a, str>,
    pub username: Cow<'a, str>,
}

#[packet(Login, 0x03, Server)]
pub struct SetCompression {
    pub threshold: i32,
}

#[packet(Play, 0x00, Server)]
pub struct KeepAlive {
    pub id: i32,
}

// FIXME: entity_id is not varint encoded
#[packet(Play, 0x01, Server)]
pub struct JoinGame<'a> {
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
    pub position: u8,
}
