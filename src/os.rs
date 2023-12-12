use std::{
    alloc::Layout,
    collections::HashMap,
    ptr::copy_nonoverlapping,
    sync::{Arc, Mutex, Once},
};

#[repr(u8)]
pub(crate) enum c_void {
    __variant1,
    __variant2,
}

#[derive(Default)]
struct Allocator {
    allocations: HashMap<*const c_void, Layout>,
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

    fn malloc(&mut self, size: usize) -> *mut c_void {
        if size > 0 {
            let layout = std::alloc::Layout::array::<u8>(size)
                .unwrap()
                .align_to(8)
                .unwrap();
            let ptr = unsafe { std::alloc::alloc(layout) };
            self.allocations.insert(ptr as *const c_void, layout);
            ptr.cast::<c_void>()
        } else {
            std::ptr::null_mut()
        }
    }

    unsafe fn free(&mut self, ptr: *const c_void) {
        if !ptr.is_null() {
            let layout = self.allocations.remove(&ptr).unwrap();
            unsafe { std::alloc::dealloc(ptr as *mut u8, layout) };
        }
    }
}

pub(crate) unsafe fn enet_malloc(size: usize) -> *mut c_void {
    let singleton = Allocator::singleton();
    let mut allocator = singleton.lock().unwrap();
    allocator.malloc(size)
}

pub(crate) unsafe fn enet_free(ptr: *mut c_void) {
    if !ptr.is_null() && ptr as usize != 1 {
        let singleton = Allocator::singleton();
        let mut allocator = singleton.lock().unwrap();
        allocator.free(ptr);
    }
}

pub(crate) unsafe fn _enet_abort() -> ! {
    std::process::abort()
}

pub(crate) unsafe fn _enet_memset(s: *mut c_void, c: i32, n: usize) -> *mut c_void {
    for offset in 0..n {
        (*(s.cast::<u8>()).add(offset)) = c as u8;
    }
    s
}

pub(crate) unsafe fn _enet_memcpy(
    destination: *mut c_void,
    source: *const c_void,
    num: usize,
) -> *mut c_void {
    copy_nonoverlapping(source, destination, num);
    destination
}
