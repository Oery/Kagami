use super::*;

#[test]
fn item_limits_identity_and_component_wire_lengths() {
    let sword = ITEMS.iter().position(|item| item.0 == "minecraft:diamond_sword").unwrap() as i32;
    assert_eq!(ItemStack::plain(sword, 2).unwrap_err(), ItemError::InvalidCount);
    let mut item = ItemStack::new(
        sword,
        1,
        ItemComponents {
            damage: Some(5),
            unbreakable: Some(false),
            name: Some("Sword".into()),
            lore: vec!["Arena".into()],
            dye: Some(0x123456),
            glint: Some(true),
            ..Default::default()
        },
    )
    .unwrap();
    let plain = ItemStack::plain(sword, 1).unwrap();
    assert!(!item.same_item(&plain));
    let bytes = futures_lite::future::block_on(async {
        let mut bytes = Vec::new();
        item.write(&mut bytes).await.unwrap();
        bytes
    });
    assert_eq!(item.body_len(), bytes.len());
    assert_eq!(&bytes[bytes.len() - 7..], &[1, 47, 0, 0x12, 0x34, 0x56, 4]);
    assert_eq!(item.take(1).count(), 1);
    assert!(item.is_empty());
    assert_eq!(item.body_len(), 1);
}

#[test]
fn enchantments_and_potions_encode_as_registry_ids() {
    let book = ITEMS.iter().position(|item| item.0 == "minecraft:enchanted_book").unwrap() as i32;
    let protection = crate::data::item_registries::enchantment_id("minecraft:projectile_protection").unwrap();
    let components = ItemComponents { stored_enchantments: vec![(protection, 4)], ..Default::default() };
    let item = ItemStack::new(book, 1, components).unwrap();
    let bytes = futures_lite::future::block_on(async {
        let mut bytes = Vec::new();
        item.write(&mut bytes).await.unwrap();
        bytes
    });
    assert_eq!(item.body_len(), bytes.len());
    // One added component, none removed, then type 45: one (ID, level) pair.
    assert_eq!(&bytes[bytes.len() - 6..], &[1, 0, 45, 1, protection as u8, 4]);

    let potion = ITEMS.iter().position(|item| item.0 == "minecraft:potion").unwrap() as i32;
    let regeneration = crate::data::item_registries::potion_id("minecraft:regeneration").unwrap();
    let nausea = crate::data::item_registries::mob_effect_id("minecraft:nausea").unwrap();
    let effect = PotionEffect { effect: nausea, amplifier: 0, duration: 600, ambient: false, particles: true, icon: true };
    let contents = PotionContents { potion: Some(regeneration), color: None, effects: vec![effect] };
    let item = ItemStack::new(potion, 1, ItemComponents { potion: Some(contents), ..Default::default() }).unwrap();
    let bytes = futures_lite::future::block_on(async {
        let mut bytes = Vec::new();
        item.write(&mut bytes).await.unwrap();
        bytes
    });
    assert_eq!(item.body_len(), bytes.len());
    // Potion present, no color, one effect (600 is VarInt 0xd8 0x04), no name.
    let tail = [1, regeneration as u8, 0, 1, nausea as u8, 0, 0xd8, 4, 0, 1, 1, 0, 0];
    assert_eq!(&bytes[bytes.len() - tail.len()..], &tail);

    let unknown = ItemComponents { enchantments: vec![(999, 1)], ..Default::default() };
    assert_eq!(ItemStack::new(book, 1, unknown).unwrap_err(), ItemError::InvalidComponents);
}
