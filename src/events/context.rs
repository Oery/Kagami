use crate::{
    context::BufferedStream,
    events::{ClientPacket, ServerPacket},
    varint::temp_convert,
};

use async_std::io::WriteExt;

pub struct Context<'a, 'b, T> {
    pub payload: T,
    pub client: Client<'a, 'b>,
    pub server: Server<'a, 'b>,
    pub should_filter: bool,
}

impl<'a, 'b, T> Context<'a, 'b, T> {
    pub fn new(
        payload: T,
        client: &'b mut BufferedStream<'a>,
        server: &'b mut BufferedStream<'a>,
    ) -> Context<'a, 'b, T> {
        Context {
            payload,
            client: Client(client),
            server: Server(server),
            should_filter: false,
        }
    }
}

pub struct Client<'a, 'b>(&'b mut BufferedStream<'a>);

impl<'a, 'b> Client<'a, 'b> {
    pub fn send<P: ServerPacket<'a>>(&mut self, packet: &P) {
        let packet = packet.serialize().unwrap();
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
}

pub struct Server<'a, 'b>(&'b mut BufferedStream<'a>);

impl<'a, 'b> Server<'a, 'b> {
    pub fn send<P: ClientPacket<'a>>(&mut self, packet: &P) {
        let packet = packet.serialize().unwrap();
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
}
