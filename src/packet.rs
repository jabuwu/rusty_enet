use core::slice;

use crate::{
    c_void, ENetPacket, _ENetPacketFlag, enet_packet_create, enet_packet_destroy, size_t,
    ENET_PACKET_FLAG_RELIABLE, ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT, ENET_PACKET_FLAG_UNSEQUENCED,
};

#[derive(Debug)]
pub struct Packet {
    pub(crate) packet: *mut ENetPacket,
}

unsafe impl Send for Packet {}
unsafe impl Sync for Packet {}

impl Packet {
    pub(crate) fn new(data: &[u8], flags: _ENetPacketFlag) -> Self {
        let packet = unsafe {
            enet_packet_create(data.as_ptr() as *const c_void, data.len() as size_t, flags)
        };
        unsafe {
            (*packet).referenceCount += 1;
        }
        Self { packet }
    }

    pub fn unreliable(data: &[u8]) -> Self {
        Self::new(data, 0)
    }

    pub fn unreliable_unsequenced(data: &[u8]) -> Self {
        Self::new(
            data,
            ENET_PACKET_FLAG_RELIABLE | ENET_PACKET_FLAG_UNSEQUENCED,
        )
    }

    pub fn unreliable_fragment(data: &[u8]) -> Self {
        Self::new(data, ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT)
    }

    pub fn reliable(data: &[u8]) -> Self {
        Self::new(data, ENET_PACKET_FLAG_RELIABLE)
    }

    pub(crate) fn new_from_ptr(packet: *mut ENetPacket) -> Self {
        unsafe {
            (*packet).referenceCount += 1;
        }
        Self { packet }
    }

    pub fn data(&self) -> &[u8] {
        unsafe { slice::from_raw_parts((*self.packet).data, (*self.packet).dataLength as usize) }
    }
}

impl Clone for Packet {
    fn clone(&self) -> Self {
        unsafe {
            (*self.packet).referenceCount += 1;
        }
        Self {
            packet: self.packet,
        }
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        unsafe {
            (*self.packet).referenceCount -= 1;
            if (*self.packet).referenceCount == 0 {
                enet_packet_destroy(self.packet);
            }
        }
    }
}
