use crate::{
    os::{_enet_abort, _enet_free, _enet_malloc, c_void},
    Socket,
};

mod compress;
mod crc32;
mod event;
mod host;
mod list;
mod packet;
mod peer;
mod protocol;

pub(crate) use compress::*;
pub(crate) use crc32::*;
pub(crate) use event::*;
pub(crate) use host::*;
pub(crate) use list::*;
pub(crate) use packet::*;
pub(crate) use peer::*;
pub(crate) use protocol::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetBuffer {
    pub(crate) data: *mut c_void,
    pub(crate) dataLength: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetCallbacks {
    pub(crate) malloc: Option<unsafe extern "C" fn(usize) -> *mut c_void>,
    pub(crate) free: Option<unsafe extern "C" fn(*mut c_void) -> ()>,
    pub(crate) no_memory: Option<unsafe extern "C" fn() -> ()>,
}
pub(crate) type ENetCallbacks = _ENetCallbacks;
pub(crate) type ENetChannel = _ENetChannel;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetChannel {
    pub(crate) outgoingReliableSequenceNumber: u16,
    pub(crate) outgoingUnreliableSequenceNumber: u16,
    pub(crate) usedReliableWindows: u16,
    pub(crate) reliableWindows: [u16; 16],
    pub(crate) incomingReliableSequenceNumber: u16,
    pub(crate) incomingUnreliableSequenceNumber: u16,
    pub(crate) incomingReliableCommands: ENetList,
    pub(crate) incomingUnreliableCommands: ENetList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetAcknowledgement {
    pub(crate) acknowledgementList: ENetListNode,
    pub(crate) sentTime: u32,
    pub(crate) command: ENetProtocol,
}
pub(crate) type ENetAcknowledgement = _ENetAcknowledgement;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetOutgoingCommand {
    pub(crate) outgoingCommandList: ENetListNode,
    pub(crate) reliableSequenceNumber: u16,
    pub(crate) unreliableSequenceNumber: u16,
    pub(crate) sentTime: u32,
    pub(crate) roundTripTimeout: u32,
    pub(crate) queueTime: u32,
    pub(crate) fragmentOffset: u32,
    pub(crate) fragmentLength: u16,
    pub(crate) sendAttempts: u16,
    pub(crate) command: ENetProtocol,
    pub(crate) packet: *mut ENetPacket,
}
pub(crate) type ENetOutgoingCommand = _ENetOutgoingCommand;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetIncomingCommand {
    pub(crate) incomingCommandList: ENetListNode,
    pub(crate) reliableSequenceNumber: u16,
    pub(crate) unreliableSequenceNumber: u16,
    pub(crate) command: ENetProtocol,
    pub(crate) fragmentCount: u32,
    pub(crate) fragmentsRemaining: u32,
    pub(crate) fragments: *mut u32,
    pub(crate) packet: *mut ENetPacket,
}
pub(crate) type ENetIncomingCommand = _ENetIncomingCommand;
static mut CALLBACKS: ENetCallbacks = unsafe {
    _ENetCallbacks {
        malloc: Some(_enet_malloc as unsafe extern "C" fn(usize) -> *mut c_void),
        free: Some(_enet_free as unsafe extern "C" fn(*mut c_void) -> ()),
        no_memory: ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> !>,
            Option<unsafe extern "C" fn() -> ()>,
        >(Some(_enet_abort as unsafe extern "C" fn() -> !)),
    }
};
#[no_mangle]
pub(crate) unsafe extern "C" fn enet_malloc(size: usize) -> *mut c_void {
    let memory: *mut c_void = (CALLBACKS.malloc).expect("non-null function pointer")(size);
    if memory.is_null() {
        (CALLBACKS.no_memory).expect("non-null function pointer")();
    }
    memory
}
#[no_mangle]
pub(crate) unsafe extern "C" fn enet_free(memory: *mut c_void) {
    (CALLBACKS.free).expect("non-null function pointer")(memory);
}
pub(crate) unsafe fn enet_time_get<S: Socket>(host: *mut ENetHost<S>) -> u32 {
    ((*host).time.assume_init_ref()().as_millis() % u32::MAX as u128) as u32
}
