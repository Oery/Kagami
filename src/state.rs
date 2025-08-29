use atomic_enum::atomic_enum;
use strum::FromRepr;

#[atomic_enum]
#[derive(FromRepr, PartialEq)]
#[repr(i32)]
pub enum State {
    Handshake = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}
