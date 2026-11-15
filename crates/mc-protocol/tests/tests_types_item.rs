use std::borrow::Cow;

use super::*;
use crate::types::{HolderSet, Nbt};

/// Decodes all of `bytes`, then checks the value encodes back to them.
fn round_trip<'p, T: PacketWriter<'p>>(bytes: &'p [u8]) -> T {
    let mut input = bytes;
    let value = T::read(&mut input).unwrap();
    assert!(input.is_empty(), "trailing bytes");

    let mut encoded = Vec::new();
    value.encode(&mut encoded).unwrap();
    assert_eq!(encoded, bytes);
    assert_eq!(value.body_len(), bytes.len());

    value
}

#[test]
fn a_zero_count_is_an_empty_slot() {
    let slot: Slot = round_trip(&[0]);

    assert_eq!(slot, Slot(None));
}

#[test]
fn a_slot_lists_added_components_then_removed_types() {
    let bytes = [
        1, 0x84, 0x07, // one of item 900
        3, 1, // three added, one removed
        3, 5, // damage 5
        6, 8, 0, 2, b'H', b'i', // custom name "Hi"
        4,    // unbreakable
        13,   // enchantments removed
    ];

    let Slot(Some(stack)) = round_trip(&bytes) else {
        panic!("the slot is empty");
    };

    assert_eq!((stack.item, stack.count), (900, 1));
    assert_eq!(
        stack.components.added,
        [Component::Damage(VarInt(5)), Component::CustomName(Nbt::Str(Cow::Borrowed("Hi"))), Component::Unbreakable]
    );
    assert_eq!(stack.components.removed, [VarInt(component_type::ENCHANTMENTS)]);
    assert_eq!(stack.component(component_type::DAMAGE), Some(&Component::Damage(VarInt(5))));
}

#[test]
fn an_untrusted_slot_prefixes_each_added_value_with_its_length() {
    let bytes = [1, 1, 2, 0, 3, 1, 5, 4, 0];

    let UntrustedSlot(Some(stack)) = round_trip(&bytes) else {
        panic!("the slot is empty");
    };

    assert_eq!(stack.components.added, [Component::Damage(VarInt(5)), Component::Unbreakable]);
}

#[test]
fn an_untrusted_value_must_fill_its_length() {
    let bytes = [1, 1, 1, 0, 3, 2, 5, 0];

    let result = UntrustedSlot::read(&mut bytes.as_slice());

    assert!(matches!(result, Err(DecodeError::TrailingData)));
}

#[test]
fn nested_stacks_put_the_item_before_the_count() {
    let bytes = [
        1, 10, 1, 0, // one of item 10, one added component
        52, 1, // bundle contents, one stack
        7, 2, 0, 0, // two of item 7, no changes
    ];

    let Slot(Some(stack)) = round_trip(&bytes) else {
        panic!("the slot is empty");
    };

    let inner = ItemStackTemplate(ItemStack::new(7, 2));
    assert_eq!(stack.components.added, [Component::BundleContents(vec![inner])]);
}

#[test]
fn holder_sets_send_a_tag_or_the_number_of_ids_plus_one() {
    let tool = Component::Tool(Tool {
        rules: vec![
            ToolRule { blocks: HolderSet::Tag("minecraft:mineable/pickaxe".into()), speed: Some(8.0), correct_for_drops: Some(true) },
            ToolRule { blocks: HolderSet::Ids(vec![VarInt(1), VarInt(2)]), speed: None, correct_for_drops: None },
        ],
        default_mining_speed: 1.0,
        damage_per_block: 1,
        can_destroy_blocks_in_creative: false,
    });

    let mut bytes = Vec::new();
    tool.encode(&mut bytes).unwrap();

    assert_eq!(bytes[..3], [28, 2, 0]);
    assert!(bytes.windows(5).any(|window| window == [3, 1, 2, 0, 0]));
    assert_eq!(round_trip::<Component>(&bytes), tool);
}

#[test]
fn an_unknown_component_type_does_not_decode() {
    let bytes = [1, 1, 1, 0, 122];

    assert!(matches!(Slot::read(&mut bytes.as_slice()), Err(DecodeError::EnumOutOfRange)));
}

/// A slot holding a bundle `depth` bundles deep.
fn nested_bundles(depth: usize) -> Vec<u8> {
    let mut bytes = vec![1, 10, 1, 0];

    for _ in 0..depth {
        // Bundle contents of one stack, holding one added component.
        bytes.extend([52, 1, 10, 1, 1, 0]);
    }

    // The innermost bundle is empty.
    bytes.extend([52, 0]);

    bytes
}

#[test]
fn deeply_nested_stacks_fail_instead_of_overflowing_the_stack() {
    let too_deep = nested_bundles(100);

    let result = Slot::read(&mut too_deep.as_slice());
    assert!(matches!(result, Err(DecodeError::NestedTooDeeply)));

    let shallow = nested_bundles(3);
    assert!(Slot::read(&mut shallow.as_slice()).is_ok());
}
