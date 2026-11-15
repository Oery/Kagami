use std::collections::VecDeque;
use std::io::Write;

use flate2::Compression;
use flate2::write::ZlibEncoder;
use futures_lite::future::block_on;

use super::*;
use crate::frame::tests::Chunks;

const CLIENTBOUND: Wire = Wire { direction: Direction::Clientbound, compressed: false };

#[test]
fn scanning_stops_at_an_incomplete_frame_and_follows_state_changes() {
    // Finish Configuration, then a Play frame, then half of another frame.
    let bytes = [1, 0x03, 2, 0x2d, 0xaa, 4, 0x01];
    let mut state = State::Config;

    assert_eq!(scan(Direction::Clientbound, &mut state, None, &bytes).unwrap(), 5);
    assert_eq!(state, State::Play);
}

#[test]
fn state_changes_are_found_inside_compressed_frames() {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&[0x03]).unwrap();
    let data = encoder.finish().unwrap();

    // Frame length, uncompressed length 1, then Finish Configuration deflated.
    let mut frame = vec![(data.len() + 1) as u8, 1];
    frame.extend_from_slice(&data);
    let mut state = State::Config;

    assert_eq!(scan(Direction::Clientbound, &mut state, Some(&mut Inflater::new()), &frame).unwrap(), frame.len());
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

    block_on(pump(CLIENTBOUND, State::Config, &mut buffer, &mut reader, &mut written)).unwrap();
    assert_eq!(written, frames);
}
