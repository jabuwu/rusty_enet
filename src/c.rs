use crate::Socket;

mod compress;
mod event;
mod host;
mod list;
mod malloc;
mod packet;
mod peer;
mod protocol;

pub(crate) use compress::*;
pub(crate) use event::*;
pub(crate) use host::*;
pub(crate) use list::*;
pub(crate) use malloc::*;
pub(crate) use packet::*;
pub(crate) use peer::*;
pub(crate) use protocol::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetBuffer {
    pub(crate) data: *mut u8,
    pub(crate) dataLength: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetChannel {
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
pub(crate) struct ENetAcknowledgement {
    pub(crate) acknowledgementList: ENetListNode,
    pub(crate) sentTime: u32,
    pub(crate) command: ENetProtocol,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetOutgoingCommand {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetIncomingCommand {
    pub(crate) incomingCommandList: ENetListNode,
    pub(crate) reliableSequenceNumber: u16,
    pub(crate) unreliableSequenceNumber: u16,
    pub(crate) command: ENetProtocol,
    pub(crate) fragmentCount: u32,
    pub(crate) fragmentsRemaining: u32,
    pub(crate) fragments: *mut u32,
    pub(crate) packet: *mut ENetPacket,
}
pub(crate) unsafe fn enet_time_get<S: Socket>(host: *mut ENetHost<S>) -> u32 {
    ((*host).time.assume_init_ref()().as_millis() % u32::MAX as u128) as u32
}
