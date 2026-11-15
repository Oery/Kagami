use super::*;
use crate::packets::Serverbound;
use crate::packets::client::ClearTitles;
use crate::packets::server::{ChatCommand, CommandSuggestion};
use crate::{IntoOwned, PacketWriter, Payload, State, VarInt};

fn encode<'p>(packet: &impl Payload<'p>) -> Vec<u8> {
    let mut bytes = Vec::new();
    packet.encode(&mut bytes).unwrap();
    let mut input = bytes.as_slice();
    assert_eq!(VarInt::read(&mut input).unwrap().0 as usize, input.len());
    VarInt::read(&mut input).unwrap();
    assert_eq!(packet.payload_len().unwrap(), input.len());

    bytes
}

#[test]
fn commands_and_completion_requests_dispatch_only_in_play() {
    for packet in [
        encode(&ChatCommand { command: "gamemode 1".into() }),
        encode(&CommandSuggestion { transaction_id: 42, text: "/gamemode creative Pl".into() }),
    ] {
        let mut input = packet.as_slice();
        VarInt::read(&mut input).unwrap();

        match Serverbound::decode(State::Play, input).unwrap() {
            Serverbound::ChatCommand(p) => assert_eq!(p.command, "gamemode 1"),
            Serverbound::CommandSuggestion(p) => {
                assert_eq!(p.transaction_id, 42);
                assert_eq!(p.text, "/gamemode creative Pl");
            }
            _ => panic!("wrong dispatch"),
        }
    }

    assert!(ChatCommand::decode_payload(&mut [8, b'g'].as_slice()).is_err());
    assert!(CommandSuggestion::decode_payload(&mut [42].as_slice()).is_err());
    assert!(matches!(Serverbound::decode(State::Handshake, &[0x07, 0]), Ok(Serverbound::Raw(_))));
}

/// Encodes `node` alone, checking its length, and decodes it back.
fn round_trip(node: &CommandNode<'_>) -> (Vec<u8>, CommandNode<'static>) {
    let mut body = Vec::new();
    node.encode(&mut body).unwrap();
    assert_eq!(node.body_len(), body.len());

    let mut input = body.as_slice();
    let decoded = CommandNode::read(&mut input).unwrap().into_owned();
    assert!(input.is_empty());

    (body, decoded)
}

fn argument(parser: CommandParser<'static>) -> CommandNode<'static> {
    let kind = CommandNodeKind::Argument { name: "n".into(), parser, suggestions: None };

    CommandNode { kind, children: vec![], redirect: None, executable: true, restricted: false }
}

#[test]
fn every_node_flag_and_field_survives_a_round_trip() {
    let suggested = CommandNode {
        kind: CommandNodeKind::Argument {
            name: "player".into(),
            parser: CommandParser::Plain(CommandParser::GAME_PROFILE),
            suggestions: Some("minecraft:ask_server".into()),
        },
        children: vec![VarInt(4)],
        redirect: None,
        executable: true,
        restricted: false,
    };
    let (body, decoded) = round_trip(&suggested);
    // Argument, executable and suggestions; one child.
    assert_eq!(&body[..3], &[0x16, 1, 4]);
    assert_eq!(decoded.kind, suggested.kind);

    let run = CommandNode {
        kind: CommandNodeKind::Literal("run".into()),
        children: vec![],
        redirect: Some(0),
        executable: false,
        restricted: true,
    };
    let (body, decoded) = round_trip(&run);
    // Literal, redirect and restricted; no children; redirect to the root.
    assert_eq!(&body[..3], &[0x29, 0, 0]);
    assert_eq!(decoded.redirect, Some(0));
    assert!(decoded.restricted && !decoded.executable);
}

#[test]
fn parsers_carry_their_properties() {
    for (parser, properties) in [
        (CommandParser::Integer { min: Some(0), max: None }, vec![3, 0x01, 0, 0, 0, 0]),
        (CommandParser::Double { min: None, max: None }, vec![2, 0]),
        (
            CommandParser::Long { min: Some(-1), max: Some(1) },
            [&[4, 0x03][..], &(-1i64).to_be_bytes(), &1i64.to_be_bytes()].concat(),
        ),
        (CommandParser::String(StringMode::Greedy), vec![5, 2]),
        (CommandParser::Entity { flags: 0x01 }, vec![6, 0x01]),
        (CommandParser::Time { min: 20 }, vec![43, 0, 0, 0, 20]),
        (
            CommandParser::Registry { id: 46, registry: "minecraft:item".into() },
            [&[46, 14][..], b"minecraft:item"].concat(),
        ),
        (CommandParser::Plain(CommandParser::VEC3), vec![10]),
    ] {
        let (body, decoded) = round_trip(&argument(parser));

        // Flags, no children and the name `n` come first.
        assert_eq!(&body[4..], properties);

        let mut again = Vec::new();
        decoded.encode(&mut again).unwrap();
        assert_eq!(again, body);
    }
}

#[test]
fn an_unknown_parser_fails_the_tree() {
    let last = [0x06, 0, 1, b'n', 61];
    assert!(CommandNode::read(&mut last.as_slice()).is_ok());

    let unknown = [0x06, 0, 1, b'n', 62];
    assert!(CommandNode::read(&mut unknown.as_slice()).is_err());
}

#[test]
fn a_tree_decodes_as_its_nodes_and_root() {
    let packet = Commands {
        nodes: vec![
            argument(CommandParser::Bool),
            CommandNode { kind: CommandNodeKind::Root, children: vec![VarInt(0)], redirect: None, executable: false, restricted: false },
        ],
        root_index: 1,
    };
    let bytes = encode(&packet);
    let mut input = bytes.as_slice();
    VarInt::read(&mut input).unwrap();
    assert_eq!(VarInt::read(&mut input).unwrap().0, 0x10);

    let decoded = Commands::decode_payload(&mut input).unwrap();
    assert!(input.is_empty());
    assert_eq!(decoded.root_index, 1);
    assert_eq!(decoded.nodes[0].kind, packet.nodes[0].kind);
    assert_eq!(decoded.nodes[1].children[0].0, 0);
}

#[test]
fn suggestions_answer_their_transaction() {
    let bytes = encode(&CommandSuggestionsResponse {
        transaction_id: 42,
        start: 10,
        length: 2,
        matches: vec![CommandMatch { text: "Player".into() }],
    });
    let mut input = bytes.as_slice();
    VarInt::read(&mut input).unwrap();
    assert_eq!(VarInt::read(&mut input).unwrap().0, 0x0f);
    let decoded = CommandSuggestionsResponse::decode_payload(&mut input).unwrap();
    assert!(input.is_empty());
    assert_eq!((decoded.transaction_id, decoded.start, decoded.length), (42, 10, 2));
    assert_eq!(decoded.matches[0].text, "Player");
}

#[test]
fn feedback_encodes_network_nbt() {
    assert_eq!(
        encode(&SystemChatMessage { content: Nbt::Str("ok".into()), overlay: false }),
        [7, 0x7c, 8, 0, 2, b'o', b'k', 0]
    );
    assert_eq!(
        encode(&SystemChatMessage { content: Nbt::Str("ok".into()), overlay: true }),
        [7, 0x7c, 8, 0, 2, b'o', b'k', 1]
    );
    assert_eq!(encode(&SetTitleText { text: Nbt::Str("ok".into()) }), [6, 0x74, 8, 0, 2, b'o', b'k']);
    assert_eq!(encode(&SetSubtitleText { text: Nbt::Str("".into()) }), [4, 0x72, 8, 0, 0]);
    assert_eq!(
        encode(&SetTitlesAnimation { fade_in: 10, stay: 70, fade_out: 20 }),
        [13, 0x75, 0, 0, 0, 10, 0, 0, 0, 70, 0, 0, 0, 20]
    );
    assert_eq!(encode(&ClearTitles { reset: true }), [2, 0x0e, 1]);
}
