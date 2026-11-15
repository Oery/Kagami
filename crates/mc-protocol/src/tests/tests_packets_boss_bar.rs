use super::*;

fn check(action: BossBarAction<'_>, expected: &[u8]) {
    let packet = BossBar { id: Uuid::from_bytes([0x42; 16]), action };
    let mut bytes = Vec::new();
    packet.encode_payload(&mut bytes).unwrap();
    assert_eq!(packet.payload_len().unwrap(), bytes.len());
    assert_eq!(&bytes[..16], &[0x42; 16]);
    assert_eq!(&bytes[16..], expected);
    // NBT is encode-only in this server. Other operations round-trip as well.
    if !matches!(packet.action, BossBarAction::Add { .. } | BossBarAction::Title(_)) {
        let mut input = bytes.as_slice();
        let decoded = BossBar::decode_payload(&mut input).unwrap();
        assert!(input.is_empty());
        let mut roundtrip = Vec::new();
        decoded.encode_payload(&mut roundtrip).unwrap();
        assert_eq!(bytes, roundtrip);
    }
    let mut framed = Vec::new();
    packet.encode(&mut framed).unwrap();
    let mut input = framed.as_slice();
    assert_eq!(VarInt::read(&mut input).unwrap().0 as usize, bytes.len() + 1);
    assert_eq!(VarInt::read(&mut input).unwrap().0, 0x09);
    assert_eq!(input, bytes);
}

#[test]
fn boss_bar_operations_have_vanilla_field_order_and_lengths() {
    // Anonymous TAG_String text component, followed by progress, style and flags.
    check(
        BossBarAction::Add {
            title: Nbt::Str("Hi"),
            progress: 0.5,
            color: BossBarColor::White,
            overlay: BossBarOverlay::Notched20,
            flags: 7,
        },
        &[0, 8, 0, 2, b'H', b'i', 0x3f, 0, 0, 0, 6, 4, 7],
    );
    check(BossBarAction::Remove, &[1]);
    check(BossBarAction::Progress(1.0), &[2, 0x3f, 0x80, 0, 0]);
    check(BossBarAction::Title(Nbt::Str("Hi")), &[3, 8, 0, 2, b'H', b'i']);
    check(BossBarAction::Style { color: BossBarColor::Red, overlay: BossBarOverlay::Notched6 }, &[4, 2, 1]);
    for flags in 0..8 {
        check(BossBarAction::Flags(flags), &[5, flags]);
    }
}

#[test]
fn boss_bar_enums_reject_unknown_values() {
    assert!(BossBarAction::read(&mut &[6][..]).is_err());
    assert!(BossBarColor::read(&mut &[7][..]).is_err());
    assert!(BossBarOverlay::read(&mut &[5][..]).is_err());
    for (value, expected) in [
        BossBarColor::Pink,
        BossBarColor::Blue,
        BossBarColor::Red,
        BossBarColor::Green,
        BossBarColor::Yellow,
        BossBarColor::Purple,
        BossBarColor::White,
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(BossBarColor::read(&mut &[value as u8][..]).unwrap(), expected);
    }
    for (value, expected) in [
        BossBarOverlay::Continuous,
        BossBarOverlay::Notched6,
        BossBarOverlay::Notched10,
        BossBarOverlay::Notched12,
        BossBarOverlay::Notched20,
    ]
    .into_iter()
    .enumerate()
    {
        assert_eq!(BossBarOverlay::read(&mut &[value as u8][..]).unwrap(), expected);
    }
}
