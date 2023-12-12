use crate::{ENetPacket, ENetPeer, Socket};

pub(crate) type ENetEventType = _ENetEventType;
pub(crate) type _ENetEventType = u32;
pub(crate) const ENET_EVENT_TYPE_RECEIVE: _ENetEventType = 3;
pub(crate) const ENET_EVENT_TYPE_DISCONNECT: _ENetEventType = 2;
pub(crate) const ENET_EVENT_TYPE_CONNECT: _ENetEventType = 1;
pub(crate) const ENET_EVENT_TYPE_NONE: _ENetEventType = 0;
pub(crate) type _ENetPacketFlag = u32;
pub(crate) const ENET_PACKET_FLAG_SENT: _ENetPacketFlag = 256;
pub(crate) const ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT: _ENetPacketFlag = 8;
pub(crate) const ENET_PACKET_FLAG_NO_ALLOCATE: _ENetPacketFlag = 4;
pub(crate) const ENET_PACKET_FLAG_UNSEQUENCED: _ENetPacketFlag = 2;
pub(crate) const ENET_PACKET_FLAG_RELIABLE: _ENetPacketFlag = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetEvent<S: Socket> {
    pub(crate) type_0: ENetEventType,
    pub(crate) peer: *mut ENetPeer<S>,
    pub(crate) channelID: u8,
    pub(crate) data: u32,
    pub(crate) packet: *mut ENetPacket,
}
pub(crate) type ENetEvent<S> = _ENetEvent<S>;
