//! Protocol 777 scoreboard objectives, display slots and score entries.
use futures_lite::AsyncWrite;
use mc_protocol_macros::{Payload, packet};

use crate::State;
use crate::decode::Result as DecodeResult;
use crate::encode::Result as EncodeResult;
use crate::error::DecodeError;
use crate::packets::{PacketWriter, Payload};
use crate::types::{Nbt, VarInt};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScoreRender {
    #[default]
    Integer,
    Hearts,
}

impl PacketWriter<'_> for ScoreRender {
    fn read(input: &mut &[u8]) -> DecodeResult<Self> {
        match VarInt::read(input)?.0 {
            0 => Ok(Self::Integer),
            1 => Ok(Self::Hearts),
            _ => Err(DecodeError::EnumOutOfRange),
        }
    }

    async fn write<W: AsyncWrite + Unpin>(&self, w: &mut W) -> EncodeResult {
        VarInt(*self as i32).write(w).await
    }

    fn body_len(&self) -> usize {
        1
    }
}

/// Styled carries a style compound; Fixed carries a text component.
#[derive(Clone, Debug)]
pub enum ScoreNumberFormat<'p> {
    Blank,
    Styled(Nbt<'p>),
    Fixed(Nbt<'p>),
}

impl<'p> PacketWriter<'p> for ScoreNumberFormat<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        match VarInt::read(input)?.0 {
            0 => Ok(Self::Blank),
            1 => Ok(Self::Styled(Nbt::read(input)?)),
            2 => Ok(Self::Fixed(Nbt::read(input)?)),
            _ => Err(DecodeError::EnumOutOfRange),
        }
    }

    async fn write<W: AsyncWrite + Unpin>(&self, w: &mut W) -> EncodeResult {
        match self {
            Self::Blank => VarInt(0).write(w).await,
            Self::Styled(style) => {
                VarInt(1).write(w).await?;
                style.write(w).await
            }
            Self::Fixed(text) => {
                VarInt(2).write(w).await?;
                text.write(w).await
            }
        }
    }

    fn body_len(&self) -> usize {
        1 + match self {
            Self::Blank => 0,
            Self::Styled(value) | Self::Fixed(value) => value.body_len(),
        }
    }
}

#[derive(Debug)]
pub struct ObjectiveProperties<'p> {
    pub title: Nbt<'p>,
    pub render: ScoreRender,
    pub number_format: Option<ScoreNumberFormat<'p>>,
}

#[derive(Debug)]
pub enum ObjectiveAction<'p> {
    Create(ObjectiveProperties<'p>),
    Remove,
    Update(ObjectiveProperties<'p>),
}

impl<'p> PacketWriter<'p> for ObjectiveAction<'p> {
    fn read(input: &mut &'p [u8]) -> DecodeResult<Self> {
        let mode = u8::read(input)?;
        if mode == 1 {
            return Ok(Self::Remove);
        }
        if mode != 0 && mode != 2 {
            return Err(DecodeError::EnumOutOfRange);
        }
        let properties =
            ObjectiveProperties { title: Nbt::read(input)?, render: ScoreRender::read(input)?, number_format: Option::read(input)? };
        Ok(if mode == 0 {
            Self::Create(properties)
        } else {
            Self::Update(properties)
        })
    }

    async fn write<W: AsyncWrite + Unpin>(&self, w: &mut W) -> EncodeResult {
        let (mode, properties) = match self {
            Self::Create(properties) => (0u8, properties),
            Self::Update(properties) => (2u8, properties),
            Self::Remove => return 1u8.write(w).await,
        };
        mode.write(w).await?;
        properties.title.write(w).await?;
        properties.render.write(w).await?;
        properties.number_format.write(w).await
    }

    fn body_len(&self) -> usize {
        1 + match self {
            Self::Remove => 0,
            Self::Create(properties) | Self::Update(properties) => {
                properties.title.body_len() + properties.render.body_len() + properties.number_format.body_len()
            }
        }
    }
}

#[packet(Play, 0x6C, Client)]
pub struct SetObjective<'p> {
    pub name: &'p str,
    pub action: ObjectiveAction<'p>,
}

/// Slots: 0 = Tab list, 1 = sidebar, 2 = below name; 3..=18 are team sidebars.
/// An empty objective name clears the slot.
#[packet(Play, 0x64, Client)]
pub struct SetDisplayObjective<'p> {
    #[format = "varint"]
    pub slot: i32,
    pub name: &'p str,
}

#[packet(Play, 0x70, Client)]
pub struct SetScore<'p> {
    pub holder: &'p str,
    pub objective: &'p str,
    #[format = "varint"]
    pub value: i32,
    pub display: Option<Nbt<'p>>,
    pub number_format: Option<ScoreNumberFormat<'p>>,
}

/// A single literal score label encoded directly into caller-owned storage.
/// The five decoration bits are bold, italic, underlined, strikethrough and
/// obfuscated. No text tree or temporary packet allocation is needed.
pub struct ScoreLine<'p> {
    pub holder: &'p str,
    pub objective: &'p str,
    pub value: i32,
    pub text: &'p str,
    pub color: Option<&'p str>,
    pub decorations: u8,
    /// Optional value appended in its own color, inheriting the root's decorations.
    pub suffix: Option<(&'p str, &'p str)>,
}

impl ScoreLine<'_> {
    pub fn component_len(&self) -> usize {
        use crate::types::nbt::string_len;
        1 + 1
            + 2
            + 4
            + 2
            + string_len(self.text)
            + self.color.map_or(0, |color| 1 + 2 + 5 + 2 + string_len(color))
            + ["bold", "italic", "underlined", "strikethrough", "obfuscated"].iter().map(|name| 1 + 2 + name.len() + 1).sum::<usize>()
            + 1
            + self.suffix.map_or(0, |(text, color)| {
                // Named extra list, compound element type and count; one child.
                1 + 2 + 5 + 1 + 4 + 1 + 2 + 4 + 2 + string_len(text) + 1 + 2 + 5 + 2 + string_len(color) + 1
            })
    }

    pub fn encode_into(&self, output: &mut [u8]) -> EncodeResult<usize> {
        use crate::types::nbt::{string_len, write_string};
        use std::io::{Cursor, Write};

        const DECORATIONS: [&str; 5] = ["bold", "italic", "underlined", "strikethrough", "obfuscated"];
        fn varint(w: &mut impl Write, value: i32) -> EncodeResult {
            let mut value = value as u32;
            loop {
                let byte = (value & 0x7F) as u8;
                value >>= 7;
                w.write_all(&[byte | if value == 0 { 0 } else { 0x80 }])?;
                if value == 0 {
                    return Ok(());
                }
            }
        }
        fn string(w: &mut impl Write, value: &str) -> EncodeResult {
            let len = i32::try_from(value.len()).map_err(|_| crate::error::EncodeError::InvalidLength)?;
            varint(w, len)?;
            w.write_all(value.as_bytes())?;
            Ok(())
        }
        let text_len = string_len(self.text);
        if text_len > u16::MAX as usize
            || self.color.is_some_and(|color| string_len(color) > u16::MAX as usize)
            || self.suffix.is_some_and(|(text, color)| string_len(text) > u16::MAX as usize || string_len(color) > u16::MAX as usize)
        {
            return Err(crate::error::EncodeError::InvalidLength);
        }
        let body_len =
            1 + self.holder.body_len() + self.objective.body_len() + VarInt(self.value).body_len() + 1 + self.component_len() + 1;
        let body_len = i32::try_from(body_len).map_err(|_| crate::error::EncodeError::InvalidLength)?;
        let total = VarInt(body_len).body_len() + body_len as usize;
        if output.len() < total {
            return Err(crate::error::EncodeError::InvalidLength);
        }
        let mut w = Cursor::new(output);
        varint(&mut w, body_len)?;
        varint(&mut w, 0x70)?;
        string(&mut w, self.holder)?;
        string(&mut w, self.objective)?;
        varint(&mut w, self.value)?;
        w.write_all(&[1, 10, 8])?; // display present; compound; named string
        write_string(&mut w, "text")?;
        write_string(&mut w, self.text)?;
        if let Some(color) = self.color {
            w.write_all(&[8])?;
            write_string(&mut w, "color")?;
            write_string(&mut w, color)?;
        }
        for (bit, name) in DECORATIONS.iter().enumerate() {
            w.write_all(&[1])?;
            write_string(&mut w, name)?;
            w.write_all(&[(self.decorations >> bit) & 1])?;
        }
        if let Some((text, color)) = self.suffix {
            w.write_all(&[9])?;
            write_string(&mut w, "extra")?;
            w.write_all(&[10, 0, 0, 0, 1, 8])?; // one compound; named text string
            write_string(&mut w, "text")?;
            write_string(&mut w, text)?;
            w.write_all(&[8])?;
            write_string(&mut w, "color")?;
            write_string(&mut w, color)?;
            w.write_all(&[0])?;
        }
        w.write_all(&[0, 0])?; // compound end; inherit objective's number format
        debug_assert_eq!(w.position() as usize, total);
        Ok(total)
    }
}

/// `None` removes this holder's scores from every objective.
#[packet(Play, 0x50, Client)]
pub struct ResetScore<'p> {
    pub holder: &'p str,
    pub objective: Option<&'p str>,
}

#[cfg(test)]
#[path = "../../tests/tests_packets_scoreboard.rs"]
mod tests;
