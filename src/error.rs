use crate::Socket;

/// Error for [`Host::new`](`crate::Host::new`).
pub enum HostNewError<S: Socket> {
    /// Failed to create a new ENet host due to a bad parameter.
    BadParameter,
    /// Failed to create a new ENet host because socket initialization failed.
    FailedToInitializeSocket(S::Error),
}

impl<S: Socket> std::fmt::Debug for HostNewError<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HostNewError::BadParameter => f.write_str("BadParameter"),
            HostNewError::FailedToInitializeSocket(f0) => f
                .debug_tuple("FailedToInitializeSocket")
                .field(&f0)
                .finish(),
        }
    }
}

impl<S: Socket> std::fmt::Display for HostNewError<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HostNewError::BadParameter => {
                f.write_str("Failed to create a new ENet host due to a bad parameter.")
            }
            HostNewError::FailedToInitializeSocket(_) => f.write_str(
                "Failed to create a new ENet host because socket initialization failed.",
            ),
        }
    }
}

/// Error for [`Peer::send`](`crate::Peer::send`).
#[derive(Debug)]
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

impl std::error::Error for PeerSendError {}

impl std::fmt::Display for PeerSendError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
#[derive(Debug)]
pub struct BadParameter {
    /// The name of the method where this parameter was checked.
    pub method: &'static str,
    /// The name of the parameter itself.
    pub parameter: &'static str,
}

impl std::error::Error for BadParameter {}

impl std::fmt::Display for BadParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&format!(
            "A bad parameter ({}) was passed to {}",
            self.parameter, self.method
        ))
    }
}

/// Failed to connect because there were no available ENet peer slots.
#[derive(Debug)]
pub struct NoAvailablePeers;

impl std::error::Error for NoAvailablePeers {}

impl std::fmt::Display for NoAvailablePeers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Failed to connect because there were no available ENet peer slots.")
    }
}
