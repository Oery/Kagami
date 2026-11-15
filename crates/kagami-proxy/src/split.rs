//! A read half and a write half of one socket, for the two pumps.

use std::cell::RefCell;
use std::io;
use std::net::Shutdown;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use futures_lite::future::poll_fn;
use futures_lite::{AsyncRead, AsyncWrite};
use glommio::net::TcpStream;

/// Glommio keeps separate sources for reads and writes, so one pending read
/// and one pending write on the same stream do not disturb each other. Each
/// poll borrows the stream only for its own duration, on the one thread.
pub fn split(stream: TcpStream) -> (ReadHalf, WriteHalf) {
    let stream = Rc::new(RefCell::new(stream));
    (ReadHalf(stream.clone()), WriteHalf(stream))
}

pub struct ReadHalf(Rc<RefCell<TcpStream>>);

pub struct WriteHalf(Rc<RefCell<TcpStream>>);

impl WriteHalf {
    /// Closes both directions of the socket, which also ends a pending read
    /// on its read half.
    pub async fn shutdown(&self) -> io::Result<()> {
        poll_fn(|cx| self.0.borrow().poll_shutdown(cx, Shutdown::Both)).await?;
        Ok(())
    }
}

impl AsyncRead for ReadHalf {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<io::Result<usize>> {
        Pin::new(&mut *self.0.borrow_mut()).poll_read(cx, buf)
    }
}

impl AsyncWrite for WriteHalf {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        Pin::new(&mut *self.0.borrow_mut()).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut *self.0.borrow_mut()).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut *self.0.borrow_mut()).poll_close(cx)
    }
}
