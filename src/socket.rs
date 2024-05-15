#[cfg(feature = "std")]
use std::{
    io::{self, ErrorKind},
    net::{SocketAddr, UdpSocket},
};

use crate::{Address, Vec};

/// Socket options provided by ENet and passed to [`Socket::init`] when creating a
/// [`Host`](`crate::Host`).
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
    type PeerAddress: Address + 'static;
    /// Errors returned by this socket.
    type Error: core::fmt::Debug + Send + Sync + 'static;

    /// Initialize the socket with options passed down by ENet.
    ///
    /// Called in [`Host::new`](`crate::Host::new`).
    fn init(&mut self, _socket_options: SocketOptions) -> Result<(), Self::Error> {
        Ok(())
    }
    /// Try to send data. Should return the number of bytes successfully sent, or an error.
    fn send(&mut self, address: Self::PeerAddress, buffer: &[u8]) -> Result<usize, Self::Error>;
    /// Try to receive data. May return an error, or optionally, a data packet.
    ///
    /// Data packets are wrapped in [`PacketReceived`]. See its docs for more info.
    fn receive(
        &mut self,
        mtu: usize,
    ) -> Result<Option<(Self::PeerAddress, PacketReceived)>, Self::Error>;
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
    /// A complete packet was received.
    Complete(Vec<u8>),
    /// A partial packet was received.
    Partial,
}

#[cfg(feature = "std")]
impl Socket for UdpSocket {
    type PeerAddress = SocketAddr;
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

    fn receive(&mut self, mtu: usize) -> Result<Option<(SocketAddr, PacketReceived)>, io::Error> {
        let mut buffer = vec![0; mtu];
        match self.recv_from(&mut buffer) {
            Ok((recv_length, recv_addr)) => {
                // TODO: MSG_TRUNC? (not supported by rust stdlib)
                Ok(Some((
                    recv_addr,
                    PacketReceived::Complete(Vec::from(&buffer[0..recv_length])),
                )))
            }
            Err(err) if err.kind() == ErrorKind::WouldBlock => Ok(None),
            Err(err) => Err(err),
        }
    }
}
