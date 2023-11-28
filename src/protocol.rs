use std::{
    mem::{size_of, zeroed},
    ptr::copy_nonoverlapping,
};

use crate::{
    c_void, enet_free, enet_list_begin, enet_list_clear, enet_list_empty, enet_list_end,
    enet_list_front, enet_list_insert, enet_list_next, enet_list_previous, enet_list_remove,
    enet_memset, enet_packet_destroy, enet_peer_disconnect,
    enet_peer_dispatch_incoming_reliable_commands, enet_peer_dispatch_incoming_unreliable_commands,
    enet_peer_has_outgoing_commands, enet_peer_on_connect, enet_peer_on_disconnect, enet_peer_ping,
    enet_peer_queue_acknowledgement, enet_peer_queue_incoming_command,
    enet_peer_queue_outgoing_command, enet_peer_receive, enet_peer_reset, enet_peer_reset_queues,
    enet_peer_throttle, enet_time_difference, enet_time_get, enet_time_greater_equal,
    enet_time_less, Address, Channel, ENetBuffer, ENetList, ENetListIterator, ENetListNode,
    ENetPacket, Event, Host, Packet, PacketFlag, Peer, PeerState, Socket,
    ENET_PEER_FLAG_CONTINUE_SENDING, ENET_PEER_FLAG_NEEDS_DISPATCH,
    ENET_PEER_FREE_RELIABLE_WINDOWS, ENET_PEER_FREE_UNSEQUENCED_WINDOWS,
    ENET_PEER_PACKET_LOSS_INTERVAL, ENET_PEER_PACKET_LOSS_SCALE, ENET_PEER_PACKET_THROTTLE_COUNTER,
    ENET_PEER_PACKET_THROTTLE_SCALE, ENET_PEER_RELIABLE_WINDOWS, ENET_PEER_RELIABLE_WINDOW_SIZE,
    ENET_PEER_UNSEQUENCED_WINDOW_SIZE, ENET_PEER_WINDOW_SIZE_SCALE,
    ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT, ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT,
    ENET_PROTOCOL_MAXIMUM_MTU, ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS,
    ENET_PROTOCOL_MAXIMUM_PEER_ID, ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE,
    ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT, ENET_PROTOCOL_MINIMUM_MTU,
    ENET_PROTOCOL_MINIMUM_WINDOW_SIZE, SENT_TIME_OFFSET,
};

pub(crate) type ENetProtocolCommand = u8;
pub(crate) const ENET_PROTOCOL_COMMAND_MASK: ENetProtocolCommand = 15;
pub(crate) const ENET_PROTOCOL_COMMAND_COUNT: ENetProtocolCommand = 13;
pub(crate) const ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT: ENetProtocolCommand = 12;
pub(crate) const ENET_PROTOCOL_COMMAND_THROTTLE_CONFIGURE: ENetProtocolCommand = 11;
pub(crate) const ENET_PROTOCOL_COMMAND_BANDWIDTH_LIMIT: ENetProtocolCommand = 10;
pub(crate) const ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED: ENetProtocolCommand = 9;
pub(crate) const ENET_PROTOCOL_COMMAND_SEND_FRAGMENT: ENetProtocolCommand = 8;
pub(crate) const ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE: ENetProtocolCommand = 7;
pub(crate) const ENET_PROTOCOL_COMMAND_SEND_RELIABLE: ENetProtocolCommand = 6;
pub(crate) const ENET_PROTOCOL_COMMAND_PING: ENetProtocolCommand = 5;
pub(crate) const ENET_PROTOCOL_COMMAND_DISCONNECT: ENetProtocolCommand = 4;
pub(crate) const ENET_PROTOCOL_COMMAND_VERIFY_CONNECT: ENetProtocolCommand = 3;
pub(crate) const ENET_PROTOCOL_COMMAND_CONNECT: ENetProtocolCommand = 2;
pub(crate) const ENET_PROTOCOL_COMMAND_ACKNOWLEDGE: ENetProtocolCommand = 1;
pub(crate) const ENET_PROTOCOL_COMMAND_NONE: ENetProtocolCommand = 0;
pub(crate) const ENET_PROTOCOL_HEADER_SESSION_SHIFT: u16 = 12;
pub(crate) const ENET_PROTOCOL_HEADER_SESSION_MASK: u16 = 12288;
pub(crate) const ENET_PROTOCOL_HEADER_FLAG_MASK: u16 = 49152;
pub(crate) const ENET_PROTOCOL_HEADER_FLAG_SENT_TIME: u16 = 32768;
pub(crate) const ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED: u8 = 64;
pub(crate) const ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE: u8 = 128;
static mut COMMAND_SIZES: [usize; 13] = [
    0,
    size_of::<ENetProtocolAcknowledge>(),
    size_of::<ENetProtocolConnect>(),
    size_of::<ENetProtocolVerifyConnect>(),
    size_of::<ENetProtocolDisconnect>(),
    size_of::<ENetProtocolPing>(),
    size_of::<ENetProtocolSendReliable>(),
    size_of::<ENetProtocolSendUnreliable>(),
    size_of::<ENetProtocolSendFragment>(),
    size_of::<ENetProtocolSendUnsequenced>(),
    size_of::<ENetProtocolBandwidthLimit>(),
    size_of::<ENetProtocolThrottleConfigure>(),
    size_of::<ENetProtocolSendFragment>(),
];
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetOutgoingCommand {
    pub outgoing_command_list: ENetListNode<ENetOutgoingCommand>,
    pub reliable_sequence_number: u16,
    pub unreliable_sequence_number: u16,
    pub sent_time: u32,
    pub round_trip_timeout: u32,
    pub queue_time: u32,
    pub fragment_offset: u32,
    pub fragment_length: u16,
    pub send_attempts: u16,
    pub command: ENetProtocol,
    pub packet: *mut ENetPacket,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetIncomingCommand {
    pub incoming_command_list: ENetListNode<ENetIncomingCommand>,
    pub reliable_sequence_number: u16,
    pub unreliable_sequence_number: u16,
    pub command: ENetProtocol,
    pub fragment_count: u32,
    pub fragments_remaining: u32,
    pub fragments: *mut u32,
    pub packet: *mut ENetPacket,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolHeader {
    pub peer_id: u16,
    pub sent_time: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolCommandHeader {
    pub command: u8,
    pub channel_id: u8,
    pub reliable_sequence_number: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolAcknowledge {
    pub header: ENetProtocolCommandHeader,
    pub received_reliable_sequence_number: u16,
    pub received_sent_time: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolConnect {
    pub header: ENetProtocolCommandHeader,
    pub outgoing_peer_id: u16,
    pub incoming_session_id: u8,
    pub outgoing_session_id: u8,
    pub mtu: u32,
    pub window_size: u32,
    pub channel_count: u32,
    pub incoming_bandwidth: u32,
    pub outgoing_bandwidth: u32,
    pub packet_throttle_interval: u32,
    pub packet_throttle_acceleration: u32,
    pub packet_throttle_deceleration: u32,
    pub connect_id: u32,
    pub data: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolVerifyConnect {
    pub header: ENetProtocolCommandHeader,
    pub outgoing_peer_id: u16,
    pub incoming_session_id: u8,
    pub outgoing_session_id: u8,
    pub mtu: u32,
    pub window_size: u32,
    pub channel_count: u32,
    pub incoming_bandwidth: u32,
    pub outgoing_bandwidth: u32,
    pub packet_throttle_interval: u32,
    pub packet_throttle_acceleration: u32,
    pub packet_throttle_deceleration: u32,
    pub connect_id: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolBandwidthLimit {
    pub header: ENetProtocolCommandHeader,
    pub incoming_bandwidth: u32,
    pub outgoing_bandwidth: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolThrottleConfigure {
    pub header: ENetProtocolCommandHeader,
    pub packet_throttle_interval: u32,
    pub packet_throttle_acceleration: u32,
    pub packet_throttle_deceleration: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolDisconnect {
    pub header: ENetProtocolCommandHeader,
    pub data: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolPing {
    pub header: ENetProtocolCommandHeader,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolSendReliable {
    pub header: ENetProtocolCommandHeader,
    pub data_length: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolSendUnreliable {
    pub header: ENetProtocolCommandHeader,
    pub unreliable_sequence_number: u16,
    pub data_length: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolSendUnsequenced {
    pub header: ENetProtocolCommandHeader,
    pub unsequenced_group: u16,
    pub data_length: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolSendFragment {
    pub header: ENetProtocolCommandHeader,
    pub start_sequence_number: u16,
    pub data_length: u16,
    pub fragment_count: u32,
    pub fragment_number: u32,
    pub total_length: u32,
    pub fragment_offset: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) union ENetProtocol {
    pub header: ENetProtocolCommandHeader,
    pub acknowledge: ENetProtocolAcknowledge,
    pub connect: ENetProtocolConnect,
    pub verify_connect: ENetProtocolVerifyConnect,
    pub disconnect: ENetProtocolDisconnect,
    pub ping: ENetProtocolPing,
    pub send_reliable: ENetProtocolSendReliable,
    pub send_unreliable: ENetProtocolSendUnreliable,
    pub send_unsequenced: ENetProtocolSendUnsequenced,
    pub send_fragment: ENetProtocolSendFragment,
    pub bandwidth_limit: ENetProtocolBandwidthLimit,
    pub throttle_configure: ENetProtocolThrottleConfigure,
}

impl Default for ENetProtocol {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

pub(crate) unsafe fn enet_protocol_command_size(command_number: u8) -> usize {
    COMMAND_SIZES[(command_number & ENET_PROTOCOL_COMMAND_MASK) as usize]
}
fn enet_protocol_change_state<S: Socket>(host: &mut Host<S>, peer: &mut Peer<S>, state: PeerState) {
    if state == PeerState::Connected || state == PeerState::DisconnectLater {
        enet_peer_on_connect(host, peer);
    } else {
        enet_peer_on_disconnect(host, peer);
    }
    peer.state = state;
}
fn enet_protocol_dispatch_state<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    state: PeerState,
) {
    enet_protocol_change_state(host, peer, state);
    if peer.flags & ENET_PEER_FLAG_NEEDS_DISPATCH == 0 {
        host.dispatch_queue.push_back(peer.index);
        peer.flags |= ENET_PEER_FLAG_NEEDS_DISPATCH;
    }
}
pub(crate) unsafe fn enet_protocol_dispatch_incoming_commands<S: Socket>(
    host: &mut Host<S>,
    event: &mut Event,
) -> Result<bool, crate::Error> {
    while let Some(peer_id) = host.dispatch_queue.pop_front() {
        let peer = &mut host.peers[peer_id.0] as *mut Peer<S>;
        (*peer).flags &= !(ENET_PEER_FLAG_NEEDS_DISPATCH);
        match (*peer).state {
            PeerState::ConnectionPending | PeerState::ConnectionSucceeded => {
                enet_protocol_change_state(host, &mut *peer, PeerState::Connected);
                (*event) = Event::Connect {
                    peer: (*peer).index,
                    data: (*peer).event_data,
                };
                return Ok(true);
            }
            PeerState::Zombie => {
                host.recalculate_bandwidth_limits = true;
                (*event) = Event::Disconnect {
                    peer: (*peer).index,
                    data: (*peer).event_data,
                };
                enet_peer_reset(host, &mut *peer);
                return Ok(true);
            }
            PeerState::Connected => {
                if enet_list_empty(&mut (*peer).dispatched_commands) {
                    continue;
                }
                let mut channel_id: u8 = 0;
                let packet = enet_peer_receive(&mut *peer, &mut channel_id);
                if packet.is_null() {
                    continue;
                }
                (*event) = Event::Receive {
                    peer: (*peer).index,
                    channel_id,
                    packet: Packet::new_internal(packet),
                };
                if !enet_list_empty(&mut (*peer).dispatched_commands) {
                    (*peer).flags |= ENET_PEER_FLAG_NEEDS_DISPATCH;
                    host.dispatch_queue.push_back((*peer).index);
                }
                return Ok(true);
            }
            _ => {}
        }
    }
    Ok(false)
}
unsafe fn enet_protocol_notify_connect<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    event: *mut Event,
) {
    host.recalculate_bandwidth_limits = true;
    if !event.is_null() {
        enet_protocol_change_state(host, peer, PeerState::Connected);
        (*event) = Event::Connect {
            peer: peer.index,
            data: peer.event_data,
        };
    } else {
        enet_protocol_dispatch_state(
            host,
            peer,
            (if peer.state == PeerState::Connecting {
                PeerState::ConnectionSucceeded
            } else {
                PeerState::ConnectionPending
            }) as PeerState,
        );
    };
}
unsafe fn enet_protocol_notify_disconnect<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    event: *mut Event,
) {
    let greater_than_or_equal_to_connection_pending = !matches!(
        peer.state,
        PeerState::Disconnected | PeerState::Connecting | PeerState::AcknowledgingConnect
    );

    if greater_than_or_equal_to_connection_pending {
        host.recalculate_bandwidth_limits = true;
    }
    if peer.state != PeerState::Connecting && !greater_than_or_equal_to_connection_pending {
        enet_peer_reset(host, peer);
    } else if !event.is_null() {
        (*event) = Event::Disconnect {
            peer: peer.index,
            data: 0,
        };
        enet_peer_reset(host, peer);
    } else {
        peer.event_data = 0;
        enet_protocol_dispatch_state(host, peer, PeerState::Zombie);
    };
}
unsafe fn enet_protocol_remove_sent_unreliable_commands<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    sent_unreliable_commands: *mut ENetList<ENetOutgoingCommand>,
) {
    let mut outgoing_command: *mut ENetOutgoingCommand;
    if enet_list_empty(sent_unreliable_commands) {
        return;
    }
    loop {
        outgoing_command = enet_list_front(sent_unreliable_commands);
        enet_list_remove(&mut (*outgoing_command).outgoing_command_list);
        if !((*outgoing_command).packet).is_null() {
            (*(*outgoing_command).packet).reference_count -= 1;
            if (*(*outgoing_command).packet).reference_count == 0 {
                (*(*outgoing_command).packet).flags.insert(PacketFlag::SENT);
                enet_packet_destroy((*outgoing_command).packet);
            }
        }
        enet_free(outgoing_command as *mut c_void);
        if enet_list_empty(sent_unreliable_commands) {
            break;
        }
    }
    if peer.state == PeerState::DisconnectLater && enet_peer_has_outgoing_commands(peer) == 0 {
        enet_peer_disconnect(host, peer, peer.event_data);
    }
}
unsafe fn enet_protocol_find_sent_reliable_command(
    list: *mut ENetList<ENetOutgoingCommand>,
    reliable_sequence_number: u16,
    channel_id: u8,
) -> *mut ENetOutgoingCommand {
    let mut current_command = enet_list_begin(list);
    while current_command != enet_list_end(list) {
        let outgoing_command: *mut ENetOutgoingCommand =
            current_command as *mut ENetOutgoingCommand;
        if (*outgoing_command).command.header.command & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE != 0
        {
            if (*outgoing_command).send_attempts < 1 {
                break;
            }
            if (*outgoing_command).reliable_sequence_number == reliable_sequence_number
                && (*outgoing_command).command.header.channel_id == channel_id
            {
                return outgoing_command;
            }
        }
        current_command = enet_list_next(current_command);
    }
    std::ptr::null_mut()
}
unsafe fn enet_protocol_remove_sent_reliable_command<S: Socket>(
    peer: *mut Peer<S>,
    reliable_sequence_number: u16,
    channel_id: u8,
) -> ENetProtocolCommand {
    let mut outgoing_command: *mut ENetOutgoingCommand = std::ptr::null_mut();
    let mut current_command: ENetListIterator<ENetOutgoingCommand>;
    let mut was_sent = true;
    current_command = enet_list_begin(&mut (*peer).sent_reliable_commands);
    while current_command != enet_list_end(&mut (*peer).sent_reliable_commands) {
        outgoing_command = current_command as *mut ENetOutgoingCommand;
        if (*outgoing_command).reliable_sequence_number == reliable_sequence_number
            && (*outgoing_command).command.header.channel_id == channel_id
        {
            break;
        }
        current_command = enet_list_next(current_command);
    }
    if current_command == enet_list_end(&mut (*peer).sent_reliable_commands) {
        outgoing_command = enet_protocol_find_sent_reliable_command(
            &mut (*peer).outgoing_commands,
            reliable_sequence_number,
            channel_id,
        );
        if outgoing_command.is_null() {
            outgoing_command = enet_protocol_find_sent_reliable_command(
                &mut (*peer).outgoing_send_reliable_commands,
                reliable_sequence_number,
                channel_id,
            );
        }
        was_sent = false;
    }
    if outgoing_command.is_null() {
        return ENET_PROTOCOL_COMMAND_NONE;
    }
    if (channel_id as usize) < (*peer).channel_count {
        let channel: *mut Channel =
            &mut *((*peer).channels.as_mut_ptr()).offset(channel_id as isize) as *mut Channel;
        let reliable_window = reliable_sequence_number / ENET_PEER_RELIABLE_WINDOW_SIZE;
        if (*channel).reliable_windows[reliable_window as usize] > 0 {
            (*channel).reliable_windows[reliable_window as usize] -= 1;
            if (*channel).reliable_windows[reliable_window as usize] == 0 {
                (*channel).used_reliable_windows &= !(1 << reliable_window);
            }
        }
    }
    let command_number = ((*outgoing_command).command.header.command & ENET_PROTOCOL_COMMAND_MASK)
        as ENetProtocolCommand;
    enet_list_remove(&mut (*outgoing_command).outgoing_command_list);
    if !((*outgoing_command).packet).is_null() {
        if was_sent {
            (*peer).reliable_data_in_transit -= (*outgoing_command).fragment_length as u32;
        }
        (*(*outgoing_command).packet).reference_count -= 1;
        if (*(*outgoing_command).packet).reference_count == 0 {
            (*(*outgoing_command).packet).flags.insert(PacketFlag::SENT);
            enet_packet_destroy((*outgoing_command).packet);
        }
    }
    enet_free(outgoing_command as *mut c_void);
    if enet_list_empty(&mut (*peer).sent_reliable_commands) {
        return command_number;
    }
    outgoing_command = enet_list_front(&mut (*peer).sent_reliable_commands);
    (*peer).next_timeout = (*outgoing_command).sent_time - (*outgoing_command).round_trip_timeout;
    command_number
}
unsafe fn enet_protocol_handle_connect<S: Socket>(
    host: &mut Host<S>,
    mut _header: *mut ENetProtocolHeader,
    command: *mut ENetProtocol,
) -> *mut Peer<S> {
    let mut incoming_session_id: u8;
    let mut outgoing_session_id: u8;
    let mut window_size: u32;
    let mut channel: *mut Channel;
    let mut channel_count: usize;
    let mut duplicate_peers: usize = 0;
    let mut current_peer: *mut Peer<S>;
    let mut peer: *mut Peer<S> = std::ptr::null_mut();
    let mut verify_command = ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channel_id: 0,
            reliable_sequence_number: 0,
        },
    };
    channel_count = u32::from_be((*command).connect.channel_count) as usize;
    if !(ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT..=ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT)
        .contains(&channel_count)
    {
        return std::ptr::null_mut();
    }
    current_peer = host.peers.as_mut_ptr();
    while current_peer < &mut *(host.peers.as_mut_ptr()).add(host.peer_count) as *mut Peer<S> {
        if (*current_peer).state == PeerState::Disconnected {
            if peer.is_null() {
                peer = current_peer;
            }
        } else if (*current_peer).state != PeerState::Connecting
            && (*current_peer)
                .address
                .as_ref()
                .unwrap()
                .same_host(host.received_address.as_ref().unwrap())
        // TODO: remove unwraps
        {
            if (*current_peer)
                .address
                .as_ref()
                .unwrap()
                .same(host.received_address.as_ref().unwrap())
                && (*current_peer).connect_id == (*command).connect.connect_id
            // TODO: remove unwraps
            {
                return std::ptr::null_mut();
            }
            duplicate_peers += 1;
        }
        current_peer = current_peer.offset(1);
    }
    if peer.is_null() || duplicate_peers >= host.duplicate_peers {
        return std::ptr::null_mut();
    }
    if channel_count > host.channel_limit {
        channel_count = host.channel_limit;
    }
    (*peer).channels = vec![];
    for _ in 0..channel_count {
        (*peer).channels.push(Channel::default());
    }
    (*peer).channel_count = channel_count;
    (*peer).state = PeerState::AcknowledgingConnect;
    (*peer).connect_id = (*command).connect.connect_id;
    (*peer).address = host.received_address.clone();
    (*peer).mtu = host.mtu;
    (*peer).outgoing_peer_id = u16::from_be((*command).connect.outgoing_peer_id);
    (*peer).incoming_bandwidth = u32::from_be((*command).connect.incoming_bandwidth);
    (*peer).outgoing_bandwidth = u32::from_be((*command).connect.outgoing_bandwidth);
    (*peer).packet_throttle_interval = u32::from_be((*command).connect.packet_throttle_interval);
    (*peer).packet_throttle_acceleration =
        u32::from_be((*command).connect.packet_throttle_acceleration);
    (*peer).packet_throttle_deceleration =
        u32::from_be((*command).connect.packet_throttle_deceleration);
    (*peer).event_data = u32::from_be((*command).connect.data);
    incoming_session_id = (if (*command).connect.incoming_session_id == 0xff {
        (*peer).outgoing_session_id
    } else {
        (*command).connect.incoming_session_id
    }) as u8;
    incoming_session_id = ((incoming_session_id.wrapping_add(1) as i32)
        & ENET_PROTOCOL_HEADER_SESSION_MASK as i32 >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as i32)
        as u8;
    if incoming_session_id == (*peer).outgoing_session_id {
        incoming_session_id = ((incoming_session_id + 1) as i32
            & ENET_PROTOCOL_HEADER_SESSION_MASK as i32 >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as i32)
            as u8;
    }
    (*peer).outgoing_session_id = incoming_session_id;
    outgoing_session_id = (if (*command).connect.outgoing_session_id == 0xff {
        (*peer).incoming_session_id
    } else {
        (*command).connect.outgoing_session_id
    }) as u8;
    outgoing_session_id = (outgoing_session_id.wrapping_add(1) as i32
        & ENET_PROTOCOL_HEADER_SESSION_MASK as i32 >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as i32)
        as u8;
    if outgoing_session_id == (*peer).incoming_session_id {
        outgoing_session_id = ((outgoing_session_id as i32 + 1)
            & ENET_PROTOCOL_HEADER_SESSION_MASK as i32 >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as i32)
            as u8;
    }
    (*peer).incoming_session_id = outgoing_session_id;
    channel = (*peer).channels.as_mut_ptr();
    while channel < &mut *((*peer).channels.as_mut_ptr()).add(channel_count) as *mut Channel {
        (*channel).outgoing_reliable_sequence_number = 0;
        (*channel).outgoing_unreliable_sequence_number = 0;
        (*channel).incoming_reliable_sequence_number = 0;
        (*channel).incoming_unreliable_sequence_number = 0;
        enet_list_clear(&mut (*channel).incoming_reliable_commands);
        enet_list_clear(&mut (*channel).incoming_unreliable_commands);
        (*channel).used_reliable_windows = 0;
        enet_memset(
            ((*channel).reliable_windows).as_mut_ptr() as *mut c_void,
            0,
            size_of::<[u16; 16]>(),
        );
        channel = channel.offset(1);
    }
    let mtu = u32::from_be((*command).connect.mtu)
        .clamp(ENET_PROTOCOL_MINIMUM_MTU, ENET_PROTOCOL_MAXIMUM_MTU);
    if mtu < (*peer).mtu {
        (*peer).mtu = mtu;
    }
    if host.outgoing_bandwidth == 0 && (*peer).incoming_bandwidth == 0 {
        (*peer).window_size = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE;
    } else if host.outgoing_bandwidth == 0 || (*peer).incoming_bandwidth == 0 {
        (*peer).window_size = ((if host.outgoing_bandwidth > (*peer).incoming_bandwidth {
            host.outgoing_bandwidth
        } else {
            (*peer).incoming_bandwidth
        }) / ENET_PEER_WINDOW_SIZE_SCALE)
            * ENET_PROTOCOL_MINIMUM_WINDOW_SIZE
    } else {
        (*peer).window_size = ((if host.outgoing_bandwidth < (*peer).incoming_bandwidth {
            host.outgoing_bandwidth
        } else {
            (*peer).incoming_bandwidth
        }) / ENET_PEER_WINDOW_SIZE_SCALE)
            * ENET_PROTOCOL_MINIMUM_WINDOW_SIZE
    }
    (*peer).window_size = (*peer).window_size.clamp(
        ENET_PROTOCOL_MINIMUM_WINDOW_SIZE,
        ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE,
    );
    if host.incoming_bandwidth == 0 {
        window_size = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE;
    } else {
        window_size = (host.incoming_bandwidth / ENET_PEER_WINDOW_SIZE_SCALE)
            * ENET_PROTOCOL_MINIMUM_WINDOW_SIZE;
    }
    if window_size > u32::from_be((*command).connect.window_size) {
        window_size = u32::from_be((*command).connect.window_size);
    }
    window_size = window_size.clamp(
        ENET_PROTOCOL_MINIMUM_WINDOW_SIZE,
        ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE,
    );
    verify_command.header.command =
        ENET_PROTOCOL_COMMAND_VERIFY_CONNECT | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE;
    verify_command.header.channel_id = 0xff;
    verify_command.verify_connect.outgoing_peer_id = ((*peer).incoming_peer_id).to_be();
    verify_command.verify_connect.incoming_session_id = incoming_session_id;
    verify_command.verify_connect.outgoing_session_id = outgoing_session_id;
    verify_command.verify_connect.mtu = (*peer).mtu.to_be();
    verify_command.verify_connect.window_size = window_size.to_be();
    verify_command.verify_connect.channel_count = (channel_count as u32).to_be();
    verify_command.verify_connect.incoming_bandwidth = host.incoming_bandwidth.to_be();
    verify_command.verify_connect.outgoing_bandwidth = host.outgoing_bandwidth.to_be();
    verify_command.verify_connect.packet_throttle_interval =
        (*peer).packet_throttle_interval.to_be();
    verify_command.verify_connect.packet_throttle_acceleration =
        (*peer).packet_throttle_acceleration.to_be();
    verify_command.verify_connect.packet_throttle_deceleration =
        (*peer).packet_throttle_deceleration.to_be();
    verify_command.verify_connect.connect_id = (*peer).connect_id;
    enet_peer_queue_outgoing_command(
        host,
        &mut *peer,
        &verify_command,
        std::ptr::null_mut(),
        0,
        0,
    );
    peer
}
unsafe fn enet_protocol_handle_send_reliable<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    command: *const ENetProtocol,
    current_data: *mut *mut u8,
) -> i32 {
    if (*command).header.channel_id as usize >= peer.channel_count
        || peer.state != PeerState::Connected && peer.state != PeerState::DisconnectLater
    {
        return -1;
    }
    let data_length = u16::from_be((*command).send_reliable.data_length) as usize;
    *current_data = (*current_data).add(data_length);
    if data_length > host.maximum_packet_size
        || *current_data < host.received_data()
        || *current_data > &mut *(host.received_data()).add(host.received_data_length) as *mut u8
    {
        return -1;
    }
    if (enet_peer_queue_incoming_command(
        host,
        peer,
        command,
        (command as *const u8).add(size_of::<ENetProtocolSendReliable>()) as *const c_void,
        data_length,
        PacketFlag::RELIABLE,
        0,
    ))
    .is_null()
    {
        return -1;
    }
    0
}
unsafe fn enet_protocol_handle_send_unsequenced<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    command: *const ENetProtocol,
    current_data: *mut *mut u8,
) -> i32 {
    let mut unsequenced_group: u32;
    if (*command).header.channel_id as usize >= peer.channel_count
        || peer.state != PeerState::Connected && peer.state != PeerState::DisconnectLater
    {
        return -1;
    }
    let data_length = u16::from_be((*command).send_unsequenced.data_length) as usize;
    *current_data = (*current_data).add(data_length);
    if data_length > host.maximum_packet_size
        || *current_data < host.received_data()
        || *current_data > &mut *(host.received_data()).add(host.received_data_length) as *mut u8
    {
        return -1;
    }
    unsequenced_group = u16::from_be((*command).send_unsequenced.unsequenced_group) as u32;
    let index = unsequenced_group % ENET_PEER_UNSEQUENCED_WINDOW_SIZE as u32;
    if unsequenced_group < peer.incoming_unsequenced_group as u32 {
        unsequenced_group += 0x10000;
    }
    if unsequenced_group
        >= (peer.incoming_unsequenced_group as u32)
            + (ENET_PEER_FREE_UNSEQUENCED_WINDOWS * ENET_PEER_UNSEQUENCED_WINDOW_SIZE as u32)
    {
        return 0;
    }
    unsequenced_group &= 0xffff;
    if unsequenced_group - index != peer.incoming_unsequenced_group as u32 {
        peer.incoming_unsequenced_group = (unsequenced_group - index) as u16;
        enet_memset(
            (peer.unsequenced_window).as_mut_ptr() as *mut c_void,
            0,
            size_of::<[u32; ENET_PEER_UNSEQUENCED_WINDOW_SIZE / 32]>(),
        );
    } else if peer.unsequenced_window[(index / 32) as usize] & (1 << (index % 32)) != 0 {
        return 0;
    }
    if (enet_peer_queue_incoming_command(
        host,
        peer,
        command,
        (command as *const u8).add(size_of::<ENetProtocolSendUnsequenced>()) as *const c_void,
        data_length,
        PacketFlag::UNSEQUENCED,
        0,
    ))
    .is_null()
    {
        return -1;
    }
    peer.unsequenced_window[(index / 32) as usize] |= 1 << (index % 32);
    0
}
unsafe fn enet_protocol_handle_send_unreliable<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    command: *const ENetProtocol,
    current_data: *mut *mut u8,
) -> i32 {
    if (*command).header.channel_id as usize >= peer.channel_count
        || peer.state != PeerState::Connected && peer.state != PeerState::DisconnectLater
    {
        return -1;
    }
    let data_length = u16::from_be((*command).send_unreliable.data_length) as usize;
    *current_data = (*current_data).add(data_length);
    if data_length > host.maximum_packet_size
        || *current_data < host.received_data()
        || *current_data > &mut *(host.received_data()).add(host.received_data_length) as *mut u8
    {
        return -1;
    }
    if (enet_peer_queue_incoming_command(
        host,
        peer,
        command,
        (command as *const u8).add(size_of::<ENetProtocolSendUnreliable>()) as *const c_void,
        data_length,
        PacketFlag::empty(),
        0,
    ))
    .is_null()
    {
        return -1;
    }
    0
}
unsafe fn enet_protocol_handle_send_fragment<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    command: *const ENetProtocol,
    current_data: *mut *mut u8,
) -> i32 {
    let mut fragment_length: u32;
    let mut start_window: u16;
    let mut start_command: *mut ENetIncomingCommand = std::ptr::null_mut();
    if (*command).header.channel_id as usize >= peer.channel_count
        || peer.state != PeerState::Connected && peer.state != PeerState::DisconnectLater
    {
        return -1;
    }
    fragment_length = u16::from_be((*command).send_fragment.data_length) as u32;
    *current_data = (*current_data).offset(fragment_length as isize);
    if fragment_length == 0
        || fragment_length as usize > host.maximum_packet_size
        || *current_data < host.received_data()
        || *current_data > &mut *(host.received_data()).add(host.received_data_length) as *mut u8
    {
        return -1;
    }
    let channel = &mut *(peer.channels.as_mut_ptr()).offset((*command).header.channel_id as isize)
        as *mut Channel;
    let start_sequence_number = u16::from_be((*command).send_fragment.start_sequence_number) as u32;
    start_window = (start_sequence_number / ENET_PEER_RELIABLE_WINDOW_SIZE as u32) as u16;
    let current_window =
        (*channel).incoming_reliable_sequence_number / ENET_PEER_RELIABLE_WINDOW_SIZE;
    if start_sequence_number < (*channel).incoming_reliable_sequence_number as u32 {
        start_window += ENET_PEER_RELIABLE_WINDOWS;
    }
    if (start_window) < current_window
        || start_window >= current_window + ENET_PEER_FREE_RELIABLE_WINDOWS - 1
    {
        return 0;
    }
    let fragment_number = u32::from_be((*command).send_fragment.fragment_number);
    let fragment_count = u32::from_be((*command).send_fragment.fragment_count);
    let fragment_offset = u32::from_be((*command).send_fragment.fragment_offset);
    let total_length = u32::from_be((*command).send_fragment.total_length);
    if fragment_count > ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT
        || fragment_number >= fragment_count
        || total_length as usize > host.maximum_packet_size
        || total_length < fragment_count
        || fragment_offset >= total_length
        || fragment_length > (total_length - fragment_offset)
    {
        return -1;
    }

    let mut current_command =
        enet_list_previous(enet_list_end(&mut (*channel).incoming_reliable_commands));
    while current_command != enet_list_end(&mut (*channel).incoming_reliable_commands) {
        let incoming_command: *mut ENetIncomingCommand =
            current_command as *mut ENetIncomingCommand;

        if start_sequence_number >= (*channel).incoming_reliable_sequence_number as u32 {
            if (*incoming_command).reliable_sequence_number
                < (*channel).incoming_reliable_sequence_number
            {
                current_command = enet_list_previous(current_command);
                continue;
            }
        } else if (*incoming_command).reliable_sequence_number
            >= (*channel).incoming_reliable_sequence_number
        {
            break;
        }

        if (*incoming_command).reliable_sequence_number <= start_sequence_number as u16 {
            if (*incoming_command).reliable_sequence_number < start_sequence_number as u16 {
                break;
            }

            if ((*incoming_command).command.header.command & ENET_PROTOCOL_COMMAND_MASK)
                != ENET_PROTOCOL_COMMAND_SEND_FRAGMENT
                || total_length as usize != (*(*incoming_command).packet).data_length
                || fragment_count != (*incoming_command).fragment_count
            {
                return -1;
            }

            start_command = incoming_command;
            break;
        }

        current_command = enet_list_previous(current_command);
    }

    if start_command.is_null() {
        let mut host_command: ENetProtocol = *command;
        host_command.header.reliable_sequence_number = start_sequence_number as u16;
        start_command = enet_peer_queue_incoming_command(
            host,
            peer,
            &host_command,
            std::ptr::null(),
            total_length as usize,
            PacketFlag::RELIABLE,
            fragment_count,
        );
        if start_command.is_null() {
            return -1;
        }
    }
    if *((*start_command).fragments).offset((fragment_number / 32) as isize)
        & (1 << (fragment_number % 32))
        == 0
    {
        (*start_command).fragments_remaining -= 1;
        let fresh32 = &mut *((*start_command).fragments).add((fragment_number / 32) as usize);
        *fresh32 |= 1 << (fragment_number % 32);
        if (fragment_offset + fragment_length) as usize > (*(*start_command).packet).data_length {
            fragment_length =
                (((*(*start_command).packet).data_length) - (fragment_offset as usize)) as u32;
        }
        copy_nonoverlapping(
            (command as *mut u8).add(size_of::<ENetProtocolSendFragment>()) as *const c_void,
            ((*(*start_command).packet).data).offset(fragment_offset as isize) as *mut c_void,
            fragment_length as usize,
        );
        if (*start_command).fragments_remaining == 0 {
            enet_peer_dispatch_incoming_reliable_commands(
                host,
                peer,
                channel,
                std::ptr::null_mut(),
            );
        }
    }
    0
}
unsafe fn enet_protocol_handle_send_unreliable_fragment<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    command: *const ENetProtocol,
    current_data: *mut *mut u8,
) -> i32 {
    let mut fragment_length: u32;
    let mut reliable_window: u16;
    let mut start_command: *mut ENetIncomingCommand = std::ptr::null_mut();
    if (*command).header.channel_id as usize >= peer.channel_count
        || peer.state != PeerState::Connected && peer.state != PeerState::DisconnectLater
    {
        return -1;
    }
    fragment_length = u16::from_be((*command).send_fragment.data_length) as u32;
    *current_data = (*current_data).offset(fragment_length as isize);
    if fragment_length as usize > host.maximum_packet_size
        || *current_data < host.received_data()
        || *current_data > &mut *(host.received_data()).add(host.received_data_length) as *mut u8
    {
        return -1;
    }
    let channel = &mut *(peer.channels.as_mut_ptr()).offset((*command).header.channel_id as isize)
        as *mut Channel;
    let reliable_sequence_number = (*command).header.reliable_sequence_number as u32;
    let start_sequence_number = u16::from_be((*command).send_fragment.start_sequence_number) as u32;
    reliable_window = (reliable_sequence_number / ENET_PEER_RELIABLE_WINDOW_SIZE as u32) as u16;
    let current_window =
        (*channel).incoming_reliable_sequence_number / ENET_PEER_RELIABLE_WINDOW_SIZE;
    if reliable_sequence_number < (*channel).incoming_reliable_sequence_number as u32 {
        reliable_window += ENET_PEER_RELIABLE_WINDOWS;
    }
    if (reliable_window) < current_window
        || reliable_window >= current_window + ENET_PEER_FREE_RELIABLE_WINDOWS - 1
    {
        return 0;
    }
    if reliable_sequence_number == (*channel).incoming_reliable_sequence_number as u32
        && start_sequence_number <= (*channel).incoming_unreliable_sequence_number as u32
    {
        return 0;
    }
    let fragment_number = u32::from_be((*command).send_fragment.fragment_number);
    let fragment_count = u32::from_be((*command).send_fragment.fragment_count);
    let fragment_offset = u32::from_be((*command).send_fragment.fragment_offset);
    let total_length = u32::from_be((*command).send_fragment.total_length);
    if fragment_count > ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT
        || fragment_number >= fragment_count
        || total_length as usize > host.maximum_packet_size
        || fragment_offset >= total_length
        || fragment_length > (total_length - fragment_offset)
    {
        return -1;
    }

    let mut current_command =
        enet_list_previous(enet_list_end(&mut (*channel).incoming_unreliable_commands));
    while current_command != enet_list_end(&mut (*channel).incoming_unreliable_commands) {
        let incoming_command: *mut ENetIncomingCommand =
            current_command as *mut ENetIncomingCommand;

        if reliable_sequence_number >= (*channel).incoming_reliable_sequence_number as u32 {
            if (*incoming_command).reliable_sequence_number
                < (*channel).incoming_reliable_sequence_number
            {
                current_command = enet_list_previous(current_command);
                continue;
            }
        } else if (*incoming_command).reliable_sequence_number
            >= (*channel).incoming_reliable_sequence_number
        {
            break;
        }

        if ((*incoming_command).reliable_sequence_number as u32) < reliable_sequence_number {
            break;
        }

        if (*incoming_command).reliable_sequence_number as u32 > reliable_sequence_number {
            current_command = enet_list_previous(current_command);
            continue;
        }

        if (*incoming_command).unreliable_sequence_number as u32 <= start_sequence_number {
            if ((*incoming_command).unreliable_sequence_number as u32) < start_sequence_number {
                break;
            }

            if ((*incoming_command).command.header.command & ENET_PROTOCOL_COMMAND_MASK)
                != ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT
                || total_length as usize != (*(*incoming_command).packet).data_length
                || fragment_count != (*incoming_command).fragment_count
            {
                return -1;
            }

            start_command = incoming_command;
            break;
        }

        current_command = enet_list_previous(current_command);
    }

    if start_command.is_null() {
        start_command = enet_peer_queue_incoming_command(
            host,
            peer,
            command,
            std::ptr::null_mut(),
            total_length as usize,
            PacketFlag::UNRELIABLE_FRAGMENT,
            fragment_count,
        );
        if start_command.is_null() {
            return -1;
        }
    }
    if *((*start_command).fragments).add(fragment_number as usize / 32)
        & (1 << (fragment_number % 32))
        == 0
    {
        (*start_command).fragments_remaining -= 1;
        let fresh33 = &mut *((*start_command).fragments).add((fragment_number / 32) as usize);
        *fresh33 |= 1 << (fragment_number % 32);
        if (fragment_offset + fragment_length) as usize > (*(*start_command).packet).data_length {
            fragment_length =
                (((*(*start_command).packet).data_length) - fragment_offset as usize) as u32;
        }
        copy_nonoverlapping(
            (command as *mut u8).add(size_of::<ENetProtocolSendFragment>()) as *const c_void,
            ((*(*start_command).packet).data).offset(fragment_offset as isize) as *mut c_void,
            fragment_length as usize,
        );
        if (*start_command).fragments_remaining == 0 {
            enet_peer_dispatch_incoming_unreliable_commands(
                host,
                peer,
                channel,
                std::ptr::null_mut(),
            );
        }
    }
    0
}
unsafe fn enet_protocol_handle_ping<S: Socket>(
    _host: &mut Host<S>,
    peer: &mut Peer<S>,
    mut _command: *const ENetProtocol,
) -> i32 {
    if peer.state != PeerState::Connected && peer.state != PeerState::DisconnectLater {
        -1
    } else {
        0
    }
}
unsafe fn enet_protocol_handle_bandwidth_limit<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    command: *const ENetProtocol,
) -> i32 {
    if peer.state != PeerState::Connected && peer.state != PeerState::DisconnectLater {
        return -1;
    }
    if peer.incoming_bandwidth != 0 {
        host.bandwidth_limited_peers -= 1;
    }
    peer.incoming_bandwidth = u32::from_be((*command).bandwidth_limit.incoming_bandwidth);
    peer.outgoing_bandwidth = u32::from_be((*command).bandwidth_limit.outgoing_bandwidth);
    if peer.incoming_bandwidth != 0 {
        host.bandwidth_limited_peers += 1;
    }
    if peer.incoming_bandwidth == 0 && host.outgoing_bandwidth == 0 {
        peer.window_size = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE
    } else if peer.incoming_bandwidth == 0 || host.outgoing_bandwidth == 0 {
        peer.window_size = ((if peer.incoming_bandwidth > host.outgoing_bandwidth {
            peer.incoming_bandwidth
        } else {
            host.outgoing_bandwidth
        }) / ENET_PEER_WINDOW_SIZE_SCALE)
            * ENET_PROTOCOL_MINIMUM_WINDOW_SIZE;
    } else {
        peer.window_size = ((if peer.incoming_bandwidth < host.outgoing_bandwidth {
            peer.incoming_bandwidth
        } else {
            host.outgoing_bandwidth
        }) / ENET_PEER_WINDOW_SIZE_SCALE)
            * ENET_PROTOCOL_MINIMUM_WINDOW_SIZE;
    }
    peer.window_size = peer.window_size.clamp(
        ENET_PROTOCOL_MINIMUM_WINDOW_SIZE,
        ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE,
    );
    0
}
unsafe fn enet_protocol_handle_throttle_configure<S: Socket>(
    _host: &mut Host<S>,
    peer: &mut Peer<S>,
    command: *const ENetProtocol,
) -> i32 {
    if peer.state != PeerState::Connected && peer.state != PeerState::DisconnectLater {
        return -1;
    }
    peer.packet_throttle_interval =
        u32::from_be((*command).throttle_configure.packet_throttle_interval);
    peer.packet_throttle_acceleration =
        u32::from_be((*command).throttle_configure.packet_throttle_acceleration);
    peer.packet_throttle_deceleration =
        u32::from_be((*command).throttle_configure.packet_throttle_deceleration);
    0
}
unsafe fn enet_protocol_handle_disconnect<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    command: *const ENetProtocol,
) -> i32 {
    if peer.state == PeerState::Disconnected
        || peer.state == PeerState::Zombie
        || peer.state == PeerState::AcknowledgingDisconnect
    {
        return 0;
    }
    enet_peer_reset_queues(host, peer);
    if peer.state == PeerState::ConnectionSucceeded
        || peer.state == PeerState::Disconnecting
        || peer.state == PeerState::Connecting
    {
        enet_protocol_dispatch_state(host, peer, PeerState::Zombie);
    } else if peer.state != PeerState::Connected && peer.state != PeerState::DisconnectLater {
        if peer.state == PeerState::ConnectionPending {
            host.recalculate_bandwidth_limits = true;
        }
        enet_peer_reset(host, peer);
    } else if (*command).header.command & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE != 0 {
        enet_protocol_change_state(host, peer, PeerState::AcknowledgingDisconnect);
    } else {
        enet_protocol_dispatch_state(host, peer, PeerState::Zombie);
    }
    if peer.state != PeerState::Disconnected {
        peer.event_data = u32::from_be((*command).disconnect.data);
    }
    0
}
unsafe fn enet_protocol_handle_acknowledge<S: Socket>(
    host: &mut Host<S>,
    event: *mut Event,
    peer: &mut Peer<S>,
    command: *const ENetProtocol,
) -> i32 {
    let mut round_trip_time: u32;
    let mut received_sent_time: u32;
    if peer.state == PeerState::Disconnected || peer.state == PeerState::Zombie {
        return 0;
    }
    received_sent_time = u16::from_be((*command).acknowledge.received_sent_time) as u32;
    received_sent_time |= host.service_time & 0xffff0000;
    if received_sent_time & 0x8000 > host.service_time & 0x8000 {
        received_sent_time -= 0x10000;
    }
    if enet_time_less(host.service_time, received_sent_time) {
        return 0;
    }
    round_trip_time = enet_time_difference(host.service_time, received_sent_time);
    round_trip_time = if round_trip_time > 1 {
        round_trip_time
    } else {
        1
    };
    if peer.last_receive_time > 0 {
        enet_peer_throttle(peer, round_trip_time);
        peer.round_trip_time_variance -= peer.round_trip_time_variance / 4;
        if round_trip_time >= peer.round_trip_time {
            let diff: u32 = round_trip_time - peer.round_trip_time;
            peer.round_trip_time_variance += diff / 4;
            peer.round_trip_time += diff / 8;
        } else {
            let diff_0: u32 = peer.round_trip_time - round_trip_time;
            peer.round_trip_time_variance += diff_0 / 4;
            peer.round_trip_time -= diff_0 / 8;
        }
    } else {
        peer.round_trip_time = round_trip_time;
        peer.round_trip_time_variance = (round_trip_time + 1) / 2;
    }
    if peer.round_trip_time < peer.lowest_round_trip_time {
        peer.lowest_round_trip_time = peer.round_trip_time;
    }
    if peer.round_trip_time_variance > peer.highest_round_trip_time_variance {
        peer.highest_round_trip_time_variance = peer.round_trip_time_variance;
    }
    if peer.packet_throttle_epoch == 0
        || enet_time_difference(host.service_time, peer.packet_throttle_epoch)
            >= peer.packet_throttle_interval
    {
        peer.last_round_trip_time = peer.lowest_round_trip_time;
        peer.last_round_trip_time_variance = if peer.highest_round_trip_time_variance > 1 {
            peer.highest_round_trip_time_variance
        } else {
            1
        };
        peer.lowest_round_trip_time = peer.round_trip_time;
        peer.highest_round_trip_time_variance = peer.round_trip_time_variance;
        peer.packet_throttle_epoch = host.service_time;
    }
    peer.last_receive_time = if host.service_time > 1 {
        host.service_time
    } else {
        1
    };
    peer.earliest_timeout = 0;
    let received_reliable_sequence_number =
        u16::from_be((*command).acknowledge.received_reliable_sequence_number) as u32;
    let command_number = enet_protocol_remove_sent_reliable_command(
        peer,
        received_reliable_sequence_number as u16,
        (*command).header.channel_id,
    );
    match peer.state {
        PeerState::AcknowledgingConnect => {
            if command_number != ENET_PROTOCOL_COMMAND_VERIFY_CONNECT {
                return -1;
            }
            enet_protocol_notify_connect(host, peer, event);
        }
        PeerState::Disconnecting => {
            if command_number != ENET_PROTOCOL_COMMAND_DISCONNECT {
                return -1;
            }
            enet_protocol_notify_disconnect(host, peer, event);
        }
        PeerState::DisconnectLater => {
            if enet_peer_has_outgoing_commands(peer) == 0 {
                enet_peer_disconnect(host, peer, peer.event_data);
            }
        }
        _ => {}
    }
    0
}
unsafe fn enet_protocol_handle_verify_connect<S: Socket>(
    host: &mut Host<S>,
    event: *mut Event,
    peer: &mut Peer<S>,
    command: *const ENetProtocol,
) -> i32 {
    if peer.state != PeerState::Connecting {
        return 0;
    }
    let channel_count = u32::from_be((*command).verify_connect.channel_count) as usize;
    if !(ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT..=ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT)
        .contains(&channel_count)
        || u32::from_be((*command).verify_connect.packet_throttle_interval)
            != peer.packet_throttle_interval
        || u32::from_be((*command).verify_connect.packet_throttle_acceleration)
            != peer.packet_throttle_acceleration
        || u32::from_be((*command).verify_connect.packet_throttle_deceleration)
            != peer.packet_throttle_deceleration
        || (*command).verify_connect.connect_id != peer.connect_id
    {
        peer.event_data = 0;
        enet_protocol_dispatch_state(host, peer, PeerState::Zombie);
        return -1;
    }
    enet_protocol_remove_sent_reliable_command(peer, 1, 0xff);
    if channel_count < peer.channel_count {
        peer.channel_count = channel_count;
    }
    peer.outgoing_peer_id = u16::from_be((*command).verify_connect.outgoing_peer_id);
    peer.incoming_session_id = (*command).verify_connect.incoming_session_id;
    peer.outgoing_session_id = (*command).verify_connect.outgoing_session_id;
    let mtu = u32::from_be((*command).verify_connect.mtu)
        .clamp(ENET_PROTOCOL_MINIMUM_MTU, ENET_PROTOCOL_MAXIMUM_MTU);
    if mtu < peer.mtu {
        peer.mtu = mtu;
    }
    let window_size = u32::from_be((*command).verify_connect.window_size).clamp(
        ENET_PROTOCOL_MINIMUM_WINDOW_SIZE,
        ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE,
    );
    if window_size < peer.window_size {
        peer.window_size = window_size;
    }
    peer.incoming_bandwidth = u32::from_be((*command).verify_connect.incoming_bandwidth);
    peer.outgoing_bandwidth = u32::from_be((*command).verify_connect.outgoing_bandwidth);
    enet_protocol_notify_connect(host, peer, event);
    0
}
unsafe fn enet_protocol_handle_incoming_commands<S: Socket>(
    host: &mut Host<S>,
    event: *mut Event,
) -> Result<bool, crate::Error> {
    let mut command: *mut ENetProtocol;
    let mut peer: *mut Peer<S>;
    let mut current_data: *mut u8;
    let mut peer_id: u16;
    if host.received_data_length < SENT_TIME_OFFSET {
        return Ok(false);
    }
    let header = host.received_data() as *mut ENetProtocolHeader;
    peer_id = u16::from_be((*header).peer_id);
    let session_id =
        ((peer_id & ENET_PROTOCOL_HEADER_SESSION_MASK) >> ENET_PROTOCOL_HEADER_SESSION_SHIFT) as u8;
    let flags = peer_id & ENET_PROTOCOL_HEADER_FLAG_MASK;
    peer_id &= !(ENET_PROTOCOL_HEADER_FLAG_MASK | ENET_PROTOCOL_HEADER_SESSION_MASK);
    let header_size = if flags & ENET_PROTOCOL_HEADER_FLAG_SENT_TIME != 0 {
        size_of::<ENetProtocolHeader>()
    } else {
        SENT_TIME_OFFSET
    };
    if peer_id == ENET_PROTOCOL_MAXIMUM_PEER_ID {
        peer = std::ptr::null_mut();
    } else if peer_id as usize >= host.peer_count {
        return Ok(false);
    } else {
        // TODO: remove unwraps
        peer = &mut *(host.peers.as_mut_ptr()).offset(peer_id as isize) as *mut Peer<S>;
        if (*peer).state == PeerState::Disconnected
            || (*peer).state == PeerState::Zombie
            || ((!host
                .received_address
                .as_ref()
                .unwrap()
                .same_host((*peer).address.as_ref().unwrap())
                || !host
                    .received_address
                    .as_ref()
                    .unwrap()
                    .same((*peer).address.as_ref().unwrap()))
                && !(*peer).address.as_ref().unwrap().is_broadcast())
            || (((*peer).outgoing_peer_id) < ENET_PROTOCOL_MAXIMUM_PEER_ID
                && session_id != (*peer).incoming_session_id)
        {
            return Ok(false);
        }
    }
    if !peer.is_null() {
        (*peer).address = host.received_address.clone();
        (*peer).incoming_data_total += host.received_data_length as u32;
    }
    current_data = (host.received_data()).add(header_size);
    while current_data < &mut *(host.received_data()).add(host.received_data_length) as *mut u8 {
        command = current_data as *mut ENetProtocol;
        if current_data.add(size_of::<ENetProtocolCommandHeader>())
            > &mut *(host.received_data()).add(host.received_data_length) as *mut u8
        {
            break;
        }
        let command_number = (*command).header.command & ENET_PROTOCOL_COMMAND_MASK;
        if command_number >= ENET_PROTOCOL_COMMAND_COUNT {
            break;
        }
        let command_size = COMMAND_SIZES[command_number as usize];
        if command_size == 0
            || current_data.add(command_size)
                > &mut *(host.received_data()).add(host.received_data_length) as *mut u8
        {
            break;
        }
        current_data = current_data.add(command_size);
        if peer.is_null() && command_number != ENET_PROTOCOL_COMMAND_CONNECT {
            break;
        }
        (*command).header.reliable_sequence_number =
            u16::from_be((*command).header.reliable_sequence_number);
        match command_number {
            1 => {
                if enet_protocol_handle_acknowledge(host, event, &mut *peer, command) != 0 {
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
                if enet_protocol_handle_verify_connect(host, event, &mut *peer, command) != 0 {
                    break;
                }
            }
            4 => {
                if enet_protocol_handle_disconnect(host, &mut *peer, command) != 0 {
                    break;
                }
            }
            5 => {
                if enet_protocol_handle_ping(host, &mut *peer, command) != 0 {
                    break;
                }
            }
            6 => {
                if enet_protocol_handle_send_reliable(host, &mut *peer, command, &mut current_data)
                    != 0
                {
                    break;
                }
            }
            7 => {
                if enet_protocol_handle_send_unreliable(
                    host,
                    &mut *peer,
                    command,
                    &mut current_data,
                ) != 0
                {
                    break;
                }
            }
            9 => {
                if enet_protocol_handle_send_unsequenced(
                    host,
                    &mut *peer,
                    command,
                    &mut current_data,
                ) != 0
                {
                    break;
                }
            }
            8 => {
                if enet_protocol_handle_send_fragment(host, &mut *peer, command, &mut current_data)
                    != 0
                {
                    break;
                }
            }
            10 => {
                if enet_protocol_handle_bandwidth_limit(host, &mut *peer, command) != 0 {
                    break;
                }
            }
            11 => {
                if enet_protocol_handle_throttle_configure(host, &mut *peer, command) != 0 {
                    break;
                }
            }
            12 => {
                if enet_protocol_handle_send_unreliable_fragment(
                    host,
                    &mut *peer,
                    command,
                    &mut current_data,
                ) != 0
                {
                    break;
                }
            }
            _ => {
                break;
            }
        }
        if peer.is_null() || (*command).header.command & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE == 0
        {
            continue;
        }
        if flags & ENET_PROTOCOL_HEADER_FLAG_SENT_TIME == 0 {
            break;
        }
        let sent_time = u16::from_be((*header).sent_time);
        match (*peer).state {
            PeerState::Disconnecting
            | PeerState::AcknowledgingConnect
            | PeerState::Disconnected
            | PeerState::Zombie => {}
            PeerState::AcknowledgingDisconnect => {
                if (*command).header.command & ENET_PROTOCOL_COMMAND_MASK
                    == ENET_PROTOCOL_COMMAND_DISCONNECT
                {
                    _ = enet_peer_queue_acknowledgement(&mut *peer, command, sent_time);
                }
            }
            _ => {
                _ = enet_peer_queue_acknowledgement(&mut *peer, command, sent_time);
            }
        }
    }
    if !event.is_null() && !matches!(*event, Event::None) {
        Ok(true)
    } else {
        Ok(false)
    }
}
pub(crate) unsafe fn enet_protocol_receive_incoming_commands<S: Socket>(
    host: &mut Host<S>,
    event: *mut Event,
) -> Result<bool, crate::Error> {
    let mut packets = 0;
    while packets < 256 {
        let mut buffer: ENetBuffer = ENetBuffer {
            data: std::ptr::null_mut(),
            data_length: 0,
        };
        buffer.data = (host.packet_data[0]).as_mut_ptr() as *mut c_void;
        buffer.data_length = size_of::<[u8; 4096]>();
        // TODO: remove unwrap
        if let Some((receive_addr, receive_data)) = host
            .socket
            .receive(4096)
            .map_err(|err| crate::Error::FailedToReceive(Box::new(err)))?
        {
            copy_nonoverlapping(
                receive_data.as_ptr() as *const c_void,
                buffer.data,
                receive_data.len(),
            );
            host.received_address = Some(receive_addr);
            host.received_data_index = 0;
            host.received_data_length = receive_data.len();
            host.total_received_data += receive_data.len() as u32;
            host.total_received_packets += 1;
            if enet_protocol_handle_incoming_commands(host, event)? {
                return Ok(true);
            }
            packets += 1;
        } else {
            return Ok(false);
        }
    }
    Ok(false)
}
unsafe fn enet_protocol_send_acknowledgements<S: Socket>(host: &mut Host<S>, peer: &mut Peer<S>) {
    let mut command: *mut ENetProtocol =
        &mut *(host.commands).as_mut_ptr().add(host.command_count) as *mut ENetProtocol;
    let mut buffer: *mut ENetBuffer =
        &mut *(host.buffers).as_mut_ptr().add(host.buffer_count) as *mut ENetBuffer;
    let mut reliable_sequence_number: u16;
    while let Some(acknowledgement) = peer.acknowledgements.front() {
        if command
            >= &mut *(host.commands).as_mut_ptr().add(
                size_of::<[ENetProtocol; ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS]>()
                    / size_of::<ENetProtocol>(),
            ) as *mut ENetProtocol
            || buffer
                >= &mut *(host.buffers)
                    .as_mut_ptr()
                    .add(size_of::<[ENetBuffer; 65]>() / size_of::<ENetBuffer>())
                    as *mut ENetBuffer
            || (peer.mtu as usize - host.packet_size) < size_of::<ENetProtocolAcknowledge>()
        {
            peer.flags |= ENET_PEER_FLAG_CONTINUE_SENDING;
            break;
        } else {
            (*buffer).data = command as *mut c_void;
            (*buffer).data_length = size_of::<ENetProtocolAcknowledge>();
            host.packet_size += (*buffer).data_length;
            reliable_sequence_number =
                (acknowledgement.command.header.reliable_sequence_number).to_be();
            (*command).header.command = ENET_PROTOCOL_COMMAND_ACKNOWLEDGE;
            (*command).header.channel_id = acknowledgement.command.header.channel_id;
            (*command).header.reliable_sequence_number = reliable_sequence_number;
            (*command).acknowledge.received_reliable_sequence_number = reliable_sequence_number;
            (*command).acknowledge.received_sent_time = (acknowledgement.sent_time as u16).to_be();
            if acknowledgement.command.header.command & ENET_PROTOCOL_COMMAND_MASK
                == ENET_PROTOCOL_COMMAND_DISCONNECT
            {
                enet_protocol_dispatch_state(host, peer, PeerState::Zombie);
            }
            peer.acknowledgements.pop_front();
            command = command.offset(1);
            buffer = buffer.offset(1);
        }
    }
    host.command_count = command.offset_from((host.commands).as_mut_ptr()) as usize;
    host.buffer_count = buffer.offset_from((host.buffers).as_mut_ptr()) as usize;
}
unsafe fn enet_protocol_check_timeouts<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    event: *mut Event,
) -> i32 {
    let mut outgoing_command: *mut ENetOutgoingCommand;
    let mut current_command = enet_list_begin(&mut peer.sent_reliable_commands);
    let insert_position = enet_list_begin(&mut peer.outgoing_commands);
    let insert_send_reliable_position = enet_list_begin(&mut peer.outgoing_send_reliable_commands);
    while current_command != enet_list_end(&mut peer.sent_reliable_commands) {
        outgoing_command = current_command as *mut ENetOutgoingCommand;
        current_command = enet_list_next(current_command);
        if enet_time_difference(host.service_time, (*outgoing_command).sent_time)
            < (*outgoing_command).round_trip_timeout
        {
            continue;
        }
        if peer.earliest_timeout == 0
            || enet_time_less((*outgoing_command).sent_time, peer.earliest_timeout)
        {
            peer.earliest_timeout = (*outgoing_command).sent_time;
        }
        if peer.earliest_timeout != 0
            && (enet_time_difference(host.service_time, peer.earliest_timeout)
                >= peer.timeout_maximum
                || (1 << ((*outgoing_command).send_attempts - 1)) >= peer.timeout_limit
                    && enet_time_difference(host.service_time, peer.earliest_timeout)
                        >= peer.timeout_minimum)
        {
            enet_protocol_notify_disconnect(host, peer, event);
            return 1;
        }
        peer.packets_lost += 1;
        (*outgoing_command).round_trip_timeout *= 2;
        if !((*outgoing_command).packet).is_null() {
            peer.reliable_data_in_transit -= (*outgoing_command).fragment_length as u32;
            enet_list_insert(
                insert_send_reliable_position,
                enet_list_remove(
                    &mut (*outgoing_command).outgoing_command_list
                        as *mut ENetListNode<ENetOutgoingCommand> as *mut c_void
                        as *mut ENetListNode<ENetListNode<ENetOutgoingCommand>>,
                ),
            );
        } else {
            enet_list_insert(
                insert_position,
                enet_list_remove(
                    &mut (*outgoing_command).outgoing_command_list
                        as *mut ENetListNode<ENetOutgoingCommand> as *mut c_void
                        as *mut ENetListNode<ENetListNode<ENetOutgoingCommand>>,
                ),
            );
        }
        if current_command == enet_list_begin(&mut peer.sent_reliable_commands)
            && !enet_list_empty(&mut peer.sent_reliable_commands)
        {
            outgoing_command = current_command as *mut ENetOutgoingCommand;
            peer.next_timeout =
                (*outgoing_command).sent_time + (*outgoing_command).round_trip_timeout;
        }
    }
    0
}
unsafe fn enet_protocol_check_outgoing_commands<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    sent_unreliable_commands: *mut ENetList<ENetOutgoingCommand>,
) -> bool {
    let mut command: *mut ENetProtocol =
        &mut *(host.commands).as_mut_ptr().add(host.command_count) as *mut ENetProtocol;
    let mut buffer: *mut ENetBuffer =
        &mut *(host.buffers).as_mut_ptr().add(host.buffer_count) as *mut ENetBuffer;
    let mut outgoing_command: *mut ENetOutgoingCommand = std::ptr::null_mut();
    let mut current_command: ENetListIterator<ENetOutgoingCommand>;
    let mut current_send_reliable_command: ENetListIterator<ENetOutgoingCommand>;
    let mut channel: *mut Channel = std::ptr::null_mut();
    let mut reliable_window: u16 = 0;
    let mut command_size: usize;
    let mut window_wrap = false;
    let mut can_ping = true;
    current_command = enet_list_begin(&mut peer.outgoing_commands);
    current_send_reliable_command = enet_list_begin(&mut peer.outgoing_send_reliable_commands);
    let mut next: bool;
    loop {
        if current_command != enet_list_end(&mut peer.outgoing_commands) {
            outgoing_command = current_command as *mut ENetOutgoingCommand;

            if current_send_reliable_command
                != enet_list_end(&mut peer.outgoing_send_reliable_commands)
                && enet_time_less(
                    (*(current_send_reliable_command as *mut ENetOutgoingCommand)).queue_time,
                    (*outgoing_command).queue_time,
                )
            {
                next = true;
            } else {
                current_command = enet_list_next(current_command);
                next = false;
            }
        } else {
            if current_send_reliable_command
                == enet_list_end(&mut peer.outgoing_send_reliable_commands)
            {
                break;
            }
            next = true;
        }

        if next {
            outgoing_command = current_send_reliable_command as *mut ENetOutgoingCommand;
            current_send_reliable_command = enet_list_next(current_send_reliable_command);
        }
        if (*outgoing_command).command.header.command & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE != 0
        {
            channel =
                if ((*outgoing_command).command.header.channel_id as usize) < peer.channel_count {
                    &mut *(peer.channels.as_mut_ptr())
                        .offset((*outgoing_command).command.header.channel_id as isize)
                        as *mut Channel
                } else {
                    std::ptr::null_mut()
                };
            reliable_window = ((*outgoing_command).reliable_sequence_number
                / ENET_PEER_RELIABLE_WINDOW_SIZE) as u16;
            if !channel.is_null() {
                if window_wrap {
                    continue;
                }
                if ((*outgoing_command).send_attempts) < 1
                    && (*outgoing_command).reliable_sequence_number % ENET_PEER_RELIABLE_WINDOW_SIZE
                        == 0
                    && ((*channel).reliable_windows[((reliable_window + ENET_PEER_RELIABLE_WINDOWS
                        - 1)
                        % ENET_PEER_RELIABLE_WINDOWS)
                        as usize]
                        >= ENET_PEER_RELIABLE_WINDOW_SIZE
                        || (*channel).used_reliable_windows
                            & (((1 << (ENET_PEER_FREE_RELIABLE_WINDOWS + 2)) - 1)
                                << reliable_window
                                | ((1 << (ENET_PEER_FREE_RELIABLE_WINDOWS + 2)) - 1)
                                    >> (ENET_PEER_RELIABLE_WINDOWS - reliable_window))
                            != 0)
                {
                    window_wrap = true;
                    current_send_reliable_command =
                        enet_list_end(&mut peer.outgoing_send_reliable_commands);
                    continue;
                }
            }
            if !((*outgoing_command).packet).is_null() {
                let window_size: u32 =
                    (peer.packet_throttle * peer.window_size) / ENET_PEER_PACKET_THROTTLE_SCALE;
                if (peer.reliable_data_in_transit + (*outgoing_command).fragment_length as u32)
                    > (if window_size > peer.mtu {
                        window_size
                    } else {
                        peer.mtu
                    })
                {
                    current_send_reliable_command =
                        enet_list_end(&mut peer.outgoing_send_reliable_commands);
                    continue;
                }
            }
            can_ping = false;
        }
        command_size = COMMAND_SIZES
            [((*outgoing_command).command.header.command & ENET_PROTOCOL_COMMAND_MASK) as usize];
        if command
            >= &mut *(host.commands).as_mut_ptr().add(
                size_of::<[ENetProtocol; ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS]>()
                    / size_of::<ENetProtocol>(),
            ) as *mut ENetProtocol
            || buffer.add(1)
                >= &mut *(host.buffers)
                    .as_mut_ptr()
                    .add(size_of::<[ENetBuffer; 65]>() / size_of::<ENetBuffer>())
                    as *mut ENetBuffer
            || (peer.mtu as usize - host.packet_size) < command_size
            || !((*outgoing_command).packet).is_null()
                && (peer.mtu as usize - host.packet_size)
                    < (command_size + (*outgoing_command).fragment_length as usize)
        {
            peer.flags |= ENET_PEER_FLAG_CONTINUE_SENDING;
            break;
        } else {
            if (*outgoing_command).command.header.command & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE
                != 0
            {
                if !channel.is_null() && (*outgoing_command).send_attempts < 1 {
                    (*channel).used_reliable_windows =
                        ((*channel).used_reliable_windows | 1 << reliable_window) as u16;
                    (*channel).reliable_windows[reliable_window as usize] += 1;
                }
                (*outgoing_command).send_attempts += 1;
                if (*outgoing_command).round_trip_timeout == 0 {
                    (*outgoing_command).round_trip_timeout =
                        peer.round_trip_time + 4 * peer.round_trip_time_variance;
                }
                if enet_list_empty(&mut peer.sent_reliable_commands) {
                    peer.next_timeout = host.service_time + (*outgoing_command).round_trip_timeout;
                }
                enet_list_insert(
                    enet_list_end(&mut peer.sent_reliable_commands),
                    enet_list_remove(
                        &mut (*outgoing_command).outgoing_command_list
                            as *mut ENetListNode<ENetOutgoingCommand>
                            as *mut c_void
                            as *mut ENetListNode<ENetListNode<ENetOutgoingCommand>>,
                    ),
                );
                (*outgoing_command).sent_time = host.service_time;
                host.header_flags |= ENET_PROTOCOL_HEADER_FLAG_SENT_TIME;
                peer.reliable_data_in_transit += (*outgoing_command).fragment_length as u32;
            } else {
                if !((*outgoing_command).packet).is_null()
                    && (*outgoing_command).fragment_offset == 0
                {
                    peer.packet_throttle_counter += ENET_PEER_PACKET_THROTTLE_COUNTER;
                    peer.packet_throttle_counter %= ENET_PEER_PACKET_THROTTLE_SCALE;
                    if peer.packet_throttle_counter > peer.packet_throttle {
                        let reliable_sequence_number: u16 =
                            (*outgoing_command).reliable_sequence_number;
                        let unreliable_sequence_number: u16 =
                            (*outgoing_command).unreliable_sequence_number;
                        loop {
                            (*(*outgoing_command).packet).reference_count -= 1;
                            if (*(*outgoing_command).packet).reference_count == 0 {
                                enet_packet_destroy((*outgoing_command).packet);
                            }
                            enet_list_remove(&mut (*outgoing_command).outgoing_command_list);
                            enet_free(outgoing_command as *mut c_void);
                            if current_command == enet_list_end(&mut peer.outgoing_commands) {
                                break;
                            }
                            outgoing_command = current_command as *mut ENetOutgoingCommand;
                            if (*outgoing_command).reliable_sequence_number
                                != reliable_sequence_number
                                || (*outgoing_command).unreliable_sequence_number
                                    != unreliable_sequence_number
                            {
                                break;
                            }
                            current_command = enet_list_next(current_command);
                        }
                        continue;
                    }
                }
                enet_list_remove(&mut (*outgoing_command).outgoing_command_list);
                if !((*outgoing_command).packet).is_null() {
                    enet_list_insert(
                        enet_list_end(sent_unreliable_commands),
                        outgoing_command as *mut ENetListNode<ENetOutgoingCommand>,
                    );
                }
            }
            (*buffer).data = command as *mut c_void;
            (*buffer).data_length = command_size;
            host.packet_size += (*buffer).data_length;
            *command = (*outgoing_command).command;
            if !((*outgoing_command).packet).is_null() {
                buffer = buffer.offset(1);
                (*buffer).data = ((*(*outgoing_command).packet).data)
                    .offset((*outgoing_command).fragment_offset as isize)
                    as *mut c_void;
                (*buffer).data_length = (*outgoing_command).fragment_length as usize;
                host.packet_size += (*outgoing_command).fragment_length as usize;
            } else if (*outgoing_command).command.header.command
                & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE
                == 0
            {
                enet_free(outgoing_command as *mut c_void);
            }
            peer.packets_sent += 1;
            command = command.offset(1);
            buffer = buffer.offset(1);
        }
    }
    host.command_count = command.offset_from((host.commands).as_mut_ptr()) as usize;
    host.buffer_count = buffer.offset_from((host.buffers).as_mut_ptr()) as usize;
    if peer.state == PeerState::DisconnectLater
        && enet_peer_has_outgoing_commands(peer) == 0
        && enet_list_empty(sent_unreliable_commands)
    {
        enet_peer_disconnect(host, peer, peer.event_data);
    }
    can_ping
}
pub(crate) unsafe fn enet_protocol_send_outgoing_commands<S: Socket>(
    host: &mut Host<S>,
    event: *mut Event,
    check_for_timeouts: bool,
) -> Result<bool, crate::Error> {
    let mut header_data: [u8; 8] = [0; 8];
    let header: *mut ENetProtocolHeader = header_data.as_mut_ptr() as *mut ENetProtocolHeader;
    let mut sent_unreliable_commands: ENetList<ENetOutgoingCommand> = ENetList::default();
    enet_list_clear(&mut sent_unreliable_commands);
    let mut send_pass = 0;
    let mut continue_sending = 0;
    while send_pass <= continue_sending {
        let mut current_peer: *mut Peer<S> = host.peers.as_mut_ptr();
        while current_peer < &mut *(host.peers.as_mut_ptr()).add(host.peer_count) as *mut Peer<S> {
            if !((*current_peer).state == PeerState::Disconnected
                || (*current_peer).state == PeerState::Zombie
                || send_pass > 0 && (*current_peer).flags & ENET_PEER_FLAG_CONTINUE_SENDING == 0)
            {
                (*current_peer).flags &= !(ENET_PEER_FLAG_CONTINUE_SENDING);
                host.header_flags = 0;
                host.command_count = 0;
                host.buffer_count = 1;
                host.packet_size = size_of::<ENetProtocolHeader>();
                if !(*current_peer).acknowledgements.is_empty() {
                    enet_protocol_send_acknowledgements(host, &mut *current_peer);
                }
                if check_for_timeouts
                    && !enet_list_empty(&mut (*current_peer).sent_reliable_commands)
                    && enet_time_greater_equal(host.service_time, (*current_peer).next_timeout)
                    && enet_protocol_check_timeouts(host, &mut *current_peer, event) == 1
                {
                    if !event.is_null() && !matches!(*event, Event::None) {
                        return Ok(true);
                    }
                } else {
                    if (enet_list_empty(&mut (*current_peer).outgoing_commands)
                        && enet_list_empty(&mut (*current_peer).outgoing_send_reliable_commands)
                        || enet_protocol_check_outgoing_commands(
                            host,
                            &mut *current_peer,
                            &mut sent_unreliable_commands,
                        ))
                        && enet_list_empty(&mut (*current_peer).sent_reliable_commands)
                        && enet_time_difference(
                            host.service_time,
                            (*current_peer).last_receive_time,
                        ) >= (*current_peer).ping_interval
                        && (*current_peer).mtu as usize - host.packet_size
                            >= size_of::<ENetProtocolPing>()
                    {
                        enet_peer_ping(host, &mut *current_peer);
                        enet_protocol_check_outgoing_commands(
                            host,
                            &mut *current_peer,
                            &mut sent_unreliable_commands,
                        );
                    }
                    if host.command_count != 0 {
                        if (*current_peer).packet_loss_epoch == 0 {
                            (*current_peer).packet_loss_epoch = host.service_time;
                        } else if enet_time_difference(
                            host.service_time,
                            (*current_peer).packet_loss_epoch,
                        ) >= ENET_PEER_PACKET_LOSS_INTERVAL
                            && (*current_peer).packets_sent > 0
                        {
                            let packet_loss: u32 = ((*current_peer).packets_lost
                                * ENET_PEER_PACKET_LOSS_SCALE)
                                / (*current_peer).packets_sent;
                            (*current_peer).packet_loss_variance =
                                ((*current_peer).packet_loss_variance * 3)
                                    .wrapping_add(if packet_loss < (*current_peer).packet_loss {
                                        ((*current_peer).packet_loss).wrapping_sub(packet_loss)
                                    } else {
                                        packet_loss.wrapping_sub((*current_peer).packet_loss)
                                    })
                                    .wrapping_div(4);
                            (*current_peer).packet_loss =
                                (((*current_peer).packet_loss * 7) + packet_loss) / 8;
                            (*current_peer).packet_loss_epoch = host.service_time;
                            (*current_peer).packets_sent = 0;
                            (*current_peer).packets_lost = 0;
                        }
                        let fresh34 = &mut (*(host.buffers).as_mut_ptr()).data;
                        *fresh34 = header_data.as_mut_ptr() as *mut c_void;
                        if host.header_flags & ENET_PROTOCOL_HEADER_FLAG_SENT_TIME != 0 {
                            (*header).sent_time = ((host.service_time & 0xffff) as u16).to_be();
                            (*(host.buffers).as_mut_ptr()).data_length =
                                size_of::<ENetProtocolHeader>();
                        } else {
                            (*(host.buffers).as_mut_ptr()).data_length = SENT_TIME_OFFSET;
                        }
                        if ((*current_peer).outgoing_peer_id) < ENET_PROTOCOL_MAXIMUM_PEER_ID {
                            host.header_flags |= ((*current_peer).outgoing_session_id as u16)
                                << ENET_PROTOCOL_HEADER_SESSION_SHIFT;
                        }
                        (*header).peer_id =
                            ((*current_peer).outgoing_peer_id | host.header_flags).to_be();
                        (*current_peer).last_send_time = host.service_time;
                        // TODO: remove unwrap
                        let mut conglomerate_buffer = vec![];
                        for buffer_index in 0..host.buffer_count {
                            let buffer = &host.buffers[buffer_index];
                            conglomerate_buffer.extend_from_slice((*buffer).slice());
                        }
                        let send_result = host.socket.send(
                            (*current_peer).address.as_ref().unwrap().clone(),
                            &conglomerate_buffer,
                        );
                        enet_protocol_remove_sent_unreliable_commands(
                            host,
                            &mut *current_peer,
                            &mut sent_unreliable_commands,
                        );
                        match send_result {
                            Ok(sent_length) => {
                                host.total_sent_data += sent_length as u32;
                                host.total_sent_packets += 1;
                            }
                            Err(err) => return Err(crate::Error::FailedToSend(Box::new(err))),
                        }
                    }
                }
                if (*current_peer).flags & ENET_PEER_FLAG_CONTINUE_SENDING != 0 {
                    continue_sending = send_pass + 1;
                }
            }
            current_peer = current_peer.offset(1);
        }
        send_pass += 1;
    }
    Ok(false)
}

pub(crate) fn enet_host_flush<S: Socket>(host: &mut Host<S>) {
    host.service_time = enet_time_get();
    unsafe {
        _ = enet_protocol_send_outgoing_commands(host, std::ptr::null_mut(), false);
    }
}

pub(crate) unsafe fn enet_host_check_events<S: Socket>(
    host: &mut Host<S>,
    event: &mut Event,
) -> Result<bool, crate::Error> {
    *event = Event::None;
    enet_protocol_dispatch_incoming_commands(host, event)
}
