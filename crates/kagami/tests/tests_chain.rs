use glommio::LocalExecutorBuilder;
use mc_protocol::packets::client::SetHealth;
use mc_protocol::packets::server::ChatCommand;

use super::*;

/// Runs the proxy's handlers on one decoded Play packet and returns their
/// action, the packet encoded again, and the state they left.
fn handle_serverbound<S: Clone, H: Handlers<S>>(proxy: Kagami<S, H>, body: &[u8]) -> (Action, Vec<u8>, S) {
    let Kagami { state, mut handlers, .. } = proxy;

    LocalExecutorBuilder::default().make().unwrap().run(async {
        let session = kagami_proxy::testing::session(state).await;

        let mut packet = Serverbound::decode(State::Play, body).unwrap();
        let action = kagami_proxy::testing::with_context(&session, |cx| handlers.serverbound(&mut packet, cx));

        let mut encoded = Vec::new();
        packet.encode(&mut encoded).unwrap();

        (action, encoded, session.with(|state| state.clone()))
    })
}

fn interest_of<S, H: Handlers<S>>(_: &Kagami<S, H>) -> Interest {
    H::INTEREST
}

#[test]
fn handlers_run_in_order_on_one_decode_and_record_their_interest() {
    let proxy = Kagami::new(Vec::new())
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

    let (action, encoded, state) = handle_serverbound(proxy, &[0x07, 4, b's', b'p', b'a', b'm']);
    assert_eq!(action, Action::Modified);
    assert_eq!(&encoded[1..], [0x07, 4, b'h', b'o', b'm', b'e']);
    assert_eq!(state, ["first saw spam", "second saw home"]);
}

#[test]
fn a_drop_skips_the_handlers_after_it() {
    let proxy = Kagami::new(0).on_packet::<ChatCommand>(|_, _| Action::Drop).on_packet::<ChatCommand>(|_, cx| {
        *cx.state += 1;
        Action::Forward
    });

    let (action, _, state) = handle_serverbound(proxy, &[0x07, 0]);
    assert_eq!(action, Action::Drop);
    assert_eq!(state, 0);
}
