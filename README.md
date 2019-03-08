# async-datagram
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Async datagram traits.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Examples
__Basic usage__
```rust
#![feature(futures_api)]

use async_datagram::AsyncDatagram;

struct Udp;

impl AsyncDatagram for Udp {
  type Target = std::net::SocketAddr
  type Err = io::Error;

  fn poll_send_to(
    &mut self,
    waker: &Waker,
    buf: &[u8],
    target: Self::Target,
  ) -> Poll<Result<usize, Self::Err>> {
    Poll::Ready(Ok(0))
  }

  fn poll_recv_from(
    &mut self,
    waker: &Waker,
    buf: &mut [u8],
  ) -> Poll<Result<(usize, Self::Target), Self::Err>> {
    Poll::Pending
  }
}
```

## Installation
```sh
$ cargo add async-datagram
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
None.

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/async-datagram.svg?style=flat-square
[2]: https://crates.io/crates/async-datagram
[3]: https://img.shields.io/travis/yoshuawuyts/async-datagram/master.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/async-datagram
[5]: https://img.shields.io/crates/d/async-datagram.svg?style=flat-square
[6]: https://crates.io/crates/async-datagram
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/async-datagram

[releases]: https://github.com/yoshuawuyts/async-datagram/releases
[contributing]: https://github.com/yoshuawuyts/async-datagram/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/async-datagram/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/async-datagram/labels/help%20wanted
