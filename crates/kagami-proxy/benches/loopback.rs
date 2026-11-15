//! End-to-end loopback benchmark: a client talks to a minimal offline server,
//! once directly and once through the proxy, and compares throughput and
//! round-trip latency.
//!
//! cargo bench -p kagami-proxy --bench loopback

use std::net::{SocketAddr, TcpListener as StdTcpListener};
use std::thread;
use std::time::{Duration, Instant};

use futures_lite::{AsyncReadExt, AsyncWriteExt};
use glommio::LocalExecutorBuilder;
use glommio::net::{TcpListener, TcpStream};
use mc_protocol::packets::client::{FinishConfiguration, LoginSuccess};
use mc_protocol::packets::server::{AcknowledgeFinish, LoginAcknowledge, LoginStart, ServerListPing};
use mc_protocol::packets::{Clientbound, RawPacket, Serverbound};
use mc_protocol::state::HandshakeIntent;
use mc_protocol::types::GameProfile;
use mc_protocol::{PROTOCOL_VERSION, PacketWriter, Payload, State, VarInt};
use uuid::Uuid;

/// Clientbound Play ids that change no state; the proxy only reads ids.
const BULK_ID: i32 = 0x2e;
const ECHO_REQUEST_ID: i32 = 0x1c;
const ECHO_RESPONSE_ID: i32 = 0x2d;

const STREAMED_BYTES: usize = 512 * 1024 * 1024;
const ROUND_TRIPS: usize = 20_000;
const WRITE_BATCH: usize = 256 * 1024;

/// The client picks a scenario through its login name.
#[derive(Clone, Copy)]
enum Scenario {
    /// The server streams frames with this body size.
    Stream { body_len: usize },
    /// The server echoes each fixed-size request.
    Echo,
}

impl Scenario {
    fn name(self) -> String {
        match self {
            Scenario::Stream { body_len } => format!("stream{body_len}"),
            Scenario::Echo => "echo".to_owned(),
        }
    }

    fn parse(name: &str) -> Self {
        match name.strip_prefix("stream") {
            Some(body_len) => Scenario::Stream { body_len: body_len.parse().unwrap() },
            None => Scenario::Echo,
        }
    }
}

fn main() {
    let server = start_on_thread("bench-server", serve_forever);
    let proxy = free_address();
    thread::spawn(move || kagami_proxy::run(proxy, &server.to_string()).unwrap());

    let executor = LocalExecutorBuilder::default().name("bench-client").make().unwrap();
    executor.run(async move {
        println!("{:<24}{:>14}{:>14}", "", "direct", "proxied");

        for body_len in [16 * 1024, 64] {
            let label = format!("throughput {body_len} B");
            let direct = throughput(server, body_len).await;
            let proxied = throughput(proxy, body_len).await;
            println!("{label:<24}{:>9.2} GiB/s{:>9.2} GiB/s", direct, proxied);
        }

        let direct = latencies(server).await;
        let proxied = latencies(proxy).await;

        for (label, quantile) in [("latency p50", 0.50), ("latency p99", 0.99), ("latency p99.9", 0.999)] {
            let direct = percentile(&direct, quantile);
            let proxied = percentile(&proxied, quantile);
            println!("{label:<24}{:>11.1} µs{:>11.1} µs", micros(direct), micros(proxied));
        }
    });
}

/// Streams `STREAMED_BYTES` and returns the client's receive rate in GiB/s.
async fn throughput(address: SocketAddr, body_len: usize) -> f64 {
    let mut stream = join(address, Scenario::Stream { body_len }).await;
    let mut buffer = vec![0; 256 * 1024];
    let started = Instant::now();

    let mut received = 0;

    while received < streamed_frame_count(body_len) * bulk_frame(body_len).len() {
        let read = stream.read(&mut buffer).await.unwrap();
        assert!(read > 0, "server closed the stream early");
        received += read;
    }

    received as f64 / started.elapsed().as_secs_f64() / (1u64 << 30) as f64
}

/// Measures every round trip of a small serverbound request echoed back.
async fn latencies(address: SocketAddr) -> Vec<Duration> {
    let mut stream = join(address, Scenario::Echo).await;
    let request = echo_frame(ECHO_REQUEST_ID);
    let mut response = vec![0; request.len()];

    let mut samples = Vec::with_capacity(ROUND_TRIPS);

    for _ in 0..ROUND_TRIPS {
        let started = Instant::now();
        stream.write_all(&request).await.unwrap();
        stream.read_exact(&mut response).await.unwrap();
        samples.push(started.elapsed());
    }

    samples.sort();

    samples
}

/// Connects and logs in, leaving the client in Play.
async fn join(address: SocketAddr, scenario: Scenario) -> TcpStream {
    let mut stream = connect(address).await;
    stream.set_nodelay(true).unwrap();
    let handshake =
        ServerListPing { version: PROTOCOL_VERSION, addr: "localhost".into(), port: address.port(), next_state: HandshakeIntent::Login };
    send(&mut stream, &handshake).await;
    send(&mut stream, &LoginStart { name: scenario.name().into(), uuid: Uuid::nil() }).await;

    let body = read_body(&mut stream).await;
    assert!(matches!(Clientbound::decode(State::Login, &body).unwrap(), Clientbound::LoginSuccess(_)));
    send(&mut stream, &LoginAcknowledge).await;

    let body = read_body(&mut stream).await;
    assert!(matches!(Clientbound::decode(State::Config, &body).unwrap(), Clientbound::FinishConfiguration(_)));
    send(&mut stream, &AcknowledgeFinish).await;

    stream
}

/// The proxy starts on its own thread, so retry until it listens.
async fn connect(address: SocketAddr) -> TcpStream {
    for _ in 0..100 {
        if let Ok(stream) = TcpStream::connect(address).await {
            return stream;
        }

        glommio::timer::sleep(Duration::from_millis(10)).await;
    }

    panic!("nothing listens on {address}");
}

async fn serve_forever(listener: TcpListener) {
    loop {
        let stream = listener.accept().await.unwrap();
        glommio::spawn_local(serve(stream)).detach();
    }
}

/// A minimal offline server: login, an empty configuration, then the scenario.
async fn serve(mut stream: TcpStream) {
    stream.set_nodelay(true).unwrap();
    let body = read_body(&mut stream).await;
    let Serverbound::ServerListPing(_) = Serverbound::decode(State::Handshake, &body).unwrap() else {
        panic!("expected a handshake");
    };
    let body = read_body(&mut stream).await;
    let Serverbound::LoginStart(login) = Serverbound::decode(State::Login, &body).unwrap() else {
        panic!("expected login start");
    };
    let scenario = Scenario::parse(&login.name);
    let profile = GameProfile { uuid: Uuid::nil(), username: login.name.clone(), properties: vec![] };
    send(&mut stream, &LoginSuccess { profile, session_id: Uuid::nil() }).await;
    read_body(&mut stream).await;
    send(&mut stream, &FinishConfiguration).await;
    read_body(&mut stream).await;

    match scenario {
        Scenario::Stream { body_len } => {
            // Writes whole batches, so small frames do not cost a syscall each.
            let frame = bulk_frame(body_len);
            let frames_per_batch = (WRITE_BATCH / frame.len()).max(1);
            let batch = frame.repeat(frames_per_batch);
            let mut remaining = streamed_frame_count(body_len);

            while remaining > 0 {
                let frames = remaining.min(frames_per_batch);
                stream.write_all(&batch[..frames * frame.len()]).await.unwrap();
                remaining -= frames;
            }
        }
        Scenario::Echo => {
            let mut request = echo_frame(ECHO_REQUEST_ID);
            let response = echo_frame(ECHO_RESPONSE_ID);

            while stream.read_exact(&mut request).await.is_ok() {
                stream.write_all(&response).await.unwrap();
            }
        }
    }
}

fn streamed_frame_count(body_len: usize) -> usize {
    STREAMED_BYTES / bulk_frame(body_len).len()
}

fn bulk_frame(body_len: usize) -> Vec<u8> {
    raw_frame(BULK_ID, vec![0x55; body_len])
}

/// A keep-alive sized frame: an id and eight bytes.
fn echo_frame(id: i32) -> Vec<u8> {
    raw_frame(id, vec![0; 8])
}

fn raw_frame(id: i32, payload: Vec<u8>) -> Vec<u8> {
    let mut frame = Vec::new();
    RawPacket { id, payload: payload.into() }.encode(&mut frame).unwrap();

    frame
}

async fn send<'p>(stream: &mut TcpStream, packet: &impl Payload<'p>) {
    let mut frame = Vec::new();
    packet.encode(&mut frame).unwrap();
    stream.write_all(&frame).await.unwrap();
}

/// Reads one frame and returns its body. Only used while logging in.
async fn read_body(stream: &mut TcpStream) -> Vec<u8> {
    let mut prefix = Vec::new();

    loop {
        let mut byte = [0];
        stream.read_exact(&mut byte).await.unwrap();
        prefix.push(byte[0]);

        if byte[0] & 0x80 == 0 {
            break;
        }
    }

    let body_len = VarInt::read(&mut prefix.as_slice()).unwrap().0 as usize;
    let mut body = vec![0; body_len];
    stream.read_exact(&mut body).await.unwrap();

    body
}

/// Runs `serve` on a new thread with its own executor, listening on a free port.
fn start_on_thread<F: Future<Output = ()> + 'static>(name: &'static str, serve: fn(TcpListener) -> F) -> SocketAddr {
    let address = free_address();
    thread::spawn(move || {
        let executor = LocalExecutorBuilder::default().name(name).make().unwrap();
        executor.run(async move { serve(TcpListener::bind(address).unwrap()).await });
    });

    address
}

fn free_address() -> SocketAddr {
    let listener = StdTcpListener::bind("127.0.0.1:0").unwrap();
    listener.local_addr().unwrap()
}

fn percentile(sorted: &[Duration], quantile: f64) -> Duration {
    let index = ((sorted.len() - 1) as f64 * quantile).round() as usize;
    sorted[index]
}

fn micros(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1e6
}
