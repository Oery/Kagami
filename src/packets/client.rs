use kagami_macros::packet;

use std::borrow::Cow;
use std::io::Write;

use crate::events::*;
use crate::packet::*;
use crate::state::*;

#[packet(Handshake, 0x00, Client)]
pub struct Handshake<'a> {
    pub protocol_version: i32,
    pub addr: Cow<'a, str>,
    pub port: i16,
    pub next_state: McState,
}

#[packet(Play, 0x01, Client)]
pub struct Chat<'a> {
    pub message: Cow<'a, str>,
}

#[packet(Play, 0x05, Client)]
pub struct KeepAlive {
    pub id: i32,
}
