use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_lite::future::block_on;

use mc_protocol::{PacketWriter, VarInt};

use super::*;

/// Returns one queued chunk per read, like a socket receiving segments.
pub struct Chunks(pub VecDeque<Vec<u8>>);

impl AsyncRead for Chunks {
    fn poll_read(mut self: Pin<&mut Self>, _: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
        let Some(mut chunk) = self.0.pop_front() else {
            return Poll::Ready(Ok(0));
        };
        let len = chunk.len().min(buf.len());
        buf[..len].copy_from_slice(&chunk[..len]);
        if len < chunk.len() {
            self.0.push_front(chunk.split_off(len));
        }
        Poll::Ready(Ok(len))
    }
}

#[test]
fn length_prefixes_are_parsed_once_complete_and_bounded() {
    assert_eq!(FrameHeader::parse(&[]).unwrap(), None);
    assert_eq!(FrameHeader::parse(&[0x80]).unwrap(), None);
    assert_eq!(FrameHeader::parse(&[2, 0xaa]).unwrap(), Some(FrameHeader { header_len: 1, body_len: 2 }));
    assert_eq!(FrameHeader::parse(&[0x80, 0x01]).unwrap(), Some(FrameHeader { header_len: 2, body_len: 128 }));
    assert_eq!(FrameHeader::parse(&[0xff, 0xff, 0x7f]).unwrap().unwrap().body_len, MAX_FRAME_LEN);
    assert!(FrameHeader::parse(&[0x80, 0x80, 0x80]).is_err());
    assert!(FrameHeader::parse(&[0x80, 0x80, 0x80, 0x01]).is_err());
}

#[test]
fn frames_split_across_reads_are_taken_once_complete() {
    let mut reader = Chunks(VecDeque::from([vec![3, 1], vec![2, 3, 1], vec![9]]));
    let mut buffer = FrameBuffer::new();

    assert!(block_on(buffer.fill(&mut reader)).unwrap());
    assert_eq!(buffer.take_frame().unwrap(), None);
    assert!(block_on(buffer.fill(&mut reader)).unwrap());
    let body = buffer.take_frame().unwrap().unwrap();
    assert_eq!(buffer.slice(body), [1, 2, 3]);
    assert_eq!(buffer.take_frame().unwrap(), None);
    assert!(block_on(buffer.fill(&mut reader)).unwrap());
    let body = buffer.take_frame().unwrap().unwrap();
    assert_eq!(buffer.slice(body), [9]);
    assert!(!block_on(buffer.fill(&mut reader)).unwrap());
}

#[test]
fn the_buffer_grows_for_frames_larger_than_it() {
    let body = vec![7; INITIAL_CAPACITY * 3];
    let mut frame = Vec::new();
    VarInt(body.len() as i32).encode(&mut frame).unwrap();
    frame.extend_from_slice(&body);
    let mut reader = Chunks(VecDeque::from([frame]));
    let mut buffer = FrameBuffer::new();

    let mut taken = None;
    while taken.is_none() {
        assert!(block_on(buffer.fill(&mut reader)).unwrap());
        taken = buffer.take_frame().unwrap();
    }
    assert_eq!(buffer.slice(taken.unwrap()), body);
}
