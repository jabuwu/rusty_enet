use crate::{enet_crc32, os::c_void, ENetBuffer};

/// ENet implementation of CRC32 checksum, for use with
/// [`HostSettings::checksum_fn`](`crate::HostSettings::checksum_fn`).
pub fn crc32(in_buffers: Vec<&[u8]>) -> u32 {
    let mut buffers = vec![];
    for in_buffer in in_buffers {
        buffers.push(ENetBuffer {
            data: in_buffer.as_ptr() as *mut c_void,
            dataLength: in_buffer.len(),
        });
    }
    unsafe { enet_crc32(buffers.as_ptr(), buffers.len()) }
}
