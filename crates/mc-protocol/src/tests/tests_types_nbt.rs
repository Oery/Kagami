use super::*;

#[test]
fn nbt_text_uses_java_modified_utf8_for_nul_and_supplementary_characters() {
    let value = Nbt::Str("A\0🏆");
    let mut bytes = Vec::new();
    futures_lite::future::block_on(value.write(&mut bytes)).unwrap();
    assert_eq!(bytes, [8, 0, 9, b'A', 0xC0, 0x80, 0xED, 0xA0, 0xBC, 0xED, 0xBF, 0x86]);
    assert_eq!(value.body_len(), bytes.len());
    let compound = Nbt::Compound(vec![("🏆", Nbt::Str("é"))]);
    bytes.clear();
    futures_lite::future::block_on(compound.write(&mut bytes)).unwrap();
    assert_eq!(compound.body_len(), bytes.len());
    assert!(bytes.windows(8).any(|w| w == [0, 6, 0xED, 0xA0, 0xBC, 0xED, 0xBF, 0x86]));
}

#[test]
fn synchronous_nbt_encoding_preserves_nested_wire_layout() {
    futures_lite::future::block_on(async {
        let value = Nbt::Compound(vec![("x", Nbt::List(vec![Nbt::Int(42)]))]);
        let mut bytes = Vec::new();
        value.write(&mut bytes).await.unwrap();
        assert_eq!(bytes, [10, 9, 0, 1, b'x', 3, 0, 0, 0, 1, 0, 0, 0, 42, 0]);
        assert_eq!(value.body_len(), bytes.len());
        let invalid = Nbt::List(vec![Nbt::Int(1), Nbt::Byte(2)]);
        assert!(invalid.write(&mut Vec::new()).await.is_err());
    });
}
