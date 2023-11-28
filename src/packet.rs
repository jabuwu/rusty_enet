use core::slice;
use std::{mem::size_of, ptr::copy_nonoverlapping};

use crate::{c_void, enet_free, enet_malloc};

use bitflags::bitflags;

#[derive(Debug)]
pub struct Packet {
    pub(crate) packet: *mut ENetPacket,
}

unsafe impl Send for Packet {}
unsafe impl Sync for Packet {}

impl Packet {
    pub(crate) fn new(data: &[u8], flags: PacketFlag) -> Self {
        let packet =
            unsafe { enet_packet_create(data.as_ptr() as *const c_void, data.len(), flags) };
        unsafe {
            (*packet).reference_count += 1;
        }
        Self { packet }
    }

    pub fn unreliable(data: &[u8]) -> Self {
        Self::new(data, PacketFlag::empty())
    }

    pub fn unreliable_unsequenced(data: &[u8]) -> Self {
        Self::new(data, PacketFlag::RELIABLE | PacketFlag::UNSEQUENCED)
    }

    pub fn unreliable_fragment(data: &[u8]) -> Self {
        Self::new(data, PacketFlag::UNRELIABLE_FRAGMENT)
    }

    pub fn reliable(data: &[u8]) -> Self {
        Self::new(data, PacketFlag::RELIABLE)
    }

    pub(crate) fn new_internal(packet: *mut ENetPacket) -> Self {
        unsafe {
            (*packet).reference_count += 1;
        }
        Self { packet }
    }

    pub fn data(&self) -> &[u8] {
        unsafe { slice::from_raw_parts((*self.packet).data, (*self.packet).data_length) }
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

#[derive(Copy, Clone)]
pub(crate) struct ENetPacket {
    pub reference_count: usize,
    pub flags: PacketFlag,
    pub data: *const u8,
    pub data_length: usize,
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub(crate) struct PacketFlag: u32 {
        const RELIABLE = 1;
        const UNSEQUENCED = 1 << 2;
        const UNRELIABLE_FRAGMENT = 1 << 4;
        const SENT = 1 << 8;
    }
}

pub(crate) unsafe fn enet_packet_create(
    data: *const c_void,
    data_length: usize,
    flags: PacketFlag,
) -> *mut ENetPacket {
    let packet: *mut ENetPacket = enet_malloc(size_of::<ENetPacket>()) as *mut ENetPacket;
    if packet.is_null() {
        return std::ptr::null_mut();
    }
    if data_length == 0 {
        (*packet).data = std::ptr::null_mut();
    } else {
        (*packet).data = enet_malloc(data_length) as *mut u8;
        if ((*packet).data).is_null() {
            enet_free(packet as *mut c_void);
            return std::ptr::null_mut();
        }
        if !data.is_null() {
            copy_nonoverlapping(data, (*packet).data as *mut c_void, data_length);
        }
    }
    (*packet).reference_count = 0;
    (*packet).flags = flags;
    (*packet).data_length = data_length;
    packet
}
#[no_mangle]
pub(crate) unsafe fn enet_packet_destroy(packet: *mut ENetPacket) {
    if packet.is_null() {
        return;
    }
    if !((*packet).data).is_null() {
        enet_free((*packet).data as *mut c_void);
    }
    enet_free(packet as *mut c_void);
}
