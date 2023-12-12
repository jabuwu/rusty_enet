use crate::{
    consts::{
        ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL, ENET_PEER_FREE_RELIABLE_WINDOWS,
        ENET_PEER_FREE_UNSEQUENCED_WINDOWS, ENET_PEER_PACKET_LOSS_INTERVAL,
        ENET_PEER_PACKET_LOSS_SCALE, ENET_PEER_PACKET_THROTTLE_COUNTER,
        ENET_PEER_PACKET_THROTTLE_SCALE, ENET_PEER_RELIABLE_WINDOWS,
        ENET_PEER_RELIABLE_WINDOW_SIZE, ENET_PEER_UNSEQUENCED_WINDOW_SIZE,
        ENET_PEER_WINDOW_SIZE_SCALE, ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT,
        ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT, ENET_PROTOCOL_MAXIMUM_MTU,
        ENET_PROTOCOL_MAXIMUM_PEER_ID, ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE,
        ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT, ENET_PROTOCOL_MINIMUM_MTU,
        ENET_PROTOCOL_MINIMUM_WINDOW_SIZE,
    },
    enet_free, enet_host_bandwidth_throttle, enet_list_clear, enet_list_insert, enet_list_remove,
    enet_malloc, enet_packet_destroy, enet_peer_disconnect,
    enet_peer_dispatch_incoming_reliable_commands, enet_peer_dispatch_incoming_unreliable_commands,
    enet_peer_has_outgoing_commands, enet_peer_on_connect, enet_peer_on_disconnect, enet_peer_ping,
    enet_peer_queue_acknowledgement, enet_peer_queue_incoming_command,
    enet_peer_queue_outgoing_command, enet_peer_receive, enet_peer_reset, enet_peer_reset_queues,
    enet_peer_throttle, enet_time_get,
    os::{_enet_memcpy, _enet_memset, c_void},
    Address, ENetAcknowledgement, ENetBuffer, ENetChannel, ENetEvent, ENetHost,
    ENetIncomingCommand, ENetList, ENetListIterator, ENetListNode, ENetOutgoingCommand, ENetPeer,
    ENetPeerState, PacketReceived, Socket, ENET_EVENT_TYPE_CONNECT, ENET_EVENT_TYPE_DISCONNECT,
    ENET_EVENT_TYPE_NONE, ENET_EVENT_TYPE_RECEIVE, ENET_PACKET_FLAG_RELIABLE,
    ENET_PACKET_FLAG_SENT, ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT, ENET_PACKET_FLAG_UNSEQUENCED,
    ENET_PEER_FLAG_CONTINUE_SENDING, ENET_PEER_FLAG_NEEDS_DISPATCH,
    ENET_PEER_STATE_ACKNOWLEDGING_CONNECT, ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT,
    ENET_PEER_STATE_CONNECTED, ENET_PEER_STATE_CONNECTING, ENET_PEER_STATE_CONNECTION_PENDING,
    ENET_PEER_STATE_CONNECTION_SUCCEEDED, ENET_PEER_STATE_DISCONNECTED,
    ENET_PEER_STATE_DISCONNECTING, ENET_PEER_STATE_DISCONNECT_LATER, ENET_PEER_STATE_ZOMBIE,
};

pub(crate) type _ENetProtocolCommand = u32;
pub(crate) const ENET_PROTOCOL_COMMAND_MASK: _ENetProtocolCommand = 15;
pub(crate) const ENET_PROTOCOL_COMMAND_COUNT: _ENetProtocolCommand = 13;
pub(crate) const ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT: _ENetProtocolCommand = 12;
pub(crate) const ENET_PROTOCOL_COMMAND_THROTTLE_CONFIGURE: _ENetProtocolCommand = 11;
pub(crate) const ENET_PROTOCOL_COMMAND_BANDWIDTH_LIMIT: _ENetProtocolCommand = 10;
pub(crate) const ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED: _ENetProtocolCommand = 9;
pub(crate) const ENET_PROTOCOL_COMMAND_SEND_FRAGMENT: _ENetProtocolCommand = 8;
pub(crate) const ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE: _ENetProtocolCommand = 7;
pub(crate) const ENET_PROTOCOL_COMMAND_SEND_RELIABLE: _ENetProtocolCommand = 6;
pub(crate) const ENET_PROTOCOL_COMMAND_PING: _ENetProtocolCommand = 5;
pub(crate) const ENET_PROTOCOL_COMMAND_DISCONNECT: _ENetProtocolCommand = 4;
pub(crate) const ENET_PROTOCOL_COMMAND_VERIFY_CONNECT: _ENetProtocolCommand = 3;
pub(crate) const ENET_PROTOCOL_COMMAND_CONNECT: _ENetProtocolCommand = 2;
pub(crate) const ENET_PROTOCOL_COMMAND_ACKNOWLEDGE: _ENetProtocolCommand = 1;
pub(crate) const ENET_PROTOCOL_COMMAND_NONE: _ENetProtocolCommand = 0;
pub(crate) type ENetProtocolCommand = _ENetProtocolCommand;
pub(crate) type _ENetProtocolFlag = u32;
pub(crate) const ENET_PROTOCOL_HEADER_SESSION_SHIFT: _ENetProtocolFlag = 12;
pub(crate) const ENET_PROTOCOL_HEADER_SESSION_MASK: _ENetProtocolFlag = 12288;
pub(crate) const ENET_PROTOCOL_HEADER_FLAG_MASK: _ENetProtocolFlag = 49152;
pub(crate) const ENET_PROTOCOL_HEADER_FLAG_SENT_TIME: _ENetProtocolFlag = 32768;
pub(crate) const ENET_PROTOCOL_HEADER_FLAG_COMPRESSED: _ENetProtocolFlag = 16384;
pub(crate) const ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED: _ENetProtocolFlag = 64;
pub(crate) const ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE: _ENetProtocolFlag = 128;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolHeader {
    pub(crate) peerID: u16,
    pub(crate) sentTime: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolCommandHeader {
    pub(crate) command: u8,
    pub(crate) channelID: u8,
    pub(crate) reliableSequenceNumber: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolAcknowledge {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) receivedReliableSequenceNumber: u16,
    pub(crate) receivedSentTime: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolConnect {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) outgoingPeerID: u16,
    pub(crate) incomingSessionID: u8,
    pub(crate) outgoingSessionID: u8,
    pub(crate) mtu: u32,
    pub(crate) windowSize: u32,
    pub(crate) channelCount: u32,
    pub(crate) incomingBandwidth: u32,
    pub(crate) outgoingBandwidth: u32,
    pub(crate) packetThrottleInterval: u32,
    pub(crate) packetThrottleAcceleration: u32,
    pub(crate) packetThrottleDeceleration: u32,
    pub(crate) connectID: u32,
    pub(crate) data: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolVerifyConnect {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) outgoingPeerID: u16,
    pub(crate) incomingSessionID: u8,
    pub(crate) outgoingSessionID: u8,
    pub(crate) mtu: u32,
    pub(crate) windowSize: u32,
    pub(crate) channelCount: u32,
    pub(crate) incomingBandwidth: u32,
    pub(crate) outgoingBandwidth: u32,
    pub(crate) packetThrottleInterval: u32,
    pub(crate) packetThrottleAcceleration: u32,
    pub(crate) packetThrottleDeceleration: u32,
    pub(crate) connectID: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolBandwidthLimit {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) incomingBandwidth: u32,
    pub(crate) outgoingBandwidth: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolThrottleConfigure {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) packetThrottleInterval: u32,
    pub(crate) packetThrottleAcceleration: u32,
    pub(crate) packetThrottleDeceleration: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolDisconnect {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) data: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolPing {
    pub(crate) header: ENetProtocolCommandHeader,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolSendReliable {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) dataLength: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolSendUnreliable {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) unreliableSequenceNumber: u16,
    pub(crate) dataLength: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolSendUnsequenced {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) unsequencedGroup: u16,
    pub(crate) dataLength: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolSendFragment {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) startSequenceNumber: u16,
    pub(crate) dataLength: u16,
    pub(crate) fragmentCount: u32,
    pub(crate) fragmentNumber: u32,
    pub(crate) totalLength: u32,
    pub(crate) fragmentOffset: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) union ENetProtocol {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) acknowledge: ENetProtocolAcknowledge,
    pub(crate) connect: ENetProtocolConnect,
    pub(crate) verifyConnect: ENetProtocolVerifyConnect,
    pub(crate) disconnect: ENetProtocolDisconnect,
    pub(crate) ping: ENetProtocolPing,
    pub(crate) sendReliable: ENetProtocolSendReliable,
    pub(crate) sendUnreliable: ENetProtocolSendUnreliable,
    pub(crate) sendUnsequenced: ENetProtocolSendUnsequenced,
    pub(crate) sendFragment: ENetProtocolSendFragment,
    pub(crate) bandwidthLimit: ENetProtocolBandwidthLimit,
    pub(crate) throttleConfigure: ENetProtocolThrottleConfigure,
}
static mut COMMAND_SIZES: [usize; 13] = [
    0_i32 as usize,
    ::core::mem::size_of::<ENetProtocolAcknowledge>(),
    ::core::mem::size_of::<ENetProtocolConnect>(),
    ::core::mem::size_of::<ENetProtocolVerifyConnect>(),
    ::core::mem::size_of::<ENetProtocolDisconnect>(),
    ::core::mem::size_of::<ENetProtocolPing>(),
    ::core::mem::size_of::<ENetProtocolSendReliable>(),
    ::core::mem::size_of::<ENetProtocolSendUnreliable>(),
    ::core::mem::size_of::<ENetProtocolSendFragment>(),
    ::core::mem::size_of::<ENetProtocolSendUnsequenced>(),
    ::core::mem::size_of::<ENetProtocolBandwidthLimit>(),
    ::core::mem::size_of::<ENetProtocolThrottleConfigure>(),
    ::core::mem::size_of::<ENetProtocolSendFragment>(),
];
pub(crate) unsafe fn enet_protocol_command_size(commandNumber: u8) -> usize {
    COMMAND_SIZES[(commandNumber as i32 & ENET_PROTOCOL_COMMAND_MASK as i32) as usize]
}
unsafe fn enet_protocol_change_state<S: Socket>(
    mut _host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    state: ENetPeerState,
) {
    if state == ENET_PEER_STATE_CONNECTED as i32 as u32
        || state == ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        enet_peer_on_connect(peer);
    } else {
        enet_peer_on_disconnect(peer);
    }
    (*peer).state = state;
}
unsafe fn enet_protocol_dispatch_state<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    state: ENetPeerState,
) {
    enet_protocol_change_state(host, peer, state);
    if (*peer).flags as i32 & ENET_PEER_FLAG_NEEDS_DISPATCH as i32 == 0 {
        enet_list_insert(
            &mut (*host).dispatchQueue.sentinel,
            &mut (*peer).dispatchList as *mut ENetListNode as *mut c_void,
        );
        (*peer).flags = ((*peer).flags as i32 | ENET_PEER_FLAG_NEEDS_DISPATCH as i32) as u16;
    }
}
unsafe fn enet_protocol_dispatch_incoming_commands<S: Socket>(
    host: *mut ENetHost<S>,
    event: *mut ENetEvent<S>,
) -> i32 {
    while (*host).dispatchQueue.sentinel.next
        != &mut (*host).dispatchQueue.sentinel as *mut ENetListNode
    {
        let peer: *mut ENetPeer<S> =
            enet_list_remove((*host).dispatchQueue.sentinel.next) as *mut ENetPeer<S>;
        (*peer).flags = ((*peer).flags as i32 & !(ENET_PEER_FLAG_NEEDS_DISPATCH as i32)) as u16;
        match (*peer).state as u32 {
            3 | 4 => {
                enet_protocol_change_state(host, peer, ENET_PEER_STATE_CONNECTED);
                (*event).type_0 = ENET_EVENT_TYPE_CONNECT;
                (*event).peer = peer;
                (*event).data = (*peer).eventData;
                return 1_i32;
            }
            9 => {
                (*host).recalculateBandwidthLimits = 1_i32;
                (*event).type_0 = ENET_EVENT_TYPE_DISCONNECT;
                (*event).peer = peer;
                (*event).data = (*peer).eventData;
                enet_peer_reset(peer);
                return 1_i32;
            }
            5 => {
                if (*peer).dispatchedCommands.sentinel.next
                    == &mut (*peer).dispatchedCommands.sentinel as *mut ENetListNode
                {
                    continue;
                }
                (*event).packet = enet_peer_receive(peer, &mut (*event).channelID);
                if ((*event).packet).is_null() {
                    continue;
                }
                (*event).type_0 = ENET_EVENT_TYPE_RECEIVE;
                (*event).peer = peer;
                if (*peer).dispatchedCommands.sentinel.next
                    != &mut (*peer).dispatchedCommands.sentinel as *mut ENetListNode
                {
                    (*peer).flags =
                        ((*peer).flags as i32 | ENET_PEER_FLAG_NEEDS_DISPATCH as i32) as u16;
                    enet_list_insert(
                        &mut (*host).dispatchQueue.sentinel,
                        &mut (*peer).dispatchList as *mut ENetListNode as *mut c_void,
                    );
                }
                return 1_i32;
            }
            _ => {}
        }
    }
    0_i32
}
unsafe fn enet_protocol_notify_connect<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    event: *mut ENetEvent<S>,
) {
    (*host).recalculateBandwidthLimits = 1_i32;
    if !event.is_null() {
        enet_protocol_change_state(host, peer, ENET_PEER_STATE_CONNECTED);
        (*event).type_0 = ENET_EVENT_TYPE_CONNECT;
        (*event).peer = peer;
        (*event).data = (*peer).eventData;
    } else {
        enet_protocol_dispatch_state(
            host,
            peer,
            (if (*peer).state == ENET_PEER_STATE_CONNECTING as i32 as u32 {
                ENET_PEER_STATE_CONNECTION_SUCCEEDED as i32
            } else {
                ENET_PEER_STATE_CONNECTION_PENDING as i32
            }) as ENetPeerState,
        );
    };
}
unsafe fn enet_protocol_notify_disconnect<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    event: *mut ENetEvent<S>,
) {
    if (*peer).state >= ENET_PEER_STATE_CONNECTION_PENDING as i32 as u32 {
        (*host).recalculateBandwidthLimits = 1_i32;
    }
    if (*peer).state != ENET_PEER_STATE_CONNECTING as i32 as u32
        && (*peer).state < ENET_PEER_STATE_CONNECTION_SUCCEEDED as i32 as u32
    {
        enet_peer_reset(peer);
    } else if !event.is_null() {
        (*event).type_0 = ENET_EVENT_TYPE_DISCONNECT;
        (*event).peer = peer;
        (*event).data = 0_i32 as u32;
        enet_peer_reset(peer);
    } else {
        (*peer).eventData = 0_i32 as u32;
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
    };
}
unsafe fn enet_protocol_remove_sent_unreliable_commands<S: Socket>(
    peer: *mut ENetPeer<S>,
    sentUnreliableCommands: *mut ENetList,
) {
    let mut outgoingCommand: *mut ENetOutgoingCommand;
    if (*sentUnreliableCommands).sentinel.next
        == &mut (*sentUnreliableCommands).sentinel as *mut ENetListNode
    {
        return;
    }
    loop {
        outgoingCommand =
            (*sentUnreliableCommands).sentinel.next as *mut c_void as *mut ENetOutgoingCommand;
        enet_list_remove(&mut (*outgoingCommand).outgoingCommandList);
        if !((*outgoingCommand).packet).is_null() {
            (*(*outgoingCommand).packet).referenceCount =
                ((*(*outgoingCommand).packet).referenceCount).wrapping_sub(1);
            if (*(*outgoingCommand).packet).referenceCount == 0_i32 as usize {
                (*(*outgoingCommand).packet).flags |= ENET_PACKET_FLAG_SENT as i32 as u32;
                enet_packet_destroy((*outgoingCommand).packet);
            }
        }
        enet_free(outgoingCommand as *mut c_void);
        if (*sentUnreliableCommands).sentinel.next
            == &mut (*sentUnreliableCommands).sentinel as *mut ENetListNode
        {
            break;
        }
    }
    if (*peer).state == ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
        && enet_peer_has_outgoing_commands(peer) == 0
    {
        enet_peer_disconnect(peer, (*peer).eventData);
    }
}
unsafe fn enet_protocol_find_sent_reliable_command(
    list: *mut ENetList,
    reliableSequenceNumber: u16,
    channelID: u8,
) -> *mut ENetOutgoingCommand {
    let mut currentCommand: ENetListIterator;
    currentCommand = (*list).sentinel.next;
    while currentCommand != &mut (*list).sentinel as *mut ENetListNode {
        let outgoingCommand: *mut ENetOutgoingCommand = currentCommand as *mut ENetOutgoingCommand;
        if (*outgoingCommand).command.header.command as i32
            & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32
            != 0
        {
            if ((*outgoingCommand).sendAttempts as i32) < 1_i32 {
                break;
            }
            if (*outgoingCommand).reliableSequenceNumber as i32 == reliableSequenceNumber as i32
                && (*outgoingCommand).command.header.channelID as i32 == channelID as i32
            {
                return outgoingCommand;
            }
        }
        currentCommand = (*currentCommand).next;
    }
    std::ptr::null_mut()
}
unsafe fn enet_protocol_remove_sent_reliable_command<S: Socket>(
    peer: *mut ENetPeer<S>,
    reliableSequenceNumber: u16,
    channelID: u8,
) -> ENetProtocolCommand {
    let mut outgoingCommand: *mut ENetOutgoingCommand = std::ptr::null_mut();
    let mut currentCommand: ENetListIterator;
    let mut wasSent: i32 = 1_i32;
    currentCommand = (*peer).sentReliableCommands.sentinel.next;
    while currentCommand != &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode {
        outgoingCommand = currentCommand as *mut ENetOutgoingCommand;
        if (*outgoingCommand).reliableSequenceNumber as i32 == reliableSequenceNumber as i32
            && (*outgoingCommand).command.header.channelID as i32 == channelID as i32
        {
            break;
        }
        currentCommand = (*currentCommand).next;
    }
    if currentCommand == &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode {
        outgoingCommand = enet_protocol_find_sent_reliable_command(
            &mut (*peer).outgoingCommands,
            reliableSequenceNumber,
            channelID,
        );
        if outgoingCommand.is_null() {
            outgoingCommand = enet_protocol_find_sent_reliable_command(
                &mut (*peer).outgoingSendReliableCommands,
                reliableSequenceNumber,
                channelID,
            );
        }
        wasSent = 0_i32;
    }
    if outgoingCommand.is_null() {
        return ENET_PROTOCOL_COMMAND_NONE;
    }
    if (channelID as usize) < (*peer).channelCount {
        let channel: *mut ENetChannel = ((*peer).channels).offset(channelID as isize);
        let reliableWindow: u16 =
            (reliableSequenceNumber as i32 / ENET_PEER_RELIABLE_WINDOW_SIZE as i32) as u16;
        if (*channel).reliableWindows[reliableWindow as usize] as i32 > 0_i32 {
            (*channel).reliableWindows[reliableWindow as usize] =
                ((*channel).reliableWindows[reliableWindow as usize]).wrapping_sub(1);
            if (*channel).reliableWindows[reliableWindow as usize] == 0 {
                (*channel).usedReliableWindows = ((*channel).usedReliableWindows as i32
                    & !(1_i32 << reliableWindow as i32))
                    as u16;
            }
        }
    }
    let commandNumber = ((*outgoingCommand).command.header.command as i32
        & ENET_PROTOCOL_COMMAND_MASK as i32) as ENetProtocolCommand;
    enet_list_remove(&mut (*outgoingCommand).outgoingCommandList);
    if !((*outgoingCommand).packet).is_null() {
        if wasSent != 0 {
            (*peer).reliableDataInTransit = (*peer)
                .reliableDataInTransit
                .wrapping_sub((*outgoingCommand).fragmentLength as u32)
                as u32 as u32;
        }
        (*(*outgoingCommand).packet).referenceCount =
            ((*(*outgoingCommand).packet).referenceCount).wrapping_sub(1);
        if (*(*outgoingCommand).packet).referenceCount == 0_i32 as usize {
            (*(*outgoingCommand).packet).flags |= ENET_PACKET_FLAG_SENT as i32 as u32;
            enet_packet_destroy((*outgoingCommand).packet);
        }
    }
    enet_free(outgoingCommand as *mut c_void);
    if (*peer).sentReliableCommands.sentinel.next
        == &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode
    {
        return commandNumber;
    }
    outgoingCommand =
        (*peer).sentReliableCommands.sentinel.next as *mut c_void as *mut ENetOutgoingCommand;
    (*peer).nextTimeout =
        ((*outgoingCommand).sentTime).wrapping_add((*outgoingCommand).roundTripTimeout);
    commandNumber
}
unsafe fn enet_protocol_handle_connect<S: Socket>(
    host: *mut ENetHost<S>,
    mut _header: *mut ENetProtocolHeader,
    command: *mut ENetProtocol,
) -> *mut ENetPeer<S> {
    let mut incomingSessionID: u8;
    let mut outgoingSessionID: u8;
    let mut mtu: u32;
    let mut windowSize: u32;
    let mut channel: *mut ENetChannel;
    let mut channelCount: usize;
    let mut duplicatePeers: usize = 0_i32 as usize;
    let mut currentPeer: *mut ENetPeer<S>;
    let mut peer: *mut ENetPeer<S> = std::ptr::null_mut();
    let mut verifyCommand: ENetProtocol = ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    channelCount = u32::from_be((*command).connect.channelCount) as usize;
    if channelCount < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as i32 as usize
        || channelCount > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as i32 as usize
    {
        return std::ptr::null_mut();
    }
    currentPeer = (*host).peers;
    while currentPeer < ((*host).peers).add((*host).peerCount) {
        if (*currentPeer).state == ENET_PEER_STATE_DISCONNECTED as i32 as u32 {
            if peer.is_null() {
                peer = currentPeer;
            }
        } else if (*currentPeer).state != ENET_PEER_STATE_CONNECTING as i32 as u32
            && (*currentPeer)
                .address
                .assume_init_ref()
                .as_ref()
                .unwrap()
                .same_host((*host).receivedAddress.assume_init_ref().as_ref().unwrap())
        {
            if (*currentPeer)
                .address
                .assume_init_ref()
                .as_ref()
                .unwrap()
                .same((*host).receivedAddress.assume_init_ref().as_ref().unwrap())
                && (*currentPeer).connectID == (*command).connect.connectID
            {
                return std::ptr::null_mut();
            }
            duplicatePeers = duplicatePeers.wrapping_add(1);
        }
        currentPeer = currentPeer.offset(1);
    }
    if peer.is_null() || duplicatePeers >= (*host).duplicatePeers {
        return std::ptr::null_mut();
    }
    if channelCount > (*host).channelLimit {
        channelCount = (*host).channelLimit;
    }
    (*peer).channels = enet_malloc(channelCount.wrapping_mul(::core::mem::size_of::<ENetChannel>()))
        as *mut ENetChannel;
    if ((*peer).channels).is_null() {
        return std::ptr::null_mut();
    }
    (*peer).channelCount = channelCount;
    (*peer).state = ENET_PEER_STATE_ACKNOWLEDGING_CONNECT;
    (*peer).connectID = (*command).connect.connectID;
    *(*peer).address.assume_init_mut() = Some(
        (*host)
            .receivedAddress
            .assume_init_ref()
            .as_ref()
            .cloned()
            .unwrap(),
    );
    (*peer).mtu = (*host).mtu;
    (*peer).outgoingPeerID = u16::from_be((*command).connect.outgoingPeerID);
    (*peer).incomingBandwidth = u32::from_be((*command).connect.incomingBandwidth);
    (*peer).outgoingBandwidth = u32::from_be((*command).connect.outgoingBandwidth);
    (*peer).packetThrottleInterval = u32::from_be((*command).connect.packetThrottleInterval);
    (*peer).packetThrottleAcceleration =
        u32::from_be((*command).connect.packetThrottleAcceleration);
    (*peer).packetThrottleDeceleration =
        u32::from_be((*command).connect.packetThrottleDeceleration);
    (*peer).eventData = u32::from_be((*command).connect.data);
    incomingSessionID = (if (*command).connect.incomingSessionID as i32 == 0xff_i32 {
        (*peer).outgoingSessionID as i32
    } else {
        (*command).connect.incomingSessionID as i32
    }) as u8;
    incomingSessionID = ((incomingSessionID as i32 + 1_i32)
        & ENET_PROTOCOL_HEADER_SESSION_MASK as i32 >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as i32)
        as u8;
    if incomingSessionID as i32 == (*peer).outgoingSessionID as i32 {
        incomingSessionID = ((incomingSessionID as i32 + 1_i32)
            & ENET_PROTOCOL_HEADER_SESSION_MASK as i32 >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as i32)
            as u8;
    }
    (*peer).outgoingSessionID = incomingSessionID;
    outgoingSessionID = (if (*command).connect.outgoingSessionID as i32 == 0xff_i32 {
        (*peer).incomingSessionID as i32
    } else {
        (*command).connect.outgoingSessionID as i32
    }) as u8;
    outgoingSessionID = ((outgoingSessionID as i32 + 1_i32)
        & ENET_PROTOCOL_HEADER_SESSION_MASK as i32 >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as i32)
        as u8;
    if outgoingSessionID as i32 == (*peer).incomingSessionID as i32 {
        outgoingSessionID = ((outgoingSessionID as i32 + 1_i32)
            & ENET_PROTOCOL_HEADER_SESSION_MASK as i32 >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as i32)
            as u8;
    }
    (*peer).incomingSessionID = outgoingSessionID;
    channel = (*peer).channels;
    while channel < ((*peer).channels).add(channelCount) {
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
    mtu = u32::from_be((*command).connect.mtu);
    if mtu < ENET_PROTOCOL_MINIMUM_MTU as i32 as u32 {
        mtu = ENET_PROTOCOL_MINIMUM_MTU as i32 as u32;
    } else if mtu > ENET_PROTOCOL_MAXIMUM_MTU as i32 as u32 {
        mtu = ENET_PROTOCOL_MAXIMUM_MTU as i32 as u32;
    }
    if mtu < (*peer).mtu {
        (*peer).mtu = mtu;
    }
    if (*host).outgoingBandwidth == 0_i32 as u32 && (*peer).incomingBandwidth == 0_i32 as u32 {
        (*peer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    } else if (*host).outgoingBandwidth == 0_i32 as u32 || (*peer).incomingBandwidth == 0_i32 as u32
    {
        (*peer).windowSize = (if (*host).outgoingBandwidth > (*peer).incomingBandwidth {
            (*host).outgoingBandwidth
        } else {
            (*peer).incomingBandwidth
        })
        .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as i32 as u32)
        .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32);
    } else {
        (*peer).windowSize = (if (*host).outgoingBandwidth < (*peer).incomingBandwidth {
            (*host).outgoingBandwidth
        } else {
            (*peer).incomingBandwidth
        })
        .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as i32 as u32)
        .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32);
    }
    if (*peer).windowSize < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32 {
        (*peer).windowSize = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32;
    } else if (*peer).windowSize > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32 {
        (*peer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    }
    if (*host).incomingBandwidth == 0_i32 as u32 {
        windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    } else {
        windowSize = ((*host).incomingBandwidth)
            .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as i32 as u32)
            .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32);
    }
    if windowSize > u32::from_be((*command).connect.windowSize) {
        windowSize = u32::from_be((*command).connect.windowSize);
    }
    if windowSize < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32 {
        windowSize = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32;
    } else if windowSize > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32 {
        windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    }
    verifyCommand.header.command = (ENET_PROTOCOL_COMMAND_VERIFY_CONNECT as i32
        | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32) as u8;
    verifyCommand.header.channelID = 0xff_i32 as u8;
    verifyCommand.verifyConnect.outgoingPeerID = (*peer).incomingPeerID.to_be();
    verifyCommand.verifyConnect.incomingSessionID = incomingSessionID;
    verifyCommand.verifyConnect.outgoingSessionID = outgoingSessionID;
    verifyCommand.verifyConnect.mtu = (*peer).mtu.to_be();
    verifyCommand.verifyConnect.windowSize = windowSize.to_be();
    verifyCommand.verifyConnect.channelCount = (channelCount as u32).to_be();
    verifyCommand.verifyConnect.incomingBandwidth = (*host).incomingBandwidth.to_be();
    verifyCommand.verifyConnect.outgoingBandwidth = (*host).outgoingBandwidth.to_be();
    verifyCommand.verifyConnect.packetThrottleInterval = (*peer).packetThrottleInterval.to_be();
    verifyCommand.verifyConnect.packetThrottleAcceleration =
        (*peer).packetThrottleAcceleration.to_be();
    verifyCommand.verifyConnect.packetThrottleDeceleration =
        (*peer).packetThrottleDeceleration.to_be();
    verifyCommand.verifyConnect.connectID = (*peer).connectID;
    enet_peer_queue_outgoing_command(
        peer,
        &verifyCommand,
        std::ptr::null_mut(),
        0_i32 as u32,
        0_i32 as u16,
    );
    peer
}
unsafe fn enet_protocol_handle_send_reliable<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
    currentData: *mut *mut u8,
) -> i32 {
    if (*command).header.channelID as usize >= (*peer).channelCount
        || (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
            && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        return -1_i32;
    }
    let dataLength = u16::from_be((*command).sendReliable.dataLength) as usize;
    *currentData = (*currentData).add(dataLength);
    if dataLength > (*host).maximumPacketSize
        || *currentData < (*host).receivedData
        || *currentData > ((*host).receivedData).add((*host).receivedDataLength)
    {
        return -1_i32;
    }
    if (enet_peer_queue_incoming_command(
        peer,
        command,
        (command as *const u8)
            .offset(::core::mem::size_of::<ENetProtocolSendReliable>() as u64 as isize)
            as *const c_void,
        dataLength,
        ENET_PACKET_FLAG_RELIABLE as i32 as u32,
        0_i32 as u32,
    ))
    .is_null()
    {
        return -1_i32;
    }
    0_i32
}
unsafe fn enet_protocol_handle_send_unsequenced<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
    currentData: *mut *mut u8,
) -> i32 {
    let mut unsequencedGroup: u32;
    if (*command).header.channelID as usize >= (*peer).channelCount
        || (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
            && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        return -1_i32;
    }
    let dataLength = u16::from_be((*command).sendUnsequenced.dataLength) as usize;
    *currentData = (*currentData).add(dataLength);
    if dataLength > (*host).maximumPacketSize
        || *currentData < (*host).receivedData
        || *currentData > ((*host).receivedData).add((*host).receivedDataLength)
    {
        return -1_i32;
    }
    unsequencedGroup = u16::from_be((*command).sendUnsequenced.unsequencedGroup) as u32;
    let index = unsequencedGroup.wrapping_rem(ENET_PEER_UNSEQUENCED_WINDOW_SIZE as i32 as u32);
    if unsequencedGroup < (*peer).incomingUnsequencedGroup as u32 {
        unsequencedGroup = unsequencedGroup.wrapping_add(0x10000_i32 as u32);
    }
    if unsequencedGroup
        >= ((*peer).incomingUnsequencedGroup as u32).wrapping_add(
            (ENET_PEER_FREE_UNSEQUENCED_WINDOWS as i32 * ENET_PEER_UNSEQUENCED_WINDOW_SIZE as i32)
                as u32,
        )
    {
        return 0_i32;
    }
    unsequencedGroup &= 0xffff_i32 as u32;
    if unsequencedGroup.wrapping_sub(index) != (*peer).incomingUnsequencedGroup as u32 {
        (*peer).incomingUnsequencedGroup = unsequencedGroup.wrapping_sub(index) as u16;
        _enet_memset(
            ((*peer).unsequencedWindow).as_mut_ptr() as *mut c_void,
            0_i32,
            ::core::mem::size_of::<[u32; 32]>(),
        );
    } else if (*peer).unsequencedWindow[index.wrapping_div(32_i32 as u32) as usize]
        & (1_i32 << index.wrapping_rem(32_i32 as u32)) as u32
        != 0
    {
        return 0_i32;
    }
    if (enet_peer_queue_incoming_command(
        peer,
        command,
        (command as *const u8)
            .offset(::core::mem::size_of::<ENetProtocolSendUnsequenced>() as u64 as isize)
            as *const c_void,
        dataLength,
        ENET_PACKET_FLAG_UNSEQUENCED as i32 as u32,
        0_i32 as u32,
    ))
    .is_null()
    {
        return -1_i32;
    }
    (*peer).unsequencedWindow[index.wrapping_div(32_i32 as u32) as usize] |=
        (1_i32 << index.wrapping_rem(32_i32 as u32)) as u32;
    0_i32
}
unsafe fn enet_protocol_handle_send_unreliable<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
    currentData: *mut *mut u8,
) -> i32 {
    if (*command).header.channelID as usize >= (*peer).channelCount
        || (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
            && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        return -1_i32;
    }
    let dataLength = u16::from_be((*command).sendUnreliable.dataLength) as usize;
    *currentData = (*currentData).add(dataLength);
    if dataLength > (*host).maximumPacketSize
        || *currentData < (*host).receivedData
        || *currentData > ((*host).receivedData).add((*host).receivedDataLength)
    {
        return -1_i32;
    }
    if (enet_peer_queue_incoming_command(
        peer,
        command,
        (command as *const u8)
            .offset(::core::mem::size_of::<ENetProtocolSendUnreliable>() as u64 as isize)
            as *const c_void,
        dataLength,
        0_i32 as u32,
        0_i32 as u32,
    ))
    .is_null()
    {
        return -1_i32;
    }
    0_i32
}
unsafe fn enet_protocol_handle_send_fragment<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
    currentData: *mut *mut u8,
) -> i32 {
    let mut fragmentLength: u32;
    let mut startWindow: u16;
    let mut currentCommand: ENetListIterator;
    let mut startCommand: *mut ENetIncomingCommand = std::ptr::null_mut();
    if (*command).header.channelID as usize >= (*peer).channelCount
        || (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
            && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        return -1_i32;
    }
    fragmentLength = u16::from_be((*command).sendFragment.dataLength) as u32;
    *currentData = (*currentData).offset(fragmentLength as isize);
    if fragmentLength <= 0_i32 as u32
        || fragmentLength as usize > (*host).maximumPacketSize
        || *currentData < (*host).receivedData
        || *currentData > ((*host).receivedData).add((*host).receivedDataLength)
    {
        return -1_i32;
    }
    let channel = ((*peer).channels).offset((*command).header.channelID as isize);
    let startSequenceNumber = u16::from_be((*command).sendFragment.startSequenceNumber) as u32;
    startWindow =
        startSequenceNumber.wrapping_div(ENET_PEER_RELIABLE_WINDOW_SIZE as i32 as u32) as u16;
    let currentWindow = ((*channel).incomingReliableSequenceNumber as i32
        / ENET_PEER_RELIABLE_WINDOW_SIZE as i32) as u16;
    if startSequenceNumber < (*channel).incomingReliableSequenceNumber as u32 {
        startWindow = (startWindow as i32 + ENET_PEER_RELIABLE_WINDOWS as i32) as u16;
    }
    if (startWindow as i32) < currentWindow as i32
        || startWindow as i32
            >= currentWindow as i32 + ENET_PEER_FREE_RELIABLE_WINDOWS as i32 - 1_i32
    {
        return 0_i32;
    }
    let fragmentNumber = u32::from_be((*command).sendFragment.fragmentNumber);
    let fragmentCount = u32::from_be((*command).sendFragment.fragmentCount);
    let fragmentOffset = u32::from_be((*command).sendFragment.fragmentOffset);
    let totalLength = u32::from_be((*command).sendFragment.totalLength);
    if fragmentCount > ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as i32 as u32
        || fragmentNumber >= fragmentCount
        || totalLength as usize > (*host).maximumPacketSize
        || totalLength < fragmentCount
        || fragmentOffset >= totalLength
        || fragmentLength > totalLength.wrapping_sub(fragmentOffset)
    {
        return -1_i32;
    }
    let mut current_block_23: u64;
    currentCommand = (*channel).incomingReliableCommands.sentinel.previous;
    while currentCommand != &mut (*channel).incomingReliableCommands.sentinel as *mut ENetListNode {
        let incomingCommand: *mut ENetIncomingCommand = currentCommand as *mut ENetIncomingCommand;
        if startSequenceNumber >= (*channel).incomingReliableSequenceNumber as u32 {
            if ((*incomingCommand).reliableSequenceNumber as i32)
                < (*channel).incomingReliableSequenceNumber as i32
            {
                current_block_23 = 13056961889198038528;
            } else {
                current_block_23 = 12147880666119273379;
            }
        } else {
            if (*incomingCommand).reliableSequenceNumber as i32
                >= (*channel).incomingReliableSequenceNumber as i32
            {
                break;
            }
            current_block_23 = 12147880666119273379;
        }
        if let 12147880666119273379 = current_block_23 {
            if (*incomingCommand).reliableSequenceNumber as u32 <= startSequenceNumber {
                if ((*incomingCommand).reliableSequenceNumber as u32) < startSequenceNumber {
                    break;
                }
                if (*incomingCommand).command.header.command as i32
                    & ENET_PROTOCOL_COMMAND_MASK as i32
                    != ENET_PROTOCOL_COMMAND_SEND_FRAGMENT as i32
                    || totalLength as usize != (*(*incomingCommand).packet).dataLength
                    || fragmentCount != (*incomingCommand).fragmentCount
                {
                    return -1_i32;
                }
                startCommand = incomingCommand;
                break;
            }
        }
        currentCommand = (*currentCommand).previous;
    }
    if startCommand.is_null() {
        let mut hostCommand: ENetProtocol = *command;
        hostCommand.header.reliableSequenceNumber = startSequenceNumber as u16;
        startCommand = enet_peer_queue_incoming_command(
            peer,
            &hostCommand,
            std::ptr::null(),
            totalLength as usize,
            ENET_PACKET_FLAG_RELIABLE as i32 as u32,
            fragmentCount,
        );
        if startCommand.is_null() {
            return -1_i32;
        }
    }
    if *((*startCommand).fragments).offset(fragmentNumber.wrapping_div(32_i32 as u32) as isize)
        & (1_i32 << fragmentNumber.wrapping_rem(32_i32 as u32)) as u32
        == 0_i32 as u32
    {
        (*startCommand).fragmentsRemaining = ((*startCommand).fragmentsRemaining).wrapping_sub(1);
        let fresh32 =
            ((*startCommand).fragments).offset(fragmentNumber.wrapping_div(32_i32 as u32) as isize);
        *fresh32 |= (1_i32 << fragmentNumber.wrapping_rem(32_i32 as u32)) as u32;
        if fragmentOffset.wrapping_add(fragmentLength) as usize
            > (*(*startCommand).packet).dataLength
        {
            fragmentLength =
                ((*(*startCommand).packet).dataLength).wrapping_sub(fragmentOffset as usize) as u32;
        }
        _enet_memcpy(
            ((*(*startCommand).packet).data).offset(fragmentOffset as isize) as *mut c_void,
            (command as *mut u8)
                .offset(::core::mem::size_of::<ENetProtocolSendFragment>() as u64 as isize)
                as *const c_void,
            fragmentLength as usize,
        );
        if (*startCommand).fragmentsRemaining <= 0_i32 as u32 {
            enet_peer_dispatch_incoming_reliable_commands(peer, channel, std::ptr::null_mut());
        }
    }
    0_i32
}
unsafe fn enet_protocol_handle_send_unreliable_fragment<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
    currentData: *mut *mut u8,
) -> i32 {
    let mut fragmentLength: u32;
    let mut reliableWindow: u16;
    let mut currentCommand: ENetListIterator;
    let mut startCommand: *mut ENetIncomingCommand = std::ptr::null_mut();
    if (*command).header.channelID as usize >= (*peer).channelCount
        || (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
            && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        return -1_i32;
    }
    fragmentLength = u16::from_be((*command).sendFragment.dataLength) as u32;
    *currentData = (*currentData).offset(fragmentLength as isize);
    if fragmentLength as usize > (*host).maximumPacketSize
        || *currentData < (*host).receivedData
        || *currentData > ((*host).receivedData).add((*host).receivedDataLength)
    {
        return -1_i32;
    }
    let channel = ((*peer).channels).offset((*command).header.channelID as isize);
    let reliableSequenceNumber = (*command).header.reliableSequenceNumber as u32;
    let startSequenceNumber = u16::from_be((*command).sendFragment.startSequenceNumber) as u32;
    reliableWindow =
        reliableSequenceNumber.wrapping_div(ENET_PEER_RELIABLE_WINDOW_SIZE as i32 as u32) as u16;
    let currentWindow = ((*channel).incomingReliableSequenceNumber as i32
        / ENET_PEER_RELIABLE_WINDOW_SIZE as i32) as u16;
    if reliableSequenceNumber < (*channel).incomingReliableSequenceNumber as u32 {
        reliableWindow = (reliableWindow as i32 + ENET_PEER_RELIABLE_WINDOWS as i32) as u16;
    }
    if (reliableWindow as i32) < currentWindow as i32
        || reliableWindow as i32
            >= currentWindow as i32 + ENET_PEER_FREE_RELIABLE_WINDOWS as i32 - 1_i32
    {
        return 0_i32;
    }
    if reliableSequenceNumber == (*channel).incomingReliableSequenceNumber as u32
        && startSequenceNumber <= (*channel).incomingUnreliableSequenceNumber as u32
    {
        return 0_i32;
    }
    let fragmentNumber = u32::from_be((*command).sendFragment.fragmentNumber);
    let fragmentCount = u32::from_be((*command).sendFragment.fragmentCount);
    let fragmentOffset = u32::from_be((*command).sendFragment.fragmentOffset);
    let totalLength = u32::from_be((*command).sendFragment.totalLength);
    if fragmentCount > ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as i32 as u32
        || fragmentNumber >= fragmentCount
        || totalLength as usize > (*host).maximumPacketSize
        || fragmentOffset >= totalLength
        || fragmentLength > totalLength.wrapping_sub(fragmentOffset)
    {
        return -1_i32;
    }
    let mut current_block_26: u64;
    currentCommand = (*channel).incomingUnreliableCommands.sentinel.previous;
    while currentCommand != &mut (*channel).incomingUnreliableCommands.sentinel as *mut ENetListNode
    {
        let incomingCommand: *mut ENetIncomingCommand = currentCommand as *mut ENetIncomingCommand;
        if reliableSequenceNumber >= (*channel).incomingReliableSequenceNumber as u32 {
            if ((*incomingCommand).reliableSequenceNumber as i32)
                < (*channel).incomingReliableSequenceNumber as i32
            {
                current_block_26 = 8457315219000651999;
            } else {
                current_block_26 = 1109700713171191020;
            }
        } else {
            if (*incomingCommand).reliableSequenceNumber as i32
                >= (*channel).incomingReliableSequenceNumber as i32
            {
                break;
            }
            current_block_26 = 1109700713171191020;
        }
        if let 1109700713171191020 = current_block_26 {
            if ((*incomingCommand).reliableSequenceNumber as u32) < reliableSequenceNumber {
                break;
            }
            if (*incomingCommand).reliableSequenceNumber as u32 <= reliableSequenceNumber
                && (*incomingCommand).unreliableSequenceNumber as u32 <= startSequenceNumber
            {
                if ((*incomingCommand).unreliableSequenceNumber as u32) < startSequenceNumber {
                    break;
                }
                if (*incomingCommand).command.header.command as i32
                    & ENET_PROTOCOL_COMMAND_MASK as i32
                    != ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT as i32
                    || totalLength as usize != (*(*incomingCommand).packet).dataLength
                    || fragmentCount != (*incomingCommand).fragmentCount
                {
                    return -1_i32;
                }
                startCommand = incomingCommand;
                break;
            }
        }
        currentCommand = (*currentCommand).previous;
    }
    if startCommand.is_null() {
        startCommand = enet_peer_queue_incoming_command(
            peer,
            command,
            std::ptr::null(),
            totalLength as usize,
            ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT as i32 as u32,
            fragmentCount,
        );
        if startCommand.is_null() {
            return -1_i32;
        }
    }
    if *((*startCommand).fragments).offset(fragmentNumber.wrapping_div(32_i32 as u32) as isize)
        & (1_i32 << fragmentNumber.wrapping_rem(32_i32 as u32)) as u32
        == 0_i32 as u32
    {
        (*startCommand).fragmentsRemaining = ((*startCommand).fragmentsRemaining).wrapping_sub(1);
        let fresh33 =
            ((*startCommand).fragments).offset(fragmentNumber.wrapping_div(32_i32 as u32) as isize);
        *fresh33 |= (1_i32 << fragmentNumber.wrapping_rem(32_i32 as u32)) as u32;
        if fragmentOffset.wrapping_add(fragmentLength) as usize
            > (*(*startCommand).packet).dataLength
        {
            fragmentLength =
                ((*(*startCommand).packet).dataLength).wrapping_sub(fragmentOffset as usize) as u32;
        }
        _enet_memcpy(
            ((*(*startCommand).packet).data).offset(fragmentOffset as isize) as *mut c_void,
            (command as *mut u8)
                .offset(::core::mem::size_of::<ENetProtocolSendFragment>() as u64 as isize)
                as *const c_void,
            fragmentLength as usize,
        );
        if (*startCommand).fragmentsRemaining <= 0_i32 as u32 {
            enet_peer_dispatch_incoming_unreliable_commands(peer, channel, std::ptr::null_mut());
        }
    }
    0_i32
}
unsafe fn enet_protocol_handle_ping<S: Socket>(
    mut _host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    mut _command: *const ENetProtocol,
) -> i32 {
    if (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
        && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        return -1_i32;
    }
    0_i32
}
unsafe fn enet_protocol_handle_bandwidth_limit<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
) -> i32 {
    if (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
        && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        return -1_i32;
    }
    if (*peer).incomingBandwidth != 0_i32 as u32 {
        (*host).bandwidthLimitedPeers = ((*host).bandwidthLimitedPeers).wrapping_sub(1);
    }
    (*peer).incomingBandwidth = u32::from_be((*command).bandwidthLimit.incomingBandwidth);
    (*peer).outgoingBandwidth = u32::from_be((*command).bandwidthLimit.outgoingBandwidth);
    if (*peer).incomingBandwidth != 0_i32 as u32 {
        (*host).bandwidthLimitedPeers = ((*host).bandwidthLimitedPeers).wrapping_add(1);
    }
    if (*peer).incomingBandwidth == 0_i32 as u32 && (*host).outgoingBandwidth == 0_i32 as u32 {
        (*peer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    } else if (*peer).incomingBandwidth == 0_i32 as u32 || (*host).outgoingBandwidth == 0_i32 as u32
    {
        (*peer).windowSize = (if (*peer).incomingBandwidth > (*host).outgoingBandwidth {
            (*peer).incomingBandwidth
        } else {
            (*host).outgoingBandwidth
        })
        .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as i32 as u32)
        .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32);
    } else {
        (*peer).windowSize = (if (*peer).incomingBandwidth < (*host).outgoingBandwidth {
            (*peer).incomingBandwidth
        } else {
            (*host).outgoingBandwidth
        })
        .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as i32 as u32)
        .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32);
    }
    if (*peer).windowSize < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32 {
        (*peer).windowSize = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32;
    } else if (*peer).windowSize > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32 {
        (*peer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    }
    0_i32
}
unsafe fn enet_protocol_handle_throttle_configure<S: Socket>(
    mut _host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
) -> i32 {
    if (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
        && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        return -1_i32;
    }
    (*peer).packetThrottleInterval =
        u32::from_be((*command).throttleConfigure.packetThrottleInterval);
    (*peer).packetThrottleAcceleration =
        u32::from_be((*command).throttleConfigure.packetThrottleAcceleration);
    (*peer).packetThrottleDeceleration =
        u32::from_be((*command).throttleConfigure.packetThrottleDeceleration);
    0_i32
}
unsafe fn enet_protocol_handle_disconnect<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
) -> i32 {
    if (*peer).state == ENET_PEER_STATE_DISCONNECTED as i32 as u32
        || (*peer).state == ENET_PEER_STATE_ZOMBIE as i32 as u32
        || (*peer).state == ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT as i32 as u32
    {
        return 0_i32;
    }
    enet_peer_reset_queues(peer);
    if (*peer).state == ENET_PEER_STATE_CONNECTION_SUCCEEDED as i32 as u32
        || (*peer).state == ENET_PEER_STATE_DISCONNECTING as i32 as u32
        || (*peer).state == ENET_PEER_STATE_CONNECTING as i32 as u32
    {
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
    } else if (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
        && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        if (*peer).state == ENET_PEER_STATE_CONNECTION_PENDING as i32 as u32 {
            (*host).recalculateBandwidthLimits = 1_i32;
        }
        enet_peer_reset(peer);
    } else if (*command).header.command as i32 & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32 != 0
    {
        enet_protocol_change_state(host, peer, ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT);
    } else {
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
    }
    if (*peer).state != ENET_PEER_STATE_DISCONNECTED as i32 as u32 {
        (*peer).eventData = u32::from_be((*command).disconnect.data);
    }
    0_i32
}
unsafe fn enet_protocol_handle_acknowledge<S: Socket>(
    host: *mut ENetHost<S>,
    event: *mut ENetEvent<S>,
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
) -> i32 {
    let mut roundTripTime: u32;
    let mut receivedSentTime: u32;
    if (*peer).state == ENET_PEER_STATE_DISCONNECTED as i32 as u32
        || (*peer).state == ENET_PEER_STATE_ZOMBIE as i32 as u32
    {
        return 0_i32;
    }
    receivedSentTime = u16::from_be((*command).acknowledge.receivedSentTime) as u32;
    receivedSentTime |= (*host).serviceTime & 0xffff0000_u32;
    if receivedSentTime & 0x8000_i32 as u32 > (*host).serviceTime & 0x8000_i32 as u32 {
        receivedSentTime = receivedSentTime.wrapping_sub(0x10000_i32 as u32);
    }
    if ((*host).serviceTime).wrapping_sub(receivedSentTime) >= 86400000_i32 as u32 {
        return 0_i32;
    }
    roundTripTime = if ((*host).serviceTime).wrapping_sub(receivedSentTime) >= 86400000_i32 as u32 {
        receivedSentTime.wrapping_sub((*host).serviceTime)
    } else {
        ((*host).serviceTime).wrapping_sub(receivedSentTime)
    };
    roundTripTime = if roundTripTime > 1_i32 as u32 {
        roundTripTime
    } else {
        1_i32 as u32
    };
    if (*peer).lastReceiveTime > 0_i32 as u32 {
        enet_peer_throttle(peer, roundTripTime);
        (*peer).roundTripTimeVariance = (*peer)
            .roundTripTimeVariance
            .wrapping_sub(((*peer).roundTripTimeVariance).wrapping_div(4_i32 as u32));
        if roundTripTime >= (*peer).roundTripTime {
            let diff: u32 = roundTripTime.wrapping_sub((*peer).roundTripTime);
            (*peer).roundTripTimeVariance = (*peer)
                .roundTripTimeVariance
                .wrapping_add(diff.wrapping_div(4_i32 as u32));
            (*peer).roundTripTime = (*peer)
                .roundTripTime
                .wrapping_add(diff.wrapping_div(8_i32 as u32));
        } else {
            let diff_0: u32 = ((*peer).roundTripTime).wrapping_sub(roundTripTime);
            (*peer).roundTripTimeVariance = (*peer)
                .roundTripTimeVariance
                .wrapping_add(diff_0.wrapping_div(4_i32 as u32));
            (*peer).roundTripTime = (*peer)
                .roundTripTime
                .wrapping_sub(diff_0.wrapping_div(8_i32 as u32));
        }
    } else {
        (*peer).roundTripTime = roundTripTime;
        (*peer).roundTripTimeVariance = roundTripTime
            .wrapping_add(1_i32 as u32)
            .wrapping_div(2_i32 as u32);
    }
    if (*peer).roundTripTime < (*peer).lowestRoundTripTime {
        (*peer).lowestRoundTripTime = (*peer).roundTripTime;
    }
    if (*peer).roundTripTimeVariance > (*peer).highestRoundTripTimeVariance {
        (*peer).highestRoundTripTimeVariance = (*peer).roundTripTimeVariance;
    }
    if (*peer).packetThrottleEpoch == 0_i32 as u32
        || (if ((*host).serviceTime).wrapping_sub((*peer).packetThrottleEpoch)
            >= 86400000_i32 as u32
        {
            ((*peer).packetThrottleEpoch).wrapping_sub((*host).serviceTime)
        } else {
            ((*host).serviceTime).wrapping_sub((*peer).packetThrottleEpoch)
        }) >= (*peer).packetThrottleInterval
    {
        (*peer).lastRoundTripTime = (*peer).lowestRoundTripTime;
        (*peer).lastRoundTripTimeVariance = if (*peer).highestRoundTripTimeVariance > 1_i32 as u32 {
            (*peer).highestRoundTripTimeVariance
        } else {
            1_i32 as u32
        };
        (*peer).lowestRoundTripTime = (*peer).roundTripTime;
        (*peer).highestRoundTripTimeVariance = (*peer).roundTripTimeVariance;
        (*peer).packetThrottleEpoch = (*host).serviceTime;
    }
    (*peer).lastReceiveTime = if (*host).serviceTime > 1_i32 as u32 {
        (*host).serviceTime
    } else {
        1_i32 as u32
    };
    (*peer).earliestTimeout = 0_i32 as u32;
    let receivedReliableSequenceNumber =
        u16::from_be((*command).acknowledge.receivedReliableSequenceNumber) as u32;
    let commandNumber = enet_protocol_remove_sent_reliable_command(
        peer,
        receivedReliableSequenceNumber as u16,
        (*command).header.channelID,
    );
    match (*peer).state {
        2 => {
            if commandNumber as u32 != ENET_PROTOCOL_COMMAND_VERIFY_CONNECT as i32 as u32 {
                return -1_i32;
            }
            enet_protocol_notify_connect(host, peer, event);
        }
        7 => {
            if commandNumber as u32 != ENET_PROTOCOL_COMMAND_DISCONNECT as i32 as u32 {
                return -1_i32;
            }
            enet_protocol_notify_disconnect(host, peer, event);
        }
        6 => {
            if enet_peer_has_outgoing_commands(peer) == 0 {
                enet_peer_disconnect(peer, (*peer).eventData);
            }
        }
        _ => {}
    }
    0_i32
}
unsafe fn enet_protocol_handle_verify_connect<S: Socket>(
    host: *mut ENetHost<S>,
    event: *mut ENetEvent<S>,
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
) -> i32 {
    let mut mtu: u32;
    let mut windowSize: u32;
    if (*peer).state != ENET_PEER_STATE_CONNECTING as i32 as u32 {
        return 0_i32;
    }
    let channelCount = u32::from_be((*command).verifyConnect.channelCount) as usize;
    if channelCount < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as i32 as usize
        || channelCount > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as i32 as usize
        || u32::from_be((*command).verifyConnect.packetThrottleInterval)
            != (*peer).packetThrottleInterval
        || u32::from_be((*command).verifyConnect.packetThrottleAcceleration)
            != (*peer).packetThrottleAcceleration
        || u32::from_be((*command).verifyConnect.packetThrottleDeceleration)
            != (*peer).packetThrottleDeceleration
        || (*command).verifyConnect.connectID != (*peer).connectID
    {
        (*peer).eventData = 0_i32 as u32;
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
        return -1_i32;
    }
    enet_protocol_remove_sent_reliable_command(peer, 1_i32 as u16, 0xff_i32 as u8);
    if channelCount < (*peer).channelCount {
        (*peer).channelCount = channelCount;
    }
    (*peer).outgoingPeerID = u16::from_be((*command).verifyConnect.outgoingPeerID);
    (*peer).incomingSessionID = (*command).verifyConnect.incomingSessionID;
    (*peer).outgoingSessionID = (*command).verifyConnect.outgoingSessionID;
    mtu = u32::from_be((*command).verifyConnect.mtu);
    if mtu < ENET_PROTOCOL_MINIMUM_MTU as i32 as u32 {
        mtu = ENET_PROTOCOL_MINIMUM_MTU as i32 as u32;
    } else if mtu > ENET_PROTOCOL_MAXIMUM_MTU as i32 as u32 {
        mtu = ENET_PROTOCOL_MAXIMUM_MTU as i32 as u32;
    }
    if mtu < (*peer).mtu {
        (*peer).mtu = mtu;
    }
    windowSize = u32::from_be((*command).verifyConnect.windowSize);
    if windowSize < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32 {
        windowSize = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32;
    }
    if windowSize > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32 {
        windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    }
    if windowSize < (*peer).windowSize {
        (*peer).windowSize = windowSize;
    }
    (*peer).incomingBandwidth = u32::from_be((*command).verifyConnect.incomingBandwidth);
    (*peer).outgoingBandwidth = u32::from_be((*command).verifyConnect.outgoingBandwidth);
    enet_protocol_notify_connect(host, peer, event);
    0_i32
}
unsafe fn enet_protocol_handle_incoming_commands<S: Socket>(
    host: *mut ENetHost<S>,
    event: *mut ENetEvent<S>,
) -> i32 {
    let mut command: *mut ENetProtocol;
    let mut peer: *mut ENetPeer<S>;
    let mut currentData: *mut u8;
    let mut headerSize: usize;
    let mut peerID: u16;
    if (*host).receivedDataLength < 2_usize {
        return 0_i32;
    }
    let header = (*host).receivedData as *mut ENetProtocolHeader;
    peerID = u16::from_be((*header).peerID);
    let sessionID = ((peerID as i32 & ENET_PROTOCOL_HEADER_SESSION_MASK as i32)
        >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as i32) as u8;
    let flags = (peerID as i32 & ENET_PROTOCOL_HEADER_FLAG_MASK as i32) as u16;
    peerID = (peerID as i32
        & !(ENET_PROTOCOL_HEADER_FLAG_MASK as i32 | ENET_PROTOCOL_HEADER_SESSION_MASK as i32))
        as u16;
    headerSize = if flags as i32 & ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as i32 != 0 {
        ::core::mem::size_of::<ENetProtocolHeader>()
    } else {
        2_usize
    };
    if ((*host).checksum.assume_init_ref()).is_some() {
        headerSize =
            (headerSize as u64).wrapping_add(::core::mem::size_of::<u32>() as u64) as usize;
    }
    if peerID as i32 == ENET_PROTOCOL_MAXIMUM_PEER_ID as i32 {
        peer = std::ptr::null_mut();
    } else if peerID as usize >= (*host).peerCount {
        return 0_i32;
    } else {
        peer = ((*host).peers).offset(peerID as isize);
        if (*peer).state == ENET_PEER_STATE_DISCONNECTED as i32 as u32
            || (*peer).state == ENET_PEER_STATE_ZOMBIE as i32 as u32
            || !(*host)
                .receivedAddress
                .assume_init_ref()
                .as_ref()
                .unwrap()
                .same((*peer).address.assume_init_ref().as_ref().unwrap())
                && !(*peer)
                    .address
                    .assume_init_ref()
                    .as_ref()
                    .unwrap()
                    .is_broadcast()
            || ((*peer).outgoingPeerID as i32) < ENET_PROTOCOL_MAXIMUM_PEER_ID as i32
                && sessionID as i32 != (*peer).incomingSessionID as i32
        {
            return 0_i32;
        }
    }
    if flags as i32 & ENET_PROTOCOL_HEADER_FLAG_COMPRESSED as i32 != 0 {
        let Some(compressor) = (*host).compressor.assume_init_mut() else {
            return 0_i32;
        };
        let in_data = std::slice::from_raw_parts(
            ((*host).receivedData).add(headerSize),
            ((*host).receivedDataLength).wrapping_sub(headerSize),
        );
        let out = std::slice::from_raw_parts_mut(
            ((*host).packetData[1_i32 as usize])
                .as_mut_ptr()
                .add(headerSize),
            ::core::mem::size_of::<[u8; 4096]>().wrapping_sub(headerSize),
        );
        let originalSize = compressor.decompress(in_data, out);
        if originalSize <= 0_i32 as usize
            || originalSize > ::core::mem::size_of::<[u8; 4096]>().wrapping_sub(headerSize)
        {
            return 0_i32;
        }
        _enet_memcpy(
            ((*host).packetData[1_i32 as usize]).as_mut_ptr() as *mut c_void,
            header as *const c_void,
            headerSize,
        );
        (*host).receivedData = ((*host).packetData[1_i32 as usize]).as_mut_ptr();
        (*host).receivedDataLength = headerSize.wrapping_add(originalSize);
    }
    if let Some(checksum_fn) = (*host).checksum.assume_init_ref() {
        let checksum_addr: *mut u8 =
            ((*host).receivedData).add(headerSize.wrapping_sub(::core::mem::size_of::<u32>()));
        let mut desiredChecksum: u32 = 0;
        _enet_memcpy(
            &mut desiredChecksum as *mut u32 as *mut c_void,
            checksum_addr as *const c_void,
            ::core::mem::size_of::<u32>(),
        );
        let mut buffer: ENetBuffer = ENetBuffer {
            data: std::ptr::null_mut(),
            dataLength: 0,
        };
        let checksum = if !peer.is_null() {
            (*peer).connectID
        } else {
            0_i32 as u32
        };
        _enet_memcpy(
            checksum_addr as *mut c_void,
            &checksum as *const u32 as *const c_void,
            ::core::mem::size_of::<u32>(),
        );
        buffer.data = (*host).receivedData as *mut c_void;
        buffer.dataLength = (*host).receivedDataLength;
        let inBuffers = vec![std::slice::from_raw_parts(
            buffer.data as *mut u8,
            buffer.dataLength,
        )];
        if checksum_fn(inBuffers) != desiredChecksum {
            return 0_i32;
        }
    }
    if !peer.is_null() {
        *(*peer).address.assume_init_mut() = Some(
            (*host)
                .receivedAddress
                .assume_init_ref()
                .as_ref()
                .cloned()
                .unwrap(),
        );
        (*peer).incomingDataTotal =
            ((*peer).incomingDataTotal as usize).wrapping_add((*host).receivedDataLength) as u32;
    }
    currentData = ((*host).receivedData).add(headerSize);
    while currentData < ((*host).receivedData).add((*host).receivedDataLength) {
        command = currentData as *mut ENetProtocol;
        if currentData.offset(::core::mem::size_of::<ENetProtocolCommandHeader>() as u64 as isize)
            > ((*host).receivedData).add((*host).receivedDataLength)
        {
            break;
        }
        let commandNumber =
            ((*command).header.command as i32 & ENET_PROTOCOL_COMMAND_MASK as i32) as u8;
        if commandNumber as i32 >= ENET_PROTOCOL_COMMAND_COUNT as i32 {
            break;
        }
        let commandSize = COMMAND_SIZES[commandNumber as usize];
        if commandSize == 0_i32 as usize
            || currentData.add(commandSize) > ((*host).receivedData).add((*host).receivedDataLength)
        {
            break;
        }
        currentData = currentData.add(commandSize);
        if peer.is_null() && commandNumber as i32 != ENET_PROTOCOL_COMMAND_CONNECT as i32 {
            break;
        }
        (*command).header.reliableSequenceNumber =
            u16::from_be((*command).header.reliableSequenceNumber);
        match commandNumber as i32 {
            1 => {
                if enet_protocol_handle_acknowledge(host, event, peer, command) != 0 {
                    break;
                }
            }
            2 => {
                if !peer.is_null() {
                    break;
                }
                peer = enet_protocol_handle_connect(host, header, command);
                if peer.is_null() {
                    break;
                }
            }
            3 => {
                if enet_protocol_handle_verify_connect(host, event, peer, command) != 0 {
                    break;
                }
            }
            4 => {
                if enet_protocol_handle_disconnect(host, peer, command) != 0 {
                    break;
                }
            }
            5 => {
                if enet_protocol_handle_ping(host, peer, command) != 0 {
                    break;
                }
            }
            6 => {
                if enet_protocol_handle_send_reliable(host, peer, command, &mut currentData) != 0 {
                    break;
                }
            }
            7 => {
                if enet_protocol_handle_send_unreliable(host, peer, command, &mut currentData) != 0
                {
                    break;
                }
            }
            9 => {
                if enet_protocol_handle_send_unsequenced(host, peer, command, &mut currentData) != 0
                {
                    break;
                }
            }
            8 => {
                if enet_protocol_handle_send_fragment(host, peer, command, &mut currentData) != 0 {
                    break;
                }
            }
            10 => {
                if enet_protocol_handle_bandwidth_limit(host, peer, command) != 0 {
                    break;
                }
            }
            11 => {
                if enet_protocol_handle_throttle_configure(host, peer, command) != 0 {
                    break;
                }
            }
            12 => {
                if enet_protocol_handle_send_unreliable_fragment(
                    host,
                    peer,
                    command,
                    &mut currentData,
                ) != 0
                {
                    break;
                }
            }
            _ => {
                break;
            }
        }
        if peer.is_null()
            || (*command).header.command as i32 & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32
                == 0_i32
        {
            continue;
        }
        if flags as i32 & ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as i32 == 0 {
            break;
        }
        let sentTime = u16::from_be((*header).sentTime);
        match (*peer).state {
            7 | 2 | 0 | 9 => {}
            8 => {
                if (*command).header.command as i32 & ENET_PROTOCOL_COMMAND_MASK as i32
                    == ENET_PROTOCOL_COMMAND_DISCONNECT as i32
                {
                    enet_peer_queue_acknowledgement(peer, command, sentTime);
                }
            }
            _ => {
                enet_peer_queue_acknowledgement(peer, command, sentTime);
            }
        }
    }
    if !event.is_null() && (*event).type_0 != ENET_EVENT_TYPE_NONE as i32 as u32 {
        return 1_i32;
    }
    0_i32
}
unsafe fn enet_protocol_receive_incoming_commands<S: Socket>(
    host: *mut ENetHost<S>,
    event: *mut ENetEvent<S>,
) -> i32 {
    let mut packets: i32;
    packets = 0_i32;
    while packets < 256_i32 {
        let mut buffer: ENetBuffer = ENetBuffer {
            data: std::ptr::null_mut(),
            dataLength: 0,
        };
        buffer.data = ((*host).packetData[0_i32 as usize]).as_mut_ptr() as *mut c_void;
        const MTU: usize = 4096;
        buffer.dataLength = ::core::mem::size_of::<[u8; MTU]>();
        let receivedLength = match (*host)
            .socket
            .assume_init_mut()
            .receive(buffer.dataLength as usize)
        {
            Ok(Some((received_address, PacketReceived::Complete(received_data)))) => {
                if received_data.len() <= MTU {
                    *(*host).receivedAddress.assume_init_mut() = Some(received_address);
                    _enet_memcpy(
                        buffer.data,
                        received_data.as_ptr() as *const c_void,
                        received_data.len(),
                    );
                    received_data.len() as i32
                } else {
                    -2
                }
            }
            Ok(Some((_, PacketReceived::Partial))) => -2,
            Ok(None) => 0,
            Err(_) => -1,
        };
        if receivedLength == -2_i32 {
            continue;
        }
        if receivedLength < 0_i32 {
            return -1_i32;
        }
        if receivedLength == 0_i32 {
            return 0_i32;
        }
        (*host).receivedData = ((*host).packetData[0_i32 as usize]).as_mut_ptr();
        (*host).receivedDataLength = receivedLength as usize;
        (*host).totalReceivedData = (*host)
            .totalReceivedData
            .wrapping_add(receivedLength as u32) as u32 as u32;
        (*host).totalReceivedPackets = ((*host).totalReceivedPackets).wrapping_add(1);
        match enet_protocol_handle_incoming_commands(host, event) {
            1 => return 1_i32,
            -1 => return -1_i32,
            _ => {}
        }
        packets += 1;
    }
    0_i32
}
unsafe fn enet_protocol_send_acknowledgements<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
) {
    let mut command: *mut ENetProtocol = ((*host).commands).as_mut_ptr().add((*host).commandCount);
    let mut buffer: *mut ENetBuffer = ((*host).buffers).as_mut_ptr().add((*host).bufferCount);
    let mut acknowledgement: *mut ENetAcknowledgement;
    let mut currentAcknowledgement: ENetListIterator;
    let mut reliableSequenceNumber: u16;
    currentAcknowledgement = (*peer).acknowledgements.sentinel.next;
    while currentAcknowledgement != &mut (*peer).acknowledgements.sentinel as *mut ENetListNode {
        if command
            >= ((*host).commands).as_mut_ptr().offset(
                (::core::mem::size_of::<[ENetProtocol; 32]>() as u64)
                    .wrapping_div(::core::mem::size_of::<ENetProtocol>() as u64)
                    as isize,
            )
            || buffer
                >= ((*host).buffers).as_mut_ptr().offset(
                    (::core::mem::size_of::<[ENetBuffer; 65]>() as u64)
                        .wrapping_div(::core::mem::size_of::<ENetBuffer>() as u64)
                        as isize,
                )
            || ((*peer).mtu as usize).wrapping_sub((*host).packetSize)
                < ::core::mem::size_of::<ENetProtocolAcknowledge>()
        {
            (*peer).flags = ((*peer).flags as i32 | ENET_PEER_FLAG_CONTINUE_SENDING as i32) as u16;
            break;
        } else {
            acknowledgement = currentAcknowledgement as *mut ENetAcknowledgement;
            currentAcknowledgement = (*currentAcknowledgement).next;
            (*buffer).data = command as *mut c_void;
            (*buffer).dataLength = ::core::mem::size_of::<ENetProtocolAcknowledge>();
            (*host).packetSize = (*host).packetSize.wrapping_add((*buffer).dataLength);
            reliableSequenceNumber = (*acknowledgement)
                .command
                .header
                .reliableSequenceNumber
                .to_be();
            (*command).header.command = ENET_PROTOCOL_COMMAND_ACKNOWLEDGE as i32 as u8;
            (*command).header.channelID = (*acknowledgement).command.header.channelID;
            (*command).header.reliableSequenceNumber = reliableSequenceNumber;
            (*command).acknowledge.receivedReliableSequenceNumber = reliableSequenceNumber;
            (*command).acknowledge.receivedSentTime = ((*acknowledgement).sentTime as u16).to_be();
            if (*acknowledgement).command.header.command as i32 & ENET_PROTOCOL_COMMAND_MASK as i32
                == ENET_PROTOCOL_COMMAND_DISCONNECT as i32
            {
                enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
            }
            enet_list_remove(&mut (*acknowledgement).acknowledgementList);
            enet_free(acknowledgement as *mut c_void);
            command = command.offset(1);
            buffer = buffer.offset(1);
        }
    }
    (*host).commandCount = command.offset_from(((*host).commands).as_mut_ptr()) as i64 as usize;
    (*host).bufferCount = buffer.offset_from(((*host).buffers).as_mut_ptr()) as i64 as usize;
}
unsafe fn enet_protocol_check_timeouts<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    event: *mut ENetEvent<S>,
) -> i32 {
    let mut outgoingCommand: *mut ENetOutgoingCommand;
    let mut currentCommand: ENetListIterator;
    currentCommand = (*peer).sentReliableCommands.sentinel.next;
    let insertPosition = (*peer).outgoingCommands.sentinel.next;
    let insertSendReliablePosition = (*peer).outgoingSendReliableCommands.sentinel.next;
    while currentCommand != &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode {
        outgoingCommand = currentCommand as *mut ENetOutgoingCommand;
        currentCommand = (*currentCommand).next;
        if (if ((*host).serviceTime).wrapping_sub((*outgoingCommand).sentTime)
            >= 86400000_i32 as u32
        {
            ((*outgoingCommand).sentTime).wrapping_sub((*host).serviceTime)
        } else {
            ((*host).serviceTime).wrapping_sub((*outgoingCommand).sentTime)
        }) < (*outgoingCommand).roundTripTimeout
        {
            continue;
        }
        if (*peer).earliestTimeout == 0_i32 as u32
            || ((*outgoingCommand).sentTime).wrapping_sub((*peer).earliestTimeout)
                >= 86400000_i32 as u32
        {
            (*peer).earliestTimeout = (*outgoingCommand).sentTime;
        }
        if (*peer).earliestTimeout != 0_i32 as u32
            && ((if ((*host).serviceTime).wrapping_sub((*peer).earliestTimeout)
                >= 86400000_i32 as u32
            {
                ((*peer).earliestTimeout).wrapping_sub((*host).serviceTime)
            } else {
                ((*host).serviceTime).wrapping_sub((*peer).earliestTimeout)
            }) >= (*peer).timeoutMaximum
                || (1_i32 << ((*outgoingCommand).sendAttempts as i32 - 1_i32)) as u32
                    >= (*peer).timeoutLimit
                    && (if ((*host).serviceTime).wrapping_sub((*peer).earliestTimeout)
                        >= 86400000_i32 as u32
                    {
                        ((*peer).earliestTimeout).wrapping_sub((*host).serviceTime)
                    } else {
                        ((*host).serviceTime).wrapping_sub((*peer).earliestTimeout)
                    }) >= (*peer).timeoutMinimum)
        {
            enet_protocol_notify_disconnect(host, peer, event);
            return 1_i32;
        }
        (*peer).packetsLost = ((*peer).packetsLost).wrapping_add(1);
        (*outgoingCommand).roundTripTimeout = (*outgoingCommand)
            .roundTripTimeout
            .wrapping_mul(2_i32 as u32);
        if !((*outgoingCommand).packet).is_null() {
            (*peer).reliableDataInTransit = (*peer)
                .reliableDataInTransit
                .wrapping_sub((*outgoingCommand).fragmentLength as u32);
            enet_list_insert(
                insertSendReliablePosition,
                enet_list_remove(&mut (*outgoingCommand).outgoingCommandList),
            );
        } else {
            enet_list_insert(
                insertPosition,
                enet_list_remove(&mut (*outgoingCommand).outgoingCommandList),
            );
        }
        if currentCommand == (*peer).sentReliableCommands.sentinel.next
            && ((*peer).sentReliableCommands.sentinel.next
                != &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode)
        {
            outgoingCommand = currentCommand as *mut ENetOutgoingCommand;
            (*peer).nextTimeout =
                ((*outgoingCommand).sentTime).wrapping_add((*outgoingCommand).roundTripTimeout);
        }
    }
    0_i32
}
unsafe fn enet_protocol_check_outgoing_commands<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    sentUnreliableCommands: *mut ENetList,
) -> i32 {
    let mut command: *mut ENetProtocol = ((*host).commands).as_mut_ptr().add((*host).commandCount);
    let mut buffer: *mut ENetBuffer = ((*host).buffers).as_mut_ptr().add((*host).bufferCount);
    let mut outgoingCommand: *mut ENetOutgoingCommand = std::ptr::null_mut();
    let mut currentCommand: ENetListIterator;
    let mut currentSendReliableCommand: ENetListIterator;
    let mut channel: *mut ENetChannel = std::ptr::null_mut();
    let mut reliableWindow: u16 = 0_i32 as u16;
    let mut commandSize: usize;
    let mut windowWrap: i32 = 0_i32;
    let mut canPing: i32 = 1_i32;
    currentCommand = (*peer).outgoingCommands.sentinel.next;
    currentSendReliableCommand = (*peer).outgoingSendReliableCommands.sentinel.next;
    let mut current_block_55: u64;
    loop {
        if currentCommand != &mut (*peer).outgoingCommands.sentinel as *mut ENetListNode {
            outgoingCommand = currentCommand as *mut ENetOutgoingCommand;
            if currentSendReliableCommand
                != &mut (*peer).outgoingSendReliableCommands.sentinel as *mut ENetListNode
                && ((*(currentSendReliableCommand as *mut ENetOutgoingCommand)).queueTime)
                    .wrapping_sub((*outgoingCommand).queueTime)
                    >= 86400000_i32 as u32
            {
                current_block_55 = 13678975718891345113;
            } else {
                currentCommand = (*currentCommand).next;
                current_block_55 = 1856101646708284338;
            }
        } else {
            if currentSendReliableCommand
                == &mut (*peer).outgoingSendReliableCommands.sentinel as *mut ENetListNode
            {
                break;
            }
            current_block_55 = 13678975718891345113;
        }
        if let 13678975718891345113 = current_block_55 {
            outgoingCommand = currentSendReliableCommand as *mut ENetOutgoingCommand;
            currentSendReliableCommand = (*currentSendReliableCommand).next;
        }
        if (*outgoingCommand).command.header.command as i32
            & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32
            != 0
        {
            channel =
                if ((*outgoingCommand).command.header.channelID as usize) < (*peer).channelCount {
                    ((*peer).channels).offset((*outgoingCommand).command.header.channelID as isize)
                } else {
                    std::ptr::null_mut()
                };
            reliableWindow = ((*outgoingCommand).reliableSequenceNumber as i32
                / ENET_PEER_RELIABLE_WINDOW_SIZE as i32) as u16;
            if !channel.is_null() {
                if windowWrap != 0 {
                    continue;
                }
                if ((*outgoingCommand).sendAttempts as i32) < 1_i32
                    && (*outgoingCommand).reliableSequenceNumber as i32
                        % ENET_PEER_RELIABLE_WINDOW_SIZE as i32
                        == 0
                    && ((*channel).reliableWindows[((reliableWindow as i32
                        + ENET_PEER_RELIABLE_WINDOWS as i32
                        - 1_i32)
                        % ENET_PEER_RELIABLE_WINDOWS as i32)
                        as usize] as i32
                        >= ENET_PEER_RELIABLE_WINDOW_SIZE as i32
                        || (*channel).usedReliableWindows as i32
                            & (((1_i32 << (ENET_PEER_FREE_RELIABLE_WINDOWS as i32 + 2_i32))
                                - 1_i32)
                                << reliableWindow as i32
                                | ((1_i32 << (ENET_PEER_FREE_RELIABLE_WINDOWS as i32 + 2_i32))
                                    - 1_i32)
                                    >> (ENET_PEER_RELIABLE_WINDOWS as i32 - reliableWindow as i32))
                            != 0)
                {
                    windowWrap = 1_i32;
                    currentSendReliableCommand = &mut (*peer).outgoingSendReliableCommands.sentinel;
                    continue;
                }
            }
            if !((*outgoingCommand).packet).is_null() {
                let windowSize: u32 = ((*peer).packetThrottle)
                    .wrapping_mul((*peer).windowSize)
                    .wrapping_div(ENET_PEER_PACKET_THROTTLE_SCALE as i32 as u32);
                if ((*peer).reliableDataInTransit)
                    .wrapping_add((*outgoingCommand).fragmentLength as u32)
                    > (if windowSize > (*peer).mtu {
                        windowSize
                    } else {
                        (*peer).mtu
                    })
                {
                    currentSendReliableCommand = &mut (*peer).outgoingSendReliableCommands.sentinel;
                    continue;
                }
            }
            canPing = 0_i32;
        }
        commandSize = COMMAND_SIZES[((*outgoingCommand).command.header.command as i32
            & ENET_PROTOCOL_COMMAND_MASK as i32) as usize];
        if command
            >= ((*host).commands).as_mut_ptr().offset(
                (::core::mem::size_of::<[ENetProtocol; 32]>() as u64)
                    .wrapping_div(::core::mem::size_of::<ENetProtocol>() as u64)
                    as isize,
            )
            || buffer.offset(1_i32 as isize)
                >= ((*host).buffers).as_mut_ptr().offset(
                    (::core::mem::size_of::<[ENetBuffer; 65]>() as u64)
                        .wrapping_div(::core::mem::size_of::<ENetBuffer>() as u64)
                        as isize,
                )
            || ((*peer).mtu as usize).wrapping_sub((*host).packetSize) < commandSize
            || !((*outgoingCommand).packet).is_null()
                && (((*peer).mtu as usize).wrapping_sub((*host).packetSize) as u16 as i32)
                    < commandSize.wrapping_add((*outgoingCommand).fragmentLength as usize) as u16
                        as i32
        {
            (*peer).flags = ((*peer).flags as i32 | ENET_PEER_FLAG_CONTINUE_SENDING as i32) as u16;
            break;
        } else {
            if (*outgoingCommand).command.header.command as i32
                & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32
                != 0
            {
                if !channel.is_null() && ((*outgoingCommand).sendAttempts as i32) < 1_i32 {
                    (*channel).usedReliableWindows = ((*channel).usedReliableWindows as i32
                        | 1_i32 << reliableWindow as i32)
                        as u16;
                    (*channel).reliableWindows[reliableWindow as usize] =
                        ((*channel).reliableWindows[reliableWindow as usize]).wrapping_add(1);
                }
                (*outgoingCommand).sendAttempts = ((*outgoingCommand).sendAttempts).wrapping_add(1);
                if (*outgoingCommand).roundTripTimeout == 0_i32 as u32 {
                    (*outgoingCommand).roundTripTimeout = ((*peer).roundTripTime)
                        .wrapping_add((4_i32 as u32).wrapping_mul((*peer).roundTripTimeVariance));
                }
                if (*peer).sentReliableCommands.sentinel.next
                    == &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode
                {
                    (*peer).nextTimeout =
                        ((*host).serviceTime).wrapping_add((*outgoingCommand).roundTripTimeout);
                }
                enet_list_insert(
                    &mut (*peer).sentReliableCommands.sentinel,
                    enet_list_remove(&mut (*outgoingCommand).outgoingCommandList),
                );
                (*outgoingCommand).sentTime = (*host).serviceTime;
                (*host).headerFlags = ((*host).headerFlags as i32
                    | ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as i32)
                    as u16;
                (*peer).reliableDataInTransit = (*peer)
                    .reliableDataInTransit
                    .wrapping_add((*outgoingCommand).fragmentLength as u32)
                    as u32 as u32;
            } else {
                if !((*outgoingCommand).packet).is_null()
                    && (*outgoingCommand).fragmentOffset == 0_i32 as u32
                {
                    (*peer).packetThrottleCounter = (*peer)
                        .packetThrottleCounter
                        .wrapping_add(ENET_PEER_PACKET_THROTTLE_COUNTER as i32 as u32);
                    (*peer).packetThrottleCounter = (*peer)
                        .packetThrottleCounter
                        .wrapping_rem(ENET_PEER_PACKET_THROTTLE_SCALE as i32 as u32);
                    if (*peer).packetThrottleCounter > (*peer).packetThrottle {
                        let reliableSequenceNumber: u16 = (*outgoingCommand).reliableSequenceNumber;
                        let unreliableSequenceNumber: u16 =
                            (*outgoingCommand).unreliableSequenceNumber;
                        loop {
                            (*(*outgoingCommand).packet).referenceCount =
                                ((*(*outgoingCommand).packet).referenceCount).wrapping_sub(1);
                            if (*(*outgoingCommand).packet).referenceCount == 0_i32 as usize {
                                enet_packet_destroy((*outgoingCommand).packet);
                            }
                            enet_list_remove(&mut (*outgoingCommand).outgoingCommandList);
                            enet_free(outgoingCommand as *mut c_void);
                            if currentCommand
                                == &mut (*peer).outgoingCommands.sentinel as *mut ENetListNode
                            {
                                break;
                            }
                            outgoingCommand = currentCommand as *mut ENetOutgoingCommand;
                            if (*outgoingCommand).reliableSequenceNumber as i32
                                != reliableSequenceNumber as i32
                                || (*outgoingCommand).unreliableSequenceNumber as i32
                                    != unreliableSequenceNumber as i32
                            {
                                break;
                            }
                            currentCommand = (*currentCommand).next;
                        }
                        continue;
                    }
                }
                enet_list_remove(&mut (*outgoingCommand).outgoingCommandList);
                if !((*outgoingCommand).packet).is_null() {
                    enet_list_insert(
                        &mut (*sentUnreliableCommands).sentinel,
                        outgoingCommand as *mut c_void,
                    );
                }
            }
            (*buffer).data = command as *mut c_void;
            (*buffer).dataLength = commandSize;
            (*host).packetSize = ((*host).packetSize).wrapping_add((*buffer).dataLength);
            *command = (*outgoingCommand).command;
            if !((*outgoingCommand).packet).is_null() {
                buffer = buffer.offset(1);
                (*buffer).data = ((*(*outgoingCommand).packet).data)
                    .offset((*outgoingCommand).fragmentOffset as isize)
                    as *mut c_void;
                (*buffer).dataLength = (*outgoingCommand).fragmentLength as usize;
                (*host).packetSize = ((*host).packetSize as u64)
                    .wrapping_add((*outgoingCommand).fragmentLength as u64)
                    as usize as usize;
            } else if (*outgoingCommand).command.header.command as i32
                & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32
                == 0
            {
                enet_free(outgoingCommand as *mut c_void);
            }
            (*peer).packetsSent = ((*peer).packetsSent).wrapping_add(1);
            command = command.offset(1);
            buffer = buffer.offset(1);
        }
    }
    (*host).commandCount = command.offset_from(((*host).commands).as_mut_ptr()) as i64 as usize;
    (*host).bufferCount = buffer.offset_from(((*host).buffers).as_mut_ptr()) as i64 as usize;
    if (*peer).state == ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
        && enet_peer_has_outgoing_commands(peer) == 0
        && (*sentUnreliableCommands).sentinel.next
            == &mut (*sentUnreliableCommands).sentinel as *mut ENetListNode
    {
        enet_peer_disconnect(peer, (*peer).eventData);
    }
    canPing
}
unsafe fn enet_protocol_send_outgoing_commands<S: Socket>(
    host: *mut ENetHost<S>,
    event: *mut ENetEvent<S>,
    checkForTimeouts: i32,
) -> i32 {
    let mut headerData: [u8; 8] = [0; 8];
    let header: *mut ENetProtocolHeader = headerData.as_mut_ptr() as *mut ENetProtocolHeader;
    let mut sentLength: i32;
    let mut shouldCompress: usize;
    let mut sentUnreliableCommands: ENetList = ENetList {
        sentinel: ENetListNode {
            next: std::ptr::null_mut(),
            previous: std::ptr::null_mut(),
        },
    };
    enet_list_clear(&mut sentUnreliableCommands);
    let mut sendPass: i32 = 0_i32;
    let mut continueSending: i32 = 0_i32;
    while sendPass <= continueSending {
        let mut currentPeer: *mut ENetPeer<S> = (*host).peers;
        while currentPeer < ((*host).peers).add((*host).peerCount) {
            if !((*currentPeer).state == ENET_PEER_STATE_DISCONNECTED as i32 as u32
                || (*currentPeer).state == ENET_PEER_STATE_ZOMBIE as i32 as u32
                || sendPass > 0_i32
                    && (*currentPeer).flags as i32 & ENET_PEER_FLAG_CONTINUE_SENDING as i32 == 0)
            {
                (*currentPeer).flags = ((*currentPeer).flags as i32
                    & !(ENET_PEER_FLAG_CONTINUE_SENDING as i32))
                    as u16;
                (*host).headerFlags = 0_i32 as u16;
                (*host).commandCount = 0_i32 as usize;
                (*host).bufferCount = 1_i32 as usize;
                (*host).packetSize = ::core::mem::size_of::<ENetProtocolHeader>();
                if (*currentPeer).acknowledgements.sentinel.next
                    != &mut (*currentPeer).acknowledgements.sentinel as *mut ENetListNode
                {
                    enet_protocol_send_acknowledgements(host, currentPeer);
                }
                if checkForTimeouts != 0_i32
                    && ((*currentPeer).sentReliableCommands.sentinel.next
                        != &mut (*currentPeer).sentReliableCommands.sentinel as *mut ENetListNode)
                    && (((*host).serviceTime).wrapping_sub((*currentPeer).nextTimeout)
                        < 86400000_i32 as u32)
                    && enet_protocol_check_timeouts(host, currentPeer, event) == 1_i32
                {
                    if !event.is_null() && (*event).type_0 != ENET_EVENT_TYPE_NONE as i32 as u32 {
                        return 1_i32;
                    }
                } else {
                    if ((*currentPeer).outgoingCommands.sentinel.next
                        == &mut (*currentPeer).outgoingCommands.sentinel as *mut ENetListNode
                        && (*currentPeer).outgoingSendReliableCommands.sentinel.next
                            == &mut (*currentPeer).outgoingSendReliableCommands.sentinel
                                as *mut ENetListNode
                        || enet_protocol_check_outgoing_commands(
                            host,
                            currentPeer,
                            &mut sentUnreliableCommands,
                        ) != 0)
                        && (*currentPeer).sentReliableCommands.sentinel.next
                            == &mut (*currentPeer).sentReliableCommands.sentinel
                                as *mut ENetListNode
                        && (if ((*host).serviceTime).wrapping_sub((*currentPeer).lastReceiveTime)
                            >= 86400000_i32 as u32
                        {
                            ((*currentPeer).lastReceiveTime).wrapping_sub((*host).serviceTime)
                        } else {
                            ((*host).serviceTime).wrapping_sub((*currentPeer).lastReceiveTime)
                        }) >= (*currentPeer).pingInterval
                        && ((*currentPeer).mtu as usize).wrapping_sub((*host).packetSize)
                            >= ::core::mem::size_of::<ENetProtocolPing>()
                    {
                        enet_peer_ping(currentPeer);
                        enet_protocol_check_outgoing_commands(
                            host,
                            currentPeer,
                            &mut sentUnreliableCommands,
                        );
                    }
                    if (*host).commandCount != 0_i32 as usize {
                        if (*currentPeer).packetLossEpoch == 0_i32 as u32 {
                            (*currentPeer).packetLossEpoch = (*host).serviceTime;
                        } else if (if ((*host).serviceTime)
                            .wrapping_sub((*currentPeer).packetLossEpoch)
                            >= 86400000_i32 as u32
                        {
                            ((*currentPeer).packetLossEpoch).wrapping_sub((*host).serviceTime)
                        } else {
                            ((*host).serviceTime).wrapping_sub((*currentPeer).packetLossEpoch)
                        }) >= ENET_PEER_PACKET_LOSS_INTERVAL as i32 as u32
                            && (*currentPeer).packetsSent > 0_i32 as u32
                        {
                            let packetLoss: u32 = ((*currentPeer).packetsLost)
                                .wrapping_mul(ENET_PEER_PACKET_LOSS_SCALE as i32 as u32)
                                .wrapping_div((*currentPeer).packetsSent);
                            (*currentPeer).packetLossVariance = ((*currentPeer).packetLossVariance)
                                .wrapping_mul(3_i32 as u32)
                                .wrapping_add(if packetLoss < (*currentPeer).packetLoss {
                                    ((*currentPeer).packetLoss).wrapping_sub(packetLoss)
                                } else {
                                    packetLoss.wrapping_sub((*currentPeer).packetLoss)
                                })
                                .wrapping_div(4_i32 as u32);
                            (*currentPeer).packetLoss = ((*currentPeer).packetLoss)
                                .wrapping_mul(7_i32 as u32)
                                .wrapping_add(packetLoss)
                                .wrapping_div(8_i32 as u32);
                            (*currentPeer).packetLossEpoch = (*host).serviceTime;
                            (*currentPeer).packetsSent = 0_i32 as u32;
                            (*currentPeer).packetsLost = 0_i32 as u32;
                        }
                        let fresh34 = &mut (*((*host).buffers).as_mut_ptr()).data;
                        *fresh34 = headerData.as_mut_ptr() as *mut c_void;
                        if (*host).headerFlags as i32 & ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as i32
                            != 0
                        {
                            (*header).sentTime =
                                (((*host).serviceTime & 0xffff_i32 as u32) as u16).to_be();
                            (*((*host).buffers).as_mut_ptr()).dataLength =
                                ::core::mem::size_of::<ENetProtocolHeader>();
                        } else {
                            (*((*host).buffers).as_mut_ptr()).dataLength = 2;
                        }
                        shouldCompress = 0_i32 as usize;
                        if let Some(compressor) = (*host).compressor.assume_init_mut() {
                            let originalSize: usize = ((*host).packetSize)
                                .wrapping_sub(::core::mem::size_of::<ENetProtocolHeader>());
                            let mut inBuffers = vec![];
                            for i in 0..((*host).bufferCount).wrapping_sub(1) {
                                let buffer = ((*host).buffers).as_mut_ptr().add(1 + i);
                                inBuffers.push(std::slice::from_raw_parts(
                                    (*buffer).data as *mut u8,
                                    (*buffer).dataLength,
                                ));
                            }
                            let compressedSize: usize = compressor.compress(
                                inBuffers,
                                originalSize,
                                std::slice::from_raw_parts_mut(
                                    ((*host).packetData[1_i32 as usize]).as_mut_ptr(),
                                    originalSize,
                                ),
                            );
                            if compressedSize > 0_i32 as usize && compressedSize < originalSize {
                                (*host).headerFlags = ((*host).headerFlags as i32
                                    | ENET_PROTOCOL_HEADER_FLAG_COMPRESSED as i32)
                                    as u16;
                                shouldCompress = compressedSize;
                            }
                        }
                        if ((*currentPeer).outgoingPeerID as i32)
                            < ENET_PROTOCOL_MAXIMUM_PEER_ID as i32
                        {
                            (*host).headerFlags = ((*host).headerFlags as i32
                                | ((*currentPeer).outgoingSessionID as i32)
                                    << ENET_PROTOCOL_HEADER_SESSION_SHIFT as i32)
                                as u16;
                        }
                        (*header).peerID = (((*currentPeer).outgoingPeerID as i32
                            | (*host).headerFlags as i32)
                            as u16)
                            .to_be();
                        if let Some(checksum_fn) = (*host).checksum.assume_init_ref() {
                            let checksum_addr: *mut u8 = headerData
                                .as_mut_ptr()
                                .add((*((*host).buffers).as_mut_ptr()).dataLength);
                            let mut checksum = if ((*currentPeer).outgoingPeerID as i32)
                                < ENET_PROTOCOL_MAXIMUM_PEER_ID as i32
                            {
                                (*currentPeer).connectID
                            } else {
                                0_i32 as u32
                            };
                            _enet_memcpy(
                                checksum_addr as *mut c_void,
                                &checksum as *const u32 as *const c_void,
                                ::core::mem::size_of::<u32>(),
                            );
                            let fresh35 = &mut (*((*host).buffers).as_mut_ptr()).dataLength;
                            *fresh35 = (*fresh35 as u64)
                                .wrapping_add(::core::mem::size_of::<u32>() as u64)
                                as usize;
                            let mut inBuffers = vec![];
                            for i in 0..(*host).bufferCount {
                                let buffer = ((*host).buffers).as_mut_ptr().add(i);
                                inBuffers.push(std::slice::from_raw_parts(
                                    (*buffer).data as *mut u8,
                                    (*buffer).dataLength,
                                ));
                            }
                            checksum = checksum_fn(inBuffers);
                            _enet_memcpy(
                                checksum_addr as *mut c_void,
                                &checksum as *const u32 as *const c_void,
                                ::core::mem::size_of::<u32>(),
                            );
                        }
                        if shouldCompress > 0_i32 as usize {
                            (*host).buffers[1_i32 as usize].data =
                                ((*host).packetData[1_i32 as usize]).as_mut_ptr() as *mut c_void;
                            (*host).buffers[1_i32 as usize].dataLength = shouldCompress;
                            (*host).bufferCount = 2_i32 as usize;
                        }
                        (*currentPeer).lastSendTime = (*host).serviceTime;
                        let mut conglomerate_buffer = vec![];
                        for buffer_index in 0..(*host).bufferCount {
                            let buffer = &(*host).buffers[buffer_index];
                            conglomerate_buffer.extend_from_slice(std::slice::from_raw_parts(
                                buffer.data as *mut u8,
                                buffer.dataLength,
                            ));
                        }
                        sentLength = match (*host).socket.assume_init_mut().send(
                            (*currentPeer)
                                .address
                                .assume_init_ref()
                                .as_ref()
                                .cloned()
                                .unwrap(),
                            &conglomerate_buffer,
                        ) {
                            Ok(sent) => sent as i32,
                            Err(_) => -1,
                        };
                        enet_protocol_remove_sent_unreliable_commands(
                            currentPeer,
                            &mut sentUnreliableCommands,
                        );
                        if sentLength < 0_i32 {
                            return -1_i32;
                        }
                        (*host).totalSentData =
                            (*host).totalSentData.wrapping_add(sentLength as u32);
                        (*host).totalSentPackets = ((*host).totalSentPackets).wrapping_add(1);
                    }
                }
                if (*currentPeer).flags as i32 & ENET_PEER_FLAG_CONTINUE_SENDING as i32 != 0 {
                    continueSending = sendPass + 1_i32;
                }
            }
            currentPeer = currentPeer.offset(1);
        }
        sendPass += 1;
    }
    0_i32
}
pub(crate) unsafe fn enet_host_flush<S: Socket>(host: *mut ENetHost<S>) {
    (*host).serviceTime = enet_time_get(host);
    enet_protocol_send_outgoing_commands(host, std::ptr::null_mut(), 0_i32);
}
pub(crate) unsafe fn enet_host_check_events<S: Socket>(
    host: *mut ENetHost<S>,
    event: *mut ENetEvent<S>,
) -> i32 {
    if event.is_null() {
        return -1_i32;
    }
    (*event).type_0 = ENET_EVENT_TYPE_NONE;
    (*event).peer = std::ptr::null_mut();
    (*event).packet = std::ptr::null_mut();
    enet_protocol_dispatch_incoming_commands(host, event)
}
pub(crate) unsafe fn enet_host_service<S: Socket>(
    host: *mut ENetHost<S>,
    event: *mut ENetEvent<S>,
) -> i32 {
    if !event.is_null() {
        (*event).type_0 = ENET_EVENT_TYPE_NONE;
        (*event).peer = std::ptr::null_mut();
        (*event).packet = std::ptr::null_mut();
        match enet_protocol_dispatch_incoming_commands(host, event) {
            1 => return 1_i32,
            -1 => return -1_i32,
            _ => {}
        }
    }
    (*host).serviceTime = enet_time_get(host);
    if (if ((*host).serviceTime).wrapping_sub((*host).bandwidthThrottleEpoch) >= 86400000_i32 as u32
    {
        ((*host).bandwidthThrottleEpoch).wrapping_sub((*host).serviceTime)
    } else {
        ((*host).serviceTime).wrapping_sub((*host).bandwidthThrottleEpoch)
    }) >= ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL as i32 as u32
    {
        enet_host_bandwidth_throttle(host);
    }
    match enet_protocol_send_outgoing_commands(host, event, 1_i32) {
        1 => return 1_i32,
        -1 => return -1_i32,
        _ => {}
    }
    match enet_protocol_receive_incoming_commands(host, event) {
        1 => return 1_i32,
        -1 => return -1_i32,
        _ => {}
    }
    match enet_protocol_send_outgoing_commands(host, event, 1_i32) {
        1 => return 1_i32,
        -1 => return -1_i32,
        _ => {}
    }
    if !event.is_null() {
        match enet_protocol_dispatch_incoming_commands(host, event) {
            1 => return 1_i32,
            -1 => return -1_i32,
            _ => {}
        }
    }
    0_i32
}
