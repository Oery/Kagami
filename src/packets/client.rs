use kagami_macros::Serializable;
use kagami_macros::SerializablePacket;
use kagami_macros::packet;

use std::borrow::Cow;
use std::io::Write;

use crate::events::*;
use crate::packet::*;
use crate::packets::data::*;
use crate::state::*;

#[packet(Handshake, 0x00, Client)]
pub struct Handshake<'a> {
    #[format = "varint"]
    pub protocol_version: i32,
    pub addr: Cow<'a, str>,
    pub port: i16,
    #[format = "varint"]
    #[from = "i32"]
    pub next_state: McState,
}

#[packet(Handshake, 0xFE, Client)]
pub struct LegacyPing {
    pub payload: u8,
}

#[packet(Play, 0x01, Client)]
pub struct Chat<'a> {
    pub message: Cow<'a, str>,
}

#[derive(Debug, Serializable)]
#[my_repr(i32)]
pub enum InteractionKind {
    Interact,
    Attack,
    InteractAt { x: f32, y: f32, z: f32 },
}

#[packet(Play, 0x02, Client)]
pub struct UseEntity {
    #[format = "varint"]
    pub target: i32,
    // #[format = "varint"]
    pub kind: InteractionKind,
}

#[packet(Play, 0x05, Client)]
pub struct KeepAlive {
    #[format = "varint"]
    pub id: i32,
}

#[packet(Play, 0x0B, Client)]
pub struct EntityAction {
    #[format = "varint"]
    pub entity_id: i32,
    pub action: ActionKind,
    #[format = "varint"]
    pub boost: i32,
}
