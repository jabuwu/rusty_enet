use core::{
    alloc::Layout,
    mem::MaybeUninit,
    ptr::{addr_of_mut, write_bytes},
};

use crate::{
    consts::*, enet_free, enet_host_flush, enet_list_clear, enet_list_insert, enet_list_move,
    enet_list_remove, enet_malloc, enet_packet_create, enet_packet_destroy,
    enet_protocol_command_size, error::PeerSendError, ENetAcknowledgement, ENetChannel,
    ENetIncomingCommand, ENetList, ENetListIterator, ENetListNode, ENetOutgoingCommand, ENetPacket,
    ENetProtocol, ENetProtocolAcknowledge, ENetProtocolCommandHeader, ENetProtocolHeader,
    ENetProtocolSendFragment, Socket, ENET_PACKET_FLAG_RELIABLE,
    ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT, ENET_PACKET_FLAG_UNSEQUENCED,
    ENET_PROTOCOL_COMMAND_DISCONNECT, ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE,
    ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED, ENET_PROTOCOL_COMMAND_MASK, ENET_PROTOCOL_COMMAND_PING,
    ENET_PROTOCOL_COMMAND_SEND_FRAGMENT, ENET_PROTOCOL_COMMAND_SEND_RELIABLE,
    ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE, ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT,
    ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED, ENET_PROTOCOL_COMMAND_THROTTLE_CONFIGURE,
};

use super::{ENetHost, ENetNewProtocolHeader};

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
#[repr(C)]
pub(crate) struct ENetPeer<S: Socket> {
    pub(crate) dispatch_list: ENetListNode,
    pub(crate) host: *mut ENetHost<S>,
    pub(crate) outgoing_peer_id: u16,
    pub(crate) incoming_peer_id: u16,
    pub(crate) connect_id: u32,
    pub(crate) outgoing_session_id: u8,
    pub(crate) incoming_session_id: u8,
    pub(crate) address: MaybeUninit<Option<S::Address>>,
    pub(crate) data: *mut u8,
    pub(crate) state: ENetPeerState,
    pub(crate) channels: *mut ENetChannel,
    pub(crate) channel_count: usize,
    pub(crate) incoming_bandwidth: u32,
    pub(crate) outgoing_bandwidth: u32,
    pub(crate) incoming_bandwidth_throttle_epoch: u32,
    pub(crate) outgoing_bandwidth_throttle_epoch: u32,
    pub(crate) incoming_data_total: u32,
    pub(crate) outgoing_data_total: u32,
    pub(crate) last_send_time: u32,
    pub(crate) last_receive_time: u32,
    pub(crate) next_timeout: u32,
    pub(crate) earliest_timeout: u32,
    pub(crate) packet_loss_epoch: u32,
    pub(crate) packets_sent: u32,
    pub(crate) packets_lost: u32,
    pub(crate) packet_loss: u32,
    pub(crate) packet_loss_variance: u32,
    pub(crate) packet_throttle: u32,
    pub(crate) packet_throttle_limit: u32,
    pub(crate) packet_throttle_counter: u32,
    pub(crate) packet_throttle_epoch: u32,
    pub(crate) packet_throttle_acceleration: u32,
    pub(crate) packet_throttle_deceleration: u32,
    pub(crate) packet_throttle_interval: u32,
    pub(crate) ping_interval: u32,
    pub(crate) timeout_limit: u32,
    pub(crate) timeout_minimum: u32,
    pub(crate) timeout_maximum: u32,
    pub(crate) last_round_trip_time: u32,
    pub(crate) lowest_round_trip_time: u32,
    pub(crate) last_round_trip_time_variance: u32,
    pub(crate) highest_round_trip_time_variance: u32,
    pub(crate) round_trip_time: u32,
    pub(crate) round_trip_time_variance: u32,
    pub(crate) mtu: u32,
    pub(crate) window_size: u32,
    pub(crate) reliable_data_in_transit: u32,
    pub(crate) outgoing_reliable_sequence_number: u16,
    pub(crate) acknowledgements: ENetList,
    pub(crate) sent_reliable_commands: ENetList,
    pub(crate) outgoing_send_reliable_commands: ENetList,
    pub(crate) outgoing_commands: ENetList,
    pub(crate) dispatched_commands: ENetList,
    pub(crate) flags: u16,
    pub(crate) reserved: u16,
    pub(crate) incoming_unsequenced_group: u16,
    pub(crate) outgoing_unsequenced_group: u16,
    pub(crate) unsequenced_window: [u32; 32],
    pub(crate) event_data: u32,
    pub(crate) total_waiting_data: usize,
}
pub(crate) unsafe fn enet_peer_throttle_configure<S: Socket>(
    peer: *mut ENetPeer<S>,
    interval: u32,
    acceleration: u32,
    deceleration: u32,
) {
    let mut command: ENetProtocol = ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channel_id: 0,
            reliable_sequence_number: 0,
        },
    };
    (*peer).packet_throttle_interval = interval;
    (*peer).packet_throttle_acceleration = acceleration;
    (*peer).packet_throttle_deceleration = deceleration;
    command.header.command = (ENET_PROTOCOL_COMMAND_THROTTLE_CONFIGURE as i32
        | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32) as u8;
    command.header.channel_id = 0xff_i32 as u8;
    command.throttle_configure.packet_throttle_interval = interval.to_be();
    command.throttle_configure.packet_throttle_acceleration = acceleration.to_be();
    command.throttle_configure.packet_throttle_deceleration = deceleration.to_be();
    enet_peer_queue_outgoing_command(
        peer,
        &command,
        core::ptr::null_mut(),
        0_i32 as u32,
        0_i32 as u16,
    );
}
pub(crate) unsafe fn enet_peer_throttle<S: Socket>(peer: *mut ENetPeer<S>, rtt: u32) -> i32 {
    if (*peer).last_round_trip_time <= (*peer).last_round_trip_time_variance {
        (*peer).packet_throttle = (*peer).packet_throttle_limit;
    } else if rtt <= (*peer).last_round_trip_time {
        (*peer).packet_throttle = (*peer)
            .packet_throttle
            .wrapping_add((*peer).packet_throttle_acceleration);
        if (*peer).packet_throttle > (*peer).packet_throttle_limit {
            (*peer).packet_throttle = (*peer).packet_throttle_limit;
        }
        return 1_i32;
    } else if rtt
        > ((*peer).last_round_trip_time)
            .wrapping_add((2_i32 as u32).wrapping_mul((*peer).last_round_trip_time_variance))
    {
        if (*peer).packet_throttle > (*peer).packet_throttle_deceleration {
            (*peer).packet_throttle = (*peer)
                .packet_throttle
                .wrapping_sub((*peer).packet_throttle_deceleration);
        } else {
            (*peer).packet_throttle = 0_i32 as u32;
        }
        return -1_i32;
    }
    0_i32
}
pub(crate) unsafe fn enet_peer_send<S: Socket>(
    peer: *mut ENetPeer<S>,
    channel_id: u8,
    packet: *mut ENetPacket,
) -> Result<(), PeerSendError> {
    let mut command: ENetProtocol = ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channel_id: 0,
            reliable_sequence_number: 0,
        },
    };
    let mut fragment_length: usize;
    if (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32 {
        return Err(PeerSendError::NotConnected);
    }
    if channel_id as usize >= (*peer).channel_count {
        return Err(PeerSendError::InvalidChannel);
    }
    if (*packet).data_length > (*(*peer).host).maximum_packet_size {
        return Err(PeerSendError::PacketTooLarge);
    }
    let channel = ((*peer).channels).offset(channel_id as isize);
    if (*(*peer).host).using_new_packet {
        fragment_length = ((*peer).mtu as usize)
            .wrapping_sub(::core::mem::size_of::<ENetNewProtocolHeader>())
            .wrapping_sub(::core::mem::size_of::<ENetProtocolSendFragment>());
    } else {
        fragment_length = ((*peer).mtu as usize)
            .wrapping_sub(::core::mem::size_of::<ENetProtocolHeader>())
            .wrapping_sub(::core::mem::size_of::<ENetProtocolSendFragment>());
    }
    if ((*(*peer).host).checksum.assume_init_ref()).is_some() {
        fragment_length =
            (fragment_length as u64).wrapping_sub(::core::mem::size_of::<u32>() as u64) as usize;
    }
    if (*packet).data_length > fragment_length {
        let fragment_count: u32 = ((*packet).data_length)
            .wrapping_add(fragment_length)
            .wrapping_sub(1_i32 as usize)
            .wrapping_div(fragment_length) as u32;
        let mut fragment_number: u32;
        let mut fragment_offset: u32;
        let command_number: u8;
        let start_sequence_number: u16;
        let mut fragments: ENetList = ENetList {
            sentinel: ENetListNode {
                next: core::ptr::null_mut(),
                previous: core::ptr::null_mut(),
            },
        };
        let mut fragment: *mut ENetOutgoingCommand;
        if fragment_count > PROTOCOL_MAXIMUM_FRAGMENT_COUNT as i32 as u32 {
            return Err(PeerSendError::FragmentsExceeded);
        }
        if (*packet).flags
            & (ENET_PACKET_FLAG_RELIABLE as i32 | ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT as i32)
                as u32
            == ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT as i32 as u32
            && ((*channel).outgoing_unreliable_sequence_number as i32) < 0xffff_i32
        {
            command_number = ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT as i32 as u8;
            start_sequence_number =
                (((*channel).outgoing_unreliable_sequence_number as i32 + 1_i32) as u16).to_be();
        } else {
            command_number = (ENET_PROTOCOL_COMMAND_SEND_FRAGMENT as i32
                | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32) as u8;
            start_sequence_number =
                (((*channel).outgoing_reliable_sequence_number as i32 + 1_i32) as u16).to_be();
        }
        enet_list_clear(&mut fragments);
        fragment_number = 0_i32 as u32;
        fragment_offset = 0_i32 as u32;
        while (fragment_offset as usize) < (*packet).data_length {
            if ((*packet).data_length).wrapping_sub(fragment_offset as usize) < fragment_length {
                fragment_length = ((*packet).data_length).wrapping_sub(fragment_offset as usize);
            }
            fragment = enet_malloc(Layout::new::<ENetOutgoingCommand>()).cast();
            (*fragment).fragment_offset = fragment_offset;
            (*fragment).fragment_length = fragment_length as u16;
            (*fragment).packet = packet;
            (*fragment).command.header.command = command_number;
            (*fragment).command.header.channel_id = channel_id;
            (*fragment).command.send_fragment.start_sequence_number = start_sequence_number;
            (*fragment).command.send_fragment.data_length = (fragment_length as u16).to_be();
            (*fragment).command.send_fragment.fragment_count = fragment_count.to_be();
            (*fragment).command.send_fragment.fragment_number = fragment_number.to_be();
            (*fragment).command.send_fragment.total_length = ((*packet).data_length as u32).to_be();
            (*fragment).command.send_fragment.fragment_offset = u32::from_be(fragment_offset);
            enet_list_insert(&mut fragments.sentinel, fragment.cast());
            fragment_number = fragment_number.wrapping_add(1);
            fragment_offset = (fragment_offset as usize).wrapping_add(fragment_length) as u32;
        }
        (*packet).reference_count =
            ((*packet).reference_count as u64).wrapping_add(fragment_number as u64) as usize;
        while fragments.sentinel.next != core::ptr::addr_of_mut!(fragments.sentinel) {
            fragment = enet_list_remove(fragments.sentinel.next).cast();
            enet_peer_setup_outgoing_command(peer, fragment);
        }
        return Ok(());
    }
    command.header.channel_id = channel_id;
    if (*packet).flags
        & (ENET_PACKET_FLAG_RELIABLE as i32 | ENET_PACKET_FLAG_UNSEQUENCED as i32) as u32
        == ENET_PACKET_FLAG_UNSEQUENCED as i32 as u32
    {
        command.header.command = (ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as i32
            | ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED as i32) as u8;
        command.send_unsequenced.data_length = ((*packet).data_length as u16).to_be();
    } else if (*packet).flags & ENET_PACKET_FLAG_RELIABLE as i32 as u32 != 0
        || (*channel).outgoing_unreliable_sequence_number as i32 >= 0xffff_i32
    {
        command.header.command = (ENET_PROTOCOL_COMMAND_SEND_RELIABLE as i32
            | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32) as u8;
        command.send_reliable.data_length = ((*packet).data_length as u16).to_be();
    } else {
        command.header.command = ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE as i32 as u8;
        command.send_unreliable.data_length = ((*packet).data_length as u16).to_be();
    }
    if (enet_peer_queue_outgoing_command(
        peer,
        &command,
        packet,
        0_i32 as u32,
        (*packet).data_length as u16,
    ))
    .is_null()
    {
        return Err(PeerSendError::FailedToQueue);
    }
    Ok(())
}
pub(crate) unsafe fn enet_peer_receive<S: Socket>(
    peer: *mut ENetPeer<S>,
    channel_id: *mut u8,
) -> *mut ENetPacket {
    if (*peer).dispatched_commands.sentinel.next
        == core::ptr::addr_of_mut!((*peer).dispatched_commands.sentinel)
    {
        return core::ptr::null_mut();
    }
    let incoming_command: *mut ENetIncomingCommand =
        enet_list_remove((*peer).dispatched_commands.sentinel.next).cast();
    if !channel_id.is_null() {
        *channel_id = (*incoming_command).command.header.channel_id;
    }
    let packet = (*incoming_command).packet;
    (*packet).reference_count = ((*packet).reference_count).wrapping_sub(1);
    if !((*incoming_command).fragments).is_null() {
        let count = (*incoming_command)
            .fragment_count
            .wrapping_add(31_i32 as u32)
            .wrapping_div(32_i32 as u32) as usize;
        enet_free(
            (*incoming_command).fragments.cast(),
            Layout::array::<u32>(count).unwrap(),
        );
    }
    enet_free(
        incoming_command.cast(),
        Layout::new::<ENetIncomingCommand>(),
    );
    (*peer).total_waiting_data = (*peer)
        .total_waiting_data
        .wrapping_sub((*packet).data_length) as usize as usize;
    packet
}
unsafe fn enet_peer_reset_outgoing_commands(queue: *mut ENetList) {
    let mut outgoing_command: *mut ENetOutgoingCommand;
    while (*queue).sentinel.next != core::ptr::addr_of_mut!((*queue).sentinel) {
        outgoing_command = enet_list_remove((*queue).sentinel.next).cast();
        if !((*outgoing_command).packet).is_null() {
            (*(*outgoing_command).packet).reference_count =
                ((*(*outgoing_command).packet).reference_count).wrapping_sub(1);
            if (*(*outgoing_command).packet).reference_count == 0_i32 as usize {
                enet_packet_destroy((*outgoing_command).packet);
            }
        }
        enet_free(
            outgoing_command.cast(),
            Layout::new::<ENetOutgoingCommand>(),
        );
    }
}
unsafe fn enet_peer_remove_incoming_commands(
    mut _queue: *mut ENetList,
    start_command: ENetListIterator,
    end_command: ENetListIterator,
    exclude_command: *mut ENetIncomingCommand,
) {
    let mut current_command: ENetListIterator;
    current_command = start_command;
    while current_command != end_command {
        let incoming_command: *mut ENetIncomingCommand = current_command.cast();
        current_command = (*current_command).next;
        if incoming_command == exclude_command {
            continue;
        }
        enet_list_remove(&mut (*incoming_command).incoming_command_list);
        if !((*incoming_command).packet).is_null() {
            (*(*incoming_command).packet).reference_count =
                ((*(*incoming_command).packet).reference_count).wrapping_sub(1);
            if (*(*incoming_command).packet).reference_count == 0_i32 as usize {
                enet_packet_destroy((*incoming_command).packet);
            }
        }
        if !((*incoming_command).fragments).is_null() {
            let count = (*incoming_command)
                .fragment_count
                .wrapping_add(31_i32 as u32)
                .wrapping_div(32_i32 as u32) as usize;
            enet_free(
                (*incoming_command).fragments.cast(),
                Layout::array::<u32>(count).unwrap(),
            );
        }
        enet_free(
            incoming_command.cast(),
            Layout::new::<ENetIncomingCommand>(),
        );
    }
}
unsafe fn enet_peer_reset_incoming_commands(queue: *mut ENetList) {
    enet_peer_remove_incoming_commands(
        queue,
        (*queue).sentinel.next,
        &mut (*queue).sentinel,
        core::ptr::null_mut(),
    );
}
pub(crate) unsafe fn enet_peer_reset_queues<S: Socket>(peer: *mut ENetPeer<S>) {
    let mut channel: *mut ENetChannel;
    if (*peer).flags as i32 & ENET_PEER_FLAG_NEEDS_DISPATCH as i32 != 0 {
        enet_list_remove(&mut (*peer).dispatch_list);
        (*peer).flags = ((*peer).flags as i32 & !(ENET_PEER_FLAG_NEEDS_DISPATCH as i32)) as u16;
    }
    while (*peer).acknowledgements.sentinel.next
        != core::ptr::addr_of_mut!((*peer).acknowledgements.sentinel)
    {
        enet_free(
            enet_list_remove((*peer).acknowledgements.sentinel.next),
            Layout::new::<ENetAcknowledgement>(),
        );
    }
    enet_peer_reset_outgoing_commands(&mut (*peer).sent_reliable_commands);
    enet_peer_reset_outgoing_commands(&mut (*peer).outgoing_commands);
    enet_peer_reset_outgoing_commands(&mut (*peer).outgoing_send_reliable_commands);
    enet_peer_reset_incoming_commands(&mut (*peer).dispatched_commands);
    if !((*peer).channels).is_null() && (*peer).channel_count > 0_i32 as usize {
        channel = (*peer).channels;
        while channel < ((*peer).channels).add((*peer).channel_count) {
            enet_peer_reset_incoming_commands(&mut (*channel).incoming_reliable_commands);
            enet_peer_reset_incoming_commands(&mut (*channel).incoming_unreliable_commands);
            channel = channel.offset(1);
        }
        enet_free(
            (*peer).channels.cast(),
            Layout::array::<ENetChannel>((*peer).channel_count).unwrap(),
        );
    }
    (*peer).channels = core::ptr::null_mut();
    (*peer).channel_count = 0_i32 as usize;
}
pub(crate) unsafe fn enet_peer_on_connect<S: Socket>(peer: *mut ENetPeer<S>) {
    if (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32
        && (*peer).state != ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        if (*peer).incoming_bandwidth != 0_i32 as u32 {
            (*(*peer).host).bandwidth_limited_peers =
                ((*(*peer).host).bandwidth_limited_peers).wrapping_add(1);
        }
        (*(*peer).host).connected_peers = ((*(*peer).host).connected_peers).wrapping_add(1);
    }
}
pub(crate) unsafe fn enet_peer_on_disconnect<S: Socket>(peer: *mut ENetPeer<S>) {
    if (*peer).state == ENET_PEER_STATE_CONNECTED as i32 as u32
        || (*peer).state == ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32
    {
        if (*peer).incoming_bandwidth != 0_i32 as u32 {
            (*(*peer).host).bandwidth_limited_peers =
                ((*(*peer).host).bandwidth_limited_peers).wrapping_sub(1);
        }
        (*(*peer).host).connected_peers = ((*(*peer).host).connected_peers).wrapping_sub(1);
    }
}
pub(crate) unsafe fn enet_peer_reset<S: Socket>(peer: *mut ENetPeer<S>) {
    enet_peer_on_disconnect(peer);
    (*peer).outgoing_peer_id = PROTOCOL_MAXIMUM_PEER_ID as i32 as u16;
    (*peer).connect_id = 0_i32 as u32;
    (*peer).state = ENET_PEER_STATE_DISCONNECTED;
    (*peer).incoming_bandwidth = 0_i32 as u32;
    (*peer).outgoing_bandwidth = 0_i32 as u32;
    (*peer).incoming_bandwidth_throttle_epoch = 0_i32 as u32;
    (*peer).outgoing_bandwidth_throttle_epoch = 0_i32 as u32;
    (*peer).incoming_data_total = 0_i32 as u32;
    (*peer).outgoing_data_total = 0_i32 as u32;
    (*peer).last_send_time = 0_i32 as u32;
    (*peer).last_receive_time = 0_i32 as u32;
    (*peer).next_timeout = 0_i32 as u32;
    (*peer).earliest_timeout = 0_i32 as u32;
    (*peer).packet_loss_epoch = 0_i32 as u32;
    (*peer).packets_sent = 0_i32 as u32;
    (*peer).packets_lost = 0_i32 as u32;
    (*peer).packet_loss = 0_i32 as u32;
    (*peer).packet_loss_variance = 0_i32 as u32;
    (*peer).packet_throttle = PEER_DEFAULT_PACKET_THROTTLE as i32 as u32;
    (*peer).packet_throttle_limit = PEER_PACKET_THROTTLE_SCALE as i32 as u32;
    (*peer).packet_throttle_counter = 0_i32 as u32;
    (*peer).packet_throttle_epoch = 0_i32 as u32;
    (*peer).packet_throttle_acceleration = PEER_PACKET_THROTTLE_ACCELERATION as i32 as u32;
    (*peer).packet_throttle_deceleration = PEER_PACKET_THROTTLE_DECELERATION as i32 as u32;
    (*peer).packet_throttle_interval = PEER_PACKET_THROTTLE_INTERVAL as i32 as u32;
    (*peer).ping_interval = PEER_PING_INTERVAL as i32 as u32;
    (*peer).timeout_limit = PEER_TIMEOUT_LIMIT as i32 as u32;
    (*peer).timeout_minimum = PEER_TIMEOUT_MINIMUM as i32 as u32;
    (*peer).timeout_maximum = PEER_TIMEOUT_MAXIMUM as i32 as u32;
    (*peer).last_round_trip_time = PEER_DEFAULT_ROUND_TRIP_TIME as i32 as u32;
    (*peer).lowest_round_trip_time = PEER_DEFAULT_ROUND_TRIP_TIME as i32 as u32;
    (*peer).last_round_trip_time_variance = 0_i32 as u32;
    (*peer).highest_round_trip_time_variance = 0_i32 as u32;
    (*peer).round_trip_time = PEER_DEFAULT_ROUND_TRIP_TIME as i32 as u32;
    (*peer).round_trip_time_variance = 0_i32 as u32;
    (*peer).mtu = (*(*peer).host).mtu;
    (*peer).reliable_data_in_transit = 0_i32 as u32;
    (*peer).outgoing_reliable_sequence_number = 0_i32 as u16;
    (*peer).window_size = PROTOCOL_MAXIMUM_WINDOW_SIZE as i32 as u32;
    (*peer).incoming_unsequenced_group = 0_i32 as u16;
    (*peer).outgoing_unsequenced_group = 0_i32 as u16;
    (*peer).event_data = 0_i32 as u32;
    (*peer).total_waiting_data = 0_i32 as usize;
    (*peer).flags = 0_i32 as u16;
    write_bytes(((*peer).unsequenced_window).as_mut_ptr(), 0, 32);
    enet_peer_reset_queues(peer);
}
pub(crate) unsafe fn enet_peer_ping<S: Socket>(peer: *mut ENetPeer<S>) {
    let mut command: ENetProtocol = ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channel_id: 0,
            reliable_sequence_number: 0,
        },
    };
    if (*peer).state != ENET_PEER_STATE_CONNECTED as i32 as u32 {
        return;
    }
    command.header.command =
        (ENET_PROTOCOL_COMMAND_PING as i32 | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32) as u8;
    command.header.channel_id = 0xff_i32 as u8;
    enet_peer_queue_outgoing_command(
        peer,
        &command,
        core::ptr::null_mut(),
        0_i32 as u32,
        0_i32 as u16,
    );
}
pub(crate) unsafe fn enet_peer_ping_interval<S: Socket>(
    peer: *mut ENetPeer<S>,
    ping_interval: u32,
) {
    (*peer).ping_interval = if ping_interval != 0 {
        ping_interval
    } else {
        PEER_PING_INTERVAL as i32 as u32
    };
}
pub(crate) unsafe fn enet_peer_timeout<S: Socket>(
    peer: *mut ENetPeer<S>,
    timeout_limit: u32,
    timeout_minimum: u32,
    timeout_maximum: u32,
) {
    (*peer).timeout_limit = if timeout_limit != 0 {
        timeout_limit
    } else {
        PEER_TIMEOUT_LIMIT as i32 as u32
    };
    (*peer).timeout_minimum = if timeout_minimum != 0 {
        timeout_minimum
    } else {
        PEER_TIMEOUT_MINIMUM as i32 as u32
    };
    (*peer).timeout_maximum = if timeout_maximum != 0 {
        timeout_maximum
    } else {
        PEER_TIMEOUT_MAXIMUM as i32 as u32
    };
}
pub(crate) unsafe fn enet_peer_disconnect_now<S: Socket>(peer: *mut ENetPeer<S>, data: u32) {
    let mut command: ENetProtocol = ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channel_id: 0,
            reliable_sequence_number: 0,
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
        command.header.channel_id = 0xff_i32 as u8;
        command.disconnect.data = data.to_be();
        enet_peer_queue_outgoing_command(
            peer,
            &command,
            core::ptr::null_mut(),
            0_i32 as u32,
            0_i32 as u16,
        );
        enet_host_flush((*peer).host);
    }
    enet_peer_reset(peer);
}
pub(crate) unsafe fn enet_peer_disconnect<S: Socket>(peer: *mut ENetPeer<S>, data: u32) {
    let mut command: ENetProtocol = ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channel_id: 0,
            reliable_sequence_number: 0,
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
    command.header.channel_id = 0xff_i32 as u8;
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
        core::ptr::null_mut(),
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
    if (*peer).outgoing_commands.sentinel.next
        == core::ptr::addr_of_mut!((*peer).outgoing_commands.sentinel)
        && (*peer).outgoing_send_reliable_commands.sentinel.next
            == core::ptr::addr_of_mut!((*peer).outgoing_send_reliable_commands.sentinel)
        && (*peer).sent_reliable_commands.sentinel.next
            == core::ptr::addr_of_mut!((*peer).sent_reliable_commands.sentinel)
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
        (*peer).event_data = data;
    } else {
        enet_peer_disconnect(peer, data);
    };
}
pub(crate) unsafe fn enet_peer_queue_acknowledgement<S: Socket>(
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
    sent_time: u16,
) -> *mut ENetAcknowledgement {
    if ((*command).header.channel_id as usize) < (*peer).channel_count {
        let channel: *mut ENetChannel =
            ((*peer).channels).offset((*command).header.channel_id as isize);
        let mut reliable_window: u16 = ((*command).header.reliable_sequence_number as i32
            / PEER_RELIABLE_WINDOW_SIZE as i32) as u16;
        let current_window: u16 = ((*channel).incoming_reliable_sequence_number as i32
            / PEER_RELIABLE_WINDOW_SIZE as i32) as u16;
        if ((*command).header.reliable_sequence_number as i32)
            < (*channel).incoming_reliable_sequence_number as i32
        {
            reliable_window = (reliable_window as i32 + PEER_RELIABLE_WINDOWS as i32) as u16;
        }
        if reliable_window as i32
            >= current_window as i32 + PEER_FREE_RELIABLE_WINDOWS as i32 - 1_i32
            && reliable_window as i32 <= current_window as i32 + PEER_FREE_RELIABLE_WINDOWS as i32
        {
            return core::ptr::null_mut();
        }
    }
    let acknowledgement: *mut ENetAcknowledgement =
        enet_malloc(Layout::new::<ENetAcknowledgement>()).cast();
    (*peer).outgoing_data_total = ((*peer).outgoing_data_total as u64)
        .wrapping_add(::core::mem::size_of::<ENetProtocolAcknowledge>() as u64)
        as u32;
    (*acknowledgement).sent_time = sent_time as u32;
    (*acknowledgement).command = *command;
    enet_list_insert(
        &mut (*peer).acknowledgements.sentinel,
        acknowledgement.cast(),
    );
    acknowledgement
}
pub(crate) unsafe fn enet_peer_setup_outgoing_command<S: Socket>(
    peer: *mut ENetPeer<S>,
    outgoing_command: *mut ENetOutgoingCommand,
) {
    (*peer).outgoing_data_total = ((*peer).outgoing_data_total as usize).wrapping_add(
        (enet_protocol_command_size((*outgoing_command).command.header.command))
            .wrapping_add((*outgoing_command).fragment_length as usize),
    ) as u32 as u32;
    if (*outgoing_command).command.header.channel_id as i32 == 0xff_i32 {
        (*peer).outgoing_reliable_sequence_number =
            ((*peer).outgoing_reliable_sequence_number).wrapping_add(1);
        (*outgoing_command).reliable_sequence_number = (*peer).outgoing_reliable_sequence_number;
        (*outgoing_command).unreliable_sequence_number = 0_i32 as u16;
    } else {
        let channel: *mut ENetChannel =
            ((*peer).channels).offset((*outgoing_command).command.header.channel_id as isize);
        if (*outgoing_command).command.header.command as i32
            & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32
            != 0
        {
            (*channel).outgoing_reliable_sequence_number =
                ((*channel).outgoing_reliable_sequence_number).wrapping_add(1);
            (*channel).outgoing_unreliable_sequence_number = 0_i32 as u16;
            (*outgoing_command).reliable_sequence_number =
                (*channel).outgoing_reliable_sequence_number;
            (*outgoing_command).unreliable_sequence_number = 0_i32 as u16;
        } else if (*outgoing_command).command.header.command as i32
            & ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED as i32
            != 0
        {
            (*peer).outgoing_unsequenced_group =
                ((*peer).outgoing_unsequenced_group).wrapping_add(1);
            (*outgoing_command).reliable_sequence_number = 0_i32 as u16;
            (*outgoing_command).unreliable_sequence_number = 0_i32 as u16;
        } else {
            if (*outgoing_command).fragment_offset == 0_i32 as u32 {
                (*channel).outgoing_unreliable_sequence_number =
                    ((*channel).outgoing_unreliable_sequence_number).wrapping_add(1);
            }
            (*outgoing_command).reliable_sequence_number =
                (*channel).outgoing_reliable_sequence_number;
            (*outgoing_command).unreliable_sequence_number =
                (*channel).outgoing_unreliable_sequence_number;
        }
    }
    (*outgoing_command).send_attempts = 0_i32 as u16;
    (*outgoing_command).sent_time = 0_i32 as u32;
    (*outgoing_command).round_trip_timeout = 0_i32 as u32;
    (*outgoing_command).command.header.reliable_sequence_number =
        (*outgoing_command).reliable_sequence_number.to_be();
    (*(*peer).host).total_queued = ((*(*peer).host).total_queued).wrapping_add(1);
    (*outgoing_command).queue_time = (*(*peer).host).total_queued;
    match (*outgoing_command).command.header.command as i32 & ENET_PROTOCOL_COMMAND_MASK as i32 {
        7 => {
            (*outgoing_command)
                .command
                .send_unreliable
                .unreliable_sequence_number =
                (*outgoing_command).unreliable_sequence_number.to_be();
        }
        9 => {
            (*outgoing_command)
                .command
                .send_unsequenced
                .unsequenced_group = (*peer).outgoing_unsequenced_group.to_be();
        }
        _ => {}
    }
    if (*outgoing_command).command.header.command as i32
        & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as i32
        != 0_i32
        && !((*outgoing_command).packet).is_null()
    {
        enet_list_insert(
            &mut (*peer).outgoing_send_reliable_commands.sentinel,
            outgoing_command.cast(),
        );
    } else {
        enet_list_insert(
            &mut (*peer).outgoing_commands.sentinel,
            outgoing_command.cast(),
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
    let outgoing_command: *mut ENetOutgoingCommand =
        enet_malloc(Layout::new::<ENetOutgoingCommand>()).cast();
    if outgoing_command.is_null() {
        return core::ptr::null_mut();
    }
    (*outgoing_command).command = *command;
    (*outgoing_command).fragment_offset = offset;
    (*outgoing_command).fragment_length = length;
    (*outgoing_command).packet = packet;
    if !packet.is_null() {
        (*packet).reference_count = ((*packet).reference_count).wrapping_add(1);
    }
    enet_peer_setup_outgoing_command(peer, outgoing_command);
    outgoing_command
}
pub(crate) unsafe fn enet_peer_dispatch_incoming_unreliable_commands<S: Socket>(
    peer: *mut ENetPeer<S>,
    channel: *mut ENetChannel,
    queued_command: *mut ENetIncomingCommand,
) {
    let mut dropped_command: ENetListIterator;
    let mut start_command: ENetListIterator;
    let mut current_command: ENetListIterator;
    let mut current_block_22: u64;
    current_command = (*channel).incoming_unreliable_commands.sentinel.next;
    start_command = current_command;
    dropped_command = start_command;
    while current_command
        != core::ptr::addr_of_mut!((*channel).incoming_unreliable_commands.sentinel)
    {
        let incoming_command: *mut ENetIncomingCommand = current_command.cast();
        if (*incoming_command).command.header.command as i32 & ENET_PROTOCOL_COMMAND_MASK as i32
            != ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as i32
        {
            if (*incoming_command).reliable_sequence_number as i32
                == (*channel).incoming_reliable_sequence_number as i32
            {
                if (*incoming_command).fragments_remaining <= 0_i32 as u32 {
                    (*channel).incoming_unreliable_sequence_number =
                        (*incoming_command).unreliable_sequence_number;
                    current_block_22 = 11174649648027449784;
                } else {
                    if start_command != current_command {
                        enet_list_move(
                            &mut (*peer).dispatched_commands.sentinel,
                            start_command.cast(),
                            (*current_command).previous.cast(),
                        );
                        if (*peer).flags as i32 & ENET_PEER_FLAG_NEEDS_DISPATCH as i32 == 0 {
                            enet_list_insert(
                                &mut (*(*peer).host).dispatch_queue.sentinel,
                                core::ptr::addr_of_mut!((*peer).dispatch_list).cast(),
                            );
                            (*peer).flags = ((*peer).flags as i32
                                | ENET_PEER_FLAG_NEEDS_DISPATCH as i32)
                                as u16;
                        }
                        dropped_command = current_command;
                    } else if dropped_command != current_command {
                        dropped_command = (*current_command).previous;
                    }
                    current_block_22 = 13472856163611868459;
                }
            } else {
                let mut reliable_window: u16 = ((*incoming_command).reliable_sequence_number as i32
                    / PEER_RELIABLE_WINDOW_SIZE as i32)
                    as u16;
                let current_window: u16 = ((*channel).incoming_reliable_sequence_number as i32
                    / PEER_RELIABLE_WINDOW_SIZE as i32)
                    as u16;
                if ((*incoming_command).reliable_sequence_number as i32)
                    < (*channel).incoming_reliable_sequence_number as i32
                {
                    reliable_window =
                        (reliable_window as i32 + PEER_RELIABLE_WINDOWS as i32) as u16;
                }
                if reliable_window as i32 >= current_window as i32
                    && (reliable_window as i32)
                        < current_window as i32 + PEER_FREE_RELIABLE_WINDOWS as i32 - 1_i32
                {
                    break;
                }
                dropped_command = (*current_command).next;
                if start_command != current_command {
                    enet_list_move(
                        &mut (*peer).dispatched_commands.sentinel,
                        start_command.cast(),
                        (*current_command).previous.cast(),
                    );
                    if (*peer).flags as i32 & ENET_PEER_FLAG_NEEDS_DISPATCH as i32 == 0 {
                        enet_list_insert(
                            &mut (*(*peer).host).dispatch_queue.sentinel,
                            core::ptr::addr_of_mut!((*peer).dispatch_list).cast(),
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
                    start_command = (*current_command).next;
                }
            }
        }
        current_command = (*current_command).next;
    }
    if start_command != current_command {
        enet_list_move(
            &mut (*peer).dispatched_commands.sentinel,
            start_command.cast(),
            (*current_command).previous.cast(),
        );
        if (*peer).flags as i32 & ENET_PEER_FLAG_NEEDS_DISPATCH as i32 == 0 {
            enet_list_insert(
                &mut (*(*peer).host).dispatch_queue.sentinel,
                core::ptr::addr_of_mut!((*peer).dispatch_list).cast(),
            );
            (*peer).flags = ((*peer).flags as i32 | ENET_PEER_FLAG_NEEDS_DISPATCH as i32) as u16;
        }
        dropped_command = current_command;
    }
    enet_peer_remove_incoming_commands(
        &mut (*channel).incoming_unreliable_commands,
        (*channel).incoming_unreliable_commands.sentinel.next,
        dropped_command,
        queued_command,
    );
}
pub(crate) unsafe fn enet_peer_dispatch_incoming_reliable_commands<S: Socket>(
    peer: *mut ENetPeer<S>,
    channel: *mut ENetChannel,
    queued_command: *mut ENetIncomingCommand,
) {
    let mut current_command: ENetListIterator;
    current_command = (*channel).incoming_reliable_commands.sentinel.next;
    while current_command != core::ptr::addr_of_mut!((*channel).incoming_reliable_commands.sentinel)
    {
        let incoming_command: *mut ENetIncomingCommand = current_command.cast();
        if (*incoming_command).fragments_remaining > 0_i32 as u32
            || (*incoming_command).reliable_sequence_number as i32
                != ((*channel).incoming_reliable_sequence_number as i32 + 1_i32) as u16 as i32
        {
            break;
        }
        (*channel).incoming_reliable_sequence_number = (*incoming_command).reliable_sequence_number;
        if (*incoming_command).fragment_count > 0_i32 as u32 {
            (*channel).incoming_reliable_sequence_number =
                ((*channel).incoming_reliable_sequence_number as u32)
                    .wrapping_add(((*incoming_command).fragment_count).wrapping_sub(1_i32 as u32))
                    as u16 as u16;
        }
        current_command = (*current_command).next;
    }
    if current_command == (*channel).incoming_reliable_commands.sentinel.next {
        return;
    }
    (*channel).incoming_unreliable_sequence_number = 0_i32 as u16;
    enet_list_move(
        &mut (*peer).dispatched_commands.sentinel,
        ((*channel).incoming_reliable_commands.sentinel.next).cast(),
        ((*current_command).previous).cast(),
    );
    if (*peer).flags as i32 & ENET_PEER_FLAG_NEEDS_DISPATCH as i32 == 0 {
        enet_list_insert(
            &mut (*(*peer).host).dispatch_queue.sentinel,
            core::ptr::addr_of_mut!((*peer).dispatch_list).cast(),
        );
        (*peer).flags = ((*peer).flags as i32 | ENET_PEER_FLAG_NEEDS_DISPATCH as i32) as u16;
    }
    if (*channel).incoming_unreliable_commands.sentinel.next
        != core::ptr::addr_of_mut!((*channel).incoming_unreliable_commands.sentinel)
    {
        enet_peer_dispatch_incoming_unreliable_commands(peer, channel, queued_command);
    }
}
pub(crate) unsafe fn enet_peer_queue_incoming_command<S: Socket>(
    peer: *mut ENetPeer<S>,
    command: *const ENetProtocol,
    data: *const u8,
    data_length: usize,
    flags: u32,
    fragment_count: u32,
) -> *mut ENetIncomingCommand {
    static mut DUMMY_COMMAND: ENetIncomingCommand = ENetIncomingCommand {
        incoming_command_list: ENetListNode {
            next: 0 as *mut ENetListNode,
            previous: 0 as *mut ENetListNode,
        },
        reliable_sequence_number: 0,
        unreliable_sequence_number: 0,
        command: ENetProtocol {
            header: ENetProtocolCommandHeader {
                command: 0,
                channel_id: 0,
                reliable_sequence_number: 0,
            },
        },
        fragment_count: 0,
        fragments_remaining: 0,
        fragments: core::ptr::null_mut(),
        packet: core::ptr::null_mut(),
    };
    let mut current_block: u64;
    let channel: *mut ENetChannel =
        ((*peer).channels).offset((*command).header.channel_id as isize);
    let mut unreliable_sequence_number: u32 = 0_i32 as u32;
    let mut reliable_sequence_number: u32 = 0_i32 as u32;
    let mut reliable_window: u16;
    let current_window: u16;
    let mut incoming_command: *mut ENetIncomingCommand;
    let mut current_command: ENetListIterator = core::ptr::null_mut();
    let mut packet: *mut ENetPacket = core::ptr::null_mut();
    if (*peer).state == ENET_PEER_STATE_DISCONNECT_LATER as i32 as u32 {
        current_block = 9207730764507465628;
    } else {
        if (*command).header.command as i32 & ENET_PROTOCOL_COMMAND_MASK as i32
            != ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as i32
        {
            reliable_sequence_number = (*command).header.reliable_sequence_number as u32;
            reliable_window = reliable_sequence_number
                .wrapping_div(PEER_RELIABLE_WINDOW_SIZE as i32 as u32)
                as u16;
            current_window = ((*channel).incoming_reliable_sequence_number as i32
                / PEER_RELIABLE_WINDOW_SIZE as i32) as u16;
            if reliable_sequence_number < (*channel).incoming_reliable_sequence_number as u32 {
                reliable_window = (reliable_window as i32 + PEER_RELIABLE_WINDOWS as i32) as u16;
            }
            if (reliable_window as i32) < current_window as i32
                || reliable_window as i32
                    >= current_window as i32 + PEER_FREE_RELIABLE_WINDOWS as i32 - 1_i32
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
                            current_command = &mut (*channel).incoming_unreliable_commands.sentinel;
                            current_block = 7746103178988627676;
                        }
                        4379360700607281851 => {
                            if reliable_sequence_number
                                == (*channel).incoming_reliable_sequence_number as u32
                            {
                                current_block = 9207730764507465628;
                            } else {
                                current_command =
                                    (*channel).incoming_reliable_commands.sentinel.previous;
                                loop {
                                    if current_command
                                        == core::ptr::addr_of_mut!(
                                            (*channel).incoming_reliable_commands.sentinel
                                        )
                                    {
                                        current_block = 7746103178988627676;
                                        break;
                                    }
                                    incoming_command = current_command.cast();
                                    if reliable_sequence_number
                                        >= (*channel).incoming_reliable_sequence_number as u32
                                    {
                                        if ((*incoming_command).reliable_sequence_number as i32)
                                            < (*channel).incoming_reliable_sequence_number as i32
                                        {
                                            current_block = 1856101646708284338;
                                        } else {
                                            current_block = 8457315219000651999;
                                        }
                                    } else {
                                        if (*incoming_command).reliable_sequence_number as i32
                                            >= (*channel).incoming_reliable_sequence_number as i32
                                        {
                                            current_block = 7746103178988627676;
                                            break;
                                        }
                                        current_block = 8457315219000651999;
                                    }
                                    if let 8457315219000651999 = current_block {
                                        if (*incoming_command).reliable_sequence_number as u32
                                            <= reliable_sequence_number
                                        {
                                            if ((*incoming_command).reliable_sequence_number as u32)
                                                < reliable_sequence_number
                                            {
                                                current_block = 7746103178988627676;
                                                break;
                                            } else {
                                                current_block = 9207730764507465628;
                                                break;
                                            }
                                        }
                                    }
                                    current_command = (*current_command).previous;
                                }
                            }
                        }
                        _ => {
                            unreliable_sequence_number =
                                u16::from_be((*command).send_unreliable.unreliable_sequence_number)
                                    as u32;
                            if reliable_sequence_number
                                == (*channel).incoming_reliable_sequence_number as u32
                                && unreliable_sequence_number
                                    <= (*channel).incoming_unreliable_sequence_number as u32
                            {
                                current_block = 9207730764507465628;
                            } else {
                                current_command =
                                    (*channel).incoming_unreliable_commands.sentinel.previous;
                                loop {
                                    if current_command
                                        == core::ptr::addr_of_mut!(
                                            (*channel).incoming_unreliable_commands.sentinel
                                        )
                                    {
                                        current_block = 7746103178988627676;
                                        break;
                                    }
                                    incoming_command = current_command.cast();
                                    if (*command).header.command as i32
                                        & ENET_PROTOCOL_COMMAND_MASK as i32
                                        != ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as i32
                                    {
                                        if reliable_sequence_number
                                            >= (*channel).incoming_reliable_sequence_number as u32
                                        {
                                            if ((*incoming_command).reliable_sequence_number as i32)
                                                < (*channel).incoming_reliable_sequence_number
                                                    as i32
                                            {
                                                current_block = 17478428563724192186;
                                            } else {
                                                current_block = 11459959175219260272;
                                            }
                                        } else {
                                            if (*incoming_command).reliable_sequence_number as i32
                                                >= (*channel).incoming_reliable_sequence_number
                                                    as i32
                                            {
                                                current_block = 7746103178988627676;
                                                break;
                                            }
                                            current_block = 11459959175219260272;
                                        }
                                        if current_block != 17478428563724192186 {
                                            if ((*incoming_command).reliable_sequence_number as u32)
                                                < reliable_sequence_number
                                            {
                                                current_block = 7746103178988627676;
                                                break;
                                            }
                                            if (*incoming_command).reliable_sequence_number as u32
                                                <= reliable_sequence_number
                                                && (*incoming_command).unreliable_sequence_number
                                                    as u32
                                                    <= unreliable_sequence_number
                                            {
                                                if ((*incoming_command).unreliable_sequence_number
                                                    as u32)
                                                    < unreliable_sequence_number
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
                                    current_command = (*current_command).previous;
                                }
                            }
                        }
                    }
                    match current_block {
                        9207730764507465628 => {}
                        _ => {
                            if (*peer).total_waiting_data >= (*(*peer).host).maximum_waiting_data {
                                current_block = 15492018734234176694;
                            } else {
                                packet = enet_packet_create(data, data_length, flags);
                                if packet.is_null() {
                                    current_block = 15492018734234176694;
                                } else {
                                    incoming_command =
                                        enet_malloc(Layout::new::<ENetIncomingCommand>()).cast();
                                    if incoming_command.is_null() {
                                        current_block = 15492018734234176694;
                                    } else {
                                        (*incoming_command).reliable_sequence_number =
                                            (*command).header.reliable_sequence_number;
                                        (*incoming_command).unreliable_sequence_number =
                                            (unreliable_sequence_number & 0xffff_i32 as u32) as u16;
                                        (*incoming_command).command = *command;
                                        (*incoming_command).fragment_count = fragment_count;
                                        (*incoming_command).fragments_remaining = fragment_count;
                                        (*incoming_command).packet = packet;
                                        (*incoming_command).fragments = core::ptr::null_mut();
                                        if fragment_count > 0_i32 as u32 {
                                            if fragment_count
                                                <= PROTOCOL_MAXIMUM_FRAGMENT_COUNT as i32 as u32
                                            {
                                                let count = fragment_count
                                                    .wrapping_add(31_i32 as u32)
                                                    .wrapping_div(32_i32 as u32)
                                                    as usize;
                                                (*incoming_command).fragments = enet_malloc(
                                                    Layout::array::<u32>(count).unwrap(),
                                                )
                                                .cast();
                                            }
                                            if ((*incoming_command).fragments).is_null() {
                                                enet_free(
                                                    incoming_command.cast(),
                                                    Layout::new::<ENetIncomingCommand>(),
                                                );
                                                current_block = 15492018734234176694;
                                            } else {
                                                write_bytes(
                                                    (*incoming_command).fragments.cast::<u8>(),
                                                    0,
                                                    (fragment_count
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
                                        if current_block != 15492018734234176694 {
                                            if !packet.is_null() {
                                                (*packet).reference_count =
                                                    ((*packet).reference_count).wrapping_add(1);
                                                (*peer).total_waiting_data = (*peer)
                                                    .total_waiting_data
                                                    .wrapping_add((*packet).data_length);
                                            }
                                            enet_list_insert(
                                                (*current_command).next,
                                                incoming_command.cast(),
                                            );
                                            match (*command).header.command as i32
                                                & ENET_PROTOCOL_COMMAND_MASK as i32
                                            {
                                                8 | 6 => {
                                                    enet_peer_dispatch_incoming_reliable_commands(
                                                        peer,
                                                        channel,
                                                        incoming_command,
                                                    );
                                                }
                                                _ => {
                                                    enet_peer_dispatch_incoming_unreliable_commands(
                                                        peer,
                                                        channel,
                                                        incoming_command,
                                                    );
                                                }
                                            }
                                            return incoming_command;
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
                            current_command = &mut (*channel).incoming_unreliable_commands.sentinel;
                            current_block = 7746103178988627676;
                        }
                        4379360700607281851 => {
                            if reliable_sequence_number
                                == (*channel).incoming_reliable_sequence_number as u32
                            {
                                current_block = 9207730764507465628;
                            } else {
                                current_command =
                                    (*channel).incoming_reliable_commands.sentinel.previous;
                                loop {
                                    if current_command
                                        == core::ptr::addr_of_mut!(
                                            (*channel).incoming_reliable_commands.sentinel
                                        )
                                    {
                                        current_block = 7746103178988627676;
                                        break;
                                    }
                                    incoming_command = current_command.cast();
                                    if reliable_sequence_number
                                        >= (*channel).incoming_reliable_sequence_number as u32
                                    {
                                        if ((*incoming_command).reliable_sequence_number as i32)
                                            < (*channel).incoming_reliable_sequence_number as i32
                                        {
                                            current_block = 1856101646708284338;
                                        } else {
                                            current_block = 8457315219000651999;
                                        }
                                    } else {
                                        if (*incoming_command).reliable_sequence_number as i32
                                            >= (*channel).incoming_reliable_sequence_number as i32
                                        {
                                            current_block = 7746103178988627676;
                                            break;
                                        }
                                        current_block = 8457315219000651999;
                                    }
                                    if let 8457315219000651999 = current_block {
                                        if (*incoming_command).reliable_sequence_number as u32
                                            <= reliable_sequence_number
                                        {
                                            if ((*incoming_command).reliable_sequence_number as u32)
                                                < reliable_sequence_number
                                            {
                                                current_block = 7746103178988627676;
                                                break;
                                            } else {
                                                current_block = 9207730764507465628;
                                                break;
                                            }
                                        }
                                    }
                                    current_command = (*current_command).previous;
                                }
                            }
                        }
                        _ => {
                            unreliable_sequence_number =
                                u16::from_be((*command).send_unreliable.unreliable_sequence_number)
                                    as u32;
                            if reliable_sequence_number
                                == (*channel).incoming_reliable_sequence_number as u32
                                && unreliable_sequence_number
                                    <= (*channel).incoming_unreliable_sequence_number as u32
                            {
                                current_block = 9207730764507465628;
                            } else {
                                current_command =
                                    (*channel).incoming_unreliable_commands.sentinel.previous;
                                loop {
                                    if current_command
                                        == core::ptr::addr_of_mut!(
                                            (*channel).incoming_unreliable_commands.sentinel
                                        )
                                    {
                                        current_block = 7746103178988627676;
                                        break;
                                    }
                                    incoming_command = current_command.cast();
                                    if (*command).header.command as i32
                                        & ENET_PROTOCOL_COMMAND_MASK as i32
                                        != ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as i32
                                    {
                                        if reliable_sequence_number
                                            >= (*channel).incoming_reliable_sequence_number as u32
                                        {
                                            if ((*incoming_command).reliable_sequence_number as i32)
                                                < (*channel).incoming_reliable_sequence_number
                                                    as i32
                                            {
                                                current_block = 17478428563724192186;
                                            } else {
                                                current_block = 11459959175219260272;
                                            }
                                        } else {
                                            if (*incoming_command).reliable_sequence_number as i32
                                                >= (*channel).incoming_reliable_sequence_number
                                                    as i32
                                            {
                                                current_block = 7746103178988627676;
                                                break;
                                            }
                                            current_block = 11459959175219260272;
                                        }
                                        if current_block != 17478428563724192186 {
                                            if ((*incoming_command).reliable_sequence_number as u32)
                                                < reliable_sequence_number
                                            {
                                                current_block = 7746103178988627676;
                                                break;
                                            }
                                            if (*incoming_command).reliable_sequence_number as u32
                                                <= reliable_sequence_number
                                                && (*incoming_command).unreliable_sequence_number
                                                    as u32
                                                    <= unreliable_sequence_number
                                            {
                                                if ((*incoming_command).unreliable_sequence_number
                                                    as u32)
                                                    < unreliable_sequence_number
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
                                    current_command = (*current_command).previous;
                                }
                            }
                        }
                    }
                    match current_block {
                        9207730764507465628 => {}
                        _ => {
                            if (*peer).total_waiting_data >= (*(*peer).host).maximum_waiting_data {
                                current_block = 15492018734234176694;
                            } else {
                                packet = enet_packet_create(data, data_length, flags);
                                if packet.is_null() {
                                    current_block = 15492018734234176694;
                                } else {
                                    incoming_command =
                                        enet_malloc(Layout::new::<ENetIncomingCommand>()).cast();
                                    if incoming_command.is_null() {
                                        current_block = 15492018734234176694;
                                    } else {
                                        (*incoming_command).reliable_sequence_number =
                                            (*command).header.reliable_sequence_number;
                                        (*incoming_command).unreliable_sequence_number =
                                            (unreliable_sequence_number & 0xffff_i32 as u32) as u16;
                                        (*incoming_command).command = *command;
                                        (*incoming_command).fragment_count = fragment_count;
                                        (*incoming_command).fragments_remaining = fragment_count;
                                        (*incoming_command).packet = packet;
                                        (*incoming_command).fragments = core::ptr::null_mut();
                                        if fragment_count > 0_i32 as u32 {
                                            if fragment_count
                                                <= PROTOCOL_MAXIMUM_FRAGMENT_COUNT as i32 as u32
                                            {
                                                let count = fragment_count
                                                    .wrapping_add(31_i32 as u32)
                                                    .wrapping_div(32_i32 as u32)
                                                    as usize;
                                                (*incoming_command).fragments = enet_malloc(
                                                    Layout::array::<u32>(count).unwrap(),
                                                )
                                                .cast();
                                            }
                                            if ((*incoming_command).fragments).is_null() {
                                                enet_free(
                                                    incoming_command.cast(),
                                                    Layout::new::<ENetIncomingCommand>(),
                                                );
                                                current_block = 15492018734234176694;
                                            } else {
                                                write_bytes(
                                                    (*incoming_command).fragments.cast::<u8>(),
                                                    0,
                                                    (fragment_count
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
                                        if current_block != 15492018734234176694 {
                                            if !packet.is_null() {
                                                (*packet).reference_count =
                                                    ((*packet).reference_count).wrapping_add(1);
                                                (*peer).total_waiting_data = (*peer)
                                                    .total_waiting_data
                                                    .wrapping_add((*packet).data_length);
                                            }
                                            enet_list_insert(
                                                (*current_command).next,
                                                incoming_command.cast(),
                                            );
                                            match (*command).header.command as i32
                                                & ENET_PROTOCOL_COMMAND_MASK as i32
                                            {
                                                8 | 6 => {
                                                    enet_peer_dispatch_incoming_reliable_commands(
                                                        peer,
                                                        channel,
                                                        incoming_command,
                                                    );
                                                }
                                                _ => {
                                                    enet_peer_dispatch_incoming_unreliable_commands(
                                                        peer,
                                                        channel,
                                                        incoming_command,
                                                    );
                                                }
                                            }
                                            return incoming_command;
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
                            current_command = &mut (*channel).incoming_unreliable_commands.sentinel;
                            current_block = 7746103178988627676;
                        }
                        4379360700607281851 => {
                            if reliable_sequence_number
                                == (*channel).incoming_reliable_sequence_number as u32
                            {
                                current_block = 9207730764507465628;
                            } else {
                                current_command =
                                    (*channel).incoming_reliable_commands.sentinel.previous;
                                loop {
                                    if current_command
                                        == core::ptr::addr_of_mut!(
                                            (*channel).incoming_reliable_commands.sentinel
                                        )
                                    {
                                        current_block = 7746103178988627676;
                                        break;
                                    }
                                    incoming_command = current_command.cast();
                                    if reliable_sequence_number
                                        >= (*channel).incoming_reliable_sequence_number as u32
                                    {
                                        if ((*incoming_command).reliable_sequence_number as i32)
                                            < (*channel).incoming_reliable_sequence_number as i32
                                        {
                                            current_block = 1856101646708284338;
                                        } else {
                                            current_block = 8457315219000651999;
                                        }
                                    } else {
                                        if (*incoming_command).reliable_sequence_number as i32
                                            >= (*channel).incoming_reliable_sequence_number as i32
                                        {
                                            current_block = 7746103178988627676;
                                            break;
                                        }
                                        current_block = 8457315219000651999;
                                    }
                                    if let 8457315219000651999 = current_block {
                                        if (*incoming_command).reliable_sequence_number as u32
                                            <= reliable_sequence_number
                                        {
                                            if ((*incoming_command).reliable_sequence_number as u32)
                                                < reliable_sequence_number
                                            {
                                                current_block = 7746103178988627676;
                                                break;
                                            } else {
                                                current_block = 9207730764507465628;
                                                break;
                                            }
                                        }
                                    }
                                    current_command = (*current_command).previous;
                                }
                            }
                        }
                        _ => {
                            unreliable_sequence_number =
                                u16::from_be((*command).send_unreliable.unreliable_sequence_number)
                                    as u32;
                            if reliable_sequence_number
                                == (*channel).incoming_reliable_sequence_number as u32
                                && unreliable_sequence_number
                                    <= (*channel).incoming_unreliable_sequence_number as u32
                            {
                                current_block = 9207730764507465628;
                            } else {
                                current_command =
                                    (*channel).incoming_unreliable_commands.sentinel.previous;
                                loop {
                                    if current_command
                                        == core::ptr::addr_of_mut!(
                                            (*channel).incoming_unreliable_commands.sentinel
                                        )
                                    {
                                        current_block = 7746103178988627676;
                                        break;
                                    }
                                    incoming_command = current_command.cast();
                                    if (*command).header.command as i32
                                        & ENET_PROTOCOL_COMMAND_MASK as i32
                                        != ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as i32
                                    {
                                        if reliable_sequence_number
                                            >= (*channel).incoming_reliable_sequence_number as u32
                                        {
                                            if ((*incoming_command).reliable_sequence_number as i32)
                                                < (*channel).incoming_reliable_sequence_number
                                                    as i32
                                            {
                                                current_block = 17478428563724192186;
                                            } else {
                                                current_block = 11459959175219260272;
                                            }
                                        } else {
                                            if (*incoming_command).reliable_sequence_number as i32
                                                >= (*channel).incoming_reliable_sequence_number
                                                    as i32
                                            {
                                                current_block = 7746103178988627676;
                                                break;
                                            }
                                            current_block = 11459959175219260272;
                                        }
                                        if current_block != 17478428563724192186 {
                                            if ((*incoming_command).reliable_sequence_number as u32)
                                                < reliable_sequence_number
                                            {
                                                current_block = 7746103178988627676;
                                                break;
                                            }
                                            if (*incoming_command).reliable_sequence_number as u32
                                                <= reliable_sequence_number
                                                && (*incoming_command).unreliable_sequence_number
                                                    as u32
                                                    <= unreliable_sequence_number
                                            {
                                                if ((*incoming_command).unreliable_sequence_number
                                                    as u32)
                                                    < unreliable_sequence_number
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
                                    current_command = (*current_command).previous;
                                }
                            }
                        }
                    }
                    match current_block {
                        9207730764507465628 => {}
                        _ => {
                            if (*peer).total_waiting_data >= (*(*peer).host).maximum_waiting_data {
                                current_block = 15492018734234176694;
                            } else {
                                packet = enet_packet_create(data, data_length, flags);
                                if packet.is_null() {
                                    current_block = 15492018734234176694;
                                } else {
                                    incoming_command =
                                        enet_malloc(Layout::new::<ENetIncomingCommand>()).cast();
                                    if incoming_command.is_null() {
                                        current_block = 15492018734234176694;
                                    } else {
                                        (*incoming_command).reliable_sequence_number =
                                            (*command).header.reliable_sequence_number;
                                        (*incoming_command).unreliable_sequence_number =
                                            (unreliable_sequence_number & 0xffff_i32 as u32) as u16;
                                        (*incoming_command).command = *command;
                                        (*incoming_command).fragment_count = fragment_count;
                                        (*incoming_command).fragments_remaining = fragment_count;
                                        (*incoming_command).packet = packet;
                                        (*incoming_command).fragments = core::ptr::null_mut();
                                        if fragment_count > 0_i32 as u32 {
                                            if fragment_count
                                                <= PROTOCOL_MAXIMUM_FRAGMENT_COUNT as i32 as u32
                                            {
                                                let count = fragment_count
                                                    .wrapping_add(31_i32 as u32)
                                                    .wrapping_div(32_i32 as u32)
                                                    as usize;
                                                (*incoming_command).fragments = enet_malloc(
                                                    Layout::array::<u32>(count).unwrap(),
                                                )
                                                .cast();
                                            }
                                            if ((*incoming_command).fragments).is_null() {
                                                enet_free(
                                                    incoming_command.cast(),
                                                    Layout::new::<ENetIncomingCommand>(),
                                                );
                                                current_block = 15492018734234176694;
                                            } else {
                                                write_bytes(
                                                    (*incoming_command).fragments.cast::<u8>(),
                                                    0,
                                                    (fragment_count
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
                                        if current_block != 15492018734234176694 {
                                            if !packet.is_null() {
                                                (*packet).reference_count =
                                                    ((*packet).reference_count).wrapping_add(1);
                                                (*peer).total_waiting_data = (*peer)
                                                    .total_waiting_data
                                                    .wrapping_add((*packet).data_length);
                                            }
                                            enet_list_insert(
                                                (*current_command).next,
                                                incoming_command.cast(),
                                            );
                                            match (*command).header.command as i32
                                                & ENET_PROTOCOL_COMMAND_MASK as i32
                                            {
                                                8 | 6 => {
                                                    enet_peer_dispatch_incoming_reliable_commands(
                                                        peer,
                                                        channel,
                                                        incoming_command,
                                                    );
                                                }
                                                _ => {
                                                    enet_peer_dispatch_incoming_unreliable_commands(
                                                        peer,
                                                        channel,
                                                        incoming_command,
                                                    );
                                                }
                                            }
                                            return incoming_command;
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
        if fragment_count <= 0_i32 as u32 {
            if !packet.is_null() && (*packet).reference_count == 0_i32 as usize {
                enet_packet_destroy(packet);
            }
            return addr_of_mut!(DUMMY_COMMAND);
        }
    }
    if !packet.is_null() && (*packet).reference_count == 0_i32 as usize {
        enet_packet_destroy(packet);
    }
    core::ptr::null_mut()
}
