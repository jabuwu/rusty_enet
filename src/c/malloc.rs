use std::{
    alloc::Layout,
    collections::HashMap,
    sync::{Arc, Mutex, Once},
};

#[derive(Default)]
struct Allocator {
    allocations: HashMap<*const u8, Layout>,
}

impl Allocator {
    fn singleton() -> Arc<Mutex<Allocator>> {
        static START: Once = Once::new();
        static mut INSTANCE: Option<Arc<Mutex<Allocator>>> = None;
        START.call_once(|| unsafe {
            INSTANCE = Some(Arc::new(Mutex::new(Allocator::default())));
        });
        unsafe {
            let singleton = INSTANCE.as_ref().unwrap();
            singleton.clone()
        }
    }

    fn malloc(&mut self, size: usize) -> *mut u8 {
        if size > 0 {
            let layout = std::alloc::Layout::array::<u8>(size)
                .unwrap()
                .align_to(8)
                .unwrap();
            let ptr = unsafe { std::alloc::alloc(layout) };
            self.allocations.insert(ptr.cast_const(), layout);
            ptr.cast::<u8>()
        } else {
            std::ptr::null_mut()
        }
    }

    unsafe fn free(&mut self, ptr: *const u8) {
        if !ptr.is_null() {
            let layout = self.allocations.remove(&ptr).unwrap();
            unsafe { std::alloc::dealloc(ptr.cast_mut(), layout) };
        }
    }
}

pub(crate) unsafe fn enet_malloc(size: usize) -> *mut u8 {
    let singleton = Allocator::singleton();
    let mut allocator = singleton.lock().unwrap();
    allocator.malloc(size)
}

pub(crate) unsafe fn enet_free(ptr: *mut u8) {
    if !ptr.is_null() && ptr as usize != 1 {
        let singleton = Allocator::singleton();
        let mut allocator = singleton.lock().unwrap();
        allocator.free(ptr);
    }
}
