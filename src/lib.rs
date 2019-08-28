#![forbid(unsafe_code, future_incompatible)]
#![deny(missing_debug_implementations, bad_style)]
#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![no_std]

//! Async datagram traits.
//!
//! ## Example
//!
//! ```rust
//! use async_datagram::AsyncDatagram;
//! use std::task::{Context, Poll};
//! use std::pin::Pin;
//!
//! struct Udp;
//!
//! impl AsyncDatagram for Udp {
//!   type Sender = std::net::SocketAddr;
//!   type Receiver = std::net::SocketAddr;
//!   type Err = std::io::Error;
//!
//!   fn poll_send_to(
//!     self: Pin<&mut Self>,
//!     cx: &mut Context<'_>,
//!     buf: &[u8],
//!     target: &Self::Receiver,
//!   ) -> Poll<Result<usize, Self::Err>> {
//!     Poll::Ready(Ok(0))
//!   }
//!
//!   fn poll_recv_from(
//!     self: Pin<&mut Self>,
//!     cx: &mut Context<'_>,
//!     buf: &mut [u8],
//!   ) -> Poll<Result<(usize, Self::Sender), Self::Err>> {
//!     Poll::Pending
//!   }
//! }
//! ```

mod ext;

pub use ext::{AsyncDatagramExt, RecvFrom, SendTo};

use core::{
  pin::Pin,
  task::{Context, Poll},
};

/// Implement a datagram protocol.
pub trait AsyncDatagram {
  /// Specifies which target to send the data to.
  type Sender;

  /// Specifies which target the data was received from.
  type Receiver;

  /// The type of failures yielded by this trait.
  type Err;

  /// Sends data on the IO interface to the specified target.
  ///
  /// On success, returns the number of bytes written.
  fn poll_send_to(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &[u8],
    receiver: &Self::Receiver,
  ) -> Poll<Result<usize, Self::Err>>;

  /// Receives data from the IO interface.
  ///
  /// On success, returns the number of bytes read and the target from whence
  /// the data came.
  #[allow(clippy::type_complexity)]
  fn poll_recv_from(
    self: Pin<&mut Self>,
    cx: &mut Context<'_>,
    buf: &mut [u8],
  ) -> Poll<Result<(usize, Self::Sender), Self::Err>>;
}
