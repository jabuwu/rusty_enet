//! Error types.

use crate::Socket;

/// Error for [`Host::new`](`crate::Host::new`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HostNewError<S: Socket> {
    /// Failed to create a new ENet host due to a bad parameter.
    BadParameter(BadParameter),
    /// Failed to create a new ENet host because socket initialization failed.
    FailedToInitializeSocket(S::Error),
}

#[cfg(feature = "std")]
impl<S: Socket> std::error::Error for HostNewError<S> {}

impl<S: Socket> core::fmt::Debug for HostNewError<S> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            HostNewError::BadParameter(err) => f.debug_tuple("BadParameter").field(&err).finish(),
            HostNewError::FailedToInitializeSocket(err) => f
                .debug_tuple("FailedToInitializeSocket")
                .field(&err)
                .finish(),
        }
    }
}

impl<S: Socket> core::fmt::Display for HostNewError<S> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            HostNewError::BadParameter(_) => {
                f.write_str("Failed to create a new ENet host due to a bad parameter.")
            }
            HostNewError::FailedToInitializeSocket(_) => f.write_str(
                "Failed to create a new ENet host because socket initialization failed.",
            ),
        }
    }
}

/// Error for [`Peer::send`](`crate::Peer::send`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PeerSendError {
    /// Cannot send to peer because it is not connected.
    NotConnected,
    /// Cannot send to peer on an invalid channel.
    InvalidChannel,
    /// Cannot send to peer because the packet is too large.
    PacketTooLarge,
    /// Cannot send to peer because the fragment count was exceeded.
    FragmentsExceeded,
    /// Cannot send to peer because the packet failed to queue.
    FailedToQueue,
}

#[cfg(feature = "std")]
impl std::error::Error for PeerSendError {}

impl core::fmt::Display for PeerSendError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            PeerSendError::NotConnected => {
                f.write_str("Cannot send to an ENet peer because it is not connected.")
            }
            PeerSendError::InvalidChannel => {
                f.write_str("Cannot send to an ENet peer on an invalid channel.")
            }
            PeerSendError::PacketTooLarge => {
                f.write_str("Cannot send to an ENet peer because the packet is too large.")
            }
            PeerSendError::FragmentsExceeded => {
                f.write_str("Cannot send to an ENet peer because the fragment count was exceeded.")
            }
            PeerSendError::FailedToQueue => {
                f.write_str("Cannot send to an ENet peer because the packet failed to queue.")
            }
        }
    }
}

/// A bad parameter was passed to a method.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BadParameter {
    /// The name of the method where this parameter was checked.
    pub method: &'static str,
    /// The name of the parameter itself.
    pub parameter: &'static str,
}

#[cfg(feature = "std")]
impl std::error::Error for BadParameter {}

impl core::fmt::Display for BadParameter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str("A bad parameter (")?;
        f.write_str(self.parameter)?;
        f.write_str(") was passed to ")?;
        f.write_str(self.method)
    }
}

/// Failed to connect because there were no available ENet peer slots.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NoAvailablePeers;

#[cfg(feature = "std")]
impl std::error::Error for NoAvailablePeers {}

impl core::fmt::Display for NoAvailablePeers {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str("Failed to connect because there were no available ENet peer slots.")
    }
}
