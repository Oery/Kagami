use super::*;
use crate::Payload;

#[test]
fn team_create_and_remove_match_protocol_777_field_order_and_lengths() {
    // Verified with the official 26.3 ClientboundSetPlayerTeamPacket decoder.
    let packet = SetPlayerTeam {
        name: "r".into(),
        action: TeamAction::Create(TeamProperties {
            display_name: Nbt::Str("d".into()),
            prefix: Nbt::Str("p".into()),
            suffix: Nbt::Str("".into()),
            name_tag_visibility: 0,
            collision_rule: 0,
            color: Some(VarInt(15)),
            friendly_flags: 1,
            entities: vec!["Oery".into()],
        }),
    };
    let mut bytes = Vec::new();
    packet.encode(&mut bytes).unwrap();
    let mut input = bytes.as_slice();
    assert_eq!(VarInt::read(&mut input).unwrap().0 as usize, input.len());
    assert_eq!(VarInt::read(&mut input).unwrap().0, 0x6f);
    assert_eq!(packet.payload_len().unwrap(), input.len());
    assert_eq!(
        input,
        [1, b'r', 0, 8, 0, 1, b'd', 8, 0, 1, b'p', 8, 0, 0, 0, 0, 1, 15, 1, 1, 4, b'O', b'e', b'r', b'y']
    );
    let TeamAction::Create(properties) = packet.action else {
        unreachable!()
    };
    let uncolored = SetPlayerTeam { name: "r".into(), action: TeamAction::Create(TeamProperties { color: None, ..properties }) };
    bytes.clear();
    uncolored.encode(&mut bytes).unwrap();
    let mut input = bytes.as_slice();
    assert_eq!(VarInt::read(&mut input).unwrap().0 as usize, input.len());
    assert_eq!(VarInt::read(&mut input).unwrap().0, 0x6f);
    assert_eq!(uncolored.payload_len().unwrap(), input.len());
    assert_eq!(input, [1, b'r', 0, 8, 0, 1, b'd', 8, 0, 1, b'p', 8, 0, 0, 0, 0, 0, 1, 1, 4, b'O', b'e', b'r', b'y']);
    bytes.clear();
    let packet = SetPlayerTeam { name: "r".into(), action: TeamAction::Remove };
    packet.encode(&mut bytes).unwrap();
    assert_eq!(bytes, [4, 0x6f, 1, b'r', 1]);
    assert_eq!(packet.payload_len().unwrap(), 3);
    assert!(matches!(SetPlayerTeam::decode_payload(&mut &bytes[2..]).unwrap().action, TeamAction::Remove));
    assert!(TeamAction::read(&mut &[2][..]).is_err());
}
