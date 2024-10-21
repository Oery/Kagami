use crate::kagami::callbacks::manager::CallbackManager;
use crate::kagami::callbacks::Actions;
use crate::minecraft::packets::handshake::client::SetProtocol;
use crate::minecraft::Packets;
use crate::serialization::{deserialize, ToVarInt, VarIntReader};
use crate::tcp::{utils::RawPacket, AtomicState, Origin, State};

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::io::AsyncReadExt;

// IDs of packets that may change the state of the connection
const STATE_PACKETS: [i32; 3] = [0x00, 0x02, 0x03];

pub async fn process_packets(
    raw_packets: &mut Vec<RawPacket>,
    is_compression_enabled: &Arc<AtomicBool>,
    s: &Arc<AtomicState>,
    origin: &Origin,
    callbacks: &Arc<CallbackManager>,
) {
    for raw_packet in raw_packets {
        let state = s.load(Ordering::Relaxed);

        let mut packet_data: Vec<u8> = Vec::new();
        let (ref mut _length, ref mut data) = raw_packet;

        let mut reader = std::io::Cursor::new(&data);

        let packet_id = match is_compression_enabled.load(Ordering::Relaxed) {
            true => {
                let _data_length = reader.read_varint().unwrap();
                reader.read_varint().unwrap()
            }
            false => reader.read_varint().unwrap(),
        };

        let _ = reader.read_to_end(&mut packet_data).await;

        // TODO: Those packets should be migrated to the callback system once it's implemented
        if STATE_PACKETS.contains(&packet_id) {
            match packet_id {
                0x00 if state == State::HandShaking && origin == &Origin::Client => {
                    let packet: SetProtocol = deserialize(&packet_data).unwrap();
                    s.store(packet.next_state, Ordering::Relaxed);
                    continue;
                }
                0x02 if state == State::Login => {
                    s.store(State::Play, Ordering::Relaxed);
                    continue;
                }
                0x03 if state == State::Login => {
                    is_compression_enabled.store(true, Ordering::Relaxed);
                    continue;
                }
                _ => {}
            }
        }

        if let Some(type_id) = callbacks.type_map.get(packet_id, &state, origin) {
            if callbacks.callbacks.contains_key(type_id) {
                let mut packet =
                    Packets::deserialize_packet(packet_id, &packet_data, &state, origin).unwrap();
                let action = callbacks.handle_packet(&mut packet).unwrap();
                match action {
                    Actions::Transfer => {}
                    Actions::Filter => {
                        raw_packet.0.clear();
                        raw_packet.1.clear();
                    }
                    Actions::Modify => {
                        println!("Raw packet: {:?}", raw_packet);
                        let (ref mut length, ref mut data) = raw_packet;
                        let packet_data =
                            Packets::serialize_packet(&packet, &state, origin).unwrap();
                        println!("Serialized packet: {:?}", packet_data);
                        data.clear();
                        data.push(0);
                        let varint = packet_id.to_varint().unwrap();
                        data.extend_from_slice(&varint);
                        data.extend_from_slice(&packet_data);
                        length.clear();
                        let varint = (data.len() as i32).to_varint().unwrap();
                        length.extend_from_slice(&varint);
                        println!("Serialized packet: {:?}", raw_packet);
                    }
                };
            }
        }
    }
}
