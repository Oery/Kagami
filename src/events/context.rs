use crate::{
    context::{BufferedStream, State},
    events::{ClientPacket, ServerPacket},
    packets::{client, data::ChatPosition, server},
    varint::temp_convert,
};

use async_std::io::WriteExt;

pub struct Context<'a, 'b, T> {
    pub payload: T,
    pub client: Client<'a, 'b>,
    pub server: Server<'a, 'b>,
    pub cancel: bool,
    pub state: State,
}

impl<'a, 'b, T> Context<'a, 'b, T> {
    pub fn new(
        payload: T,
        client: &'b mut BufferedStream<'a>,
        server: &'b mut BufferedStream<'a>,
        state: &State,
    ) -> Context<'a, 'b, T> {
        Context {
            payload,
            client: Client(client),
            server: Server(server),
            cancel: false,
            state: State {
                mc_state: state.mc_state.clone(),
                compress_threshold: state.compress_threshold.clone(),
            },
        }
    }

    pub fn cancel_packet(&mut self) {
        self.cancel = true;
    }
}

pub struct Client<'a, 'b>(&'b mut BufferedStream<'a>);

impl<'a, 'b> Client<'a, 'b> {
    pub fn send<P: ServerPacket<'a>>(&mut self, packet: &P) {
        let packet = packet.serialize_packet().unwrap();
        let packet_id = temp_convert(packet.id).unwrap();
        let packet_len = packet.raw_payload.len() + packet_id.len();

        async_std::task::block_on(async {
            self.0
                .writer
                .write(&temp_convert((packet_len + 1) as i32).unwrap())
                .await
                .unwrap();
            self.0.writer.write(&temp_convert(0).unwrap()).await.unwrap();
            self.0.writer.write(&packet_id).await.unwrap();
            self.0.writer.write(&packet.raw_payload).await.unwrap();
        })
    }

    pub fn chat(&mut self, message: &str) {
        self.send(&server::Chat {
            json: format!("'{message}'").into(),
            position: ChatPosition::Chat,
        });
    }
}

pub struct Server<'a, 'b>(&'b mut BufferedStream<'a>);

impl<'a, 'b> Server<'a, 'b> {
    pub fn send<P: ClientPacket<'a>>(&mut self, packet: &P) {
        let packet = packet.serialize_packet().unwrap();
        let packet_id = temp_convert(packet.id).unwrap();
        let packet_len = packet.raw_payload.len() + packet_id.len();

        async_std::task::block_on(async {
            self.0
                .writer
                .write(&temp_convert((packet_len + 1) as i32).unwrap())
                .await
                .unwrap();
            self.0.writer.write(&temp_convert(0).unwrap()).await.unwrap();
            self.0.writer.write(&packet_id).await.unwrap();
            self.0.writer.write(&packet.raw_payload).await.unwrap();
        });
    }
}
