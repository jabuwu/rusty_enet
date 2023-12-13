use std::{fmt::Debug, slice};

use crate::{
    enet_packet_create, enet_packet_destroy, ENetPacket, ENET_PACKET_FLAG_RELIABLE,
    ENET_PACKET_FLAG_SENT, ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT, ENET_PACKET_FLAG_UNSEQUENCED,
};

/// Types of packets supported by ENet, used with [`Packet::new`].
///
/// See [`Sequencing`](`crate#sequencing`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketKind {
    /// An unreliable packet, with optional sequencing. A sequenced unreliable packet will cause
    /// unsequenced packets to simply be discarded if they were to be dispatched out of order.
    ///
    /// Note that packets of this kind will be sent reliably if they are too large to fit within the
    /// maximum transmission unit (MTU). To avoid this behavior, use
    /// [`PacketKind::AlwaysUnreliable`] instead.
    Unreliable {
        /// Should the packets be sequenced? Packets received out of order will be discarded.
        sequenced: bool,
    },
    /// An unreliable packet, with optional sequencing. Guaranteed to be unreliable, see
    /// [`PacketKind::Unreliable`].
    AlwaysUnreliable {
        /// Should the packets be sequenced? Packets received out of order will be discarded.
        sequenced: bool,
    },
    /// An reliable packet, with enforced sequencing.
    ///
    /// See [`Reliability`](`crate#reliability`).
    Reliable,
}

/// An ENet data packet that may be sent to or received from a peer.
///
/// See [`Fragmentation and Reassembly`](`crate#fragmentation-and-reassembly`).
///
/// For more information on the kinds of ENet packets, see [`PacketKind`].
pub struct Packet {
    pub(crate) packet: *mut ENetPacket,
}

unsafe impl Send for Packet {}
unsafe impl Sync for Packet {}

impl Packet {
    /// Create a new packet manually from its data and kind.
    ///
    /// Some convenience methods exist that can be used instead:
    /// - [`Packet::unreliable`]
    /// - [`Packet::unreliable_unsequenced`]
    /// - [`Packet::always_unreliable`]
    /// - [`Packet::always_unreliable_unsequenced`]
    /// - [`Packet::reliable`]
    #[must_use]
    pub fn new(data: &[u8], flags: PacketKind) -> Self {
        let packet = unsafe {
            enet_packet_create(
                data.as_ptr(),
                data.len(),
                match flags {
                    PacketKind::Unreliable { sequenced: true } => 0,
                    PacketKind::Unreliable { sequenced: false } => ENET_PACKET_FLAG_UNSEQUENCED,
                    PacketKind::AlwaysUnreliable { sequenced: true } => {
                        ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT
                    }
                    PacketKind::AlwaysUnreliable { sequenced: false } => {
                        ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT | ENET_PACKET_FLAG_UNSEQUENCED
                    }
                    PacketKind::Reliable => ENET_PACKET_FLAG_RELIABLE,
                },
            )
        };
        unsafe {
            (*packet).reference_count += 1;
        }
        Self { packet }
    }

    /// Create a new unreliable packet with
    /// [`PacketKind::Unreliable { sequenced: true }`](`PacketKind::Unreliable`)
    #[must_use]
    pub fn unreliable(data: &[u8]) -> Self {
        Self::new(data, PacketKind::Unreliable { sequenced: true })
    }

    /// Create a new unreliable packet with
    /// [`PacketKind::Unreliable { sequenced: false }`](`PacketKind::Unreliable`)
    #[must_use]
    pub fn unreliable_unsequenced(data: &[u8]) -> Self {
        Self::new(data, PacketKind::Unreliable { sequenced: false })
    }

    /// Create a new always unreliable packet with
    /// [`PacketKind::AlwaysUnreliable { sequenced: true }`](`PacketKind::AlwaysUnreliable`)
    #[must_use]
    pub fn always_unreliable(data: &[u8]) -> Self {
        Self::new(data, PacketKind::AlwaysUnreliable { sequenced: true })
    }

    /// Create a new always unreliable packet with
    /// [`PacketKind::AlwaysUnreliable { sequenced: false }`](`PacketKind::AlwaysUnreliable`)
    #[must_use]
    pub fn always_unreliable_unsequenced(data: &[u8]) -> Self {
        Self::new(data, PacketKind::AlwaysUnreliable { sequenced: false })
    }

    /// Create a new unreliable packet with [`PacketKind::Reliable`]
    #[must_use]
    pub fn reliable(data: &[u8]) -> Self {
        Self::new(data, PacketKind::Reliable)
    }

    /// Get this packet's [`PacketKind`].
    #[must_use]
    pub fn kind(&self) -> PacketKind {
        let flags = unsafe { (*self.packet).flags | !ENET_PACKET_FLAG_SENT };
        let sequenced = flags & ENET_PACKET_FLAG_UNSEQUENCED == 0;
        if flags & ENET_PACKET_FLAG_RELIABLE != 0 {
            PacketKind::Reliable
        } else if flags & ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT != 0 {
            PacketKind::AlwaysUnreliable { sequenced }
        } else {
            PacketKind::Unreliable { sequenced }
        }
    }

    /// Get the byte array contained in this packet.
    #[must_use]
    pub fn data(&self) -> &[u8] {
        unsafe { slice::from_raw_parts((*self.packet).data, (*self.packet).data_length) }
    }

    pub(crate) fn new_from_ptr(packet: *mut ENetPacket) -> Self {
        unsafe {
            (*packet).reference_count += 1;
        }
        Self { packet }
    }
}

impl Clone for Packet {
    fn clone(&self) -> Self {
        unsafe {
            (*self.packet).reference_count += 1;
        }
        Self {
            packet: self.packet,
        }
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        unsafe {
            (*self.packet).reference_count -= 1;
            if (*self.packet).reference_count == 0 {
                enet_packet_destroy(self.packet);
            }
        }
    }
}

impl Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let packet = unsafe { &(*self.packet) };
        f.debug_struct("Packet")
            .field("data", &packet.data)
            .field("dataLength", &packet.data_length)
            .field("flags", &packet.flags)
            .field("kind", &self.kind())
            .finish()
    }
}
