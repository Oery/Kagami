use super::*;

fn frame<P: Payload<'static>>(packet: &P) -> Vec<u8> {
    let mut bytes = Vec::new();
    packet.encode(&mut bytes).unwrap();
    let mut input = bytes.as_slice();
    let length = VarInt::read(&mut input).unwrap().0 as usize;
    assert_eq!(input.len(), length);
    assert_eq!(VarInt::read(&mut input).unwrap().0, P::ID.0);
    assert_eq!(input.len(), packet.payload_len().unwrap());
    bytes
}

#[test]
fn movement_uses_properties_before_signed_delta_triplet() {
    assert_eq!(
        frame(&MoveEntityPosition { entity_id: 1, properties: 1, dx: i16::MIN, dy: i16::MAX, dz: -1 }),
        [9, 0x36, 1, 1, 0x80, 0, 0x7F, 0xFF, 0xFF, 0xFF],
    );
    assert_eq!(
        frame(&MoveEntityPositionRotation { entity_id: 1, properties: 0, dx: 4096, dy: -4096, dz: 0, yaw: 192, pitch: 64 }),
        [11, 0x37, 1, 0, 0x10, 0, 0xF0, 0, 0, 0, 192, 64],
    );
    assert_eq!(
        frame(&MoveEntityRotation { entity_id: 1, on_ground: true, yaw: 128, pitch: 0 }),
        [5, 0x39, 1, 1, 128, 0]
    );
    assert_eq!(frame(&SetHeadRotation { entity_id: 1, yaw: 64 }), [3, 0x55, 1, 64]);
}

#[test]
fn absolute_position_uses_linear_path_and_float_angles() {
    let packet = EntityPositionSync { entity_id: 3, path_type: 0, x: 1.0, y: 2.0, z: -3.0, yaw: 12.5, pitch: -35.0, on_ground: true };
    let mut expected = vec![36, 0x23, 3, 0];
    expected.extend_from_slice(&1.0f64.to_be_bytes());
    expected.extend_from_slice(&2.0f64.to_be_bytes());
    expected.extend_from_slice(&(-3.0f64).to_be_bytes());
    expected.extend_from_slice(&12.5f32.to_be_bytes());
    expected.extend_from_slice(&(-35.0f32).to_be_bytes());
    expected.push(1);
    assert_eq!(frame(&packet), expected);
    let mut input = &expected[2..];
    let decoded = EntityPositionSync::decode_payload(&mut input).unwrap();
    assert_eq!(decoded.x, packet.x);
    assert_eq!(decoded.yaw, packet.yaw);
    assert!(decoded.on_ground);
    assert!(input.is_empty());
}

#[test]
fn removal_lists_use_varint_counts_and_entity_ids() {
    assert_eq!(
        frame(&RemoveEntities { entity_ids: vec![VarInt(1), VarInt(128), VarInt(-1)] }),
        [10, 0x4E, 3, 1, 128, 1, 255, 255, 255, 255, 15]
    );
    assert_eq!(frame(&RemoveEntities { entity_ids: vec![] }), [2, 0x4E, 0]);
    let mut expected = vec![34, 0x46, 2];
    expected.extend_from_slice(&0u128.to_be_bytes());
    expected.extend_from_slice(&1u128.to_be_bytes());
    assert_eq!(frame(&PlayerInfoRemove { uuids: vec![Uuid::nil(), Uuid::from_u128(1)] }), expected);
}

#[test]
fn angles_wrap_and_quantize_negative_and_large_degrees() {
    assert_eq!(encode_angle(0.0), 0);
    assert_eq!(encode_angle(360.0), 0);
    assert_eq!(encode_angle(-1.0), 255);
    assert_eq!(encode_angle(90.0), 64);
    assert_eq!(encode_angle(-450.0), 192);
    assert_eq!(encode_angle(f32::MAX), 0);
    for angle in 0..=u8::MAX {
        assert_eq!(encode_angle(decode_angle(angle)), angle);
    }
}

#[test]
fn collect_packet_uses_varint_ids() {
    assert_eq!(frame(&TakeItemEntity { item_id: 5, collector_id: 128, amount: 2 }), [5, 0x7F, 5, 0x80, 0x01, 2]);
}
