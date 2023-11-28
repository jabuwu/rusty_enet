use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    User(String),
    #[error("Invalid value for {param}")]
    InvalidValueForParameter { param: &'static str },
    #[error("No available peer.")]
    NoAvailablePeer,
    #[error("Invalid peer.")]
    InvalidPeer,
    #[error("Peer not connected.")]
    PeerNotConnected,
    #[error("Packet too large.")]
    PacketTooLarge,
    #[error("Too many fragments.")]
    TooManyFragments,
    #[error("Failed to bind.")]
    FailedToBind(Box<dyn std::error::Error>),
    #[error("Failed to set socket option.")]
    FailedToSetSocketOption(Box<dyn std::error::Error>),
    #[error("Failed to send.")]
    FailedToSend(Box<dyn std::error::Error>),
    #[error("Failed to receive.")]
    FailedToReceive(Box<dyn std::error::Error>),
    #[error("Unknown error.")]
    Unknown,
}
