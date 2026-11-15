//! The Play login packet and the state of this client's player.
use std::borrow::Cow;

use mc_protocol_macros::{IntoOwned, packet};

use super::DeathLocation;
use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::packets::PacketWriter;
use crate::types::VarInt;

#[packet(Play, 0x32, Clientbound)]
pub struct PlayLogin<'p> {
    pub entity_id: i32,
    pub hardcore: bool,
    pub dimension_names: Vec<Cow<'p, str>>,
    #[format = "varint"]
    pub max_players: i32,
    #[format = "varint"]
    pub view_distance: i32,
    #[format = "varint"]
    pub simulation_distance: i32,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub do_limited_crafting: bool,
    #[format = "varint"]
    pub dimension_type: i32,
    pub dimension_name: Cow<'p, str>,
    pub hashed_seed: i64,
    #[format = "varint"]
    pub game_mode: i32,
    #[format = "varint"]
    pub previous_game_mode: i32,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death_location: Option<DeathLocation<'p>>,
    #[format = "varint"]
    pub portal_cooldown: i32,
    #[format = "varint"]
    pub sea_level: i32,
    pub online_mode: bool,
    pub enforces_secure_chat: bool,
}

/// `EntityAnchorArgument.Anchor` ordinals: which point of an entity looks or is looked at.
pub mod entity_anchor {
    pub const FEET: i32 = 0;
    pub const EYES: i32 = 1;
}

#[derive(Debug, IntoOwned)]
pub struct LookAtEntity {
    pub entity_id: i32,
    /// An [`entity_anchor`] value.
    pub anchor: i32,
}

impl PacketWriter<'_> for LookAtEntity {
    fn read(input: &mut &[u8]) -> DecodeResult<Self> {
        let entity_id = VarInt::read(input)?.0;
        let anchor = VarInt::read(input)?.0;

        Ok(Self { entity_id, anchor })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        VarInt(self.entity_id).encode(out)?;
        VarInt(self.anchor).encode(out)
    }

    fn body_len(&self) -> usize {
        VarInt(self.entity_id).body_len() + VarInt(self.anchor).body_len()
    }
}

/// Turns the player to face a point, or an entity when one is given.
#[packet(Play, 0x48, Clientbound)]
pub struct LookAt {
    /// The [`entity_anchor`] of this player that aims.
    #[format = "varint"]
    pub anchor: i32,
    pub target_x: f64,
    pub target_y: f64,
    pub target_z: f64,
    pub entity: Option<LookAtEntity>,
}

/// Degrees; each angle is absolute unless flagged relative.
#[packet(Play, 0x4A, Clientbound)]
pub struct PlayerRotation {
    pub yaw: f32,
    pub relative_yaw: bool,
    pub pitch: f32,
    pub relative_pitch: bool,
}

#[packet(Play, 0x69, Clientbound)]
pub struct SetExperience {
    /// Progress to the next level, 0 to 1.
    pub progress: f32,
    #[format = "varint"]
    pub level: i32,
    #[format = "varint"]
    pub total: i32,
}

#[derive(Debug, IntoOwned)]
pub struct Statistic {
    /// ID in the `minecraft:stat_type` registry.
    pub category: i32,
    /// ID in the registry the category counts, such as blocks or items.
    pub statistic: i32,
    pub value: i32,
}

impl PacketWriter<'_> for Statistic {
    fn read(input: &mut &[u8]) -> DecodeResult<Self> {
        let category = VarInt::read(input)?.0;
        let statistic = VarInt::read(input)?.0;
        let value = VarInt::read(input)?.0;

        Ok(Self { category, statistic, value })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        VarInt(self.category).encode(out)?;
        VarInt(self.statistic).encode(out)?;

        VarInt(self.value).encode(out)
    }

    fn body_len(&self) -> usize {
        VarInt(self.category).body_len() + VarInt(self.statistic).body_len() + VarInt(self.value).body_len()
    }
}

/// Answers the statistics request of Client Status.
#[packet(Play, 0x03, Clientbound)]
pub struct AwardStatistics {
    pub statistics: Vec<Statistic>,
}
