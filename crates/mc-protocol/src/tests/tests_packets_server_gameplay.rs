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
fn hashed_predictions_are_bounded_and_never_become_items() {
    // window/state/slot/button/mode/count, slot36 -> stone x64/no overrides, cursor empty
    let bytes = [0, 0, 0, 36, 0, 0, 1, 0, 36, 1, 1, 64, 0, 0, 0];
    let mut input = bytes.as_slice();
    let click = ClickContainer::decode_payload(&mut input).unwrap();
    assert_eq!(click.predicted_slots, 1 << 36);
    assert!(input.is_empty());
    assert!(ClickContainer::decode_payload(&mut &bytes[..bytes.len() - 1]).is_err());
    let excessive = [0, 0, 0, 36, 0, 0, 0x81, 1];
    assert!(ClickContainer::decode_payload(&mut excessive.as_slice()).is_err());
    let mut unknown = bytes;
    unknown[10] = 0;
    assert!(ClickContainer::decode_payload(&mut unknown.as_slice()).is_err());
}

#[test]
fn predictions_include_slot_89_and_reject_duplicates_and_out_of_range() {
    let bytes = [1, 0, 0, 89, 0, 0, 1, 0, 89, 0, 0];
    let click = ClickContainer::decode_payload(&mut bytes.as_slice()).unwrap();
    assert_eq!(click.predicted_slots, 1u128 << 89);
    let mut out = bytes;
    out[8] = 90;
    assert!(ClickContainer::decode_payload(&mut out.as_slice()).is_err());
    let duplicate = [1, 0, 0, 89, 0, 0, 2, 0, 89, 0, 0, 89, 0, 0];
    assert!(ClickContainer::decode_payload(&mut duplicate.as_slice()).is_err());
}
