use kagami_macros::packet;

use std::borrow::Cow;
use std::io::Write;

use crate::context::ProxyContext;
use crate::error::PResult;
use crate::events::*;
use crate::packet::{Packet, string, varint_i32};
use crate::proxy::{Dispatch, Payload};
use crate::varint::temp_convert;

#[packet(Login, 0x02, Server)]
pub struct LoginSuccess<'a> {
    pub uuid: Cow<'a, str>,
    pub username: Cow<'a, str>,
}

#[packet(Login, 0x03, Server)]
pub struct SetCompression {
    pub threshold: i32,
}

#[packet(Play, 0x02, Server)]
pub struct Chat<'a> {
    pub json: Cow<'a, str>,
    pub position: u8,
}
