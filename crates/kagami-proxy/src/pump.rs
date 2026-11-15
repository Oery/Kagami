//! Forwards one direction of a session after login, frame by frame.

use std::io;

use futures_lite::{AsyncRead, AsyncWrite, AsyncWriteExt};
use mc_protocol::{Direction, PacketWriter, State, VarInt};

use crate::compression::Inflater;
use crate::frame::{FrameBuffer, FrameHeader};
use crate::invalid_data;
use crate::transition::next_state;

/// What a pump needs to know about the frames it forwards.
pub struct Wire {
    pub direction: Direction,
    /// Both legs use the compressed framing once the server enabled it.
    pub compressed: bool,
}

/// Copies frames from `reader` to `writer` until `reader` ends. Complete
/// frames are written straight from the read buffer, in runs, and are
/// never inflated: compression is mirrored on both legs.
pub async fn pump<R, W>(wire: Wire, mut state: State, buffer: &mut FrameBuffer, reader: &mut R, writer: &mut W) -> io::Result<()>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let mut inflater = wire.compressed.then(Inflater::new);

    loop {
        let complete = scan(wire.direction, &mut state, inflater.as_mut(), buffer.pending())?;

        if complete > 0 {
            writer.write_all(&buffer.pending()[..complete]).await?;
            buffer.consume(complete);
        }

        if !buffer.fill(reader).await? {
            return Ok(());
        }
    }
}

/// Returns the length of the complete frames at the start of `bytes`, and
/// follows the state changes they cause.
fn scan(direction: Direction, state: &mut State, mut inflater: Option<&mut Inflater>, bytes: &[u8]) -> io::Result<usize> {
    let mut offset = 0;

    while let Some(header) = FrameHeader::parse(&bytes[offset..])? {
        let frame_end = offset + header.frame_len();
        if frame_end > bytes.len() {
            break;
        }

        let body = &bytes[offset + header.header_len..frame_end];
        let id = match inflater.as_deref_mut() {
            Some(inflater) => inflater.packet_id(body)?,
            None => VarInt::read(&mut &body[..]).map_err(invalid_data)?.0,
        };

        // Ids past a byte belong to no known packet, so they never change state.
        if let Ok(id) = u8::try_from(id) {
            *state = next_state(direction, *state, id);
        }

        offset = frame_end;
    }

    Ok(offset)
}

#[cfg(test)]
#[path = "../tests/tests_pump.rs"]
mod tests;
