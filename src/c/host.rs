use core::{alloc::Layout, mem::MaybeUninit, ptr::write_bytes, time::Duration};

use crate::{
    consts::*, enet_free, enet_list_clear, enet_malloc, enet_packet_destroy,
    enet_peer_queue_outgoing_command, enet_peer_reset, enet_peer_send, enet_time_get, Box,
    Compressor, ENetBuffer, ENetChannel, ENetList, ENetPacket, ENetPeer, ENetProtocol,
    ENetProtocolCommandHeader, Socket, SocketOptions, ENET_PEER_STATE_CONNECTED,
    ENET_PEER_STATE_CONNECTING, ENET_PEER_STATE_DISCONNECTED, ENET_PEER_STATE_DISCONNECT_LATER,
    ENET_PROTOCOL_COMMAND_BANDWIDTH_LIMIT, ENET_PROTOCOL_COMMAND_CONNECT,
    ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE,
};

#[allow(clippy::type_complexity)]
pub(crate) struct ENetHost<S: Socket> {
    pub(crate) socket: MaybeUninit<S>,
    pub(crate) incoming_bandwidth: u32,
    pub(crate) outgoing_bandwidth: u32,
    pub(crate) bandwidth_throttle_epoch: u32,
    pub(crate) mtu: u32,
    pub(crate) random_seed: u32,
    pub(crate) recalculate_bandwidth_limits: i32,
    pub(crate) peers: *mut ENetPeer<S>,
    pub(crate) peer_count: usize,
    pub(crate) channel_limit: usize,
    pub(crate) service_time: u32,
    pub(crate) dispatch_queue: ENetList,
    pub(crate) total_queued: u32,
    pub(crate) packet_size: usize,
    pub(crate) header_flags: u16,
    pub(crate) commands: [ENetProtocol; PROTOCOL_MAXIMUM_PACKET_COMMANDS as usize],
    pub(crate) command_count: usize,
    pub(crate) buffers: [ENetBuffer; BUFFER_MAXIMUM as usize],
    pub(crate) buffer_count: usize,
    pub(crate) checksum: MaybeUninit<Option<Box<dyn Fn(&[&[u8]]) -> u32>>>,
    pub(crate) time: MaybeUninit<Box<dyn Fn() -> Duration>>,
    pub(crate) compressor: MaybeUninit<Option<Box<dyn Compressor>>>,
    pub(crate) packet_data: [[u8; PROTOCOL_MAXIMUM_MTU]; 2],
    pub(crate) received_address: MaybeUninit<Option<S::Address>>,
    pub(crate) received_data: *mut u8,
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
pub(crate) unsafe fn enet_host_create<S: Socket>(
    mut socket: S,
    peer_count: usize,
    mut channel_limit: usize,
    incoming_bandwidth: u32,
    outgoing_bandwidth: u32,
    time: Box<dyn Fn() -> Duration>,
    seed: Option<u32>,
) -> Result<*mut ENetHost<S>, S::Error> {
    let mut current_peer: *mut ENetPeer<S>;
    let host: *mut ENetHost<S> = enet_malloc(Layout::new::<ENetHost<S>>()).cast();
    write_bytes(host, 0, 1);
    (*host).peers = enet_malloc(Layout::array::<ENetPeer<S>>(peer_count).unwrap()).cast();
    write_bytes((*host).peers, 0, peer_count);
    socket.init(SocketOptions {
        receive_buffer: HOST_RECEIVE_BUFFER_SIZE as usize,
        send_buffer: HOST_SEND_BUFFER_SIZE as usize,
    })?;
    (*host).socket.write(socket);
    if channel_limit == 0 || channel_limit > PROTOCOL_MAXIMUM_CHANNEL_COUNT as i32 as usize {
        channel_limit = PROTOCOL_MAXIMUM_CHANNEL_COUNT as i32 as usize;
    } else if channel_limit < PROTOCOL_MINIMUM_CHANNEL_COUNT as i32 as usize {
        channel_limit = PROTOCOL_MINIMUM_CHANNEL_COUNT as i32 as usize;
    }
    (*host).time.write(time);
    if let Some(seed) = seed {
        (*host).random_seed = seed;
    } else {
        (*host).random_seed = host as usize as u32;
        (*host).random_seed =
            ((*host).random_seed as u32).wrapping_add(enet_time_get(host)) as u32 as u32;
        (*host).random_seed = (*host).random_seed << 16_i32 | (*host).random_seed >> 16_i32;
    }
    (*host).channel_limit = channel_limit;
    (*host).incoming_bandwidth = incoming_bandwidth;
    (*host).outgoing_bandwidth = outgoing_bandwidth;
    (*host).bandwidth_throttle_epoch = 0_i32 as u32;
    (*host).recalculate_bandwidth_limits = 0_i32;
    (*host).mtu = HOST_DEFAULT_MTU as i32 as u32;
    (*host).peer_count = peer_count;
    (*host).command_count = 0_i32 as usize;
    (*host).buffer_count = 0_i32 as usize;
    (*host).checksum.write(None);
    (*host).received_address.write(None);
    (*host).received_data = core::ptr::null_mut();
    (*host).received_data_length = 0_i32 as usize;
    (*host).total_sent_data = 0_i32 as u32;
    (*host).total_sent_packets = 0_i32 as u32;
    (*host).total_received_data = 0_i32 as u32;
    (*host).total_received_packets = 0_i32 as u32;
    (*host).total_queued = 0_i32 as u32;
    (*host).connected_peers = 0_i32 as usize;
    (*host).bandwidth_limited_peers = 0_i32 as usize;
    (*host).duplicate_peers = PROTOCOL_MAXIMUM_PEER_ID as i32 as usize;
    (*host).maximum_packet_size = HOST_DEFAULT_MAXIMUM_PACKET_SIZE as i32 as usize;
    (*host).maximum_waiting_data = HOST_DEFAULT_MAXIMUM_WAITING_DATA as i32 as usize;
    (*host).compressor.write(None);
    enet_list_clear(&mut (*host).dispatch_queue);
    current_peer = (*host).peers;
    while current_peer < ((*host).peers).add((*host).peer_count) {
        (*current_peer).host = host;
        (*current_peer).incoming_peer_id = current_peer.offset_from((*host).peers) as i64 as u16;
        (*current_peer).incoming_session_id = 0xff_i32 as u8;
        (*current_peer).outgoing_session_id = (*current_peer).incoming_session_id;
        (*current_peer).address.write(None);
        (*current_peer).data = core::ptr::null_mut();
        enet_list_clear(&mut (*current_peer).acknowledgements);
        enet_list_clear(&mut (*current_peer).sent_reliable_commands);
        enet_list_clear(&mut (*current_peer).outgoing_commands);
        enet_list_clear(&mut (*current_peer).outgoing_send_reliable_commands);
        enet_list_clear(&mut (*current_peer).dispatched_commands);
        enet_peer_reset(current_peer);
        current_peer = current_peer.offset(1);
    }
    Ok(host)
}
pub(crate) unsafe fn enet_host_destroy<S: Socket>(host: *mut ENetHost<S>) {
    let mut current_peer: *mut ENetPeer<S>;
    if host.is_null() {
        return;
    }
    (*host).socket.assume_init_drop();
    current_peer = (*host).peers;
    while current_peer < ((*host).peers).add((*host).peer_count) {
        enet_peer_reset(current_peer);
        (*current_peer).address.assume_init_drop();
        current_peer = current_peer.offset(1);
    }
    (*host).checksum.assume_init_drop();
    (*host).time.assume_init_drop();
    (*host).compressor.assume_init_drop();
    (*host).received_address.assume_init_drop();
    enet_free(
        (*host).peers.cast(),
        Layout::array::<ENetPeer<S>>((*host).peer_count).unwrap(),
    );
    enet_free(host.cast(), Layout::new::<ENetHost<S>>());
}
pub(crate) unsafe fn enet_host_random<S: Socket>(host: *mut ENetHost<S>) -> u32 {
    (*host).random_seed = (*host).random_seed.wrapping_add(0x6d2b79f5_u32);
    let mut n: u32 = (*host).random_seed;
    n = (n ^ n >> 15_i32).wrapping_mul(n | 1_u32);
    n ^= n.wrapping_add((n ^ n >> 7_i32).wrapping_mul(n | 61_u32));
    n ^ n >> 14_i32
}
pub(crate) unsafe fn enet_host_connect<S: Socket>(
    host: *mut ENetHost<S>,
    address: S::Address,
    mut channel_count: usize,
    data: u32,
) -> *mut ENetPeer<S> {
    let mut current_peer: *mut ENetPeer<S>;
    let mut channel: *mut ENetChannel;
    let mut command: ENetProtocol = ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channel_id: 0,
            reliable_sequence_number: 0,
        },
    };
    if channel_count < PROTOCOL_MINIMUM_CHANNEL_COUNT as i32 as usize {
        channel_count = PROTOCOL_MINIMUM_CHANNEL_COUNT as i32 as usize;
    } else if channel_count > PROTOCOL_MAXIMUM_CHANNEL_COUNT as i32 as usize {
        channel_count = PROTOCOL_MAXIMUM_CHANNEL_COUNT as i32 as usize;
    }
    current_peer = (*host).peers;
    while current_peer < ((*host).peers).add((*host).peer_count) {
        if (*current_peer).state == ENET_PEER_STATE_DISCONNECTED as i32 as u32 {
            break;
        }
        current_peer = current_peer.offset(1);
    }
    if current_peer >= ((*host).peers).add((*host).peer_count) {
        return core::ptr::null_mut();
    }
    (*current_peer).channels =
        enet_malloc(Layout::array::<ENetChannel>(channel_count).unwrap()).cast();
    (*current_peer).channel_count = channel_count;
    (*current_peer).state = ENET_PEER_STATE_CONNECTING;
    *(*current_peer).address.assume_init_mut() = Some(address);
    (*current_peer).connect_id = enet_host_random(host);
    (*current_peer).mtu = (*host).mtu;
    if (*host).outgoing_bandwidth == 0_i32 as u32 {
        (*current_peer).window_size = PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    } else {
        (*current_peer).window_size = ((*host).outgoing_bandwidth)
            .wrapping_div(PEER_WINDOW_SIZE_SCALE as i32 as u32)
            .wrapping_mul(PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32);
    }
    if (*current_peer).window_size < PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32 {
        (*current_peer).window_size = PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32;
    } else if (*current_peer).window_size > PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32 {
        (*current_peer).window_size = PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    }
    channel = (*current_peer).channels;
    while channel < ((*current_peer).channels).add(channel_count) {
        (*channel).outgoing_reliable_sequence_number = 0_i32 as u16;
        (*channel).outgoing_unreliable_sequence_number = 0_i32 as u16;
        (*channel).incoming_reliable_sequence_number = 0_i32 as u16;
        (*channel).incoming_unreliable_sequence_number = 0_i32 as u16;
        enet_list_clear(&mut (*channel).incoming_reliable_commands);
        enet_list_clear(&mut (*channel).incoming_unreliable_commands);
        (*channel).used_reliable_windows = 0_i32 as u16;
        write_bytes(((*channel).reliable_windows).as_mut_ptr(), 0, 16);
        channel = channel.offset(1);
    }
    command.header.command = (ENET_PROTOCOL_COMMAND_CONNECT as i32
        | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32) as u8;
    command.header.channel_id = 0xff_i32 as u8;
    command.connect.outgoing_peer_id = (*current_peer).incoming_peer_id.to_be();
    command.connect.incoming_session_id = (*current_peer).incoming_session_id;
    command.connect.outgoing_session_id = (*current_peer).outgoing_session_id;
    command.connect.mtu = (*current_peer).mtu.to_be();
    command.connect.window_size = (*current_peer).window_size.to_be();
    command.connect.channel_count = (channel_count as u32).to_be();
    command.connect.incoming_bandwidth = (*host).incoming_bandwidth.to_be();
    command.connect.outgoing_bandwidth = (*host).outgoing_bandwidth.to_be();
    command.connect.packet_throttle_interval = (*current_peer).packet_throttle_interval.to_be();
    command.connect.packet_throttle_acceleration =
        (*current_peer).packet_throttle_acceleration.to_be();
    command.connect.packet_throttle_deceleration =
        (*current_peer).packet_throttle_deceleration.to_be();
    command.connect.connect_id = (*current_peer).connect_id;
    command.connect.data = data.to_be();
    enet_peer_queue_outgoing_command(
        current_peer,
        &command,
        core::ptr::null_mut(),
        0_i32 as u32,
        0_i32 as u16,
    );
    current_peer
}
pub(crate) unsafe fn enet_host_broadcast<S: Socket>(
    host: *mut ENetHost<S>,
    channel_id: u8,
    packet: *mut ENetPacket,
) {
    let mut current_peer: *mut ENetPeer<S>;
    current_peer = (*host).peers;
    while current_peer < ((*host).peers).add((*host).peer_count) {
        if (*current_peer).state == ENET_PEER_STATE_CONNECTED as i32 as u32 {
            // TODO: do we really want to ignore the result type here?
            _ = enet_peer_send(current_peer, channel_id, packet);
        }
        current_peer = current_peer.offset(1);
    }
    if (*packet).reference_count == 0_i32 as usize {
        enet_packet_destroy(packet);
    }
}
pub(crate) unsafe fn enet_host_compress<S: Socket>(
    host: *mut ENetHost<S>,
    compressor: Option<Box<dyn Compressor>>,
) {
    *(*host).compressor.assume_init_mut() = compressor;
}
pub(crate) unsafe fn enet_host_channel_limit<S: Socket>(
    host: *mut ENetHost<S>,
    mut channel_limit: usize,
) {
    if channel_limit == 0 || channel_limit > PROTOCOL_MAXIMUM_CHANNEL_COUNT as i32 as usize {
        channel_limit = PROTOCOL_MAXIMUM_CHANNEL_COUNT as i32 as usize;
    } else if channel_limit < PROTOCOL_MINIMUM_CHANNEL_COUNT as i32 as usize {
        channel_limit = PROTOCOL_MINIMUM_CHANNEL_COUNT as i32 as usize;
    }
    (*host).channel_limit = channel_limit;
}
pub(crate) unsafe fn enet_host_bandwidth_limit<S: Socket>(
    host: *mut ENetHost<S>,
    incoming_bandwidth: u32,
    outgoing_bandwidth: u32,
) {
    (*host).incoming_bandwidth = incoming_bandwidth;
    (*host).outgoing_bandwidth = outgoing_bandwidth;
    (*host).recalculate_bandwidth_limits = 1_i32;
}
pub(crate) unsafe fn enet_host_bandwidth_throttle<S: Socket>(host: *mut ENetHost<S>) {
    let time_current: u32 = enet_time_get(host);
    let elapsed_time: u32 = time_current.wrapping_sub((*host).bandwidth_throttle_epoch);
    let mut peers_remaining: u32 = (*host).connected_peers as u32;
    let mut data_total: u32 = !0_i32 as u32;
    let mut bandwidth: u32 = !0_i32 as u32;
    let mut throttle: u32;
    let mut bandwidth_limit: u32 = 0_i32 as u32;
    let mut needs_adjustment = (*host).bandwidth_limited_peers > 0_usize;
    let mut peer: *mut ENetPeer<S>;
    let mut command: ENetProtocol = ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channel_id: 0,
            reliable_sequence_number: 0,
        },
    };
    if elapsed_time < HOST_BANDWIDTH_THROTTLE_INTERVAL as i32 as u32 {
        return;
    }
    (*host).bandwidth_throttle_epoch = time_current;
    if peers_remaining == 0_i32 as u32 {
        return;
    }
    if (*host).outgoing_bandwidth != 0_i32 as u32 {
        data_total = 0_i32 as u32;
        bandwidth = ((*host).outgoing_bandwidth)
            .wrapping_mul(elapsed_time)
            .wrapping_div(1000_i32 as u32);
        peer = (*host).peers;
        while peer < ((*host).peers).add((*host).peer_count) {
            if !((*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
                && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32)
            {
                data_total = data_total.wrapping_add((*peer).outgoing_data_total);
            }
            peer = peer.offset(1);
        }
    }
    while peers_remaining > 0_i32 as u32 && needs_adjustment {
        needs_adjustment = false;
        if data_total <= bandwidth {
            throttle = PEER_PACKET_THROTTLE_SCALE as i32 as u32;
        } else {
            throttle = bandwidth
                .wrapping_mul(PEER_PACKET_THROTTLE_SCALE as i32 as u32)
                .wrapping_div(data_total);
        }
        peer = (*host).peers;
        while peer < ((*host).peers).add((*host).peer_count) {
            let peer_bandwidth: u32;
            if !((*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
                && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
                || (*peer).incoming_bandwidth == 0_i32 as u32
                || (*peer).outgoing_bandwidth_throttle_epoch == time_current)
            {
                peer_bandwidth = ((*peer).incoming_bandwidth)
                    .wrapping_mul(elapsed_time)
                    .wrapping_div(1000_i32 as u32);
                if throttle
                    .wrapping_mul((*peer).outgoing_data_total)
                    .wrapping_div(PEER_PACKET_THROTTLE_SCALE as i32 as u32)
                    > peer_bandwidth
                {
                    (*peer).packet_throttle_limit = peer_bandwidth
                        .wrapping_mul(PEER_PACKET_THROTTLE_SCALE as i32 as u32)
                        .wrapping_div((*peer).outgoing_data_total);
                    if (*peer).packet_throttle_limit == 0_i32 as u32 {
                        (*peer).packet_throttle_limit = 1_i32 as u32;
                    }
                    if (*peer).packet_throttle > (*peer).packet_throttle_limit {
                        (*peer).packet_throttle = (*peer).packet_throttle_limit;
                    }
                    (*peer).outgoing_bandwidth_throttle_epoch = time_current;
                    (*peer).incoming_data_total = 0_i32 as u32;
                    (*peer).outgoing_data_total = 0_i32 as u32;
                    needs_adjustment = true;
                    peers_remaining = peers_remaining.wrapping_sub(1);
                    bandwidth = bandwidth.wrapping_sub(peer_bandwidth);
                    data_total = data_total.wrapping_sub(peer_bandwidth);
                }
            }
            peer = peer.offset(1);
        }
    }
    if peers_remaining > 0_i32 as u32 {
        if data_total <= bandwidth {
            throttle = PEER_PACKET_THROTTLE_SCALE as i32 as u32;
        } else {
            throttle = bandwidth
                .wrapping_mul(PEER_PACKET_THROTTLE_SCALE as i32 as u32)
                .wrapping_div(data_total);
        }
        peer = (*host).peers;
        while peer < ((*host).peers).add((*host).peer_count) {
            if !((*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
                && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
                || (*peer).outgoing_bandwidth_throttle_epoch == time_current)
            {
                (*peer).packet_throttle_limit = throttle;
                if (*peer).packet_throttle > (*peer).packet_throttle_limit {
                    (*peer).packet_throttle = (*peer).packet_throttle_limit;
                }
                (*peer).incoming_data_total = 0_i32 as u32;
                (*peer).outgoing_data_total = 0_i32 as u32;
            }
            peer = peer.offset(1);
        }
    }
    if (*host).recalculate_bandwidth_limits != 0 {
        (*host).recalculate_bandwidth_limits = 0_i32;
        peers_remaining = (*host).connected_peers as u32;
        bandwidth = (*host).incoming_bandwidth;
        needs_adjustment = true;
        if bandwidth == 0_i32 as u32 {
            bandwidth_limit = 0_i32 as u32;
        } else {
            while peers_remaining > 0_i32 as u32 && needs_adjustment {
                needs_adjustment = false;
                bandwidth_limit = bandwidth.wrapping_div(peers_remaining);
                peer = (*host).peers;
                while peer < ((*host).peers).add((*host).peer_count) {
                    if !((*peer).incoming_bandwidth_throttle_epoch == time_current
                        || (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
                            && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
                        || (*peer).outgoing_bandwidth > 0_i32 as u32
                            && (*peer).outgoing_bandwidth >= bandwidth_limit)
                    {
                        (*peer).incoming_bandwidth_throttle_epoch = time_current;
                        needs_adjustment = true;
                        peers_remaining = peers_remaining.wrapping_sub(1);
                        bandwidth = bandwidth.wrapping_sub((*peer).outgoing_bandwidth);
                    }
                    peer = peer.offset(1);
                }
            }
        }
        peer = (*host).peers;
        while peer < ((*host).peers).add((*host).peer_count) {
            if !((*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
                && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32)
            {
                command.header.command = (ENET_PROTOCOL_COMMAND_BANDWIDTH_LIMIT as i32
                    | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32)
                    as u8;
                command.header.channel_id = 0xff_i32 as u8;
                command.bandwidth_limit.outgoing_bandwidth = (*host).outgoing_bandwidth.to_be();
                if (*peer).incoming_bandwidth_throttle_epoch == time_current {
                    command.bandwidth_limit.incoming_bandwidth = (*peer).outgoing_bandwidth.to_be();
                } else {
                    command.bandwidth_limit.incoming_bandwidth = bandwidth_limit.to_be();
                }
                enet_peer_queue_outgoing_command(
                    peer,
                    &command,
                    core::ptr::null_mut(),
                    0_i32 as u32,
                    0_i32 as u16,
                );
            }
            peer = peer.offset(1);
        }
    }
}
