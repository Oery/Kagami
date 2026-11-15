use crate::data::config::*;
use crate::state::HandshakeIntent;
use crate::types::KnownPack;
use crate::types::bytes::Remainder;

use mc_protocol_macros::packet;
use uuid::Uuid;

#[packet(Handshake, 0x00, Server)]
pub struct ServerListPing<'p> {
    #[format = "varint"]
    pub version: i32,
    pub addr: &'p str,
    pub port: u16,
    #[format = "varint"]
    pub next_state: HandshakeIntent,
}

#[packet(Status, 0x00, Server)]
pub struct StatusRequest;

#[packet(Status, 0x01, Server)]
pub struct PingRequest {
    pub time: i64,
}

#[packet(Login, 0x00, Server)]
pub struct LoginStart<'p> {
    pub name: &'p str,
    pub uuid: Uuid,
}

#[packet(Login, 0x01, Server)]
pub struct EncryptionResponse<'p> {
    pub shared_secret: &'p [u8],
    pub verify_token: &'p [u8],
}

#[packet(Login, 0x02, Server)]
pub struct LoginPluginResponse<'p> {
    #[format = "varint"]
    pub message_id: i32,
    pub data: &'p [u8],
}

#[packet(Login, 0x03, Server)]
pub struct LoginAcknowledge;

#[packet(Login, 0x04, Server)]
pub struct LoginCookieResponse<'p> {
    pub ident: &'p str,
    pub data: &'p [u8],
}

#[packet(Config, 0x00, Server)]
pub struct ClientInformation<'p> {
    pub locale: &'p str,
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

#[packet(Config, 0x01, Server)]
pub struct ConfigCookieResponse<'p> {
    pub key: &'p str,
    pub payload: Option<&'p [u8]>,
}

#[packet(Config, 0x02, Server)]
pub struct ConfigPluginMessage<'p> {
    pub channel: &'p str,
    pub data: Remainder<'p>,
}

#[packet(Config, 0x03, Server)]
pub struct AcknowledgeFinish;

#[packet(Config, 0x07, Server)]
pub struct ConfigKnownPacks<'p> {
    pub packs: Vec<KnownPack<'p>>,
}

#[packet(Config, 0x04, Server)]
pub struct ConfigKeepAlive {
    pub id: i64,
}

#[packet(Config, 0x06, Server)]
pub struct ConfigResourcePackResponse {
    pub uuid: Uuid,
    #[format = "varint"]
    pub result: ResourcePackResult,
}

#[packet(Config, 0x09, Server)]
pub struct AcceptCodeOfConduct;

#[packet(Play, 0x03, Server)]
pub struct BundleItemSelected {
    #[format = "varint"]
    pub slot_of_bundle: i32,
    #[format = "varint"]
    pub slot_in_bundle: i32,
}

#[packet(Play, 0x06, Server)]
pub struct ChatAcknowledgement {
    #[format = "varint"]
    pub message_count: i32,
}

#[packet(Play, 0x07, Server)]
pub struct ChatCommand<'p> {
    pub command: &'p str,
}

#[packet(Play, 0x0D, Server)]
pub struct ClientTickEnd;

#[packet(Play, 0x0F, Server)]
pub struct CommandSuggestion<'p> {
    #[format = "varint"]
    pub transaction_id: i32,
    pub text: &'p str,
}

#[packet(Play, 0x10, Server)]
pub struct AcknowledgeConfiguration;

#[packet(Play, 0x2C, Server)]
pub struct PlayerLoaded;

#[packet(Play, 0x2E, Server)]
pub struct Punch;

#[packet(Play, 0x01, Server)]
pub struct Attack {
    #[format = "varint"]
    pub entity_id: i32,
}

/// Spectator-menu teleport; the target may be any entity UUID.
#[packet(Play, 0x40, Server)]
pub struct TeleportToEntity {
    pub target: Uuid,
}

#[packet(Play, 0x00, Server)]
pub struct ConfirmTeleportation {
    #[format = "varint"]
    pub teleport_id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

#[packet(Play, 0x1C, Server)]
pub struct PlayKeepAliveResponse {
    pub id: i64,
}

#[packet(Play, 0x0B, Server)]
pub struct ChunkBatchReceived {
    pub chunks_per_tick: f32,
}

#[packet(Play, 0x1E, Server)]
pub struct SetPlayerPosition {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub flags: u8,
}

#[packet(Play, 0x1F, Server)]
pub struct SetPlayerPositionRotation {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
}

#[packet(Play, 0x20, Server)]
pub struct SetPlayerRotation {
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
}

#[packet(Play, 0x21, Server)]
pub struct SetPlayerMovementFlags {
    /// Bit 0: on ground; bit 1: horizontal collision.
    pub flags: u8,
}
mod gameplay;
pub use gameplay::*;
mod chat;
pub use chat::*;
