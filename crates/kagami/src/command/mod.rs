//! Chat commands parsed into Rust types.
//!
//! A command is a type deriving [`Command`]: a struct whose fields are
//! positional arguments, or an enum whose variants are subcommands. The
//! derive generates the parser and the syntax, which Kagami uses for usage
//! messages and to add the command to the client's command tree.
//!
//! ```
//! use kagami::command::{Command, GreedyText};
//!
//! /// `/remind clear` or `/remind <seconds> <text>`.
//! #[derive(Command)]
//! enum Remind<'c> {
//!     Clear,
//!     #[command(default)]
//!     Later { seconds: u32, text: GreedyText<'c> },
//! }
//!
//! let Some(Ok(Remind::Later { seconds, text })) = kagami::command::parse::<Remind>("remind 5 tea is ready") else {
//!     panic!("not a reminder");
//! };
//! assert_eq!((seconds, text.0), (5, "tea is ready"));
//! ```
//!
//! - Struct fields are parsed in order. `Option` fields must come after the
//!   required ones, and [`GreedyText`] takes the rest of the command, so it
//!   comes last. `#[arg(name = "…")]` renames a field in usage and
//!   completion.
//! - Enum variants are subcommands named after the variant in lowercase, or
//!   `#[command(name = "…")]`. A variant is a unit, has named fields that are
//!   its arguments, or holds one type deriving [`CommandArgs`], such as
//!   another subcommand enum. The variant marked `#[command(default)]` has no
//!   name of its own and is parsed when no other one matches.
//! - The command is named after the type in lowercase, or
//!   `#[command(name = "…")]` on the type.
//! - Enums deriving [`CommandValue`] are one word among fixed choices.
//! - Other argument types implement [`CommandArgument`].
//!
//! Parsing only checks the text. Whatever the command does with the world,
//! the player or the server belongs to its handler.

mod argument;
mod tree;

use std::fmt;

pub use argument::{Coordinate, GreedyText, PlayerName, Position};
pub use kagami_macros::{Command, CommandArgs, CommandValue};
pub(crate) use tree::splice;

/// A command type, registered with [`Kagami::on_command`](crate::Kagami::on_command).
/// Derived with [`Command`] rather than implemented.
pub trait Command: 'static {
    const NAME: &'static str;
    const SYNTAX: Syntax = <Self::Parsed<'static> as CommandArgs<'static>>::SYNTAX;

    /// This command, borrowing the text it was parsed from.
    type Parsed<'c>: CommandArgs<'c>;
}

/// The arguments of a command or subcommand. Derived with [`CommandArgs`]
/// or [`Command`].
pub trait CommandArgs<'c>: Sized {
    const SYNTAX: Syntax;

    /// Parses every remaining word: words left over are an error.
    fn parse(words: &mut Words<'c>) -> Result<Self, ArgumentError>;
}

/// One argument type, parsed from one word or more.
pub trait CommandArgument<'c>: Sized {
    const KIND: ArgumentKind;

    /// Parses one word. The message tells the player what was expected.
    fn parse(word: &'c str) -> Result<Self, &'static str>;

    /// Parses an argument starting at `first`, which may take more words.
    fn parse_remaining(first: &'c str, _words: &mut Words<'c>) -> Result<Self, &'static str> {
        Self::parse(first)
    }
}

/// Parses `text`, a command as the client sends it, without the slash.
/// `None` if `text` is another command.
pub fn parse<C: Command>(text: &str) -> Option<Result<C::Parsed<'_>, ParseError>> {
    let mut words = Words::new(text);
    if words.next() != Some(C::NAME) {
        return None;
    }

    let parsed = C::Parsed::parse(&mut words).map_err(|error| ParseError { command: C::NAME, syntax: C::SYNTAX, error });

    Some(parsed)
}

/// The words of a command, borrowed from its text.
#[derive(Clone, Debug)]
pub struct Words<'c> {
    source: &'c str,
    /// Where the next word is searched from.
    offset: usize,
    /// Where the last word returned starts.
    start: usize,
}

impl<'c> Words<'c> {
    pub fn new(source: &'c str) -> Self {
        Self { source, offset: 0, start: 0 }
    }

    /// The text from the start of the last word returned to the end, with
    /// its spacing intact. Nothing is left after it.
    pub fn take_tail(&mut self) -> &'c str {
        let tail = self.source[self.start..].trim_end();
        self.offset = self.source.len();

        tail
    }
}

impl<'c> Iterator for Words<'c> {
    type Item = &'c str;

    fn next(&mut self) -> Option<&'c str> {
        let remaining = self.source[self.offset..].trim_start();
        if remaining.is_empty() {
            self.offset = self.source.len();
            return None;
        }

        let length = remaining.find(char::is_whitespace).unwrap_or(remaining.len());
        self.start = self.source.len() - remaining.len();
        self.offset = self.start + length;

        Some(&remaining[..length])
    }
}

/// What an argument accepts, which picks its node in the client's command
/// tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgumentKind {
    /// One of fixed words, each a literal in the tree.
    Choices(&'static [&'static str]),
    Word,
    /// The rest of the command.
    GreedyText,
    Integer {
        min: Option<i32>,
        max: Option<i32>,
    },
    Long {
        min: Option<i64>,
        max: Option<i64>,
    },
    Float,
    Double,
    /// Three world coordinates.
    Position,
    /// An online player, completed from the client's player list.
    Player,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArgumentSpec {
    pub name: &'static str,
    pub optional: bool,
    pub kind: ArgumentKind,
}

/// The shape of a command after its name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syntax {
    Arguments(&'static [ArgumentSpec]),
    Subcommands(&'static [Subcommand]),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Subcommand {
    /// `None` for the default subcommand, whose arguments directly follow
    /// the parent.
    pub name: Option<&'static str>,
    pub syntax: &'static Syntax,
}

impl Syntax {
    /// Every form of `command`, such as `/remind clear | /remind <seconds> <text>`.
    pub fn usage(self, command: &str) -> String {
        let mut forms = Vec::new();
        self.forms(format!("/{command}"), &mut forms);

        forms.join(" | ")
    }

    /// Pushes every form starting with `prefix`: one per subcommand, down to
    /// the arguments.
    fn forms(self, prefix: String, forms: &mut Vec<String>) {
        let subcommands = match self {
            Syntax::Arguments(arguments) => {
                forms.push(form(prefix, arguments));
                return;
            }
            Syntax::Subcommands(subcommands) => subcommands,
        };

        for subcommand in subcommands {
            let prefix = match subcommand.name {
                Some(name) => format!("{prefix} {name}"),
                None => prefix.clone(),
            };

            subcommand.syntax.forms(prefix, forms);
        }
    }
}

/// `prefix` followed by `arguments`, written `<required>` and `[optional]`.
fn form(prefix: String, arguments: &[ArgumentSpec]) -> String {
    let mut form = prefix;

    for argument in arguments {
        let (open, close) = if argument.optional { ('[', ']') } else { ('<', '>') };

        form.push(' ');
        form.push(open);
        match argument.kind {
            ArgumentKind::Choices(choices) => form.push_str(&choices.join("|")),
            ArgumentKind::Position => form.push_str("x y z"),
            _ => form.push_str(argument.name),
        }
        form.push(close);
    }

    form
}

/// Why the arguments of a command did not parse.
#[derive(Debug, PartialEq, Eq)]
pub enum ArgumentError {
    Missing(&'static str),
    Invalid {
        name: &'static str,
        message: &'static str,
    },
    /// Words were left after the last argument.
    Extra,
}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(name) => write!(f, "Missing {name}."),
            Self::Invalid { name, message } => write!(f, "Invalid {name}: {message}"),
            Self::Extra => f.write_str("Too many arguments."),
        }
    }
}

impl std::error::Error for ArgumentError {}

/// A command whose arguments did not parse, shown with its usage.
#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    pub command: &'static str,
    pub syntax: Syntax,
    pub error: ArgumentError,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Usage: {}", self.error, self.syntax.usage(self.command))
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

#[cfg(test)]
#[path = "../../tests/tests_command.rs"]
mod tests;
