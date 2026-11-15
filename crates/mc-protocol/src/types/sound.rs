use std::borrow::Cow;

use mc_protocol_macros::IntoOwned;

use crate::packets::PacketWriter;

/// A sound defined inline rather than by its `minecraft:sound_event` ID.
#[derive(Clone, Debug, PartialEq, PacketWriter, IntoOwned)]
pub struct SoundEvent<'p> {
    pub location: Cow<'p, str>,
    /// The distance it is heard from; scaled by volume when absent.
    pub fixed_range: Option<f32>,
}
