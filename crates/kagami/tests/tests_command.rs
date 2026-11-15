use mc_protocol::VarInt;
use mc_protocol::packets::client::{CommandNode, CommandNodeKind, CommandParser, Commands, StringMode};

use super::*;

#[derive(Debug, PartialEq, CommandValue)]
enum Mode {
    Survival,
    #[value(alias = "1")]
    Creative,
}

#[derive(Debug, PartialEq, Command)]
#[command(name = "gm")]
struct GameMode<'c> {
    mode: Mode,
    #[arg(name = "player")]
    target: Option<PlayerName<'c>>,
}

#[derive(Debug, PartialEq, Command)]
enum Remind<'c> {
    Clear,
    #[command(default)]
    Later {
        seconds: u32,
        text: GreedyText<'c>,
    },
}

#[derive(Debug, PartialEq, CommandArgs)]
enum Toggle {
    On,
    Off,
}

#[derive(Debug, PartialEq, Command)]
enum Joins {
    Hide(Toggle),
}

fn error<C: Command>(text: &str) -> ArgumentError {
    let Some(Err(failure)) = parse::<C>(text) else {
        panic!("{text} parsed");
    };

    failure.error
}

#[test]
fn arguments_parse_in_order_and_optional_ones_may_be_left_out() {
    let parsed = parse::<GameMode>("gm 1 Steve").unwrap().unwrap();
    assert_eq!(parsed.mode, Mode::Creative);
    assert_eq!(parsed.target.unwrap().as_str(), "Steve");

    let parsed = parse::<GameMode>("gm survival").unwrap().unwrap();
    assert_eq!((parsed.mode, parsed.target), (Mode::Survival, None));

    assert!(parse::<GameMode>("gamemode survival").is_none());
    assert_eq!(error::<GameMode>("gm"), ArgumentError::Missing("mode"));
    assert_eq!(error::<GameMode>("gm survival Steve Alex"), ArgumentError::Extra);
}

#[test]
fn subcommands_match_by_name_before_the_default() {
    assert_eq!(parse::<Remind>("remind clear").unwrap(), Ok(Remind::Clear));
    assert_eq!(
        parse::<Remind>("remind 5  tea  is ready ").unwrap(),
        Ok(Remind::Later { seconds: 5, text: GreedyText("tea  is ready") })
    );

    assert_eq!(error::<Remind>("remind clear now"), ArgumentError::Extra);
    assert_eq!(error::<Remind>("remind"), ArgumentError::Missing("seconds"));
    assert!(matches!(error::<Remind>("remind -5 tea"), ArgumentError::Invalid { name: "seconds", .. }));

    assert_eq!(parse::<Joins>("joins hide on").unwrap(), Ok(Joins::Hide(Toggle::On)));
    assert_eq!(error::<Joins>("joins"), ArgumentError::Missing("subcommand"));
}

#[test]
fn errors_show_every_form_of_the_command() {
    let failure = parse::<Remind>("remind soon").unwrap().unwrap_err();
    assert_eq!(
        failure.to_string(),
        "Invalid seconds: Expected a positive whole number. Usage: /remind clear | /remind <seconds> <text>"
    );

    assert_eq!(<GameMode as Command>::SYNTAX.usage(GameMode::NAME), "/gm <survival|creative|1> [player]");
    assert_eq!(<Joins as Command>::SYNTAX.usage(Joins::NAME), "/joins hide on | /joins hide off");
}

fn literal(name: &'static str, children: Vec<i32>) -> CommandNode<'static> {
    let children = children.into_iter().map(VarInt).collect();

    CommandNode { kind: CommandNodeKind::Literal(name.into()), children, redirect: None, executable: true, restricted: false }
}

fn kind_of<'a>(tree: &'a Commands<'_>, index: &VarInt) -> &'a CommandNodeKind<'a> {
    &tree.nodes[index.0 as usize].kind
}

#[test]
fn a_spliced_command_replaces_the_server_one_of_the_same_name() {
    let root = CommandNode { kind: CommandNodeKind::Root, ..literal("", vec![0, 1]) };
    let mut tree = Commands { nodes: vec![literal("remind", vec![]), literal("tp", vec![]), root], root_index: 2 };

    splice(&mut tree, Remind::NAME, <Remind as Command>::SYNTAX);

    // The server's nodes keep their indices, but its `remind` is unlinked.
    let root = &tree.nodes[2];
    assert_eq!(root.children.len(), 2);
    assert_eq!(root.children[0].0, 1);

    let remind = &tree.nodes[root.children[1].0 as usize];
    assert_eq!(remind.kind, CommandNodeKind::Literal("remind".into()));
    assert!(!remind.executable);

    // `clear`, then `<seconds>` followed by `<text>`.
    let [clear, seconds] = &remind.children[..] else {
        panic!("remind has {} children", remind.children.len());
    };
    assert_eq!(kind_of(&tree, clear), &CommandNodeKind::Literal("clear".into()));

    let seconds = &tree.nodes[seconds.0 as usize];
    assert!(!seconds.executable);
    assert!(matches!(
        &seconds.kind,
        CommandNodeKind::Argument { parser: CommandParser::Long { min: Some(0), .. }, .. }
    ));

    let text = &tree.nodes[seconds.children[0].0 as usize];
    assert!(text.executable);
    assert!(matches!(
        &text.kind,
        CommandNodeKind::Argument { parser: CommandParser::String(StringMode::Greedy), .. }
    ));
}

#[test]
fn choices_become_literals_sharing_the_arguments_after_them() {
    let root = CommandNode { kind: CommandNodeKind::Root, ..literal("", vec![]) };
    let mut tree = Commands { nodes: vec![root], root_index: 0 };

    splice(&mut tree, GameMode::NAME, <GameMode as Command>::SYNTAX);

    let gm = &tree.nodes[tree.nodes[0].children[0].0 as usize];
    let modes: Vec<_> = gm.children.iter().map(|mode| kind_of(&tree, mode)).collect();
    assert_eq!(
        modes,
        [&CommandNodeKind::Literal("survival".into()), &CommandNodeKind::Literal("creative".into()), &CommandNodeKind::Literal("1".into())]
    );

    // Every mode runs alone or before the same optional player.
    let players: Vec<i32> = gm.children.iter().map(|mode| tree.nodes[mode.0 as usize].children[0].0).collect();
    assert!(players.iter().all(|&player| player == players[0]));
    assert!(gm.children.iter().all(|mode| tree.nodes[mode.0 as usize].executable));
}
