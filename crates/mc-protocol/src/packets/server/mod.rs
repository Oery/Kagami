use std::borrow::Cow;

use mc_protocol_macros::packet;
use uuid::Uuid;

use crate::data::config::*;
use crate::state::HandshakeIntent;
use crate::types::KnownPack;
use crate::types::bytes::Remainder;

#[packet(Handshake, 0x00, Serverbound)]
pub struct ServerListPing<'p> {
    #[format = "varint"]
    pub version: i32,
    pub addr: Cow<'p, str>,
    pub port: u16,
    #[format = "varint"]
    pub next_state: HandshakeIntent,
}

#[packet(Status, 0x00, Serverbound)]
pub struct StatusRequest;

#[packet(Status, 0x01, Serverbound)]
pub struct PingRequest {
    pub time: i64,
}

#[packet(Login, 0x00, Serverbound)]
pub struct LoginStart<'p> {
    pub name: Cow<'p, str>,
    pub uuid: Uuid,
}

#[packet(Login, 0x01, Serverbound)]
pub struct EncryptionResponse<'p> {
    pub shared_secret: Cow<'p, [u8]>,
    pub verify_token: Cow<'p, [u8]>,
}

#[packet(Login, 0x02, Serverbound)]
pub struct LoginPluginResponse<'p> {
    #[format = "varint"]
    pub message_id: i32,
    pub data: Cow<'p, [u8]>,
}

#[packet(Login, 0x03, Serverbound)]
pub struct LoginAcknowledge;

#[packet(Login, 0x04, Serverbound)]
pub struct LoginCookieResponse<'p> {
    pub ident: Cow<'p, str>,
    pub data: Cow<'p, [u8]>,
}

#[packet(Config, 0x00, Serverbound)]
pub struct ClientInformation<'p> {
    pub locale: Cow<'p, str>,
    pub render_distance: u8,
    #[format = "varint"]
    pub chat_mode: ChatMode,
    pub chat_colors: bool,
    pub skin_parts: SkinParts,
    #[format = "varint"]
    pub main_hand: Hand,
    pub text_filtering: bool,
    pub allow_server_listing: bool,
    #[format = "varint"]
    pub particle_status: ParticleStatus,
}

/// The same settings as [`ClientInformation`], resent in Play when they change.
#[packet(Play, 0x0E, Serverbound)]
pub struct PlayClientInformation<'p> {
    pub locale: Cow<'p, str>,
    pub render_distance: u8,
    #[format = "varint"]
    pub chat_mode: ChatMode,
    pub chat_colors: bool,
    pub skin_parts: SkinParts,
    #[format = "varint"]
    pub main_hand: Hand,
    pub text_filtering: bool,
    pub allow_server_listing: bool,
    #[format = "varint"]
    pub particle_status: ParticleStatus,
}

#[packet(Config, 0x01, Serverbound)]
pub struct ConfigCookieResponse<'p> {
    pub key: Cow<'p, str>,
    pub payload: Option<Cow<'p, [u8]>>,
}

#[packet(Config, 0x02, Serverbound)]
pub struct ConfigPluginMessage<'p> {
    pub channel: Cow<'p, str>,
    pub data: Remainder<'p>,
}

#[packet(Config, 0x03, Serverbound)]
pub struct AcknowledgeFinish;

#[packet(Config, 0x07, Serverbound)]
pub struct ConfigKnownPacks<'p> {
    pub packs: Vec<KnownPack<'p>>,
}

#[packet(Config, 0x04, Serverbound)]
pub struct ConfigKeepAliveResponse {
    pub id: i64,
}

#[packet(Config, 0x06, Serverbound)]
pub struct ConfigResourcePackResponse {
    pub uuid: Uuid,
    #[format = "varint"]
    pub result: ResourcePackResult,
}

#[packet(Config, 0x09, Serverbound)]
pub struct AcceptCodeOfConduct;

#[packet(Play, 0x03, Serverbound)]
pub struct BundleItemSelected {
    #[format = "varint"]
    pub slot_of_bundle: i32,
    #[format = "varint"]
    pub slot_in_bundle: i32,
}

#[packet(Play, 0x06, Serverbound)]
pub struct ChatAcknowledgement {
    #[format = "varint"]
    pub message_count: i32,
}

#[packet(Play, 0x07, Serverbound)]
pub struct ChatCommand<'p> {
    pub command: Cow<'p, str>,
}

#[packet(Play, 0x0D, Serverbound)]
pub struct ClientTickEnd;

#[packet(Play, 0x0F, Serverbound)]
pub struct CommandSuggestion<'p> {
    #[format = "varint"]
    pub transaction_id: i32,
    pub text: Cow<'p, str>,
}

#[packet(Play, 0x10, Serverbound)]
pub struct AcknowledgeConfiguration;

#[packet(Play, 0x2C, Serverbound)]
pub struct PlayerLoaded;

#[packet(Play, 0x2E, Serverbound)]
pub struct Punch;

#[packet(Play, 0x01, Serverbound)]
pub struct Attack {
    #[format = "varint"]
    pub entity_id: i32,
}

/// Spectator-menu teleport; the target may be any entity UUID.
#[packet(Play, 0x40, Serverbound)]
pub struct TeleportToEntity {
    pub target: Uuid,
}

#[packet(Play, 0x00, Serverbound)]
pub struct ConfirmTeleportation {
    #[format = "varint"]
    pub teleport_id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

#[packet(Play, 0x1C, Serverbound)]
pub struct PlayKeepAliveResponse {
    pub id: i64,
}

#[packet(Play, 0x0B, Serverbound)]
pub struct ChunkBatchReceived {
    pub chunks_per_tick: f32,
}

#[packet(Play, 0x1E, Serverbound)]
pub struct SetPlayerPosition {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub flags: u8,
}

#[packet(Play, 0x1F, Serverbound)]
pub struct SetPlayerPositionRotation {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
}

#[packet(Play, 0x20, Serverbound)]
pub struct SetPlayerRotation {
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
}

#[packet(Play, 0x21, Serverbound)]
pub struct SetPlayerMovementFlags {
    /// Bit 0: on ground; bit 1: horizontal collision.
    pub flags: u8,
}
mod gameplay;
pub use gameplay::*;
mod chat;
pub use chat::*;
mod common;
pub use common::*;
