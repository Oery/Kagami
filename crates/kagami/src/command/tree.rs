//! Commands added to the tree the server sends, so the client completes and
//! highlights them like its own.

use mc_protocol::VarInt;
use mc_protocol::packets::client::{CommandNode, CommandNodeKind, CommandParser, Commands, StringMode};

use super::{ArgumentKind, ArgumentSpec, Syntax};

/// Adds the command `name` to `tree`, in place of the server's command of
/// the same name. Its nodes go after the server's, whose indices stay valid.
pub fn splice(tree: &mut Commands<'_>, name: &'static str, syntax: Syntax) {
    let root = tree.root_index as usize;
    if root >= tree.nodes.len() {
        return;
    }

    let mut siblings = Vec::new();
    for child in &tree.nodes[root].children {
        if !is_literal(&tree.nodes, child.0, name) {
            siblings.push(child.0);
        }
    }

    let (following, executable) = append(&mut tree.nodes, syntax);
    let command = push(&mut tree.nodes, CommandNodeKind::Literal(name.into()), &following, executable);
    siblings.push(command);

    tree.nodes[root].children = children(&siblings);
}

fn is_literal(nodes: &[CommandNode<'_>], index: i32, name: &str) -> bool {
    let Some(node) = nodes.get(index as usize) else {
        return false;
    };

    matches!(&node.kind, CommandNodeKind::Literal(literal) if literal == name)
}

/// Appends the nodes of `syntax`. Returns the nodes that follow its parent,
/// and whether the parent can run without any of them.
fn append(nodes: &mut Vec<CommandNode<'_>>, syntax: Syntax) -> (Vec<i32>, bool) {
    let subcommands = match syntax {
        Syntax::Arguments(arguments) => return append_arguments(nodes, arguments),
        Syntax::Subcommands(subcommands) => subcommands,
    };

    let mut following = Vec::new();
    let mut executable = false;

    for subcommand in subcommands {
        let (next, complete) = append(nodes, *subcommand.syntax);

        // The default subcommand's arguments follow the parent directly.
        let Some(name) = subcommand.name else {
            following.extend(next);
            executable |= complete;
            continue;
        };

        let literal = push(nodes, CommandNodeKind::Literal(name.into()), &next, complete);
        following.push(literal);
    }

    (following, executable)
}

/// Appends a chain of arguments, last first so each node knows the ones
/// after it. A node is executable when every argument after it is optional.
fn append_arguments(nodes: &mut Vec<CommandNode<'_>>, arguments: &[ArgumentSpec]) -> (Vec<i32>, bool) {
    let mut next = Vec::new();
    let mut executable = true;

    for argument in arguments.iter().rev() {
        let mut current = Vec::new();

        match argument.kind {
            ArgumentKind::Choices(choices) => {
                for choice in choices {
                    current.push(push(nodes, CommandNodeKind::Literal((*choice).into()), &next, executable));
                }
            }
            kind => {
                let parser = parser(kind);
                let kind = CommandNodeKind::Argument { name: argument.name.into(), parser, suggestions: None };
                current.push(push(nodes, kind, &next, executable));
            }
        }

        next = current;
        executable &= argument.optional;
    }

    (next, executable)
}

fn parser(kind: ArgumentKind) -> CommandParser<'static> {
    match kind {
        ArgumentKind::Word => CommandParser::String(StringMode::SingleWord),
        ArgumentKind::GreedyText => CommandParser::String(StringMode::Greedy),
        ArgumentKind::Integer { min, max } => CommandParser::Integer { min, max },
        ArgumentKind::Long { min, max } => CommandParser::Long { min, max },
        ArgumentKind::Float => CommandParser::Float { min: None, max: None },
        ArgumentKind::Double => CommandParser::Double { min: None, max: None },
        ArgumentKind::Position => CommandParser::Plain(CommandParser::VEC3),
        ArgumentKind::Player => CommandParser::Plain(CommandParser::GAME_PROFILE),
        ArgumentKind::Choices(_) => unreachable!("choices are literals"),
    }
}

/// Appends a node and returns its index.
fn push<'p>(nodes: &mut Vec<CommandNode<'p>>, kind: CommandNodeKind<'p>, following: &[i32], executable: bool) -> i32 {
    let node = CommandNode { kind, children: children(following), redirect: None, executable, restricted: false };
    nodes.push(node);

    nodes.len() as i32 - 1
}

fn children(indices: &[i32]) -> Vec<VarInt> {
    indices.iter().map(|&index| VarInt(index)).collect()
}
