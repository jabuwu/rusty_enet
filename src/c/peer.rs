use std::mem::MaybeUninit;

use crate::{
    os::{_enet_memset, c_void},
    ENetChannel, ENetList, ENetListNode, Socket, _ENetHost,
    c::{
        _ENetListNode, enet_list_insert, enet_malloc, enet_packet_create,
        ENET_PROTOCOL_COMMAND_MASK,
    },
    consts::*,
    ENetIncomingCommand, ENetListIterator, ENetOutgoingCommand, ENetPacket, ENetProtocol,
    ENetProtocolCommandHeader, ENetProtocolHeader, ENetProtocolSendFragment, _ENetProtocol,
    enet_free, enet_host_flush, enet_list_clear, enet_list_move, enet_list_remove,
    enet_packet_destroy, enet_protocol_command_size, ENetAcknowledgement, ENetProtocolAcknowledge,
    ENET_PACKET_FLAG_RELIABLE, ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT, ENET_PACKET_FLAG_UNSEQUENCED,
    ENET_PROTOCOL_COMMAND_DISCONNECT, ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE,
    ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED, ENET_PROTOCOL_COMMAND_PING,
    ENET_PROTOCOL_COMMAND_SEND_FRAGMENT, ENET_PROTOCOL_COMMAND_SEND_RELIABLE,
    ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE, ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT,
    ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED, ENET_PROTOCOL_COMMAND_THROTTLE_CONFIGURE,
};

pub(crate) type ENetPeerState = _ENetPeerState;
pub(crate) type _ENetPeerState = u32;
pub(crate) const ENET_PEER_STATE_ZOMBIE: _ENetPeerState = 9;
pub(crate) const ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT: _ENetPeerState = 8;
pub(crate) const ENET_PEER_STATE_DISCONNECTING: _ENetPeerState = 7;
pub(crate) const ENET_PEER_STATE_DISCONNECT_LATER: _ENetPeerState = 6;
pub(crate) const ENET_PEER_STATE_CONNECTED: _ENetPeerState = 5;
pub(crate) const ENET_PEER_STATE_CONNECTION_SUCCEEDED: _ENetPeerState = 4;
pub(crate) const ENET_PEER_STATE_CONNECTION_PENDING: _ENetPeerState = 3;
pub(crate) const ENET_PEER_STATE_ACKNOWLEDGING_CONNECT: _ENetPeerState = 2;
pub(crate) const ENET_PEER_STATE_CONNECTING: _ENetPeerState = 1;
pub(crate) const ENET_PEER_STATE_DISCONNECTED: _ENetPeerState = 0;
pub(crate) type _ENetPeerFlag = u32;
pub(crate) const ENET_PEER_FLAG_CONTINUE_SENDING: _ENetPeerFlag = 2;
pub(crate) const ENET_PEER_FLAG_NEEDS_DISPATCH: _ENetPeerFlag = 1;
pub(crate) type ENetPeer<S> = _ENetPeer<S>;
#[repr(C)]
pub(crate) struct _ENetPeer<S: Socket> {
    pub(crate) dispatchList: ENetListNode,
    pub(crate) host: *mut _ENetHost<S>,
    pub(crate) outgoingPeerID: u16,
    pub(crate) incomingPeerID: u16,
    pub(crate) connectID: u32,
    pub(crate) outgoingSessionID: u8,
    pub(crate) incomingSessionID: u8,
    pub(crate) address: MaybeUninit<Option<S::PeerAddress>>,
    pub(crate) data: *mut c_void,
    pub(crate) state: ENetPeerState,
    pub(crate) channels: *mut ENetChannel,
    pub(crate) channelCount: usize,
    pub(crate) incomingBandwidth: u32,
    pub(crate) outgoingBandwidth: u32,
    pub(crate) incomingBandwidthThrottleEpoch: u32,
    pub(crate) outgoingBandwidthThrottleEpoch: u32,
    pub(crate) incomingDataTotal: u32,
    pub(crate) outgoingDataTotal: u32,
    pub(crate) lastSendTime: u32,
    pub(crate) lastReceiveTime: u32,
    pub(crate) nextTimeout: u32,
    pub(crate) earliestTimeout: u32,
    pub(crate) packetLossEpoch: u32,
    pub(crate) packetsSent: u32,
    pub(crate) packetsLost: u32,
    pub(crate) packetLoss: u32,
    pub(crate) packetLossVariance: u32,
    pub(crate) packetThrottle: u32,
    pub(crate) packetThrottleLimit: u32,
    pub(crate) packetThrottleCounter: u32,
    pub(crate) packetThrottleEpoch: u32,
    pub(crate) packetThrottleAcceleration: u32,
    pub(crate) packetThrottleDeceleration: u32,
    pub(crate) packetThrottleInterval: u32,
    pub(crate) pingInterval: u32,
    pub(crate) timeoutLimit: u32,
    pub(crate) timeoutMinimum: u32,
    pub(crate) timeoutMaximum: u32,
    pub(crate) lastRoundTripTime: u32,
    pub(crate) lowestRoundTripTime: u32,
    pub(crate) lastRoundTripTimeVariance: u32,
    pub(crate) highestRoundTripTimeVariance: u32,
    pub(crate) roundTripTime: u32,
    pub(crate) roundTripTimeVariance: u32,
    pub(crate) mtu: u32,
    pub(crate) windowSize: u32,
    pub(crate) reliableDataInTransit: u32,
    pub(crate) outgoingReliableSequenceNumber: u16,
    pub(crate) acknowledgements: ENetList,
    pub(crate) sentReliableCommands: ENetList,
    pub(crate) outgoingSendReliableCommands: ENetList,
    pub(crate) outgoingCommands: ENetList,
    pub(crate) dispatchedCommands: ENetList,
    pub(crate) flags: u16,
    pub(crate) reserved: u16,
    pub(crate) incomingUnsequencedGroup: u16,
    pub(crate) outgoingUnsequencedGroup: u16,
    pub(crate) unsequencedWindow: [u32; 32],
    pub(crate) eventData: u32,
    pub(crate) totalWaitingData: usize,
}
pub(crate) unsafe fn enet_peer_throttle_configure<S: Socket>(
    peer: *mut ENetPeer<S>,
    interval: u32,
    acceleration: u32,
    deceleration: u32,
) {
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    (*peer).packetThrottleInterval = interval;
    (*peer).packetThrottleAcceleration = acceleration;
    (*peer).packetThrottleDeceleration = deceleration;
    command.header.command = (ENET_PROTOCOL_COMMAND_THROTTLE_CONFIGURE as i32
        | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32) as u8;
    command.header.channelID = 0xff_i32 as u8;
    command.throttleConfigure.packetThrottleInterval = interval.to_be();
    command.throttleConfigure.packetThrottleAcceleration = acceleration.to_be();
    command.throttleConfigure.packetThrottleDeceleration = deceleration.to_be();
    enet_peer_queue_outgoing_command(
        peer,
        &command,
        std::ptr::null_mut(),
        0_i32 as u32,
        0_i32 as u16,
    );
}
pub(crate) unsafe fn enet_peer_throttle<S: Socket>(peer: *mut ENetPeer<S>, rtt: u32) -> i32 {
    if (*peer).lastRoundTripTime <= (*peer).lastRoundTripTimeVariance {
        (*peer).packetThrottle = (*peer).packetThrottleLimit;
    } else if rtt <= (*peer).lastRoundTripTime {
        (*peer).packetThrottle = (*peer)
            .packetThrottle
            .wrapping_add((*peer).packetThrottleAcceleration);
        if (*peer).packetThrottle > (*peer).packetThrottleLimit {
            (*peer).packetThrottle = (*peer).packetThrottleLimit;
        }
        return 1_i32;
    } else if rtt
        > ((*peer).lastRoundTripTime)
            .wrapping_add((2_i32 as u32).wrapping_mul((*peer).lastRoundTripTimeVariance))
    {
        if (*peer).packetThrottle > (*peer).packetThrottleDeceleration {
            (*peer).packetThrottle = (*peer)
                .packetThrottle
                .wrapping_sub((*peer).packetThrottleDeceleration);
        } else {
            (*peer).packetThrottle = 0_i32 as u32;
        }
        return -1_i32;
    }
    0_i32
}
pub(crate) unsafe fn enet_peer_send<S: Socket>(
    peer: *mut ENetPeer<S>,
    channelID: u8,
    packet: *mut ENetPacket,
) -> i32 {
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    let mut fragmentLength: usize;
    if (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
        || channelID as usize >= (*peer).channelCount
        || (*packet).dataLength > (*(*peer).host).maximumPacketSize
    {
        return -1_i32;
    }
    let channel = &mut *((*peer).channels).offset(channelID as isize) as *mut ENetChannel;
    fragmentLength = ((*peer).mtu as usize)
        .wrapping_sub(::core::mem::size_of::<ENetProtocolHeader>())
        .wrapping_sub(::core::mem::size_of::<ENetProtocolSendFragment>());
    if ((*(*peer).host).checksum.assume_init_ref()).is_some() {
        fragmentLength =
            (fragmentLength as u64).wrapping_sub(::core::mem::size_of::<u32>() as u64) as usize;
    }
    if (*packet).dataLength > fragmentLength {
        let fragmentCount: u32 = ((*packet).dataLength)
            .wrapping_add(fragmentLength)
            .wrapping_sub(1_i32 as usize)
            .wrapping_div(fragmentLength) as u32;
        let mut fragmentNumber: u32;
        let mut fragmentOffset: u32;
        let commandNumber: u8;
        let startSequenceNumber: u16;
        let mut fragments: ENetList = ENetList {
            sentinel: ENetListNode {
                next: std::ptr::null_mut(),
                previous: std::ptr::null_mut(),
            },
        };
        let mut fragment: *mut ENetOutgoingCommand;
        if fragmentCount > ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as i32 as u32 {
            return -1_i32;
        }
        if (*packet).flags
            & (ENET_PACKET_FLAG_RELIABLE as i32 | ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT as i32)
                as u32
            == ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT as i32 as u32
            && ((*channel).outgoingUnreliableSequenceNumber as i32) < 0xffff_i32
        {
            commandNumber = ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT as i32 as u8;
            startSequenceNumber =
                (((*channel).outgoingUnreliableSequenceNumber as i32 + 1_i32) as u16).to_be();
        } else {
            commandNumber = (ENET_PROTOCOL_COMMAND_SEND_FRAGMENT as i32
                | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32) as u8;
            startSequenceNumber =
                (((*channel).outgoingReliableSequenceNumber as i32 + 1_i32) as u16).to_be();
        }
        enet_list_clear(&mut fragments);
        fragmentNumber = 0_i32 as u32;
        fragmentOffset = 0_i32 as u32;
        while (fragmentOffset as usize) < (*packet).dataLength {
            if ((*packet).dataLength).wrapping_sub(fragmentOffset as usize) < fragmentLength {
                fragmentLength = ((*packet).dataLength).wrapping_sub(fragmentOffset as usize);
            }
            fragment = enet_malloc(::core::mem::size_of::<ENetOutgoingCommand>())
                as *mut ENetOutgoingCommand;
            if fragment.is_null() {
                while fragments.sentinel.next != &mut fragments.sentinel as *mut ENetListNode {
                    fragment =
                        enet_list_remove(fragments.sentinel.next) as *mut ENetOutgoingCommand;
                    enet_free(fragment as *mut c_void);
                }
                return -1_i32;
            }
            (*fragment).fragmentOffset = fragmentOffset;
            (*fragment).fragmentLength = fragmentLength as u16;
            (*fragment).packet = packet;
            (*fragment).command.header.command = commandNumber;
            (*fragment).command.header.channelID = channelID;
            (*fragment).command.sendFragment.startSequenceNumber = startSequenceNumber;
            (*fragment).command.sendFragment.dataLength = (fragmentLength as u16).to_be();
            (*fragment).command.sendFragment.fragmentCount = fragmentCount.to_be();
            (*fragment).command.sendFragment.fragmentNumber = fragmentNumber.to_be();
            (*fragment).command.sendFragment.totalLength = ((*packet).dataLength as u32).to_be();
            (*fragment).command.sendFragment.fragmentOffset = u32::from_be(fragmentOffset);
            enet_list_insert(&mut fragments.sentinel, fragment as *mut c_void);
            fragmentNumber = fragmentNumber.wrapping_add(1);
            fragmentOffset = (fragmentOffset as usize).wrapping_add(fragmentLength) as u32;
        }
        (*packet).referenceCount =
            ((*packet).referenceCount as u64).wrapping_add(fragmentNumber as u64) as usize;
        while fragments.sentinel.next != &mut fragments.sentinel as *mut ENetListNode {
            fragment = enet_list_remove(fragments.sentinel.next) as *mut ENetOutgoingCommand;
            enet_peer_setup_outgoing_command(peer, fragment);
        }
        return 0_i32;
    }
    command.header.channelID = channelID;
    if (*packet).flags
        & (ENET_PACKET_FLAG_RELIABLE as i32 | ENET_PACKET_FLAG_UNSEQUENCED as i32) as u32
        == ENET_PACKET_FLAG_UNSEQUENCED as i32 as u32
    {
        command.header.command = (ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as i32
            | ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED as i32) as u8;
        command.sendUnsequenced.dataLength = ((*packet).dataLength as u16).to_be();
    } else if (*packet).flags & ENET_PACKET_FLAG_RELIABLE as i32 as u32 != 0
        || (*channel).outgoingUnreliableSequenceNumber as i32 >= 0xffff_i32
    {
        command.header.command = (ENET_PROTOCOL_COMMAND_SEND_RELIABLE as i32
            | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32) as u8;
        command.sendReliable.dataLength = ((*packet).dataLength as u16).to_be();
    } else {
        command.header.command = ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE as i32 as u8;
        command.sendUnreliable.dataLength = ((*packet).dataLength as u16).to_be();
    }
    if (enet_peer_queue_outgoing_command(
        peer,
        &command,
        packet,
        0_i32 as u32,
        (*packet).dataLength as u16,
    ))
    .is_null()
    {
        return -1_i32;
    }
    0_i32
}
pub(crate) unsafe fn enet_peer_receive<S: Socket>(
    peer: *mut ENetPeer<S>,
    channelID: *mut u8,
) -> *mut ENetPacket {
    if (*peer).dispatchedCommands.sentinel.next
        == &mut (*peer).dispatchedCommands.sentinel as *mut ENetListNode
    {
        return std::ptr::null_mut();
    }
    let incomingCommand =
        enet_list_remove((*peer).dispatchedCommands.sentinel.next) as *mut ENetIncomingCommand;
    if !channelID.is_null() {
        *channelID = (*incomingCommand).command.header.channelID;
    }
    let packet = (*incomingCommand).packet;
    (*packet).referenceCount = ((*packet).referenceCount).wrapping_sub(1);
    if !((*incomingCommand).fragments).is_null() {
        enet_free((*incomingCommand).fragments as *mut c_void);
    }
    enet_free(incomingCommand as *mut c_void);
    (*peer).totalWaitingData =
        (*peer).totalWaitingData.wrapping_sub((*packet).dataLength) as usize as usize;
    packet
}
unsafe fn enet_peer_reset_outgoing_commands(queue: *mut ENetList) {
    let mut outgoingCommand: *mut ENetOutgoingCommand;
    while (*queue).sentinel.next != &mut (*queue).sentinel as *mut ENetListNode {
        outgoingCommand = enet_list_remove((*queue).sentinel.next) as *mut ENetOutgoingCommand;
        if !((*outgoingCommand).packet).is_null() {
            (*(*outgoingCommand).packet).referenceCount =
                ((*(*outgoingCommand).packet).referenceCount).wrapping_sub(1);
            if (*(*outgoingCommand).packet).referenceCount == 0_i32 as usize {
                enet_packet_destroy((*outgoingCommand).packet);
            }
        }
        enet_free(outgoingCommand as *mut c_void);
    }
}
unsafe fn enet_peer_remove_incoming_commands(
    mut _queue: *mut ENetList,
    startCommand: ENetListIterator,
    endCommand: ENetListIterator,
    excludeCommand: *mut ENetIncomingCommand,
) {
    let mut currentCommand: ENetListIterator;
    currentCommand = startCommand;
    while currentCommand != endCommand {
        let incomingCommand: *mut ENetIncomingCommand = currentCommand as *mut ENetIncomingCommand;
        currentCommand = (*currentCommand).next;
        if incomingCommand == excludeCommand {
            continue;
        }
        enet_list_remove(&mut (*incomingCommand).incomingCommandList);
        if !((*incomingCommand).packet).is_null() {
            (*(*incomingCommand).packet).referenceCount =
                ((*(*incomingCommand).packet).referenceCount).wrapping_sub(1);
            if (*(*incomingCommand).packet).referenceCount == 0_i32 as usize {
                enet_packet_destroy((*incomingCommand).packet);
            }
        }
        if !((*incomingCommand).fragments).is_null() {
            enet_free((*incomingCommand).fragments as *mut c_void);
        }
        enet_free(incomingCommand as *mut c_void);
    }
}
unsafe fn enet_peer_reset_incoming_commands(queue: *mut ENetList) {
    enet_peer_remove_incoming_commands(
        queue,
        (*queue).sentinel.next,
        &mut (*queue).sentinel,
        std::ptr::null_mut(),
    );
}
pub(crate) unsafe fn enet_peer_reset_queues<S: Socket>(peer: *mut ENetPeer<S>) {
    let mut channel: *mut ENetChannel;
    if (*peer).flags as i32 & ENET_PEER_FLAG_NEEDS_DISPATCH as i32 != 0 {
        enet_list_remove(&mut (*peer).dispatchList);
        (*peer).flags = ((*peer).flags as i32 & !(ENET_PEER_FLAG_NEEDS_DISPATCH as i32)) as u16;
    }
    while (*peer).acknowledgements.sentinel.next
        != &mut (*peer).acknowledgements.sentinel as *mut ENetListNode
    {
        enet_free(enet_list_remove((*peer).acknowledgements.sentinel.next));
    }
    enet_peer_reset_outgoing_commands(&mut (*peer).sentReliableCommands);
    enet_peer_reset_outgoing_commands(&mut (*peer).outgoingCommands);
    enet_peer_reset_outgoing_commands(&mut (*peer).outgoingSendReliableCommands);
    enet_peer_reset_incoming_commands(&mut (*peer).dispatchedCommands);
    if !((*peer).channels).is_null() && (*peer).channelCount > 0_i32 as usize {
        channel = (*peer).channels;
        while channel < &mut *((*peer).channels).add((*peer).channelCount) as *mut ENetChannel {
            enet_peer_reset_incoming_commands(&mut (*channel).incomingReliableCommands);
            enet_peer_reset_incoming_commands(&mut (*channel).incomingUnreliableCommands);
            channel = channel.offset(1);
        }
        enet_free((*peer).channels as *mut c_void);
    }
    (*peer).channels = std::ptr::null_mut();
    (*peer).channelCount = 0_i32 as usize;
}
pub(crate) unsafe fn enet_peer_on_connect<S: Socket>(peer: *mut ENetPeer<S>) {
    if (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
        && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        if (*peer).incomingBandwidth != 0_i32 as u32 {
            (*(*peer).host).bandwidthLimitedPeers =
                ((*(*peer).host).bandwidthLimitedPeers).wrapping_add(1);
        }
        (*(*peer).host).connectedPeers = ((*(*peer).host).connectedPeers).wrapping_add(1);
    }
}
pub(crate) unsafe fn enet_peer_on_disconnect<S: Socket>(peer: *mut ENetPeer<S>) {
    if (*peer).state == ENET_PEER_STATE_CONNECTED as i32 as u32
        || (*peer).state == ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        if (*peer).incomingBandwidth != 0_i32 as u32 {
            (*(*peer).host).bandwidthLimitedPeers =
                ((*(*peer).host).bandwidthLimitedPeers).wrapping_sub(1);
        }
        (*(*peer).host).connectedPeers = ((*(*peer).host).connectedPeers).wrapping_sub(1);
    }
}
pub(crate) unsafe fn enet_peer_reset<S: Socket>(peer: *mut ENetPeer<S>) {
    enet_peer_on_disconnect(peer);
    (*peer).outgoingPeerID = ENET_PROTOCOL_MAXIMUM_PEER_ID as i32 as u16;
    (*peer).connectID = 0_i32 as u32;
    (*peer).state = ENET_PEER_STATE_DISCONNECTED;
    (*peer).incomingBandwidth = 0_i32 as u32;
    (*peer).outgoingBandwidth = 0_i32 as u32;
    (*peer).incomingBandwidthThrottleEpoch = 0_i32 as u32;
    (*peer).outgoingBandwidthThrottleEpoch = 0_i32 as u32;
    (*peer).incomingDataTotal = 0_i32 as u32;
    (*peer).outgoingDataTotal = 0_i32 as u32;
    (*peer).lastSendTime = 0_i32 as u32;
    (*peer).lastReceiveTime = 0_i32 as u32;
    (*peer).nextTimeout = 0_i32 as u32;
    (*peer).earliestTimeout = 0_i32 as u32;
    (*peer).packetLossEpoch = 0_i32 as u32;
    (*peer).packetsSent = 0_i32 as u32;
    (*peer).packetsLost = 0_i32 as u32;
    (*peer).packetLoss = 0_i32 as u32;
    (*peer).packetLossVariance = 0_i32 as u32;
    (*peer).packetThrottle = ENET_PEER_DEFAULT_PACKET_THROTTLE as i32 as u32;
    (*peer).packetThrottleLimit = ENET_PEER_PACKET_THROTTLE_SCALE as i32 as u32;
    (*peer).packetThrottleCounter = 0_i32 as u32;
    (*peer).packetThrottleEpoch = 0_i32 as u32;
    (*peer).packetThrottleAcceleration = ENET_PEER_PACKET_THROTTLE_ACCELERATION as i32 as u32;
    (*peer).packetThrottleDeceleration = ENET_PEER_PACKET_THROTTLE_DECELERATION as i32 as u32;
    (*peer).packetThrottleInterval = ENET_PEER_PACKET_THROTTLE_INTERVAL as i32 as u32;
    (*peer).pingInterval = ENET_PEER_PING_INTERVAL as i32 as u32;
    (*peer).timeoutLimit = ENET_PEER_TIMEOUT_LIMIT as i32 as u32;
    (*peer).timeoutMinimum = ENET_PEER_TIMEOUT_MINIMUM as i32 as u32;
    (*peer).timeoutMaximum = ENET_PEER_TIMEOUT_MAXIMUM as i32 as u32;
    (*peer).lastRoundTripTime = ENET_PEER_DEFAULT_ROUND_TRIP_TIME as i32 as u32;
    (*peer).lowestRoundTripTime = ENET_PEER_DEFAULT_ROUND_TRIP_TIME as i32 as u32;
    (*peer).lastRoundTripTimeVariance = 0_i32 as u32;
    (*peer).highestRoundTripTimeVariance = 0_i32 as u32;
    (*peer).roundTripTime = ENET_PEER_DEFAULT_ROUND_TRIP_TIME as i32 as u32;
    (*peer).roundTripTimeVariance = 0_i32 as u32;
    (*peer).mtu = (*(*peer).host).mtu;
    (*peer).reliableDataInTransit = 0_i32 as u32;
    (*peer).outgoingReliableSequenceNumber = 0_i32 as u16;
    (*peer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    (*peer).incomingUnsequencedGroup = 0_i32 as u16;
    (*peer).outgoingUnsequencedGroup = 0_i32 as u16;
    (*peer).eventData = 0_i32 as u32;
    (*peer).totalWaitingData = 0_i32 as usize;
    (*peer).flags = 0_i32 as u16;
    _enet_memset(
        ((*peer).unsequencedWindow).as_mut_ptr() as *mut c_void,
        0_i32,
        ::core::mem::size_of::<[u32; 32]>(),
    );
    enet_peer_reset_queues(peer);
}
pub(crate) unsafe fn enet_peer_ping<S: Socket>(peer: *mut ENetPeer<S>) {
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    if (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32 {
        return;
    }
    command.header.command =
        (ENET_PROTOCOL_COMMAND_PING as i32 | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32) as u8;
    command.header.channelID = 0xff_i32 as u8;
    enet_peer_queue_outgoing_command(
        peer,
        &command,
        std::ptr::null_mut(),
        0_i32 as u32,
        0_i32 as u16,
    );
}
pub(crate) unsafe fn enet_peer_ping_interval<S: Socket>(peer: *mut ENetPeer<S>, pingInterval: u32) {
    (*peer).pingInterval = if pingInterval != 0 {
        pingInterval
    } else {
        ENET_PEER_PING_INTERVAL as i32 as u32
    };
}
pub(crate) unsafe fn enet_peer_timeout<S: Socket>(
    peer: *mut ENetPeer<S>,
    timeoutLimit: u32,
    timeoutMinimum: u32,
    timeoutMaximum: u32,
) {
    (*peer).timeoutLimit = if timeoutLimit != 0 {
        timeoutLimit
    } else {
        ENET_PEER_TIMEOUT_LIMIT as i32 as u32
    };
    (*peer).timeoutMinimum = if timeoutMinimum != 0 {
        timeoutMinimum
    } else {
        ENET_PEER_TIMEOUT_MINIMUM as i32 as u32
    };
    (*peer).timeoutMaximum = if timeoutMaximum != 0 {
        timeoutMaximum
    } else {
        ENET_PEER_TIMEOUT_MAXIMUM as i32 as u32
    };
}
pub(crate) unsafe fn enet_peer_disconnect_now<S: Socket>(peer: *mut ENetPeer<S>, data: u32) {
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    if (*peer).state == ENET_PEER_STATE_DISCONNECTED as i32 as u32 {
        return;
    }
    if (*peer).state != ENET_PEER_STATE_ZOMBIE as i32 as u32
        && (*peer).state != ENET_PEER_STATE_DISCONNECTING as i32 as u32
    {
        enet_peer_reset_queues(peer);
        command.header.command = (ENET_PROTOCOL_COMMAND_DISCONNECT as i32
            | ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED as i32) as u8;
        command.header.channelID = 0xff_i32 as u8;
        command.disconnect.data = data.to_be();
        enet_peer_queue_outgoing_command(
            peer,
            &command,
            std::ptr::null_mut(),
            0_i32 as u32,
            0_i32 as u16,
        );
        enet_host_flush((*peer).host);
    }
    enet_peer_reset(peer);
}
pub(crate) unsafe fn enet_peer_disconnect<S: Socket>(peer: *mut ENetPeer<S>, data: u32) {
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    if (*peer).state == ENET_PEER_STATE_DISCONNECTING as i32 as u32
        || (*peer).state == ENET_PEER_STATE_DISCONNECTED as i32 as u32
        || (*peer).state == ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT as i32 as u32
        || (*peer).state == ENET_PEER_STATE_ZOMBIE as i32 as u32
    {
        return;
    }
    enet_peer_reset_queues(peer);
    command.header.command = ENET_PROTOCOL_COMMAND_DISCONNECT as i32 as u8;
    command.header.channelID = 0xff_i32 as u8;
    command.disconnect.data = data.to_be();
    if (*peer).state == ENET_PEER_STATE_CONNECTED as i32 as u32
        || (*peer).state == ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        command.header.command =
            (command.header.command as i32 | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32) as u8;
    } else {
        command.header.command =
            (command.header.command as i32 | ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED as i32) as u8;
    }
    enet_peer_queue_outgoing_command(
        peer,
        &command,
        std::ptr::null_mut(),
        0_i32 as u32,
        0_i32 as u16,
    );
    if (*peer).state == ENET_PEER_STATE_CONNECTED as i32 as u32
        || (*peer).state == ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        enet_peer_on_disconnect(peer);
        (*peer).state = ENET_PEER_STATE_DISCONNECTING;
    } else {
        enet_host_flush((*peer).host);
        enet_peer_reset(peer);
    };
}
pub(crate) unsafe fn enet_peer_has_outgoing_commands<S: Socket>(peer: *mut ENetPeer<S>) -> i32 {
    if (*peer).outgoingCommands.sentinel.next
        == &mut (*peer).outgoingCommands.sentinel as *mut ENetListNode
        && (*peer).outgoingSendReliableCommands.sentinel.next
            == &mut (*peer).outgoingSendReliableCommands.sentinel as *mut ENetListNode
        && (*peer).sentReliableCommands.sentinel.next
            == &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode
    {
        return 0_i32;
    }
    1_i32
}
pub(crate) unsafe fn enet_peer_disconnect_later<S: Socket>(peer: *mut ENetPeer<S>, data: u32) {
    if ((*peer).state == ENET_PEER_STATE_CONNECTED as i32 as u32
        || (*peer).state == ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32)
        && enet_peer_has_outgoing_commands(peer) != 0
    {
        (*peer).state = ENET_PEER_STATE_DISCONNECT_LATER;
        (*peer).eventData = data;
    } else {
        enet_peer_disconnect(peer, data);
    };
}
pub(crate) unsafe fn enet_peer_queue_acknowledgement<S: Socket>(
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
    sentTime: u16,
) -> *mut ENetAcknowledgement {
    if ((*command).header.channelID as usize) < (*peer).channelCount {
        let channel: *mut ENetChannel = &mut *((*peer).channels)
            .offset((*command).header.channelID as isize)
            as *mut ENetChannel;
        let mut reliableWindow: u16 = ((*command).header.reliableSequenceNumber as i32
            / ENET_PEER_RELIABLE_WINDOW_SIZE as i32) as u16;
        let currentWindow: u16 = ((*channel).incomingReliableSequenceNumber as i32
            / ENET_PEER_RELIABLE_WINDOW_SIZE as i32) as u16;
        if ((*command).header.reliableSequenceNumber as i32)
            < (*channel).incomingReliableSequenceNumber as i32
        {
            reliableWindow = (reliableWindow as i32 + ENET_PEER_RELIABLE_WINDOWS as i32) as u16;
        }
        if reliableWindow as i32
            >= currentWindow as i32 + ENET_PEER_FREE_RELIABLE_WINDOWS as i32 - 1_i32
            && reliableWindow as i32
                <= currentWindow as i32 + ENET_PEER_FREE_RELIABLE_WINDOWS as i32
        {
            return std::ptr::null_mut();
        }
    }
    let acknowledgement =
        enet_malloc(::core::mem::size_of::<ENetAcknowledgement>()) as *mut ENetAcknowledgement;
    if acknowledgement.is_null() {
        return std::ptr::null_mut();
    }
    (*peer).outgoingDataTotal = ((*peer).outgoingDataTotal as u64)
        .wrapping_add(::core::mem::size_of::<ENetProtocolAcknowledge>() as u64)
        as u32 as u32;
    (*acknowledgement).sentTime = sentTime as u32;
    (*acknowledgement).command = *command;
    enet_list_insert(
        &mut (*peer).acknowledgements.sentinel,
        acknowledgement as *mut c_void,
    );
    acknowledgement
}
pub(crate) unsafe fn enet_peer_setup_outgoing_command<S: Socket>(
    peer: *mut ENetPeer<S>,
    outgoingCommand: *mut ENetOutgoingCommand,
) {
    (*peer).outgoingDataTotal = ((*peer).outgoingDataTotal as usize).wrapping_add(
        (enet_protocol_command_size((*outgoingCommand).command.header.command))
            .wrapping_add((*outgoingCommand).fragmentLength as usize),
    ) as u32 as u32;
    if (*outgoingCommand).command.header.channelID as i32 == 0xff_i32 {
        (*peer).outgoingReliableSequenceNumber =
            ((*peer).outgoingReliableSequenceNumber).wrapping_add(1);
        (*outgoingCommand).reliableSequenceNumber = (*peer).outgoingReliableSequenceNumber;
        (*outgoingCommand).unreliableSequenceNumber = 0_i32 as u16;
    } else {
        let channel: *mut ENetChannel = &mut *((*peer).channels)
            .offset((*outgoingCommand).command.header.channelID as isize)
            as *mut ENetChannel;
        if (*outgoingCommand).command.header.command as i32
            & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32
            != 0
        {
            (*channel).outgoingReliableSequenceNumber =
                ((*channel).outgoingReliableSequenceNumber).wrapping_add(1);
            (*channel).outgoingUnreliableSequenceNumber = 0_i32 as u16;
            (*outgoingCommand).reliableSequenceNumber = (*channel).outgoingReliableSequenceNumber;
            (*outgoingCommand).unreliableSequenceNumber = 0_i32 as u16;
        } else if (*outgoingCommand).command.header.command as i32
            & ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED as i32
            != 0
        {
            (*peer).outgoingUnsequencedGroup = ((*peer).outgoingUnsequencedGroup).wrapping_add(1);
            (*outgoingCommand).reliableSequenceNumber = 0_i32 as u16;
            (*outgoingCommand).unreliableSequenceNumber = 0_i32 as u16;
        } else {
            if (*outgoingCommand).fragmentOffset == 0_i32 as u32 {
                (*channel).outgoingUnreliableSequenceNumber =
                    ((*channel).outgoingUnreliableSequenceNumber).wrapping_add(1);
            }
            (*outgoingCommand).reliableSequenceNumber = (*channel).outgoingReliableSequenceNumber;
            (*outgoingCommand).unreliableSequenceNumber =
                (*channel).outgoingUnreliableSequenceNumber;
        }
    }
    (*outgoingCommand).sendAttempts = 0_i32 as u16;
    (*outgoingCommand).sentTime = 0_i32 as u32;
    (*outgoingCommand).roundTripTimeout = 0_i32 as u32;
    (*outgoingCommand).command.header.reliableSequenceNumber =
        (*outgoingCommand).reliableSequenceNumber.to_be();
    (*(*peer).host).totalQueued = ((*(*peer).host).totalQueued).wrapping_add(1);
    (*outgoingCommand).queueTime = (*(*peer).host).totalQueued;
    match (*outgoingCommand).command.header.command as i32 & ENET_PROTOCOL_COMMAND_MASK as i32 {
        7 => {
            (*outgoingCommand)
                .command
                .sendUnreliable
                .unreliableSequenceNumber = (*outgoingCommand).unreliableSequenceNumber.to_be();
        }
        9 => {
            (*outgoingCommand).command.sendUnsequenced.unsequencedGroup =
                (*peer).outgoingUnsequencedGroup.to_be();
        }
        _ => {}
    }
    if (*outgoingCommand).command.header.command as i32
        & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32
        != 0_i32
        && !((*outgoingCommand).packet).is_null()
    {
        enet_list_insert(
            &mut (*peer).outgoingSendReliableCommands.sentinel,
            outgoingCommand as *mut c_void,
        );
    } else {
        enet_list_insert(
            &mut (*peer).outgoingCommands.sentinel,
            outgoingCommand as *mut c_void,
        );
    };
}
pub(crate) unsafe fn enet_peer_queue_outgoing_command<S: Socket>(
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
    packet: *mut ENetPacket,
    offset: u32,
    length: u16,
) -> *mut ENetOutgoingCommand {
    let outgoingCommand: *mut ENetOutgoingCommand =
        enet_malloc(::core::mem::size_of::<ENetOutgoingCommand>()) as *mut ENetOutgoingCommand;
    if outgoingCommand.is_null() {
        return std::ptr::null_mut();
    }
    (*outgoingCommand).command = *command;
    (*outgoingCommand).fragmentOffset = offset;
    (*outgoingCommand).fragmentLength = length;
    (*outgoingCommand).packet = packet;
    if !packet.is_null() {
        (*packet).referenceCount = ((*packet).referenceCount).wrapping_add(1);
    }
    enet_peer_setup_outgoing_command(peer, outgoingCommand);
    outgoingCommand
}
pub(crate) unsafe fn enet_peer_dispatch_incoming_unreliable_commands<S: Socket>(
    peer: *mut ENetPeer<S>,
    channel: *mut ENetChannel,
    queuedCommand: *mut ENetIncomingCommand,
) {
    let mut droppedCommand: ENetListIterator;
    let mut startCommand: ENetListIterator;
    let mut currentCommand: ENetListIterator;
    let mut current_block_22: u64;
    currentCommand = (*channel).incomingUnreliableCommands.sentinel.next;
    startCommand = currentCommand;
    droppedCommand = startCommand;
    while currentCommand != &mut (*channel).incomingUnreliableCommands.sentinel as *mut ENetListNode
    {
        let incomingCommand: *mut ENetIncomingCommand = currentCommand as *mut ENetIncomingCommand;
        if (*incomingCommand).command.header.command as i32 & ENET_PROTOCOL_COMMAND_MASK as i32
            != ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as i32
        {
            if (*incomingCommand).reliableSequenceNumber as i32
                == (*channel).incomingReliableSequenceNumber as i32
            {
                if (*incomingCommand).fragmentsRemaining <= 0_i32 as u32 {
                    (*channel).incomingUnreliableSequenceNumber =
                        (*incomingCommand).unreliableSequenceNumber;
                    current_block_22 = 11174649648027449784;
                } else {
                    if startCommand != currentCommand {
                        enet_list_move(
                            &mut (*peer).dispatchedCommands.sentinel,
                            startCommand as *mut c_void,
                            (*currentCommand).previous as *mut c_void,
                        );
                        if (*peer).flags as i32 & ENET_PEER_FLAG_NEEDS_DISPATCH as i32 == 0 {
                            enet_list_insert(
                                &mut (*(*peer).host).dispatchQueue.sentinel,
                                &mut (*peer).dispatchList as *mut ENetListNode as *mut c_void,
                            );
                            (*peer).flags = ((*peer).flags as i32
                                | ENET_PEER_FLAG_NEEDS_DISPATCH as i32)
                                as u16;
                        }
                        droppedCommand = currentCommand;
                    } else if droppedCommand != currentCommand {
                        droppedCommand = (*currentCommand).previous;
                    }
                    current_block_22 = 13472856163611868459;
                }
            } else {
                let mut reliableWindow: u16 = ((*incomingCommand).reliableSequenceNumber as i32
                    / ENET_PEER_RELIABLE_WINDOW_SIZE as i32)
                    as u16;
                let currentWindow: u16 = ((*channel).incomingReliableSequenceNumber as i32
                    / ENET_PEER_RELIABLE_WINDOW_SIZE as i32)
                    as u16;
                if ((*incomingCommand).reliableSequenceNumber as i32)
                    < (*channel).incomingReliableSequenceNumber as i32
                {
                    reliableWindow =
                        (reliableWindow as i32 + ENET_PEER_RELIABLE_WINDOWS as i32) as u16;
                }
                if reliableWindow as i32 >= currentWindow as i32
                    && (reliableWindow as i32)
                        < currentWindow as i32 + ENET_PEER_FREE_RELIABLE_WINDOWS as i32 - 1_i32
                {
                    break;
                }
                droppedCommand = (*currentCommand).next;
                if startCommand != currentCommand {
                    enet_list_move(
                        &mut (*peer).dispatchedCommands.sentinel,
                        startCommand as *mut c_void,
                        (*currentCommand).previous as *mut c_void,
                    );
                    if (*peer).flags as i32 & ENET_PEER_FLAG_NEEDS_DISPATCH as i32 == 0 {
                        enet_list_insert(
                            &mut (*(*peer).host).dispatchQueue.sentinel,
                            &mut (*peer).dispatchList as *mut ENetListNode as *mut c_void,
                        );
                        (*peer).flags =
                            ((*peer).flags as i32 | ENET_PEER_FLAG_NEEDS_DISPATCH as i32) as u16;
                    }
                }
                current_block_22 = 13472856163611868459;
            }
            match current_block_22 {
                11174649648027449784 => {}
                _ => {
                    startCommand = (*currentCommand).next;
                }
            }
        }
        currentCommand = (*currentCommand).next;
    }
    if startCommand != currentCommand {
        enet_list_move(
            &mut (*peer).dispatchedCommands.sentinel,
            startCommand as *mut c_void,
            (*currentCommand).previous as *mut c_void,
        );
        if (*peer).flags as i32 & ENET_PEER_FLAG_NEEDS_DISPATCH as i32 == 0 {
            enet_list_insert(
                &mut (*(*peer).host).dispatchQueue.sentinel,
                &mut (*peer).dispatchList as *mut ENetListNode as *mut c_void,
            );
            (*peer).flags = ((*peer).flags as i32 | ENET_PEER_FLAG_NEEDS_DISPATCH as i32) as u16;
        }
        droppedCommand = currentCommand;
    }
    enet_peer_remove_incoming_commands(
        &mut (*channel).incomingUnreliableCommands,
        (*channel).incomingUnreliableCommands.sentinel.next,
        droppedCommand,
        queuedCommand,
    );
}
pub(crate) unsafe fn enet_peer_dispatch_incoming_reliable_commands<S: Socket>(
    peer: *mut ENetPeer<S>,
    channel: *mut ENetChannel,
    queuedCommand: *mut ENetIncomingCommand,
) {
    let mut currentCommand: ENetListIterator;
    currentCommand = (*channel).incomingReliableCommands.sentinel.next;
    while currentCommand != &mut (*channel).incomingReliableCommands.sentinel as *mut ENetListNode {
        let incomingCommand: *mut ENetIncomingCommand = currentCommand as *mut ENetIncomingCommand;
        if (*incomingCommand).fragmentsRemaining > 0_i32 as u32
            || (*incomingCommand).reliableSequenceNumber as i32
                != ((*channel).incomingReliableSequenceNumber as i32 + 1_i32) as u16 as i32
        {
            break;
        }
        (*channel).incomingReliableSequenceNumber = (*incomingCommand).reliableSequenceNumber;
        if (*incomingCommand).fragmentCount > 0_i32 as u32 {
            (*channel).incomingReliableSequenceNumber = ((*channel).incomingReliableSequenceNumber
                as u32)
                .wrapping_add(((*incomingCommand).fragmentCount).wrapping_sub(1_i32 as u32))
                as u16 as u16;
        }
        currentCommand = (*currentCommand).next;
    }
    if currentCommand == (*channel).incomingReliableCommands.sentinel.next {
        return;
    }
    (*channel).incomingUnreliableSequenceNumber = 0_i32 as u16;
    enet_list_move(
        &mut (*peer).dispatchedCommands.sentinel,
        (*channel).incomingReliableCommands.sentinel.next as *mut c_void,
        (*currentCommand).previous as *mut c_void,
    );
    if (*peer).flags as i32 & ENET_PEER_FLAG_NEEDS_DISPATCH as i32 == 0 {
        enet_list_insert(
            &mut (*(*peer).host).dispatchQueue.sentinel,
            &mut (*peer).dispatchList as *mut ENetListNode as *mut c_void,
        );
        (*peer).flags = ((*peer).flags as i32 | ENET_PEER_FLAG_NEEDS_DISPATCH as i32) as u16;
    }
    if (*channel).incomingUnreliableCommands.sentinel.next
        != &mut (*channel).incomingUnreliableCommands.sentinel as *mut ENetListNode
    {
        enet_peer_dispatch_incoming_unreliable_commands(peer, channel, queuedCommand);
    }
}
pub(crate) unsafe fn enet_peer_queue_incoming_command<S: Socket>(
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
    data: *const c_void,
    dataLength: usize,
    flags: u32,
    fragmentCount: u32,
) -> *mut ENetIncomingCommand {
    let mut current_block: u64;
    static mut DUMMY_COMMAND: ENetIncomingCommand = ENetIncomingCommand {
        incomingCommandList: ENetListNode {
            next: 0 as *mut _ENetListNode,
            previous: 0 as *mut _ENetListNode,
        },
        reliableSequenceNumber: 0,
        unreliableSequenceNumber: 0,
        command: _ENetProtocol {
            header: ENetProtocolCommandHeader {
                command: 0,
                channelID: 0,
                reliableSequenceNumber: 0,
            },
        },
        fragmentCount: 0,
        fragmentsRemaining: 0,
        fragments: 0 as *const u32 as *mut u32,
        packet: 0 as *const ENetPacket as *mut ENetPacket,
    };
    let channel: *mut ENetChannel =
        &mut *((*peer).channels).offset((*command).header.channelID as isize) as *mut ENetChannel;
    let mut unreliableSequenceNumber: u32 = 0_i32 as u32;
    let mut reliableSequenceNumber: u32 = 0_i32 as u32;
    let mut reliableWindow: u16;
    let currentWindow: u16;
    let mut incomingCommand: *mut ENetIncomingCommand;
    let mut currentCommand: ENetListIterator = std::ptr::null_mut();
    let mut packet: *mut ENetPacket = std::ptr::null_mut();
    if (*peer).state == ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32 {
        current_block = 9207730764507465628;
    } else {
        if (*command).header.command as i32 & ENET_PROTOCOL_COMMAND_MASK as i32
            != ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as i32
        {
            reliableSequenceNumber = (*command).header.reliableSequenceNumber as u32;
            reliableWindow = reliableSequenceNumber
                .wrapping_div(ENET_PEER_RELIABLE_WINDOW_SIZE as i32 as u32)
                as u16;
            currentWindow = ((*channel).incomingReliableSequenceNumber as i32
                / ENET_PEER_RELIABLE_WINDOW_SIZE as i32) as u16;
            if reliableSequenceNumber < (*channel).incomingReliableSequenceNumber as u32 {
                reliableWindow = (reliableWindow as i32 + ENET_PEER_RELIABLE_WINDOWS as i32) as u16;
            }
            if (reliableWindow as i32) < currentWindow as i32
                || reliableWindow as i32
                    >= currentWindow as i32 + ENET_PEER_FREE_RELIABLE_WINDOWS as i32 - 1_i32
            {
                current_block = 9207730764507465628;
            } else {
                current_block = 13183875560443969876;
            }
        } else {
            current_block = 13183875560443969876;
        }
        match current_block {
            9207730764507465628 => {}
            _ => match (*command).header.command as i32 & ENET_PROTOCOL_COMMAND_MASK as i32 {
                8 | 6 => {
                    current_block = 4379360700607281851;
                    match current_block {
                        10107555224945550073 => {
                            currentCommand = &mut (*channel).incomingUnreliableCommands.sentinel;
                            current_block = 7746103178988627676;
                        }
                        4379360700607281851 => {
                            if reliableSequenceNumber
                                == (*channel).incomingReliableSequenceNumber as u32
                            {
                                current_block = 9207730764507465628;
                            } else {
                                currentCommand =
                                    (*channel).incomingReliableCommands.sentinel.previous;
                                loop {
                                    if currentCommand
                                        == &mut (*channel).incomingReliableCommands.sentinel
                                            as *mut ENetListNode
                                    {
                                        current_block = 7746103178988627676;
                                        break;
                                    }
                                    incomingCommand = currentCommand as *mut ENetIncomingCommand;
                                    if reliableSequenceNumber
                                        >= (*channel).incomingReliableSequenceNumber as u32
                                    {
                                        if ((*incomingCommand).reliableSequenceNumber as i32)
                                            < (*channel).incomingReliableSequenceNumber as i32
                                        {
                                            current_block = 1856101646708284338;
                                        } else {
                                            current_block = 8457315219000651999;
                                        }
                                    } else {
                                        if (*incomingCommand).reliableSequenceNumber as i32
                                            >= (*channel).incomingReliableSequenceNumber as i32
                                        {
                                            current_block = 7746103178988627676;
                                            break;
                                        }
                                        current_block = 8457315219000651999;
                                    }
                                    if let 8457315219000651999 = current_block {
                                        if (*incomingCommand).reliableSequenceNumber as u32
                                            <= reliableSequenceNumber
                                        {
                                            if ((*incomingCommand).reliableSequenceNumber as u32)
                                                < reliableSequenceNumber
                                            {
                                                current_block = 7746103178988627676;
                                                break;
                                            } else {
                                                current_block = 9207730764507465628;
                                                break;
                                            }
                                        }
                                    }
                                    currentCommand = (*currentCommand).previous;
                                }
                            }
                        }
                        _ => {
                            unreliableSequenceNumber =
                                u16::from_be((*command).sendUnreliable.unreliableSequenceNumber)
                                    as u32;
                            if reliableSequenceNumber
                                == (*channel).incomingReliableSequenceNumber as u32
                                && unreliableSequenceNumber
                                    <= (*channel).incomingUnreliableSequenceNumber as u32
                            {
                                current_block = 9207730764507465628;
                            } else {
                                currentCommand =
                                    (*channel).incomingUnreliableCommands.sentinel.previous;
                                loop {
                                    if currentCommand
                                        == &mut (*channel).incomingUnreliableCommands.sentinel
                                            as *mut ENetListNode
                                    {
                                        current_block = 7746103178988627676;
                                        break;
                                    }
                                    incomingCommand = currentCommand as *mut ENetIncomingCommand;
                                    if (*command).header.command as i32
                                        & ENET_PROTOCOL_COMMAND_MASK as i32
                                        != ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as i32
                                    {
                                        if reliableSequenceNumber
                                            >= (*channel).incomingReliableSequenceNumber as u32
                                        {
                                            if ((*incomingCommand).reliableSequenceNumber as i32)
                                                < (*channel).incomingReliableSequenceNumber as i32
                                            {
                                                current_block = 17478428563724192186;
                                            } else {
                                                current_block = 11459959175219260272;
                                            }
                                        } else {
                                            if (*incomingCommand).reliableSequenceNumber as i32
                                                >= (*channel).incomingReliableSequenceNumber as i32
                                            {
                                                current_block = 7746103178988627676;
                                                break;
                                            }
                                            current_block = 11459959175219260272;
                                        }
                                        match current_block {
                                            17478428563724192186 => {}
                                            _ => {
                                                if ((*incomingCommand).reliableSequenceNumber
                                                    as u32)
                                                    < reliableSequenceNumber
                                                {
                                                    current_block = 7746103178988627676;
                                                    break;
                                                }
                                                if (*incomingCommand).reliableSequenceNumber as u32
                                                    <= reliableSequenceNumber
                                                    && (*incomingCommand).unreliableSequenceNumber
                                                        as u32
                                                        <= unreliableSequenceNumber
                                                {
                                                    if ((*incomingCommand).unreliableSequenceNumber
                                                        as u32)
                                                        < unreliableSequenceNumber
                                                    {
                                                        current_block = 7746103178988627676;
                                                        break;
                                                    } else {
                                                        current_block = 9207730764507465628;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    currentCommand = (*currentCommand).previous;
                                }
                            }
                        }
                    }
                    match current_block {
                        9207730764507465628 => {}
                        _ => {
                            if (*peer).totalWaitingData >= (*(*peer).host).maximumWaitingData {
                                current_block = 15492018734234176694;
                            } else {
                                packet = enet_packet_create(data, dataLength, flags);
                                if packet.is_null() {
                                    current_block = 15492018734234176694;
                                } else {
                                    incomingCommand =
                                        enet_malloc(::core::mem::size_of::<ENetIncomingCommand>())
                                            as *mut ENetIncomingCommand;
                                    if incomingCommand.is_null() {
                                        current_block = 15492018734234176694;
                                    } else {
                                        (*incomingCommand).reliableSequenceNumber =
                                            (*command).header.reliableSequenceNumber;
                                        (*incomingCommand).unreliableSequenceNumber =
                                            (unreliableSequenceNumber & 0xffff_i32 as u32) as u16;
                                        (*incomingCommand).command = *command;
                                        (*incomingCommand).fragmentCount = fragmentCount;
                                        (*incomingCommand).fragmentsRemaining = fragmentCount;
                                        (*incomingCommand).packet = packet;
                                        (*incomingCommand).fragments = std::ptr::null_mut();
                                        if fragmentCount > 0_i32 as u32 {
                                            if fragmentCount
                                                <= ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as i32
                                                    as u32
                                            {
                                                (*incomingCommand).fragments =
                                                    enet_malloc(
                                                        (fragmentCount
                                                            .wrapping_add(31_i32 as u32)
                                                            .wrapping_div(32_i32 as u32)
                                                            as usize)
                                                            .wrapping_mul(::core::mem::size_of::<
                                                                u32,
                                                            >(
                                                            )),
                                                    )
                                                        as *mut u32;
                                            }
                                            if ((*incomingCommand).fragments).is_null() {
                                                enet_free(incomingCommand as *mut c_void);
                                                current_block = 15492018734234176694;
                                            } else {
                                                _enet_memset(
                                                    (*incomingCommand).fragments as *mut c_void,
                                                    0_i32,
                                                    (fragmentCount
                                                        .wrapping_add(31_i32 as u32)
                                                        .wrapping_div(32_i32 as u32)
                                                        as usize)
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<u32>(),
                                                        ),
                                                );
                                                current_block = 13321564401369230990;
                                            }
                                        } else {
                                            current_block = 13321564401369230990;
                                        }
                                        match current_block {
                                            15492018734234176694 => {}
                                            _ => {
                                                if !packet.is_null() {
                                                    (*packet).referenceCount =
                                                        ((*packet).referenceCount).wrapping_add(1);
                                                    (*peer).totalWaitingData = (*peer)
                                                        .totalWaitingData
                                                        .wrapping_add((*packet).dataLength)
                                                        as usize
                                                        as usize;
                                                }
                                                enet_list_insert(
                                                    (*currentCommand).next,
                                                    incomingCommand as *mut c_void,
                                                );
                                                match (*command).header.command as i32
                                                    & ENET_PROTOCOL_COMMAND_MASK as i32
                                                {
                                                    8 | 6 => {
                                                        enet_peer_dispatch_incoming_reliable_commands(
                                                                peer,
                                                                channel,
                                                                incomingCommand,
                                                            );
                                                    }
                                                    _ => {
                                                        enet_peer_dispatch_incoming_unreliable_commands(
                                                                peer,
                                                                channel,
                                                                incomingCommand,
                                                            );
                                                    }
                                                }
                                                return incomingCommand;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                7 | 12 => {
                    current_block = 1195130990526578986;
                    match current_block {
                        10107555224945550073 => {
                            currentCommand = &mut (*channel).incomingUnreliableCommands.sentinel;
                            current_block = 7746103178988627676;
                        }
                        4379360700607281851 => {
                            if reliableSequenceNumber
                                == (*channel).incomingReliableSequenceNumber as u32
                            {
                                current_block = 9207730764507465628;
                            } else {
                                currentCommand =
                                    (*channel).incomingReliableCommands.sentinel.previous;
                                loop {
                                    if currentCommand
                                        == &mut (*channel).incomingReliableCommands.sentinel
                                            as *mut ENetListNode
                                    {
                                        current_block = 7746103178988627676;
                                        break;
                                    }
                                    incomingCommand = currentCommand as *mut ENetIncomingCommand;
                                    if reliableSequenceNumber
                                        >= (*channel).incomingReliableSequenceNumber as u32
                                    {
                                        if ((*incomingCommand).reliableSequenceNumber as i32)
                                            < (*channel).incomingReliableSequenceNumber as i32
                                        {
                                            current_block = 1856101646708284338;
                                        } else {
                                            current_block = 8457315219000651999;
                                        }
                                    } else {
                                        if (*incomingCommand).reliableSequenceNumber as i32
                                            >= (*channel).incomingReliableSequenceNumber as i32
                                        {
                                            current_block = 7746103178988627676;
                                            break;
                                        }
                                        current_block = 8457315219000651999;
                                    }
                                    if let 8457315219000651999 = current_block {
                                        if (*incomingCommand).reliableSequenceNumber as u32
                                            <= reliableSequenceNumber
                                        {
                                            if ((*incomingCommand).reliableSequenceNumber as u32)
                                                < reliableSequenceNumber
                                            {
                                                current_block = 7746103178988627676;
                                                break;
                                            } else {
                                                current_block = 9207730764507465628;
                                                break;
                                            }
                                        }
                                    }
                                    currentCommand = (*currentCommand).previous;
                                }
                            }
                        }
                        _ => {
                            unreliableSequenceNumber =
                                u16::from_be((*command).sendUnreliable.unreliableSequenceNumber)
                                    as u32;
                            if reliableSequenceNumber
                                == (*channel).incomingReliableSequenceNumber as u32
                                && unreliableSequenceNumber
                                    <= (*channel).incomingUnreliableSequenceNumber as u32
                            {
                                current_block = 9207730764507465628;
                            } else {
                                currentCommand =
                                    (*channel).incomingUnreliableCommands.sentinel.previous;
                                loop {
                                    if currentCommand
                                        == &mut (*channel).incomingUnreliableCommands.sentinel
                                            as *mut ENetListNode
                                    {
                                        current_block = 7746103178988627676;
                                        break;
                                    }
                                    incomingCommand = currentCommand as *mut ENetIncomingCommand;
                                    if (*command).header.command as i32
                                        & ENET_PROTOCOL_COMMAND_MASK as i32
                                        != ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as i32
                                    {
                                        if reliableSequenceNumber
                                            >= (*channel).incomingReliableSequenceNumber as u32
                                        {
                                            if ((*incomingCommand).reliableSequenceNumber as i32)
                                                < (*channel).incomingReliableSequenceNumber as i32
                                            {
                                                current_block = 17478428563724192186;
                                            } else {
                                                current_block = 11459959175219260272;
                                            }
                                        } else {
                                            if (*incomingCommand).reliableSequenceNumber as i32
                                                >= (*channel).incomingReliableSequenceNumber as i32
                                            {
                                                current_block = 7746103178988627676;
                                                break;
                                            }
                                            current_block = 11459959175219260272;
                                        }
                                        match current_block {
                                            17478428563724192186 => {}
                                            _ => {
                                                if ((*incomingCommand).reliableSequenceNumber
                                                    as u32)
                                                    < reliableSequenceNumber
                                                {
                                                    current_block = 7746103178988627676;
                                                    break;
                                                }
                                                if (*incomingCommand).reliableSequenceNumber as u32
                                                    <= reliableSequenceNumber
                                                    && (*incomingCommand).unreliableSequenceNumber
                                                        as u32
                                                        <= unreliableSequenceNumber
                                                {
                                                    if ((*incomingCommand).unreliableSequenceNumber
                                                        as u32)
                                                        < unreliableSequenceNumber
                                                    {
                                                        current_block = 7746103178988627676;
                                                        break;
                                                    } else {
                                                        current_block = 9207730764507465628;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    currentCommand = (*currentCommand).previous;
                                }
                            }
                        }
                    }
                    match current_block {
                        9207730764507465628 => {}
                        _ => {
                            if (*peer).totalWaitingData >= (*(*peer).host).maximumWaitingData {
                                current_block = 15492018734234176694;
                            } else {
                                packet = enet_packet_create(data, dataLength, flags);
                                if packet.is_null() {
                                    current_block = 15492018734234176694;
                                } else {
                                    incomingCommand =
                                        enet_malloc(::core::mem::size_of::<ENetIncomingCommand>())
                                            as *mut ENetIncomingCommand;
                                    if incomingCommand.is_null() {
                                        current_block = 15492018734234176694;
                                    } else {
                                        (*incomingCommand).reliableSequenceNumber =
                                            (*command).header.reliableSequenceNumber;
                                        (*incomingCommand).unreliableSequenceNumber =
                                            (unreliableSequenceNumber & 0xffff_i32 as u32) as u16;
                                        (*incomingCommand).command = *command;
                                        (*incomingCommand).fragmentCount = fragmentCount;
                                        (*incomingCommand).fragmentsRemaining = fragmentCount;
                                        (*incomingCommand).packet = packet;
                                        (*incomingCommand).fragments = std::ptr::null_mut();
                                        if fragmentCount > 0_i32 as u32 {
                                            if fragmentCount
                                                <= ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as i32
                                                    as u32
                                            {
                                                (*incomingCommand).fragments =
                                                    enet_malloc(
                                                        (fragmentCount
                                                            .wrapping_add(31_i32 as u32)
                                                            .wrapping_div(32_i32 as u32)
                                                            as usize)
                                                            .wrapping_mul(::core::mem::size_of::<
                                                                u32,
                                                            >(
                                                            )),
                                                    )
                                                        as *mut u32;
                                            }
                                            if ((*incomingCommand).fragments).is_null() {
                                                enet_free(incomingCommand as *mut c_void);
                                                current_block = 15492018734234176694;
                                            } else {
                                                _enet_memset(
                                                    (*incomingCommand).fragments as *mut c_void,
                                                    0_i32,
                                                    (fragmentCount
                                                        .wrapping_add(31_i32 as u32)
                                                        .wrapping_div(32_i32 as u32)
                                                        as usize)
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<u32>(),
                                                        ),
                                                );
                                                current_block = 13321564401369230990;
                                            }
                                        } else {
                                            current_block = 13321564401369230990;
                                        }
                                        match current_block {
                                            15492018734234176694 => {}
                                            _ => {
                                                if !packet.is_null() {
                                                    (*packet).referenceCount =
                                                        ((*packet).referenceCount).wrapping_add(1);
                                                    (*peer).totalWaitingData = (*peer)
                                                        .totalWaitingData
                                                        .wrapping_add((*packet).dataLength)
                                                        as usize
                                                        as usize;
                                                }
                                                enet_list_insert(
                                                    (*currentCommand).next,
                                                    incomingCommand as *mut c_void,
                                                );
                                                match (*command).header.command as i32
                                                    & ENET_PROTOCOL_COMMAND_MASK as i32
                                                {
                                                    8 | 6 => {
                                                        enet_peer_dispatch_incoming_reliable_commands(
                                                                peer,
                                                                channel,
                                                                incomingCommand,
                                                            );
                                                    }
                                                    _ => {
                                                        enet_peer_dispatch_incoming_unreliable_commands(
                                                                peer,
                                                                channel,
                                                                incomingCommand,
                                                            );
                                                    }
                                                }
                                                return incomingCommand;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                9 => {
                    current_block = 10107555224945550073;
                    match current_block {
                        10107555224945550073 => {
                            currentCommand = &mut (*channel).incomingUnreliableCommands.sentinel;
                            current_block = 7746103178988627676;
                        }
                        4379360700607281851 => {
                            if reliableSequenceNumber
                                == (*channel).incomingReliableSequenceNumber as u32
                            {
                                current_block = 9207730764507465628;
                            } else {
                                currentCommand =
                                    (*channel).incomingReliableCommands.sentinel.previous;
                                loop {
                                    if currentCommand
                                        == &mut (*channel).incomingReliableCommands.sentinel
                                            as *mut ENetListNode
                                    {
                                        current_block = 7746103178988627676;
                                        break;
                                    }
                                    incomingCommand = currentCommand as *mut ENetIncomingCommand;
                                    if reliableSequenceNumber
                                        >= (*channel).incomingReliableSequenceNumber as u32
                                    {
                                        if ((*incomingCommand).reliableSequenceNumber as i32)
                                            < (*channel).incomingReliableSequenceNumber as i32
                                        {
                                            current_block = 1856101646708284338;
                                        } else {
                                            current_block = 8457315219000651999;
                                        }
                                    } else {
                                        if (*incomingCommand).reliableSequenceNumber as i32
                                            >= (*channel).incomingReliableSequenceNumber as i32
                                        {
                                            current_block = 7746103178988627676;
                                            break;
                                        }
                                        current_block = 8457315219000651999;
                                    }
                                    if let 8457315219000651999 = current_block {
                                        if (*incomingCommand).reliableSequenceNumber as u32
                                            <= reliableSequenceNumber
                                        {
                                            if ((*incomingCommand).reliableSequenceNumber as u32)
                                                < reliableSequenceNumber
                                            {
                                                current_block = 7746103178988627676;
                                                break;
                                            } else {
                                                current_block = 9207730764507465628;
                                                break;
                                            }
                                        }
                                    }
                                    currentCommand = (*currentCommand).previous;
                                }
                            }
                        }
                        _ => {
                            unreliableSequenceNumber =
                                u16::from_be((*command).sendUnreliable.unreliableSequenceNumber)
                                    as u32;
                            if reliableSequenceNumber
                                == (*channel).incomingReliableSequenceNumber as u32
                                && unreliableSequenceNumber
                                    <= (*channel).incomingUnreliableSequenceNumber as u32
                            {
                                current_block = 9207730764507465628;
                            } else {
                                currentCommand =
                                    (*channel).incomingUnreliableCommands.sentinel.previous;
                                loop {
                                    if currentCommand
                                        == &mut (*channel).incomingUnreliableCommands.sentinel
                                            as *mut ENetListNode
                                    {
                                        current_block = 7746103178988627676;
                                        break;
                                    }
                                    incomingCommand = currentCommand as *mut ENetIncomingCommand;
                                    if (*command).header.command as i32
                                        & ENET_PROTOCOL_COMMAND_MASK as i32
                                        != ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as i32
                                    {
                                        if reliableSequenceNumber
                                            >= (*channel).incomingReliableSequenceNumber as u32
                                        {
                                            if ((*incomingCommand).reliableSequenceNumber as i32)
                                                < (*channel).incomingReliableSequenceNumber as i32
                                            {
                                                current_block = 17478428563724192186;
                                            } else {
                                                current_block = 11459959175219260272;
                                            }
                                        } else {
                                            if (*incomingCommand).reliableSequenceNumber as i32
                                                >= (*channel).incomingReliableSequenceNumber as i32
                                            {
                                                current_block = 7746103178988627676;
                                                break;
                                            }
                                            current_block = 11459959175219260272;
                                        }
                                        match current_block {
                                            17478428563724192186 => {}
                                            _ => {
                                                if ((*incomingCommand).reliableSequenceNumber
                                                    as u32)
                                                    < reliableSequenceNumber
                                                {
                                                    current_block = 7746103178988627676;
                                                    break;
                                                }
                                                if (*incomingCommand).reliableSequenceNumber as u32
                                                    <= reliableSequenceNumber
                                                    && (*incomingCommand).unreliableSequenceNumber
                                                        as u32
                                                        <= unreliableSequenceNumber
                                                {
                                                    if ((*incomingCommand).unreliableSequenceNumber
                                                        as u32)
                                                        < unreliableSequenceNumber
                                                    {
                                                        current_block = 7746103178988627676;
                                                        break;
                                                    } else {
                                                        current_block = 9207730764507465628;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    currentCommand = (*currentCommand).previous;
                                }
                            }
                        }
                    }
                    match current_block {
                        9207730764507465628 => {}
                        _ => {
                            if (*peer).totalWaitingData >= (*(*peer).host).maximumWaitingData {
                                current_block = 15492018734234176694;
                            } else {
                                packet = enet_packet_create(data, dataLength, flags);
                                if packet.is_null() {
                                    current_block = 15492018734234176694;
                                } else {
                                    incomingCommand =
                                        enet_malloc(::core::mem::size_of::<ENetIncomingCommand>())
                                            as *mut ENetIncomingCommand;
                                    if incomingCommand.is_null() {
                                        current_block = 15492018734234176694;
                                    } else {
                                        (*incomingCommand).reliableSequenceNumber =
                                            (*command).header.reliableSequenceNumber;
                                        (*incomingCommand).unreliableSequenceNumber =
                                            (unreliableSequenceNumber & 0xffff_i32 as u32) as u16;
                                        (*incomingCommand).command = *command;
                                        (*incomingCommand).fragmentCount = fragmentCount;
                                        (*incomingCommand).fragmentsRemaining = fragmentCount;
                                        (*incomingCommand).packet = packet;
                                        (*incomingCommand).fragments = std::ptr::null_mut();
                                        if fragmentCount > 0_i32 as u32 {
                                            if fragmentCount
                                                <= ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as i32
                                                    as u32
                                            {
                                                (*incomingCommand).fragments =
                                                    enet_malloc(
                                                        (fragmentCount
                                                            .wrapping_add(31_i32 as u32)
                                                            .wrapping_div(32_i32 as u32)
                                                            as usize)
                                                            .wrapping_mul(::core::mem::size_of::<
                                                                u32,
                                                            >(
                                                            )),
                                                    )
                                                        as *mut u32;
                                            }
                                            if ((*incomingCommand).fragments).is_null() {
                                                enet_free(incomingCommand as *mut c_void);
                                                current_block = 15492018734234176694;
                                            } else {
                                                _enet_memset(
                                                    (*incomingCommand).fragments as *mut c_void,
                                                    0_i32,
                                                    (fragmentCount
                                                        .wrapping_add(31_i32 as u32)
                                                        .wrapping_div(32_i32 as u32)
                                                        as usize)
                                                        .wrapping_mul(
                                                            ::core::mem::size_of::<u32>(),
                                                        ),
                                                );
                                                current_block = 13321564401369230990;
                                            }
                                        } else {
                                            current_block = 13321564401369230990;
                                        }
                                        match current_block {
                                            15492018734234176694 => {}
                                            _ => {
                                                if !packet.is_null() {
                                                    (*packet).referenceCount =
                                                        ((*packet).referenceCount).wrapping_add(1);
                                                    (*peer).totalWaitingData = (*peer)
                                                        .totalWaitingData
                                                        .wrapping_add((*packet).dataLength)
                                                        as usize
                                                        as usize;
                                                }
                                                enet_list_insert(
                                                    (*currentCommand).next,
                                                    incomingCommand as *mut c_void,
                                                );
                                                match (*command).header.command as i32
                                                    & ENET_PROTOCOL_COMMAND_MASK as i32
                                                {
                                                    8 | 6 => {
                                                        enet_peer_dispatch_incoming_reliable_commands(
                                                                peer,
                                                                channel,
                                                                incomingCommand,
                                                            );
                                                    }
                                                    _ => {
                                                        enet_peer_dispatch_incoming_unreliable_commands(
                                                                peer,
                                                                channel,
                                                                incomingCommand,
                                                            );
                                                    }
                                                }
                                                return incomingCommand;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    current_block = 9207730764507465628;
                }
            },
        }
    }
    if let 9207730764507465628 = current_block {
        if fragmentCount <= 0_i32 as u32 {
            if !packet.is_null() && (*packet).referenceCount == 0_i32 as usize {
                enet_packet_destroy(packet);
            }
            return &mut DUMMY_COMMAND;
        }
    }
    if !packet.is_null() && (*packet).referenceCount == 0_i32 as usize {
        enet_packet_destroy(packet);
    }
    std::ptr::null_mut()
}
