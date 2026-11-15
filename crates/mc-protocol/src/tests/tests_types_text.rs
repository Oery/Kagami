use super::*;

#[test]
fn empty_text_omits_the_extra_list_and_encodes_a_literal_empty_component() {
    let text = Text::default();
    assert_eq!(text.validate(), Ok(()));
    let nbt = text.nbt();
    assert_eq!(fields(&nbt).len(), 1);
    assert!(matches!(field(&nbt, "text"), Nbt::Str("")));
    let mut bytes = Vec::new();
    futures_lite::future::block_on(nbt.write(&mut bytes)).unwrap();
    assert_eq!(bytes, b"\x0a\x08\x00\x04text\x00\x00\x00");
    assert_eq!(nbt.body_len(), bytes.len());
}

fn fields<'a>(nbt: &'a Nbt<'a>) -> &'a [(&'a str, Nbt<'a>)] {
    let Nbt::Compound(fields) = nbt else {
        panic!("expected a compound")
    };
    fields
}

fn field<'a>(nbt: &'a Nbt<'a>, name: &str) -> &'a Nbt<'a> {
    &fields(nbt).iter().find(|(key, _)| *key == name).unwrap().1
}

#[test]
fn builder_spans_keep_their_order_and_styles() {
    let text = Text::new().gold("Round 3").plain(": ").push("Steve", Style::YELLOW.bold());
    assert_eq!(text.to_string(), "Round 3: Steve");
    let nbt = text.nbt();
    let Nbt::List(extra) = field(&nbt, "extra") else {
        panic!("expected extra spans")
    };
    assert_eq!(extra.len(), 3);
    assert!(matches!(field(&extra[0], "color"), Nbt::Str("gold")));
    assert!(matches!(field(&extra[0], "bold"), Nbt::Bool(false)));
    assert!(!fields(&extra[1]).iter().any(|(key, _)| *key == "color"));
    assert!(matches!(field(&extra[2], "color"), Nbt::Str("yellow")));
    assert!(matches!(field(&extra[2], "bold"), Nbt::Bool(true)));
    assert!(matches!(field(&extra[2], "text"), Nbt::Str("Steve")));
    assert_eq!(text.validate(), Ok(()));
}

#[test]
fn validation_rejects_unknown_colors_and_oversized_text() {
    assert_eq!(Text::new().push("x", Style::color("#12ab9F")).validate(), Ok(()));
    assert_eq!(Text::new().push("x", Style::color("golden")).validate(), Err(InvalidText));
    assert_eq!(Text::new().push("x", Style::color("#12ab9")).validate(), Err(InvalidText));
    assert_eq!(Text::from("x".repeat(MAX_TEXT_BYTES)).validate(), Err(InvalidText));
    let spans = (0..=MAX_SPANS).fold(Text::new(), |text, _| text.plain("x"));
    assert_eq!(spans.validate(), Err(InvalidText));
    let title = Title::new("ok").subtitle(Text::new().push("x", Style::color("nope")));
    assert_eq!(title.validate(), Err(InvalidText));
    assert_eq!(Title::new("ok").subtitle("fine").validate(), Ok(()));
}
