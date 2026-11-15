//! Micro-benchmarks of the proxy's own work, on one thread and without
//! sockets: walking frames, running handlers on them, the cipher, and
//! inflating. The loopback benchmark measures the same work end to end.
//!
//! cargo bench -p kagami-proxy --bench micro

use std::hint::black_box;
use std::io::{Read, Write};
use std::marker::PhantomData;
use std::time::{Duration, Instant};

use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use glommio::LocalExecutorBuilder;
use kagami_proxy::testing::{Scanner, ciphers};
use kagami_proxy::{Action, Context, Handlers, Interest};
use mc_protocol::packets::client::{FinishConfiguration, PlayKeepAlive, SystemChatMessage};
use mc_protocol::packets::server::{AcknowledgeFinish, SetPlayerPosition};
use mc_protocol::packets::{Clientbound, PacketKind, Serverbound};
use mc_protocol::types::text::{Style, Text};
use mc_protocol::{Direction, PacketWriter, Payload, VarInt};

/// Bytes of frames walked per scan: about what one read brings.
const READ_LEN: usize = 256 * 1024;
const MEASURE: Duration = Duration::from_millis(500);
const COMPRESSION_THRESHOLD: usize = 256;
/// A play id with no handler in these benchmarks; the scan only reads ids.
const BULK_ID: i32 = 0x2e;
const CIPHER_LEN: usize = 64 * 1024;
const INFLATED_LEN: usize = 16 * 1024;

/// Runs the handlers of one packet kind, which always decide `action`.
struct Always<K> {
    action: Action,
    kind: PhantomData<fn() -> K>,
}

impl<K> Always<K> {
    fn new(action: Action) -> Self {
        Self { action, kind: PhantomData }
    }
}

impl<K: PacketKind> Handlers<()> for Always<K> {
    const INTEREST: Interest = Interest::NONE.with(K::DIRECTION, K::STATE, K::ID);

    fn serverbound(&mut self, packet: &mut Serverbound<'_>, _: &mut Context<'_, ()>) -> Action {
        black_box(K::from_serverbound(packet));

        self.action
    }

    fn clientbound(&mut self, packet: &mut Clientbound<'_>, _: &mut Context<'_, ()>) -> Action {
        black_box(K::from_clientbound(packet));

        self.action
    }
}

fn main() {
    let executor = LocalExecutorBuilder::default().name("bench-micro").make().unwrap();

    executor.run(async {
        println!("{:<34}{:>12}{:>14}", "", "per frame", "throughput");

        passthrough().await;
        handled().await;
        cipher();
        inflate();
    });
}

/// Frames no handler wants: the pump only reads their ids.
async fn passthrough() {
    let bulk = wire_frame(BULK_ID, &payload(64), None);
    let mut scanner = play_scanner(Direction::Clientbound, None, ()).await;
    report_scan("64 B, no handlers", &mut scanner, &bulk);

    let mut scanner = play_scanner(Direction::Clientbound, None, Always::<PlayKeepAlive>::new(Action::Forward)).await;
    report_scan("64 B, handlers on other ids", &mut scanner, &bulk);

    let compressed = Some(COMPRESSION_THRESHOLD);
    let small = wire_frame(BULK_ID, &payload(64), compressed);
    let mut scanner = play_scanner(Direction::Clientbound, compressed, ()).await;
    report_scan("64 B, compressed framing", &mut scanner, &small);

    // Above the threshold, the id hides in the deflate stream. It is only
    // read when something wants packets in the same direction and state.
    let deflated = wire_frame(BULK_ID, &payload(1024), compressed);
    let mut scanner = play_scanner(Direction::Clientbound, compressed, ()).await;
    report_scan("1 KiB deflated, no handlers", &mut scanner, &deflated);

    let mut scanner = play_scanner(Direction::Clientbound, compressed, Always::<PlayKeepAlive>::new(Action::Forward)).await;
    report_scan("1 KiB deflated, id peeked", &mut scanner, &deflated);
}

/// Frames with handlers: decoded, and encoded again when edited.
async fn handled() {
    let position = SetPlayerPosition { x: 12.5, feet_y: 64.0, z: -3.25, flags: 1 };
    let position = encode(&position);

    for (label, action) in [("position, forwarded", Action::Forward), ("position, edited", Action::Modified)] {
        let mut scanner = play_scanner(Direction::Serverbound, None, Always::<SetPlayerPosition>::new(action)).await;
        report_scan(label, &mut scanner, &position);
    }

    let text = Text::new().push("Steve", Style::color("yellow")).plain(" joined the game");
    let chat = encode(&SystemChatMessage { content: text.nbt(), overlay: false });

    for (label, action) in [("system chat, forwarded", Action::Forward), ("system chat, edited", Action::Modified)] {
        let mut scanner = play_scanner(Direction::Clientbound, None, Always::<SystemChatMessage>::new(action)).await;
        report_scan(label, &mut scanner, &chat);
    }
}

/// AES-CFB8 in place, over one read's worth of bytes.
fn cipher() {
    let secret = [7; 16];
    let (mut encryptor, mut decryptor) = ciphers(&secret);
    let mut bytes = payload(CIPHER_LEN);

    let per_call = measure(|| encryptor.encrypt(black_box(&mut bytes)));
    report_bytes("AES-CFB8 encrypting", per_call, CIPHER_LEN);

    let per_call = measure(|| decryptor.decrypt(black_box(&mut bytes)));
    report_bytes("AES-CFB8 decrypting", per_call, CIPHER_LEN);
}

/// What an uncompressed client leg would cost on top: inflating every
/// compressed packet from the server.
fn inflate() {
    let frame = wire_frame(BULK_ID, &payload(INFLATED_LEN), Some(COMPRESSION_THRESHOLD));
    let mut data = &frame[..];
    VarInt::read(&mut data).unwrap();
    VarInt::read(&mut data).unwrap();

    let mut packet = Vec::with_capacity(INFLATED_LEN + 8);
    let per_call = measure(|| {
        packet.clear();
        ZlibDecoder::new(black_box(data)).read_to_end(&mut packet).unwrap();
    });

    report_bytes("inflating a 16 KiB packet", per_call, INFLATED_LEN);
}

/// A scanner that has seen configuration finish, so it reads play ids.
async fn play_scanner<H: Handlers<()>>(direction: Direction, compression: Option<usize>, handlers: H) -> Scanner<(), H> {
    let mut scanner = Scanner::new(direction, compression.is_some(), (), handlers).await;

    let finish = match direction {
        Direction::Serverbound => encode(&AcknowledgeFinish),
        Direction::Clientbound => encode(&FinishConfiguration),
    };
    let finish = reframe(&finish, compression);
    scanner.scan(&finish).unwrap();

    scanner
}

/// Scans `READ_LEN` bytes of copies of `frame`, over and over, and prints
/// the time per frame.
fn report_scan<H: Handlers<()>>(label: &str, scanner: &mut Scanner<(), H>, frame: &[u8]) {
    let frames = READ_LEN / frame.len();
    let bytes = frame.repeat(frames);

    let per_call = measure(|| scanner.scan(black_box(&bytes)).unwrap());

    let per_frame = per_call.as_secs_f64() / frames as f64;
    println!("{label:<34}{:>9.1} ns{:>9.2} GiB/s", per_frame * 1e9, gibibytes_per_second(per_call, bytes.len()));
}

fn report_bytes(label: &str, per_call: Duration, bytes: usize) {
    println!("{label:<34}{:>12}{:>9.2} GiB/s", "", gibibytes_per_second(per_call, bytes));
}

/// Calls `f` for about `MEASURE`, after one call to warm up, and returns
/// the mean time per call.
fn measure(mut f: impl FnMut()) -> Duration {
    f();

    let started = Instant::now();
    let mut calls = 0;

    while started.elapsed() < MEASURE {
        f();
        calls += 1;
    }

    started.elapsed() / calls
}

fn gibibytes_per_second(per_call: Duration, bytes: usize) -> f64 {
    bytes as f64 / per_call.as_secs_f64() / (1u64 << 30) as f64
}

/// The uncompressed frame of `packet`.
fn encode<'p>(packet: &impl Payload<'p>) -> Vec<u8> {
    let mut frame = Vec::new();
    packet.encode(&mut frame).unwrap();

    frame
}

/// Puts an uncompressed frame into the framing `compression` sets.
fn reframe(frame: &[u8], compression: Option<usize>) -> Vec<u8> {
    let mut packet = frame;
    VarInt::read(&mut packet).unwrap();

    let mut id = packet;
    let id = VarInt::read(&mut id).unwrap().0;

    wire_frame(id, &packet[VarInt(id).body_len()..], compression)
}

/// A frame in the framing `compression` sets: compressed framing when it
/// is on, and deflated at or above its threshold.
fn wire_frame(id: i32, payload: &[u8], compression: Option<usize>) -> Vec<u8> {
    let mut packet = Vec::new();
    VarInt(id).encode(&mut packet).unwrap();
    packet.extend_from_slice(payload);

    let body = match compression {
        None => packet,
        Some(threshold) if packet.len() < threshold => [&[0], packet.as_slice()].concat(),
        Some(_) => {
            let mut body = Vec::new();
            VarInt(packet.len() as i32).encode(&mut body).unwrap();

            let mut encoder = ZlibEncoder::new(body, Compression::default());
            encoder.write_all(&packet).unwrap();
            encoder.finish().unwrap()
        }
    };

    let mut frame = Vec::new();
    VarInt(body.len() as i32).encode(&mut frame).unwrap();
    frame.extend_from_slice(&body);

    frame
}

/// Bytes that compress about as well as game data.
fn payload(len: usize) -> Vec<u8> {
    let mut state = 0x2545_f491_u32;

    (0..len)
        .map(|_| {
            state = state.wrapping_mul(1_103_515_245).wrapping_add(12_345);
            (state >> 24) as u8 & 0x0f
        })
        .collect()
}
