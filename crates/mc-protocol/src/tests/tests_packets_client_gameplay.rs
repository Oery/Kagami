use super::*;

#[test]
fn damage_event_entity_ids_remain_varints_after_documentation_attributes() {
    futures_lite::future::block_on(async {
        let event = DamageEvent { entity_id: 1, source_type_id: 18, source_cause_id: 0, source_direct_id: 0, source_position: None };
        let mut bytes = Vec::new();
        event.write_packet(&mut bytes).await.unwrap();
        assert_eq!(bytes, [6, 0x19, 1, 18, 0, 0, 0]);
    });
}

#[test]
fn inventory_equipment_and_respawn_lengths_match_frames() {
    futures_lite::future::block_on(async {
        let empty = ItemStack::default();
        let slots = std::array::from_fn(|_| empty.clone());
        let packet = SetEquipment { entity_id: 1000, slots: &slots };
        let mut bytes = Vec::new();
        packet.write_packet(&mut bytes).await.unwrap();
        assert_eq!(bytes.len(), packet.payload_len().unwrap() + 2);
        assert_eq!(&bytes[4..], &[0x80, 0, 0x81, 0, 0x82, 0, 0x83, 0, 0x84, 0, 5, 0]);
        let respawn = Respawn {
            dimension_type: 0,
            dimension_name: "minecraft:overworld",
            hashed_seed: 0,
            game_mode: 0,
            previous_game_mode: 1,
            is_debug: false,
            is_flat: false,
            death_location: Some(DeathLocation { dimension: "minecraft:overworld", location: 1 }),
            portal_cooldown: 0,
            sea_level: 63,
            data_kept: 0,
        };
        bytes.clear();
        respawn.write_packet(&mut bytes).await.unwrap();
        let mut input = bytes.as_slice();
        let length = VarInt::read(&mut input).unwrap().0 as usize;
        assert_eq!(length, input.len());
        VarInt::read(&mut input).unwrap();
        Respawn::decode_payload(&mut input).unwrap();
        assert!(input.is_empty());
    });
}

#[test]
fn screen_packets_use_protocol_777_varint_window_ids() {
    futures_lite::future::block_on(async {
        let mut bytes = Vec::new();
        OpenScreen { window_id: 300, kind: 5, title: Nbt::Str("Games") }.write_packet(&mut bytes).await.unwrap();
        let mut input = bytes.as_slice();
        let length = VarInt::read(&mut input).unwrap().0 as usize;
        assert_eq!(input.len(), length);
        assert_eq!(VarInt::read(&mut input).unwrap().0, 0x3c);
        assert_eq!(VarInt::read(&mut input).unwrap().0, 300);
        assert_eq!(VarInt::read(&mut input).unwrap().0, 5);
        assert_eq!(input[0], 8); // unnamed network NBT string
        bytes.clear();
        CloseScreen { window_id: 300 }.write_packet(&mut bytes).await.unwrap();
        assert_eq!(bytes, [3, 0x11, 0xac, 2]);
    });
}

#[test]
fn chest_events_use_the_installed_clients_block_event_id_and_field_order() {
    futures_lite::future::block_on(async {
        let mut bytes = Vec::new();
        BlockEvent { location: 1, action: 1, parameter: 2, block: 300 }.write_packet(&mut bytes).await.unwrap();
        assert_eq!(bytes, [13, 0x07, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 0xac, 2]);
    });
}
