use glommio::LocalExecutorBuilder;
use mc_protocol::packets::client::SetHealth;
use mc_protocol::packets::server::ChatCommand;

use super::*;

/// Runs `handlers` on one decoded Play packet and returns their action and
/// the packet encoded again.
fn handle_serverbound<S, H: Handlers<S>>(proxy: &mut Kagami<S, H>, body: &[u8]) -> (Action, Vec<u8>) {
    LocalExecutorBuilder::default().make().unwrap().run(async {
        let (client, server) = kagami_proxy::testing::handles().await;
        let mut cx = Context { state: &mut proxy.state, client: &client, server: &server };

        let mut packet = Serverbound::decode(State::Play, body).unwrap();
        let action = proxy.handlers.serverbound(&mut packet, &mut cx);

        let mut encoded = Vec::new();
        packet.encode(&mut encoded).unwrap();

        (action, encoded)
    })
}

fn interest_of<S, H: Handlers<S>>(_: &Kagami<S, H>) -> Interest {
    H::INTEREST
}

#[test]
fn handlers_run_in_order_on_one_decode_and_record_their_interest() {
    let mut proxy = Kagami::new(Vec::new())
        .on_packet::<ChatCommand>(|command, cx| {
            cx.state.push(format!("first saw {}", command.command));
            command.command = "home".into();
            Action::Modified
        })
        .on_packet::<SetHealth>(|_, _| Action::Drop)
        .on_packet::<ChatCommand>(|command, cx| {
            cx.state.push(format!("second saw {}", command.command));
            Action::Forward
        });

    let interest = interest_of(&proxy);
    assert!(interest.contains(Direction::Serverbound, State::Play, <ChatCommand as PacketKind>::ID));
    assert!(interest.contains(Direction::Clientbound, State::Play, <SetHealth as PacketKind>::ID));

    let (action, encoded) = handle_serverbound(&mut proxy, &[0x07, 4, b's', b'p', b'a', b'm']);
    assert_eq!(action, Action::Modified);
    assert_eq!(&encoded[1..], [0x07, 4, b'h', b'o', b'm', b'e']);
    assert_eq!(proxy.state, ["first saw spam", "second saw home"]);
}

#[test]
fn a_drop_skips_the_handlers_after_it() {
    let mut proxy = Kagami::new(0).on_packet::<ChatCommand>(|_, _| Action::Drop).on_packet::<ChatCommand>(|_, cx| {
        *cx.state += 1;
        Action::Forward
    });

    let (action, _) = handle_serverbound(&mut proxy, &[0x07, 0]);
    assert_eq!(action, Action::Drop);
    assert_eq!(proxy.state, 0);
}
