//! Clientbound chat for protocol 777: player and disguised messages, deletion,
//! the action bar and chat suggestions.
//!
//! Signatures are carried as received; nothing here verifies them.
use std::borrow::Cow;

use mc_protocol_macros::{IntoOwned, packet};
use uuid::Uuid;

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::error::DecodeError;
use crate::packets::PacketWriter;
use crate::packets::server::Signature;
use crate::types::{IdOr, Nbt, VarInt};

/// How the client renders one use of a chat type: a translation whose
/// arguments are the listed parameters.
#[derive(Debug, IntoOwned)]
pub struct ChatTypeDecoration<'p> {
    pub translation_key: Cow<'p, str>,
    /// [`chat_parameter`] values, in argument order.
    pub parameters: Vec<VarInt>,
    pub style: Nbt<'p>,
}

/// `ChatTypeDecoration` parameter ordinals.
pub mod chat_parameter {
    pub const SENDER: i32 = 0;
    pub const TARGET: i32 = 1;
    pub const CONTENT: i32 = 2;
}

impl<'p> PacketWriter<'p> for ChatTypeDecoration<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let translation_key = Cow::read(input)?;
        let parameters = Vec::read(input)?;
        let style = Nbt::read(input)?;

        Ok(Self { translation_key, parameters, style })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.translation_key.encode(out)?;
        self.parameters.encode(out)?;

        self.style.encode(out)
    }

    fn body_len(&self) -> usize {
        self.translation_key.body_len() + self.parameters.body_len() + self.style.body_len()
    }
}

/// An inline entry of the `minecraft:chat_type` registry.
#[derive(Debug, IntoOwned)]
pub struct ChatType<'p> {
    pub chat: ChatTypeDecoration<'p>,
    pub narration: ChatTypeDecoration<'p>,
}

impl<'p> PacketWriter<'p> for ChatType<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let chat = ChatTypeDecoration::read(input)?;
        let narration = ChatTypeDecoration::read(input)?;

        Ok(Self { chat, narration })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.chat.encode(out)?;
        self.narration.encode(out)
    }

    fn body_len(&self) -> usize {
        self.chat.body_len() + self.narration.body_len()
    }
}

/// Which characters of a player message the server filtered.
#[derive(Clone, Debug, PartialEq, Eq, IntoOwned)]
pub enum FilterMask {
    PassThrough,
    FullyFiltered,
    /// A BitSet of the character indices the client shows as `#`.
    PartiallyFiltered(Vec<i64>),
}

impl PacketWriter<'_> for FilterMask {
    fn read(input: &mut &[u8]) -> DecodeResult<Self> {
        match VarInt::read(input)?.0 {
            0 => Ok(Self::PassThrough),
            1 => Ok(Self::FullyFiltered),
            2 => Ok(Self::PartiallyFiltered(Vec::read(input)?)),
            _ => Err(DecodeError::EnumOutOfRange),
        }
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        match self {
            Self::PassThrough => VarInt(0).encode(out),
            Self::FullyFiltered => VarInt(1).encode(out),
            Self::PartiallyFiltered(bits) => {
                VarInt(2).encode(out)?;
                bits.encode(out)
            }
        }
    }

    fn body_len(&self) -> usize {
        match self {
            Self::PassThrough | Self::FullyFiltered => 1,
            Self::PartiallyFiltered(bits) => 1 + bits.body_len(),
        }
    }
}

/// A message another player sent, as the server relays it.
///
/// A previous message is either an ID in the client's cache of recent
/// signatures or the full signature.
#[packet(Play, 0x42, Clientbound)]
pub struct PlayerChat<'p> {
    /// Counts every chat message sent to this client.
    #[format = "varint"]
    pub global_index: i32,
    pub sender: Uuid,
    /// Counts the sender's messages.
    #[format = "varint"]
    pub index: i32,
    pub signature: Option<Signature<'p>>,
    pub message: Cow<'p, str>,
    /// Milliseconds since the Unix epoch.
    pub timestamp: i64,
    pub salt: i64,
    pub previous_messages: Vec<IdOr<Signature<'p>>>,
    pub unsigned_content: Option<Nbt<'p>>,
    pub filter: FilterMask,
    pub chat_type: IdOr<ChatType<'p>>,
    pub sender_name: Nbt<'p>,
    pub target_name: Option<Nbt<'p>>,
}

/// A message formatted like player chat but without a signature, such as `/say`.
#[packet(Play, 0x21, Clientbound)]
pub struct DisguisedChat<'p> {
    pub message: Nbt<'p>,
    pub chat_type: IdOr<ChatType<'p>>,
    pub sender_name: Nbt<'p>,
    pub target_name: Option<Nbt<'p>>,
}

/// Hides a signed message, by cache ID or by its full signature.
#[packet(Play, 0x1F, Clientbound)]
pub struct DeleteMessage<'p> {
    pub signature: IdOr<Signature<'p>>,
}

#[packet(Play, 0x59, Clientbound)]
pub struct SetActionBarText<'p> {
    pub text: Nbt<'p>,
}

/// `ChatSuggestions` actions.
pub mod chat_suggestions_action {
    pub const ADD: i32 = 0;
    pub const REMOVE: i32 = 1;
    pub const SET: i32 = 2;
}

/// Extra names the client offers when completing chat, such as for mentions.
#[packet(Play, 0x17, Clientbound)]
pub struct ChatSuggestions<'p> {
    /// A [`chat_suggestions_action`] value.
    #[format = "varint"]
    pub action: i32,
    pub entries: Vec<Cow<'p, str>>,
}
