use super::*;
use crate::{Payload, VarInt};

fn frame<'p, P: Payload<'p>>(packet: &P) -> Vec<u8> {
    let mut bytes = Vec::new();
    packet.encode(&mut bytes).unwrap();
    assert_eq!(bytes.len(), 1 + VarInt(P::ID.0).body_len() + packet.payload_len().unwrap());

    bytes
}

#[test]
fn map_colors_without_a_patch_are_a_single_zero_column_count() {
    let unchanged = MapData { map_id: 3, scale: 0, locked: false, icons: None, colors: MapColors::Unchanged };
    assert_eq!(frame(&unchanged), [6, 0x34, 3, 0, 0, 0, 0]);

    let patch = MapPatch { columns: 1, rows: 2, x: 5, z: 6, colors: vec![9, 10].into() };
    let patched = MapData { map_id: 3, scale: 0, locked: false, icons: Some(vec![]), colors: MapColors::Patch(patch) };
    let bytes = frame(&patched);
    assert_eq!(bytes, [13, 0x34, 3, 0, 0, 1, 0, 1, 2, 5, 6, 2, 9, 10]);

    let mut input = &bytes[2..];
    let decoded = MapData::decode_payload(&mut input).unwrap();
    assert!(input.is_empty());
    assert!(matches!(decoded.colors, MapColors::Patch(MapPatch { rows: 2, .. })));
}

#[test]
fn waypoints_pick_uuid_or_name_with_a_leading_bool() {
    let waypoint = Waypoint {
        operation: waypoint_operation::TRACK,
        id: WaypointId::Uuid(Uuid::from_u128(1)),
        style: "default".into(),
        color: Some(Rgb { red: 255, green: 0, blue: 0 }),
        target: WaypointTarget::Chunk { x: 1, z: -1 },
    };

    // The id 0x8D takes two bytes as a VarInt.
    let bytes = frame(&waypoint);
    let payload = &bytes[3..];
    assert_eq!(&payload[..2], [0, 1]);

    let mut input = payload;
    let decoded = Waypoint::decode_payload(&mut input).unwrap();
    assert!(input.is_empty());
    assert!(matches!(decoded.id, WaypointId::Uuid(uuid) if uuid == Uuid::from_u128(1)));
    assert_eq!(decoded.target, WaypointTarget::Chunk { x: 1, z: -1 });
}
