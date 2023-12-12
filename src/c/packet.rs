use crate::{_enet_memcpy, enet_free, enet_malloc, os::c_void, ENET_PACKET_FLAG_NO_ALLOCATE};

pub(crate) type ENetPacket = _ENetPacket;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetPacket {
    pub(crate) referenceCount: usize,
    pub(crate) flags: u32,
    pub(crate) data: *mut u8,
    pub(crate) dataLength: usize,
    pub(crate) freeCallback: ENetPacketFreeCallback,
    pub(crate) userData: *mut c_void,
}
pub(crate) type ENetPacketFreeCallback = Option<unsafe extern "C" fn(*mut _ENetPacket) -> ()>;
pub(crate) unsafe fn enet_packet_create(
    data: *const c_void,
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
        (*packet).data = enet_malloc(dataLength) as *mut u8;
        if ((*packet).data).is_null() {
            enet_free(packet as *mut c_void);
            return std::ptr::null_mut();
        }
        if !data.is_null() {
            _enet_memcpy((*packet).data as *mut c_void, data, dataLength);
        }
    }
    (*packet).referenceCount = 0_i32 as usize;
    (*packet).flags = flags;
    (*packet).dataLength = dataLength;
    (*packet).freeCallback = None;
    (*packet).userData = std::ptr::null_mut();
    packet
}
pub(crate) unsafe fn enet_packet_destroy(packet: *mut ENetPacket) {
    if packet.is_null() {
        return;
    }
    if ((*packet).freeCallback).is_some() {
        ((*packet).freeCallback).expect("non-null function pointer")(packet);
    }
    if (*packet).flags & ENET_PACKET_FLAG_NO_ALLOCATE as i32 as u32 == 0
        && !((*packet).data).is_null()
    {
        enet_free((*packet).data as *mut c_void);
    }
    enet_free(packet as *mut c_void);
}
