use crate::{Packet, Peer, PeerID, Socket};

/// An ENet event returned by [`Host::service`](`crate::Host::service`).
#[derive(Debug)]
pub enum Event<'a, S: Socket> {
    /// A new peer has connected.
    Connect {
        /// Peer that generated the event.
        peer: &'a mut Peer<S>,
        /// Data associated with the event, sent by the client on connect.
        data: u32,
    },
    /// A peer has disconnected.
    Disconnect {
        /// Peer that generated the event.
        peer: &'a mut Peer<S>,
        /// Data associated with the event, sent by the client on disconnect.
        data: u32,
    },
    /// A peer sent a packet to us.
    Receive {
        /// Peer that generated the event.
        peer: &'a mut Peer<S>,
        /// Channel on the peer that generated the event.
        channel_id: u8,
        /// Packet associated with the event.
        packet: Packet,
    },
}

impl<'a, S: Socket> Event<'a, S> {
    /// Remove the peer reference from this event, converting into an [`EventNoRef`].
    pub fn no_ref(self) -> EventNoRef {
        match self {
            Self::Connect { peer, data } => EventNoRef::Connect {
                peer: peer.id(),
                data,
            },
            Self::Disconnect { peer, data } => EventNoRef::Disconnect {
                peer: peer.id(),
                data,
            },
            Self::Receive {
                peer,
                channel_id,
                packet,
            } => EventNoRef::Receive {
                peer: peer.id(),
                channel_id,
                packet,
            },
        }
    }
}

/// An ENet event, like [`Event`], but without peer references.
///
/// Acquired with [`Event::no_ref`].
#[derive(Debug, Clone)]
pub enum EventNoRef {
    /// A new peer has connected.
    Connect {
        /// Peer that generated the event.
        peer: PeerID,
        /// Data associated with the event, sent by the client on connect.
        data: u32,
    },
    /// A peer has disconnected.
    Disconnect {
        /// Peer that generated the event.
        peer: PeerID,
        /// Data associated with the event, sent by the client on disconnect.
        data: u32,
    },
    /// A peer sent a packet to us.
    Receive {
        /// Peer that generated the event.
        peer: PeerID,
        /// Channel on the peer that generated the event.
        channel_id: u8,
        /// Packet associated with the event.
        packet: Packet,
    },
}
