//! The command tree the server declares, from which the client completes and
//! highlights commands, then suggestions, chat feedback and titles.
use std::borrow::Cow;

use mc_protocol_macros::{IntoOwned, packet};

use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::error::DecodeError;
use crate::packets::PacketWriter;
use crate::types::{Nbt, VarInt};

const TYPE_MASK: u8 = 0x03;
const TYPE_ROOT: u8 = 0;
const TYPE_LITERAL: u8 = 1;
const TYPE_ARGUMENT: u8 = 2;

const FLAG_EXECUTABLE: u8 = 0x04;
const FLAG_REDIRECT: u8 = 0x08;
const FLAG_SUGGESTIONS: u8 = 0x10;
const FLAG_RESTRICTED: u8 = 0x20;
const KNOWN_FLAGS: u8 = TYPE_MASK | FLAG_EXECUTABLE | FLAG_REDIRECT | FLAG_SUGGESTIONS | FLAG_RESTRICTED;

/// Numeric parsers prefix their bounds with which of them are present.
const HAS_MIN: u8 = 0x01;
const HAS_MAX: u8 = 0x02;

#[derive(Debug, IntoOwned)]
pub struct CommandNode<'p> {
    pub kind: CommandNodeKind<'p>,
    pub children: Vec<VarInt>,
    /// The node whose children follow this one, as `/execute run` does.
    pub redirect: Option<i32>,
    pub executable: bool,
    /// Needs a permission the client asks the player to confirm.
    pub restricted: bool,
}

#[derive(Debug, PartialEq, IntoOwned)]
pub enum CommandNodeKind<'p> {
    Root,
    Literal(Cow<'p, str>),
    Argument {
        name: Cow<'p, str>,
        parser: CommandParser<'p>,
        /// The provider the client asks for completions, such as
        /// `minecraft:ask_server`.
        suggestions: Option<Cow<'p, str>>,
    },
}

/// How Brigadier string arguments end.
#[derive(Clone, Copy, Debug, PartialEq, Eq, IntoOwned)]
pub enum StringMode {
    SingleWord,
    Quotable,
    /// The rest of the command.
    Greedy,
}

/// An argument parser of the `minecraft:command_argument_type` registry,
/// with its properties.
#[derive(Debug, PartialEq, IntoOwned)]
pub enum CommandParser<'p> {
    Bool,
    Float {
        min: Option<f32>,
        max: Option<f32>,
    },
    Double {
        min: Option<f64>,
        max: Option<f64>,
    },
    Integer {
        min: Option<i32>,
        max: Option<i32>,
    },
    Long {
        min: Option<i64>,
        max: Option<i64>,
    },
    String(StringMode),
    Entity {
        flags: u8,
    },
    ScoreHolder {
        flags: u8,
    },
    /// In ticks.
    Time {
        min: i32,
    },
    /// `resource_or_tag`, `resource_or_tag_key`, `resource`, `resource_key`
    /// or `resource_selector`, over the registry it names.
    Registry {
        id: i32,
        registry: Cow<'p, str>,
    },
    /// Any other parser: none of them has properties.
    Plain(i32),
}

impl CommandParser<'_> {
    pub const GAME_PROFILE: i32 = 7;
    pub const VEC3: i32 = 10;

    const BOOL: i32 = 0;
    const FLOAT: i32 = 1;
    const DOUBLE: i32 = 2;
    const INTEGER: i32 = 3;
    const LONG: i32 = 4;
    const STRING: i32 = 5;
    const ENTITY: i32 = 6;
    const SCORE_HOLDER: i32 = 31;
    const TIME: i32 = 43;
    const REGISTRIES: std::ops::RangeInclusive<i32> = 44..=48;
    /// `uuid`, the last parser of 26.3.
    const LAST: i32 = 61;

    fn id(&self) -> i32 {
        match self {
            Self::Bool => Self::BOOL,
            Self::Float { .. } => Self::FLOAT,
            Self::Double { .. } => Self::DOUBLE,
            Self::Integer { .. } => Self::INTEGER,
            Self::Long { .. } => Self::LONG,
            Self::String(_) => Self::STRING,
            Self::Entity { .. } => Self::ENTITY,
            Self::ScoreHolder { .. } => Self::SCORE_HOLDER,
            Self::Time { .. } => Self::TIME,
            Self::Registry { id, .. } => *id,
            Self::Plain(id) => *id,
        }
    }
}

impl<'p> PacketWriter<'p> for CommandParser<'p> {
    /// Properties carry no length, so an unknown parser cannot be skipped
    /// and fails the whole tree.
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let id = VarInt::read(input)?.0;

        let parser = match id {
            Self::BOOL => Self::Bool,
            Self::FLOAT => {
                let (min, max) = read_bounds(input)?;
                Self::Float { min, max }
            }
            Self::DOUBLE => {
                let (min, max) = read_bounds(input)?;
                Self::Double { min, max }
            }
            Self::INTEGER => {
                let (min, max) = read_bounds(input)?;
                Self::Integer { min, max }
            }
            Self::LONG => {
                let (min, max) = read_bounds(input)?;
                Self::Long { min, max }
            }
            Self::STRING => {
                let mode = match VarInt::read(input)?.0 {
                    0 => StringMode::SingleWord,
                    1 => StringMode::Quotable,
                    2 => StringMode::Greedy,
                    _ => return Err(DecodeError::EnumOutOfRange),
                };
                Self::String(mode)
            }
            Self::ENTITY => Self::Entity { flags: u8::read(input)? },
            Self::SCORE_HOLDER => Self::ScoreHolder { flags: u8::read(input)? },
            Self::TIME => Self::Time { min: i32::read(input)? },
            id if Self::REGISTRIES.contains(&id) => Self::Registry { id, registry: Cow::read(input)? },
            id if (0..=Self::LAST).contains(&id) => Self::Plain(id),
            _ => return Err(DecodeError::EnumOutOfRange),
        };

        Ok(parser)
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        VarInt(self.id()).encode(out)?;

        match self {
            Self::Bool | Self::Plain(_) => Ok(()),
            Self::Float { min, max } => encode_bounds(min, max, out),
            Self::Double { min, max } => encode_bounds(min, max, out),
            Self::Integer { min, max } => encode_bounds(min, max, out),
            Self::Long { min, max } => encode_bounds(min, max, out),
            Self::String(mode) => VarInt(*mode as i32).encode(out),
            Self::Entity { flags } | Self::ScoreHolder { flags } => flags.encode(out),
            Self::Time { min } => min.encode(out),
            Self::Registry { registry, .. } => registry.encode(out),
        }
    }

    fn body_len(&self) -> usize {
        let properties = match self {
            Self::Bool | Self::Plain(_) => 0,
            Self::Float { min, max } => bounds_len(min, max),
            Self::Double { min, max } => bounds_len(min, max),
            Self::Integer { min, max } => bounds_len(min, max),
            Self::Long { min, max } => bounds_len(min, max),
            Self::String(mode) => VarInt(*mode as i32).body_len(),
            Self::Entity { .. } | Self::ScoreHolder { .. } => 1,
            Self::Time { .. } => 4,
            Self::Registry { registry, .. } => registry.body_len(),
        };

        VarInt(self.id()).body_len() + properties
    }
}

fn read_bounds<'p, T: PacketWriter<'p>>(input: &mut &'p [u8]) -> DecodeResult<(Option<T>, Option<T>)> {
    let flags = u8::read(input)?;
    if flags & !(HAS_MIN | HAS_MAX) != 0 {
        return Err(DecodeError::EnumOutOfRange);
    }

    let min = if flags & HAS_MIN != 0 {
        Some(T::read(input)?)
    } else {
        None
    };
    let max = if flags & HAS_MAX != 0 {
        Some(T::read(input)?)
    } else {
        None
    };

    Ok((min, max))
}

fn encode_bounds<'p, T: PacketWriter<'p>>(min: &Option<T>, max: &Option<T>, out: &mut Vec<u8>) -> EncodeResult {
    let mut flags = 0;
    if min.is_some() {
        flags |= HAS_MIN;
    }
    if max.is_some() {
        flags |= HAS_MAX;
    }
    flags.encode(out)?;

    if let Some(min) = min {
        min.encode(out)?;
    }
    if let Some(max) = max {
        max.encode(out)?;
    }

    Ok(())
}

fn bounds_len<'p, T: PacketWriter<'p>>(min: &Option<T>, max: &Option<T>) -> usize {
    let min = min.as_ref().map_or(0, T::body_len);
    let max = max.as_ref().map_or(0, T::body_len);

    1 + min + max
}

impl<'p> PacketWriter<'p> for CommandNode<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let flags = u8::read(input)?;
        if flags & !KNOWN_FLAGS != 0 {
            return Err(DecodeError::EnumOutOfRange);
        }

        let children = Vec::<VarInt>::read(input)?;
        let redirect = if flags & FLAG_REDIRECT != 0 {
            Some(VarInt::read(input)?.0)
        } else {
            None
        };

        // Only arguments carry a suggestion provider.
        let suggests = flags & FLAG_SUGGESTIONS != 0;
        let kind = match flags & TYPE_MASK {
            TYPE_ROOT if !suggests => CommandNodeKind::Root,
            TYPE_LITERAL if !suggests => CommandNodeKind::Literal(Cow::read(input)?),
            TYPE_ARGUMENT => {
                let name = Cow::read(input)?;
                let parser = CommandParser::read(input)?;
                let suggestions = if suggests { Some(Cow::read(input)?) } else { None };

                CommandNodeKind::Argument { name, parser, suggestions }
            }
            _ => return Err(DecodeError::EnumOutOfRange),
        };

        let executable = flags & FLAG_EXECUTABLE != 0;
        let restricted = flags & FLAG_RESTRICTED != 0;

        Ok(Self { kind, children, redirect, executable, restricted })
    }

    fn encode(&self, out: &mut Vec<u8>) -> EncodeResult {
        self.flags().encode(out)?;
        self.children.encode(out)?;

        if let Some(redirect) = self.redirect {
            VarInt(redirect).encode(out)?;
        }

        match &self.kind {
            CommandNodeKind::Root => {}
            CommandNodeKind::Literal(name) => name.encode(out)?,
            CommandNodeKind::Argument { name, parser, suggestions } => {
                name.encode(out)?;
                parser.encode(out)?;

                if let Some(suggestions) = suggestions {
                    suggestions.encode(out)?;
                }
            }
        }

        Ok(())
    }

    fn body_len(&self) -> usize {
        let redirect = self.redirect.map_or(0, |redirect| VarInt(redirect).body_len());

        let kind = match &self.kind {
            CommandNodeKind::Root => 0,
            CommandNodeKind::Literal(name) => name.body_len(),
            CommandNodeKind::Argument { name, parser, suggestions } => {
                let suggestions = suggestions.as_ref().map_or(0, |suggestions| suggestions.body_len());

                name.body_len() + parser.body_len() + suggestions
            }
        };

        1 + self.children.body_len() + redirect + kind
    }
}

impl CommandNode<'_> {
    fn flags(&self) -> u8 {
        let mut flags = match &self.kind {
            CommandNodeKind::Root => TYPE_ROOT,
            CommandNodeKind::Literal(_) => TYPE_LITERAL,
            CommandNodeKind::Argument { suggestions: None, .. } => TYPE_ARGUMENT,
            CommandNodeKind::Argument { suggestions: Some(_), .. } => TYPE_ARGUMENT | FLAG_SUGGESTIONS,
        };

        if self.executable {
            flags |= FLAG_EXECUTABLE;
        }
        if self.redirect.is_some() {
            flags |= FLAG_REDIRECT;
        }
        if self.restricted {
            flags |= FLAG_RESTRICTED;
        }

        flags
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
