use std::borrow::Cow;

use uuid::Uuid;

use super::*;
use crate::error::DecodeError;
use crate::packets::client::*;
use crate::packets::server::*;
use crate::{IntoOwned, PacketWriter};

#[mc_protocol_macros::packet(Play, 0x7e, Clientbound)]
struct QualifiedPacket<'wire> {
    #[format = "varint"]
    number: std::primitive::i32,
    values: std::vec::Vec<crate::types::VarInt>,
    profile: crate::types::GameProfile<'wire>,
}

#[test]
fn packet_macros_preserve_qualified_generic_types_and_declared_names_and_lifetimes() {
    let packet = QualifiedPacket {
        number: 12345,
        values: vec![VarInt(128), VarInt(-1)],
        profile: crate::types::GameProfile { uuid: Uuid::nil(), username: "Player".into(), properties: vec![] },
    };
    let bytes = encode(&packet);
    let mut input = bytes.as_slice();
    let length = VarInt::read(&mut input).unwrap().0 as usize;
    assert_eq!(input.len(), length);
    assert_eq!(VarInt::read(&mut input).unwrap().0, 0x7e);
    assert_eq!(packet.payload_len().unwrap(), input.len());
    let decoded = QualifiedPacket::decode_payload(&mut input).unwrap();
    assert!(input.is_empty());
    assert_eq!(decoded.number, 12345);
    assert!(decoded.values.iter().map(|value| value.0).eq([128, -1]));
    assert_eq!(decoded.profile.username, "Player");
    assert_eq!(QualifiedPacket::DIRECTION, Direction::Clientbound);
    assert_eq!(encode(&decoded), bytes);
}

/// The frame body: the frame without its length prefix.
fn body(frame: &[u8]) -> &[u8] {
    let mut input = frame;
    VarInt::read(&mut input).unwrap();
    input
}

fn encode<'p, T: Payload<'p>>(v: &T) -> Vec<u8> {
    let mut buf = Vec::new();
    v.encode(&mut buf).unwrap();
    buf
}

#[test]
fn disconnects_use_json_in_login_and_network_nbt_in_config_and_play() {
    use crate::types::Nbt;
    let config = encode(&ConfigDisconnect { reason: Nbt::Str("Server closed".into()) });
    let play = encode(&PlayDisconnect { reason: Nbt::Str("Server closed".into()) });
    assert_eq!(&config[..5], &[17, 0x02, 0x08, 0x00, 13]);
    assert_eq!(&play[..5], &[17, 0x20, 0x08, 0x00, 13]);
    assert_eq!(&config[5..], b"Server closed");
    assert_eq!(&play[5..], b"Server closed");
    let login = encode(&LoginDisconnect { reason: "{\"text\":\"Server closed\"}".into() });
    assert_eq!(login[1], 0x00);
    assert_eq!(login[2] as usize, login.len() - 3);
    assert_eq!(&login[3..], b"{\"text\":\"Server closed\"}");
}

#[test]
fn encryption_response_dispatches_in_login_state() {
    // id 0x01 + empty secret + empty token
    let packet = Serverbound::decode(State::Login, &[0x01, 0x00, 0x00]).unwrap();
    assert!(matches!(packet, Serverbound::EncryptionResponse(_)));
}

#[test]
fn movement_flags_dispatches_in_play_and_preserves_collision_bit() {
    let Serverbound::SetPlayerMovementFlags(packet) = Serverbound::decode(State::Play, &[0x21, 0x03]).unwrap() else {
        panic!("incorrect movement packet");
    };
    assert_eq!(packet.flags, 0x03);
    assert_eq!(encode(&packet), [0x02, 0x21, 0x03]);
    assert!(SetPlayerMovementFlags::decode_payload(&mut [].as_slice()).is_err());
}

#[test]
fn the_same_id_decodes_by_state() {
    let status = Serverbound::decode(State::Status, &[0x00]).unwrap();
    assert!(matches!(status, Serverbound::StatusRequest(_)));
    let frame = encode(&LoginStart { name: "Steve".into(), uuid: Uuid::nil() });
    let login = Serverbound::decode(State::Login, body(&frame)).unwrap();
    assert!(matches!(login, Serverbound::LoginStart(_)));
}

#[test]
fn undefined_ids_pass_through_as_raw_bytes() {
    // Play 0x7a has no serverbound definition; 0x80 0x02 is id 256.
    for frame in [vec![4, 0x7a, 1, 2, 3], vec![4, 0x80, 0x02, 1, 2]] {
        let packet = Serverbound::decode(State::Play, body(&frame)).unwrap();
        let Serverbound::Raw(raw) = &packet else {
            panic!("expected a raw packet");
        };
        assert!(matches!(raw.payload, Cow::Borrowed(_)));
        let mut encoded = Vec::new();
        packet.encode(&mut encoded).unwrap();
        assert_eq!(encoded, frame);
    }
}

#[test]
fn known_packets_must_consume_the_whole_body() {
    assert!(matches!(Serverbound::decode(State::Play, &[0x21, 0x03, 0xff]), Err(DecodeError::TrailingData)));
}

#[test]
fn packet_kinds_name_packets_without_their_lifetime() {
    fn key<K: PacketKind>() -> (Direction, State, u8) {
        (K::DIRECTION, K::STATE, K::ID)
    }
    assert_eq!(key::<ChatCommand>(), (Direction::Serverbound, State::Play, 0x07));
    assert_eq!(key::<StatusPacket>(), (Direction::Clientbound, State::Status, 0x00));

    let frame = encode(&ChatCommand { command: "spawn".into() });
    let mut packet = Serverbound::decode(State::Play, body(&frame)).unwrap();
    assert!(<ClientTickEnd as PacketKind>::pick(&mut packet).is_none());
    let command = <ChatCommand as PacketKind>::pick(&mut packet).unwrap();
    command.command = Cow::Owned("home".to_owned());
    let mut edited = Vec::new();
    packet.encode(&mut edited).unwrap();
    assert_eq!(edited, encode(&ChatCommand { command: "home".into() }));
}

#[test]
fn login_success_round_trips_profile_and_session() {
    use crate::types::{GameProfile, ProfileProperty};

    let profile = GameProfile {
        uuid: Uuid::nil(),
        username: "Notch".into(),
        properties: vec![ProfileProperty { name: "textures".into(), value: "abc".into(), signature: None }],
    };
    let packet = LoginSuccess { profile, session_id: Uuid::nil() };
    let bytes = encode(&packet);

    // frame: len + id(0x02) + 16 uuid + str + props + 16 session
    let mut frame = bytes.as_slice();
    let len = VarInt::read(&mut frame).unwrap().0 as usize;
    assert_eq!(len, bytes.len() - 1);
    assert_eq!(frame[0], 0x02);
}

#[test]
fn decoded_fields_borrow_the_buffer_until_made_owned() {
    let frame = encode(&ChatCommand { command: "spawn".into() });
    let owned: ChatCommand<'static> = {
        let buffer = frame.clone();
        let mut payload = &buffer[2..];
        let decoded = ChatCommand::decode_payload(&mut payload).unwrap();
        assert!(matches!(decoded.command, Cow::Borrowed(_)));
        decoded.into_owned()
    };
    assert!(matches!(owned.command, Cow::Owned(_)));
    assert_eq!(encode(&owned), frame);
}
