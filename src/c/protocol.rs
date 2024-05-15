use core::{
    alloc::Layout,
    ptr::{copy_nonoverlapping, write_bytes},
};

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
    enet_peer_throttle, enet_time_get, Address, ENetAcknowledgement, ENetBuffer, ENetChannel,
    ENetEvent, ENetHost, ENetIncomingCommand, ENetList, ENetListIterator, ENetListNode,
    ENetOutgoingCommand, ENetPeer, ENetPeerState, PacketReceived, Socket, Vec,
    ENET_EVENT_TYPE_CONNECT, ENET_EVENT_TYPE_DISCONNECT, ENET_EVENT_TYPE_NONE,
    ENET_EVENT_TYPE_RECEIVE, ENET_PACKET_FLAG_RELIABLE, ENET_PACKET_FLAG_SENT,
    ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT, ENET_PACKET_FLAG_UNSEQUENCED,
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
    pub(crate) peer_id: u16,
    pub(crate) sent_time: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolCommandHeader {
    pub(crate) command: u8,
    pub(crate) channel_id: u8,
    pub(crate) reliable_sequence_number: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolAcknowledge {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) received_reliable_sequence_number: u16,
    pub(crate) received_sent_time: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolConnect {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) outgoing_peer_id: u16,
    pub(crate) incoming_session_id: u8,
    pub(crate) outgoing_session_id: u8,
    pub(crate) mtu: u32,
    pub(crate) window_size: u32,
    pub(crate) channel_count: u32,
    pub(crate) incoming_bandwidth: u32,
    pub(crate) outgoing_bandwidth: u32,
    pub(crate) packet_throttle_interval: u32,
    pub(crate) packet_throttle_acceleration: u32,
    pub(crate) packet_throttle_deceleration: u32,
    pub(crate) connect_id: u32,
    pub(crate) data: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolVerifyConnect {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) outgoing_peer_id: u16,
    pub(crate) incoming_session_id: u8,
    pub(crate) outgoing_session_id: u8,
    pub(crate) mtu: u32,
    pub(crate) window_size: u32,
    pub(crate) channel_count: u32,
    pub(crate) incoming_bandwidth: u32,
    pub(crate) outgoing_bandwidth: u32,
    pub(crate) packet_throttle_interval: u32,
    pub(crate) packet_throttle_acceleration: u32,
    pub(crate) packet_throttle_deceleration: u32,
    pub(crate) connect_id: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolBandwidthLimit {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) incoming_bandwidth: u32,
    pub(crate) outgoing_bandwidth: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolThrottleConfigure {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) packet_throttle_interval: u32,
    pub(crate) packet_throttle_acceleration: u32,
    pub(crate) packet_throttle_deceleration: u32,
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
    pub(crate) data_length: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolSendUnreliable {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) unreliable_sequence_number: u16,
    pub(crate) data_length: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolSendUnsequenced {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) unsequenced_group: u16,
    pub(crate) data_length: u16,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct ENetProtocolSendFragment {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) start_sequence_number: u16,
    pub(crate) data_length: u16,
    pub(crate) fragment_count: u32,
    pub(crate) fragment_number: u32,
    pub(crate) total_length: u32,
    pub(crate) fragment_offset: u32,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) union ENetProtocol {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) acknowledge: ENetProtocolAcknowledge,
    pub(crate) connect: ENetProtocolConnect,
    pub(crate) verify_connect: ENetProtocolVerifyConnect,
    pub(crate) disconnect: ENetProtocolDisconnect,
    pub(crate) ping: ENetProtocolPing,
    pub(crate) send_reliable: ENetProtocolSendReliable,
    pub(crate) send_unreliable: ENetProtocolSendUnreliable,
    pub(crate) send_unsequenced: ENetProtocolSendUnsequenced,
    pub(crate) send_fragment: ENetProtocolSendFragment,
    pub(crate) bandwidth_limit: ENetProtocolBandwidthLimit,
    pub(crate) throttle_configure: ENetProtocolThrottleConfigure,
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
pub(crate) unsafe fn enet_protocol_command_size(command_number: u8) -> usize {
    COMMAND_SIZES[(command_number as i32 & ENET_PROTOCOL_COMMAND_MASK as i32) as usize]
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
            &mut (*host).dispatch_queue.sentinel,
            core::ptr::addr_of_mut!((*peer).dispatch_list).cast(),
        );
        (*peer).flags = ((*peer).flags as i32 | ENET_PEER_FLAG_NEEDS_DISPATCH as i32) as u16;
    }
}
unsafe fn enet_protocol_dispatch_incoming_commands<S: Socket>(
    host: *mut ENetHost<S>,
    event: *mut ENetEvent<S>, // SAFETY: should not be null
) -> bool {
    while (*host).dispatch_queue.sentinel.next
        != core::ptr::addr_of_mut!((*host).dispatch_queue.sentinel)
    {
        let peer: *mut ENetPeer<S> = enet_list_remove((*host).dispatch_queue.sentinel.next).cast();
        (*peer).flags = ((*peer).flags as i32 & !(ENET_PEER_FLAG_NEEDS_DISPATCH as i32)) as u16;
        match (*peer).state as u32 {
            3 | 4 => {
                enet_protocol_change_state(host, peer, ENET_PEER_STATE_CONNECTED);
                (*event).type_0 = ENET_EVENT_TYPE_CONNECT;
                (*event).peer = peer;
                (*event).data = (*peer).event_data;
                return true;
            }
            9 => {
                (*host).recalculate_bandwidth_limits = 1_i32;
                (*event).type_0 = ENET_EVENT_TYPE_DISCONNECT;
                (*event).peer = peer;
                (*event).data = (*peer).event_data;
                enet_peer_reset(peer);
                return true;
            }
            5 => {
                if (*peer).dispatched_commands.sentinel.next
                    == core::ptr::addr_of_mut!((*peer).dispatched_commands.sentinel)
                {
                    continue;
                }
                (*event).packet = enet_peer_receive(peer, &mut (*event).channel_id);
                if ((*event).packet).is_null() {
                    continue;
                }
                (*event).type_0 = ENET_EVENT_TYPE_RECEIVE;
                (*event).peer = peer;
                if (*peer).dispatched_commands.sentinel.next
                    != core::ptr::addr_of_mut!((*peer).dispatched_commands.sentinel)
                {
                    (*peer).flags =
                        ((*peer).flags as i32 | ENET_PEER_FLAG_NEEDS_DISPATCH as i32) as u16;
                    enet_list_insert(
                        &mut (*host).dispatch_queue.sentinel,
                        core::ptr::addr_of_mut!((*peer).dispatch_list).cast::<u8>(),
                    );
                }
                return true;
            }
            _ => {}
        }
    }
    false
}
unsafe fn enet_protocol_notify_connect<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    event: *mut ENetEvent<S>,
) {
    (*host).recalculate_bandwidth_limits = 1_i32;
    if !event.is_null() {
        enet_protocol_change_state(host, peer, ENET_PEER_STATE_CONNECTED);
        (*event).type_0 = ENET_EVENT_TYPE_CONNECT;
        (*event).peer = peer;
        (*event).data = (*peer).event_data;
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
        (*host).recalculate_bandwidth_limits = 1_i32;
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
        (*peer).event_data = 0_i32 as u32;
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
    };
}
unsafe fn enet_protocol_remove_sent_unreliable_commands<S: Socket>(
    peer: *mut ENetPeer<S>,
    sent_unreliable_commands: *mut ENetList,
) {
    let mut outgoing_command: *mut ENetOutgoingCommand;
    if (*sent_unreliable_commands).sentinel.next
        == core::ptr::addr_of_mut!((*sent_unreliable_commands).sentinel)
    {
        return;
    }
    loop {
        outgoing_command = (*sent_unreliable_commands)
            .sentinel
            .next
            .cast::<u8>()
            .cast::<ENetOutgoingCommand>();
        enet_list_remove(&mut (*outgoing_command).outgoing_command_list);
        if !((*outgoing_command).packet).is_null() {
            (*(*outgoing_command).packet).reference_count =
                ((*(*outgoing_command).packet).reference_count).wrapping_sub(1);
            if (*(*outgoing_command).packet).reference_count == 0_i32 as usize {
                (*(*outgoing_command).packet).flags |= ENET_PACKET_FLAG_SENT as i32 as u32;
                enet_packet_destroy((*outgoing_command).packet);
            }
        }
        enet_free(
            outgoing_command.cast(),
            Layout::new::<ENetOutgoingCommand>(),
        );
        if (*sent_unreliable_commands).sentinel.next
            == core::ptr::addr_of_mut!((*sent_unreliable_commands).sentinel)
        {
            break;
        }
    }
    if (*peer).state == ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
        && enet_peer_has_outgoing_commands(peer) == 0
    {
        enet_peer_disconnect(peer, (*peer).event_data);
    }
}
unsafe fn enet_protocol_find_sent_reliable_command(
    list: *mut ENetList,
    reliable_sequence_number: u16,
    channel_id: u8,
) -> *mut ENetOutgoingCommand {
    let mut current_command: ENetListIterator;
    current_command = (*list).sentinel.next;
    while current_command != core::ptr::addr_of_mut!((*list).sentinel) {
        let outgoing_command: *mut ENetOutgoingCommand = current_command.cast();
        if (*outgoing_command).command.header.command as i32
            & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32
            != 0
        {
            if ((*outgoing_command).send_attempts as i32) < 1_i32 {
                break;
            }
            if (*outgoing_command).reliable_sequence_number as i32
                == reliable_sequence_number as i32
                && (*outgoing_command).command.header.channel_id as i32 == channel_id as i32
            {
                return outgoing_command;
            }
        }
        current_command = (*current_command).next;
    }
    core::ptr::null_mut()
}
unsafe fn enet_protocol_remove_sent_reliable_command<S: Socket>(
    peer: *mut ENetPeer<S>,
    reliable_sequence_number: u16,
    channel_id: u8,
) -> ENetProtocolCommand {
    let mut outgoing_command: *mut ENetOutgoingCommand = core::ptr::null_mut();
    let mut current_command: ENetListIterator;
    let mut was_sent: i32 = 1_i32;
    current_command = (*peer).sent_reliable_commands.sentinel.next;
    while current_command != core::ptr::addr_of_mut!((*peer).sent_reliable_commands.sentinel) {
        outgoing_command = current_command.cast();
        if (*outgoing_command).reliable_sequence_number as i32 == reliable_sequence_number as i32
            && (*outgoing_command).command.header.channel_id as i32 == channel_id as i32
        {
            break;
        }
        current_command = (*current_command).next;
    }
    if current_command == core::ptr::addr_of_mut!((*peer).sent_reliable_commands.sentinel) {
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
        was_sent = 0_i32;
    }
    if outgoing_command.is_null() {
        return ENET_PROTOCOL_COMMAND_NONE;
    }
    if (channel_id as usize) < (*peer).channel_count {
        let channel: *mut ENetChannel = ((*peer).channels).offset(channel_id as isize);
        let reliable_window: u16 =
            (reliable_sequence_number as i32 / ENET_PEER_RELIABLE_WINDOW_SIZE as i32) as u16;
        if (*channel).reliable_windows[reliable_window as usize] as i32 > 0_i32 {
            (*channel).reliable_windows[reliable_window as usize] =
                ((*channel).reliable_windows[reliable_window as usize]).wrapping_sub(1);
            if (*channel).reliable_windows[reliable_window as usize] == 0 {
                (*channel).used_reliable_windows = ((*channel).used_reliable_windows as i32
                    & !(1_i32 << reliable_window as i32))
                    as u16;
            }
        }
    }
    let command_number = ((*outgoing_command).command.header.command as i32
        & ENET_PROTOCOL_COMMAND_MASK as i32) as ENetProtocolCommand;
    enet_list_remove(&mut (*outgoing_command).outgoing_command_list);
    if !((*outgoing_command).packet).is_null() {
        if was_sent != 0 {
            (*peer).reliable_data_in_transit = (*peer)
                .reliable_data_in_transit
                .wrapping_sub((*outgoing_command).fragment_length as u32);
        }
        (*(*outgoing_command).packet).reference_count =
            ((*(*outgoing_command).packet).reference_count).wrapping_sub(1);
        if (*(*outgoing_command).packet).reference_count == 0_i32 as usize {
            (*(*outgoing_command).packet).flags |= ENET_PACKET_FLAG_SENT as i32 as u32;
            enet_packet_destroy((*outgoing_command).packet);
        }
    }
    enet_free(
        outgoing_command.cast(),
        Layout::new::<ENetOutgoingCommand>(),
    );
    if (*peer).sent_reliable_commands.sentinel.next
        == core::ptr::addr_of_mut!((*peer).sent_reliable_commands.sentinel)
    {
        return command_number;
    }
    outgoing_command = (*peer)
        .sent_reliable_commands
        .sentinel
        .next
        .cast::<u8>()
        .cast::<ENetOutgoingCommand>();
    (*peer).next_timeout =
        ((*outgoing_command).sent_time).wrapping_add((*outgoing_command).round_trip_timeout);
    command_number
}
unsafe fn enet_protocol_handle_connect<S: Socket>(
    host: *mut ENetHost<S>,
    mut _header: *mut ENetProtocolHeader,
    command: *mut ENetProtocol,
) -> *mut ENetPeer<S> {
    let mut incoming_session_id: u8;
    let mut outgoing_session_id: u8;
    let mut mtu: u32;
    let mut window_size: u32;
    let mut channel: *mut ENetChannel;
    let mut channel_count: usize;
    let mut duplicate_peers: usize = 0_i32 as usize;
    let mut current_peer: *mut ENetPeer<S>;
    let mut peer: *mut ENetPeer<S> = core::ptr::null_mut();
    let mut verify_command: ENetProtocol = ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channel_id: 0,
            reliable_sequence_number: 0,
        },
    };
    channel_count = u32::from_be((*command).connect.channel_count) as usize;
    if channel_count < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as i32 as usize
        || channel_count > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as i32 as usize
    {
        return core::ptr::null_mut();
    }
    current_peer = (*host).peers;
    while current_peer < ((*host).peers).add((*host).peer_count) {
        if (*current_peer).state == ENET_PEER_STATE_DISCONNECTED as i32 as u32 {
            if peer.is_null() {
                peer = current_peer;
            }
        } else if (*current_peer).state != ENET_PEER_STATE_CONNECTING as i32 as u32
            && (*current_peer)
                .address
                .assume_init_ref()
                .as_ref()
                .unwrap()
                .same_host((*host).received_address.assume_init_ref().as_ref().unwrap())
        {
            if (*current_peer)
                .address
                .assume_init_ref()
                .as_ref()
                .unwrap()
                .same((*host).received_address.assume_init_ref().as_ref().unwrap())
                && (*current_peer).connect_id == (*command).connect.connect_id
            {
                return core::ptr::null_mut();
            }
            duplicate_peers = duplicate_peers.wrapping_add(1);
        }
        current_peer = current_peer.offset(1);
    }
    if peer.is_null() || duplicate_peers >= (*host).duplicate_peers {
        return core::ptr::null_mut();
    }
    if channel_count > (*host).channel_limit {
        channel_count = (*host).channel_limit;
    }
    (*peer).channels = enet_malloc(Layout::array::<ENetChannel>(channel_count).unwrap()).cast();
    (*peer).channel_count = channel_count;
    (*peer).state = ENET_PEER_STATE_ACKNOWLEDGING_CONNECT;
    (*peer).connect_id = (*command).connect.connect_id;
    *(*peer).address.assume_init_mut() = Some(
        (*host)
            .received_address
            .assume_init_ref()
            .as_ref()
            .cloned()
            .unwrap(),
    );
    (*peer).mtu = (*host).mtu;
    (*peer).outgoing_peer_id = u16::from_be((*command).connect.outgoing_peer_id);
    (*peer).incoming_bandwidth = u32::from_be((*command).connect.incoming_bandwidth);
    (*peer).outgoing_bandwidth = u32::from_be((*command).connect.outgoing_bandwidth);
    (*peer).packet_throttle_interval = u32::from_be((*command).connect.packet_throttle_interval);
    (*peer).packet_throttle_acceleration =
        u32::from_be((*command).connect.packet_throttle_acceleration);
    (*peer).packet_throttle_deceleration =
        u32::from_be((*command).connect.packet_throttle_deceleration);
    (*peer).event_data = u32::from_be((*command).connect.data);
    incoming_session_id = (if (*command).connect.incoming_session_id as i32 == 0xff_i32 {
        (*peer).outgoing_session_id as i32
    } else {
        (*command).connect.incoming_session_id as i32
    }) as u8;
    incoming_session_id = ((incoming_session_id as i32 + 1_i32)
        & ENET_PROTOCOL_HEADER_SESSION_MASK as i32 >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as i32)
        as u8;
    if incoming_session_id as i32 == (*peer).outgoing_session_id as i32 {
        incoming_session_id = ((incoming_session_id as i32 + 1_i32)
            & ENET_PROTOCOL_HEADER_SESSION_MASK as i32 >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as i32)
            as u8;
    }
    (*peer).outgoing_session_id = incoming_session_id;
    outgoing_session_id = (if (*command).connect.outgoing_session_id as i32 == 0xff_i32 {
        (*peer).incoming_session_id as i32
    } else {
        (*command).connect.outgoing_session_id as i32
    }) as u8;
    outgoing_session_id = ((outgoing_session_id as i32 + 1_i32)
        & ENET_PROTOCOL_HEADER_SESSION_MASK as i32 >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as i32)
        as u8;
    if outgoing_session_id as i32 == (*peer).incoming_session_id as i32 {
        outgoing_session_id = ((outgoing_session_id as i32 + 1_i32)
            & ENET_PROTOCOL_HEADER_SESSION_MASK as i32 >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as i32)
            as u8;
    }
    (*peer).incoming_session_id = outgoing_session_id;
    channel = (*peer).channels;
    while channel < ((*peer).channels).add(channel_count) {
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
    mtu = u32::from_be((*command).connect.mtu);
    if mtu < ENET_PROTOCOL_MINIMUM_MTU as i32 as u32 {
        mtu = ENET_PROTOCOL_MINIMUM_MTU as i32 as u32;
    } else if mtu > ENET_PROTOCOL_MAXIMUM_MTU as i32 as u32 {
        mtu = ENET_PROTOCOL_MAXIMUM_MTU as i32 as u32;
    }
    if mtu < (*peer).mtu {
        (*peer).mtu = mtu;
    }
    if (*host).outgoing_bandwidth == 0_i32 as u32 && (*peer).incoming_bandwidth == 0_i32 as u32 {
        (*peer).window_size = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    } else if (*host).outgoing_bandwidth == 0_i32 as u32
        || (*peer).incoming_bandwidth == 0_i32 as u32
    {
        (*peer).window_size = (if (*host).outgoing_bandwidth > (*peer).incoming_bandwidth {
            (*host).outgoing_bandwidth
        } else {
            (*peer).incoming_bandwidth
        })
        .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as i32 as u32)
        .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32);
    } else {
        (*peer).window_size = (if (*host).outgoing_bandwidth < (*peer).incoming_bandwidth {
            (*host).outgoing_bandwidth
        } else {
            (*peer).incoming_bandwidth
        })
        .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as i32 as u32)
        .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32);
    }
    if (*peer).window_size < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32 {
        (*peer).window_size = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32;
    } else if (*peer).window_size > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32 {
        (*peer).window_size = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    }
    if (*host).incoming_bandwidth == 0_i32 as u32 {
        window_size = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    } else {
        window_size = ((*host).incoming_bandwidth)
            .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as i32 as u32)
            .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32);
    }
    if window_size > u32::from_be((*command).connect.window_size) {
        window_size = u32::from_be((*command).connect.window_size);
    }
    if window_size < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32 {
        window_size = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32;
    } else if window_size > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32 {
        window_size = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    }
    verify_command.header.command = (ENET_PROTOCOL_COMMAND_VERIFY_CONNECT as i32
        | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32) as u8;
    verify_command.header.channel_id = 0xff_i32 as u8;
    verify_command.verify_connect.outgoing_peer_id = (*peer).incoming_peer_id.to_be();
    verify_command.verify_connect.incoming_session_id = incoming_session_id;
    verify_command.verify_connect.outgoing_session_id = outgoing_session_id;
    verify_command.verify_connect.mtu = (*peer).mtu.to_be();
    verify_command.verify_connect.window_size = window_size.to_be();
    verify_command.verify_connect.channel_count = (channel_count as u32).to_be();
    verify_command.verify_connect.incoming_bandwidth = (*host).incoming_bandwidth.to_be();
    verify_command.verify_connect.outgoing_bandwidth = (*host).outgoing_bandwidth.to_be();
    verify_command.verify_connect.packet_throttle_interval =
        (*peer).packet_throttle_interval.to_be();
    verify_command.verify_connect.packet_throttle_acceleration =
        (*peer).packet_throttle_acceleration.to_be();
    verify_command.verify_connect.packet_throttle_deceleration =
        (*peer).packet_throttle_deceleration.to_be();
    verify_command.verify_connect.connect_id = (*peer).connect_id;
    enet_peer_queue_outgoing_command(
        peer,
        &verify_command,
        core::ptr::null_mut(),
        0_i32 as u32,
        0_i32 as u16,
    );
    peer
}
unsafe fn enet_protocol_handle_send_reliable<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
    current_data: *mut *mut u8,
) -> i32 {
    if (*command).header.channel_id as usize >= (*peer).channel_count
        || (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
            && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        return -1_i32;
    }
    let data_length = u16::from_be((*command).send_reliable.data_length) as usize;
    *current_data = (*current_data).add(data_length);
    if data_length > (*host).maximum_packet_size
        || *current_data < (*host).received_data
        || *current_data > ((*host).received_data).add((*host).received_data_length)
    {
        return -1_i32;
    }
    if (enet_peer_queue_incoming_command(
        peer,
        command,
        command
            .cast::<u8>()
            .offset(::core::mem::size_of::<ENetProtocolSendReliable>() as u64 as isize),
        data_length,
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
    current_data: *mut *mut u8,
) -> i32 {
    let mut unsequenced_group: u32;
    if (*command).header.channel_id as usize >= (*peer).channel_count
        || (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
            && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        return -1_i32;
    }
    let data_length = u16::from_be((*command).send_unsequenced.data_length) as usize;
    *current_data = (*current_data).add(data_length);
    if data_length > (*host).maximum_packet_size
        || *current_data < (*host).received_data
        || *current_data > ((*host).received_data).add((*host).received_data_length)
    {
        return -1_i32;
    }
    unsequenced_group = u16::from_be((*command).send_unsequenced.unsequenced_group) as u32;
    let index = unsequenced_group.wrapping_rem(ENET_PEER_UNSEQUENCED_WINDOW_SIZE as i32 as u32);
    if unsequenced_group < (*peer).incoming_unsequenced_group as u32 {
        unsequenced_group = unsequenced_group.wrapping_add(0x10000_i32 as u32);
    }
    if unsequenced_group
        >= ((*peer).incoming_unsequenced_group as u32).wrapping_add(
            (ENET_PEER_FREE_UNSEQUENCED_WINDOWS as i32 * ENET_PEER_UNSEQUENCED_WINDOW_SIZE as i32)
                as u32,
        )
    {
        return 0_i32;
    }
    unsequenced_group &= 0xffff_i32 as u32;
    if unsequenced_group.wrapping_sub(index) != (*peer).incoming_unsequenced_group as u32 {
        (*peer).incoming_unsequenced_group = unsequenced_group.wrapping_sub(index) as u16;
        write_bytes((*peer).unsequenced_window.as_mut_ptr(), 0, 32);
    } else if (*peer).unsequenced_window[index.wrapping_div(32_i32 as u32) as usize]
        & (1_i32 << index.wrapping_rem(32_i32 as u32)) as u32
        != 0
    {
        return 0_i32;
    }
    if (enet_peer_queue_incoming_command(
        peer,
        command,
        command
            .cast::<u8>()
            .offset(::core::mem::size_of::<ENetProtocolSendUnsequenced>() as u64 as isize),
        data_length,
        ENET_PACKET_FLAG_UNSEQUENCED as i32 as u32,
        0_i32 as u32,
    ))
    .is_null()
    {
        return -1_i32;
    }
    (*peer).unsequenced_window[index.wrapping_div(32_i32 as u32) as usize] |=
        (1_i32 << index.wrapping_rem(32_i32 as u32)) as u32;
    0_i32
}
unsafe fn enet_protocol_handle_send_unreliable<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
    current_data: *mut *mut u8,
) -> i32 {
    if (*command).header.channel_id as usize >= (*peer).channel_count
        || (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
            && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        return -1_i32;
    }
    let data_length = u16::from_be((*command).send_unreliable.data_length) as usize;
    *current_data = (*current_data).add(data_length);
    if data_length > (*host).maximum_packet_size
        || *current_data < (*host).received_data
        || *current_data > ((*host).received_data).add((*host).received_data_length)
    {
        return -1_i32;
    }
    if (enet_peer_queue_incoming_command(
        peer,
        command,
        command
            .cast::<u8>()
            .offset(::core::mem::size_of::<ENetProtocolSendUnreliable>() as u64 as isize),
        data_length,
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
    current_data: *mut *mut u8,
) -> i32 {
    let mut fragment_length: u32;
    let mut start_window: u16;
    let mut current_command: ENetListIterator;
    let mut start_command: *mut ENetIncomingCommand = core::ptr::null_mut();
    if (*command).header.channel_id as usize >= (*peer).channel_count
        || (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
            && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        return -1_i32;
    }
    fragment_length = u16::from_be((*command).send_fragment.data_length) as u32;
    *current_data = (*current_data).offset(fragment_length as isize);
    if fragment_length <= 0_i32 as u32
        || fragment_length as usize > (*host).maximum_packet_size
        || *current_data < (*host).received_data
        || *current_data > ((*host).received_data).add((*host).received_data_length)
    {
        return -1_i32;
    }
    let channel = ((*peer).channels).offset((*command).header.channel_id as isize);
    let start_sequence_number = u16::from_be((*command).send_fragment.start_sequence_number) as u32;
    start_window =
        start_sequence_number.wrapping_div(ENET_PEER_RELIABLE_WINDOW_SIZE as i32 as u32) as u16;
    let current_window = ((*channel).incoming_reliable_sequence_number as i32
        / ENET_PEER_RELIABLE_WINDOW_SIZE as i32) as u16;
    if start_sequence_number < (*channel).incoming_reliable_sequence_number as u32 {
        start_window = (start_window as i32 + ENET_PEER_RELIABLE_WINDOWS as i32) as u16;
    }
    if (start_window as i32) < current_window as i32
        || start_window as i32
            >= current_window as i32 + ENET_PEER_FREE_RELIABLE_WINDOWS as i32 - 1_i32
    {
        return 0_i32;
    }
    let fragment_number = u32::from_be((*command).send_fragment.fragment_number);
    let fragment_count = u32::from_be((*command).send_fragment.fragment_count);
    let fragment_offset = u32::from_be((*command).send_fragment.fragment_offset);
    let total_length = u32::from_be((*command).send_fragment.total_length);
    if fragment_count > ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as i32 as u32
        || fragment_number >= fragment_count
        || total_length as usize > (*host).maximum_packet_size
        || total_length < fragment_count
        || fragment_offset >= total_length
        || fragment_length > total_length.wrapping_sub(fragment_offset)
    {
        return -1_i32;
    }
    let mut current_block_23: u64;
    current_command = (*channel).incoming_reliable_commands.sentinel.previous;
    while current_command != core::ptr::addr_of_mut!((*channel).incoming_reliable_commands.sentinel)
    {
        let incoming_command: *mut ENetIncomingCommand = current_command.cast();
        if start_sequence_number >= (*channel).incoming_reliable_sequence_number as u32 {
            if ((*incoming_command).reliable_sequence_number as i32)
                < (*channel).incoming_reliable_sequence_number as i32
            {
                current_block_23 = 13056961889198038528;
            } else {
                current_block_23 = 12147880666119273379;
            }
        } else {
            if (*incoming_command).reliable_sequence_number as i32
                >= (*channel).incoming_reliable_sequence_number as i32
            {
                break;
            }
            current_block_23 = 12147880666119273379;
        }
        if let 12147880666119273379 = current_block_23 {
            if (*incoming_command).reliable_sequence_number as u32 <= start_sequence_number {
                if ((*incoming_command).reliable_sequence_number as u32) < start_sequence_number {
                    break;
                }
                if (*incoming_command).command.header.command as i32
                    & ENET_PROTOCOL_COMMAND_MASK as i32
                    != ENET_PROTOCOL_COMMAND_SEND_FRAGMENT as i32
                    || total_length as usize != (*(*incoming_command).packet).data_length
                    || fragment_count != (*incoming_command).fragment_count
                {
                    return -1_i32;
                }
                start_command = incoming_command;
                break;
            }
        }
        current_command = (*current_command).previous;
    }
    if start_command.is_null() {
        let mut host_command: ENetProtocol = *command;
        host_command.header.reliable_sequence_number = start_sequence_number as u16;
        start_command = enet_peer_queue_incoming_command(
            peer,
            &host_command,
            core::ptr::null(),
            total_length as usize,
            ENET_PACKET_FLAG_RELIABLE as i32 as u32,
            fragment_count,
        );
        if start_command.is_null() {
            return -1_i32;
        }
    }
    if *((*start_command).fragments).offset(fragment_number.wrapping_div(32_i32 as u32) as isize)
        & (1_i32 << fragment_number.wrapping_rem(32_i32 as u32)) as u32
        == 0_i32 as u32
    {
        (*start_command).fragments_remaining =
            ((*start_command).fragments_remaining).wrapping_sub(1);
        let fresh32 = ((*start_command).fragments)
            .offset(fragment_number.wrapping_div(32_i32 as u32) as isize);
        *fresh32 |= (1_i32 << fragment_number.wrapping_rem(32_i32 as u32)) as u32;
        if fragment_offset.wrapping_add(fragment_length) as usize
            > (*(*start_command).packet).data_length
        {
            fragment_length = ((*(*start_command).packet).data_length)
                .wrapping_sub(fragment_offset as usize) as u32;
        }
        copy_nonoverlapping(
            (command as *mut u8)
                .offset(::core::mem::size_of::<ENetProtocolSendFragment>() as u64 as isize)
                .cast_const(),
            ((*(*start_command).packet).data).offset(fragment_offset as isize),
            fragment_length as usize,
        );
        if (*start_command).fragments_remaining <= 0_i32 as u32 {
            enet_peer_dispatch_incoming_reliable_commands(peer, channel, core::ptr::null_mut());
        }
    }
    0_i32
}
unsafe fn enet_protocol_handle_send_unreliable_fragment<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
    current_data: *mut *mut u8,
) -> i32 {
    let mut fragment_length: u32;
    let mut reliable_window: u16;
    let mut current_command: ENetListIterator;
    let mut start_command: *mut ENetIncomingCommand = core::ptr::null_mut();
    if (*command).header.channel_id as usize >= (*peer).channel_count
        || (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
            && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        return -1_i32;
    }
    fragment_length = u16::from_be((*command).send_fragment.data_length) as u32;
    *current_data = (*current_data).offset(fragment_length as isize);
    if fragment_length as usize > (*host).maximum_packet_size
        || *current_data < (*host).received_data
        || *current_data > ((*host).received_data).add((*host).received_data_length)
    {
        return -1_i32;
    }
    let channel = ((*peer).channels).offset((*command).header.channel_id as isize);
    let reliable_sequence_number = (*command).header.reliable_sequence_number as u32;
    let start_sequence_number = u16::from_be((*command).send_fragment.start_sequence_number) as u32;
    reliable_window =
        reliable_sequence_number.wrapping_div(ENET_PEER_RELIABLE_WINDOW_SIZE as i32 as u32) as u16;
    let current_window = ((*channel).incoming_reliable_sequence_number as i32
        / ENET_PEER_RELIABLE_WINDOW_SIZE as i32) as u16;
    if reliable_sequence_number < (*channel).incoming_reliable_sequence_number as u32 {
        reliable_window = (reliable_window as i32 + ENET_PEER_RELIABLE_WINDOWS as i32) as u16;
    }
    if (reliable_window as i32) < current_window as i32
        || reliable_window as i32
            >= current_window as i32 + ENET_PEER_FREE_RELIABLE_WINDOWS as i32 - 1_i32
    {
        return 0_i32;
    }
    if reliable_sequence_number == (*channel).incoming_reliable_sequence_number as u32
        && start_sequence_number <= (*channel).incoming_unreliable_sequence_number as u32
    {
        return 0_i32;
    }
    let fragment_number = u32::from_be((*command).send_fragment.fragment_number);
    let fragment_count = u32::from_be((*command).send_fragment.fragment_count);
    let fragment_offset = u32::from_be((*command).send_fragment.fragment_offset);
    let total_length = u32::from_be((*command).send_fragment.total_length);
    if fragment_count > ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as i32 as u32
        || fragment_number >= fragment_count
        || total_length as usize > (*host).maximum_packet_size
        || fragment_offset >= total_length
        || fragment_length > total_length.wrapping_sub(fragment_offset)
    {
        return -1_i32;
    }
    let mut current_block_26: u64;
    current_command = (*channel).incoming_unreliable_commands.sentinel.previous;
    while current_command
        != core::ptr::addr_of_mut!((*channel).incoming_unreliable_commands.sentinel)
    {
        let incoming_command: *mut ENetIncomingCommand = current_command.cast();
        if reliable_sequence_number >= (*channel).incoming_reliable_sequence_number as u32 {
            if ((*incoming_command).reliable_sequence_number as i32)
                < (*channel).incoming_reliable_sequence_number as i32
            {
                current_block_26 = 8457315219000651999;
            } else {
                current_block_26 = 1109700713171191020;
            }
        } else {
            if (*incoming_command).reliable_sequence_number as i32
                >= (*channel).incoming_reliable_sequence_number as i32
            {
                break;
            }
            current_block_26 = 1109700713171191020;
        }
        if let 1109700713171191020 = current_block_26 {
            if ((*incoming_command).reliable_sequence_number as u32) < reliable_sequence_number {
                break;
            }
            if (*incoming_command).reliable_sequence_number as u32 <= reliable_sequence_number
                && (*incoming_command).unreliable_sequence_number as u32 <= start_sequence_number
            {
                if ((*incoming_command).unreliable_sequence_number as u32) < start_sequence_number {
                    break;
                }
                if (*incoming_command).command.header.command as i32
                    & ENET_PROTOCOL_COMMAND_MASK as i32
                    != ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT as i32
                    || total_length as usize != (*(*incoming_command).packet).data_length
                    || fragment_count != (*incoming_command).fragment_count
                {
                    return -1_i32;
                }
                start_command = incoming_command;
                break;
            }
        }
        current_command = (*current_command).previous;
    }
    if start_command.is_null() {
        start_command = enet_peer_queue_incoming_command(
            peer,
            command,
            core::ptr::null(),
            total_length as usize,
            ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT as i32 as u32,
            fragment_count,
        );
        if start_command.is_null() {
            return -1_i32;
        }
    }
    if *((*start_command).fragments).offset(fragment_number.wrapping_div(32_i32 as u32) as isize)
        & (1_i32 << fragment_number.wrapping_rem(32_i32 as u32)) as u32
        == 0_i32 as u32
    {
        (*start_command).fragments_remaining =
            ((*start_command).fragments_remaining).wrapping_sub(1);
        let fresh33 = ((*start_command).fragments)
            .offset(fragment_number.wrapping_div(32_i32 as u32) as isize);
        *fresh33 |= (1_i32 << fragment_number.wrapping_rem(32_i32 as u32)) as u32;
        if fragment_offset.wrapping_add(fragment_length) as usize
            > (*(*start_command).packet).data_length
        {
            fragment_length = ((*(*start_command).packet).data_length)
                .wrapping_sub(fragment_offset as usize) as u32;
        }
        copy_nonoverlapping(
            (command as *mut u8)
                .offset(::core::mem::size_of::<ENetProtocolSendFragment>() as u64 as isize)
                .cast_const(),
            ((*(*start_command).packet).data).offset(fragment_offset as isize),
            fragment_length as usize,
        );
        if (*start_command).fragments_remaining <= 0_i32 as u32 {
            enet_peer_dispatch_incoming_unreliable_commands(peer, channel, core::ptr::null_mut());
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
    if (*peer).incoming_bandwidth != 0_i32 as u32 {
        (*host).bandwidth_limited_peers = ((*host).bandwidth_limited_peers).wrapping_sub(1);
    }
    (*peer).incoming_bandwidth = u32::from_be((*command).bandwidth_limit.incoming_bandwidth);
    (*peer).outgoing_bandwidth = u32::from_be((*command).bandwidth_limit.outgoing_bandwidth);
    if (*peer).incoming_bandwidth != 0_i32 as u32 {
        (*host).bandwidth_limited_peers = ((*host).bandwidth_limited_peers).wrapping_add(1);
    }
    if (*peer).incoming_bandwidth == 0_i32 as u32 && (*host).outgoing_bandwidth == 0_i32 as u32 {
        (*peer).window_size = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    } else if (*peer).incoming_bandwidth == 0_i32 as u32
        || (*host).outgoing_bandwidth == 0_i32 as u32
    {
        (*peer).window_size = (if (*peer).incoming_bandwidth > (*host).outgoing_bandwidth {
            (*peer).incoming_bandwidth
        } else {
            (*host).outgoing_bandwidth
        })
        .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as i32 as u32)
        .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32);
    } else {
        (*peer).window_size = (if (*peer).incoming_bandwidth < (*host).outgoing_bandwidth {
            (*peer).incoming_bandwidth
        } else {
            (*host).outgoing_bandwidth
        })
        .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as i32 as u32)
        .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32);
    }
    if (*peer).window_size < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32 {
        (*peer).window_size = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32;
    } else if (*peer).window_size > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32 {
        (*peer).window_size = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
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
    (*peer).packet_throttle_interval =
        u32::from_be((*command).throttle_configure.packet_throttle_interval);
    (*peer).packet_throttle_acceleration =
        u32::from_be((*command).throttle_configure.packet_throttle_acceleration);
    (*peer).packet_throttle_deceleration =
        u32::from_be((*command).throttle_configure.packet_throttle_deceleration);
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
            (*host).recalculate_bandwidth_limits = 1_i32;
        }
        enet_peer_reset(peer);
    } else if (*command).header.command as i32 & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32 != 0
    {
        enet_protocol_change_state(host, peer, ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT);
    } else {
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
    }
    if (*peer).state != ENET_PEER_STATE_DISCONNECTED as i32 as u32 {
        (*peer).event_data = u32::from_be((*command).disconnect.data);
    }
    0_i32
}
unsafe fn enet_protocol_handle_acknowledge<S: Socket>(
    host: *mut ENetHost<S>,
    event: *mut ENetEvent<S>,
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
) -> i32 {
    let mut round_trip_time: u32;
    let mut received_sent_time: u32;
    if (*peer).state == ENET_PEER_STATE_DISCONNECTED as i32 as u32
        || (*peer).state == ENET_PEER_STATE_ZOMBIE as i32 as u32
    {
        return 0_i32;
    }
    received_sent_time = u16::from_be((*command).acknowledge.received_sent_time) as u32;
    received_sent_time |= (*host).service_time & 0xffff0000_u32;
    if received_sent_time & 0x8000_i32 as u32 > (*host).service_time & 0x8000_i32 as u32 {
        received_sent_time = received_sent_time.wrapping_sub(0x10000_i32 as u32);
    }
    if ((*host).service_time).wrapping_sub(received_sent_time) >= 86400000_i32 as u32 {
        return 0_i32;
    }
    round_trip_time =
        if ((*host).service_time).wrapping_sub(received_sent_time) >= 86400000_i32 as u32 {
            received_sent_time.wrapping_sub((*host).service_time)
        } else {
            ((*host).service_time).wrapping_sub(received_sent_time)
        };
    round_trip_time = if round_trip_time > 1_i32 as u32 {
        round_trip_time
    } else {
        1_i32 as u32
    };
    if (*peer).last_receive_time > 0_i32 as u32 {
        enet_peer_throttle(peer, round_trip_time);
        (*peer).round_trip_time_variance = (*peer)
            .round_trip_time_variance
            .wrapping_sub(((*peer).round_trip_time_variance).wrapping_div(4_i32 as u32));
        if round_trip_time >= (*peer).round_trip_time {
            let diff: u32 = round_trip_time.wrapping_sub((*peer).round_trip_time);
            (*peer).round_trip_time_variance = (*peer)
                .round_trip_time_variance
                .wrapping_add(diff.wrapping_div(4_i32 as u32));
            (*peer).round_trip_time = (*peer)
                .round_trip_time
                .wrapping_add(diff.wrapping_div(8_i32 as u32));
        } else {
            let diff_0: u32 = ((*peer).round_trip_time).wrapping_sub(round_trip_time);
            (*peer).round_trip_time_variance = (*peer)
                .round_trip_time_variance
                .wrapping_add(diff_0.wrapping_div(4_i32 as u32));
            (*peer).round_trip_time = (*peer)
                .round_trip_time
                .wrapping_sub(diff_0.wrapping_div(8_i32 as u32));
        }
    } else {
        (*peer).round_trip_time = round_trip_time;
        (*peer).round_trip_time_variance = round_trip_time
            .wrapping_add(1_i32 as u32)
            .wrapping_div(2_i32 as u32);
    }
    if (*peer).round_trip_time < (*peer).lowest_round_trip_time {
        (*peer).lowest_round_trip_time = (*peer).round_trip_time;
    }
    if (*peer).round_trip_time_variance > (*peer).highest_round_trip_time_variance {
        (*peer).highest_round_trip_time_variance = (*peer).round_trip_time_variance;
    }
    if (*peer).packet_throttle_epoch == 0_i32 as u32
        || (if ((*host).service_time).wrapping_sub((*peer).packet_throttle_epoch)
            >= 86400000_i32 as u32
        {
            ((*peer).packet_throttle_epoch).wrapping_sub((*host).service_time)
        } else {
            ((*host).service_time).wrapping_sub((*peer).packet_throttle_epoch)
        }) >= (*peer).packet_throttle_interval
    {
        (*peer).last_round_trip_time = (*peer).lowest_round_trip_time;
        (*peer).last_round_trip_time_variance =
            if (*peer).highest_round_trip_time_variance > 1_i32 as u32 {
                (*peer).highest_round_trip_time_variance
            } else {
                1_i32 as u32
            };
        (*peer).lowest_round_trip_time = (*peer).round_trip_time;
        (*peer).highest_round_trip_time_variance = (*peer).round_trip_time_variance;
        (*peer).packet_throttle_epoch = (*host).service_time;
    }
    (*peer).last_receive_time = if (*host).service_time > 1_i32 as u32 {
        (*host).service_time
    } else {
        1_i32 as u32
    };
    (*peer).earliest_timeout = 0_i32 as u32;
    let received_reliable_sequence_number =
        u16::from_be((*command).acknowledge.received_reliable_sequence_number) as u32;
    let command_number = enet_protocol_remove_sent_reliable_command(
        peer,
        received_reliable_sequence_number as u16,
        (*command).header.channel_id,
    );
    match (*peer).state {
        2 => {
            if command_number as u32 != ENET_PROTOCOL_COMMAND_VERIFY_CONNECT as i32 as u32 {
                return -1_i32;
            }
            enet_protocol_notify_connect(host, peer, event);
        }
        7 => {
            if command_number as u32 != ENET_PROTOCOL_COMMAND_DISCONNECT as i32 as u32 {
                return -1_i32;
            }
            enet_protocol_notify_disconnect(host, peer, event);
        }
        6 => {
            if enet_peer_has_outgoing_commands(peer) == 0 {
                enet_peer_disconnect(peer, (*peer).event_data);
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
    let mut window_size: u32;
    if (*peer).state != ENET_PEER_STATE_CONNECTING as i32 as u32 {
        return 0_i32;
    }
    let channel_count = u32::from_be((*command).verify_connect.channel_count) as usize;
    if channel_count < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as i32 as usize
        || channel_count > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as i32 as usize
        || u32::from_be((*command).verify_connect.packet_throttle_interval)
            != (*peer).packet_throttle_interval
        || u32::from_be((*command).verify_connect.packet_throttle_acceleration)
            != (*peer).packet_throttle_acceleration
        || u32::from_be((*command).verify_connect.packet_throttle_deceleration)
            != (*peer).packet_throttle_deceleration
        || (*command).verify_connect.connect_id != (*peer).connect_id
    {
        (*peer).event_data = 0_i32 as u32;
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
        return -1_i32;
    }
    enet_protocol_remove_sent_reliable_command(peer, 1_i32 as u16, 0xff_i32 as u8);
    if channel_count < (*peer).channel_count {
        (*peer).channel_count = channel_count;
    }
    (*peer).outgoing_peer_id = u16::from_be((*command).verify_connect.outgoing_peer_id);
    (*peer).incoming_session_id = (*command).verify_connect.incoming_session_id;
    (*peer).outgoing_session_id = (*command).verify_connect.outgoing_session_id;
    mtu = u32::from_be((*command).verify_connect.mtu);
    if mtu < ENET_PROTOCOL_MINIMUM_MTU as i32 as u32 {
        mtu = ENET_PROTOCOL_MINIMUM_MTU as i32 as u32;
    } else if mtu > ENET_PROTOCOL_MAXIMUM_MTU as i32 as u32 {
        mtu = ENET_PROTOCOL_MAXIMUM_MTU as i32 as u32;
    }
    if mtu < (*peer).mtu {
        (*peer).mtu = mtu;
    }
    window_size = u32::from_be((*command).verify_connect.window_size);
    if window_size < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32 {
        window_size = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as i32 as u32;
    }
    if window_size > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32 {
        window_size = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    }
    if window_size < (*peer).window_size {
        (*peer).window_size = window_size;
    }
    (*peer).incoming_bandwidth = u32::from_be((*command).verify_connect.incoming_bandwidth);
    (*peer).outgoing_bandwidth = u32::from_be((*command).verify_connect.outgoing_bandwidth);
    enet_protocol_notify_connect(host, peer, event);
    0_i32
}
unsafe fn enet_protocol_handle_incoming_commands<S: Socket>(
    host: *mut ENetHost<S>,
    event: *mut ENetEvent<S>,
) -> bool {
    let mut command: *mut ENetProtocol;
    let mut peer: *mut ENetPeer<S>;
    let mut current_data: *mut u8;
    let mut header_size: usize;
    let mut peer_id: u16;
    if (*host).received_data_length < 2_usize {
        return false;
    }
    let header: *mut ENetProtocolHeader = (*host).received_data.cast();
    peer_id = u16::from_be((*header).peer_id);
    let session_id = ((peer_id as i32 & ENET_PROTOCOL_HEADER_SESSION_MASK as i32)
        >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as i32) as u8;
    let flags = (peer_id as i32 & ENET_PROTOCOL_HEADER_FLAG_MASK as i32) as u16;
    peer_id = (peer_id as i32
        & !(ENET_PROTOCOL_HEADER_FLAG_MASK as i32 | ENET_PROTOCOL_HEADER_SESSION_MASK as i32))
        as u16;
    header_size = if flags as i32 & ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as i32 != 0 {
        ::core::mem::size_of::<ENetProtocolHeader>()
    } else {
        2_usize
    };
    if ((*host).checksum.assume_init_ref()).is_some() {
        header_size =
            (header_size as u64).wrapping_add(::core::mem::size_of::<u32>() as u64) as usize;
    }
    if peer_id as i32 == ENET_PROTOCOL_MAXIMUM_PEER_ID as i32 {
        peer = core::ptr::null_mut();
    } else if peer_id as usize >= (*host).peer_count {
        return false;
    } else {
        peer = ((*host).peers).offset(peer_id as isize);
        if (*peer).state == ENET_PEER_STATE_DISCONNECTED as i32 as u32
            || (*peer).state == ENET_PEER_STATE_ZOMBIE as i32 as u32
            || !(*host)
                .received_address
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
            || ((*peer).outgoing_peer_id as i32) < ENET_PROTOCOL_MAXIMUM_PEER_ID as i32
                && session_id as i32 != (*peer).incoming_session_id as i32
        {
            return false;
        }
    }
    if flags as i32 & ENET_PROTOCOL_HEADER_FLAG_COMPRESSED as i32 != 0 {
        let Some(compressor) = (*host).compressor.assume_init_mut() else {
            return false;
        };
        let in_data = super::from_raw_parts_or_empty(
            ((*host).received_data).add(header_size),
            ((*host).received_data_length).wrapping_sub(header_size),
        );
        let out = super::from_raw_parts_or_empty_mut(
            ((*host).packet_data[1_i32 as usize])
                .as_mut_ptr()
                .add(header_size),
            ::core::mem::size_of::<[u8; 4096]>().wrapping_sub(header_size),
        );
        let original_size = compressor.decompress(in_data, out);
        if original_size <= 0_i32 as usize
            || original_size > ::core::mem::size_of::<[u8; 4096]>().wrapping_sub(header_size)
        {
            return false;
        }
        copy_nonoverlapping(
            header as *const u8,
            ((*host).packet_data[1_i32 as usize]).as_mut_ptr(),
            header_size,
        );
        (*host).received_data = ((*host).packet_data[1_i32 as usize]).as_mut_ptr();
        (*host).received_data_length = header_size.wrapping_add(original_size);
    }
    if let Some(checksum_fn) = (*host).checksum.assume_init_ref() {
        let checksum_addr: *mut u8 =
            ((*host).received_data).add(header_size.wrapping_sub(::core::mem::size_of::<u32>()));
        let mut desired_checksum: u32 = 0;
        copy_nonoverlapping(
            checksum_addr.cast_const(),
            core::ptr::addr_of_mut!(desired_checksum).cast(),
            ::core::mem::size_of::<u32>(),
        );
        let mut buffer: ENetBuffer = ENetBuffer {
            data: core::ptr::null_mut(),
            data_length: 0,
        };
        let checksum = if !peer.is_null() {
            (*peer).connect_id
        } else {
            0_i32 as u32
        };
        copy_nonoverlapping(
            core::ptr::addr_of!(checksum).cast(),
            checksum_addr,
            ::core::mem::size_of::<u32>(),
        );
        buffer.data = (*host).received_data;
        buffer.data_length = (*host).received_data_length;
        let in_buffers = [super::from_raw_parts_or_empty(
            buffer.data,
            buffer.data_length,
        )];
        if checksum_fn(&in_buffers) != desired_checksum {
            return false;
        }
    }
    if !peer.is_null() {
        *(*peer).address.assume_init_mut() = Some(
            (*host)
                .received_address
                .assume_init_ref()
                .as_ref()
                .cloned()
                .unwrap(),
        );
        (*peer).incoming_data_total = ((*peer).incoming_data_total as usize)
            .wrapping_add((*host).received_data_length)
            as u32;
    }
    current_data = ((*host).received_data).add(header_size);
    while current_data < ((*host).received_data).add((*host).received_data_length) {
        command = current_data.cast();
        if current_data.offset(::core::mem::size_of::<ENetProtocolCommandHeader>() as u64 as isize)
            > ((*host).received_data).add((*host).received_data_length)
        {
            break;
        }
        let command_number =
            ((*command).header.command as i32 & ENET_PROTOCOL_COMMAND_MASK as i32) as u8;
        if command_number as i32 >= ENET_PROTOCOL_COMMAND_COUNT as i32 {
            break;
        }
        let command_size = COMMAND_SIZES[command_number as usize];
        if command_size == 0_i32 as usize
            || current_data.add(command_size)
                > ((*host).received_data).add((*host).received_data_length)
        {
            break;
        }
        current_data = current_data.add(command_size);
        if peer.is_null() && command_number as i32 != ENET_PROTOCOL_COMMAND_CONNECT as i32 {
            break;
        }
        (*command).header.reliable_sequence_number =
            u16::from_be((*command).header.reliable_sequence_number);
        match command_number as i32 {
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
                if enet_protocol_handle_send_reliable(host, peer, command, &mut current_data) != 0 {
                    break;
                }
            }
            7 => {
                if enet_protocol_handle_send_unreliable(host, peer, command, &mut current_data) != 0
                {
                    break;
                }
            }
            9 => {
                if enet_protocol_handle_send_unsequenced(host, peer, command, &mut current_data)
                    != 0
                {
                    break;
                }
            }
            8 => {
                if enet_protocol_handle_send_fragment(host, peer, command, &mut current_data) != 0 {
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
        if peer.is_null()
            || (*command).header.command as i32 & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32
                == 0_i32
        {
            continue;
        }
        if flags as i32 & ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as i32 == 0 {
            break;
        }
        let sent_time = u16::from_be((*header).sent_time);
        match (*peer).state {
            7 | 2 | 0 | 9 => {}
            8 => {
                if (*command).header.command as i32 & ENET_PROTOCOL_COMMAND_MASK as i32
                    == ENET_PROTOCOL_COMMAND_DISCONNECT as i32
                {
                    enet_peer_queue_acknowledgement(peer, command, sent_time);
                }
            }
            _ => {
                enet_peer_queue_acknowledgement(peer, command, sent_time);
            }
        }
    }
    if !event.is_null() && (*event).type_0 != ENET_EVENT_TYPE_NONE as i32 as u32 {
        return true;
    }
    false
}
unsafe fn enet_protocol_receive_incoming_commands<S: Socket>(
    host: *mut ENetHost<S>,
    event: *mut ENetEvent<S>,
) -> Result<bool, S::Error> {
    let mut packets: i32;
    packets = 0_i32;
    while packets < 256_i32 {
        let mut buffer: ENetBuffer = ENetBuffer {
            data: core::ptr::null_mut(),
            data_length: 0,
        };
        buffer.data = ((*host).packet_data[0_i32 as usize]).as_mut_ptr();
        buffer.data_length = ::core::mem::size_of::<[u8; ENET_PROTOCOL_MAXIMUM_MTU as usize]>();
        let received_length = match (*host).socket.assume_init_mut().receive(buffer.data_length) {
            Ok(Some((received_address, PacketReceived::Complete(received_data)))) => {
                if received_data.len() <= ENET_PROTOCOL_MAXIMUM_MTU as usize {
                    *(*host).received_address.assume_init_mut() = Some(received_address);
                    copy_nonoverlapping(received_data.as_ptr(), buffer.data, received_data.len());
                    received_data.len() as i32
                } else {
                    continue;
                }
            }
            Ok(Some((_, PacketReceived::Partial))) => {
                continue;
            }
            Ok(None) => {
                return Ok(false);
            }
            Err(err) => {
                return Err(err);
            }
        };
        (*host).received_data = ((*host).packet_data[0_i32 as usize]).as_mut_ptr();
        (*host).received_data_length = received_length as usize;
        (*host).total_received_data = (*host)
            .total_received_data
            .wrapping_add(received_length as u32);
        (*host).total_received_packets = ((*host).total_received_packets).wrapping_add(1);
        if enet_protocol_handle_incoming_commands(host, event) {
            return Ok(true);
        }
        packets += 1;
    }
    Ok(false)
}
unsafe fn enet_protocol_send_acknowledgements<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
) {
    let mut command: *mut ENetProtocol = ((*host).commands).as_mut_ptr().add((*host).command_count);
    let mut buffer: *mut ENetBuffer = ((*host).buffers).as_mut_ptr().add((*host).buffer_count);
    let mut acknowledgement: *mut ENetAcknowledgement;
    let mut current_acknowledgement: ENetListIterator;
    let mut reliable_sequence_number: u16;
    current_acknowledgement = (*peer).acknowledgements.sentinel.next;
    while current_acknowledgement != core::ptr::addr_of_mut!((*peer).acknowledgements.sentinel) {
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
            || ((*peer).mtu as usize).wrapping_sub((*host).packet_size)
                < ::core::mem::size_of::<ENetProtocolAcknowledge>()
        {
            (*peer).flags = ((*peer).flags as i32 | ENET_PEER_FLAG_CONTINUE_SENDING as i32) as u16;
            break;
        } else {
            acknowledgement = current_acknowledgement.cast();
            current_acknowledgement = (*current_acknowledgement).next;
            (*buffer).data = command.cast();
            (*buffer).data_length = ::core::mem::size_of::<ENetProtocolAcknowledge>();
            (*host).packet_size = (*host).packet_size.wrapping_add((*buffer).data_length);
            reliable_sequence_number = (*acknowledgement)
                .command
                .header
                .reliable_sequence_number
                .to_be();
            (*command).header.command = ENET_PROTOCOL_COMMAND_ACKNOWLEDGE as i32 as u8;
            (*command).header.channel_id = (*acknowledgement).command.header.channel_id;
            (*command).header.reliable_sequence_number = reliable_sequence_number;
            (*command).acknowledge.received_reliable_sequence_number = reliable_sequence_number;
            (*command).acknowledge.received_sent_time =
                ((*acknowledgement).sent_time as u16).to_be();
            if (*acknowledgement).command.header.command as i32 & ENET_PROTOCOL_COMMAND_MASK as i32
                == ENET_PROTOCOL_COMMAND_DISCONNECT as i32
            {
                enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
            }
            enet_list_remove(&mut (*acknowledgement).acknowledgement_list);
            enet_free(acknowledgement.cast(), Layout::new::<ENetAcknowledgement>());
            command = command.offset(1);
            buffer = buffer.offset(1);
        }
    }
    (*host).command_count = command.offset_from(((*host).commands).as_mut_ptr()) as i64 as usize;
    (*host).buffer_count = buffer.offset_from(((*host).buffers).as_mut_ptr()) as i64 as usize;
}
unsafe fn enet_protocol_check_timeouts<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    event: *mut ENetEvent<S>,
) -> i32 {
    let mut outgoing_command: *mut ENetOutgoingCommand;
    let mut current_command: ENetListIterator;
    current_command = (*peer).sent_reliable_commands.sentinel.next;
    let insert_position = (*peer).outgoing_commands.sentinel.next;
    let insert_send_reliable_position = (*peer).outgoing_send_reliable_commands.sentinel.next;
    while current_command != core::ptr::addr_of_mut!((*peer).sent_reliable_commands.sentinel) {
        outgoing_command = current_command.cast();
        current_command = (*current_command).next;
        if (if ((*host).service_time).wrapping_sub((*outgoing_command).sent_time)
            >= 86400000_i32 as u32
        {
            ((*outgoing_command).sent_time).wrapping_sub((*host).service_time)
        } else {
            ((*host).service_time).wrapping_sub((*outgoing_command).sent_time)
        }) < (*outgoing_command).round_trip_timeout
        {
            continue;
        }
        if (*peer).earliest_timeout == 0_i32 as u32
            || ((*outgoing_command).sent_time).wrapping_sub((*peer).earliest_timeout)
                >= 86400000_i32 as u32
        {
            (*peer).earliest_timeout = (*outgoing_command).sent_time;
        }
        if (*peer).earliest_timeout != 0_i32 as u32
            && ((if ((*host).service_time).wrapping_sub((*peer).earliest_timeout)
                >= 86400000_i32 as u32
            {
                ((*peer).earliest_timeout).wrapping_sub((*host).service_time)
            } else {
                ((*host).service_time).wrapping_sub((*peer).earliest_timeout)
            }) >= (*peer).timeout_maximum
                || (1_i32 << ((*outgoing_command).send_attempts as i32 - 1_i32)) as u32
                    >= (*peer).timeout_limit
                    && (if ((*host).service_time).wrapping_sub((*peer).earliest_timeout)
                        >= 86400000_i32 as u32
                    {
                        ((*peer).earliest_timeout).wrapping_sub((*host).service_time)
                    } else {
                        ((*host).service_time).wrapping_sub((*peer).earliest_timeout)
                    }) >= (*peer).timeout_minimum)
        {
            enet_protocol_notify_disconnect(host, peer, event);
            return 1_i32;
        }
        (*peer).packets_lost = ((*peer).packets_lost).wrapping_add(1);
        (*outgoing_command).round_trip_timeout = (*outgoing_command)
            .round_trip_timeout
            .wrapping_mul(2_i32 as u32);
        if !((*outgoing_command).packet).is_null() {
            (*peer).reliable_data_in_transit = (*peer)
                .reliable_data_in_transit
                .wrapping_sub((*outgoing_command).fragment_length as u32);
            enet_list_insert(
                insert_send_reliable_position,
                enet_list_remove(&mut (*outgoing_command).outgoing_command_list),
            );
        } else {
            enet_list_insert(
                insert_position,
                enet_list_remove(&mut (*outgoing_command).outgoing_command_list),
            );
        }
        if current_command == (*peer).sent_reliable_commands.sentinel.next
            && ((*peer).sent_reliable_commands.sentinel.next
                != core::ptr::addr_of_mut!((*peer).sent_reliable_commands.sentinel))
        {
            outgoing_command = current_command.cast();
            (*peer).next_timeout = ((*outgoing_command).sent_time)
                .wrapping_add((*outgoing_command).round_trip_timeout);
        }
    }
    0_i32
}
unsafe fn enet_protocol_check_outgoing_commands<S: Socket>(
    host: *mut ENetHost<S>,
    peer: *mut ENetPeer<S>,
    sent_unreliable_commands: *mut ENetList,
) -> i32 {
    let mut command: *mut ENetProtocol = ((*host).commands).as_mut_ptr().add((*host).command_count);
    let mut buffer: *mut ENetBuffer = ((*host).buffers).as_mut_ptr().add((*host).buffer_count);
    let mut outgoing_command: *mut ENetOutgoingCommand = core::ptr::null_mut();
    let mut current_command: ENetListIterator;
    let mut current_send_reliable_command: ENetListIterator;
    let mut channel: *mut ENetChannel = core::ptr::null_mut();
    let mut reliable_window: u16 = 0_i32 as u16;
    let mut command_size: usize;
    let mut window_wrap: i32 = 0_i32;
    let mut can_ping: i32 = 1_i32;
    current_command = (*peer).outgoing_commands.sentinel.next;
    current_send_reliable_command = (*peer).outgoing_send_reliable_commands.sentinel.next;
    let mut current_block_55: u64;
    loop {
        if current_command != core::ptr::addr_of_mut!((*peer).outgoing_commands.sentinel) {
            outgoing_command = current_command.cast();
            if current_send_reliable_command
                != core::ptr::addr_of_mut!((*peer).outgoing_send_reliable_commands.sentinel)
                && ((*(current_send_reliable_command.cast::<ENetOutgoingCommand>())).queue_time)
                    .wrapping_sub((*outgoing_command).queue_time)
                    >= 86400000_i32 as u32
            {
                current_block_55 = 13678975718891345113;
            } else {
                current_command = (*current_command).next;
                current_block_55 = 1856101646708284338;
            }
        } else {
            if current_send_reliable_command
                == core::ptr::addr_of_mut!((*peer).outgoing_send_reliable_commands.sentinel)
            {
                break;
            }
            current_block_55 = 13678975718891345113;
        }
        if let 13678975718891345113 = current_block_55 {
            outgoing_command = current_send_reliable_command.cast();
            current_send_reliable_command = (*current_send_reliable_command).next;
        }
        if (*outgoing_command).command.header.command as i32
            & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32
            != 0
        {
            channel = if ((*outgoing_command).command.header.channel_id as usize)
                < (*peer).channel_count
            {
                ((*peer).channels).offset((*outgoing_command).command.header.channel_id as isize)
            } else {
                core::ptr::null_mut()
            };
            reliable_window = ((*outgoing_command).reliable_sequence_number as i32
                / ENET_PEER_RELIABLE_WINDOW_SIZE as i32) as u16;
            if !channel.is_null() {
                if window_wrap != 0 {
                    continue;
                }
                if ((*outgoing_command).send_attempts as i32) < 1_i32
                    && (*outgoing_command).reliable_sequence_number as i32
                        % ENET_PEER_RELIABLE_WINDOW_SIZE as i32
                        == 0
                    && ((*channel).reliable_windows[((reliable_window as i32
                        + ENET_PEER_RELIABLE_WINDOWS as i32
                        - 1_i32)
                        % ENET_PEER_RELIABLE_WINDOWS as i32)
                        as usize] as i32
                        >= ENET_PEER_RELIABLE_WINDOW_SIZE as i32
                        || (*channel).used_reliable_windows as i32
                            & (((1_i32 << (ENET_PEER_FREE_RELIABLE_WINDOWS as i32 + 2_i32))
                                - 1_i32)
                                << reliable_window as i32
                                | ((1_i32 << (ENET_PEER_FREE_RELIABLE_WINDOWS as i32 + 2_i32))
                                    - 1_i32)
                                    >> (ENET_PEER_RELIABLE_WINDOWS as i32
                                        - reliable_window as i32))
                            != 0)
                {
                    window_wrap = 1_i32;
                    current_send_reliable_command =
                        &mut (*peer).outgoing_send_reliable_commands.sentinel;
                    continue;
                }
            }
            if !((*outgoing_command).packet).is_null() {
                let window_size: u32 = ((*peer).packet_throttle)
                    .wrapping_mul((*peer).window_size)
                    .wrapping_div(ENET_PEER_PACKET_THROTTLE_SCALE as i32 as u32);
                if ((*peer).reliable_data_in_transit)
                    .wrapping_add((*outgoing_command).fragment_length as u32)
                    > (if window_size > (*peer).mtu {
                        window_size
                    } else {
                        (*peer).mtu
                    })
                {
                    current_send_reliable_command =
                        &mut (*peer).outgoing_send_reliable_commands.sentinel;
                    continue;
                }
            }
            can_ping = 0_i32;
        }
        command_size = COMMAND_SIZES[((*outgoing_command).command.header.command as i32
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
            || ((*peer).mtu as usize).wrapping_sub((*host).packet_size) < command_size
            || !((*outgoing_command).packet).is_null()
                && (((*peer).mtu as usize).wrapping_sub((*host).packet_size) as u16 as i32)
                    < command_size.wrapping_add((*outgoing_command).fragment_length as usize) as u16
                        as i32
        {
            (*peer).flags = ((*peer).flags as i32 | ENET_PEER_FLAG_CONTINUE_SENDING as i32) as u16;
            break;
        } else {
            if (*outgoing_command).command.header.command as i32
                & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32
                != 0
            {
                if !channel.is_null() && ((*outgoing_command).send_attempts as i32) < 1_i32 {
                    (*channel).used_reliable_windows = ((*channel).used_reliable_windows as i32
                        | 1_i32 << reliable_window as i32)
                        as u16;
                    (*channel).reliable_windows[reliable_window as usize] =
                        ((*channel).reliable_windows[reliable_window as usize]).wrapping_add(1);
                }
                (*outgoing_command).send_attempts =
                    ((*outgoing_command).send_attempts).wrapping_add(1);
                if (*outgoing_command).round_trip_timeout == 0_i32 as u32 {
                    (*outgoing_command).round_trip_timeout = ((*peer).round_trip_time)
                        .wrapping_add(
                            (4_i32 as u32).wrapping_mul((*peer).round_trip_time_variance),
                        );
                }
                if (*peer).sent_reliable_commands.sentinel.next
                    == core::ptr::addr_of_mut!((*peer).sent_reliable_commands.sentinel)
                {
                    (*peer).next_timeout =
                        ((*host).service_time).wrapping_add((*outgoing_command).round_trip_timeout);
                }
                enet_list_insert(
                    &mut (*peer).sent_reliable_commands.sentinel,
                    enet_list_remove(&mut (*outgoing_command).outgoing_command_list),
                );
                (*outgoing_command).sent_time = (*host).service_time;
                (*host).header_flags = ((*host).header_flags as i32
                    | ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as i32)
                    as u16;
                (*peer).reliable_data_in_transit = (*peer)
                    .reliable_data_in_transit
                    .wrapping_add((*outgoing_command).fragment_length as u32);
            } else {
                if !((*outgoing_command).packet).is_null()
                    && (*outgoing_command).fragment_offset == 0_i32 as u32
                {
                    (*peer).packet_throttle_counter = (*peer)
                        .packet_throttle_counter
                        .wrapping_add(ENET_PEER_PACKET_THROTTLE_COUNTER as i32 as u32);
                    (*peer).packet_throttle_counter = (*peer)
                        .packet_throttle_counter
                        .wrapping_rem(ENET_PEER_PACKET_THROTTLE_SCALE as i32 as u32);
                    if (*peer).packet_throttle_counter > (*peer).packet_throttle {
                        let reliable_sequence_number: u16 =
                            (*outgoing_command).reliable_sequence_number;
                        let unreliable_sequence_number: u16 =
                            (*outgoing_command).unreliable_sequence_number;
                        loop {
                            (*(*outgoing_command).packet).reference_count =
                                ((*(*outgoing_command).packet).reference_count).wrapping_sub(1);
                            if (*(*outgoing_command).packet).reference_count == 0_i32 as usize {
                                enet_packet_destroy((*outgoing_command).packet);
                            }
                            enet_list_remove(&mut (*outgoing_command).outgoing_command_list);
                            enet_free(
                                outgoing_command.cast(),
                                Layout::new::<ENetOutgoingCommand>(),
                            );
                            if current_command
                                == core::ptr::addr_of_mut!((*peer).outgoing_commands.sentinel)
                            {
                                break;
                            }
                            outgoing_command = current_command.cast();
                            if (*outgoing_command).reliable_sequence_number as i32
                                != reliable_sequence_number as i32
                                || (*outgoing_command).unreliable_sequence_number as i32
                                    != unreliable_sequence_number as i32
                            {
                                break;
                            }
                            current_command = (*current_command).next;
                        }
                        continue;
                    }
                }
                enet_list_remove(&mut (*outgoing_command).outgoing_command_list);
                if !((*outgoing_command).packet).is_null() {
                    enet_list_insert(
                        &mut (*sent_unreliable_commands).sentinel,
                        outgoing_command.cast(),
                    );
                }
            }
            (*buffer).data = command.cast();
            (*buffer).data_length = command_size;
            (*host).packet_size = ((*host).packet_size).wrapping_add((*buffer).data_length);
            *command = (*outgoing_command).command;
            if !((*outgoing_command).packet).is_null() {
                buffer = buffer.offset(1);
                (*buffer).data = ((*(*outgoing_command).packet).data)
                    .offset((*outgoing_command).fragment_offset as isize);
                (*buffer).data_length = (*outgoing_command).fragment_length as usize;
                (*host).packet_size = ((*host).packet_size as u64)
                    .wrapping_add((*outgoing_command).fragment_length as u64)
                    as usize;
            } else if (*outgoing_command).command.header.command as i32
                & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32
                == 0
            {
                enet_free(
                    outgoing_command.cast(),
                    Layout::new::<ENetOutgoingCommand>(),
                );
            }
            (*peer).packets_sent = ((*peer).packets_sent).wrapping_add(1);
            command = command.offset(1);
            buffer = buffer.offset(1);
        }
    }
    (*host).command_count = command.offset_from(((*host).commands).as_mut_ptr()) as i64 as usize;
    (*host).buffer_count = buffer.offset_from(((*host).buffers).as_mut_ptr()) as i64 as usize;
    if (*peer).state == ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
        && enet_peer_has_outgoing_commands(peer) == 0
        && (*sent_unreliable_commands).sentinel.next
            == core::ptr::addr_of_mut!((*sent_unreliable_commands).sentinel)
    {
        enet_peer_disconnect(peer, (*peer).event_data);
    }
    can_ping
}
unsafe fn enet_protocol_send_outgoing_commands<S: Socket>(
    host: *mut ENetHost<S>,
    event: *mut ENetEvent<S>,
    check_for_timeouts: i32,
) -> Result<bool, S::Error> {
    let mut header_data: [u8; 8] = [0; 8];
    let header: *mut ENetProtocolHeader = header_data.as_mut_ptr().cast();
    let mut should_compress: usize;
    let mut sent_unreliable_commands: ENetList = ENetList {
        sentinel: ENetListNode {
            next: core::ptr::null_mut(),
            previous: core::ptr::null_mut(),
        },
    };
    enet_list_clear(&mut sent_unreliable_commands);
    let mut send_pass: i32 = 0_i32;
    let mut continue_sending: i32 = 0_i32;
    while send_pass <= continue_sending {
        let mut current_peer: *mut ENetPeer<S> = (*host).peers;
        while current_peer < ((*host).peers).add((*host).peer_count) {
            if !((*current_peer).state == ENET_PEER_STATE_DISCONNECTED as i32 as u32
                || (*current_peer).state == ENET_PEER_STATE_ZOMBIE as i32 as u32
                || send_pass > 0_i32
                    && (*current_peer).flags as i32 & ENET_PEER_FLAG_CONTINUE_SENDING as i32 == 0)
            {
                (*current_peer).flags = ((*current_peer).flags as i32
                    & !(ENET_PEER_FLAG_CONTINUE_SENDING as i32))
                    as u16;
                (*host).header_flags = 0_i32 as u16;
                (*host).command_count = 0_i32 as usize;
                (*host).buffer_count = 1_i32 as usize;
                (*host).packet_size = ::core::mem::size_of::<ENetProtocolHeader>();
                if (*current_peer).acknowledgements.sentinel.next
                    != core::ptr::addr_of_mut!((*current_peer).acknowledgements.sentinel)
                {
                    enet_protocol_send_acknowledgements(host, current_peer);
                }
                if check_for_timeouts != 0_i32
                    && ((*current_peer).sent_reliable_commands.sentinel.next
                        != core::ptr::addr_of_mut!((*current_peer).sent_reliable_commands.sentinel))
                    && (((*host).service_time).wrapping_sub((*current_peer).next_timeout)
                        < 86400000_i32 as u32)
                    && enet_protocol_check_timeouts(host, current_peer, event) == 1_i32
                {
                    if !event.is_null() && (*event).type_0 != ENET_EVENT_TYPE_NONE as i32 as u32 {
                        return Ok(true);
                    }
                } else {
                    if ((*current_peer).outgoing_commands.sentinel.next
                        == core::ptr::addr_of_mut!((*current_peer).outgoing_commands.sentinel)
                        && (*current_peer)
                            .outgoing_send_reliable_commands
                            .sentinel
                            .next
                            == core::ptr::addr_of_mut!(
                                (*current_peer).outgoing_send_reliable_commands.sentinel
                            )
                        || enet_protocol_check_outgoing_commands(
                            host,
                            current_peer,
                            &mut sent_unreliable_commands,
                        ) != 0)
                        && (*current_peer).sent_reliable_commands.sentinel.next
                            == core::ptr::addr_of_mut!(
                                (*current_peer).sent_reliable_commands.sentinel
                            )
                        && (if ((*host).service_time)
                            .wrapping_sub((*current_peer).last_receive_time)
                            >= 86400000_i32 as u32
                        {
                            ((*current_peer).last_receive_time).wrapping_sub((*host).service_time)
                        } else {
                            ((*host).service_time).wrapping_sub((*current_peer).last_receive_time)
                        }) >= (*current_peer).ping_interval
                        && ((*current_peer).mtu as usize).wrapping_sub((*host).packet_size)
                            >= ::core::mem::size_of::<ENetProtocolPing>()
                    {
                        enet_peer_ping(current_peer);
                        enet_protocol_check_outgoing_commands(
                            host,
                            current_peer,
                            &mut sent_unreliable_commands,
                        );
                    }
                    if (*host).command_count != 0_i32 as usize {
                        if (*current_peer).packet_loss_epoch == 0_i32 as u32 {
                            (*current_peer).packet_loss_epoch = (*host).service_time;
                        } else if (if ((*host).service_time)
                            .wrapping_sub((*current_peer).packet_loss_epoch)
                            >= 86400000_i32 as u32
                        {
                            ((*current_peer).packet_loss_epoch).wrapping_sub((*host).service_time)
                        } else {
                            ((*host).service_time).wrapping_sub((*current_peer).packet_loss_epoch)
                        }) >= ENET_PEER_PACKET_LOSS_INTERVAL as i32 as u32
                            && (*current_peer).packets_sent > 0_i32 as u32
                        {
                            let packet_loss: u32 = ((*current_peer).packets_lost)
                                .wrapping_mul(ENET_PEER_PACKET_LOSS_SCALE as i32 as u32)
                                .wrapping_div((*current_peer).packets_sent);
                            (*current_peer).packet_loss_variance = ((*current_peer)
                                .packet_loss_variance)
                                .wrapping_mul(3_i32 as u32)
                                .wrapping_add(if packet_loss < (*current_peer).packet_loss {
                                    ((*current_peer).packet_loss).wrapping_sub(packet_loss)
                                } else {
                                    packet_loss.wrapping_sub((*current_peer).packet_loss)
                                })
                                .wrapping_div(4_i32 as u32);
                            (*current_peer).packet_loss = ((*current_peer).packet_loss)
                                .wrapping_mul(7_i32 as u32)
                                .wrapping_add(packet_loss)
                                .wrapping_div(8_i32 as u32);
                            (*current_peer).packet_loss_epoch = (*host).service_time;
                            (*current_peer).packets_sent = 0_i32 as u32;
                            (*current_peer).packets_lost = 0_i32 as u32;
                        }
                        let fresh34 = &mut (*((*host).buffers).as_mut_ptr()).data;
                        *fresh34 = header_data.as_mut_ptr();
                        if (*host).header_flags as i32 & ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as i32
                            != 0
                        {
                            (*header).sent_time =
                                (((*host).service_time & 0xffff_i32 as u32) as u16).to_be();
                            (*((*host).buffers).as_mut_ptr()).data_length =
                                ::core::mem::size_of::<ENetProtocolHeader>();
                        } else {
                            (*((*host).buffers).as_mut_ptr()).data_length = 2;
                        }
                        should_compress = 0_i32 as usize;
                        if let Some(compressor) = (*host).compressor.assume_init_mut() {
                            let original_size: usize = ((*host).packet_size)
                                .wrapping_sub(::core::mem::size_of::<ENetProtocolHeader>());
                            let mut in_buffers = Vec::new();
                            for i in 0..((*host).buffer_count).wrapping_sub(1) {
                                let buffer = ((*host).buffers).as_mut_ptr().add(1 + i);
                                in_buffers.push(super::from_raw_parts_or_empty(
                                    (*buffer).data,
                                    (*buffer).data_length,
                                ));
                            }
                            let compressed_size: usize = compressor.compress(
                                in_buffers,
                                original_size,
                                super::from_raw_parts_or_empty_mut(
                                    ((*host).packet_data[1_i32 as usize]).as_mut_ptr(),
                                    original_size,
                                ),
                            );
                            if compressed_size > 0_i32 as usize && compressed_size < original_size {
                                (*host).header_flags = ((*host).header_flags as i32
                                    | ENET_PROTOCOL_HEADER_FLAG_COMPRESSED as i32)
                                    as u16;
                                should_compress = compressed_size;
                            }
                        }
                        if ((*current_peer).outgoing_peer_id as i32)
                            < ENET_PROTOCOL_MAXIMUM_PEER_ID as i32
                        {
                            (*host).header_flags = ((*host).header_flags as i32
                                | ((*current_peer).outgoing_session_id as i32)
                                    << ENET_PROTOCOL_HEADER_SESSION_SHIFT as i32)
                                as u16;
                        }
                        (*header).peer_id = (((*current_peer).outgoing_peer_id as i32
                            | (*host).header_flags as i32)
                            as u16)
                            .to_be();
                        if let Some(checksum_fn) = (*host).checksum.assume_init_ref() {
                            let checksum_addr: *mut u8 = header_data
                                .as_mut_ptr()
                                .add((*((*host).buffers).as_mut_ptr()).data_length);
                            let mut checksum = if ((*current_peer).outgoing_peer_id as i32)
                                < ENET_PROTOCOL_MAXIMUM_PEER_ID as i32
                            {
                                (*current_peer).connect_id
                            } else {
                                0_i32 as u32
                            };
                            copy_nonoverlapping(
                                core::ptr::addr_of!(checksum).cast(),
                                checksum_addr,
                                ::core::mem::size_of::<u32>(),
                            );
                            let fresh35 = &mut (*((*host).buffers).as_mut_ptr()).data_length;
                            *fresh35 = (*fresh35 as u64)
                                .wrapping_add(::core::mem::size_of::<u32>() as u64)
                                as usize;
                            let mut in_buffers = Vec::new();
                            for i in 0..(*host).buffer_count {
                                let buffer = ((*host).buffers).as_mut_ptr().add(i);
                                in_buffers.push(super::from_raw_parts_or_empty(
                                    (*buffer).data,
                                    (*buffer).data_length,
                                ));
                            }
                            checksum = checksum_fn(&in_buffers);
                            copy_nonoverlapping(
                                core::ptr::addr_of!(checksum).cast(),
                                checksum_addr,
                                ::core::mem::size_of::<u32>(),
                            );
                        }
                        if should_compress > 0_i32 as usize {
                            (*host).buffers[1_i32 as usize].data =
                                ((*host).packet_data[1_i32 as usize]).as_mut_ptr();
                            (*host).buffers[1_i32 as usize].data_length = should_compress;
                            (*host).buffer_count = 2_i32 as usize;
                        }
                        (*current_peer).last_send_time = (*host).service_time;
                        let mut conglomerate_buffer = Vec::new();
                        for buffer_index in 0..(*host).buffer_count {
                            let buffer = &(*host).buffers[buffer_index];
                            conglomerate_buffer.extend_from_slice(super::from_raw_parts_or_empty(
                                buffer.data,
                                buffer.data_length,
                            ));
                        }
                        let sent_length = (*host).socket.assume_init_mut().send(
                            (*current_peer)
                                .address
                                .assume_init_ref()
                                .as_ref()
                                .cloned()
                                .unwrap(),
                            &conglomerate_buffer,
                        );
                        enet_protocol_remove_sent_unreliable_commands(
                            current_peer,
                            &mut sent_unreliable_commands,
                        );
                        match sent_length {
                            Err(err) => return Err(err),
                            Ok(sent_length) => {
                                (*host).total_sent_data =
                                    (*host).total_sent_data.wrapping_add(sent_length as u32);
                            }
                        }
                        (*host).total_sent_packets = ((*host).total_sent_packets).wrapping_add(1);
                    }
                }
                if (*current_peer).flags as i32 & ENET_PEER_FLAG_CONTINUE_SENDING as i32 != 0 {
                    continue_sending = send_pass + 1_i32;
                }
            }
            current_peer = current_peer.offset(1);
        }
        send_pass += 1;
    }
    Ok(false)
}
pub(crate) unsafe fn enet_host_flush<S: Socket>(host: *mut ENetHost<S>) {
    (*host).service_time = enet_time_get(host);
    // TODO: enet ignores the error here, but is that really what we want?
    // a socket's send could error and no one would know
    _ = enet_protocol_send_outgoing_commands(host, core::ptr::null_mut(), 0_i32);
}
pub(crate) unsafe fn enet_host_check_events<S: Socket>(
    host: *mut ENetHost<S>,
    event: *mut ENetEvent<S>, // SAFETY: should not be null
) -> bool {
    (*event).type_0 = ENET_EVENT_TYPE_NONE;
    (*event).peer = core::ptr::null_mut();
    (*event).packet = core::ptr::null_mut();
    enet_protocol_dispatch_incoming_commands(host, event)
}
pub(crate) unsafe fn enet_host_service<S: Socket>(
    host: *mut ENetHost<S>,
    event: *mut ENetEvent<S>, // SAFETY: should not be null
) -> Result<bool, S::Error> {
    (*event).type_0 = ENET_EVENT_TYPE_NONE;
    (*event).peer = core::ptr::null_mut();
    (*event).packet = core::ptr::null_mut();
    if enet_protocol_dispatch_incoming_commands(host, event) {
        return Ok(true);
    }
    (*host).service_time = enet_time_get(host);
    if (if ((*host).service_time).wrapping_sub((*host).bandwidth_throttle_epoch)
        >= 86400000_i32 as u32
    {
        ((*host).bandwidth_throttle_epoch).wrapping_sub((*host).service_time)
    } else {
        ((*host).service_time).wrapping_sub((*host).bandwidth_throttle_epoch)
    }) >= ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL
    {
        enet_host_bandwidth_throttle(host);
    }
    match enet_protocol_send_outgoing_commands(host, event, 1_i32) {
        Ok(true) => return Ok(true),
        Ok(false) => {}
        Err(err) => return Err(err),
    }
    match enet_protocol_receive_incoming_commands(host, event) {
        Ok(true) => return Ok(true),
        Ok(false) => {}
        Err(err) => return Err(err),
    }
    match enet_protocol_send_outgoing_commands(host, event, 1_i32) {
        Ok(true) => return Ok(true),
        Ok(false) => {}
        Err(err) => return Err(err),
    }
    if enet_protocol_dispatch_incoming_commands(host, event) {
        return Ok(true);
    }
    Ok(false)
}
