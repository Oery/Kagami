//! Command trees with literals, words, numeric arguments and world coordinates.
use futures_lite::AsyncWrite;
use mc_protocol_macros::{Payload, packet};
use uuid::Uuid;

use crate::State;
use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::error::DecodeError;
use crate::packets::{PacketWriter, Payload};
use crate::types::{GameMode, Nbt, VarInt};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandNumberKind {
    Integer,
    Long,
    Float,
    Double,
}

impl CommandNumberKind {
    fn parser(self) -> i32 {
        match self {
            Self::Float => 1,
            Self::Double => 2,
            Self::Integer => 3,
            Self::Long => 4,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommandNodeKind<'p> {
    Root,
    Literal(&'p str),
    Word { name: &'p str, suggest: bool },
    GreedyText { name: &'p str },
    Number { name: &'p str, kind: CommandNumberKind },
    Position { name: &'p str },
}

#[derive(Debug)]
pub struct CommandNode<'p> {
    pub kind: CommandNodeKind<'p>,
    pub children: Vec<VarInt>,
    pub executable: bool,
}

impl<'p> PacketWriter<'p> for CommandNode<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let flags = u8::read(input)?;
        if flags & !0x17 != 0 {
            return Err(DecodeError::EnumOutOfRange);
        }
        let children = Vec::<VarInt>::read(input)?;
        let kind = match flags & 3 {
            0 if flags & 0x10 == 0 => CommandNodeKind::Root,
            1 if flags & 0x10 == 0 => CommandNodeKind::Literal(<&str>::read(input)?),
            2 => {
                let name = <&str>::read(input)?;
                match VarInt::read(input)?.0 {
                    5 => {
                        let mode = VarInt::read(input)?.0;
                        let suggest = flags & 0x10 != 0;
                        if mode == 2 && !suggest {
                            return Ok(Self { kind: CommandNodeKind::GreedyText { name }, children, executable: flags & 4 != 0 });
                        }
                        if mode != 0 {
                            return Err(DecodeError::EnumOutOfRange);
                        }
                        if suggest && <&str>::read(input)? != "minecraft:ask_server" {
                            return Err(DecodeError::EnumOutOfRange);
                        }
                        CommandNodeKind::Word { name, suggest }
                    }
                    parser @ 1..=4 => {
                        // Numeric parsers carry a min/max bitmask; these have no bounds.
                        if flags & 0x10 != 0 || u8::read(input)? != 0 {
                            return Err(DecodeError::EnumOutOfRange);
                        }
                        let kind = match parser {
                            1 => CommandNumberKind::Float,
                            2 => CommandNumberKind::Double,
                            3 => CommandNumberKind::Integer,
                            _ => CommandNumberKind::Long,
                        };
                        CommandNodeKind::Number { name, kind }
                    }
                    10 if flags & 0x10 == 0 => CommandNodeKind::Position { name },
                    _ => return Err(DecodeError::EnumOutOfRange),
                }
            }
            _ => return Err(DecodeError::EnumOutOfRange),
        };
        Ok(Self { kind, children, executable: flags & 4 != 0 })
    }

    async fn write<W: AsyncWrite + Unpin>(&self, w: &mut W) -> EncodeResult {
        let flags = match self.kind {
            CommandNodeKind::Root => 0,
            CommandNodeKind::Literal(_) => 1,
            CommandNodeKind::Word { suggest, .. } => 2 | if suggest { 0x10 } else { 0 },
            CommandNodeKind::GreedyText { .. } => 2,
            CommandNodeKind::Number { .. } => 2,
            CommandNodeKind::Position { .. } => 2,
        } | if self.executable { 4 } else { 0 };
        (flags as u8).write(w).await?;
        self.children.write(w).await?;
        match self.kind {
            CommandNodeKind::Root => {}
            CommandNodeKind::Literal(name) => name.write(w).await?,
            CommandNodeKind::Word { name, suggest } => {
                name.write(w).await?;
                VarInt(5).write(w).await?;
                VarInt(0).write(w).await?;
                if suggest {
                    "minecraft:ask_server".write(w).await?;
                }
            }
            CommandNodeKind::GreedyText { name } => {
                name.write(w).await?;
                VarInt(5).write(w).await?;
                VarInt(2).write(w).await?;
            }
            CommandNodeKind::Number { name, kind } => {
                name.write(w).await?;
                VarInt(kind.parser()).write(w).await?;
                0u8.write(w).await?;
            }
            CommandNodeKind::Position { name } => {
                name.write(w).await?;
                // minecraft:vec3 (26.3); singleton parser, no properties.
                VarInt(10).write(w).await?;
            }
        }
        Ok(())
    }

    fn body_len(&self) -> usize {
        1 + self.children.body_len()
            + match self.kind {
                CommandNodeKind::Root => 0,
                CommandNodeKind::Literal(name) => name.body_len(),
                CommandNodeKind::Word { name, suggest } => {
                    name.body_len() + 2 + if suggest { "minecraft:ask_server".body_len() } else { 0 }
                }
                CommandNodeKind::GreedyText { name } => name.body_len() + 2,
                CommandNodeKind::Number { name, .. } => name.body_len() + 2,
                CommandNodeKind::Position { name } => name.body_len() + 1,
            }
    }
}

#[packet(Play, 0x10, Client)]
pub struct Commands<'p> {
    pub nodes: Vec<CommandNode<'p>>,
    #[format = "varint"]
    pub root_index: i32,
}

#[derive(Debug)]
pub struct CommandMatch<'p> {
    pub text: &'p str,
}

impl<'p> PacketWriter<'p> for CommandMatch<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let text = <&str>::read(input)?;
        if bool::read(input)? {
            return Err(DecodeError::EnumOutOfRange);
        }
        Ok(Self { text })
    }

    async fn write<W: AsyncWrite + Unpin>(&self, w: &mut W) -> EncodeResult {
        self.text.write(w).await?;
        false.write(w).await
    }

    fn body_len(&self) -> usize {
        self.text.body_len() + 1
    }
}

#[packet(Play, 0x0F, Client)]
pub struct CommandSuggestionsResponse<'p> {
    #[format = "varint"]
    pub transaction_id: i32,
    #[format = "varint"]
    pub start: i32,
    #[format = "varint"]
    pub length: i32,
    pub matches: Vec<CommandMatch<'p>>,
}

#[packet(Play, 0x7C, Client)]
pub struct SystemChatMessage<'p> {
    pub content: Nbt<'p>,
    pub overlay: bool,
}

/// Stored until the next title is shown.
#[packet(Play, 0x72, Client)]
pub struct SetSubtitleText<'p> {
    pub text: Nbt<'p>,
}

/// Shows the title together with the stored subtitle.
#[packet(Play, 0x74, Client)]
pub struct SetTitleText<'p> {
    pub text: Nbt<'p>,
}

/// Durations in ticks.
#[packet(Play, 0x75, Client)]
pub struct SetTitlesAnimation {
    pub fade_in: i32,
    pub stay: i32,
    pub fade_out: i32,
}

/// Only the Update Game Mode action; profiles remain immutable and shared.
#[derive(Debug)]
pub struct PlayerInfoGameMode<'p> {
    pub players: &'p [(Uuid, GameMode)],
}

impl<'p> Payload<'p> for PlayerInfoGameMode<'_> {
    const ID: VarInt = VarInt(0x47);
    const STATE: State = State::Play;

    fn payload_len(&self) -> EncodeResult<usize> {
        Ok(1 + VarInt(self.players.len() as i32).body_len() + self.players.len() * 17)
    }

    fn decode_payload(_input: &mut &'p [u8]) -> DecodeResult<Self> {
        Err(DecodeError::EnumOutOfRange)
    }

    async fn write_payload<W: AsyncWrite + Unpin>(&self, w: &mut W) -> EncodeResult {
        0x04u8.write(w).await?;
        VarInt(self.players.len() as i32).write(w).await?;
        for (uuid, mode) in self.players {
            uuid.write(w).await?;
            VarInt(*mode as i32).write(w).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "../../tests/tests_packets_commands.rs"]
mod tests;
