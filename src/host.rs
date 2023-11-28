use std::{
    collections::VecDeque,
    mem::{self, size_of},
};

use crate::{
    c_void, enet_list_clear, enet_memset, enet_peer_disconnect, enet_peer_disconnect_later,
    enet_peer_disconnect_now, enet_peer_ping, enet_peer_ping_interval,
    enet_peer_queue_outgoing_command, enet_peer_reset, enet_peer_send,
    enet_peer_throttle_configure, enet_peer_timeout, enet_time_get,
    protocol::{
        enet_host_check_events, enet_host_flush, enet_protocol_dispatch_incoming_commands,
        enet_protocol_receive_incoming_commands, enet_protocol_send_outgoing_commands,
    },
    time::enet_time_difference,
    Channel, ENetBuffer, ENetProtocol, ENetProtocolCommandHeader, Event, Packet, Peer, PeerID,
    PeerState, Socket, SocketOption, ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL,
    ENET_HOST_DEFAULT_MAXIMUM_PACKET_SIZE, ENET_HOST_DEFAULT_MAXIMUM_WAITING_DATA,
    ENET_HOST_DEFAULT_MTU, ENET_HOST_RECEIVE_BUFFER_SIZE, ENET_HOST_SEND_BUFFER_SIZE,
    ENET_PEER_PACKET_THROTTLE_SCALE, ENET_PEER_WINDOW_SIZE_SCALE,
    ENET_PROTOCOL_COMMAND_BANDWIDTH_LIMIT, ENET_PROTOCOL_COMMAND_CONNECT,
    ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE, ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT,
    ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS, ENET_PROTOCOL_MAXIMUM_PEER_ID,
    ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE, ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT,
    ENET_PROTOCOL_MINIMUM_WINDOW_SIZE,
};

pub struct Host<S: Socket> {
    pub(crate) socket: S,
    pub(crate) incoming_bandwidth: u32,
    pub(crate) outgoing_bandwidth: u32,
    pub(crate) bandwidth_throttle_epoch: u32,
    pub(crate) mtu: u32,
    pub(crate) random_seed: u32,
    pub(crate) recalculate_bandwidth_limits: bool,
    pub(crate) peers: Vec<Peer<S>>,
    pub(crate) peer_count: usize,
    pub(crate) channel_limit: usize,
    pub(crate) service_time: u32,
    pub(crate) dispatch_queue: VecDeque<PeerID>,
    pub(crate) total_queued: u32,
    pub(crate) packet_size: usize,
    pub(crate) header_flags: u16,
    pub(crate) commands: [ENetProtocol; ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS],
    pub(crate) command_count: usize,
    pub(crate) buffers: [ENetBuffer; 65],
    pub(crate) buffer_count: usize,
    pub(crate) packet_data: [[u8; 4096]; 2],
    pub(crate) received_address: Option<S::PeerAddress>,
    pub(crate) received_data_index: usize,
    pub(crate) received_data_length: usize,
    pub(crate) total_sent_data: u32,
    pub(crate) total_sent_packets: u32,
    pub(crate) total_received_data: u32,
    pub(crate) total_received_packets: u32,
    pub(crate) connected_peers: usize,
    pub(crate) bandwidth_limited_peers: usize,
    pub(crate) duplicate_peers: usize,
    pub(crate) maximum_packet_size: usize,
    pub(crate) maximum_waiting_data: usize,
}

unsafe impl<S: Socket> Send for Host<S> {}
unsafe impl<S: Socket> Sync for Host<S> {}

impl<S: Socket> Host<S> {
    pub fn create(
        address: S::BindAddress,
        peer_count: usize,
        mut channel_limit: usize,
        incoming_bandwidth: u32,
        outgoing_bandwidth: u32,
    ) -> Result<Host<S>, crate::Error> {
        if peer_count > ENET_PROTOCOL_MAXIMUM_PEER_ID as usize {
            return Err(crate::Error::InvalidValueForParameter {
                param: "peer_count",
            });
        }
        let mut peers = vec![];
        for _ in 0..peer_count {
            peers.push(Peer::default());
        }

        let mut socket =
            S::bind(address).map_err(|err| crate::Error::FailedToBind(Box::new(err)))?;
        // TODO: ignore these?
        _ = socket.set_option(SocketOption::NonBlocking, 1);
        _ = socket.set_option(SocketOption::Broadcast, 1);
        _ = socket.set_option(SocketOption::ReceiveBuffer, ENET_HOST_RECEIVE_BUFFER_SIZE);
        _ = socket.set_option(SocketOption::SendBuffer, ENET_HOST_SEND_BUFFER_SIZE);
        if channel_limit == 0 || channel_limit > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT {
            channel_limit = ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT;
        } else if channel_limit < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT {
            channel_limit = ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT;
        }
        let mut random_seed = peers.as_ptr() as usize as u32;
        random_seed = random_seed.wrapping_add(enet_time_get());
        random_seed = random_seed << 16 | random_seed >> 16;
        let mut host = Host {
            socket,
            incoming_bandwidth,
            outgoing_bandwidth,
            bandwidth_throttle_epoch: 0,
            mtu: ENET_HOST_DEFAULT_MTU,
            random_seed,
            recalculate_bandwidth_limits: false,
            peers: vec![],
            peer_count,
            channel_limit,
            service_time: 0,
            dispatch_queue: VecDeque::new(),
            total_queued: 0,
            packet_size: 0,
            header_flags: 0,
            commands: [ENetProtocol::default(); ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS],
            command_count: 0,
            buffers: [ENetBuffer::default(); 65],
            buffer_count: 0,
            packet_data: [[0; 4096]; 2],
            received_address: None,
            received_data_index: 0,
            received_data_length: 0,
            total_sent_data: 0,
            total_sent_packets: 0,
            total_received_data: 0,
            total_received_packets: 0,
            connected_peers: 0,
            bandwidth_limited_peers: 0,
            duplicate_peers: ENET_PROTOCOL_MAXIMUM_PEER_ID as usize,
            maximum_packet_size: ENET_HOST_DEFAULT_MAXIMUM_PACKET_SIZE as usize,
            maximum_waiting_data: ENET_HOST_DEFAULT_MAXIMUM_WAITING_DATA as usize,
        };
        for (index, current_peer) in peers.iter_mut().enumerate() {
            current_peer.index = PeerID(index);
            current_peer.incoming_peer_id = index as u16;
            current_peer.incoming_session_id = 0xff;
            current_peer.outgoing_session_id = current_peer.incoming_session_id;
            current_peer.acknowledgements.clear();
            enet_list_clear(&mut current_peer.sent_reliable_commands);
            enet_list_clear(&mut current_peer.outgoing_commands);
            enet_list_clear(&mut current_peer.outgoing_send_reliable_commands);
            enet_list_clear(&mut current_peer.dispatched_commands);
            enet_peer_reset(&mut host, current_peer);
        }
        host.peers = peers;
        Ok(host)
    }

    pub fn connect(
        &mut self,
        address: S::PeerAddress,
        mut channel_count: usize,
        data: u32,
    ) -> Result<PeerID, crate::Error> {
        channel_count = channel_count.clamp(
            ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT,
            ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT,
        );

        let Some(peer_id) = self
            .peers
            .iter()
            .position(|peer| peer.state == PeerState::Disconnected)
            .map(PeerID)
        else {
            return Err(crate::Error::NoAvailablePeer);
        };

        self.peer_scope_unchecked(peer_id, |host, current_peer| {
            current_peer.channels = vec![];
            for _ in 0..channel_count {
                current_peer.channels.push(Channel::default());
            }
            current_peer.channel_count = channel_count;
            current_peer.state = PeerState::Connecting;
            current_peer.address = Some(address.clone());
            current_peer.connect_id = enet_host_random(host);
            current_peer.mtu = host.mtu;
            if host.outgoing_bandwidth == 0 {
                current_peer.window_size = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE;
            } else {
                current_peer.window_size = (host.outgoing_bandwidth / ENET_PEER_WINDOW_SIZE_SCALE)
                    * ENET_PROTOCOL_MINIMUM_WINDOW_SIZE;
            }
            current_peer.window_size = current_peer.window_size.clamp(
                ENET_PROTOCOL_MINIMUM_WINDOW_SIZE,
                ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE,
            );
            for channel in current_peer.channels.iter_mut() {
                channel.outgoing_reliable_sequence_number = 0;
                channel.outgoing_unreliable_sequence_number = 0;
                channel.incoming_reliable_sequence_number = 0;
                channel.incoming_unreliable_sequence_number = 0;
                enet_list_clear(&mut channel.incoming_reliable_commands);
                enet_list_clear(&mut channel.incoming_unreliable_commands);
                channel.used_reliable_windows = 0;
                unsafe {
                    enet_memset(
                        (channel.reliable_windows).as_mut_ptr() as *mut c_void,
                        0,
                        size_of::<[u16; 16]>(),
                    );
                }
            }

            let mut command = ENetProtocol {
                header: ENetProtocolCommandHeader {
                    command: 0,
                    channel_id: 0,
                    reliable_sequence_number: 0,
                },
            };
            command.header.command =
                ENET_PROTOCOL_COMMAND_CONNECT | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE;
            command.header.channel_id = 0xff;
            command.connect.outgoing_peer_id = current_peer.incoming_peer_id.to_be();
            command.connect.incoming_session_id = current_peer.incoming_session_id;
            command.connect.outgoing_session_id = current_peer.outgoing_session_id;
            command.connect.mtu = (current_peer.mtu).to_be();
            command.connect.window_size = (current_peer.window_size).to_be();
            command.connect.channel_count = (channel_count as u32).to_be();
            command.connect.incoming_bandwidth = (host.incoming_bandwidth).to_be();
            command.connect.outgoing_bandwidth = (host.outgoing_bandwidth).to_be();
            command.connect.packet_throttle_interval =
                (current_peer.packet_throttle_interval).to_be();
            command.connect.packet_throttle_acceleration =
                (current_peer.packet_throttle_acceleration).to_be();
            command.connect.packet_throttle_deceleration =
                (current_peer.packet_throttle_deceleration).to_be();
            command.connect.connect_id = current_peer.connect_id;
            command.connect.data = (data).to_be();
            unsafe {
                enet_peer_queue_outgoing_command(
                    host,
                    current_peer,
                    &command,
                    std::ptr::null_mut(),
                    0,
                    0,
                );
            }
            Ok(peer_id)
        })
    }

    pub fn check_event(&mut self) -> Result<Option<Event>, crate::Error> {
        let mut event = Event::None;
        if unsafe { enet_host_check_events(self, &mut event)? } {
            Ok(Some(event))
        } else {
            Ok(None)
        }
    }

    pub fn service(&mut self) -> Result<Option<Event>, crate::Error> {
        unsafe {
            let mut event = Event::None;
            if enet_protocol_dispatch_incoming_commands(self, &mut event)? {
                return Ok(Some(event));
            }
            self.service_time = enet_time_get();
            if enet_time_difference(self.service_time, self.bandwidth_throttle_epoch)
                >= ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL
            {
                enet_host_bandwidth_throttle(self);
            }
            if enet_protocol_send_outgoing_commands(self, &mut event, true)? {
                return Ok(Some(event));
            }
            if enet_protocol_receive_incoming_commands(self, &mut event)? {
                return Ok(Some(event));
            }
            if enet_protocol_send_outgoing_commands(self, &mut event, true)? {
                return Ok(Some(event));
            }
            if enet_protocol_dispatch_incoming_commands(self, &mut event)? {
                return Ok(Some(event));
            }
            Ok(None)
        }
    }

    pub fn flush(&mut self) {
        enet_host_flush(self);
    }

    pub fn channel_limit(&self) -> usize {
        self.channel_limit
    }

    pub fn set_channel_limit(&mut self, channel_limit: usize) {
        enet_host_channel_limit(self, channel_limit);
    }

    pub fn bandwidth_limit(&self) -> (u32, u32) {
        (self.incoming_bandwidth, self.outgoing_bandwidth)
    }

    pub fn set_bandwidth_limit(&mut self, incoming_bandwidth: u32, outgoing_bandwidth: u32) {
        enet_host_bandwidth_limit(self, incoming_bandwidth, outgoing_bandwidth);
    }

    pub fn ping(&mut self, peer: PeerID) -> Result<(), crate::Error> {
        self.peer_scope(peer, |host, peer| unsafe { enet_peer_ping(host, peer) })
    }

    pub fn send(
        &mut self,
        peer: PeerID,
        channel_id: u8,
        packet: Packet,
    ) -> Result<(), crate::Error> {
        self.peer_scope(peer, move |host, peer| {
            enet_peer_send(host, peer, channel_id, packet)
        })?
    }

    pub fn broadcast(&mut self, channel_id: u8, packet: Packet) -> Result<(), crate::Error> {
        enet_host_broadcast(self, channel_id, packet)
    }

    pub fn set_timeout(
        &mut self,
        peer: PeerID,
        limit: u32,
        minimum: u32,
        maximum: u32,
    ) -> Result<(), crate::Error> {
        self.peer_scope(peer, |_, peer| {
            enet_peer_timeout(peer, limit, minimum, maximum);
        })
    }

    pub fn set_ping_interval(
        &mut self,
        peer: PeerID,
        ping_interval: u32,
    ) -> Result<(), crate::Error> {
        self.peer_scope(peer, |_, peer| {
            enet_peer_ping_interval(peer, ping_interval);
        })
    }

    pub fn set_throttle(
        &mut self,
        peer: PeerID,
        interval: u32,
        acceleration: u32,
        deceleration: u32,
    ) -> Result<(), crate::Error> {
        self.peer_scope(peer, |host, peer| {
            enet_peer_throttle_configure(host, peer, interval, acceleration, deceleration);
        })
    }

    pub fn disconnect(&mut self, peer: PeerID) -> Result<(), crate::Error> {
        self.peer_scope(peer, |host, peer| {
            enet_peer_disconnect(host, peer, 0);
        })
    }

    pub fn disconnect_now(&mut self, peer: PeerID) -> Result<(), crate::Error> {
        self.peer_scope(peer, |host, peer| {
            enet_peer_disconnect_now(host, peer, 0);
        })
    }

    pub fn disconnect_later(&mut self, peer: PeerID) -> Result<(), crate::Error> {
        self.peer_scope(peer, |host, peer| {
            enet_peer_disconnect_later(host, peer, 0);
        })
    }

    pub(crate) fn peer_scope<T>(
        &mut self,
        id: PeerID,
        scope: impl FnOnce(&mut Host<S>, &mut Peer<S>) -> T,
    ) -> Result<T, crate::Error> {
        if id.0 < self.peers.len() {
            Ok(self.peer_scope_unchecked(id, scope))
        } else {
            Err(crate::Error::InvalidPeer)
        }
    }

    pub(crate) fn peer_scope_unchecked<T>(
        &mut self,
        id: PeerID,
        scope: impl FnOnce(&mut Host<S>, &mut Peer<S>) -> T,
    ) -> T {
        if let Some(peer) = self.peers.get_mut(id.0) {
            let mut peer = mem::take(peer);
            let result = scope(self, &mut peer);
            self.peers[id.0] = peer;
            result
        } else {
            unreachable!();
        }
    }

    pub(crate) fn received_data(&mut self) -> *mut u8 {
        self.packet_data[self.received_data_index].as_mut_ptr()
    }
}

pub(crate) fn enet_host_random<S: Socket>(host: &mut Host<S>) -> u32 {
    host.random_seed = (host.random_seed).wrapping_add(0x6d2b79f5);
    let mut n: u32 = host.random_seed;
    n = (n ^ n >> 15).wrapping_mul(n | 1);
    n ^= n.wrapping_add((n ^ n >> 7).wrapping_mul(n | 61));
    n ^ n >> 14
}

pub(crate) fn enet_host_broadcast<S: Socket>(
    host: &mut Host<S>,
    channel_id: u8,
    packet: Packet,
) -> Result<(), crate::Error> {
    unsafe {
        let mut current_peer = host.peers.as_mut_ptr();
        while current_peer < &mut *(host.peers.as_mut_ptr()).add(host.peer_count) as *mut Peer<S> {
            if !((*current_peer).state != PeerState::Connected) {
                _ = enet_peer_send(host, &mut *current_peer, channel_id, packet.clone());
            }
            current_peer = current_peer.offset(1);
        }
        Ok(())
    }
}

pub(crate) fn enet_host_channel_limit<S: Socket>(host: &mut Host<S>, mut channel_limit: usize) {
    if channel_limit == 0 || channel_limit > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT {
        channel_limit = ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT;
    } else if channel_limit < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT {
        channel_limit = ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT;
    }
    host.channel_limit = channel_limit;
}

pub(crate) fn enet_host_bandwidth_limit<S: Socket>(
    host: &mut Host<S>,
    incoming_bandwidth: u32,
    outgoing_bandwidth: u32,
) {
    host.incoming_bandwidth = incoming_bandwidth;
    host.outgoing_bandwidth = outgoing_bandwidth;
    host.recalculate_bandwidth_limits = true;
}

pub(crate) unsafe fn enet_host_bandwidth_throttle<S: Socket>(host: &mut Host<S>) {
    let time_current: u32 = enet_time_get();
    let elapsed_time: u32 = time_current.wrapping_sub(host.bandwidth_throttle_epoch);
    let mut peers_remaining: u32 = host.connected_peers as u32;
    let mut data_total: u32 = !0;
    let mut bandwidth: u32 = !0;
    let mut throttle;
    let mut bandwidth_limit: u32 = 0;
    let mut needs_adjustment: bool = host.bandwidth_limited_peers > 0;
    let mut peer: *mut Peer<S>;
    let mut command = ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channel_id: 0,
            reliable_sequence_number: 0,
        },
    };
    if elapsed_time < ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL {
        return;
    }
    host.bandwidth_throttle_epoch = time_current;
    if peers_remaining == 0 {
        return;
    }
    if host.outgoing_bandwidth != 0 {
        data_total = 0;
        bandwidth = (host.outgoing_bandwidth)
            .wrapping_mul(elapsed_time)
            .wrapping_div(1000);
        peer = host.peers.as_mut_ptr();
        while peer < &mut *(host.peers.as_mut_ptr()).add(host.peer_count) as *mut Peer<S> {
            if !((*peer).state != PeerState::Connected
                && (*peer).state != PeerState::DisconnectLater)
            {
                data_total += (*peer).outgoing_data_total;
            }
            peer = peer.offset(1);
        }
    }
    while peers_remaining > 0 && needs_adjustment {
        needs_adjustment = false;
        if data_total <= bandwidth {
            throttle = ENET_PEER_PACKET_THROTTLE_SCALE;
        } else {
            throttle = (bandwidth * ENET_PEER_PACKET_THROTTLE_SCALE) / data_total;
        }
        peer = host.peers.as_mut_ptr();
        while peer < &mut *(host.peers.as_mut_ptr()).add(host.peer_count) as *mut Peer<S> {
            let peer_bandwidth: u32;
            if !((*peer).state != PeerState::Connected
                && (*peer).state != PeerState::DisconnectLater
                || (*peer).incoming_bandwidth == 0
                || (*peer).outgoing_bandwidth_throttle_epoch == time_current)
            {
                peer_bandwidth = ((*peer).incoming_bandwidth)
                    .wrapping_mul(elapsed_time)
                    .wrapping_div(1000);
                if (throttle * (*peer).outgoing_data_total) / ENET_PEER_PACKET_THROTTLE_SCALE
                    > peer_bandwidth
                {
                    (*peer).packet_throttle_limit = (peer_bandwidth
                        * ENET_PEER_PACKET_THROTTLE_SCALE)
                        / (*peer).outgoing_data_total;
                    if (*peer).packet_throttle_limit == 0 {
                        (*peer).packet_throttle_limit = 1;
                    }
                    if (*peer).packet_throttle > (*peer).packet_throttle_limit {
                        (*peer).packet_throttle = (*peer).packet_throttle_limit;
                    }
                    (*peer).outgoing_bandwidth_throttle_epoch = time_current;
                    (*peer).incoming_data_total = 0;
                    (*peer).outgoing_data_total = 0;
                    needs_adjustment = true;
                    peers_remaining -= 1;
                    bandwidth -= peer_bandwidth;
                    data_total -= peer_bandwidth;
                }
            }
            peer = peer.offset(1);
        }
    }
    if peers_remaining > 0 {
        if data_total <= bandwidth {
            throttle = ENET_PEER_PACKET_THROTTLE_SCALE;
        } else {
            throttle = (bandwidth * ENET_PEER_PACKET_THROTTLE_SCALE) / data_total;
        }
        peer = host.peers.as_mut_ptr();
        while peer < &mut *(host.peers.as_mut_ptr()).add(host.peer_count) as *mut Peer<S> {
            if !((*peer).state != PeerState::Connected
                && (*peer).state != PeerState::DisconnectLater
                || (*peer).outgoing_bandwidth_throttle_epoch == time_current)
            {
                (*peer).packet_throttle_limit = throttle;
                if (*peer).packet_throttle > (*peer).packet_throttle_limit {
                    (*peer).packet_throttle = (*peer).packet_throttle_limit;
                }
                (*peer).incoming_data_total = 0;
                (*peer).outgoing_data_total = 0;
            }
            peer = peer.offset(1);
        }
    }
    if host.recalculate_bandwidth_limits {
        host.recalculate_bandwidth_limits = false;
        peers_remaining = host.connected_peers as u32;
        bandwidth = host.incoming_bandwidth;
        needs_adjustment = true;
        if bandwidth == 0 {
            bandwidth_limit = 0;
        } else {
            while peers_remaining > 0 && needs_adjustment {
                needs_adjustment = false;
                bandwidth_limit = bandwidth / peers_remaining;
                peer = host.peers.as_mut_ptr();
                while peer < &mut *(host.peers.as_mut_ptr()).add(host.peer_count) as *mut Peer<S> {
                    if !((*peer).incoming_bandwidth_throttle_epoch == time_current
                        || (*peer).state != PeerState::Connected
                            && (*peer).state != PeerState::DisconnectLater
                        || (*peer).outgoing_bandwidth > 0
                            && (*peer).outgoing_bandwidth >= bandwidth_limit)
                    {
                        (*peer).incoming_bandwidth_throttle_epoch = time_current;
                        needs_adjustment = true;
                        peers_remaining -= 1;
                        bandwidth -= (*peer).outgoing_bandwidth;
                    }
                    peer = peer.offset(1);
                }
            }
        }
        peer = host.peers.as_mut_ptr();
        while peer < &mut *(host.peers.as_mut_ptr()).add(host.peer_count) as *mut Peer<S> {
            if !((*peer).state != PeerState::Connected
                && (*peer).state != PeerState::DisconnectLater)
            {
                command.header.command =
                    ENET_PROTOCOL_COMMAND_BANDWIDTH_LIMIT | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE;
                command.header.channel_id = 0xff;
                command.bandwidth_limit.outgoing_bandwidth = (host.outgoing_bandwidth).to_be();
                if (*peer).incoming_bandwidth_throttle_epoch == time_current {
                    command.bandwidth_limit.incoming_bandwidth =
                        ((*peer).outgoing_bandwidth).to_be();
                } else {
                    command.bandwidth_limit.incoming_bandwidth = (bandwidth_limit).to_be();
                }
                enet_peer_queue_outgoing_command(
                    host,
                    &mut *peer,
                    &command,
                    std::ptr::null_mut(),
                    0,
                    0,
                );
            }
            peer = peer.offset(1);
        }
    }
}
