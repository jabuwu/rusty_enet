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
    pub(crate) data_length: usize,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetChannel {
    pub(crate) outgoing_reliable_sequence_number: u16,
    pub(crate) outgoing_unreliable_sequence_number: u16,
    pub(crate) used_reliable_windows: u16,
    pub(crate) reliable_windows: [u16; 16],
    pub(crate) incoming_reliable_sequence_number: u16,
    pub(crate) incoming_unreliable_sequence_number: u16,
    pub(crate) incoming_reliable_commands: ENetList,
    pub(crate) incoming_unreliable_commands: ENetList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetAcknowledgement {
    pub(crate) acknowledgement_list: ENetListNode,
    pub(crate) sent_time: u32,
    pub(crate) command: ENetProtocol,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetOutgoingCommand {
    pub(crate) outgoing_command_list: ENetListNode,
    pub(crate) reliable_sequence_number: u16,
    pub(crate) unreliable_sequence_number: u16,
    pub(crate) sent_time: u32,
    pub(crate) round_trip_timeout: u32,
    pub(crate) queue_time: u32,
    pub(crate) fragment_offset: u32,
    pub(crate) fragment_length: u16,
    pub(crate) send_attempts: u16,
    pub(crate) command: ENetProtocol,
    pub(crate) packet: *mut ENetPacket,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetIncomingCommand {
    pub(crate) incoming_command_list: ENetListNode,
    pub(crate) reliable_sequence_number: u16,
    pub(crate) unreliable_sequence_number: u16,
    pub(crate) command: ENetProtocol,
    pub(crate) fragment_count: u32,
    pub(crate) fragments_remaining: u32,
    pub(crate) fragments: *mut u32,
    pub(crate) packet: *mut ENetPacket,
}
pub(crate) unsafe fn enet_time_get<S: Socket>(host: *mut ENetHost<S>) -> u32 {
    ((*host).time.assume_init_ref()().as_millis() % u32::MAX as u128) as u32
}
