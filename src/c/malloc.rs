use core::alloc::Layout;

#[cfg(not(feature = "std"))]
use alloc::alloc::{alloc, dealloc, handle_alloc_error};
#[cfg(feature = "std")]
use std::alloc::{alloc, dealloc, handle_alloc_error};

pub(crate) unsafe fn enet_malloc(layout: Layout) -> *mut u8 {
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        handle_alloc_error(layout);
    }
    ptr
}

pub(crate) unsafe fn enet_free(ptr: *mut u8, layout: Layout) {
    dealloc(ptr, layout);
}
