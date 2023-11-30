use crate::{Packet, PeerID};

#[derive(Debug, Clone)]
pub enum Event {
    Connect {
        peer: PeerID,
        data: u32,
    },
    Disconnect {
        peer: PeerID,
        data: u32,
    },
    Receive {
        peer: PeerID,
        channel_id: u8,
        packet: Packet,
    },
}
