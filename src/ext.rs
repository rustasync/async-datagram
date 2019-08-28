use core::{
  future::Future,
  pin::Pin,
  task::{Context, Poll},
};

use crate::AsyncDatagram;

/// An extension trait that adds utility methods to [`AsyncDatagram`] types.
pub trait AsyncDatagramExt: AsyncDatagram {
  /// Creates a future that will send a given message to the specified target.
  ///
  /// The returned future will resolve to the number of bytes actually sent once the operation
  /// completes.
  fn send_to<'a>(
    &'a mut self,
    buf: &'a [u8],
    receiver: &'a Self::Receiver,
  ) -> SendTo<'a, Self>
  where
    Self: Unpin,
  {
    SendTo {
      socket: self,
      buf,
      receiver,
    }
  }

  /// Creates a future that will receive a message into the provided buffer.
  ///
  /// The returned future will resolve to the number of bytes received and target from whence the
  /// data came  once the operation completes.
  fn recv_from<'a>(&'a mut self, buf: &'a mut [u8]) -> RecvFrom<'a, Self>
  where
    Self: Unpin,
  {
    RecvFrom { socket: self, buf }
  }
}

impl<T: AsyncDatagram> AsyncDatagramExt for T {}

/// Future for the [`send_to`](AsyncDatagramExt::send_to) method.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct SendTo<'a, T: AsyncDatagram + ?Sized> {
  socket: &'a mut T,
  buf: &'a [u8],
  receiver: &'a T::Receiver,
}

impl<T: AsyncDatagram + Unpin + ?Sized> Future for SendTo<'_, T> {
  type Output = Result<usize, T::Err>;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let Self {
      socket,
      buf,
      receiver,
    } = self.get_mut();
    Pin::new(&mut **socket).poll_send_to(cx, buf, receiver)
  }
}

/// Future for the [`recv_from`](AsyncDatagramExt::recv_from) method.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct RecvFrom<'a, T: AsyncDatagram + ?Sized> {
  socket: &'a mut T,
  buf: &'a mut [u8],
}

impl<T: AsyncDatagram + Unpin + ?Sized> Future for RecvFrom<'_, T> {
  type Output = Result<(usize, T::Sender), T::Err>;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let Self { socket, buf } = self.get_mut();
    Pin::new(&mut **socket).poll_recv_from(cx, buf)
  }
}
