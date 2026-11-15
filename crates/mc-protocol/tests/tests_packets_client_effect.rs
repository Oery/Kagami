use super::*;
use std::borrow::Cow;

fn encode<'p>(packet: &impl Payload<'p>) -> Vec<u8> {
    let mut bytes = Vec::new();
    packet.encode_payload(&mut bytes).unwrap();
    assert_eq!(bytes.len(), packet.payload_len().unwrap());

    bytes
}

#[test]
fn swing_animation_carries_hand_kind_and_duration() {
    let packet = SwingAnimation { entity_id: 300, hand: InteractionHand::Off as i32, kind: SwingKind::Whack as i32, duration: SWING_TICKS };
    assert_eq!(encode(&packet), [0xac, 0x02, 1, 1, 6]);
}

#[test]
fn inline_sound_event_round_trips_without_a_registry_id() {
    let packet = SoundEffect {
        sound: InlineSound { name: "a:b".into(), fixed_range: None },
        category: sound_category::PLAYER,
        x: 8,
        y: -16,
        z: 0,
        volume: 1.0,
        pitch: 1.0,
        seed: 7,
    };
    let bytes = encode(&packet);
    assert_eq!(&bytes[..7], [0, 3, b'a', b':', b'b', 0, 7]);
    assert_eq!(&bytes[7..11], 8i32.to_be_bytes());
    assert_eq!(&bytes[11..15], (-16i32).to_be_bytes());
    assert_eq!(&bytes[bytes.len() - 8..], 7i64.to_be_bytes());
    let mut input = bytes.as_slice();
    let sound = InlineSound::read(&mut input).unwrap();
    assert_eq!(sound.name, "a:b");
    assert_eq!(sound.fixed_range, None);
}

#[test]
fn stop_sound_flags_control_optional_fields_and_frame_lengths() {
    for (category, name, expected) in [
        (None, None, vec![0]),
        (Some(7), None, vec![1, 7]),
        (None, Some("a:b"), vec![2, 3, b'a', b':', b'b']),
        (Some(7), Some("a:b"), vec![3, 7, 3, b'a', b':', b'b']),
    ] {
        let packet = StopSound { category, name: name.map(Cow::Borrowed) };
        assert_eq!(encode(&packet), expected);
        let mut input = expected.as_slice();
        assert_eq!(StopSound::decode_payload(&mut input).unwrap(), packet);
        assert!(input.is_empty());
        let mut frame = Vec::new();
        packet.encode(&mut frame).unwrap();
        assert_eq!(frame[0] as usize, expected.len() + 1);
        assert_eq!(frame[1], 0x79);
        assert_eq!(&frame[2..], expected);
    }

    // Both the identifier and packet length can require multi-byte VarInts.
    let name = format!("a:{}", "b".repeat(128));
    let packet = StopSound { category: Some(7), name: Some((&name).into()) };
    let mut frame = Vec::new();
    packet.encode(&mut frame).unwrap();
    let mut input = frame.as_slice();
    assert_eq!(VarInt::read(&mut input).unwrap().0 as usize, input.len());
    assert_eq!(VarInt::read(&mut input).unwrap().0, 0x79);
    assert_eq!(StopSound::decode_payload(&mut input).unwrap(), packet);
    assert!(input.is_empty());

    for invalid in [&[4][..], &[1][..], &[2][..], &[3, 7][..]] {
        assert!(StopSound::decode_payload(&mut &invalid[..]).is_err());
    }
}

#[test]
fn level_particles_put_the_particle_type_first() {
    let packet = LevelParticles {
        particle: particle::SWEEP_ATTACK,
        override_limiter: false,
        always_visible: false,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        offset_x: 0.0,
        offset_y: 0.0,
        offset_z: 0.0,
        speed_x: 0.0,
        speed_y: 0.0,
        speed_z: 0.0,
        count: 2,
        randomization: 0,
    };
    let bytes = encode(&packet);
    assert_eq!(bytes.len(), 1 + 2 + 24 + 24 + 1 + 1);
    assert_eq!(bytes[0], 77);
    assert_eq!(&bytes[bytes.len() - 2..], [2, 0]);
}

#[test]
fn mob_effect_updates_use_varints_and_presentation_flags() {
    let packet = UpdateMobEffect {
        entity_id: 300,
        effect_id: 27,
        amplifier: 1,
        duration: -1,
        flags: mob_effect_flags::PARTICLES | mob_effect_flags::ICON,
    };
    let bytes = encode(&packet);
    assert_eq!(bytes, [0xac, 0x02, 27, 1, 0xff, 0xff, 0xff, 0xff, 0x0f, 0x06]);
    assert_eq!(UpdateMobEffect::decode_payload(&mut bytes.as_slice()).unwrap().duration, -1);
    assert_eq!(UpdateMobEffect::ID.0, 0x87);
    assert_eq!(encode(&RemoveMobEffect { entity_id: 1, effect_id: 13 }), [1, 13]);
    assert_eq!(RemoveMobEffect::ID.0, 0x4F);
}

#[test]
fn attribute_snapshots_round_trip_with_modifiers() {
    let packet = UpdateAttributes {
        entity_id: 1,
        attributes: vec![AttributeSnapshot {
            attribute: attribute::MOVEMENT_SPEED,
            base: 0.1,
            modifiers: vec![AttributeModifier {
                id: "minecraft:effect.speed".into(),
                amount: 0.4,
                operation: attribute_operation::ADD_MULTIPLIED_TOTAL,
            }],
        }],
    };
    let bytes = encode(&packet);
    assert_eq!(&bytes[..3], [1, 1, 26]);
    assert_eq!(&bytes[3..11], 0.1f64.to_be_bytes());
    assert_eq!(bytes[11], 1);
    assert_eq!(*bytes.last().unwrap(), 2);
    let mut input = bytes.as_slice();
    let decoded = UpdateAttributes::decode_payload(&mut input).unwrap();
    assert!(input.is_empty());
    assert_eq!(decoded.attributes, packet.attributes);
    assert_eq!(UpdateAttributes::ID.0, 0x86);
}
