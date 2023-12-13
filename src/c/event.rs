use crate::{ENetPacket, ENetPeer, Socket};

pub(crate) type ENetEventType = u32;
pub(crate) const ENET_EVENT_TYPE_RECEIVE: ENetEventType = 3;
pub(crate) const ENET_EVENT_TYPE_DISCONNECT: ENetEventType = 2;
pub(crate) const ENET_EVENT_TYPE_CONNECT: ENetEventType = 1;
pub(crate) const ENET_EVENT_TYPE_NONE: ENetEventType = 0;
pub(crate) type ENetPacketFlag = u32;
pub(crate) const ENET_PACKET_FLAG_SENT: ENetPacketFlag = 256;
pub(crate) const ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT: ENetPacketFlag = 8;
pub(crate) const ENET_PACKET_FLAG_NO_ALLOCATE: ENetPacketFlag = 4;
pub(crate) const ENET_PACKET_FLAG_UNSEQUENCED: ENetPacketFlag = 2;
pub(crate) const ENET_PACKET_FLAG_RELIABLE: ENetPacketFlag = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetEvent<S: Socket> {
    pub(crate) type_0: ENetEventType,
    pub(crate) peer: *mut ENetPeer<S>,
    pub(crate) channel_id: u8,
    pub(crate) data: u32,
    pub(crate) packet: *mut ENetPacket,
}
