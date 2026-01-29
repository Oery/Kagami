use kagami_macros::packet;

use std::borrow::Cow;
use std::io::Write;

use crate::context::ProxyContext;
use crate::error::PResult;
use crate::events::*;
use crate::packet::{Packet, short, state, string, varint_i32};
use crate::proxy::{Dispatch, Payload};
use crate::state::McState;
use crate::varint::temp_convert;

#[packet(Handshake, 0x00, Client)]
pub struct Handshake<'a> {
    pub protocol_version: i32,
    pub addr: Cow<'a, str>,
    pub port: i16,
    pub next_state: McState,
}

#[packet(Login, 0x02, Server)]
pub struct LoginSuccess<'a> {
    pub uuid: Cow<'a, str>,
    pub username: Cow<'a, str>,
}

#[packet(Login, 0x03, Server)]
pub struct SetCompression {
    pub threshold: i32,
}

#[packet(Play, 0x01, Client)]
pub struct Chat<'a> {
    pub message: Cow<'a, str>,
}

#[packet(Play, 0x05, Client)]
pub struct KeepAlive {
    pub id: i32,
}

#[packet(Play, 0x02, Server)]
pub struct ServerChat<'a> {
    pub json: Cow<'a, str>,
    pub position: u8,
}
