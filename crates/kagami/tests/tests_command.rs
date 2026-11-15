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
