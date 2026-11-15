use super::*;
use futures_lite::future::block_on;

#[test]
fn tab_list_encodes_two_anonymous_nbt_components_with_exact_lengths() {
    for (header, footer, expected) in [
        (Nbt::Str("Hi"), Nbt::Str("Bye"), &b"\x08\x00\x02Hi\x08\x00\x03Bye"[..]),
        (
            Nbt::Compound(vec![("text", Nbt::Str(""))]),
            Nbt::Compound(vec![("text", Nbt::Str(""))]),
            &b"\x0a\x08\x00\x04text\x00\x00\x00\x0a\x08\x00\x04text\x00\x00\x00"[..],
        ),
    ] {
        let packet = TabList { header, footer };
        let mut bytes = Vec::new();
        block_on(packet.write_payload(&mut bytes)).unwrap();
        assert_eq!(bytes, expected);
        assert_eq!(packet.payload_len().unwrap(), bytes.len());
        let mut framed = Vec::new();
        block_on(packet.write_packet(&mut framed)).unwrap();
        let mut input = framed.as_slice();
        assert_eq!(VarInt::read(&mut input).unwrap().0 as usize, bytes.len() + 1);
        assert_eq!(VarInt::read(&mut input).unwrap().0, 0x7D);
        assert_eq!(input, bytes);
    }
}
