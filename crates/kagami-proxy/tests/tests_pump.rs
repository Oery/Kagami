use std::collections::VecDeque;

use futures_lite::future::block_on;

use super::*;
use crate::frame::tests::Chunks;

#[test]
fn scanning_stops_at_an_incomplete_frame_and_follows_state_changes() {
    // Finish Configuration, then a Play frame, then half of another frame.
    let bytes = [1, 0x03, 2, 0x2d, 0xaa, 4, 0x01];
    let mut state = State::Config;
    assert_eq!(scan(Direction::Clientbound, &mut state, &bytes).unwrap(), 5);
    assert_eq!(state, State::Play);
}

#[test]
fn frames_are_forwarded_unchanged_however_they_arrive() {
    let frames = [1, 0x03, 3, 0x2d, 0xaa, 0xbb, 2, 0x78, 0x00];
    // Cut the stream inside a length prefix, a body and between frames.
    let chunks = VecDeque::from([frames[..1].to_vec(), frames[1..4].to_vec(), frames[4..].to_vec()]);
    let mut reader = Chunks(chunks);
    let mut written = Vec::new();
    let mut buffer = FrameBuffer::new();

    block_on(pump(Direction::Clientbound, State::Config, &mut buffer, &mut reader, &mut written)).unwrap();
    assert_eq!(written, frames);
}
