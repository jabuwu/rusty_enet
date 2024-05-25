use core::{fmt::Debug, mem::zeroed, time::Duration};

use crate::{Box, Vec};

use crate::{
    consts::{
        PROTOCOL_MAXIMUM_CHANNEL_COUNT, PROTOCOL_MAXIMUM_MTU, PROTOCOL_MAXIMUM_PEER_ID,
        PROTOCOL_MINIMUM_MTU,
    },
    enet_host_bandwidth_limit, enet_host_broadcast, enet_host_channel_limit,
    enet_host_check_events, enet_host_compress, enet_host_connect, enet_host_create,
    enet_host_destroy, enet_host_flush, enet_host_service,
    error::{BadParameter, HostNewError, NoAvailablePeers},
    time_since_epoch, Compressor, ENetEvent, ENetHost, ENetPeer, Event, Packet, Peer, PeerID,
    PeerState, Socket, ENET_EVENT_TYPE_CONNECT, ENET_EVENT_TYPE_DISCONNECT,
    ENET_EVENT_TYPE_RECEIVE,
};

/// Settings for a newly created host, passed into [`Host::new`].
#[allow(clippy::type_complexity)]
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
    /// The compressor to use when sending and receiving packets, or [`None`] for no compression.
    pub compressor: Option<Box<dyn Compressor>>,
    /// The checksum function to use when sending and receiving packets, or [`None`] for no
    /// checksum.
    pub checksum: Option<Box<dyn Fn(&[&[u8]]) -> u32>>,
    /// A custom time function to use, or [`None`] to use the default one. Should return an
    /// an accurate, incrementally increasing [`Duration`]. Defaults to [`time_since_epoch`].
    pub time: Box<dyn Fn() -> Duration>,
    /// Seed the host with a specific random seed, or set to [`None`] to use a random seed.
    pub seed: Option<u32>,
}

impl Default for HostSettings {
    fn default() -> Self {
        Self {
            peer_limit: PeerID::MAX,
            channel_limit: PROTOCOL_MAXIMUM_CHANNEL_COUNT as usize,
            incoming_bandwidth_limit: None,
            outgoing_bandwidth_limit: None,
            compressor: None,
            checksum: None,
            time: Box::new(time_since_epoch),
            seed: None,
        }
    }
}

/// A host for communicating with peers.
///
/// Requires a [`Socket`] implementation.
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
    /// Returns [`HostNewError::BadParameter`] if one of the host settings was invalid:
    /// - If [`HostSettings::channel_limit`] is equal to `0`.
    /// - If [`HostSettings::incoming_bandwidth_limit`] is equal to `Some(0)`.
    /// - If [`HostSettings::outgoing_bandwidth_limit`] is equal to `Some(0)`.
    /// - If [`HostSettings::peer_limit`] is equal to `0` or greater than
    /// [`PROTOCOL_MAXIMUM_PEER_ID`].
    ///
    /// Returns [`HostNewError::FailedToInitializeSocket`] if the call to [`Socket::init`] fails.
    pub fn new(socket: S, settings: HostSettings) -> Result<Host<S>, HostNewError<S>> {
        if settings.channel_limit == 0 {
            return Err(HostNewError::BadParameter(BadParameter {
                method: "Host::new",
                parameter: "settings.channel_limit",
            }));
        }
        if settings.incoming_bandwidth_limit == Some(0) {
            return Err(HostNewError::BadParameter(BadParameter {
                method: "Host::new",
                parameter: "settings.incoming_bandwidth_limit",
            }));
        }
        if settings.outgoing_bandwidth_limit == Some(0) {
            return Err(HostNewError::BadParameter(BadParameter {
                method: "Host::new",
                parameter: "settings.outgoing_bandwidth_limit",
            }));
        }
        if settings.peer_limit == 0 || settings.peer_limit > PROTOCOL_MAXIMUM_PEER_ID as usize {
            return Err(HostNewError::BadParameter(BadParameter {
                method: "Host::new",
                parameter: "settings.peer_limit",
            }));
        }
        unsafe {
            let host = enet_host_create::<S>(
                socket,
                settings.peer_limit,
                settings.channel_limit,
                settings.incoming_bandwidth_limit.unwrap_or(0),
                settings.outgoing_bandwidth_limit.unwrap_or(0),
                settings.time,
                settings.seed,
            )
            .map_err(|err| HostNewError::FailedToInitializeSocket(err))?;
            let mut peers = Vec::new();
            peers.reserve_exact((*host).peer_count);
            for peer_index in 0..(*host).peer_count {
                peers.push(Peer((*host).peers.add(peer_index)));
            }
            if let Some(compressor) = settings.compressor {
                enet_host_compress(host, Some(compressor));
            }
            if let Some(checksum) = settings.checksum {
                *(*host).checksum.assume_init_mut() = Some(checksum);
            }
            Ok(Self { host, peers })
        }
    }

    /// Initiates a connection to a foreign host, with the specified channel count.
    ///
    /// `data` is an integer value passed to the host upon connection, which can be anything.
    /// Retrieved with [`Event::Connect`].
    ///
    /// # Errors
    ///
    /// Returns [`NoAvailablePeers`] if all peer slots have been filled.
    pub fn connect(
        &mut self,
        address: S::Address,
        channel_count: usize,
        data: u32,
    ) -> Result<&mut Peer<S>, NoAvailablePeers> {
        unsafe {
            let peer = enet_host_connect(self.host, address, channel_count, data);
            if !peer.is_null() {
                Ok(self.peer_mut(self.peer_index(peer)))
            } else {
                Err(NoAvailablePeers)
            }
        }
    }

    /// Checks for any queued events on the host and dispatches one if available.
    pub fn check_events(&mut self) -> Option<Event<S>> {
        unsafe {
            let mut event: ENetEvent<S> = zeroed();
            if enet_host_check_events(self.host, &mut event) {
                Some(self.create_event(&event))
            } else {
                None
            }
        }
    }

    /// Checks for events on the host and shuttles packets between the host and its peers.
    ///
    /// Should be called fairly regularly for adequate performance.
    ///
    /// # Errors
    ///
    /// Returns any error reported by the socket implementation during [`Socket::receive`] or
    /// [`Socket::send`].
    pub fn service(&mut self) -> Result<Option<Event<S>>, S::Error> {
        unsafe {
            let mut event: ENetEvent<S> = zeroed();
            if enet_host_service(self.host, &mut event)? {
                Ok(Some(self.create_event(&event)))
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

    /// Get a reference to the underlying socket.
    #[must_use]
    pub fn socket(&self) -> &S {
        unsafe { (*self.host).socket.assume_init_ref() }
    }

    /// Get a mutable reference to the underlying socket.
    pub fn socket_mut(&mut self) -> &mut S {
        unsafe { (*self.host).socket.assume_init_mut() }
    }

    /// Get the maximum number of peers that can connect to this host.
    #[must_use]
    pub fn peer_limit(&self) -> usize {
        self.peers.len()
    }

    /// Get a reference to a single peer, if it exists.
    ///
    /// # Note
    ///
    /// Acquires the peer object, even if the peer is not in a connected state. See [`Peer::state`].
    ///
    /// # Panics
    ///
    /// Panics if the peer ID is outside the bounds of peers allocated for this host. Use
    /// [`Host::get_peer`] for a non-panicking version.
    #[must_use]
    pub fn peer(&self, peer: PeerID) -> &Peer<S> {
        self.peers
            .get(peer.0)
            .expect("Expected the peer id to be in bounds.")
    }

    /// Get a reference to a single peer.
    ///
    /// # Note
    ///
    /// Acquires the peer object, even if the peer is not in a connected state. See [`Peer::state`].
    #[must_use]
    pub fn get_peer(&self, peer: PeerID) -> Option<&Peer<S>> {
        self.peers.get(peer.0)
    }

    /// Get a mutable reference to a single peer.
    ///
    /// # Note
    ///
    /// Acquires the peer object, even if the peer is not in a connected state. See [`Peer::state`].
    ///
    /// # Panics
    ///
    /// Panics if the peer ID is outside the bounds of peers allocated for this host. Use
    /// [`Host::get_peer_mut`] for a non-panicking version.
    pub fn peer_mut(&mut self, peer: PeerID) -> &mut Peer<S> {
        self.peers
            .get_mut(peer.0)
            .expect("Expected the peer id to be in bounds.")
    }

    /// Get a mutable reference to a single peer, if it exists.
    ///
    /// # Note
    ///
    /// Acquires the peer object, even if the peer is not in a connected state. See [`Peer::state`].
    #[must_use]
    pub fn get_peer_mut(&mut self, peer: PeerID) -> Option<&mut Peer<S>> {
        self.peers.get_mut(peer.0)
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
    pub fn broadcast(&mut self, channel_id: u8, packet: &Packet) {
        unsafe {
            enet_host_broadcast(self.host, channel_id, packet.packet);
        }
    }

    /// Get the maximum allowed channels for future incoming connections.
    #[must_use]
    pub fn channel_limit(&self) -> usize {
        unsafe { (*self.host).channel_limit }
    }

    /// Limits the maximum allowed channels of future incoming connections. Cannot be 0.
    ///
    /// # Errors
    ///
    /// Returns [`BadParameter`] if `channel_limit` is `0`.
    pub fn set_channel_limit(&mut self, channel_limit: usize) -> Result<(), BadParameter> {
        if channel_limit == 0 {
            return Err(BadParameter {
                method: "Host::set_channel_limit",
                parameter: "channel_limit",
            });
        }
        unsafe {
            enet_host_channel_limit(self.host, channel_limit);
        }
        Ok(())
    }

    /// Get the host's current bandwidth limit as (`incoming bandwidth`, `outgoing bandwidth`) in
    /// bytes/second. Returns [`None`] if there is no limit.
    #[must_use]
    pub fn bandwidth_limit(&self) -> (Option<u32>, Option<u32>) {
        unsafe {
            (
                match (*self.host).incoming_bandwidth {
                    0 => None,
                    limit => Some(limit),
                },
                match (*self.host).outgoing_bandwidth {
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
    /// Returns [`BadParameter`] if `incoming_bandwidth_limit` or `outgoing_bandwidth_limit`
    /// is `Some(0)`.
    pub fn set_bandwidth_limit(
        &mut self,
        incoming_bandwidth_limit: Option<u32>,
        outgoing_bandwidth_limit: Option<u32>,
    ) -> Result<(), BadParameter> {
        if incoming_bandwidth_limit == Some(0) {
            return Err(BadParameter {
                method: "Host::set_bandwidth_limit",
                parameter: "incoming_bandwidth_limit",
            });
        }
        if outgoing_bandwidth_limit == Some(0) {
            return Err(BadParameter {
                method: "Host::set_bandwidth_limit",
                parameter: "outgoing_bandwidth_limit",
            });
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

    /// The maximum transmission unit, or the maximum packet size that will be sent by this host.
    #[must_use]
    pub fn mtu(&self) -> u16 {
        unsafe { (*self.host).mtu as u16 }
    }

    /// Set the maximum transmission unit. See [`Host::mtu`].
    ///
    /// # Errors
    ///
    /// Returns [`BadParameter`] if `mtu` is greater than [`PROTOCOL_MAXIMUM_MTU`] or less than
    /// [`PROTOCOL_MINIMUM_MTU`].
    pub fn set_mtu(&mut self, mtu: u16) -> Result<(), BadParameter> {
        if mtu > PROTOCOL_MAXIMUM_MTU as u16 || mtu < PROTOCOL_MINIMUM_MTU as u16 {
            return Err(BadParameter {
                method: "Host::set_mtu",
                parameter: "mtu",
            });
        }
        unsafe {
            (*self.host).mtu = mtu as u32;
        }
        Ok(())
    }

    /// Get the time according to this host, as provided by
    /// [`HostSettings::time`](`crate::HostSettings::time`).
    pub fn now(&self) -> Duration {
        unsafe { (*self.host).time.assume_init_ref()() }
    }

    fn create_event<'a>(&'a mut self, event: &ENetEvent<S>) -> Event<'a, S> {
        match event.type_0 {
            ENET_EVENT_TYPE_CONNECT => Event::Connect {
                peer: self.peer_mut(self.peer_index(event.peer)),
                data: event.data,
            },
            ENET_EVENT_TYPE_DISCONNECT => Event::Disconnect {
                peer: self.peer_mut(self.peer_index(event.peer)),
                data: event.data,
            },
            ENET_EVENT_TYPE_RECEIVE => Event::Receive {
                peer: self.peer_mut(self.peer_index(event.peer)),
                channel_id: event.channel_id,
                packet: Packet::new_from_ptr(event.packet),
            },
            _ => unreachable!(),
        }
    }

    fn peer_index(&self, peer: *const ENetPeer<S>) -> PeerID {
        PeerID(unsafe { peer.offset_from((*self.host).peers) as usize })
    }
}

impl<S: Socket> Drop for Host<S> {
    fn drop(&mut self) {
        unsafe { enet_host_destroy(self.host) }
    }
}

impl<S: Socket> Debug for Host<S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let host = unsafe { &(*self.host) };
        f.debug_struct("Host")
            .field("socket", &host.socket)
            .field("incomingBandwidth", &host.incoming_bandwidth)
            .field("incomingBandwidth", &host.incoming_bandwidth)
            .field("outgoingBandwidth", &host.outgoing_bandwidth)
            .field("bandwidthThrottleEpoch", &host.bandwidth_throttle_epoch)
            .field("mtu", &host.mtu)
            .field("randomSeed", &host.random_seed)
            .field(
                "recalculateBandwidthLimits",
                &host.recalculate_bandwidth_limits,
            )
            .field("peers", &host.peers)
            .field("peerCount", &host.peer_count)
            .field("channelLimit", &host.channel_limit)
            .field("serviceTime", &host.service_time)
            .field("dispatchQueue", &core::ptr::addr_of!(host.dispatch_queue))
            .field("totalQueued", &host.total_queued)
            .field("packetSize", &host.packet_size)
            .field("headerFlags", &host.header_flags)
            .field("commands", &core::ptr::addr_of!(host.commands))
            .field("commandCount", &host.command_count)
            .field("buffers", &core::ptr::addr_of!(host.buffers))
            .field("bufferCount", &host.buffer_count)
            .field("checksum", &host.checksum)
            .field("time", &host.time)
            .field("compressor", &host.compressor)
            .field("packetData", &host.packet_data)
            .field("receivedAddress", &host.received_address)
            .field("receivedData", &host.received_data)
            .field("receivedDataLength", &host.received_data_length)
            .field("totalSentData", &host.total_sent_data)
            .field("totalSentPackets", &host.total_sent_packets)
            .field("totalReceivedData", &host.total_received_data)
            .field("totalReceivedPackets", &host.total_received_packets)
            .field("connectedPeers", &host.connected_peers)
            .field("bandwidthLimitedPeers", &host.bandwidth_limited_peers)
            .field("duplicatePeers", &host.duplicate_peers)
            .field("maximumPacketSize", &host.maximum_packet_size)
            .field("maximumWaitingData", &host.maximum_waiting_data)
            .field("peers", &self.peers)
            .finish()
    }
}
