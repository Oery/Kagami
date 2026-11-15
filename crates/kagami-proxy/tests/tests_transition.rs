use mc_protocol::Payload;

use super::*;

#[test]
fn each_direction_changes_state_on_its_own_packets() {
    assert_eq!(next_state(Direction::Clientbound, State::Config, 0x03), State::Play);
    assert_eq!(next_state(Direction::Clientbound, State::Play, 0x78), State::Config);
    assert_eq!(next_state(Direction::Serverbound, State::Config, 0x03), State::Play);
    assert_eq!(next_state(Direction::Serverbound, State::Play, 0x10), State::Config);

    // The same ids mean other packets in the other state or direction.
    assert_eq!(next_state(Direction::Clientbound, State::Play, 0x03), State::Play);
    assert_eq!(next_state(Direction::Serverbound, State::Play, 0x78), State::Play);
    assert_eq!(next_state(Direction::Clientbound, State::Config, 0x10), State::Config);
}

#[test]
fn packets_that_change_state_are_one_byte_long() {
    let mut lengths = Vec::new();

    for frame in [encode(&FinishConfiguration), encode(&StartConfiguration), encode(&AcknowledgeFinish), encode(&AcknowledgeConfiguration)]
    {
        // The frame's first byte is the packet's length.
        lengths.push(frame[0] as usize);
    }

    assert_eq!(lengths, [STATE_CHANGE_LEN; 4]);
}

fn encode<'p>(packet: &impl Payload<'p>) -> Vec<u8> {
    let mut frame = Vec::new();
    packet.encode(&mut frame).unwrap();

    frame
}
