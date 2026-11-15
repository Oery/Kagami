//! Serverbound packets sent in both configuration and play, for protocol 777.
use std::borrow::Cow;

use mc_protocol_macros::packet;
use uuid::Uuid;

use crate::data::config::ResourcePackResult;
use crate::types::bytes::Remainder;

/// Answers the server's Ping with the same id.
#[packet(Config, 0x05, Serverbound)]
pub struct ConfigPong {
    pub id: i32,
}

#[packet(Play, 0x2D, Serverbound)]
pub struct PlayPong {
    pub id: i32,
}

/// The client measuring its own latency in play; answered by Ping Response.
#[packet(Play, 0x26, Serverbound)]
pub struct PlayPingRequest {
    pub time: i64,
}

#[packet(Play, 0x15, Serverbound)]
pub struct PlayCookieResponse<'p> {
    pub key: Cow<'p, str>,
    pub payload: Option<Cow<'p, [u8]>>,
}

/// A plugin message from the client in play. The configuration one is
/// [`ConfigPluginMessage`](super::ConfigPluginMessage).
#[packet(Play, 0x16, Serverbound)]
pub struct ClientPluginMessage<'p> {
    pub channel: Cow<'p, str>,
    pub data: Remainder<'p>,
}

#[packet(Play, 0x32, Serverbound)]
pub struct PlayResourcePackResponse {
    pub uuid: Uuid,
    #[format = "varint"]
    pub result: ResourcePackResult,
}

/// A dialog or text click with a `custom` action.
#[packet(Config, 0x08, Serverbound)]
pub struct ConfigCustomClickAction<'p> {
    pub id: Cow<'p, str>,
    /// Network NBT behind a length prefix; a lone `TAG_End` when absent.
    pub payload: Cow<'p, [u8]>,
}

#[packet(Play, 0x44, Serverbound)]
pub struct PlayCustomClickAction<'p> {
    pub id: Cow<'p, str>,
    pub payload: Cow<'p, [u8]>,
}
