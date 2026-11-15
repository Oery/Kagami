use super::*;
use crate::Payload;

#[test]
fn block_feedback_uses_protocol_777_field_order_and_lengths() {
    let mut bytes = Vec::new();
    let crack = SetBlockDestroyStage { entity_id: 300, location: -7, stage: 255 };
    crack.encode_payload(&mut bytes).unwrap();
    assert_eq!(bytes.len(), crack.payload_len().unwrap());
    assert_eq!(&bytes[..2], [0xac, 2]);
    assert_eq!(&bytes[2..10], (-7i64).to_be_bytes());
    assert_eq!(bytes[10], 255);
    bytes.clear();
    let entity = BlockEntityData { location: 5, kind: 7, data: Nbt::Compound(vec![]) };
    entity.encode_payload(&mut bytes).unwrap();
    assert_eq!(bytes.len(), entity.payload_len().unwrap());
    assert_eq!(&bytes[8..], [7, 10, 0]);
    bytes.clear();
    let event = WorldEvent { event: 1010, location: 5, data: 2, global: false };
    event.encode_payload(&mut bytes).unwrap();
    assert_eq!(bytes.len(), event.payload_len().unwrap());
    assert_eq!(&bytes[..4], 1010i32.to_be_bytes());
    assert_eq!(&bytes[12..], [0, 0, 0, 2, 0]);
}
