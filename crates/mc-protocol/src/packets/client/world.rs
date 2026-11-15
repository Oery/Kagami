//! Clientbound world and server state for protocol 777: blocks, light,
//! biomes, explosions, the world border, time, game rules and tick rate.
use std::borrow::Cow;

use mc_protocol_macros::{IntoOwned, packet};

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::packets::PacketWriter;
use crate::types::bytes::Remainder;
use crate::types::{LightData, Nbt, VarInt, VarLong, Vec3};

/// `Difficulty` ordinals.
pub mod difficulty {
    pub const PEACEFUL: u8 = 0;
    pub const EASY: u8 = 1;
    pub const NORMAL: u8 = 2;
    pub const HARD: u8 = 3;
}

#[packet(Play, 0x0A, Clientbound)]
pub struct ChangeDifficulty {
    /// A [`difficulty`] value.
    pub difficulty: u8,
    pub locked: bool,
}

/// The MOTD and icon the client shows for this server.
#[packet(Play, 0x58, Clientbound)]
pub struct ServerData<'p> {
    pub motd: Nbt<'p>,
    /// PNG bytes.
    pub icon: Option<Cow<'p, [u8]>>,
}

/// Many block changes in one chunk section.
#[packet(Play, 0x56, Clientbound)]
pub struct UpdateSectionBlocks {
    /// Section x and z in 22 bits each, then y in 20, from the top bit down.
    pub section: i64,
    /// Each is a block state ID shifted left by 12, then x, z and y within
    /// the section in 4 bits each.
    pub blocks: Vec<VarLong>,
}

#[derive(Debug, IntoOwned)]
pub struct ChunkBiomeData<'p> {
    pub chunk_z: i32,
    pub chunk_x: i32,
    /// The chunk sections with their biome palettes only.
    pub data: Cow<'p, [u8]>,
}

impl<'p> PacketWriter<'p> for ChunkBiomeData<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let chunk_z = i32::read(input)?;
        let chunk_x = i32::read(input)?;
        let data = Cow::read(input)?;

        Ok(Self { chunk_z, chunk_x, data })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.chunk_z.encode(out)?;
        self.chunk_x.encode(out)?;

        self.data.encode(out)
    }

    fn body_len(&self) -> usize {
        8 + self.data.body_len()
    }
}

#[packet(Play, 0x0D, Clientbound)]
pub struct ChunkBiomes<'p> {
    pub chunks: Vec<ChunkBiomeData<'p>>,
}

#[packet(Play, 0x31, Clientbound)]
pub struct UpdateLight<'p> {
    #[format = "varint"]
    pub chunk_x: i32,
    #[format = "varint"]
    pub chunk_z: i32,
    pub light: LightData<'p>,
}

#[packet(Play, 0x24, Clientbound)]
pub struct Explosion<'p> {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub radius: f32,
    pub block_count: i32,
    /// Velocity added to this client's player.
    pub player_knockback: Option<Vec3>,
    /// The explosion particle, its sound, the weighted block particles and
    /// whether the sound plays; not decoded.
    pub effects: Remainder<'p>,
}

#[packet(Play, 0x2C, Clientbound)]
pub struct InitializeWorldBorder {
    pub center_x: f64,
    pub center_z: f64,
    pub old_diameter: f64,
    pub new_diameter: f64,
    /// Real-time milliseconds until `new_diameter` is reached.
    #[format = "varlong"]
    pub speed: i64,
    /// Portal destinations are clamped to plus or minus this.
    #[format = "varint"]
    pub portal_teleport_boundary: i32,
    #[format = "varint"]
    pub warning_blocks: i32,
    #[format = "varint"]
    pub warning_time: i32,
}

#[packet(Play, 0x5A, Clientbound)]
pub struct SetBorderCenter {
    pub x: f64,
    pub z: f64,
}

#[packet(Play, 0x5B, Clientbound)]
pub struct SetBorderLerpSize {
    pub old_diameter: f64,
    pub new_diameter: f64,
    /// Real-time milliseconds until `new_diameter` is reached.
    #[format = "varlong"]
    pub speed: i64,
}

#[packet(Play, 0x5C, Clientbound)]
pub struct SetBorderSize {
    pub diameter: f64,
}

/// Seconds.
#[packet(Play, 0x5D, Clientbound)]
pub struct SetBorderWarningDelay {
    #[format = "varint"]
    pub warning_time: i32,
}

#[packet(Play, 0x5E, Clientbound)]
pub struct SetBorderWarningDistance {
    #[format = "varint"]
    pub warning_blocks: i32,
}

/// In chunks.
#[packet(Play, 0x61, Clientbound)]
pub struct SetRenderDistance {
    #[format = "varint"]
    pub view_distance: i32,
}

/// In chunks.
#[packet(Play, 0x71, Clientbound)]
pub struct SetSimulationDistance {
    #[format = "varint"]
    pub simulation_distance: i32,
}

/// The state of one entry of the `minecraft:world_clock` registry.
#[derive(Debug, IntoOwned)]
pub struct ClockState {
    pub clock: i32,
    /// Ticks.
    pub time: i64,
    /// The fraction of a tick past `time`.
    pub partial_tick: f32,
    /// Clock ticks the client advances per client tick.
    pub rate: f32,
}

impl PacketWriter<'_> for ClockState {
    fn read(input: &mut &[u8]) -> DecodeResult<Self> {
        let clock = VarInt::read(input)?.0;
        let time = VarLong::read(input)?.0;
        let partial_tick = f32::read(input)?;
        let rate = f32::read(input)?;

        Ok(Self { clock, time, partial_tick, rate })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        VarInt(self.clock).encode(out)?;
        VarLong(self.time).encode(out)?;
        self.partial_tick.encode(out)?;

        self.rate.encode(out)
    }

    fn body_len(&self) -> usize {
        VarInt(self.clock).body_len() + VarLong(self.time).body_len() + 8
    }
}

#[packet(Play, 0x73, Clientbound)]
pub struct UpdateTime {
    /// Ticks; commands do not change it.
    pub world_age: i64,
    pub clocks: Vec<ClockState>,
}

/// A game rule and its value, as text.
#[derive(Debug, IntoOwned)]
pub struct GameRule<'p> {
    pub name: Cow<'p, str>,
    pub value: Cow<'p, str>,
}

impl<'p> PacketWriter<'p> for GameRule<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let name = Cow::read(input)?;
        let value = Cow::read(input)?;

        Ok(Self { name, value })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.name.encode(out)?;
        self.value.encode(out)
    }

    fn body_len(&self) -> usize {
        self.name.body_len() + self.value.body_len()
    }
}

/// Answers a game rule screen's query.
#[packet(Play, 0x28, Clientbound)]
pub struct GameRuleValues<'p> {
    pub rules: Vec<GameRule<'p>>,
}

#[packet(Play, 0x82, Clientbound)]
pub struct SetTickingState {
    /// Ticks per second.
    pub tick_rate: f32,
    pub frozen: bool,
}

/// Advances a frozen game by this many ticks.
#[packet(Play, 0x83, Clientbound)]
pub struct StepTick {
    #[format = "varint"]
    pub steps: i32,
}
