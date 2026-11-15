use super::*;
use crate::error::DecodeError;
use crate::types::particle::{ParticleOptions, particle_type};

#[test]
fn entries_are_indexed_typed_values_ending_with_0xff() {
    let bytes = [
        0, 0, 0x02, // flags: sneaking
        2, 6, 0, // no custom name
        6, 20, 5, // pose: crouching
        8, 7, 0, // empty item stack
        9, 16, 21, 0, 0xFF, 0, 0, 0x3F, 0x80, 0, 0, // red dust, scale 1
        0xFF,
    ];

    let mut input = bytes.as_slice();
    let metadata = EntityMetadata::read(&mut input).unwrap();
    assert!(input.is_empty());

    assert_eq!(metadata.get(0), Some(&MetadataValue::Byte(0x02)));
    assert_eq!(metadata.get(2), Some(&MetadataValue::OptionalText(None)));
    assert_eq!(metadata.get(6), Some(&MetadataValue::Pose(VarInt(5))));
    assert_eq!(metadata.get(8), Some(&MetadataValue::ItemStack(Slot(None))));

    let dust = Particle { kind: particle_type::DUST, options: ParticleOptions::Dust { color: 0xFF0000, scale: 1.0 } };
    assert_eq!(metadata.get(9), Some(&MetadataValue::Particle(dust)));

    let mut encoded = Vec::new();
    metadata.encode(&mut encoded).unwrap();
    assert_eq!(encoded, bytes);
    assert_eq!(metadata.body_len(), bytes.len());
}

#[test]
fn an_unknown_serializer_does_not_decode() {
    let bytes = [0, 44, 0xFF];

    let result = EntityMetadata::read(&mut bytes.as_slice());

    assert!(matches!(result, Err(DecodeError::EnumOutOfRange)));
}
