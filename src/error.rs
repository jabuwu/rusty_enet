/// Error type for all ENet functions.
#[derive(Debug)]
pub enum Error {
    /// Invalid peer ID.
    InvalidPeerID,
    /// The requested peer is disconnected.
    PeerDisconnected,
    /// Failed to initialize socket.
    FailedToInitializeSocket,
    /// Failed to send.
    FailedToSend,
    /// Failed to receive.
    FailedToReceive,
    /// A bad parameter was passed to the function.
    BadParameter,
    /// Failed to create host.
    FailedToCreateHost,
    /// Failed to connect.
    FailedToConnect,
    /// Failed to check events.
    FailedToCheckEvents,
    /// Failed to service host.
    FailedToServiceHost,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidPeerID => f.write_str("Invalid peer ID."),
            Error::PeerDisconnected => f.write_str("The requested peer is disconnected."),
            Error::FailedToInitializeSocket => f.write_str("Failed to initialize socket."),
            Error::FailedToSend => f.write_str("Failed to send."),
            Error::FailedToReceive => f.write_str("Failed to receive."),
            Error::BadParameter => {
                ::core::write!(f, "A bad parameter was passed to the function.")
            }
            Error::FailedToCreateHost => f.write_str("Failed to create host."),
            Error::FailedToConnect => f.write_str("Failed to connect."),
            Error::FailedToCheckEvents => f.write_str("Failed to check events."),
            Error::FailedToServiceHost => f.write_str("Failed to service host."),
        }
    }
}
