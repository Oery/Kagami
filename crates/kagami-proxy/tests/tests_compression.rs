use std::io::Write;

use flate2::Compression;
use flate2::write::ZlibEncoder;

use super::*;

/// A compressed frame body: the uncompressed length, then the zlib data.
fn compressed_body(packet: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(packet).unwrap();
    let data = encoder.finish().unwrap();

    let mut body = Vec::new();
    VarInt(packet.len() as i32).encode(&mut body).unwrap();
    body.extend_from_slice(&data);

    body
}

#[test]
fn ids_are_read_from_uncompressed_and_compressed_bodies() {
    let mut inflater = Inflater::new();
    assert_eq!(inflater.packet_id(&[0, 0x2e, 1, 2], true).unwrap(), Some(0x2e));

    let mut packet = vec![0x80, 0x01];
    packet.extend(std::iter::repeat_n(7, 1000));
    assert_eq!(inflater.packet_id(&compressed_body(&packet), true).unwrap(), Some(128));
    // The stream is reset between frames.
    assert_eq!(inflater.packet_id(&compressed_body(&[0x03]), true).unwrap(), Some(0x03));
}

#[test]
fn unwanted_ids_are_only_inflated_when_they_could_change_the_state() {
    let mut inflater = Inflater::new();
    let mut packet = vec![0x2e];
    packet.extend(std::iter::repeat_n(7, 1000));

    assert_eq!(inflater.packet_id(&compressed_body(&packet), false).unwrap(), None);
    assert_eq!(inflater.packet_id(&compressed_body(&[0x03]), false).unwrap(), Some(0x03));
    assert_eq!(inflater.packet_id(&[0, 0x2e, 1, 2], false).unwrap(), Some(0x2e));
}

#[test]
fn packets_are_borrowed_when_uncompressed_and_inflated_otherwise() {
    let mut inflater = Inflater::new();
    let mut inflated = Vec::new();
    assert_eq!(inflater.packet(&[0, 0x2e, 1], &mut inflated).unwrap(), [0x2e, 1]);

    let packet: Vec<u8> = (0..=255).collect();
    let body = compressed_body(&packet);
    assert_eq!(inflater.packet(&body, &mut inflated).unwrap(), packet);

    let mut wrong_length = body.clone();
    wrong_length[0] = 0x7f;
    assert!(inflater.packet(&wrong_length, &mut inflated).is_err());
}

#[test]
fn proxy_packets_are_framed_uncompressed() {
    let mut frames = vec![2, 0x2e, 9];
    add_zero_data_length(&mut frames, 0).unwrap();
    assert_eq!(frames, [3, 0, 0x2e, 9]);

    // A frame appended after others.
    frames.extend_from_slice(&[1, 0x03]);
    add_zero_data_length(&mut frames, 4).unwrap();
    assert_eq!(frames, [3, 0, 0x2e, 9, 2, 0, 0x03]);
}
