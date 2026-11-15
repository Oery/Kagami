//! The Play login packet.

use mc_protocol_macros::packet;

use super::DeathLocation;

#[packet(Play, 0x32, Client)]
pub struct PlayLogin<'p> {
    pub entity_id: i32,
    pub hardcore: bool,
    pub dimension_names: Vec<&'p str>,
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
    pub dimension_name: &'p str,
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
