use std::mem::zeroed;

use crate::{
    enet_host_bandwidth_limit, enet_host_broadcast, enet_host_channel_limit,
    enet_host_check_events, enet_host_connect, enet_host_create, enet_host_flush,
    enet_host_service, ENetEvent, ENetHost, ENetPeer, Error, Event, Packet, Peer, PeerID,
    PeerState, Socket, ENET_EVENT_TYPE_CONNECT, ENET_EVENT_TYPE_DISCONNECT,
    ENET_EVENT_TYPE_RECEIVE, ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT,
};

/// Settings for a newly created host, passed into [`Host::create`].
#[derive(Debug, Clone, Copy)]
pub struct HostSettings {
    /// The maximum number of peers that should be allocated for the host.
    pub peer_limit: usize,
    /// The maximum number of channels allowed. Cannot be 0.
    pub channel_limit: usize,
    /// Downstream bandwidth limit of the host in bytes/second, or [`None`] for no limit. Cannot be
    /// set to 0 bytes/second.
    ///
    /// See [`Host::set_bandwidth_limit`] for more info.
    pub incoming_bandwidth_limit: Option<u32>,
    /// Upstream bandwidth limit of the host in bytes/second, or [`None`] for no limit. Cannot be
    /// set to 0 bytes/second.
    ///
    /// See [`Host::set_bandwidth_limit`] for more info.
    pub outgoing_bandwidth_limit: Option<u32>,
}

impl Default for HostSettings {
    fn default() -> Self {
        Self {
            peer_limit: PeerID::MAX,
            channel_limit: ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as usize,
            incoming_bandwidth_limit: None,
            outgoing_bandwidth_limit: None,
        }
    }
}

/// A host for communicating with peers.
pub struct Host<S: Socket> {
    host: *mut ENetHost<S>,
    peers: Vec<Peer<S>>,
}

unsafe impl<S: Socket> Send for Host<S> {}
unsafe impl<S: Socket> Sync for Host<S> {}

impl<S: Socket> Host<S> {
    /// Creates a host for communicating to peers, using the socket provided as a transport layer.
    ///
    /// Supports [`std::net::UdpSocket`] out of the box, but other transport protocols can be
    /// provided by implementing the [`Socket`] trait.
    ///
    /// # Errors
    ///
    /// Returns [`Error::BadParameter`] if one of the host settings was invalid, or
    /// [`Error::FailedToCreateHost`] if the underlying ENet call failed.
    pub fn create(socket: S, settings: HostSettings) -> Result<Host<S>, Error> {
        if settings.channel_limit == 0 {
            return Err(Error::BadParameter);
        }
        if settings.incoming_bandwidth_limit == Some(0) {
            return Err(Error::BadParameter);
        }
        if settings.outgoing_bandwidth_limit == Some(0) {
            return Err(Error::BadParameter);
        }
        unsafe {
            let host = enet_host_create::<S>(
                socket,
                settings.peer_limit,
                settings.channel_limit,
                settings.incoming_bandwidth_limit.unwrap_or(0),
                settings.outgoing_bandwidth_limit.unwrap_or(0),
            );
            let mut peers = vec![];
            for peer_index in 0..(*host).peerCount {
                peers.push(Peer((*host).peers.add(peer_index)));
            }
            if !host.is_null() {
                Ok(Self { host, peers })
            } else {
                Err(Error::FailedToCreateHost)
            }
        }
    }

    /// Initiates a connection to a foreign host, with the specified channel count.
    ///
    /// `data` is an integer value passed to the host upon connection, which can be anything.
    /// Retrieved with [`Event::Connect`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::FailedToConnect`] if the underlying ENet call failed.
    pub fn connect(
        &mut self,
        address: S::PeerAddress,
        channel_count: usize,
        data: u32,
    ) -> Result<&mut Peer<S>, Error> {
        unsafe {
            let peer = enet_host_connect(self.host, address, channel_count, data);
            if !peer.is_null() {
                Ok(self.peer_mut(self.peer_index(peer))?)
            } else {
                Err(Error::FailedToConnect)
            }
        }
    }

    /// Checks for any queued events on the host and dispatches one if available.
    ///
    /// # Errors
    ///
    /// Returns [`Error::FailedToCheckEvents`] if the underlying ENet call failed.
    pub fn check_events(&mut self) -> Result<Option<Event<S>>, Error> {
        unsafe {
            let mut event: ENetEvent<S> = zeroed();
            let result = enet_host_check_events(self.host, &mut event);
            if result > 0 {
                Ok(Some(self.create_event(&event)))
            } else if result < 0 {
                Err(Error::FailedToCheckEvents)
            } else {
                Ok(None)
            }
        }
    }

    /// Checks for events on the host and shuttles packets between the host and its peers.
    ///
    /// Should be called fairly regularly for adequate performance.
    ///
    /// # Errors
    ///
    /// Returns [`Error::FailedToServiceHost`] if the underlying ENet call failed.
    pub fn service(&mut self) -> Result<Option<Event<S>>, Error> {
        unsafe {
            let mut event: ENetEvent<S> = zeroed();
            let result = enet_host_service(self.host, &mut event);
            if result > 0 {
                Ok(Some(self.create_event(&event)))
            } else if result < 0 {
                Err(Error::FailedToServiceHost)
            } else {
                Ok(None)
            }
        }
    }

    /// Sends any queued packets on the host specified to its designated peers.
    pub fn flush(&mut self) {
        unsafe {
            enet_host_flush(self.host);
        }
    }

    /// Get a reference to a single peer.
    ///
    /// # Note
    ///
    /// Acquires the peer object, even if the peer is not in a connected state. See [`Peer::state`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidPeerID`] if the requested peer ID is outside the bounds of peers
    /// allocated for this host.
    pub fn peer(&mut self, peer: PeerID) -> Result<&Peer<S>, Error> {
        self.peers.get(peer.0).ok_or(Error::InvalidPeerID)
    }

    /// Get a mutable reference to a single peer.
    ///
    /// # Note
    ///
    /// Acquires the peer object, even if the peer is not in a connected state. See [`Peer::state`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidPeerID`] if the requested peer ID is outside the bounds of peers
    /// allocated for this host.
    pub fn peer_mut(&mut self, peer: PeerID) -> Result<&mut Peer<S>, Error> {
        self.peers.get_mut(peer.0).ok_or(Error::InvalidPeerID)
    }

    /// Iterate over all peer objects.
    ///
    /// # Note
    ///
    /// Acquires the peer objects, even if the peers are not in a connected state. See
    /// [`Peer::state`]. Use [`Host::connected_peers`] for only connected peers.
    pub fn peers(&mut self) -> impl Iterator<Item = &Peer<S>> {
        self.peers.iter()
    }

    /// Mutably iterate over all peer objects.
    ///
    /// # Note
    ///
    /// Acquires the peer objects, even if the peers are not in a connected state. See
    /// [`Peer::state`]. Use [`Host::connected_peers_mut`] for only connected peers.
    pub fn peers_mut(&mut self) -> impl Iterator<Item = &mut Peer<S>> {
        self.peers.iter_mut()
    }

    /// Iterate over all connected peers.
    pub fn connected_peers(&mut self) -> impl Iterator<Item = &Peer<S>> {
        self.peers
            .iter()
            .filter(|peer| peer.state() == PeerState::Connected)
    }

    /// Mutably iterate over all connected peers.
    pub fn connected_peers_mut(&mut self) -> impl Iterator<Item = &mut Peer<S>> {
        self.peers
            .iter_mut()
            .filter(|peer| peer.state() == PeerState::Connected)
    }

    /// Queues a packet to be sent to all peers.
    pub fn broadcast(&mut self, channel_id: u8, packet: Packet) -> Result<(), Error> {
        unsafe {
            enet_host_broadcast(self.host, channel_id, packet.packet);
            Ok(())
        }
    }

    /// Get the maximum allowed channels for future incoming connections.
    pub fn channel_limit(&self) -> usize {
        unsafe { (*self.host).channelLimit }
    }

    /// Limits the maximum allowed channels of future incoming connections. Cannot be 0.
    ///
    /// # Errors
    ///
    /// Returns [`Error::BadParameter`] if `channel_limit` is `0``.
    pub fn set_channel_limit(&mut self, channel_limit: usize) -> Result<(), Error> {
        if channel_limit == 0 {
            return Err(Error::BadParameter);
        }
        unsafe {
            enet_host_channel_limit(self.host, channel_limit);
        }
        Ok(())
    }

    /// Get the host's current bandwidth limit as (`incoming bandwidth`, `outgoing bandwidth`) in
    /// bytes/second. Returns [`None`] if there is no limit.
    pub fn bandwidth_limit(&self) -> (Option<u32>, Option<u32>) {
        unsafe {
            (
                match (*self.host).incomingBandwidth {
                    0 => None,
                    limit => Some(limit),
                },
                match (*self.host).outgoingBandwidth {
                    0 => None,
                    limit => Some(limit),
                },
            )
        }
    }

    /// Adjusts the bandwidth limits of a host, specified in bytes/second, or [`None`] for no limit.
    ///
    /// The incoming and outgoing bandwidth limits cannot be set to 0 bytes/second.
    ///
    /// ENet will strategically drop packets on specific sides of a connection between hosts to
    /// ensure the host's bandwidth is not overwhelmed. The bandwidth parameters also determine the
    /// window size of a connection which limits the amount of reliable packets that may be in
    /// transit at any given time.
    ///
    /// # Errors
    ///
    /// Returns [`Error::BadParameter`] if `incoming_bandwidth_limit` or `outgoing_bandwidth_limit`
    /// is `Some(0)``.
    pub fn set_bandwidth_limit(
        &mut self,
        incoming_bandwidth_limit: Option<u32>,
        outgoing_bandwidth_limit: Option<u32>,
    ) -> Result<(), Error> {
        if incoming_bandwidth_limit == Some(0) {
            return Err(Error::BadParameter);
        }
        if outgoing_bandwidth_limit == Some(0) {
            return Err(Error::BadParameter);
        }
        unsafe {
            enet_host_bandwidth_limit(
                self.host,
                incoming_bandwidth_limit.unwrap_or(0),
                outgoing_bandwidth_limit.unwrap_or(0),
            );
        }
        Ok(())
    }

    fn create_event<'a>(&'a mut self, event: &ENetEvent<S>) -> Event<'a, S> {
        match event.type_0 {
            ENET_EVENT_TYPE_CONNECT => Event::Connect {
                peer: self.peer_mut(self.peer_index(event.peer)).unwrap(),
                data: event.data,
            },
            ENET_EVENT_TYPE_DISCONNECT => Event::Disconnect {
                peer: self.peer_mut(self.peer_index(event.peer)).unwrap(),
                data: event.data,
            },
            ENET_EVENT_TYPE_RECEIVE => Event::Receive {
                peer: self.peer_mut(self.peer_index(event.peer)).unwrap(),
                channel_id: event.channelID,
                packet: Packet::new_from_ptr(event.packet),
            },
            _ => unreachable!(),
        }
    }

    fn peer_index(&self, peer: *const ENetPeer<S>) -> PeerID {
        PeerID(unsafe { peer.offset_from((*self.host).peers) as usize })
    }

    fn peer_ptr(&self, peer: PeerID) -> Result<*mut ENetPeer<S>, Error> {
        unsafe {
            if peer.0 < (*self.host).peerCount as usize {
                Ok((*self.host).peers.add(peer.0))
            } else {
                Err(Error::InvalidPeerID)
            }
        }
    }
}
