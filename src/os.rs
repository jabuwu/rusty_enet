use std::{
    alloc::Layout,
    collections::HashMap,
    ptr::copy_nonoverlapping,
    sync::{Arc, Mutex, Once},
};

// socket related imports
extern "C" {
    pub fn socket(__domain: c_int, __type: c_int, __protocol: c_int) -> c_int;
    pub fn bind(__fd: c_int, __addr: *const sockaddr, __len: socklen_t) -> c_int;
    pub fn getsockname(__fd: c_int, __addr: *mut sockaddr, __len: *mut socklen_t) -> c_int;
    pub fn sendmsg(__fd: c_int, __message: *const msghdr, __flags: c_int) -> ssize_t;
    pub fn recvmsg(__fd: c_int, __message: *mut msghdr, __flags: c_int) -> ssize_t;
    pub fn setsockopt(
        __fd: c_int,
        __level: c_int,
        __optname: c_int,
        __optval: *const c_void,
        __optlen: socklen_t,
    ) -> c_int;
    pub fn inet_pton(__af: c_int, __cp: *const c_char, __buf: *mut c_void) -> c_int;
    pub fn close(__fd: c_int) -> c_int;
    pub fn freeaddrinfo(__ai: *mut addrinfo);
    pub fn fcntl(__fd: c_int, __cmd: c_int, _: ...) -> c_int;
    pub fn __errno_location() -> *mut c_int;
}
extern "C" {
    pub fn time(__timer: *mut time_t) -> time_t;
    pub fn gettimeofday(__tv: *mut timeval, __tz: *mut c_void) -> c_int;
}
pub type c_void = libc::c_void;
pub type c_char = libc::c_char;
pub type c_uchar = libc::c_uchar;
pub type c_short = libc::c_short;
pub type c_ushort = libc::c_ushort;
pub type c_int = libc::c_int;
pub type c_uint = libc::c_uint;
pub type c_long = libc::c_long;
pub type c_ulong = c_long;
pub type size_t = c_ulong;
pub type __uint16_t = c_ushort;
pub type __uint32_t = c_uint;
pub type __time_t = c_long;
pub type __suseconds_t = c_long;
pub type __ssize_t = c_long;
pub type __socklen_t = c_uint;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}
pub type socklen_t = __socklen_t;
pub type __socket_type = c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [c_char; 14],
}
pub type C2RustUnnamed = c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
pub const MSG_BATCH: C2RustUnnamed = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut c_void,
    pub msg_controllen: size_t,
    pub msg_flags: c_int,
}
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type C2RustUnnamed_0 = c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_0 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: c_int,
    pub events: c_short,
    pub revents: c_short,
}
pub type nfds_t = c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: c_int,
    pub ai_family: c_int,
    pub ai_socktype: c_int,
    pub ai_protocol: c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut c_char,
    pub ai_next: *mut addrinfo,
}

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

pub(crate) unsafe extern "C" fn _enet_malloc(size: c_ulong) -> *mut c_void {
    let singleton = Allocator::singleton();
    let mut allocator = singleton.lock().unwrap();
    allocator.malloc(size as usize)
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

pub(crate) unsafe extern "C" fn _enet_memset(s: *mut c_void, c: c_int, n: c_ulong) -> *mut c_void {
    for offset in 0..n {
        (*(s.cast::<u8>()).add(offset as usize)) = c as u8;
    }
    s
}

pub(crate) unsafe extern "C" fn _enet_memcpy(
    destination: *mut c_void,
    source: *const c_void,
    num: c_ulong,
) -> *mut c_void {
    copy_nonoverlapping(source, destination, num as usize);
    destination
}
