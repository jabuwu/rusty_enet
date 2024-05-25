#[cfg(feature = "std")]
use std::{
    io::{self, ErrorKind},
    net::{SocketAddr, UdpSocket},
};

use crate::{consts::PROTOCOL_MAXIMUM_MTU, Address};

// This macro allows the same doc comment to apply to both variants.
macro_rules! socket_error {
    ($(#[$($attrss:tt)*])*) => {
        #[cfg(feature = "std")]
        $(#[$($attrss)*])*
        pub trait SocketError: std::error::Error {}
        #[cfg(feature = "std")]
        impl<T: std::error::Error> SocketError for T {}

        #[cfg(not(feature = "std"))]
        $(#[$($attrss)*])*
        pub trait SocketError: core::fmt::Debug {}
        #[cfg(not(feature = "std"))]
        impl<T: core::fmt::Debug> SocketError for T {}
    };
}
socket_error!(
    /// A trait specifying the bounds of [`Socket::Error`].
    ///
    /// This normally binds errors to [`std::error::Error`], however in a `#![no_std]` environment,
    /// it binds them to only [`core::fmt::Debug`].
);

/// The maximum amount of bytes ENet will ever send or receive. Useful for allocating buffers when
/// sending and receiving.
///
/// The actual MTU used by hosts and peers is typically much lower than this maximum and can be
/// changed with [`Host::set_mtu`](`crate::Host::set_mtu`) and
/// [`Peer::set_mtu`](`crate::Peer::set_mtu`).
///
/// A shorter an easier to remember equivalent to [`PROTOCOL_MAXIMUM_MTU`].
pub const MTU_MAX: usize = PROTOCOL_MAXIMUM_MTU;

/// Socket options provided by ENet and passed to [`Socket::init`] when creating a
/// [`Host`](`crate::Host`).
///
/// In the C library, ENet sets the socket to non-blocking mode (`O_NONBLOCK`) and enables the
/// broadcast (`SO_BROADCAST`) address, as well as setting the maximum socket receive buffer
/// (`SO_RCVBUF`) and send buffer (`SO_SNDBUF`).
///
/// An implementation of [`Socket::init`] should always assume ENet runs in non-blocking mode. The
/// maximum buffers ENet wants to set are sent as fields in this struct, although setting these
/// isn't actually necessary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SocketOptions {
    /// Size of the receive buffer desired by ENet.
    pub receive_buffer: usize,
    /// Size of the send buffer desired by ENet.
    pub send_buffer: usize,
}

/// A trait for implementing the underlying data transport layer ENet uses.
///
/// An implementation for [`std::net::UdpSocket`] is provided out of the box.
///
/// If implementing this trait is cumbersome, is may be easier to use
/// [`ReadWrite`](`crate::ReadWrite`).
#[allow(clippy::type_complexity, clippy::missing_errors_doc)]
pub trait Socket: Sized {
    /// The address type to use, which must implement [`Address`].
    ///
    /// An example is the standard library's [`std::net::SocketAddr`], used with
    /// [`std::net::UdpSocket`].
    type Address: Address;
    /// Errors returned by this socket.
    type Error: SocketError;

    /// Initialize the socket with options passed down by ENet.
    ///
    /// Called in [`Host::new`](`crate::Host::new`). If this function returns an error, it is
    /// bubbled up through [`Host::new`](`crate::Host::new`).
    fn init(&mut self, _socket_options: SocketOptions) -> Result<(), Self::Error> {
        Ok(())
    }
    /// Try to send data. Should return the number of bytes successfully sent, or an error.
    fn send(&mut self, address: Self::Address, buffer: &[u8]) -> Result<usize, Self::Error>;

    /// Try to receive data from the socket into a buffer of size [`MTU_MAX`].
    ///
    /// A received packet should be written into the provided buffer. If a packet is received that
    /// is larger than [`MTU_MAX`], it should simply be discarded. ENet will never send a packet
    /// that is larger than this maximum, so if one is received, it was not sent by ENet.
    ///
    /// The return value should be `Ok(None)` if no packet was received. If a packet was received,
    /// the address of the peer socket, as well as the amount of bytes received should be returned.
    /// Packets received may be complete or partial. See [`PacketReceived`] for more info.
    fn receive(
        &mut self,
        buffer: &mut [u8; MTU_MAX],
    ) -> Result<Option<(Self::Address, PacketReceived)>, Self::Error>;
}

/// Return type of [`Socket::receive`], representing either a complete packet, or a partial
/// packet. A partial packet is simply discarded, and ENet may or may not try to send the packet
/// again.
///
/// In the original ENet library, this was used if [recvfrom](https://linux.die.net/man/2/recvmsg)
/// set the `MSG_TRUNC` flag. In this context, it means, "we received something, but we know it's
/// not everything". It should be used for a similar purpose in any [`Socket`] implementations.
///
/// If an implementation can ensure that a full packet is always received, it can always return
/// [`PacketReceived::Complete`].
#[derive(Debug)]
pub enum PacketReceived {
    /// A complete packet was received. The inner value is the size of the packet in bytes.
    Complete(usize),
    /// A partial packet was received.
    Partial,
}

#[cfg(feature = "std")]
impl Socket for UdpSocket {
    type Address = SocketAddr;
    type Error = io::Error;

    fn init(&mut self, _socket_options: SocketOptions) -> Result<(), io::Error> {
        self.set_nonblocking(true)?;
        self.set_broadcast(true)?;
        // TODO: set receive_buffer and send_buffer (not supported by rust stdlib)
        Ok(())
    }

    fn send(&mut self, address: SocketAddr, buffer: &[u8]) -> Result<usize, io::Error> {
        match self.send_to(buffer, address) {
            Ok(sent_length) => Ok(sent_length),
            Err(err) if err.kind() == ErrorKind::WouldBlock => Ok(0),
            Err(err) => Err(err),
        }
    }

    fn receive(
        &mut self,
        buffer: &mut [u8; MTU_MAX],
    ) -> Result<Option<(SocketAddr, PacketReceived)>, io::Error> {
        match self.recv_from(buffer) {
            Ok((recv_length, recv_addr)) => {
                // TODO: MSG_TRUNC? (not supported by rust stdlib)
                Ok(Some((recv_addr, PacketReceived::Complete(recv_length))))
            }
            Err(err) if err.kind() == ErrorKind::WouldBlock => Ok(None),
            Err(err) => Err(err),
        }
    }
}
