/// Which way a packet travels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    /// Sent by the client to the server.
    Serverbound,
    /// Sent by the server to the client.
    Clientbound,
}
