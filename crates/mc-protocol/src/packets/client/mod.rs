use crate::types::{BlockEntity, GameProfile, Heightmap, KnownPack, LightData, Nbt, RegistryEntry, TaggedRegistry, bytes::Remainder};

use mc_protocol_macros::packet;
use uuid::Uuid;

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
#[packet(Status, 0x00, Client)]
pub struct StatusPacket<'p> {
    pub json: &'p str,
}

#[packet(Status, 0x01, Client)]
pub struct PingResponse {
    pub time: i64,
}

#[packet(Login, 0x00, Client)]
pub struct LoginDisconnect<'p> {
    pub reason: &'p str,
}

#[packet(Login, 0x01, Client)]
pub struct EncryptionRequest<'p> {
    pub server_id: &'p str,
    pub public_key: &'p [u8],
    pub verify_token: &'p [u8],
    pub should_authenticate: bool,
}

#[packet(Login, 0x02, Client)]
pub struct LoginSuccess<'p> {
    pub profile: GameProfile<'p>,
    pub session_id: Uuid,
}

#[packet(Login, 0x03, Client)]
pub struct SetCompression {
    #[format = "varint"]
    pub threshold: i32,
}

#[packet(Login, 0x04, Client)]
pub struct LoginPluginRequest<'p> {
    #[format = "varint"]
    pub message_id: i32,
    pub channel: &'p str,
    pub data: &'p [u8],
}

#[packet(Login, 0x05, Client)]
pub struct LoginCookieRequest<'p> {
    pub key: &'p str,
}

#[packet(Config, 0x00, Client)]
pub struct ConfigCookieRequest<'p> {
    pub key: &'p str,
}

#[packet(Config, 0x01, Client)]
pub struct PluginMessage<'p> {
    pub ident: &'p str,
    pub data: Remainder<'p>,
}

#[packet(Config, 0x02, Client)]
pub struct ConfigDisconnect<'p> {
    pub reason: Nbt<'p>,
}

#[packet(Play, 0x20, Client)]
pub struct PlayDisconnect<'p> {
    pub reason: Nbt<'p>,
}

#[packet(Config, 0x04, Client)]
pub struct ConfigKeepAlive {
    pub id: i64,
}

#[packet(Config, 0x06, Client)]
pub struct ResetChat;

#[packet(Config, 0x14, Client)]
pub struct CodeOfConduct<'p> {
    pub coc: &'p str,
}

// NOTE: unused by vanilla server
// #[packet(Config, 0x05, Client)]
// pub struct ConfigPing {
//     pub id: i32,
// }

#[packet(Config, 0x08, Client)]
pub struct RemoveResourcePack {
    pub uuid: Option<Uuid>,
}

#[packet(Config, 0x0B, Client)]
pub struct StoreCookie<'p> {
    pub key: &'p str,
    pub payload: &'p [u8],
}

// TODO: enforce max sizes in parsing

#[packet(Config, 0x0C, Client)]
pub struct Transfer<'p> {
    pub host: &'p str,
    #[format = "varint"]
    pub port: i32,
}

#[packet(Config, 0x12, Client)]
pub struct ClearDialog;

#[packet(Play, 0x00, Client)]
pub struct BundleDelimiter;

#[packet(Play, 0x04, Client)]
pub struct AcknowledgeBlockChange {
    #[format = "varint"]
    pub sequence_id: i32,
}

#[packet(Play, 0x0B, Client)]
pub struct ChunkBatchFinished {
    #[format = "varint"]
    pub batch_size: i32,
}

#[packet(Play, 0x0C, Client)]
pub struct ChunkBatchStart;

#[packet(Play, 0x26, Client)]
pub struct UnloadChunk {
    pub chunk_z: i32,
    pub chunk_x: i32,
}

#[packet(Play, 0x2D, Client)]
pub struct PlayKeepAlive {
    pub id: i64,
}

#[packet(Play, 0x27, Client)]
pub struct GameEvent {
    pub event: u8,
    pub value: f32,
}

#[packet(Play, 0x5F, Client)]
pub struct SetCamera {
    #[format = "varint"]
    pub camera_id: i32,
}

#[packet(Play, 0x41, Client)]
pub struct PlayerAbilities {
    pub flags: u8,
    pub flying_speed: f32,
    pub fov_modifier: f32,
}

#[packet(Play, 0x63, Client)]
pub struct SetDefaultSpawnPosition<'p> {
    pub dimension_name: &'p str,
    pub location: i64,
    pub yaw: f32,
    pub pitch: f32,
}

#[packet(Play, 0x6A, Client)]
pub struct SetHealth {
    pub health: f32,
    #[format = "varint"]
    pub food: i32,
    pub saturation: f32,
}

#[packet(Play, 0x60, Client)]
pub struct SetCenterChunk {
    #[format = "varint"]
    pub chunk_x: i32,
    #[format = "varint"]
    pub chunk_z: i32,
}
#[packet(Play, 0x49, Client)]
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

#[packet(Play, 0x2E, Client)]
pub struct ChunkData<'p> {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub heightmaps: Vec<Heightmap>,
    pub data: &'p [u8],
    pub block_entities: Vec<BlockEntity<'p>>,
    pub light: LightData<'p>,
}

#[packet(Play, 0x0E, Client)]
pub struct ClearTitles {
    pub reset: bool,
}

/// Shared presentation above and below the player list. Empty components clear it.
#[packet(Play, 0x7D, Client)]
pub struct TabList<'p> {
    pub header: Nbt<'p>,
    pub footer: Nbt<'p>,
}

#[cfg(test)]
#[path = "../../tests/tests_packets_tab_list.rs"]
mod tab_list_tests;

#[packet(Play, 0x33, Client)]
pub struct LowDiskSpaceWarning;

#[packet(Play, 0x43, Client)]
pub struct EndCombat {
    #[format = "varint"]
    pub duration: i32,
}

#[packet(Play, 0x44, Client)]
pub struct EnterCombat;

#[packet(Play, 0x78, Client)]
pub struct StartConfiguration;

#[packet(Config, 0x0F, Client)]
pub struct KnownPacks<'p> {
    pub packs: Vec<KnownPack<'p>>,
}

#[packet(Config, 0x07, Client)]
pub struct RegistryData<'p> {
    pub registry: &'p str,
    pub entries: Vec<RegistryEntry<'p>>,
}

#[packet(Config, 0x0D, Client)]
pub struct FeatureFlags<'p> {
    pub flags: Vec<&'p str>,
}

#[packet(Config, 0x0E, Client)]
pub struct UpdateTags<'p> {
    pub registries: Vec<TaggedRegistry<'p>>,
}

#[packet(Config, 0x03, Client)]
pub struct FinishConfiguration;
