use super::*;

#[test]
fn player_metadata_uses_the_target_clients_serializer_ids() {
    futures_lite::future::block_on(async {
        let packet = PlayerStateMetadata {
            entity_id: 1,
            skin_parts: 0x7f,
            sneaking: true,
            pose: PlayerPose::Crouching,
            swimming: false,
            sprinting: true,
            invisible: false,
            glowing: false,
            using: None,
            health: 20.0,
            effect_colors: &[],
            effect_ambient: false,
        };
        let mut bytes = Vec::new();
        packet.write_payload(&mut bytes).await.unwrap();
        assert_eq!(bytes.len(), packet.payload_len().unwrap());
        assert_eq!(
            bytes,
            [1, 0, 0, 0x0a, 6, 20, 5, 8, 0, 0, 9, 3, 0x41, 0xa0, 0, 0, 10, 17, 0, 11, 8, 0, 16, 0, 0x7f, 0xff]
        );
        let packet = PlayerStateMetadata { sneaking: false, ..packet };
        bytes.clear();
        packet.write_payload(&mut bytes).await.unwrap();
        assert_eq!(bytes[3], 0x08);
        assert_eq!(bytes[6], 5);
        let packet = PlayerStateMetadata { pose: PlayerPose::Swimming, swimming: true, ..packet };
        bytes.clear();
        packet.write_payload(&mut bytes).await.unwrap();
        assert_eq!(bytes[3], 0x18);
        assert_eq!(bytes[6], 3);
        let packet = PlayerStateMetadata { swimming: false, ..packet };
        bytes.clear();
        packet.write_payload(&mut bytes).await.unwrap();
        assert_eq!(bytes[3], 0x08);
        assert_eq!(bytes[6], 3);
        let packet = PlayerStateMetadata { using: Some(crate::packets::client::InteractionHand::Off), ..packet };
        bytes.clear();
        packet.write_payload(&mut bytes).await.unwrap();
        assert_eq!(bytes[9], 0x03);
        let packet = PlayerStateMetadata { health: 0.0, ..packet };
        bytes.clear();
        packet.write_payload(&mut bytes).await.unwrap();
        assert_eq!(bytes[6], 7);
    });
}

#[test]
fn player_metadata_carries_effect_flags_and_swirls() {
    futures_lite::future::block_on(async {
        let colors = [0x33ebff, 0x00f9_1234];
        let packet = PlayerStateMetadata {
            entity_id: 1,
            skin_parts: 0,
            sneaking: false,
            pose: PlayerPose::Standing,
            swimming: false,
            sprinting: false,
            invisible: true,
            glowing: true,
            using: None,
            health: 20.0,
            effect_colors: &colors,
            effect_ambient: true,
        };
        let mut bytes = Vec::new();
        packet.write_payload(&mut bytes).await.unwrap();
        assert_eq!(bytes.len(), packet.payload_len().unwrap());
        assert_eq!(bytes[3], 0x60);
        let particles = &bytes[16..];
        assert_eq!(&particles[..3], [10, 17, 2]);
        assert_eq!(particles[3], 28);
        assert_eq!(&particles[4..8], 0x33ebffi32.to_be_bytes());
        assert_eq!(particles[8], 28);
        assert_eq!(&particles[9..13], 0x00f9_1234i32.to_be_bytes());
        assert_eq!(&particles[13..16], [11, 8, 1]);
    });
}

fn frame<P: Payload<'static>>(packet: &P) -> Vec<u8> {
    futures_lite::future::block_on(async {
        let mut bytes = Vec::new();
        packet.write_packet(&mut bytes).await.unwrap();
        let mut input = bytes.as_slice();
        let length = VarInt::read(&mut input).unwrap().0 as usize;
        assert_eq!(input.len(), length);
        assert_eq!(VarInt::read(&mut input).unwrap().0, P::ID.0);
        assert_eq!(input.len(), packet.payload_len().unwrap());
        bytes
    })
}

#[test]
fn spawn_uses_player_registry_and_zero_lp_velocity_before_angles() {
    let packet = SpawnPlayer { entity_id: 128, uuid: Uuid::from_u128(1), x: 1.0, y: -2.0, z: 3.0, yaw: 90.0, pitch: -90.0 };
    let actual = frame(&packet);
    let mut expected = vec![50, 0x01, 0x80, 0x01];
    expected.extend_from_slice(&1u128.to_be_bytes());
    expected.extend_from_slice(&[0x9F, 0x01]);
    expected.extend_from_slice(&1.0f64.to_be_bytes());
    expected.extend_from_slice(&(-2.0f64).to_be_bytes());
    expected.extend_from_slice(&3.0f64.to_be_bytes());
    expected.extend_from_slice(&[0x00, 192, 64, 64, 0x00]);
    assert_eq!(actual, expected);
    let mut input = &actual[2..];
    let decoded = SpawnPlayer::decode_payload(&mut input).unwrap();
    assert_eq!(decoded.entity_id, 128);
    assert_eq!(decoded.uuid, packet.uuid);
    assert_eq!(decoded.yaw, 90.0);
    assert_eq!(decoded.pitch, 270.0);
    assert!(input.is_empty());
}

#[test]
fn movement_uses_properties_before_signed_delta_triplet() {
    assert_eq!(
        frame(&MoveEntityPosition { entity_id: 1, dx: i16::MIN, dy: i16::MAX, dz: -1, on_ground: true }),
        [9, 0x36, 1, 1, 0x80, 0, 0x7F, 0xFF, 0xFF, 0xFF],
    );
    assert_eq!(
        frame(&MoveEntityPositionRotation { entity_id: 1, dx: 4096, dy: -4096, dz: 0, yaw: -90.0, pitch: 90.0, on_ground: false }),
        [11, 0x37, 1, 0, 0x10, 0, 0xF0, 0, 0, 0, 192, 64],
    );
    assert_eq!(
        frame(&MoveEntityRotation { entity_id: 1, yaw: 180.0, pitch: 0.0, on_ground: true }),
        [5, 0x39, 1, 1, 128, 0],
    );
    assert_eq!(frame(&SetHeadRotation { entity_id: 1, yaw: 450.0 }), [3, 0x55, 1, 64],);
}

#[test]
fn absolute_position_uses_linear_path_and_float_angles() {
    let packet = EntityPositionSync { entity_id: 3, x: 1.0, y: 2.0, z: -3.0, yaw: 12.5, pitch: -35.0, on_ground: true };
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
        frame(&RemoveEntities { entity_ids: &[1, 128, -1] }),
        [10, 0x4E, 3, 1, 128, 1, 255, 255, 255, 255, 15]
    );
    assert_eq!(frame(&RemoveEntities { entity_ids: &[] }), [2, 0x4E, 0]);
    let mut expected = vec![34, 0x46, 2];
    expected.extend_from_slice(&0u128.to_be_bytes());
    expected.extend_from_slice(&1u128.to_be_bytes());
    assert_eq!(frame(&PlayerInfoRemove { uuids: &[Uuid::nil(), Uuid::from_u128(1)] }), expected);
}

#[test]
fn angles_wrap_and_quantize_negative_and_large_degrees() {
    assert_eq!(encode_angle(0.0), 0);
    assert_eq!(encode_angle(360.0), 0);
    assert_eq!(encode_angle(-1.0), 255);
    assert_eq!(encode_angle(90.0), 64);
    assert_eq!(encode_angle(-450.0), 192);
    assert_eq!(encode_angle(f32::MAX), 0);
}

#[test]
fn item_entities_use_the_item_stack_serializer_and_collect_packet() {
    static STACK: std::sync::LazyLock<crate::types::ItemStack> = std::sync::LazyLock::new(|| crate::types::ItemStack::plain(1, 3).unwrap());
    let stack = &*STACK;
    assert_eq!(frame(&ItemEntityMetadata { entity_id: 5, stack }), [9, 0x65, 5, 8, 7, 3, 1, 0, 0, 0xff]);
    assert_eq!(frame(&TakeItemEntity { item_id: 5, collector_id: 128, amount: 2 }), [5, 0x7F, 5, 0x80, 0x01, 2]);
}
