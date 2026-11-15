//! Styled literal text shared by chat, action bars, titles and scoreboards.
//!
//! ```ignore
//! Text::new().gold("Round 3").gray(": ").push(name, Style::YELLOW.bold())
//! ```
use std::fmt;

use crate::PacketWriter;
use crate::types::Nbt;

pub const MAX_TEXT_BYTES: usize = 4096;
const MAX_SPANS: usize = 64;

/// Text exceeds its bounds or names an unknown color.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidText;

macro_rules! colors {
    ($($name:literal => $style:ident, $method:ident;)*) => {
        const COLORS: &[&str] = &[$($name),*];

        #[allow(dead_code)] // Public game API; games use a subset.
        impl Style {
            $(pub const $style: Self = Self::color($name);)*
        }

        #[allow(dead_code)] // Public game API; games use a subset.
        impl Text {
            $(pub fn $method(self, text: impl Into<String>) -> Self {
                self.push(text, Style::$style)
            })*
        }
    };
}

colors! {
    "black" => BLACK, black;
    "dark_blue" => DARK_BLUE, dark_blue;
    "dark_green" => DARK_GREEN, dark_green;
    "dark_aqua" => DARK_AQUA, dark_aqua;
    "dark_red" => DARK_RED, dark_red;
    "dark_purple" => DARK_PURPLE, dark_purple;
    "gold" => GOLD, gold;
    "gray" => GRAY, gray;
    "dark_gray" => DARK_GRAY, dark_gray;
    "blue" => BLUE, blue;
    "green" => GREEN, green;
    "aqua" => AQUA, aqua;
    "red" => RED, red;
    "light_purple" => LIGHT_PURPLE, light_purple;
    "yellow" => YELLOW, yellow;
    "white" => WHITE, white;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Style {
    /// A vanilla color name or `#rrggbb`.
    pub color: Option<&'static str>,
    pub bold: bool,
    pub italic: bool,
    pub underlined: bool,
    pub strikethrough: bool,
    pub obfuscated: bool,
}

#[allow(dead_code)] // Public game API; games use a subset.
impl Style {
    pub const PLAIN: Self = Self { color: None, bold: false, italic: false, underlined: false, strikethrough: false, obfuscated: false };

    /// A vanilla color name or `#rrggbb`; checked when the text is sent.
    pub const fn color(color: &'static str) -> Self {
        Self { color: Some(color), ..Self::PLAIN }
    }

    pub const fn bold(self) -> Self {
        Self { bold: true, ..self }
    }

    pub const fn italic(self) -> Self {
        Self { italic: true, ..self }
    }

    pub const fn underlined(self) -> Self {
        Self { underlined: true, ..self }
    }

    pub const fn strikethrough(self) -> Self {
        Self { strikethrough: true, ..self }
    }

    pub const fn obfuscated(self) -> Self {
        Self { obfuscated: true, ..self }
    }

    pub fn nbt(&self) -> Nbt<'_> {
        let mut fields = vec![
            ("bold", Nbt::Bool(self.bold)),
            ("italic", Nbt::Bool(self.italic)),
            ("underlined", Nbt::Bool(self.underlined)),
            ("strikethrough", Nbt::Bool(self.strikethrough)),
            ("obfuscated", Nbt::Bool(self.obfuscated)),
        ];
        if let Some(color) = self.color {
            fields.push(("color", Nbt::Str(color)));
        }
        Nbt::Compound(fields)
    }

    pub fn validate(&self) -> Result<(), InvalidText> {
        if let Some(color) = self.color
            && !COLORS.contains(&color)
            && !(color.len() == 7 && color.starts_with('#') && color.as_bytes()[1..].iter().all(u8::is_ascii_hexdigit))
        {
            return Err(InvalidText);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Span {
    pub text: String,
    pub style: Style,
}

/// Literal text, with explicit styling on each span; no executable text events.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Text(pub Vec<Span>);

impl From<&str> for Text {
    fn from(text: &str) -> Self {
        text.to_owned().into()
    }
}

impl From<String> for Text {
    fn from(text: String) -> Self {
        Self(vec![Span { text, style: Style::default() }])
    }
}

impl From<&String> for Text {
    fn from(text: &String) -> Self {
        text.clone().into()
    }
}

/// The unstyled characters, in order.
impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|span| f.write_str(&span.text))
    }
}

#[allow(dead_code)] // Public game API; games use a subset.
impl Text {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(mut self, text: impl Into<String>, style: Style) -> Self {
        self.0.push(Span { text: text.into(), style });
        self
    }

    pub fn plain(self, text: impl Into<String>) -> Self {
        self.push(text, Style::PLAIN)
    }

    /// Continues with another text's styled spans.
    pub fn append(mut self, other: Text) -> Self {
        self.0.extend(other.0);
        self
    }

    pub fn nbt(&self) -> Nbt<'_> {
        // The client requires a present `extra` list to contain at least one
        // component. Empty text is represented by the literal alone.
        if self.0.is_empty() {
            return Nbt::Compound(vec![("text", Nbt::Str(""))]);
        }
        let extra = self
            .0
            .iter()
            .map(|span| {
                let Nbt::Compound(mut fields) = span.style.nbt() else {
                    unreachable!()
                };
                fields.push(("text", Nbt::Str(&span.text)));
                Nbt::Compound(fields)
            })
            .collect();
        Nbt::Compound(vec![("text", Nbt::Str("")), ("extra", Nbt::List(extra))])
    }

    pub fn validate(&self) -> Result<(), InvalidText> {
        // Bound the input before constructing any NBT containers.
        if self.0.len() > MAX_SPANS || self.0.iter().map(|s| s.text.len()).sum::<usize>() > MAX_TEXT_BYTES {
            return Err(InvalidText);
        }
        for span in &self.0 {
            span.style.validate()?;
        }
        if self.nbt().body_len() > MAX_TEXT_BYTES {
            return Err(InvalidText);
        }
        Ok(())
    }
}

/// A title, with an optional subtitle, shown for the given ticks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Title {
    pub title: Text,
    pub subtitle: Option<Text>,
    pub fade_in: u32,
    pub stay: u32,
    pub fade_out: u32,
}

#[allow(dead_code)] // Public game API; games use a subset.
impl Title {
    /// Vanilla's default timings: 10 ticks in, 70 shown, 20 out.
    pub fn new(title: impl Into<Text>) -> Self {
        Self { title: title.into(), subtitle: None, fade_in: 10, stay: 70, fade_out: 20 }
    }

    pub fn subtitle(self, subtitle: impl Into<Text>) -> Self {
        Self { subtitle: Some(subtitle.into()), ..self }
    }

    pub fn timings(self, fade_in: u32, stay: u32, fade_out: u32) -> Self {
        Self { fade_in, stay, fade_out, ..self }
    }

    pub fn validate(&self) -> Result<(), InvalidText> {
        self.title.validate()?;
        self.subtitle.as_ref().map_or(Ok(()), Text::validate)
    }
}

#[cfg(test)]
#[path = "../tests/tests_types_text.rs"]
mod tests;
