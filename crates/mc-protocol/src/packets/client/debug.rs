//! Clientbound debugging, game test and data query packets for protocol 777.
//!
//! Debug values depend on the subscription; they are kept as raw bytes.
use mc_protocol_macros::packet;

use crate::types::bytes::Remainder;
use crate::types::{Nbt, OptionalNbt, Vec3};

/// A block's value for a debug subscription the client requested.
#[packet(Play, 0x1A, Clientbound)]
pub struct DebugBlockValue<'p> {
    pub location: i64,
    /// ID in the `minecraft:debug_subscription` registry.
    #[format = "varint"]
    pub subscription: i32,
    /// The optional, subscription-specific value; not decoded.
    pub value: Remainder<'p>,
}

#[packet(Play, 0x1B, Clientbound)]
pub struct DebugChunkValue<'p> {
    pub chunk_z: i32,
    pub chunk_x: i32,
    #[format = "varint"]
    pub subscription: i32,
    /// The optional, subscription-specific value; not decoded.
    pub value: Remainder<'p>,
}

#[packet(Play, 0x1C, Clientbound)]
pub struct DebugEntityValue<'p> {
    #[format = "varint"]
    pub entity_id: i32,
    #[format = "varint"]
    pub subscription: i32,
    /// The optional, subscription-specific value; not decoded.
    pub value: Remainder<'p>,
}

#[packet(Play, 0x1D, Clientbound)]
pub struct DebugEvent<'p> {
    #[format = "varint"]
    pub subscription: i32,
    /// The subscription-specific value; not decoded.
    pub value: Remainder<'p>,
}

/// `RemoteDebugSampleType` ordinals.
pub mod debug_sample {
    /// Full, server, tasks and idle tick time, in nanoseconds.
    pub const TICK_TIME: i32 = 0;
}

#[packet(Play, 0x1E, Clientbound)]
pub struct DebugSample {
    pub sample: Vec<i64>,
    /// A [`debug_sample`] value.
    #[format = "varint"]
    pub kind: i32,
}

#[packet(Play, 0x29, Clientbound)]
pub struct GameTestHighlightPosition {
    pub absolute: i64,
    pub relative: i64,
}

#[packet(Play, 0x81, Clientbound)]
pub struct TestInstanceBlockStatus<'p> {
    pub status: Nbt<'p>,
    pub size: Option<Vec3>,
}

/// Answers Query Block Entity Tag or Query Entity Tag.
#[packet(Play, 0x7E, Clientbound)]
pub struct TagQueryResponse<'p> {
    #[format = "varint"]
    pub transaction_id: i32,
    pub nbt: OptionalNbt<'p>,
}
