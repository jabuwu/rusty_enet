//! [ENet](https://github.com/lsalzman/enet) transpiled to Rust, and made agnostic to the underlying
//! socket. Supports [`std::net::UdpSocket`] out of the box. Works in WASM if you bring your own WebRTC
//! interface or similar.
//!
//! Much of the docs are copied from the [ENet Website](http://sauerbraten.org/enet/index.html),
//! both for convenience, and in case that resource is unavailable for any reason.
//!
//! > ENet's purpose is to provide a relatively thin, simple and robust network communication layer
//! > on top of UDP (User Datagram Protocol). The primary feature it provides is optional reliable,
//! > in-order delivery of packets.  
//! >
//! > ENet omits certain higher level networking features such as authentication, lobbying, server
//! > discovery, encryption, or other similar tasks that are particularly application specific so
//! > that the library remains flexible, portable, and easily embeddable.
//!
//! [See the examples](https://github.com/jabuwu/rusty_enet/tree/main/examples)
//!
//! # Features and Architecture
//!
//! ENet evolved specifically as a UDP networking layer for the multiplayer first person shooter
//! Cube.
//!
//! Cube necessitated low latency communication with data sent out very frequently, so TCP was an
//! unsuitable choice due to its high latency and stream orientation. UDP, however, lacks many
//! sometimes necessary features from TCP such as reliability, sequencing, unrestricted packet
//! sizes, and connection management. So UDP by itself was not suitable as a network protocol
//! either. No suitable freely available networking libraries existed at the time of ENet's creation
//! to fill this niche.
//!
//! UDP and TCP could have been used together in Cube to benefit somewhat from both of their
//! features, however, the resulting combinations of protocols still leaves much to be desired.
//! TCP lacks multiple streams of communication without resorting to opening many sockets and
//! complicates delineation of packets due to its buffering behavior. UDP lacks sequencing,
//! connection management, management of bandwidth resources, and imposes limitations on the size of
//! packets. A significant investment is required to integrate these two protocols, and the end
//! result is worse off in features and performance than the uniform protocol presented by ENet.
//!
//! ENet thus attempts to address these issues and provide a single, uniform protocol layered over
//! UDP to the developer with the best features of UDP and TCP as well as some useful features
//! neither provide, with a much cleaner integration than any resulting from a mixture of UDP and
//! TCP.
//!
//! ## Connection Management
//!
//! ENet provides a simple connection interface over which to communicate with a foreign host. The
//! liveness of the connection is actively monitored by pinging the foreign host at frequent
//! intervals, and also monitors the network conditions from the local host to the foreign host such
//! as the mean round trip time and packet loss in this fashion.
//!
//! ## Sequencing
//!
//! Rather than a single byte stream that complicates the delineation of packets, ENet presents
//! connections as multiple, properly sequenced packet streams that simplify the transfer of various
//! types of data.
//!
//! ENet provides sequencing for all packets by assigning to each sent packet a sequence number that
//! is incremented as packets are sent. ENet guarantees that no packet with a higher sequence number
//! will be delivered before a packet with a lower sequence number, thus ensuring packets are
//! delivered exactly in the order they are sent.
//!
//! For unreliable packets, ENet will simply discard the lower sequence number packet if a packet
//! with a higher sequence number has already been delivered. This allows the packets to be
//! dispatched immediately as they arrive, and reduce latency of unreliable packets to an absolute
//! minimum. For reliable packets, if a higher sequence number packet arrives, but the preceding
//! packets in the sequence have not yet arrived, ENet will stall delivery of the higher sequence
//! number packets until its predecessors have arrived.
//!
//! ## Channels
//!
//! Since ENet will stall delivery of reliable packets to ensure proper sequencing, and consequently
//! any packets of higher sequence number whether reliable or unreliable, in the event the reliable
//! packet's predecessors have not yet arrived, this can introduce latency into the delivery of
//! other packets which may not need to be as strictly ordered with respect to the packet that
//! stalled their delivery.
//!
//! To combat this latency and reduce the ordering restrictions on packets, ENet provides multiple
//! channels of communication over a given connection. Each channel is independently sequenced, and
//! so the delivery status of a packet in one channel will not stall the delivery of other packets
//! in another channel.
//!
//! ## Reliability
//!
//! ENet provides optional reliability of packet delivery by ensuring the foreign host acknowledges
//! receipt of all reliable packets. ENet will attempt to resend the packet up to a reasonable
//! amount of times, if no acknowledgement of the packet's receipt happens within a specified
//! timeout. Retry timeouts are progressive and become more lenient with every failed attempt to
//! allow for temporary turbulence in network conditions.
//!
//! ## Fragmentation and Reassembly
//!
//! ENet will send and deliver packets regardless of size. Large packets are fragmented into many
//! smaller packets of suitable size, and reassembled on the foreign host to recover the original
//! packet for delivery. The process is entirely transparent to the developer.
//!
//! ## Aggregation
//!
//! ENet aggregates all protocol commands, including acknowledgements and packet transfer, into
//! larger protocol packets to ensure the proper utilization of the connection and to limit the
//! opportunities for packet loss that might otherwise result in further delivery latency.
//!
//! ## Adaptability
//!
//! ENet provides an in-flight data window for reliable packets to ensure connections are not
//! overwhelmed by volumes of packets. It also provides a static bandwidth allocation mechanism to
//! ensure the total volume of packets sent and received to a host don't exceed the host's
//! capabilities. Further, ENet also provides a dynamic throttle that responds to deviations from
//! normal network connections to rectify various types of network congestion by further limiting
//! the volume of packets sent.
//!
//! # Host Servicing
//!
//! ENet only sends and receives packets during [`Host::service`], so performance is closely tied
//! to this function. Applications have three options when it comes to calling this function:
//!
//! 1. Run on the main thread, updating every frame.
//! 2. Run on a separate thread, blocking between updates.
//! 3. Run on a separate thread, sleeping for a fixed amount of time between updates.
//!
//! Running on the main thread (option 1) might have a noticeable impact on performance, as games
//! are typically tied to vsync, and a 60hz display may cause an additional 16ms delay to
//! networking. For this reason, it's common to have a dedicated thread to service network sockets.
//!
//! The original C library provides a convenient mechanism for servicing ENet in a dedicated
//! thread while blocking between updates (option 2): the `enet_host_service` function takes a
//! `timeout` parameter, which when set to a number greater than 0, blocks the thread until either:
//!
//! * The underlying socket is ready to send or receive data.
//! * The specified timeout duration (in milliseconds) is reached (important for sending occasional
//!   keep-alive packets).
//!
//! The C library implements this behavior by using [select], which does not have an equivalent in
//! the Rust standard library. For that reason, this library does not implement similar behavior,
//! and it's not possible to achieve option 2 with [`UdpSocket`](`std::net::UdpSocket`). Option 3
//! is the recommended approach if using [`UdpSocket`](`std::net::UdpSocket`), however, it would be
//! possible to implement option 2 using [`mio::net::UdpSocket`], [`socket2::Socket`], or even
//! [`tokio::net::UdpSocket`] with the proper setup, as they each provide some [select] mechanism.
//!
//! [select]: https://en.wikipedia.org/wiki/Select_(Unix)
//! [`mio::net::UdpSocket`]: https://docs.rs/mio/latest/mio/net/struct.UdpSocket.html
//! [`socket2::Socket`]: https://docs.rs/socket2/latest/socket2/struct.Socket.html
//! [`tokio::net::UdpSocket`]: https://docs.rs/tokio/latest/tokio/net/struct.UdpSocket.html

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
    missing_docs,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::manual_assert,
    clippy::ptr_cast_constness,
    clippy::ptr_as_ptr,
    clippy::default_trait_access,
    clippy::explicit_iter_loop,
    clippy::explicit_into_iter_loop,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::redundant_feature_names,
    clippy::semicolon_if_nothing_returned,
    clippy::must_use_candidate,
    clippy::borrow_as_ptr,
    clippy::items_after_statements,
    clippy::single_match_else,
    clippy::bool_to_int_with_if,
    clippy::unnecessary_cast
)]
// https://github.com/rust-lang/rust-clippy/issues/11382
#![allow(clippy::arc_with_non_send_sync)]

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate alloc;

mod address;
mod c;
mod compressor;
mod crc32;
mod event;
mod host;
mod packet;
mod peer;
mod read_write;
mod socket;
mod time;
mod version;

pub use address::*;
pub(crate) use c::*;
pub use compressor::*;
pub use crc32::*;
pub use event::*;
pub use host::*;
pub use packet::*;
pub use peer::*;
pub use read_write::*;
pub use socket::*;
pub use time::*;
pub use version::*;

#[cfg(any(feature = "connected", doc))]
#[cfg_attr(docsrs, doc(cfg(feature = "connected")))]
pub mod connected;
pub mod error;

/// Constants provided by ENet.
#[allow(missing_docs)]
pub mod consts;

#[cfg(feature = "std")]
#[cfg(test)]
mod test;

#[cfg(feature = "std")]
pub(crate) use std::{boxed::Box, collections::VecDeque, vec::Vec};

#[cfg(not(feature = "std"))]
pub(crate) use alloc::{boxed::Box, collections::VecDeque, vec::Vec};
