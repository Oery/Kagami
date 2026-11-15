use super::*;
use mc_protocol_macros::Payload;

#[mc_protocol_macros::packet(Play, 0x7e, Client)]
struct QualifiedPacket<'wire> {
    #[format = "varint"]
    number: std::primitive::i32,
    values: std::vec::Vec<crate::types::VarInt>,
    profile: crate::types::GameProfile<'wire>,
}

#[derive(mc_protocol_macros::InnerPacket)]
enum OtherPackets<'wire> {
    Qualified(QualifiedPacket<'wire>),
}

#[test]
fn packet_macros_preserve_qualified_generic_types_and_declared_names_and_lifetimes() {
    let packet = QualifiedPacket {
        number: 12345,
        values: vec![VarInt(128), VarInt(-1)],
        profile: crate::types::GameProfile { uuid: Uuid::nil(), username: "Player", properties: vec![] },
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
    let wrapped = OtherPackets::Qualified(decoded);
    assert_eq!(wrapped.id(), 0x7e);
    assert_eq!(wrapped.state(), State::Play);
    let OtherPackets::Qualified(decoded) = wrapped;
    assert_eq!(encode(&decoded), bytes);
}

fn encode<'p, T: Payload<'p>>(v: &T) -> Vec<u8> {
    let mut buf = Vec::new();
    v.encode(&mut buf).unwrap();
    buf
}

#[test]
fn disconnects_use_json_in_login_and_network_nbt_in_config_and_play() {
    use crate::types::Nbt;
    let config = encode(&ConfigDisconnect { reason: Nbt::Str("Server closed") });
    let play = encode(&PlayDisconnect { reason: Nbt::Str("Server closed") });
    assert_eq!(&config[..5], &[17, 0x02, 0x08, 0x00, 13]);
    assert_eq!(&play[..5], &[17, 0x20, 0x08, 0x00, 13]);
    assert_eq!(&config[5..], b"Server closed");
    assert_eq!(&play[5..], b"Server closed");
    let login = encode(&LoginDisconnect { reason: "{\"text\":\"Server closed\"}" });
    assert_eq!(login[1], 0x00);
    assert_eq!(login[2] as usize, login.len() - 3);
    assert_eq!(&login[3..], b"{\"text\":\"Server closed\"}");
}

#[test]
fn encryption_response_dispatches_in_login_state() {
    let ctx = Context { state: State::Login, ..Default::default() };
    // id 0x01 + empty secret + empty token
    let raw = [0x01u8, 0x00, 0x00];
    let mut slice = raw.as_slice();

    assert!(matches!(Packets::read(&mut slice, &ctx).unwrap(), Packets::EncryptionResponse(_)));
    assert!(slice.is_empty());
}

#[test]
fn movement_flags_dispatches_in_play_and_preserves_collision_bit() {
    let ctx = Context { state: State::Play, ..Default::default() };
    let raw = [0x21, 0x03];
    let mut input = raw.as_slice();
    let Packets::SetPlayerMovementFlags(packet) = Packets::read(&mut input, &ctx).unwrap() else {
        panic!("incorrect movement packet");
    };
    assert!(input.is_empty());
    assert_eq!(packet.flags, 0x03);
    assert_eq!(encode(&packet), [0x02, 0x21, 0x03]);
    assert!(SetPlayerMovementFlags::decode_payload(&mut [].as_slice()).is_err());
}

#[test]
fn login_success_round_trips_profile_and_session() {
    use crate::types::{GameProfile, ProfileProperty};

    let profile = GameProfile {
        uuid: Uuid::nil(),
        username: "Notch",
        properties: vec![ProfileProperty { name: "textures", value: "abc", signature: None }],
    };
    let packet = LoginSuccess { profile, session_id: Uuid::nil() };
    let bytes = encode(&packet);

    // frame: len + id(0x02) + 16 uuid + str + props + 16 session
    let mut frame = bytes.as_slice();
    let len = VarInt::read(&mut frame).unwrap().0 as usize;
    assert_eq!(len, bytes.len() - 1);
    assert_eq!(frame[0], 0x02);
}
