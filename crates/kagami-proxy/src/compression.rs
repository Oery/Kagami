//! The compressed framing: each frame body starts with the packet's
//! uncompressed length, or zero when the packet is sent uncompressed.

use std::io;

use flate2::{Decompress, FlushDecompress};
use mc_protocol::{PacketWriter, VarInt};

use crate::invalid_data;

/// The largest packet vanilla accepts once inflated.
const MAX_INFLATED_LEN: usize = 8 * 1024 * 1024;

/// A VarInt packet id never takes more than five bytes.
const MAX_ID_LEN: usize = 5;

/// Splits a frame body into the packet's uncompressed length and its data.
/// A length of zero means the data is the packet itself.
pub fn split_data_length(body: &[u8]) -> io::Result<(usize, &[u8])> {
    let mut data = body;
    let data_len = VarInt::read(&mut data).map_err(invalid_data)?.0;
    let data_len = usize::try_from(data_len).map_err(|_| invalid_data("negative uncompressed length"))?;

    if data_len > MAX_INFLATED_LEN {
        return Err(invalid_data("packet exceeds the maximum uncompressed length"));
    }

    Ok((data_len, data))
}

/// Prefixes the body of an uncompressed frame with a zero data length, which
/// sends the packet uncompressed in the compressed framing.
pub fn add_zero_data_length(frame: &mut Vec<u8>) -> io::Result<()> {
    let mut body = frame.as_slice();
    let body_len = VarInt::read(&mut body).map_err(invalid_data)?.0;
    let header_len = frame.len() - body.len();

    let mut header = Vec::with_capacity(4);
    VarInt(body_len + 1).encode(&mut header).map_err(invalid_data)?;
    header.push(0);
    frame.splice(..header_len, header);

    Ok(())
}

/// Inflates compressed packets, reusing one zlib stream across frames.
pub struct Inflater {
    stream: Decompress,
}

impl Inflater {
    pub fn new() -> Self {
        Self { stream: Decompress::new(true) }
    }

    /// Reads the packet id of a frame body, inflating only its first bytes.
    pub fn packet_id(&mut self, body: &[u8]) -> io::Result<i32> {
        let (data_len, mut data) = split_data_length(body)?;
        if data_len == 0 {
            return Ok(VarInt::read(&mut data).map_err(invalid_data)?.0);
        }

        let mut head = [0; MAX_ID_LEN];
        let produced = self.inflate_into(data, &mut head)?;
        let mut head = &head[..produced];

        Ok(VarInt::read(&mut head).map_err(invalid_data)?.0)
    }

    /// Returns the packet of a frame body, id then payload. A compressed
    /// packet is inflated into `inflated`; an uncompressed one is borrowed.
    pub fn packet<'a>(&mut self, body: &'a [u8], inflated: &'a mut Vec<u8>) -> io::Result<&'a [u8]> {
        let (data_len, data) = split_data_length(body)?;
        if data_len == 0 {
            return Ok(data);
        }

        inflated.clear();
        inflated.resize(data_len, 0);
        let produced = self.inflate_into(data, inflated)?;

        if produced != data_len {
            return Err(invalid_data("packet inflated to a different length than announced"));
        }

        Ok(inflated)
    }

    /// Inflates the start of `data` into `out`, stopping once `out` is full.
    fn inflate_into(&mut self, data: &[u8], out: &mut [u8]) -> io::Result<usize> {
        self.stream.reset(true);
        self.stream.decompress(data, out, FlushDecompress::None).map_err(invalid_data)?;

        Ok(self.stream.total_out() as usize)
    }
}

#[cfg(test)]
#[path = "../tests/tests_compression.rs"]
mod tests;
