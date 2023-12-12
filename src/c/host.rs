use std::{mem::MaybeUninit, time::Duration};

use crate::{
    consts::*,
    enet_free, enet_list_clear, enet_malloc, enet_peer_reset, enet_time_get,
    os::{_enet_memset, c_void},
    Compressor, ENetBuffer, ENetChannel, ENetList, ENetPeer, ENetProtocol,
    ENetProtocolCommandHeader, Socket, SocketOptions, _ENetEvent, _ENetProtocol,
    enet_packet_destroy, enet_peer_queue_outgoing_command, enet_peer_send, ENetPacket,
    ENET_PEER_STATE_CONNECTED, ENET_PEER_STATE_CONNECTING, ENET_PEER_STATE_DISCONNECTED,
    ENET_PEER_STATE_DISCONNECT_LATER, ENET_PROTOCOL_COMMAND_BANDWIDTH_LIMIT,
    ENET_PROTOCOL_COMMAND_CONNECT, ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE,
};

pub(crate) type ENetInterceptCallback<S> =
    Option<unsafe extern "C" fn(*mut _ENetHost<S>, *mut _ENetEvent<S>) -> i32>;
#[allow(clippy::type_complexity)]
pub(crate) struct _ENetHost<S: Socket> {
    pub(crate) socket: MaybeUninit<S>,
    pub(crate) incomingBandwidth: u32,
    pub(crate) outgoingBandwidth: u32,
    pub(crate) bandwidthThrottleEpoch: u32,
    pub(crate) mtu: u32,
    pub(crate) randomSeed: u32,
    pub(crate) recalculateBandwidthLimits: i32,
    pub(crate) peers: *mut ENetPeer<S>,
    pub(crate) peerCount: usize,
    pub(crate) channelLimit: usize,
    pub(crate) serviceTime: u32,
    pub(crate) dispatchQueue: ENetList,
    pub(crate) totalQueued: u32,
    pub(crate) packetSize: usize,
    pub(crate) headerFlags: u16,
    pub(crate) commands: [ENetProtocol; 32],
    pub(crate) commandCount: usize,
    pub(crate) buffers: [ENetBuffer; 65],
    pub(crate) bufferCount: usize,
    pub(crate) checksum: MaybeUninit<Option<Box<dyn Fn(Vec<&[u8]>) -> u32>>>,
    pub(crate) time: MaybeUninit<Box<dyn Fn() -> Duration>>,
    pub(crate) compressor: MaybeUninit<Option<Box<dyn Compressor>>>,
    pub(crate) packetData: [[u8; 4096]; 2],
    pub(crate) receivedAddress: MaybeUninit<Option<S::PeerAddress>>,
    pub(crate) receivedData: *mut u8,
    pub(crate) receivedDataLength: usize,
    pub(crate) totalSentData: u32,
    pub(crate) totalSentPackets: u32,
    pub(crate) totalReceivedData: u32,
    pub(crate) totalReceivedPackets: u32,
    pub(crate) intercept: ENetInterceptCallback<S>,
    pub(crate) connectedPeers: usize,
    pub(crate) bandwidthLimitedPeers: usize,
    pub(crate) duplicatePeers: usize,
    pub(crate) maximumPacketSize: usize,
    pub(crate) maximumWaitingData: usize,
}
pub(crate) type ENetHost<S> = _ENetHost<S>;
pub(crate) unsafe fn enet_host_create<S: Socket>(
    mut socket: S,
    peerCount: usize,
    mut channelLimit: usize,
    incomingBandwidth: u32,
    outgoingBandwidth: u32,
    time: Box<dyn Fn() -> Duration>,
    seed: Option<u32>,
) -> *mut ENetHost<S> {
    let mut currentPeer: *mut ENetPeer<S>;
    if peerCount > ENET_PROTOCOL_MAXIMUM_PEER_ID as i32 as usize {
        return std::ptr::null_mut();
    }
    let host = enet_malloc(::core::mem::size_of::<ENetHost<S>>()) as *mut ENetHost<S>;
    if host.is_null() {
        return std::ptr::null_mut();
    }
    _enet_memset(
        host as *mut c_void,
        0_i32,
        ::core::mem::size_of::<ENetHost<S>>(),
    );
    (*host).peers = enet_malloc(peerCount.wrapping_mul(::core::mem::size_of::<ENetPeer<S>>()))
        as *mut ENetPeer<S>;
    if ((*host).peers).is_null() {
        enet_free(host as *mut c_void);
        return std::ptr::null_mut();
    }
    _enet_memset(
        (*host).peers as *mut c_void,
        0_i32,
        peerCount.wrapping_mul(::core::mem::size_of::<ENetPeer<S>>()),
    );
    _ = socket.init(SocketOptions {
        receive_buffer: ENET_HOST_RECEIVE_BUFFER_SIZE as usize,
        send_buffer: ENET_HOST_SEND_BUFFER_SIZE as usize,
    });
    (*host).socket.write(socket);
    if channelLimit == 0 || channelLimit > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as i32 as usize {
        channelLimit = ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as i32 as usize;
    } else if channelLimit < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as i32 as usize {
        channelLimit = ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as i32 as usize;
    }
    (*host).time.write(time);
    if let Some(seed) = seed {
        (*host).randomSeed = seed;
    } else {
        (*host).randomSeed = host as usize as u32;
        (*host).randomSeed =
            ((*host).randomSeed as u32).wrapping_add(enet_time_get(host)) as u32 as u32;
        (*host).randomSeed = (*host).randomSeed << 16_i32 | (*host).randomSeed >> 16_i32;
    }
    (*host).channelLimit = channelLimit;
    (*host).incomingBandwidth = incomingBandwidth;
    (*host).outgoingBandwidth = outgoingBandwidth;
    (*host).bandwidthThrottleEpoch = 0_i32 as u32;
    (*host).recalculateBandwidthLimits = 0_i32;
    (*host).mtu = ENET_HOST_DEFAULT_MTU as i32 as u32;
    (*host).peerCount = peerCount;
    (*host).commandCount = 0_i32 as usize;
    (*host).bufferCount = 0_i32 as usize;
    (*host).checksum.write(None);
    (*host).receivedAddress.write(None);
    (*host).receivedData = std::ptr::null_mut();
    (*host).receivedDataLength = 0_i32 as usize;
    (*host).totalSentData = 0_i32 as u32;
    (*host).totalSentPackets = 0_i32 as u32;
    (*host).totalReceivedData = 0_i32 as u32;
    (*host).totalReceivedPackets = 0_i32 as u32;
    (*host).totalQueued = 0_i32 as u32;
    (*host).connectedPeers = 0_i32 as usize;
    (*host).bandwidthLimitedPeers = 0_i32 as usize;
    (*host).duplicatePeers = ENET_PROTOCOL_MAXIMUM_PEER_ID as i32 as usize;
    (*host).maximumPacketSize = ENET_HOST_DEFAULT_MAXIMUM_PACKET_SIZE as i32 as usize;
    (*host).maximumWaitingData = ENET_HOST_DEFAULT_MAXIMUM_WAITING_DATA as i32 as usize;
    (*host).compressor.write(None);
    (*host).intercept = None;
    enet_list_clear(&mut (*host).dispatchQueue);
    currentPeer = (*host).peers;
    while currentPeer < &mut *((*host).peers).add((*host).peerCount) as *mut ENetPeer<S> {
        (*currentPeer).host = host;
        (*currentPeer).incomingPeerID = currentPeer.offset_from((*host).peers) as i64 as u16;
        (*currentPeer).incomingSessionID = 0xff_i32 as u8;
        (*currentPeer).outgoingSessionID = (*currentPeer).incomingSessionID;
        (*currentPeer).address.write(None);
        (*currentPeer).data = std::ptr::null_mut();
        enet_list_clear(&mut (*currentPeer).acknowledgements);
        enet_list_clear(&mut (*currentPeer).sentReliableCommands);
        enet_list_clear(&mut (*currentPeer).outgoingCommands);
        enet_list_clear(&mut (*currentPeer).outgoingSendReliableCommands);
        enet_list_clear(&mut (*currentPeer).dispatchedCommands);
        enet_peer_reset(currentPeer);
        currentPeer = currentPeer.offset(1);
    }
    host
}
pub(crate) unsafe fn enet_host_destroy<S: Socket>(host: *mut ENetHost<S>) {
    let mut currentPeer: *mut ENetPeer<S>;
    if host.is_null() {
        return;
    }
    (*host).socket.assume_init_drop();
    currentPeer = (*host).peers;
    while currentPeer < &mut *((*host).peers).add((*host).peerCount) as *mut ENetPeer<S> {
        enet_peer_reset(currentPeer);
        (*currentPeer).address.assume_init_drop();
        currentPeer = currentPeer.offset(1);
    }
    (*host).checksum.assume_init_drop();
    (*host).time.assume_init_drop();
    (*host).compressor.assume_init_drop();
    (*host).receivedAddress.assume_init_drop();
    enet_free((*host).peers as *mut c_void);
    enet_free(host as *mut c_void);
}
pub(crate) unsafe fn enet_host_random<S: Socket>(host: *mut ENetHost<S>) -> u32 {
    (*host).randomSeed = (*host).randomSeed.wrapping_add(0x6d2b79f5_u32);
    let mut n: u32 = (*host).randomSeed;
    n = (n ^ n >> 15_i32).wrapping_mul(n | 1_u32);
    n ^= n.wrapping_add((n ^ n >> 7_i32).wrapping_mul(n | 61_u32));
    n ^ n >> 14_i32
}
pub(crate) unsafe fn enet_host_connect<S: Socket>(
    host: *mut ENetHost<S>,
    address: S::PeerAddress,
    mut channelCount: usize,
    data: u32,
) -> *mut ENetPeer<S> {
    let mut currentPeer: *mut ENetPeer<S>;
    let mut channel: *mut ENetChannel;
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    if channelCount < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as i32 as usize {
        channelCount = ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as i32 as usize;
    } else if channelCount > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as i32 as usize {
        channelCount = ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as i32 as usize;
    }
    currentPeer = (*host).peers;
    while currentPeer < &mut *((*host).peers).add((*host).peerCount) as *mut ENetPeer<S> {
        if (*currentPeer).state == ENET_PEER_STATE_DISCONNECTED as i32 as u32 {
            break;
        }
        currentPeer = currentPeer.offset(1);
    }
    if currentPeer >= &mut *((*host).peers).add((*host).peerCount) as *mut ENetPeer<S> {
        return std::ptr::null_mut();
    }
    (*currentPeer).channels =
        enet_malloc(channelCount.wrapping_mul(::core::mem::size_of::<ENetChannel>()))
            as *mut ENetChannel;
    if ((*currentPeer).channels).is_null() {
        return std::ptr::null_mut();
    }
    (*currentPeer).channelCount = channelCount;
    (*currentPeer).state = ENET_PEER_STATE_CONNECTING;
    *(*currentPeer).address.assume_init_mut() = Some(address);
    (*currentPeer).connectID = enet_host_random(host);
    (*currentPeer).mtu = (*host).mtu;
    if (*host).outgoingBandwidth == 0_i32 as u32 {
        (*currentPeer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    } else {
        (*currentPeer).windowSize = ((*host).outgoingBandwidth)
            .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as i32 as u32)
            .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32);
    }
    if (*currentPeer).windowSize < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32 {
        (*currentPeer).windowSize = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32;
    } else if (*currentPeer).windowSize > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32 {
        (*currentPeer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    }
    channel = (*currentPeer).channels;
    while channel < &mut *((*currentPeer).channels).add(channelCount) as *mut ENetChannel {
        (*channel).outgoingReliableSequenceNumber = 0_i32 as u16;
        (*channel).outgoingUnreliableSequenceNumber = 0_i32 as u16;
        (*channel).incomingReliableSequenceNumber = 0_i32 as u16;
        (*channel).incomingUnreliableSequenceNumber = 0_i32 as u16;
        enet_list_clear(&mut (*channel).incomingReliableCommands);
        enet_list_clear(&mut (*channel).incomingUnreliableCommands);
        (*channel).usedReliableWindows = 0_i32 as u16;
        _enet_memset(
            ((*channel).reliableWindows).as_mut_ptr() as *mut c_void,
            0_i32,
            ::core::mem::size_of::<[u16; 16]>(),
        );
        channel = channel.offset(1);
    }
    command.header.command = (ENET_PROTOCOL_COMMAND_CONNECT as i32
        | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32) as u8;
    command.header.channelID = 0xff_i32 as u8;
    command.connect.outgoingPeerID = (*currentPeer).incomingPeerID.to_be();
    command.connect.incomingSessionID = (*currentPeer).incomingSessionID;
    command.connect.outgoingSessionID = (*currentPeer).outgoingSessionID;
    command.connect.mtu = (*currentPeer).mtu.to_be();
    command.connect.windowSize = (*currentPeer).windowSize.to_be();
    command.connect.channelCount = (channelCount as u32).to_be();
    command.connect.incomingBandwidth = (*host).incomingBandwidth.to_be();
    command.connect.outgoingBandwidth = (*host).outgoingBandwidth.to_be();
    command.connect.packetThrottleInterval = (*currentPeer).packetThrottleInterval.to_be();
    command.connect.packetThrottleAcceleration = (*currentPeer).packetThrottleAcceleration.to_be();
    command.connect.packetThrottleDeceleration = (*currentPeer).packetThrottleDeceleration.to_be();
    command.connect.connectID = (*currentPeer).connectID;
    command.connect.data = data.to_be();
    enet_peer_queue_outgoing_command(
        currentPeer,
        &command,
        std::ptr::null_mut(),
        0_i32 as u32,
        0_i32 as u16,
    );
    currentPeer
}
pub(crate) unsafe fn enet_host_broadcast<S: Socket>(
    host: *mut ENetHost<S>,
    channelID: u8,
    packet: *mut ENetPacket,
) {
    let mut currentPeer: *mut ENetPeer<S>;
    currentPeer = (*host).peers;
    while currentPeer < &mut *((*host).peers).add((*host).peerCount) as *mut ENetPeer<S> {
        if (*currentPeer).state == ENET_PEER_STATE_CONNECTED as i32 as u32 {
            enet_peer_send(currentPeer, channelID, packet);
        }
        currentPeer = currentPeer.offset(1);
    }
    if (*packet).referenceCount == 0_i32 as usize {
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
    mut channelLimit: usize,
) {
    if channelLimit == 0 || channelLimit > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as i32 as usize {
        channelLimit = ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as i32 as usize;
    } else if channelLimit < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as i32 as usize {
        channelLimit = ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as i32 as usize;
    }
    (*host).channelLimit = channelLimit;
}
pub(crate) unsafe fn enet_host_bandwidth_limit<S: Socket>(
    host: *mut ENetHost<S>,
    incomingBandwidth: u32,
    outgoingBandwidth: u32,
) {
    (*host).incomingBandwidth = incomingBandwidth;
    (*host).outgoingBandwidth = outgoingBandwidth;
    (*host).recalculateBandwidthLimits = 1_i32;
}
pub(crate) unsafe fn enet_host_bandwidth_throttle<S: Socket>(host: *mut ENetHost<S>) {
    let timeCurrent: u32 = enet_time_get(host);
    let elapsedTime: u32 = timeCurrent.wrapping_sub((*host).bandwidthThrottleEpoch);
    let mut peersRemaining: u32 = (*host).connectedPeers as u32;
    let mut dataTotal: u32 = !0_i32 as u32;
    let mut bandwidth: u32 = !0_i32 as u32;
    let mut throttle: u32;
    let mut bandwidthLimit: u32 = 0_i32 as u32;
    let mut needsAdjustment: i32 = if (*host).bandwidthLimitedPeers > 0_i32 as usize {
        1_i32
    } else {
        0_i32
    };
    let mut peer: *mut ENetPeer<S>;
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    if elapsedTime < ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL as i32 as u32 {
        return;
    }
    (*host).bandwidthThrottleEpoch = timeCurrent;
    if peersRemaining == 0_i32 as u32 {
        return;
    }
    if (*host).outgoingBandwidth != 0_i32 as u32 {
        dataTotal = 0_i32 as u32;
        bandwidth = ((*host).outgoingBandwidth)
            .wrapping_mul(elapsedTime)
            .wrapping_div(1000_i32 as u32);
        peer = (*host).peers;
        while peer < &mut *((*host).peers).add((*host).peerCount) as *mut ENetPeer<S> {
            if !((*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
                && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32)
            {
                dataTotal = dataTotal.wrapping_add((*peer).outgoingDataTotal);
            }
            peer = peer.offset(1);
        }
    }
    while peersRemaining > 0_i32 as u32 && needsAdjustment != 0_i32 {
        needsAdjustment = 0_i32;
        if dataTotal <= bandwidth {
            throttle = ENET_PEER_PACKET_THROTTLE_SCALE as i32 as u32;
        } else {
            throttle = bandwidth
                .wrapping_mul(ENET_PEER_PACKET_THROTTLE_SCALE as i32 as u32)
                .wrapping_div(dataTotal);
        }
        peer = (*host).peers;
        while peer < &mut *((*host).peers).add((*host).peerCount) as *mut ENetPeer<S> {
            let peerBandwidth: u32;
            if !((*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
                && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
                || (*peer).incomingBandwidth == 0_i32 as u32
                || (*peer).outgoingBandwidthThrottleEpoch == timeCurrent)
            {
                peerBandwidth = ((*peer).incomingBandwidth)
                    .wrapping_mul(elapsedTime)
                    .wrapping_div(1000_i32 as u32);
                if throttle
                    .wrapping_mul((*peer).outgoingDataTotal)
                    .wrapping_div(ENET_PEER_PACKET_THROTTLE_SCALE as i32 as u32)
                    > peerBandwidth
                {
                    (*peer).packetThrottleLimit = peerBandwidth
                        .wrapping_mul(ENET_PEER_PACKET_THROTTLE_SCALE as i32 as u32)
                        .wrapping_div((*peer).outgoingDataTotal);
                    if (*peer).packetThrottleLimit == 0_i32 as u32 {
                        (*peer).packetThrottleLimit = 1_i32 as u32;
                    }
                    if (*peer).packetThrottle > (*peer).packetThrottleLimit {
                        (*peer).packetThrottle = (*peer).packetThrottleLimit;
                    }
                    (*peer).outgoingBandwidthThrottleEpoch = timeCurrent;
                    (*peer).incomingDataTotal = 0_i32 as u32;
                    (*peer).outgoingDataTotal = 0_i32 as u32;
                    needsAdjustment = 1_i32;
                    peersRemaining = peersRemaining.wrapping_sub(1);
                    bandwidth = bandwidth.wrapping_sub(peerBandwidth);
                    dataTotal = dataTotal.wrapping_sub(peerBandwidth);
                }
            }
            peer = peer.offset(1);
        }
    }
    if peersRemaining > 0_i32 as u32 {
        if dataTotal <= bandwidth {
            throttle = ENET_PEER_PACKET_THROTTLE_SCALE as i32 as u32;
        } else {
            throttle = bandwidth
                .wrapping_mul(ENET_PEER_PACKET_THROTTLE_SCALE as i32 as u32)
                .wrapping_div(dataTotal);
        }
        peer = (*host).peers;
        while peer < &mut *((*host).peers).add((*host).peerCount) as *mut ENetPeer<S> {
            if !((*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
                && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
                || (*peer).outgoingBandwidthThrottleEpoch == timeCurrent)
            {
                (*peer).packetThrottleLimit = throttle;
                if (*peer).packetThrottle > (*peer).packetThrottleLimit {
                    (*peer).packetThrottle = (*peer).packetThrottleLimit;
                }
                (*peer).incomingDataTotal = 0_i32 as u32;
                (*peer).outgoingDataTotal = 0_i32 as u32;
            }
            peer = peer.offset(1);
        }
    }
    if (*host).recalculateBandwidthLimits != 0 {
        (*host).recalculateBandwidthLimits = 0_i32;
        peersRemaining = (*host).connectedPeers as u32;
        bandwidth = (*host).incomingBandwidth;
        needsAdjustment = 1_i32;
        if bandwidth == 0_i32 as u32 {
            bandwidthLimit = 0_i32 as u32;
        } else {
            while peersRemaining > 0_i32 as u32 && needsAdjustment != 0_i32 {
                needsAdjustment = 0_i32;
                bandwidthLimit = bandwidth.wrapping_div(peersRemaining);
                peer = (*host).peers;
                while peer < &mut *((*host).peers).add((*host).peerCount) as *mut ENetPeer<S> {
                    if !((*peer).incomingBandwidthThrottleEpoch == timeCurrent
                        || (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
                            && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
                        || (*peer).outgoingBandwidth > 0_i32 as u32
                            && (*peer).outgoingBandwidth >= bandwidthLimit)
                    {
                        (*peer).incomingBandwidthThrottleEpoch = timeCurrent;
                        needsAdjustment = 1_i32;
                        peersRemaining = peersRemaining.wrapping_sub(1);
                        bandwidth = bandwidth.wrapping_sub((*peer).outgoingBandwidth);
                    }
                    peer = peer.offset(1);
                }
            }
        }
        peer = (*host).peers;
        while peer < &mut *((*host).peers).add((*host).peerCount) as *mut ENetPeer<S> {
            if !((*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
                && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32)
            {
                command.header.command = (ENET_PROTOCOL_COMMAND_BANDWIDTH_LIMIT as i32
                    | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32)
                    as u8;
                command.header.channelID = 0xff_i32 as u8;
                command.bandwidthLimit.outgoingBandwidth = (*host).outgoingBandwidth.to_be();
                if (*peer).incomingBandwidthThrottleEpoch == timeCurrent {
                    command.bandwidthLimit.incomingBandwidth = (*peer).outgoingBandwidth.to_be();
                } else {
                    command.bandwidthLimit.incomingBandwidth = bandwidthLimit.to_be();
                }
                enet_peer_queue_outgoing_command(
                    peer,
                    &command,
                    std::ptr::null_mut(),
                    0_i32 as u32,
                    0_i32 as u16,
                );
            }
            peer = peer.offset(1);
        }
    }
}
