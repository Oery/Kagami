use std::io::Write;

use flate2::Compression;
use flate2::write::ZlibEncoder;
use futures_lite::AsyncReadExt;
use futures_lite::future::zip;
use glommio::LocalExecutorBuilder;
use glommio::net::{TcpListener, TcpStream};
use mc_protocol::Payload;
use mc_protocol::packets::PacketKind;
use mc_protocol::packets::client::SystemChatMessage;
use mc_protocol::packets::server::{AcknowledgeFinish, ChatCommand};
use mc_protocol::types::Nbt;

use super::*;
use crate::frame::tests::Chunks;
use crate::split::split;
use crate::stream::Writer;

/// An outlet writing into one end of a loopback socket pair, and the other
/// end, which reads what it wrote.
async fn outlet() -> (Rc<Outlet>, TcpStream) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let (ours, theirs) = zip(TcpStream::connect(address), listener.accept()).await;

    let (_, writer) = split(ours.unwrap());
    let outlet = Rc::new(Outlet::new(Writer::new(writer), false));

    (outlet, theirs.unwrap())
}

fn frame<'p>(packet: &impl Payload<'p>) -> Vec<u8> {
    let mut frame = Vec::new();
    packet.encode(&mut frame).unwrap();

    frame
}

fn command(text: &str) -> Vec<u8> {
    frame(&ChatCommand { command: text.into() })
}

/// Counts chat commands in the state and acts on their text.
struct Commands;

impl Handlers<u32> for Commands {
    const INTEREST: Interest = Interest::NONE.with(Direction::Serverbound, State::Play, <ChatCommand as PacketKind>::ID);

    fn serverbound(&mut self, packet: &mut Serverbound<'_>, cx: &mut Context<'_, u32>) -> Action {
        let Some(command) = <ChatCommand as PacketKind>::from_serverbound(packet) else {
            return Action::Forward;
        };
        *cx.state += 1;

        match command.command.as_ref() {
            "drop" => Action::Drop,
            "edit" => {
                command.command = "edited".into();
                Action::Modified
            }
            "reply" => {
                cx.client.send(&SystemChatMessage { content: Nbt::Str("hi".into()), overlay: false });
                cx.server.send(&ChatCommand { command: "after".into() });
                Action::Forward
            }
            _ => Action::Forward,
        }
    }

    fn clientbound(&mut self, _: &mut Clientbound<'_>, _: &mut Context<'_, u32>) -> Action {
        Action::Forward
    }
}

#[test]
fn handlers_drop_edit_and_send_after_their_packet() {
    LocalExecutorBuilder::default().make().unwrap().run(async {
        let (client, _client_peer) = outlet().await;
        let (server, mut server_peer) = outlet().await;
        let shared = Rc::new(Shared { state: RefCell::new(0), handlers: RefCell::new(Commands) });
        let mut pump = Pump::new(Direction::Serverbound, false, shared.clone(), Client(client.clone()), Server(server));

        // Chat commands only have handlers in Play, which the acknowledgement starts.
        let acknowledge = frame(&AcknowledgeFinish);
        let input = [acknowledge.clone(), command("drop"), command("edit"), command("keep"), command("reply")].concat();
        let expected = [acknowledge, command("edited"), command("keep"), command("reply"), command("after")].concat();

        // Three bytes per read, so frames and headers are cut everywhere.
        let mut reader = Chunks(input.chunks(3).map(<[u8]>::to_vec).collect());
        pump.run(&mut FrameBuffer::new(), &mut reader).await.unwrap();

        let mut written = vec![0; expected.len()];
        server_peer.read_exact(&mut written).await.unwrap();
        assert_eq!(written, expected);
        assert_eq!(*shared.state.borrow(), 4);
        // The reply to the client waits for the client's flusher.
        assert!(client.has_outbox());
    });
}

#[test]
fn state_changes_are_found_inside_compressed_frames() {
    LocalExecutorBuilder::default().make().unwrap().run(async {
        let (client, _client_peer) = outlet().await;
        let (server, _server_peer) = outlet().await;
        let shared = Rc::new(Shared { state: RefCell::new(()), handlers: RefCell::new(()) });
        let mut pump = Pump::new(Direction::Clientbound, true, shared, Client(client), Server(server));

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&[0x03]).unwrap();
        let data = encoder.finish().unwrap();

        // Frame length, uncompressed length 1, then Finish Configuration deflated.
        let mut frame = vec![(data.len() + 1) as u8, 1];
        frame.extend_from_slice(&data);

        assert_eq!(pump.scan(&frame).unwrap(), Step { run: frame.len(), skipped: 0 });
        assert_eq!(pump.state, State::Play);
    });
}
