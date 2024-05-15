use std::alloc::{handle_alloc_error, Layout};

pub(crate) unsafe fn enet_malloc(layout: Layout) -> *mut u8 {
    let ptr = unsafe { std::alloc::alloc(layout) };
    if ptr.is_null() {
        handle_alloc_error(layout);
    }
    ptr
}

pub(crate) unsafe fn enet_free(ptr: *mut u8, layout: Layout) {
    std::alloc::dealloc(ptr, layout);
}
