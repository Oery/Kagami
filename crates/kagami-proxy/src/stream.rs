//! One side of a socket, with the cipher once the connection is encrypted.
//! Bytes are decrypted and encrypted in place, in the proxy's own buffers.

use std::io;
use std::pin::Pin;
use std::task::{Context, Poll, ready};

use futures_lite::{AsyncRead, AsyncWrite, AsyncWriteExt};

use crate::cipher::{Decryptor, Encryptor};
use crate::split::WriteHalf;

/// Decrypts what it reads, in the caller's buffer, once encryption is on.
pub struct Reader<R> {
    inner: R,
    decryptor: Option<Decryptor>,
}

impl<R> Reader<R> {
    pub fn new(inner: R) -> Self {
        Self { inner, decryptor: None }
    }

    pub fn enable_decryption(&mut self, decryptor: Decryptor) {
        self.decryptor = Some(decryptor);
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for Reader<R> {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
        let this = self.get_mut();
        let read = ready!(Pin::new(&mut this.inner).poll_read(cx, buf))?;

        if let Some(decryptor) = &mut this.decryptor {
            decryptor.decrypt(&mut buf[..read]);
        }

        Poll::Ready(Ok(read))
    }
}

/// Encrypts what it writes, in the caller's buffer, once encryption is on.
pub struct Writer<W> {
    inner: W,
    encryptor: Option<Encryptor>,
}

impl<W: AsyncWrite + Unpin> Writer<W> {
    pub fn new(inner: W) -> Self {
        Self { inner, encryptor: None }
    }

    pub fn enable_encryption(&mut self, encryptor: Encryptor) {
        self.encryptor = Some(encryptor);
    }

    /// Writes all of `bytes`, which hold ciphertext afterwards when
    /// encryption is on. The cipher stream advances by exactly these bytes,
    /// so a failed write ends the connection.
    pub async fn write_all_in_place(&mut self, bytes: &mut [u8]) -> io::Result<()> {
        if let Some(encryptor) = &mut self.encryptor {
            encryptor.encrypt(bytes);
        }

        self.inner.write_all(bytes).await
    }
}

impl Writer<WriteHalf> {
    pub async fn shutdown(&self) -> io::Result<()> {
        self.inner.shutdown().await
    }
}
