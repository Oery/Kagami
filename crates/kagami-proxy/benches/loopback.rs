//! End-to-end loopback benchmark: a client talks to a minimal server, once
//! directly and once through the proxy, and compares throughput and round
//! trip latency. The server runs plain, compressed, and online: encrypted
//! and compressed, without the Mojang session check. The micro benchmark
//! measures the cipher and inflating on their own.
//!
//! cargo bench -p kagami-proxy --bench loopback

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener as StdTcpListener};
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};

use aes::Aes128;
use cfb8::cipher::KeyIvInit;
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use futures_lite::{AsyncReadExt, AsyncWriteExt};
use glommio::LocalExecutorBuilder;
use glommio::net::{TcpListener, TcpStream};
use kagami_proxy::{Account, Action, Config, Context, Handlers, Interest, Upstream};
use mc_protocol::packets::PacketKind;
use mc_protocol::packets::client::{EncryptionRequest, FinishConfiguration, LoginSuccess, PlayKeepAlive, SetCompression};
use mc_protocol::packets::server::{
    AcknowledgeFinish, EncryptionResponse, LoginAcknowledge, LoginStart, PlayKeepAliveResponse, ServerListPing,
};
use mc_protocol::packets::{Clientbound, Serverbound};
use mc_protocol::state::HandshakeIntent;
use mc_protocol::types::GameProfile;
use mc_protocol::{Direction, PROTOCOL_VERSION, PacketWriter, Payload, State, VarInt};
use rsa::pkcs8::{DecodePublicKey, EncodePublicKey};
use rsa::{Pkcs1v15Encrypt, PrivateKey, PublicKey};
use uuid::Uuid;

type Encryptor = cfb8::Encryptor<Aes128>;
type Decryptor = cfb8::Decryptor<Aes128>;

/// Play ids that change no state; the proxy only reads ids.
const BULK_ID: i32 = 0x2e;
const ECHO_REQUEST_ID: i32 = 0x1c;
const ECHO_RESPONSE_ID: i32 = 0x2d;
/// The client's first Play packet tells the server what to do.
const SCENARIO_ID: i32 = 0x7a;

const STREAMED_BYTES: usize = 256 * 1024 * 1024;
const ROUND_TRIPS: usize = 20_000;
const WRITE_BATCH: usize = 256 * 1024;
const LARGE_BODY: usize = 16 * 1024;
const SMALL_BODY: usize = 64;
const COMPRESSION_THRESHOLD: i32 = 256;

/// How the server sets up each connection.
#[derive(Clone, Copy)]
struct Wire {
    name: &'static str,
    compression: Option<i32>,
    encryption: bool,
    /// The proxy runs handlers on the keep-alives the echo uses.
    handled: bool,
}

const WIRES: [Wire; 4] = [
    Wire { name: "plain", compression: None, encryption: false, handled: false },
    Wire { name: "handled", compression: None, encryption: false, handled: true },
    Wire { name: "compressed", compression: Some(COMPRESSION_THRESHOLD), encryption: false, handled: false },
    Wire { name: "online", compression: Some(COMPRESSION_THRESHOLD), encryption: true, handled: false },
];

/// Handlers on both keep-alives, so each echo round trip is decoded and
/// handled once per direction. Bulk packets have no handler.
struct KeepAlives;

impl Handlers<i64> for KeepAlives {
    const INTEREST: Interest = Interest::NONE.with(Direction::Serverbound, State::Play, <PlayKeepAliveResponse as PacketKind>::ID).with(
        Direction::Clientbound,
        State::Play,
        <PlayKeepAlive as PacketKind>::ID,
    );

    fn serverbound(&mut self, packet: &mut Serverbound<'_>, cx: &mut Context<'_, i64>) -> Action {
        if let Some(response) = PlayKeepAliveResponse::from_serverbound(packet) {
            *cx.state += response.id;
        }

        Action::Forward
    }

    fn clientbound(&mut self, packet: &mut Clientbound<'_>, cx: &mut Context<'_, i64>) -> Action {
        if let Some(keep_alive) = PlayKeepAlive::from_clientbound(packet) {
            *cx.state += keep_alive.id;
        }

        Action::Forward
    }
}

#[derive(Clone, Copy)]
enum Scenario {
    /// The server streams frames with this body size, then closes.
    Stream { body_len: usize },
    /// The server echoes each request.
    Echo,
}

fn main() {
    let mut routes = Vec::new();

    for wire in WIRES {
        let server = start_server(wire);
        let proxy = start_proxy(server, wire);
        routes.push((wire, server, proxy));
    }

    let executor = LocalExecutorBuilder::default().name("bench-client").make().unwrap();
    executor.run(async move {
        println!("{:<30}{:>14}{:>14}", "", "direct", "proxied");

        for (wire, server, proxy) in &routes {
            for body_len in [LARGE_BODY, SMALL_BODY] {
                let scenario = Scenario::Stream { body_len };
                let direct = throughput(*server, *wire, scenario).await;
                let proxied = throughput(*proxy, *wire, scenario).await;
                let label = format!("{} {body_len} B packets", wire.name);
                println!("{label:<30}{direct:>9.2} GiB/s{proxied:>9.2} GiB/s");
            }

            let direct = latencies(*server, *wire).await;
            let proxied = latencies(*proxy, *wire).await;

            for (quantile, label) in [(0.50, "p50"), (0.99, "p99")] {
                let direct = micros(percentile(&direct, quantile));
                let proxied = micros(percentile(&proxied, quantile));
                let label = format!("{} round trip {label}", wire.name);
                println!("{label:<30}{direct:>11.1} µs{proxied:>11.1} µs");
            }
        }
    });
}

/// Streams `STREAMED_BYTES` of packets and returns the rate the client
/// receives them at, in GiB/s of uncompressed packet bodies.
async fn throughput(address: SocketAddr, wire: Wire, scenario: Scenario) -> f64 {
    let Scenario::Stream { body_len } = scenario else {
        unreachable!("only streams measure throughput");
    };
    let mut peer = join(address, wire, scenario).await;
    let mut buffer = vec![0; 256 * 1024];
    let started = Instant::now();

    while peer.read(&mut buffer).await > 0 {}

    let streamed = frame_count(body_len) * body_len;

    streamed as f64 / started.elapsed().as_secs_f64() / (1u64 << 30) as f64
}

/// Measures every round trip of a keep-alive sized request echoed back.
async fn latencies(address: SocketAddr, wire: Wire) -> Vec<Duration> {
    let mut peer = join(address, wire, Scenario::Echo).await;
    let request = wire_frame(ECHO_REQUEST_ID, &[0; 8], wire.compression);
    let mut response = vec![0; wire_frame(ECHO_RESPONSE_ID, &[0; 8], wire.compression).len()];
    let mut samples = Vec::with_capacity(ROUND_TRIPS);

    for _ in 0..ROUND_TRIPS {
        let started = Instant::now();
        peer.write(&mut request.clone()).await;
        peer.read_exact(&mut response).await;
        samples.push(started.elapsed());
    }

    samples.sort();

    samples
}

/// Logs in like a client, then asks for `scenario`.
async fn join(address: SocketAddr, wire: Wire, scenario: Scenario) -> Peer {
    let mut peer = Peer::new(connect(address).await);

    let handshake =
        ServerListPing { version: PROTOCOL_VERSION, addr: "localhost".into(), port: address.port(), next_state: HandshakeIntent::Login };
    peer.send(&handshake).await;
    peer.send(&LoginStart { name: "bench".into(), uuid: Uuid::nil() }).await;

    loop {
        let packet = peer.read_packet().await;

        match Clientbound::decode(State::Login, &packet).unwrap() {
            // Only a direct client sees this: the proxy encrypts the server leg alone.
            Clientbound::EncryptionRequest(request) => {
                let secret: [u8; 16] = rand::random();
                let public_key = PublicKey::from_public_key_der(&request.public_key).unwrap();
                let mut rng = rand::rng();
                let shared_secret = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &secret).unwrap();
                let verify_token = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &request.verify_token).unwrap();

                peer.send(&EncryptionResponse { shared_secret: shared_secret.into(), verify_token: verify_token.into() }).await;
                peer.enable_encryption(&secret);
            }
            Clientbound::SetCompression(compression) => peer.compression = Some(compression.threshold),
            Clientbound::LoginSuccess(_) => break,
            _ => panic!("unexpected login packet"),
        }
    }

    assert_eq!(peer.compression, wire.compression);
    peer.send(&LoginAcknowledge).await;

    let packet = peer.read_packet().await;
    assert!(matches!(Clientbound::decode(State::Config, &packet).unwrap(), Clientbound::FinishConfiguration(_)));
    peer.send(&AcknowledgeFinish).await;

    let mut request = wire_frame(SCENARIO_ID, &scenario_payload(scenario), peer.compression);
    peer.write(&mut request).await;

    peer
}

/// The proxy starts on its own thread, so retry until it listens.
async fn connect(address: SocketAddr) -> TcpStream {
    for _ in 0..200 {
        if let Ok(stream) = TcpStream::connect(address).await {
            stream.set_nodelay(true).unwrap();
            return stream;
        }

        glommio::timer::sleep(Duration::from_millis(10)).await;
    }

    panic!("nothing listens on {address}");
}

/// A minimal server: login, an empty configuration, then the scenario.
async fn serve(stream: TcpStream, wire: Wire, key: Rc<PrivateKey>) {
    stream.set_nodelay(true).unwrap();
    let mut peer = Peer::new(stream);

    let packet = peer.read_packet().await;
    assert!(matches!(Serverbound::decode(State::Handshake, &packet).unwrap(), Serverbound::ServerListPing(_)));
    let packet = peer.read_packet().await;
    assert!(matches!(Serverbound::decode(State::Login, &packet).unwrap(), Serverbound::LoginStart(_)));

    if wire.encryption {
        encrypt(&mut peer, &key).await;
    }

    if let Some(threshold) = wire.compression {
        peer.send(&SetCompression { threshold }).await;
        peer.compression = Some(threshold);
    }

    let profile = GameProfile { uuid: Uuid::nil(), username: "bench".into(), properties: vec![] };
    peer.send(&LoginSuccess { profile, session_id: Uuid::nil() }).await;
    peer.read_packet().await;
    peer.send(&FinishConfiguration).await;
    peer.read_packet().await;

    let packet = peer.read_packet().await;

    match parse_scenario(&packet) {
        Scenario::Stream { body_len } => stream_frames(&mut peer, body_len).await,
        Scenario::Echo => echo(&mut peer).await,
    }
}

/// The server side of the encryption request, without the session check.
async fn encrypt(peer: &mut Peer, key: &PrivateKey) {
    let public_key = key.to_public_key().to_public_key_der().unwrap();
    let verify_token = [1, 2, 3, 4];
    let request = EncryptionRequest {
        server_id: "".into(),
        public_key: public_key.as_bytes().to_vec().into(),
        verify_token: verify_token.as_slice().into(),
        should_authenticate: false,
    };
    peer.send(&request).await;

    let packet = peer.read_packet().await;
    let Serverbound::EncryptionResponse(response) = Serverbound::decode(State::Login, &packet).unwrap() else {
        panic!("expected an encryption response");
    };
    assert_eq!(key.decrypt(Pkcs1v15Encrypt, &response.verify_token).unwrap(), verify_token);

    let secret = key.decrypt(Pkcs1v15Encrypt, &response.shared_secret).unwrap();
    peer.enable_encryption(&secret.try_into().unwrap());
}

/// Writes whole batches, so small frames do not cost a syscall each.
async fn stream_frames(peer: &mut Peer, body_len: usize) {
    let frame = wire_frame(BULK_ID, &payload(body_len), peer.compression);
    let frames_per_batch = (WRITE_BATCH / frame.len()).max(1);
    let batch = frame.repeat(frames_per_batch);
    let mut remaining = frame_count(body_len);

    while remaining > 0 {
        let frames = remaining.min(frames_per_batch);
        let mut bytes = batch[..frames * frame.len()].to_vec();
        peer.write(&mut bytes).await;
        remaining -= frames;
    }
}

async fn echo(peer: &mut Peer) {
    let mut request = vec![0; wire_frame(ECHO_REQUEST_ID, &[0; 8], peer.compression).len()];
    let response = wire_frame(ECHO_RESPONSE_ID, &[0; 8], peer.compression);

    while peer.try_read_exact(&mut request).await {
        peer.write(&mut response.clone()).await;
    }
}

/// One end of a connection, with the framing and cipher the login set up.
struct Peer {
    stream: TcpStream,
    compression: Option<i32>,
    encryptor: Option<Encryptor>,
    decryptor: Option<Decryptor>,
}

impl Peer {
    fn new(stream: TcpStream) -> Self {
        Self { stream, compression: None, encryptor: None, decryptor: None }
    }

    fn enable_encryption(&mut self, secret: &[u8; 16]) {
        self.encryptor = Some(Encryptor::new(secret.into(), secret.into()));
        self.decryptor = Some(Decryptor::new(secret.into(), secret.into()));
    }

    async fn send<'p, P: Payload<'p>>(&mut self, packet: &P) {
        let mut payload = Vec::new();
        packet.encode_payload(&mut payload).unwrap();

        let mut frame = wire_frame(P::ID.0, &payload, self.compression);
        self.write(&mut frame).await;
    }

    async fn write(&mut self, bytes: &mut [u8]) {
        if let Some(encryptor) = &mut self.encryptor {
            encryptor.encrypt(bytes);
        }

        self.stream.write_all(bytes).await.unwrap();
    }

    /// Reads and decrypts some bytes; zero once the peer closed.
    async fn read(&mut self, buffer: &mut [u8]) -> usize {
        let read = self.stream.read(buffer).await.unwrap_or(0);

        if let Some(decryptor) = &mut self.decryptor {
            decryptor.decrypt(&mut buffer[..read]);
        }

        read
    }

    async fn read_exact(&mut self, buffer: &mut [u8]) {
        assert!(self.try_read_exact(buffer).await, "peer closed the stream early");
    }

    /// Fills `buffer`, or returns `false` once the peer closed.
    async fn try_read_exact(&mut self, buffer: &mut [u8]) -> bool {
        if self.stream.read_exact(buffer).await.is_err() {
            return false;
        }

        if let Some(decryptor) = &mut self.decryptor {
            decryptor.decrypt(buffer);
        }

        true
    }

    /// Reads one packet, id then payload. Only used while logging in.
    async fn read_packet(&mut self) -> Vec<u8> {
        let mut prefix = Vec::new();

        loop {
            let mut byte = [0];
            self.read_exact(&mut byte).await;
            prefix.push(byte[0]);

            if byte[0] & 0x80 == 0 {
                break;
            }
        }

        let body_len = VarInt::read(&mut prefix.as_slice()).unwrap().0 as usize;
        let mut body = vec![0; body_len];
        self.read_exact(&mut body).await;

        if self.compression.is_none() {
            return body;
        }

        let mut data = body.as_slice();
        let data_len = VarInt::read(&mut data).unwrap().0 as usize;
        if data_len == 0 {
            return data.to_vec();
        }

        let mut packet = Vec::with_capacity(data_len);
        ZlibDecoder::new(data).read_to_end(&mut packet).unwrap();

        packet
    }
}

/// A frame in a connection's framing: compressed from the threshold up.
fn wire_frame(id: i32, payload: &[u8], compression: Option<i32>) -> Vec<u8> {
    let mut packet = Vec::new();
    VarInt(id).encode(&mut packet).unwrap();
    packet.extend_from_slice(payload);

    let body = match compression {
        None => packet,
        Some(threshold) if packet.len() < threshold as usize => [&[0], packet.as_slice()].concat(),
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

/// Packet bodies that compress to about half, like chunk data.
fn payload(len: usize) -> Vec<u8> {
    let mut state = 0x2545_f491_u32;

    (0..len)
        .map(|_| {
            state = state.wrapping_mul(1_103_515_245).wrapping_add(12_345);
            (state >> 24) as u8 & 0x0f
        })
        .collect()
}

fn frame_count(body_len: usize) -> usize {
    STREAMED_BYTES / body_len
}

fn scenario_payload(scenario: Scenario) -> Vec<u8> {
    match scenario {
        Scenario::Stream { body_len } => [&[0], (body_len as u32).to_be_bytes().as_slice()].concat(),
        Scenario::Echo => vec![1],
    }
}

fn parse_scenario(packet: &[u8]) -> Scenario {
    let mut payload = packet;
    assert_eq!(VarInt::read(&mut payload).unwrap().0, SCENARIO_ID);

    match payload[0] {
        0 => Scenario::Stream { body_len: u32::from_be_bytes(payload[1..5].try_into().unwrap()) as usize },
        _ => Scenario::Echo,
    }
}

fn start_server(wire: Wire) -> SocketAddr {
    let address = free_address();

    thread::spawn(move || {
        let key = Rc::new(PrivateKey::new(&mut rand::rng(), 1024).unwrap());
        let executor = LocalExecutorBuilder::default().name("bench-server").make().unwrap();

        executor.run(async move {
            let listener = TcpListener::bind(address).unwrap();

            loop {
                let stream = listener.accept().await.unwrap();
                glommio::spawn_local(serve(stream, wire, key.clone())).detach();
            }
        });
    });

    address
}

/// An online proxy logs in with a dummy account: the bench server does not
/// ask for the session check.
fn start_proxy(server: SocketAddr, wire: Wire) -> SocketAddr {
    let listen = free_address();
    let account = wire.encryption.then(|| Account { access_token: String::new(), uuid: Uuid::nil(), name: "bench".to_owned() });
    let upstream = Upstream { host: server.ip().to_string(), port: server.port() };

    let config = Config { listen, upstream, account };

    thread::spawn(move || match wire.handled {
        true => kagami_proxy::run(config, 0, KeepAlives).unwrap(),
        false => kagami_proxy::run(config, (), ()).unwrap(),
    });

    listen
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
