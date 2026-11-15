// Lets generated packet code use `::mc_protocol` paths inside this crate too.
extern crate self as mc_protocol;

pub mod auth;
pub mod data;
pub mod decode;
pub mod encode;
pub mod error;
pub mod packets;
pub mod state;
pub mod types;

pub use crate::packets::{PacketWriter, Payload};
pub use crate::state::State;
pub use crate::types::VarInt;

/// The Minecraft Java Edition wire protocol implemented by this crate.
pub const PROTOCOL_VERSION: i32 = 777;
pub const MINECRAFT_VERSION: &str = "26.3";
