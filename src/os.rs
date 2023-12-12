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

pub(crate) type c_uchar = u8;
pub(crate) type c_ushort = u16;
pub(crate) type c_int = i32;
pub(crate) type c_uint = u32;
pub(crate) type c_long = i64;
pub(crate) type c_ulong = u64;

pub(crate) type size_t = usize;
pub(crate) type __uint16_t = c_ushort;
pub(crate) type __uint32_t = c_uint;
pub(crate) type __time_t = c_long;
pub(crate) type __suseconds_t = c_long;
pub(crate) type __ssize_t = c_long;
pub(crate) type __socklen_t = c_uint;
pub(crate) type uint16_t = __uint16_t;
pub(crate) type uint32_t = __uint32_t;

pub(crate) fn ntohl(__netlong: uint32_t) -> uint32_t {
    uint32_t::from_be(__netlong)
}
pub(crate) fn ntohs(__netshort: uint16_t) -> uint16_t {
    uint16_t::from_be(__netshort)
}
pub(crate) fn htonl(__hostlong: uint32_t) -> uint32_t {
    __hostlong.to_be()
}
pub(crate) fn htons(__hostshort: uint16_t) -> uint16_t {
    __hostshort.to_be()
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

pub(crate) unsafe extern "C" fn _enet_malloc(size: size_t) -> *mut c_void {
    let singleton = Allocator::singleton();
    let mut allocator = singleton.lock().unwrap();
    allocator.malloc(size)
}

pub(crate) unsafe extern "C" fn _enet_free(ptr: *mut c_void) {
    if !ptr.is_null() && ptr as usize != 1 {
        let singleton = Allocator::singleton();
        let mut allocator = singleton.lock().unwrap();
        allocator.free(ptr);
    }
}

pub(crate) unsafe extern "C" fn _enet_abort() -> ! {
    std::process::abort()
}

pub(crate) unsafe extern "C" fn _enet_memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void {
    for offset in 0..n {
        (*(s.cast::<u8>()).add(offset)) = c as u8;
    }
    s
}

pub(crate) unsafe extern "C" fn _enet_memcpy(
    destination: *mut c_void,
    source: *const c_void,
    num: size_t,
) -> *mut c_void {
    copy_nonoverlapping(source, destination, num);
    destination
}
