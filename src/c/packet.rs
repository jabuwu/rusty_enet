use std::ptr::copy_nonoverlapping;

use crate::{enet_free, enet_malloc, ENET_PACKET_FLAG_NO_ALLOCATE};

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetPacket {
    pub(crate) referenceCount: usize,
    pub(crate) flags: u32,
    pub(crate) data: *mut u8,
    pub(crate) dataLength: usize,
}
pub(crate) unsafe fn enet_packet_create(
    data: *const u8,
    dataLength: usize,
    flags: u32,
) -> *mut ENetPacket {
    let packet: *mut ENetPacket =
        enet_malloc(::core::mem::size_of::<ENetPacket>()) as *mut ENetPacket;
    if packet.is_null() {
        return std::ptr::null_mut();
    }
    if flags & ENET_PACKET_FLAG_NO_ALLOCATE as i32 as u32 != 0 {
        (*packet).data = data as *mut u8;
    } else if dataLength <= 0_i32 as usize {
        (*packet).data = std::ptr::null_mut();
    } else {
        (*packet).data = enet_malloc(dataLength);
        if ((*packet).data).is_null() {
            enet_free(packet as *mut u8);
            return std::ptr::null_mut();
        }
        if !data.is_null() {
            copy_nonoverlapping(data, (*packet).data, dataLength);
        }
    }
    (*packet).referenceCount = 0_i32 as usize;
    (*packet).flags = flags;
    (*packet).dataLength = dataLength;
    packet
}
pub(crate) unsafe fn enet_packet_destroy(packet: *mut ENetPacket) {
    if packet.is_null() {
        return;
    }
    if (*packet).flags & ENET_PACKET_FLAG_NO_ALLOCATE as i32 as u32 == 0
        && !((*packet).data).is_null()
    {
        enet_free((*packet).data);
    }
    enet_free(packet as *mut u8);
}
