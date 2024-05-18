use std::collections::VecDeque;

use crate::{Address, PacketReceived, Socket, SocketOptions};

/// Provides a Read/Write interface for use with [`Host`](`crate::Host`).
///
/// This provides a useful alternative to implementing the [`Socket`] trait, especially when
/// interfacing with multiple kinds of sockets at once.
/// 
/// The call to [`Socket::init`] never fails for this type, so it is safe to assume
/// [`Host::new`](`crate::Host::new`) will not fail with
/// [`HostNewError::FailedToInitializeSocket`](`crate::error::HostNewError::FailedToInitializeSocket`).
///
/// ```
/// use std::convert::Infallible;
///
/// use rusty_enet::{Host, HostSettings, ReadWrite};
///
/// let mut host = Host::new(ReadWrite::<(), Infallible>::new(), HostSettings::default()).unwrap();
///
/// // Write packets to the host (usually from one or more sockets)
/// host.socket_mut().write((/*some address*/), vec![]);
///
/// // Read packets ENet wants to send, then send to sockets based on the address
/// if let Some((address, packet)) = host.socket_mut().read() {
///     dbg!((address, packet));
/// }
#[derive(Debug)]
pub struct ReadWrite<A: Address + 'static, E: std::error::Error + Send + Sync + 'static> {
    inbound: VecDeque<(A, Vec<u8>)>,
    outbound: VecDeque<(A, Vec<u8>)>,
    error: Option<E>,
}

impl<A: Address + 'static, E: std::error::Error + Send + Sync + 'static> ReadWrite<A, E> {
    /// Create an intermediate Read/Write socket for use with [`Host`](`crate::Host`).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Write packets to the ENet host.
    pub fn write(&mut self, address: A, buffer: Vec<u8>) {
        self.inbound.push_back((address, buffer));
    }

    /// Read packets from the ENet host.
    pub fn read(&mut self) -> Option<(A, Vec<u8>)> {
        self.outbound.pop_front()
    }

    /// Send an error to the ENet host, which will bubble up as a receive error.
    pub fn error(&mut self, error: E) {
        self.error = Some(error);
    }
}

impl<A: Address + 'static, E: std::error::Error + Send + Sync + 'static> Default
    for ReadWrite<A, E>
{
    fn default() -> Self {
        Self {
            inbound: VecDeque::new(),
            outbound: VecDeque::new(),
            error: None,
        }
    }
}

impl<A: Address + 'static, E: std::error::Error + Send + Sync + 'static> Socket
    for ReadWrite<A, E>
{
    type Address = A;
    type Error = E;

    fn init(&mut self, _socket_options: SocketOptions) -> Result<(), Self::Error> {
        // NOTE: this implementation must not become fallable
        Ok(())
    }

    fn send(&mut self, address: A, buffer: &[u8]) -> Result<usize, E> {
        self.outbound.push_back((address, buffer.to_vec()));
        Ok(buffer.len())
    }

    fn receive(&mut self, _mtu: usize) -> Result<Option<(A, PacketReceived)>, E> {
        if let Some(error) = self.error.take() {
            Err(error)
        } else if let Some((address, buffer)) = self.inbound.pop_front() {
            Ok(Some((address, PacketReceived::Complete(buffer))))
        } else {
            Ok(None)
        }
    }
}
