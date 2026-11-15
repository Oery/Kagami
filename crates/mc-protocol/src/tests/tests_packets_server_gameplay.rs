use super::*;

#[test]
fn flight_requests_dispatch_with_a_single_flags_byte() {
    use crate::packets::{Context, Packets};
    let ctx = Context { state: State::Play, ..Default::default() };
    let mut input = [0x28, 0x02].as_slice();
    let Packets::PlayerAbilitiesRequest(request) = Packets::read(&mut input, &ctx).unwrap() else {
        panic!("wrong abilities packet");
    };
    assert!(input.is_empty());
    assert_eq!(request.flags, 2);
    assert!(PlayerAbilitiesRequest::decode_payload(&mut [].as_slice()).is_err());
}

#[test]
fn click_container_keeps_hashed_predictions() {
    // window/state/slot/button/mode, one changed slot: 36 -> item 1 x64 with
    // one added component hash and one removed component, cursor empty.
    let bytes = [0, 0, 0, 36, 0, 0, 1, 0, 36, 1, 1, 64, 1, 5, 0, 0, 0, 7, 1, 9, 0];
    let mut input = bytes.as_slice();
    let click = ClickContainer::decode_payload(&mut input).unwrap();
    assert!(input.is_empty());
    let changed = &click.changed_slots[0];
    assert_eq!(changed.slot, 36);
    let item = changed.item.as_ref().unwrap();
    assert_eq!((item.item_id, item.count), (1, 64));
    assert_eq!((item.added[0].component, item.added[0].hash), (5, 7));
    assert_eq!(item.removed[0].0, 9);
    assert!(click.carried.is_none());

    let mut encoded = Vec::new();
    click.encode_payload(&mut encoded).unwrap();
    assert_eq!(encoded, bytes);
    assert_eq!(click.payload_len().unwrap(), bytes.len());
    assert!(ClickContainer::decode_payload(&mut &bytes[..bytes.len() - 1]).is_err());
}
