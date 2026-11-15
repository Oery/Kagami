use super::*;

fn round_trip(value: i64) -> Vec<u8> {
    let mut bytes = Vec::new();
    VarLong(value).encode(&mut bytes).unwrap();
    assert_eq!(bytes.len(), VarLong(value).body_len());

    let mut input = bytes.as_slice();
    assert_eq!(VarLong::read(&mut input).unwrap(), VarLong(value));
    assert!(input.is_empty());

    bytes
}

#[test]
fn varlongs_use_seven_bit_groups_and_ten_bytes_for_negatives() {
    assert_eq!(round_trip(0), [0x00]);
    assert_eq!(round_trip(300), [0xac, 0x02]);
    assert_eq!(round_trip(i64::MAX), [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]);
    assert_eq!(round_trip(-1), [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]);
    round_trip(i64::MIN);
}

#[test]
fn varlongs_beyond_64_bits_are_rejected() {
    let mut too_wide: &[u8] = &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x02];
    assert!(VarLong::read(&mut too_wide).is_err());

    let mut too_long: &[u8] = &[0x80; 11];
    assert!(VarLong::read(&mut too_long).is_err());
}
