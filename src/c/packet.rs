use std::{alloc::Layout, ptr::copy_nonoverlapping};

use crate::{enet_free, enet_malloc, ENET_PACKET_FLAG_NO_ALLOCATE};

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetPacket {
    pub(crate) reference_count: usize,
    pub(crate) flags: u32,
    pub(crate) data: *mut u8,
    pub(crate) data_length: usize,
}
pub(crate) unsafe fn enet_packet_create(
    data: *const u8,
    data_length: usize,
    flags: u32,
) -> *mut ENetPacket {
    let packet: *mut ENetPacket = enet_malloc(Layout::new::<ENetPacket>()).cast();
    if flags & ENET_PACKET_FLAG_NO_ALLOCATE as i32 as u32 != 0 {
        (*packet).data = data.cast_mut();
    } else if data_length <= 0_i32 as usize {
        (*packet).data = std::ptr::null_mut();
    } else {
        (*packet).data = enet_malloc(Layout::array::<u8>(data_length).unwrap());
        if !data.is_null() {
            copy_nonoverlapping(data, (*packet).data, data_length);
        }
    }
    (*packet).reference_count = 0_i32 as usize;
    (*packet).flags = flags;
    (*packet).data_length = data_length;
    packet
}
pub(crate) unsafe fn enet_packet_destroy(packet: *mut ENetPacket) {
    if packet.is_null() {
        return;
    }
    if (*packet).flags & ENET_PACKET_FLAG_NO_ALLOCATE as i32 as u32 == 0
        && !((*packet).data).is_null()
    {
        enet_free((*packet).data, Layout::array::<u8>((*packet).data_length).unwrap());
    }
    enet_free(packet.cast(), Layout::new::<ENetPacket>());
}
