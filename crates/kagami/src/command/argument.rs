//! The argument types commands are made of.

use super::{ArgumentKind, CommandArgument, Words};

/// The rest of the command, with its spacing intact. Only the last argument
/// can be one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GreedyText<'c>(pub &'c str);

impl<'c> CommandArgument<'c> for GreedyText<'c> {
    const KIND: ArgumentKind = ArgumentKind::GreedyText;

    fn parse(word: &'c str) -> Result<Self, &'static str> {
        let text = word.trim();
        if text.is_empty() {
            return Err("Expected a message.");
        }

        Ok(Self(text))
    }

    fn parse_remaining(_first: &'c str, words: &mut Words<'c>) -> Result<Self, &'static str> {
        Self::parse(words.take_tail())
    }
}

/// A username, as the client completes it from its player list. Whether the
/// player is online is up to the handler.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerName<'c>(&'c str);

impl<'c> PlayerName<'c> {
    pub fn as_str(self) -> &'c str {
        self.0
    }
}

impl<'c> CommandArgument<'c> for PlayerName<'c> {
    const KIND: ArgumentKind = ArgumentKind::Player;

    fn parse(word: &'c str) -> Result<Self, &'static str> {
        let valid = word.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'_');
        if word.is_empty() || word.len() > 16 || !valid {
            return Err("Expected a player name of 1 to 16 letters, digits or underscores.");
        }

        Ok(Self(word))
    }
}

/// One unquoted word of letters, digits and `_-.+`, as Brigadier reads them.
fn check_word(word: &str) -> Result<(), &'static str> {
    let valid = word.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b'+'));
    if word.is_empty() || !valid {
        return Err("Expected one word of letters, digits or _-.+");
    }

    Ok(())
}

impl<'c> CommandArgument<'c> for &'c str {
    const KIND: ArgumentKind = ArgumentKind::Word;

    fn parse(word: &'c str) -> Result<Self, &'static str> {
        check_word(word)?;

        Ok(word)
    }
}

impl<'c> CommandArgument<'c> for String {
    const KIND: ArgumentKind = ArgumentKind::Word;

    fn parse(word: &'c str) -> Result<Self, &'static str> {
        check_word(word)?;

        Ok(word.to_owned())
    }
}

impl<'c> CommandArgument<'c> for bool {
    const KIND: ArgumentKind = ArgumentKind::Choices(&["true", "false"]);

    fn parse(word: &'c str) -> Result<Self, &'static str> {
        word.parse().map_err(|_| "Expected true or false.")
    }
}

/// Brigadier's number syntax. Rust's parsers also take `+` and exponents,
/// which the client would show as errors.
fn is_number(word: &str) -> bool {
    !word.is_empty() && word.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'-' | b'.'))
}

macro_rules! number {
    ($ty:ty, $kind:expr, $message:literal) => {
        impl<'c> CommandArgument<'c> for $ty {
            const KIND: ArgumentKind = $kind;

            fn parse(word: &'c str) -> Result<Self, &'static str> {
                if !is_number(word) {
                    return Err($message);
                }

                word.parse().map_err(|_| $message)
            }
        }
    };
}

number!(i32, ArgumentKind::Integer { min: None, max: None }, "Expected a whole number.");
number!(i64, ArgumentKind::Long { min: None, max: None }, "Expected a whole number.");
number!(
    u32,
    ArgumentKind::Long { min: Some(0), max: Some(u32::MAX as i64) },
    "Expected a positive whole number."
);
number!(u64, ArgumentKind::Long { min: Some(0), max: None }, "Expected a positive whole number.");

macro_rules! decimal {
    ($ty:ty, $kind:expr) => {
        impl<'c> CommandArgument<'c> for $ty {
            const KIND: ArgumentKind = $kind;

            fn parse(word: &'c str) -> Result<Self, &'static str> {
                const MESSAGE: &str = "Expected a number.";

                if !is_number(word) {
                    return Err(MESSAGE);
                }

                let number: Self = word.parse().map_err(|_| MESSAGE)?;
                if !number.is_finite() {
                    return Err(MESSAGE);
                }

                Ok(number)
            }
        }
    };
}

decimal!(f32, ArgumentKind::Float);
decimal!(f64, ArgumentKind::Double);

const POSITION_MESSAGE: &str = "Expected three coordinates, such as 10 ~ -4.5";

/// One axis of a [`Position`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Coordinate {
    Absolute(f64),
    /// An offset from the player, written `~` or `~2.5`.
    Relative(f64),
}

impl Coordinate {
    /// `center` moves a whole X or Z to the middle of its block, as the game
    /// does.
    fn parse(word: &str, center: bool) -> Result<Self, &'static str> {
        if let Some(offset) = word.strip_prefix('~') {
            if offset.is_empty() {
                return Ok(Self::Relative(0.0));
            }

            let offset = <f64 as CommandArgument>::parse(offset).map_err(|_| POSITION_MESSAGE)?;
            return Ok(Self::Relative(offset));
        }

        let mut value = <f64 as CommandArgument>::parse(word).map_err(|_| POSITION_MESSAGE)?;
        if center && !word.contains('.') {
            value += 0.5;
        }

        Ok(Self::Absolute(value))
    }

    fn resolve(self, origin: f64) -> f64 {
        match self {
            Self::Absolute(value) => value,
            Self::Relative(offset) => origin + offset,
        }
    }
}

/// World coordinates: three words, each absolute or relative to the player.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position(pub [Coordinate; 3]);

impl Position {
    /// The point these coordinates name, seen from `origin`.
    pub fn resolve(self, origin: [f64; 3]) -> [f64; 3] {
        let [x, y, z] = self.0;

        [x.resolve(origin[0]), y.resolve(origin[1]), z.resolve(origin[2])]
    }
}

impl<'c> CommandArgument<'c> for Position {
    const KIND: ArgumentKind = ArgumentKind::Position;

    /// A position takes three words, so one is never enough.
    fn parse(_word: &'c str) -> Result<Self, &'static str> {
        Err(POSITION_MESSAGE)
    }

    fn parse_remaining(first: &'c str, words: &mut Words<'c>) -> Result<Self, &'static str> {
        let x = Coordinate::parse(first, true)?;

        let Some(y) = words.next() else {
            return Err(POSITION_MESSAGE);
        };
        let y = Coordinate::parse(y, false)?;

        let Some(z) = words.next() else {
            return Err(POSITION_MESSAGE);
        };
        let z = Coordinate::parse(z, true)?;

        Ok(Self([x, y, z]))
    }
}
