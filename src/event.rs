use crate::{Packet, Peer, Socket};

/// An ENet event returned by [`Host::service`](`crate::Host::service`).
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
