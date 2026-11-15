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
