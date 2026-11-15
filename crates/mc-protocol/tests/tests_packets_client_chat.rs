use std::borrow::Cow;

use super::*;
use crate::{PacketWriter, Payload, VarInt};

fn round_trip(packet: &PlayerChat) -> Vec<u8> {
    let mut bytes = Vec::new();
    packet.encode(&mut bytes).unwrap();

    let mut input = bytes.as_slice();
    VarInt::read(&mut input).unwrap();
    VarInt::read(&mut input).unwrap();
    assert_eq!(input.len(), packet.payload_len().unwrap());

    let decoded = PlayerChat::decode_payload(&mut input).unwrap();
    assert!(input.is_empty());

    let mut again = Vec::new();
    decoded.encode(&mut again).unwrap();
    assert_eq!(again, bytes);

    bytes
}

fn decoration() -> ChatTypeDecoration<'static> {
    ChatTypeDecoration {
        translation_key: "chat.type.text".into(),
        parameters: vec![VarInt(chat_parameter::SENDER), VarInt(chat_parameter::CONTENT)],
        style: Nbt::Compound(vec![]),
    }
}

#[test]
fn player_chat_round_trips_cached_and_full_signatures_and_inline_chat_types() {
    let signature = Signature(Cow::Owned(vec![7; 256]));
    let packet = PlayerChat {
        global_index: 4,
        sender: Uuid::from_u128(2),
        index: 1,
        signature: None,
        message: "hello".into(),
        timestamp: 1_700_000_000_000,
        salt: -5,
        previous_messages: vec![IdOr::Id(3), IdOr::Inline(signature)],
        unsigned_content: None,
        filter: FilterMask::PartiallyFiltered(vec![0b101]),
        chat_type: IdOr::Inline(ChatType { chat: decoration(), narration: decoration() }),
        sender_name: Nbt::Str("Alex".into()),
        target_name: None,
    };

    let bytes = round_trip(&packet);

    // After the header and body, a cached signature is sent as its ID plus
    // one, and a full one after a zero.
    let frame_header = bytes.len() - packet.payload_len().unwrap();
    let previous = frame_header + 1 + 16 + 1 + 1 + 6 + 8 + 8;
    assert_eq!(&bytes[previous..previous + 4], [2, 4, 0, 7]);
}
