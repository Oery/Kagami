use super::*;
use crate::error::EncodeError;
use crate::{PacketWriter, Payload, VarInt};

fn payload(packet: &PlayerInfoUpdate) -> Vec<u8> {
    let mut bytes = Vec::new();
    packet.encode(&mut bytes).unwrap();

    let mut input = bytes.as_slice();
    let length = VarInt::read(&mut input).unwrap().0 as usize;
    assert_eq!(input.len(), length);
    assert_eq!(VarInt::read(&mut input).unwrap().0, 0x47);
    assert_eq!(input.len(), packet.payload_len().unwrap());

    input.to_vec()
}

#[test]
fn entries_carry_the_fields_of_each_action_bit_in_bit_order() {
    let uuid = Uuid::from_u128(1);
    let packet = PlayerInfoUpdate {
        actions: ADD_PLAYER | UPDATE_LISTED | UPDATE_LATENCY,
        entries: vec![PlayerInfoEntry {
            uuid,
            added: Some(AddedPlayer { name: "Steve".into(), properties: vec![] }),
            listed: Some(true),
            latency: Some(300),
            // Not flagged, so never written.
            show_hat: Some(true),
            ..Default::default()
        }],
    };

    let bytes = payload(&packet);
    let mut expected = vec![ADD_PLAYER | UPDATE_LISTED | UPDATE_LATENCY, 1];
    expected.extend_from_slice(uuid.as_bytes());
    expected.extend_from_slice(&[5, b'S', b't', b'e', b'v', b'e', 0, 1, 0xac, 0x02]);
    assert_eq!(bytes, expected);

    let mut input = bytes.as_slice();
    let decoded = PlayerInfoUpdate::decode_payload(&mut input).unwrap();
    assert!(input.is_empty());

    let entry = &decoded.entries[0];
    assert_eq!(entry.added.as_ref().unwrap().name, "Steve");
    assert_eq!(entry.listed, Some(true));
    assert_eq!(entry.latency, Some(300));
    assert!(entry.show_hat.is_none());
    assert!(entry.chat_session.is_none());
}

#[test]
fn a_flagged_action_without_its_field_does_not_encode() {
    let packet = PlayerInfoUpdate { actions: UPDATE_GAME_MODE, entries: vec![PlayerInfoEntry::default()] };

    let mut bytes = Vec::new();
    let result = packet.encode(&mut bytes);

    assert!(matches!(result, Err(EncodeError::MissingField)));
}
