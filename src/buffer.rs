use std::mem::zeroed;

use crate::c_void;

#[derive(Copy, Clone)]
pub(crate) struct ENetBuffer {
    pub data: *mut c_void,
    pub data_length: usize,
}

impl Default for ENetBuffer {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

impl ENetBuffer {
    pub(crate) fn slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.data as *mut u8, self.data_length) }
    }
}
