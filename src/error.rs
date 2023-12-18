use thiserror::Error;

/// Error type for all ENet functions.
#[derive(Debug, Error)]
pub enum Error {
    /// Invalid peer ID.
    #[error("Invalid peer ID.")]
    InvalidPeerID,
    /// The requested peer is disconnected.
    #[error("The requested peer is disconnected.")]
    PeerDisconnected,
    /// Failed to send.
    #[error("Failed to send.")]
    FailedToSend,
    /// Failed to receive.
    #[error("Failed to receive.")]
    FailedToReceive,
    /// A bad parameter was passed to the function.
    #[error("A bad parameter was passed to the function.")]
    BadParameter,
    /// Failed to create host.
    #[error("Failed to create host.")]
    FailedToCreateHost,
    /// Failed to connect.
    #[error("Failed to connect.")]
    FailedToConnect,
    /// Failed to check events.
    #[error("Failed to check events.")]
    FailedToCheckEvents,
    /// Failed to service host.
    #[error("Failed to service host.")]
    FailedToServiceHost,
}
