use std::mem::zeroed;

use crate::{
    enet_host_bandwidth_limit, enet_host_broadcast, enet_host_channel_limit,
    enet_host_check_events, enet_host_connect, enet_host_create, enet_host_flush,
    enet_host_service, enet_peer_disconnect, enet_peer_disconnect_later, enet_peer_disconnect_now,
    enet_peer_ping, enet_peer_ping_interval, enet_peer_send, enet_peer_throttle_configure,
    enet_peer_timeout, ENetEvent, ENetHost, ENetPeer, Event, Packet, Socket,
    ENET_EVENT_TYPE_CONNECT, ENET_EVENT_TYPE_DISCONNECT, ENET_EVENT_TYPE_RECEIVE,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PeerID(pub usize);

pub struct Host<S: Socket> {
    host: *mut ENetHost<S>,
}

unsafe impl<S: Socket> Send for Host<S> {}
unsafe impl<S: Socket> Sync for Host<S> {}

impl<S: Socket> Host<S> {
    pub fn create(
        address: S::BindAddress,
        peer_count: usize,
        channel_limit: usize,
        incoming_bandwidth: u32,
        outgoing_bandwidth: u32,
    ) -> Result<Host<S>, crate::Error> {
        unsafe {
            let host = enet_host_create::<S>(
                address,
                peer_count,
                channel_limit,
                incoming_bandwidth,
                outgoing_bandwidth,
            );
            if !host.is_null() {
                Ok(Self { host })
            } else {
                Err(crate::Error::Unknown)
            }
        }
    }

    pub fn connect(
        &mut self,
        address: S::PeerAddress,
        mut channel_count: usize,
        data: u32,
    ) -> Result<PeerID, crate::Error> {
        unsafe {
            let peer = enet_host_connect(self.host, address, channel_count, data);
            if !peer.is_null() {
                Ok(self.peer_index(peer))
            } else {
                Err(crate::Error::Unknown)
            }
        }
    }

    pub fn check_events(&mut self) -> Result<Option<Event>, crate::Error> {
        unsafe {
            let mut event: ENetEvent<S> = zeroed();
            let result = enet_host_check_events(self.host, &mut event);
            if result > 0 {
                Ok(Some(self.create_event(&event)))
            } else if result < 0 {
                Err(crate::Error::Unknown)
            } else {
                Ok(None)
            }
        }
    }

    pub fn service(&mut self) -> Result<Option<Event>, crate::Error> {
        unsafe {
            let mut event: ENetEvent<S> = zeroed();
            let result = enet_host_service(self.host, &mut event);
            if result > 0 {
                Ok(Some(self.create_event(&event)))
            } else if result < 0 {
                Err(crate::Error::Unknown)
            } else {
                Ok(None)
            }
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            enet_host_flush(self.host);
        }
    }

    pub fn channel_limit(&self) -> usize {
        unsafe { (*self.host).channelLimit }
    }

    pub fn set_channel_limit(&mut self, channel_limit: usize) {
        unsafe {
            enet_host_channel_limit(self.host, channel_limit);
        }
    }

    pub fn bandwidth_limit(&self) -> (u32, u32) {
        unsafe {
            (
                (*self.host).incomingBandwidth,
                (*self.host).outgoingBandwidth,
            )
        }
    }

    pub fn set_bandwidth_limit(&mut self, incoming_bandwidth: u32, outgoing_bandwidth: u32) {
        unsafe {
            enet_host_bandwidth_limit(self.host, incoming_bandwidth, outgoing_bandwidth);
        }
    }

    pub fn ping(&mut self, peer: PeerID) -> Result<(), crate::Error> {
        unsafe {
            enet_peer_ping(self.peer_ptr(peer)?);
            Ok(())
        }
    }

    pub fn send(
        &mut self,
        peer: PeerID,
        channel_id: u8,
        packet: Packet,
    ) -> Result<(), crate::Error> {
        unsafe {
            enet_peer_send(self.peer_ptr(peer)?, channel_id, packet.packet);
            Ok(())
        }
    }

    pub fn broadcast(&mut self, channel_id: u8, packet: Packet) -> Result<(), crate::Error> {
        unsafe {
            enet_host_broadcast(self.host, channel_id, packet.packet);
            Ok(())
        }
    }

    pub fn set_timeout(
        &mut self,
        peer: PeerID,
        limit: u32,
        minimum: u32,
        maximum: u32,
    ) -> Result<(), crate::Error> {
        unsafe {
            enet_peer_timeout(self.peer_ptr(peer)?, limit, minimum, maximum);
            Ok(())
        }
    }

    pub fn set_ping_interval(
        &mut self,
        peer: PeerID,
        ping_interval: u32,
    ) -> Result<(), crate::Error> {
        unsafe {
            enet_peer_ping_interval(self.peer_ptr(peer)?, ping_interval);
            Ok(())
        }
    }

    pub fn set_throttle(
        &mut self,
        peer: PeerID,
        interval: u32,
        acceleration: u32,
        deceleration: u32,
    ) -> Result<(), crate::Error> {
        unsafe {
            enet_peer_throttle_configure(
                self.peer_ptr(peer)?,
                interval,
                acceleration,
                deceleration,
            );
            Ok(())
        }
    }

    pub fn disconnect(&mut self, peer: PeerID, data: u32) -> Result<(), crate::Error> {
        unsafe { enet_peer_disconnect(self.peer_ptr(peer)?, data) }
        Ok(())
    }

    pub fn disconnect_now(&mut self, peer: PeerID, data: u32) -> Result<(), crate::Error> {
        unsafe { enet_peer_disconnect_now(self.peer_ptr(peer)?, data) }
        Ok(())
    }

    pub fn disconnect_later(&mut self, peer: PeerID, data: u32) -> Result<(), crate::Error> {
        unsafe { enet_peer_disconnect_later(self.peer_ptr(peer)?, data) }
        Ok(())
    }

    fn create_event(&mut self, event: &ENetEvent<S>) -> Event {
        match event.type_0 {
            ENET_EVENT_TYPE_CONNECT => Event::Connect {
                peer: self.peer_index(event.peer),
                data: event.data,
            },
            ENET_EVENT_TYPE_DISCONNECT => Event::Disconnect {
                peer: self.peer_index(event.peer),
                data: event.data,
            },
            ENET_EVENT_TYPE_RECEIVE => Event::Receive {
                peer: self.peer_index(event.peer),
                channel_id: event.channelID,
                packet: Packet::new_from_ptr(event.packet),
            },
            _ => unreachable!(),
        }
    }

    fn peer_index(&mut self, peer: *const ENetPeer<S>) -> PeerID {
        PeerID(unsafe { peer.offset_from((*self.host).peers) as usize })
    }

    fn peer_ptr(&mut self, peer: PeerID) -> Result<*mut ENetPeer<S>, crate::Error> {
        unsafe {
            if peer.0 < (*self.host).peerCount as usize {
                Ok((*self.host).peers.add(peer.0))
            } else {
                Err(crate::Error::Unknown)
            }
        }
    }
}
