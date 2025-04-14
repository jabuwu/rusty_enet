use core::fmt::Debug;

use crate::{
    enet_packet_create, enet_packet_destroy, Box, ENetPacket, Vec, ENET_PACKET_FLAG_NO_ALLOCATE,
    ENET_PACKET_FLAG_RELIABLE, ENET_PACKET_FLAG_SENT, ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT,
    ENET_PACKET_FLAG_UNSEQUENCED,
};

/// Types of packets supported by ENet, used with [`Packet::new`].
///
/// See [`Sequencing`](`crate#sequencing`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    pub fn new<P: ToRawPacket>(data: P, kind: PacketKind) -> Self {
        let kind_flags = match kind {
            PacketKind::Unreliable { sequenced: true } => 0,
            PacketKind::Unreliable { sequenced: false } => ENET_PACKET_FLAG_UNSEQUENCED,
            PacketKind::AlwaysUnreliable { sequenced: true } => {
                ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT
            }
            PacketKind::AlwaysUnreliable { sequenced: false } => {
                ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT | ENET_PACKET_FLAG_UNSEQUENCED
            }
            PacketKind::Reliable => ENET_PACKET_FLAG_RELIABLE,
        };

        let raw_packet = data.into_packet_data();

        let packet = unsafe {
            enet_packet_create(
                raw_packet.data,
                raw_packet.len,
                kind_flags | ENET_PACKET_FLAG_NO_ALLOCATE,
            )
        };
        unsafe {
            (*packet).free_callback = Some(Self::drop_packet_data::<P>);
            (*packet).user_data = raw_packet.user_data as *mut _;
            (*packet).reference_count += 1;
        }
        Self { packet }
    }

    /// Create a new unreliable packet with
    /// [`PacketKind::Unreliable { sequenced: true }`](`PacketKind::Unreliable`)
    #[must_use]
    pub fn unreliable(data: impl ToRawPacket) -> Self {
        Self::new(data, PacketKind::Unreliable { sequenced: true })
    }

    /// Create a new unreliable packet with
    /// [`PacketKind::Unreliable { sequenced: false }`](`PacketKind::Unreliable`)
    #[must_use]
    pub fn unreliable_unsequenced(data: impl ToRawPacket) -> Self {
        Self::new(data, PacketKind::Unreliable { sequenced: false })
    }

    /// Create a new always unreliable packet with
    /// [`PacketKind::AlwaysUnreliable { sequenced: true }`](`PacketKind::AlwaysUnreliable`)
    #[must_use]
    pub fn always_unreliable(data: impl ToRawPacket) -> Self {
        Self::new(data, PacketKind::AlwaysUnreliable { sequenced: true })
    }

    /// Create a new always unreliable packet with
    /// [`PacketKind::AlwaysUnreliable { sequenced: false }`](`PacketKind::AlwaysUnreliable`)
    #[must_use]
    pub fn always_unreliable_unsequenced(data: impl ToRawPacket) -> Self {
        Self::new(data, PacketKind::AlwaysUnreliable { sequenced: false })
    }

    /// Create a new unreliable packet with [`PacketKind::Reliable`]
    #[must_use]
    pub fn reliable(data: impl ToRawPacket) -> Self {
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
        unsafe { super::from_raw_parts_or_empty((*self.packet).data, (*self.packet).data_length) }
    }

    pub(crate) fn new_from_ptr(packet: *mut ENetPacket) -> Self {
        unsafe {
            (*packet).reference_count += 1;
        }
        Self { packet }
    }

    unsafe fn drop_packet_data<P: ToRawPacket>(packet: *mut ENetPacket) {
        P::drop_packet_data(RawPacket {
            data: (*packet).data,
            len: (*packet).data_length,
            user_data: (*packet).user_data as usize,
        });
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
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let packet = unsafe { &(*self.packet) };
        f.debug_struct("Packet")
            .field("data", &packet.data)
            .field("dataLength", &packet.data_length)
            .field("flags", &packet.flags)
            .field("kind", &self.kind())
            .finish()
    }
}

/// Denotes a byte buffer that can be wrapped in a [`Packet`].
pub trait ToRawPacket {
    /// Takes ownership of this value and converts it into raw packet data.
    fn into_packet_data(self) -> RawPacket;

    /// Drops packet data that was previously created for this type.
    ///
    /// # Safety
    ///
    /// For this function call to be sound, `packet` must have been previously
    /// created with [`Self::into_packet_data`].
    unsafe fn drop_packet_data(packet: RawPacket);
}

impl ToRawPacket for &[u8] {
    fn into_packet_data(self) -> RawPacket {
        self.to_vec().into_packet_data()
    }

    unsafe fn drop_packet_data(packet: RawPacket) {
        Vec::<u8>::drop_packet_data(packet);
    }
}

impl ToRawPacket for Vec<u8> {
    fn into_packet_data(mut self) -> RawPacket {
        let data = self.as_mut_ptr();
        let len = self.len();
        let user_data = self.capacity();
        core::mem::forget(self);
        RawPacket {
            data,
            len,
            user_data,
        }
    }

    unsafe fn drop_packet_data(packet: RawPacket) {
        Vec::from_raw_parts(packet.data, packet.len, packet.user_data);
    }
}

impl ToRawPacket for Box<[u8]> {
    fn into_packet_data(self) -> RawPacket {
        let len = self.len();
        let data = Box::into_raw(self).cast();
        RawPacket {
            data,
            len,
            user_data: 0,
        }
    }

    unsafe fn drop_packet_data(packet: RawPacket) {
        let _ = Box::from_raw(core::slice::from_raw_parts_mut(packet.data, packet.len));
    }
}

impl<T: AsRef<[u8]> + Sized> ToRawPacket for Box<T> {
    fn into_packet_data(self) -> RawPacket {
        let slice = (*self).as_ref();
        let data = slice.as_ptr().cast_mut();
        let len = slice.len();
        let user_data = Box::into_raw(self) as usize;
        RawPacket {
            data,
            len,
            user_data,
        }
    }

    unsafe fn drop_packet_data(packet: RawPacket) {
        let _ = Box::<T>::from_raw(packet.user_data as *mut _);
    }
}

/// Describes a raw, Rust-owned packet object.
pub struct RawPacket {
    /// The underlying data.
    pub data: *mut u8,
    /// The length of the data in bytes.
    pub len: usize,
    /// Extra data identifying the object.
    pub user_data: usize,
}
