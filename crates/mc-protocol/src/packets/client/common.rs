//! Clientbound packets sent in both configuration and play, for protocol 777.
//!
//! Each exists once per state, with its own id. Play copies of packets the
//! configuration state already had are prefixed `Play`.
use std::borrow::Cow;

use mc_protocol_macros::{IntoOwned, packet};
use uuid::Uuid;

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::packets::PacketWriter;
use crate::types::bytes::Remainder;
use crate::types::{IdOr, Nbt, TaggedRegistry, VarInt};

/// Answered by a Pong with the same id. Unused by the vanilla server.
#[packet(Config, 0x05, Clientbound)]
pub struct ConfigPing {
    pub id: i32,
}

#[packet(Play, 0x3E, Clientbound)]
pub struct PlayPing {
    pub id: i32,
}

/// Answers the client's Ping Request in play with the same time.
#[packet(Play, 0x3F, Clientbound)]
pub struct PlayPingResponse {
    pub time: i64,
}

#[packet(Play, 0x15, Clientbound)]
pub struct PlayCookieRequest<'p> {
    pub key: Cow<'p, str>,
}

#[packet(Play, 0x7A, Clientbound)]
pub struct PlayStoreCookie<'p> {
    pub key: Cow<'p, str>,
    pub payload: Cow<'p, [u8]>,
}

#[packet(Play, 0x18, Clientbound)]
pub struct PlayPluginMessage<'p> {
    pub channel: Cow<'p, str>,
    pub data: Remainder<'p>,
}

/// The prompt is shown when the client asks the player to accept the pack.
#[packet(Config, 0x09, Clientbound)]
pub struct ConfigAddResourcePack<'p> {
    pub uuid: Uuid,
    pub url: Cow<'p, str>,
    /// Hexadecimal SHA-1 of the pack; the client skips verification unless
    /// it is 40 hex digits.
    pub hash: Cow<'p, str>,
    pub forced: bool,
    pub prompt: Option<Nbt<'p>>,
}

#[packet(Play, 0x52, Clientbound)]
pub struct PlayAddResourcePack<'p> {
    pub uuid: Uuid,
    pub url: Cow<'p, str>,
    pub hash: Cow<'p, str>,
    pub forced: bool,
    pub prompt: Option<Nbt<'p>>,
}

/// `None` removes every server resource pack.
#[packet(Play, 0x51, Clientbound)]
pub struct PlayRemoveResourcePack {
    pub uuid: Option<Uuid>,
}

/// Post-processing shaders to activate, by identifier.
#[packet(Config, 0x0A, Clientbound)]
pub struct ConfigPostEffects<'p> {
    pub effects: Vec<Cow<'p, str>>,
}

#[packet(Play, 0x53, Clientbound)]
pub struct PlayPostEffects<'p> {
    pub effects: Vec<Cow<'p, str>>,
}

#[packet(Play, 0x84, Clientbound)]
pub struct PlayTransfer<'p> {
    pub host: Cow<'p, str>,
    #[format = "varint"]
    pub port: i32,
}

#[packet(Play, 0x89, Clientbound)]
pub struct PlayUpdateTags<'p> {
    pub registries: Vec<TaggedRegistry<'p>>,
}

/// One entry of a crash or disconnect report the server adds.
#[derive(Clone, Debug, PartialEq, IntoOwned)]
pub struct ReportDetail<'p> {
    pub title: Cow<'p, str>,
    pub description: Cow<'p, str>,
}

impl<'p> PacketWriter<'p> for ReportDetail<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let title = Cow::read(input)?;
        let description = Cow::read(input)?;

        Ok(Self { title, description })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.title.encode(out)?;
        self.description.encode(out)
    }

    fn body_len(&self) -> usize {
        self.title.body_len() + self.description.body_len()
    }
}

#[packet(Config, 0x10, Clientbound)]
pub struct ConfigCustomReportDetails<'p> {
    pub details: Vec<ReportDetail<'p>>,
}

#[packet(Play, 0x8B, Clientbound)]
pub struct PlayCustomReportDetails<'p> {
    pub details: Vec<ReportDetail<'p>>,
}

/// Built-in server link labels, translated by the client.
pub mod server_link {
    pub const BUG_REPORT: i32 = 0;
    pub const COMMUNITY_GUIDELINES: i32 = 1;
    pub const SUPPORT: i32 = 2;
    pub const STATUS: i32 = 3;
    pub const FEEDBACK: i32 = 4;
    pub const COMMUNITY: i32 = 5;
    pub const WEBSITE: i32 = 6;
    pub const FORUMS: i32 = 7;
    pub const NEWS: i32 = 8;
    pub const ANNOUNCEMENTS: i32 = 9;
}

#[derive(Clone, Debug, PartialEq, IntoOwned)]
pub enum ServerLinkLabel<'p> {
    /// One of the [`server_link`] labels.
    BuiltIn(i32),
    Custom(Nbt<'p>),
}

impl<'p> PacketWriter<'p> for ServerLinkLabel<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let is_built_in = bool::read(input)?;
        if is_built_in {
            return Ok(Self::BuiltIn(VarInt::read(input)?.0));
        }

        Ok(Self::Custom(Nbt::read(input)?))
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        match self {
            Self::BuiltIn(label) => {
                true.encode(out)?;
                VarInt(*label).encode(out)
            }
            Self::Custom(text) => {
                false.encode(out)?;
                text.encode(out)
            }
        }
    }

    fn body_len(&self) -> usize {
        let label = match self {
            Self::BuiltIn(label) => VarInt(*label).body_len(),
            Self::Custom(text) => text.body_len(),
        };

        1 + label
    }
}

#[derive(Clone, Debug, PartialEq, IntoOwned)]
pub struct ServerLink<'p> {
    pub label: ServerLinkLabel<'p>,
    pub url: Cow<'p, str>,
}

impl<'p> PacketWriter<'p> for ServerLink<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let label = ServerLinkLabel::read(input)?;
        let url = Cow::read(input)?;

        Ok(Self { label, url })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.label.encode(out)?;
        self.url.encode(out)
    }

    fn body_len(&self) -> usize {
        self.label.body_len() + self.url.body_len()
    }
}

/// Links shown in the pause menu and on the disconnect screen.
#[packet(Config, 0x11, Clientbound)]
pub struct ConfigServerLinks<'p> {
    pub links: Vec<ServerLink<'p>>,
}

#[packet(Play, 0x8C, Clientbound)]
pub struct PlayServerLinks<'p> {
    pub links: Vec<ServerLink<'p>>,
}

/// An inline dialog definition, in the format of the `minecraft:dialog` registry.
#[packet(Config, 0x13, Clientbound)]
pub struct ConfigShowDialog<'p> {
    pub dialog: Nbt<'p>,
}

/// In play the dialog may also name an entry of the `minecraft:dialog` registry.
#[packet(Play, 0x8F, Clientbound)]
pub struct PlayShowDialog<'p> {
    pub dialog: IdOr<Nbt<'p>>,
}

#[packet(Play, 0x8E, Clientbound)]
pub struct PlayClearDialog;
