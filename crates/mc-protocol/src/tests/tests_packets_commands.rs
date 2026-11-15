use super::*;
use crate::packets::Serverbound;
use crate::packets::client::ClearTitles;
use crate::packets::server::{ChatCommand, CommandSuggestion};
use crate::{PacketWriter, Payload, State, VarInt};

#[test]
fn greedy_text_command_nodes_use_brigadier_string_mode_two() {
    let node = CommandNode { kind: CommandNodeKind::GreedyText { name: "message".into() }, children: vec![], executable: true };
    let mut body = Vec::new();
    node.encode(&mut body).unwrap();
    assert_eq!(node.body_len(), body.len());
    assert_eq!(&body[body.len() - 2..], &[5, 2]);
    let mut input = body.as_slice();
    assert_eq!(CommandNode::read(&mut input).unwrap().kind, node.kind);
    assert!(input.is_empty());
}

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

#[test]
fn command_tree_and_suggestions_use_brigadier_wire_fields() {
    let packet = Commands {
        nodes: vec![
            CommandNode { kind: CommandNodeKind::Word { name: "player".into(), suggest: true }, children: vec![], executable: true },
            CommandNode { kind: CommandNodeKind::Literal("creative".into()), children: vec![VarInt(0)], executable: true },
            CommandNode { kind: CommandNodeKind::Root, children: vec![VarInt(1)], executable: false },
        ],
        root_index: 2,
    };
    let bytes = encode(&packet);
    let mut input = bytes.as_slice();
    VarInt::read(&mut input).unwrap();
    assert_eq!(VarInt::read(&mut input).unwrap().0, 0x10);
    // Count; argument + executable + custom-suggestions flags; no children.
    assert_eq!(&input[..3], &[3, 0x16, 0]);
    let decoded = Commands::decode_payload(&mut input).unwrap();
    assert!(input.is_empty());
    assert_eq!(decoded.root_index, 2);
    assert_eq!(decoded.nodes[0].kind, CommandNodeKind::Word { name: "player".into(), suggest: true });
    assert_eq!(decoded.nodes[1].children[0].0, 0);
    assert_eq!(decoded.nodes[2].kind, CommandNodeKind::Root);

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

#[test]
fn numeric_command_arguments_encode_the_native_parser_ids_and_unbounded_flags() {
    for (parser, kind) in
        [(1, CommandNumberKind::Float), (2, CommandNumberKind::Double), (3, CommandNumberKind::Integer), (4, CommandNumberKind::Long)]
    {
        let packet = Commands {
            nodes: vec![CommandNode { kind: CommandNodeKind::Number { name: "n".into(), kind }, children: vec![], executable: true }],
            root_index: 0,
        };
        let bytes = encode(&packet);
        assert_eq!(&bytes[2..], &[1, 6, 0, 1, b'n', parser, 0, 0]);
        let mut input = &bytes[2..];
        let decoded = Commands::decode_payload(&mut input).unwrap();
        assert_eq!(decoded.nodes[0].kind, CommandNodeKind::Number { name: "n".into(), kind });
        assert!(input.is_empty());
    }
}

#[test]
fn position_command_arguments_use_the_vec3_singleton_parser() {
    let packet = Commands {
        nodes: vec![CommandNode { kind: CommandNodeKind::Position { name: "p".into() }, children: vec![], executable: true }],
        root_index: 0,
    };
    let bytes = encode(&packet);
    assert_eq!(&bytes[2..], &[1, 6, 0, 1, b'p', 10, 0]);
    let mut input = &bytes[2..];
    let decoded = Commands::decode_payload(&mut input).unwrap();
    assert_eq!(decoded.nodes[0].kind, CommandNodeKind::Position { name: "p".into() });
    assert!(input.is_empty());
}
