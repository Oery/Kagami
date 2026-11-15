//! Minecraft frames: a VarInt body length, then the body.

use std::io;
use std::ops::Range;

use futures_lite::{AsyncRead, AsyncReadExt};
use mc_protocol::decode::MAX_FRAME_LEN;

use crate::invalid_data;

/// `MAX_FRAME_LEN` is below 2^21, so its VarInt never takes more than three bytes.
const MAX_HEADER_LEN: usize = 3;

const INITIAL_CAPACITY: usize = 256 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameHeader {
    /// Bytes taken by the length prefix.
    pub header_len: usize,
    pub body_len: usize,
}

impl FrameHeader {
    /// Parses the length prefix at the start of `bytes`; `None` until it is complete.
    pub fn parse(bytes: &[u8]) -> io::Result<Option<Self>> {
        let mut body_len = 0;
        for (index, &byte) in bytes.iter().take(MAX_HEADER_LEN).enumerate() {
            body_len |= usize::from(byte & 0x7f) << (7 * index);
            if byte & 0x80 != 0 {
                continue;
            }
            if body_len > MAX_FRAME_LEN {
                return Err(invalid_data("frame exceeds the maximum length"));
            }
            return Ok(Some(Self { header_len: index + 1, body_len }));
        }

        if bytes.len() >= MAX_HEADER_LEN {
            return Err(invalid_data("frame length prefix is longer than three bytes"));
        }
        Ok(None)
    }

    pub fn frame_len(self) -> usize {
        self.header_len + self.body_len
    }
}

/// Bytes read from one socket. Complete frames are used in place; only an
/// incomplete frame at the end is moved to the front before the next read.
pub struct FrameBuffer {
    bytes: Vec<u8>,
    /// First byte not consumed yet.
    start: usize,
    /// End of the bytes read so far.
    end: usize,
}

impl FrameBuffer {
    pub fn new() -> Self {
        Self { bytes: vec![0; INITIAL_CAPACITY], start: 0, end: 0 }
    }

    /// Bytes read but not consumed.
    pub fn pending(&self) -> &[u8] {
        &self.bytes[self.start..self.end]
    }

    pub fn consume(&mut self, len: usize) {
        assert!(len <= self.end - self.start, "consumed more bytes than were read");
        self.start += len;
    }

    /// Consumes the next complete frame and returns the position of its body,
    /// which stays readable through [`slice`](Self::slice) until the next fill.
    pub fn take_frame(&mut self) -> io::Result<Option<Range<usize>>> {
        let Some(header) = FrameHeader::parse(self.pending())? else {
            return Ok(None);
        };
        if self.pending().len() < header.frame_len() {
            return Ok(None);
        }

        let body_start = self.start + header.header_len;
        let body = body_start..body_start + header.body_len;
        self.consume(header.frame_len());
        Ok(Some(body))
    }

    pub fn slice(&self, range: Range<usize>) -> &[u8] {
        &self.bytes[range]
    }

    /// Reads more bytes from `reader`. Returns `false` at the end of the stream.
    pub async fn fill<R: AsyncRead + Unpin>(&mut self, reader: &mut R) -> io::Result<bool> {
        self.make_room()?;
        let read = reader.read(&mut self.bytes[self.end..]).await?;
        self.end += read;
        Ok(read > 0)
    }

    /// Moves the unconsumed bytes to the front, then grows the buffer if the
    /// frame they start does not fit.
    fn make_room(&mut self) -> io::Result<()> {
        if self.start == self.end {
            self.start = 0;
            self.end = 0;
        } else if self.start > 0 {
            self.bytes.copy_within(self.start..self.end, 0);
            self.end -= self.start;
            self.start = 0;
        }

        let frame_len = match FrameHeader::parse(self.pending())? {
            Some(header) => header.frame_len(),
            None => MAX_HEADER_LEN,
        };
        // Room for the whole frame, and for at least one more byte.
        let needed = frame_len.max(self.end + 1);
        if needed > self.bytes.len() {
            // `needed` never exceeds the largest frame, so the cap keeps it.
            let grown = needed.max(self.bytes.len() * 2).min(MAX_HEADER_LEN + MAX_FRAME_LEN);
            self.bytes.resize(grown, 0);
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "../tests/tests_frame.rs"]
pub(crate) mod tests;
