use std::borrow::Cow;

use super::*;

#[test]
fn nbt_text_uses_java_modified_utf8_for_nul_and_supplementary_characters() {
    let value = Nbt::Str("A\0🏆".into());
    let mut bytes = Vec::new();
    value.encode(&mut bytes).unwrap();
    assert_eq!(bytes, [8, 0, 9, b'A', 0xC0, 0x80, 0xED, 0xA0, 0xBC, 0xED, 0xBF, 0x86]);
    assert_eq!(value.body_len(), bytes.len());
    let compound = Nbt::Compound(vec![("🏆".into(), Nbt::Str("é".into()))]);
    bytes.clear();
    compound.encode(&mut bytes).unwrap();
    assert_eq!(compound.body_len(), bytes.len());
    assert!(bytes.windows(8).any(|w| w == [0, 6, 0xED, 0xA0, 0xBC, 0xED, 0xBF, 0x86]));
}

#[test]
fn synchronous_nbt_encoding_preserves_nested_wire_layout() {
    let value = Nbt::Compound(vec![("x".into(), Nbt::List(vec![Nbt::Int(42)]))]);
    let mut bytes = Vec::new();
    value.encode(&mut bytes).unwrap();
    assert_eq!(bytes, [10, 9, 0, 1, b'x', 3, 0, 0, 0, 1, 0, 0, 0, 42, 0]);
    assert_eq!(value.body_len(), bytes.len());
    let invalid = Nbt::List(vec![Nbt::Int(1), Nbt::Byte(2)]);
    assert!(invalid.encode(&mut Vec::new()).is_err());
}

fn decode(bytes: &[u8]) -> Result<Nbt<'_>, DecodeError> {
    let mut input = bytes;
    let value = Nbt::read(&mut input)?;
    assert!(input.is_empty(), "trailing bytes");

    Ok(value)
}

#[test]
fn decoding_round_trips_every_tag() {
    let value = Nbt::Compound(vec![
        ("text".into(), Nbt::Str("Steve".into())),
        ("bold".into(), Nbt::Byte(1)),
        ("numbers".into(), Nbt::List(vec![Nbt::Short(-2), Nbt::Short(3)])),
        ("wide".into(), Nbt::List(vec![Nbt::Long(1), Nbt::Long(-1)])),
        ("real".into(), Nbt::List(vec![Nbt::Float(0.5), Nbt::Float(2.0)])),
        ("precise".into(), Nbt::Double(0.25)),
        ("bytes".into(), Nbt::ByteArray(vec![1, 2, 3].into())),
        ("ints".into(), Nbt::IntArray(vec![7, -7])),
        ("longs".into(), Nbt::LongArray(vec![i64::MIN])),
        ("nested".into(), Nbt::Compound(vec![("extra".into(), Nbt::List(vec![]))])),
    ]);
    let mut bytes = Vec::new();
    value.encode(&mut bytes).unwrap();
    assert_eq!(value.body_len(), bytes.len());

    let decoded = decode(&bytes).unwrap();
    assert_eq!(decoded, value);

    let mut encoded = Vec::new();
    decoded.encode(&mut encoded).unwrap();
    assert_eq!(encoded, bytes);
}

#[test]
fn strings_are_borrowed_unless_they_need_modified_utf8_decoding() {
    let plain = [8, 0, 3, b'a', 0xc3, 0xa9];
    assert!(matches!(decode(&plain).unwrap(), Nbt::Str(Cow::Borrowed("aé"))));

    let modified = [8, 0, 9, b'A', 0xC0, 0x80, 0xED, 0xA0, 0xBC, 0xED, 0xBF, 0x86];
    let Nbt::Str(text) = decode(&modified).unwrap() else {
        panic!("expected a string");
    };
    assert!(matches!(text, Cow::Owned(_)));
    assert_eq!(text, "A\0🏆");

    // A lone surrogate is not a character.
    assert!(decode(&[8, 0, 3, 0xED, 0xA0, 0xBC]).is_err());
}

#[test]
fn malformed_and_oversized_values_are_rejected() {
    // A list claiming more ints than the input holds.
    assert!(decode(&[9, 3, 0x7f, 0xff, 0xff, 0xff, 0, 0, 0, 1]).is_err());
    // Elements without a type.
    assert!(decode(&[9, 0, 0, 0, 0, 1]).is_err());
    // A negative array length.
    assert!(decode(&[7, 0xff, 0xff, 0xff, 0xff]).is_err());
    // An unterminated compound.
    assert!(decode(&[10, 1, 0, 1, b'a', 5]).is_err());

    // Lists nested deeper than vanilla allows.
    let mut deep = vec![9];
    for _ in 0..MAX_DEPTH + 1 {
        deep.extend_from_slice(&[9, 0, 0, 0, 1]);
    }
    deep.extend_from_slice(&[0, 0, 0, 0, 0]);
    assert!(decode(&deep).is_err());
}
