use super::*;

#[test]
fn collection_lengths_are_bounded_before_allocation() {
    for bytes in [&[0xff, 0xff, 0xff, 0xff, 0x0f][..], &[0xff, 0xff, 0xff, 0xff, 0x07]] {
        assert!(matches!(Vec::<u8>::read(&mut &bytes[..]), Err(DecodeError::InvalidLength)));
    }
    assert!(matches!(Vec::<u8>::read(&mut &[0x80, 0x80, 0x04][..]), Err(DecodeError::UnexpectedEof)));
    assert_eq!(Vec::<u8>::read(&mut &[2, 10, 20][..]).unwrap(), [10, 20]);
}
