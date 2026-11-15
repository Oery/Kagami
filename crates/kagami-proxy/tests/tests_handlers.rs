use super::*;

#[test]
fn interest_is_kept_per_direction_state_and_id() {
    let interest = Interest::NONE.with(Direction::Serverbound, State::Play, 0x07).with(Direction::Clientbound, State::Config, 200);

    assert!(interest.contains(Direction::Serverbound, State::Play, 0x07));
    assert!(interest.contains(Direction::Clientbound, State::Config, 200));
    assert!(!interest.contains(Direction::Clientbound, State::Play, 0x07));
    assert!(!interest.contains(Direction::Serverbound, State::Config, 0x07));
    assert!(!interest.contains(Direction::Serverbound, State::Play, 0x08));
}

#[test]
fn a_drop_wins_then_a_modification() {
    assert_eq!(Action::Forward.then(Action::Forward), Action::Forward);
    assert_eq!(Action::Modified.then(Action::Forward), Action::Modified);
    assert_eq!(Action::Forward.then(Action::Modified), Action::Modified);
    assert_eq!(Action::Modified.then(Action::Drop), Action::Drop);
    assert_eq!(Action::Drop.then(Action::Modified), Action::Drop);
}
