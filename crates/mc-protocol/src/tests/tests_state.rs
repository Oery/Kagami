use super::*;

#[test]
fn handshake_cannot_select_config_or_play() {
    for value in [-1, 0, 4, 5] {
        assert!(HandshakeIntent::try_from(value).is_err());
    }
    assert_eq!(HandshakeIntent::try_from(3).unwrap().state(), State::Login);
}
