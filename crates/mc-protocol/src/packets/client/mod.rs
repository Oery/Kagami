use std::borrow::Cow;

use mc_protocol_macros::packet;
use uuid::Uuid;

use crate::types::{BlockEntity, GameProfile, Heightmap, KnownPack, LightData, Nbt, RegistryEntry, TaggedRegistry, bytes::Remainder};

mod boss_bar;
pub use boss_bar::*;
mod effect;
pub use effect::*;
mod entity;
pub use entity::*;
mod gameplay;
pub use gameplay::*;
mod player;
pub use player::*;
mod commands;
pub use commands::*;
mod team;
pub use team::*;
mod blocks;
pub use blocks::*;
mod scoreboard;
pub use scoreboard::*;

/// The raw JSON is kept so unknown fields survive; see
/// [`Status`](crate::data::motd::Status) for building one.
#[packet(Status, 0x00, Clientbound)]
pub struct StatusPacket<'p> {
    pub json: Cow<'p, str>,
}

#[packet(Status, 0x01, Clientbound)]
pub struct PingResponse {
    pub time: i64,
}

#[packet(Login, 0x00, Clientbound)]
pub struct LoginDisconnect<'p> {
    pub reason: Cow<'p, str>,
}

#[packet(Login, 0x01, Clientbound)]
pub struct EncryptionRequest<'p> {
    pub server_id: Cow<'p, str>,
    pub public_key: Cow<'p, [u8]>,
    pub verify_token: Cow<'p, [u8]>,
    pub should_authenticate: bool,
}

#[packet(Login, 0x02, Clientbound)]
pub struct LoginSuccess<'p> {
    pub profile: GameProfile<'p>,
    pub session_id: Uuid,
}

#[packet(Login, 0x03, Clientbound)]
pub struct SetCompression {
    #[format = "varint"]
    pub threshold: i32,
}

#[packet(Login, 0x04, Clientbound)]
pub struct LoginPluginRequest<'p> {
    #[format = "varint"]
    pub message_id: i32,
    pub channel: Cow<'p, str>,
    pub data: Cow<'p, [u8]>,
}

#[packet(Login, 0x05, Clientbound)]
pub struct LoginCookieRequest<'p> {
    pub key: Cow<'p, str>,
}

#[packet(Config, 0x00, Clientbound)]
pub struct ConfigCookieRequest<'p> {
    pub key: Cow<'p, str>,
}

#[packet(Config, 0x01, Clientbound)]
pub struct PluginMessage<'p> {
    pub ident: Cow<'p, str>,
    pub data: Remainder<'p>,
}

#[packet(Config, 0x02, Clientbound)]
pub struct ConfigDisconnect<'p> {
    pub reason: Nbt<'p>,
}

#[packet(Play, 0x20, Clientbound)]
pub struct PlayDisconnect<'p> {
    pub reason: Nbt<'p>,
}

#[packet(Config, 0x04, Clientbound)]
pub struct ConfigKeepAlive {
    pub id: i64,
}

#[packet(Config, 0x06, Clientbound)]
pub struct ResetChat;

#[packet(Config, 0x14, Clientbound)]
pub struct CodeOfConduct<'p> {
    pub coc: Cow<'p, str>,
}

// NOTE: unused by vanilla server
// #[packet(Config, 0x05, Clientbound)]
// pub struct ConfigPing {
//     pub id: i32,
// }

#[packet(Config, 0x08, Clientbound)]
pub struct RemoveResourcePack {
    pub uuid: Option<Uuid>,
}

#[packet(Config, 0x0B, Clientbound)]
pub struct StoreCookie<'p> {
    pub key: Cow<'p, str>,
    pub payload: Cow<'p, [u8]>,
}

// TODO: enforce max sizes in parsing

#[packet(Config, 0x0C, Clientbound)]
pub struct Transfer<'p> {
    pub host: Cow<'p, str>,
    #[format = "varint"]
    pub port: i32,
}

#[packet(Config, 0x12, Clientbound)]
pub struct ClearDialog;

#[packet(Play, 0x00, Clientbound)]
pub struct BundleDelimiter;

#[packet(Play, 0x04, Clientbound)]
pub struct AcknowledgeBlockChange {
    #[format = "varint"]
    pub sequence_id: i32,
}

#[packet(Play, 0x0B, Clientbound)]
pub struct ChunkBatchFinished {
    #[format = "varint"]
    pub batch_size: i32,
}

#[packet(Play, 0x0C, Clientbound)]
pub struct ChunkBatchStart;

#[packet(Play, 0x26, Clientbound)]
pub struct UnloadChunk {
    pub chunk_z: i32,
    pub chunk_x: i32,
}

#[packet(Play, 0x2D, Clientbound)]
pub struct PlayKeepAlive {
    pub id: i64,
}

#[packet(Play, 0x27, Clientbound)]
pub struct GameEvent {
    pub event: u8,
    pub value: f32,
}

#[packet(Play, 0x5F, Clientbound)]
pub struct SetCamera {
    #[format = "varint"]
    pub camera_id: i32,
}

#[packet(Play, 0x41, Clientbound)]
pub struct PlayerAbilities {
    pub flags: u8,
    pub flying_speed: f32,
    pub fov_modifier: f32,
}

#[packet(Play, 0x63, Clientbound)]
pub struct SetDefaultSpawnPosition<'p> {
    pub dimension_name: Cow<'p, str>,
    pub location: i64,
    pub yaw: f32,
    pub pitch: f32,
}

#[packet(Play, 0x6A, Clientbound)]
pub struct SetHealth {
    pub health: f32,
    #[format = "varint"]
    pub food: i32,
    pub saturation: f32,
}

#[packet(Play, 0x60, Clientbound)]
pub struct SetCenterChunk {
    #[format = "varint"]
    pub chunk_x: i32,
    #[format = "varint"]
    pub chunk_z: i32,
}
#[packet(Play, 0x49, Clientbound)]
pub struct SyncPlayerPosition {
    #[format = "varint"]
    pub teleport_id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub vel_z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: i32,
}

#[packet(Play, 0x2E, Clientbound)]
pub struct ChunkData<'p> {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub heightmaps: Vec<Heightmap>,
    pub data: Cow<'p, [u8]>,
    pub block_entities: Vec<BlockEntity<'p>>,
    pub light: LightData<'p>,
}

#[packet(Play, 0x0E, Clientbound)]
pub struct ClearTitles {
    pub reset: bool,
}

/// Shared presentation above and below the player list. Empty components clear it.
#[packet(Play, 0x7D, Clientbound)]
pub struct TabList<'p> {
    pub header: Nbt<'p>,
    pub footer: Nbt<'p>,
}

#[cfg(test)]
#[path = "../../tests/tests_packets_tab_list.rs"]
mod tab_list_tests;

#[packet(Play, 0x33, Clientbound)]
pub struct LowDiskSpaceWarning;

#[packet(Play, 0x43, Clientbound)]
pub struct EndCombat {
    #[format = "varint"]
    pub duration: i32,
}

#[packet(Play, 0x44, Clientbound)]
pub struct EnterCombat;

#[packet(Play, 0x78, Clientbound)]
pub struct StartConfiguration;

#[packet(Config, 0x0F, Clientbound)]
pub struct KnownPacks<'p> {
    pub packs: Vec<KnownPack<'p>>,
}

#[packet(Config, 0x07, Clientbound)]
pub struct RegistryData<'p> {
    pub registry: Cow<'p, str>,
    pub entries: Vec<RegistryEntry<'p>>,
}

#[packet(Config, 0x0D, Clientbound)]
pub struct FeatureFlags<'p> {
    pub flags: Vec<Cow<'p, str>>,
}

#[packet(Config, 0x0E, Clientbound)]
pub struct UpdateTags<'p> {
    pub registries: Vec<TaggedRegistry<'p>>,
}

#[packet(Config, 0x03, Clientbound)]
pub struct FinishConfiguration;
