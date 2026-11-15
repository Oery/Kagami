//! Serverbound operator, debugging and game test tools for protocol 777:
//! command, jigsaw, structure and test blocks, game rules and data queries.
use std::borrow::Cow;

use mc_protocol_macros::packet;

use crate::packets::client::GameRule;
use crate::types::{Nbt, VarInt};

/// The data of a block entity, for the debug screen; answered by Tag Query Response.
#[packet(Play, 0x02, Serverbound)]
pub struct QueryBlockEntityTag {
    #[format = "varint"]
    pub transaction_id: i32,
    pub location: i64,
}

#[packet(Play, 0x19, Serverbound)]
pub struct QueryEntityTag {
    #[format = "varint"]
    pub transaction_id: i32,
    #[format = "varint"]
    pub entity_id: i32,
}

/// Replaces the client's debug subscriptions.
#[packet(Play, 0x17, Serverbound)]
pub struct DebugSubscriptionRequest {
    /// IDs in the `minecraft:debug_subscription` registry.
    pub subscriptions: Vec<VarInt>,
}

/// Command block flags.
pub mod command_block_flags {
    pub const TRACK_OUTPUT: u8 = 0x01;
    pub const CONDITIONAL: u8 = 0x02;
    pub const AUTOMATIC: u8 = 0x04;
}

/// `CommandBlockEntity.Mode` ordinals.
pub mod command_block_mode {
    pub const CHAIN: i32 = 0;
    pub const REPEATING: i32 = 1;
    pub const IMPULSE: i32 = 2;
}

#[packet(Play, 0x37, Serverbound)]
pub struct ProgramCommandBlock<'p> {
    pub location: i64,
    pub command: Cow<'p, str>,
    /// A [`command_block_mode`] value.
    #[format = "varint"]
    pub mode: i32,
    /// [`command_block_flags`] bits.
    pub flags: u8,
}

#[packet(Play, 0x38, Serverbound)]
pub struct ProgramCommandBlockMinecart<'p> {
    #[format = "varint"]
    pub entity_id: i32,
    pub command: Cow<'p, str>,
    pub track_output: bool,
}

#[packet(Play, 0x3A, Serverbound)]
pub struct SetGameRules<'p> {
    pub rules: Vec<GameRule<'p>>,
}

#[packet(Play, 0x1B, Serverbound)]
pub struct JigsawGenerate {
    pub location: i64,
    #[format = "varint"]
    pub levels: i32,
    pub keep_jigsaws: bool,
}

#[packet(Play, 0x3B, Serverbound)]
pub struct ProgramJigsawBlock<'p> {
    pub location: i64,
    pub name: Cow<'p, str>,
    pub target: Cow<'p, str>,
    pub pool: Cow<'p, str>,
    /// The block state the jigsaw turns into.
    pub final_state: Cow<'p, str>,
    /// `rollable` or `aligned`.
    pub joint_type: Cow<'p, str>,
    #[format = "varint"]
    pub selection_priority: i32,
    #[format = "varint"]
    pub placement_priority: i32,
}

/// Structure block flags.
pub mod structure_block_flags {
    pub const IGNORE_ENTITIES: u8 = 0x01;
    pub const SHOW_AIR: u8 = 0x02;
    pub const SHOW_BOUNDING_BOX: u8 = 0x04;
    pub const STRICT_PLACEMENT: u8 = 0x08;
}

#[packet(Play, 0x3C, Serverbound)]
pub struct ProgramStructureBlock<'p> {
    pub location: i64,
    /// What to do besides saving the settings: 0 update, 1 save, 2 load, 3 detect size.
    #[format = "varint"]
    pub action: i32,
    /// 0 save, 1 load, 2 corner, 3 data.
    #[format = "varint"]
    pub mode: i32,
    pub name: Cow<'p, str>,
    pub offset_x: i8,
    pub offset_y: i8,
    pub offset_z: i8,
    pub size_x: i8,
    pub size_y: i8,
    pub size_z: i8,
    /// 0 none, 1 left-right, 2 front-back.
    #[format = "varint"]
    pub mirror: i32,
    /// Clockwise quarter turns, 0 to 3.
    #[format = "varint"]
    pub rotation: i32,
    pub metadata: Cow<'p, str>,
    /// 0 to 1.
    pub integrity: f32,
    #[format = "varlong"]
    pub seed: i64,
    /// [`structure_block_flags`] bits.
    pub flags: u8,
}

/// A game test block: 0 start, 1 log, 2 fail, 3 accept.
#[packet(Play, 0x3D, Serverbound)]
pub struct SetTestBlock<'p> {
    pub location: i64,
    #[format = "varint"]
    pub mode: i32,
    pub message: Cow<'p, str>,
}

/// A test instance block: init, query, set, reset, save, export or run (0 to 6).
#[packet(Play, 0x41, Serverbound)]
pub struct TestInstanceBlockAction<'p> {
    pub location: i64,
    #[format = "varint"]
    pub action: i32,
    /// ID in the `minecraft:test_instance` registry.
    pub test: Option<Cow<'p, str>>,
    #[format = "varint"]
    pub size_x: i32,
    #[format = "varint"]
    pub size_y: i32,
    #[format = "varint"]
    pub size_z: i32,
    /// Clockwise quarter turns, 0 to 3.
    #[format = "varint"]
    pub rotation: i32,
    pub ignore_entities: bool,
    /// 0 cleared, 1 running, 2 finished.
    #[format = "varint"]
    pub status: i32,
    pub error_message: Option<Nbt<'p>>,
}
