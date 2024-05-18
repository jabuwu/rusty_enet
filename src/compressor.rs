use crate::{
    enet_range_coder_compress, enet_range_coder_create, enet_range_coder_decompress,
    enet_range_coder_destroy, ENetBuffer, ENetRangeCoder,
};

/// An interface for compressing ENet packets.
pub trait Compressor {
    /// Compress the incoming buffers.
    fn compress(&mut self, in_buffers: Vec<&[u8]>, in_limit: usize, out: &mut [u8]) -> usize;
    /// Decompress the buffer.
    fn decompress(&mut self, in_data: &[u8], out: &mut [u8]) -> usize;
}

/// The built-in range coder compression provided by ENet.
pub struct RangeCoder(*mut ENetRangeCoder);

unsafe impl Send for RangeCoder {}
unsafe impl Sync for RangeCoder {}

impl RangeCoder {
    /// Create a new range coder compressor.
    #[must_use]
    pub fn new() -> Self {
        Self(unsafe { enet_range_coder_create().cast() })
    }
}

impl Default for RangeCoder {
    fn default() -> Self {
        RangeCoder::new()
    }
}

impl Compressor for RangeCoder {
    fn compress(&mut self, in_buffers: Vec<&[u8]>, in_limit: usize, out: &mut [u8]) -> usize {
        unsafe {
            let mut buffers = vec![];
            for in_buffer in in_buffers {
                buffers.push(ENetBuffer {
                    data: in_buffer.as_ptr().cast_mut(),
                    data_length: in_buffer.len(),
                });
            }
            enet_range_coder_compress(
                self.0.cast(),
                buffers.as_ptr(),
                buffers.len(),
                in_limit,
                out.as_mut_ptr(),
                out.len(),
            )
        }
    }

    fn decompress(&mut self, in_data: &[u8], out: &mut [u8]) -> usize {
        unsafe {
            enet_range_coder_decompress(
                self.0.cast(),
                in_data.as_ptr(),
                in_data.len(),
                out.as_mut_ptr(),
                out.len(),
            )
        }
    }
}

impl Drop for RangeCoder {
    fn drop(&mut self) {
        unsafe { enet_range_coder_destroy(self.0.cast::<u8>()) };
    }
}
