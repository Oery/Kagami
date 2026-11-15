//! Command trees with literals, words, numeric arguments and world coordinates.
use std::borrow::Cow;

use mc_protocol_macros::{IntoOwned, packet};

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::error::DecodeError;
use crate::packets::PacketWriter;
use crate::types::{Nbt, VarInt};

#[derive(Clone, Copy, Debug, PartialEq, Eq, IntoOwned)]
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

#[derive(Debug, PartialEq, Eq, IntoOwned)]
pub enum CommandNodeKind<'p> {
    Root,
    Literal(Cow<'p, str>),
    Word { name: Cow<'p, str>, suggest: bool },
    GreedyText { name: Cow<'p, str> },
    Number { name: Cow<'p, str>, kind: CommandNumberKind },
    Position { name: Cow<'p, str> },
}

#[derive(Debug, IntoOwned)]
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
            1 if flags & 0x10 == 0 => CommandNodeKind::Literal(Cow::read(input)?),
            2 => {
                let name = Cow::read(input)?;

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

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        let flags = match self.kind {
            CommandNodeKind::Root => 0,
            CommandNodeKind::Literal(_) => 1,
            CommandNodeKind::Word { suggest, .. } => 2 | if suggest { 0x10 } else { 0 },
            CommandNodeKind::GreedyText { .. } => 2,
            CommandNodeKind::Number { .. } => 2,
            CommandNodeKind::Position { .. } => 2,
        } | if self.executable { 4 } else { 0 };
        (flags as u8).encode(out)?;
        self.children.encode(out)?;

        match &self.kind {
            CommandNodeKind::Root => {}
            CommandNodeKind::Literal(name) => name.encode(out)?,
            CommandNodeKind::Word { name, suggest } => {
                name.encode(out)?;
                VarInt(5).encode(out)?;
                VarInt(0).encode(out)?;

                if *suggest {
                    "minecraft:ask_server".encode(out)?;
                }
            }
            CommandNodeKind::GreedyText { name } => {
                name.encode(out)?;
                VarInt(5).encode(out)?;
                VarInt(2).encode(out)?;
            }
            CommandNodeKind::Number { name, kind } => {
                name.encode(out)?;
                VarInt(kind.parser()).encode(out)?;
                0u8.encode(out)?;
            }
            CommandNodeKind::Position { name } => {
                name.encode(out)?;
                // minecraft:vec3 (26.3); singleton parser, no properties.
                VarInt(10).encode(out)?;
            }
        }

        Ok(())
    }

    fn body_len(&self) -> usize {
        1 + self.children.body_len()
            + match &self.kind {
                CommandNodeKind::Root => 0,
                CommandNodeKind::Literal(name) => name.body_len(),
                CommandNodeKind::Word { name, suggest } => {
                    name.body_len() + 2 + if *suggest { "minecraft:ask_server".body_len() } else { 0 }
                }
                CommandNodeKind::GreedyText { name } => name.body_len() + 2,
                CommandNodeKind::Number { name, .. } => name.body_len() + 2,
                CommandNodeKind::Position { name } => name.body_len() + 1,
            }
    }
}

#[packet(Play, 0x10, Clientbound)]
pub struct Commands<'p> {
    pub nodes: Vec<CommandNode<'p>>,
    #[format = "varint"]
    pub root_index: i32,
}

#[derive(Debug, IntoOwned)]
pub struct CommandMatch<'p> {
    pub text: Cow<'p, str>,
}

impl<'p> PacketWriter<'p> for CommandMatch<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let text = Cow::read(input)?;

        if bool::read(input)? {
            return Err(DecodeError::EnumOutOfRange);
        }

        Ok(Self { text })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.text.encode(out)?;
        false.encode(out)
    }

    fn body_len(&self) -> usize {
        self.text.body_len() + 1
    }
}

#[packet(Play, 0x0F, Clientbound)]
pub struct CommandSuggestionsResponse<'p> {
    #[format = "varint"]
    pub transaction_id: i32,
    #[format = "varint"]
    pub start: i32,
    #[format = "varint"]
    pub length: i32,
    pub matches: Vec<CommandMatch<'p>>,
}

#[packet(Play, 0x7C, Clientbound)]
pub struct SystemChatMessage<'p> {
    pub content: Nbt<'p>,
    pub overlay: bool,
}

/// Stored until the next title is shown.
#[packet(Play, 0x72, Clientbound)]
pub struct SetSubtitleText<'p> {
    pub text: Nbt<'p>,
}

/// Shows the title together with the stored subtitle.
#[packet(Play, 0x74, Clientbound)]
pub struct SetTitleText<'p> {
    pub text: Nbt<'p>,
}

/// Durations in ticks.
#[packet(Play, 0x75, Clientbound)]
pub struct SetTitlesAnimation {
    pub fade_in: i32,
    pub stay: i32,
    pub fade_out: i32,
}

#[cfg(test)]
#[path = "../../../tests/tests_packets_commands.rs"]
mod tests;
