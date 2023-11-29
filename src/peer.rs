use std::{collections::VecDeque, mem::size_of};

use crate::{
    c_void, enet_free, enet_host_flush, enet_list_begin, enet_list_clear, enet_list_empty,
    enet_list_end, enet_list_insert, enet_list_move, enet_list_next, enet_list_previous,
    enet_list_remove, enet_malloc, enet_memset, enet_packet_create, enet_packet_destroy,
    enet_protocol_command_size, Channel, ENetIncomingCommand, ENetList, ENetListIterator,
    ENetListNode, ENetOutgoingCommand, ENetPacket, ENetProtocol, ENetProtocolAcknowledge,
    ENetProtocolCommandHeader, ENetProtocolHeader, ENetProtocolSendFragment, Host, Packet,
    PacketFlag, Socket, ENET_PEER_DEFAULT_PACKET_THROTTLE, ENET_PEER_DEFAULT_ROUND_TRIP_TIME,
    ENET_PEER_FREE_RELIABLE_WINDOWS, ENET_PEER_PACKET_THROTTLE_ACCELERATION,
    ENET_PEER_PACKET_THROTTLE_DECELERATION, ENET_PEER_PACKET_THROTTLE_INTERVAL,
    ENET_PEER_PACKET_THROTTLE_SCALE, ENET_PEER_PING_INTERVAL, ENET_PEER_RELIABLE_WINDOWS,
    ENET_PEER_RELIABLE_WINDOW_SIZE, ENET_PEER_TIMEOUT_LIMIT, ENET_PEER_TIMEOUT_MAXIMUM,
    ENET_PEER_TIMEOUT_MINIMUM, ENET_PEER_UNSEQUENCED_WINDOW_SIZE, ENET_PROTOCOL_COMMAND_DISCONNECT,
    ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE, ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED,
    ENET_PROTOCOL_COMMAND_MASK, ENET_PROTOCOL_COMMAND_PING, ENET_PROTOCOL_COMMAND_SEND_FRAGMENT,
    ENET_PROTOCOL_COMMAND_SEND_RELIABLE, ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE,
    ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT, ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED,
    ENET_PROTOCOL_COMMAND_THROTTLE_CONFIGURE, ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT,
    ENET_PROTOCOL_MAXIMUM_PEER_ID, ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PeerState {
    Zombie,
    AcknowledgingDisconnect,
    Disconnecting,
    DisconnectLater,
    Connected,
    ConnectionSucceeded,
    ConnectionPending,
    AcknowledgingConnect,
    Connecting,
    Disconnected,
}

pub(crate) const ENET_PEER_FLAG_CONTINUE_SENDING: u16 = 2;
pub(crate) const ENET_PEER_FLAG_NEEDS_DISPATCH: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PeerID(pub usize);

pub(crate) struct Peer<S: Socket> {
    pub(crate) index: PeerID,
    pub(crate) outgoing_peer_id: u16,
    pub(crate) incoming_peer_id: u16,
    pub(crate) connect_id: u32,
    pub(crate) outgoing_session_id: u8,
    pub(crate) incoming_session_id: u8,
    pub(crate) address: Option<S::PeerAddress>,
    pub(crate) state: PeerState,
    pub(crate) channels: Vec<Channel>,
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
    pub(crate) acknowledgements: VecDeque<ENetAcknowledgement>,
    pub(crate) sent_reliable_commands: ENetList<ENetOutgoingCommand>,
    pub(crate) outgoing_send_reliable_commands: ENetList<ENetOutgoingCommand>,
    pub(crate) outgoing_commands: ENetList<ENetOutgoingCommand>,
    pub(crate) dispatched_commands: ENetList<ENetIncomingCommand>,
    pub(crate) flags: u16,
    pub(crate) _reserved: u16,
    pub(crate) incoming_unsequenced_group: u16,
    pub(crate) outgoing_unsequenced_group: u16,
    pub(crate) unsequenced_window: [u32; ENET_PEER_UNSEQUENCED_WINDOW_SIZE / 32],
    pub(crate) event_data: u32,
    pub(crate) total_waiting_data: usize,
}

impl<S: Socket> Default for Peer<S> {
    fn default() -> Self {
        Self {
            outgoing_peer_id: 0,
            incoming_peer_id: 0,
            connect_id: 0,
            outgoing_session_id: 0,
            incoming_session_id: 0,
            address: None,
            state: PeerState::Disconnected,
            channels: vec![],
            channel_count: 0,
            incoming_bandwidth: 0,
            outgoing_bandwidth: 0,
            incoming_bandwidth_throttle_epoch: 0,
            outgoing_bandwidth_throttle_epoch: 0,
            incoming_data_total: 0,
            outgoing_data_total: 0,
            last_send_time: 0,
            last_receive_time: 0,
            next_timeout: 0,
            earliest_timeout: 0,
            packet_loss_epoch: 0,
            packets_sent: 0,
            packets_lost: 0,
            packet_loss: 0,
            packet_loss_variance: 0,
            packet_throttle: 0,
            packet_throttle_limit: 0,
            packet_throttle_counter: 0,
            packet_throttle_epoch: 0,
            packet_throttle_acceleration: 0,
            packet_throttle_deceleration: 0,
            packet_throttle_interval: 0,
            ping_interval: 0,
            timeout_limit: 0,
            timeout_minimum: 0,
            timeout_maximum: 0,
            last_round_trip_time: 0,
            lowest_round_trip_time: 0,
            last_round_trip_time_variance: 0,
            highest_round_trip_time_variance: 0,
            round_trip_time: 0,
            round_trip_time_variance: 0,
            mtu: 0,
            window_size: 0,
            reliable_data_in_transit: 0,
            outgoing_reliable_sequence_number: 0,
            acknowledgements: VecDeque::new(),
            sent_reliable_commands: ENetList::default(),
            outgoing_send_reliable_commands: ENetList::default(),
            outgoing_commands: ENetList::default(),
            dispatched_commands: ENetList::default(),
            flags: 0,
            _reserved: 0,
            incoming_unsequenced_group: 0,
            outgoing_unsequenced_group: 0,
            unsequenced_window: [0; ENET_PEER_UNSEQUENCED_WINDOW_SIZE / 32],
            event_data: 0,
            total_waiting_data: 0,
            index: PeerID(0),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetAcknowledgement {
    pub sent_time: u32,
    pub command: ENetProtocol,
}

pub(crate) fn enet_peer_throttle_configure<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    interval: u32,
    acceleration: u32,
    deceleration: u32,
) {
    let mut command = ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channel_id: 0,
            reliable_sequence_number: 0,
        },
    };
    peer.packet_throttle_interval = interval;
    peer.packet_throttle_acceleration = acceleration;
    peer.packet_throttle_deceleration = deceleration;
    command.header.command =
        ENET_PROTOCOL_COMMAND_THROTTLE_CONFIGURE | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE;
    command.header.channel_id = 0xff;
    command.throttle_configure.packet_throttle_interval = interval.to_be();
    command.throttle_configure.packet_throttle_acceleration = acceleration.to_be();
    command.throttle_configure.packet_throttle_deceleration = deceleration.to_be();
    unsafe {
        enet_peer_queue_outgoing_command(host, peer, &command, std::ptr::null_mut(), 0, 0);
    }
}

pub(crate) fn enet_peer_throttle<S: Socket>(peer: &mut Peer<S>, rtt: u32) -> i32 {
    if peer.last_round_trip_time <= peer.last_round_trip_time_variance {
        peer.packet_throttle = peer.packet_throttle_limit;
    } else if rtt <= peer.last_round_trip_time {
        peer.packet_throttle += peer.packet_throttle_acceleration;
        if peer.packet_throttle > peer.packet_throttle_limit {
            peer.packet_throttle = peer.packet_throttle_limit;
        }
        return 1;
    } else if rtt > (peer.last_round_trip_time + 2) * peer.last_round_trip_time_variance {
        if peer.packet_throttle > peer.packet_throttle_deceleration {
            peer.packet_throttle -= peer.packet_throttle_deceleration;
        } else {
            peer.packet_throttle = 0;
        }
        return -1;
    }
    0
}

pub(crate) fn enet_peer_send<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    channel_id: u8,
    packet: Packet,
) -> Result<(), crate::Error> {
    unsafe {
        let mut command = ENetProtocol {
            header: ENetProtocolCommandHeader {
                command: 0,
                channel_id: 0,
                reliable_sequence_number: 0,
            },
        };
        let mut fragment_length: usize;
        if peer.state != PeerState::Connected {
            return Err(crate::Error::PeerNotConnected);
        }
        if channel_id as usize >= peer.channel_count {
            return Err(crate::Error::InvalidValueForParameter {
                param: "channel_id",
            });
        }
        if (*packet.packet).data_length > host.maximum_packet_size {
            return Err(crate::Error::PacketTooLarge);
        }
        let channel =
            &mut *(peer.channels.as_mut_ptr()).offset(channel_id as isize) as *mut Channel;
        fragment_length = (peer.mtu as usize)
            - size_of::<ENetProtocolHeader>()
            - size_of::<ENetProtocolSendFragment>();
        if (*packet.packet).data_length > fragment_length {
            let fragment_count: u32 =
                (((*packet.packet).data_length + fragment_length - 1) / fragment_length) as u32;
            let mut fragment_number: u32;
            let mut fragment_offset: u32;
            let command_number: u8;
            let start_sequence_number: u16;
            let mut fragments: ENetList<ENetOutgoingCommand> = ENetList::default();
            let mut fragment: *mut ENetOutgoingCommand;
            if fragment_count > ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT {
                return Err(crate::Error::TooManyFragments);
            }
            if !(*packet.packet).flags.contains(PacketFlag::RELIABLE)
                && (*packet.packet)
                    .flags
                    .contains(PacketFlag::UNRELIABLE_FRAGMENT)
                && ((*channel).outgoing_unreliable_sequence_number) < 0xffff
            {
                command_number = ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT;
                start_sequence_number =
                    ((*channel).outgoing_unreliable_sequence_number + 1).to_be();
            } else {
                command_number =
                    ENET_PROTOCOL_COMMAND_SEND_FRAGMENT | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE;
                start_sequence_number = ((*channel).outgoing_reliable_sequence_number + 1).to_be();
            }
            enet_list_clear(&mut fragments);
            fragment_number = 0;
            fragment_offset = 0;
            while (fragment_offset as usize) < (*packet.packet).data_length {
                if (*packet.packet).data_length - (fragment_offset as usize) < fragment_length {
                    fragment_length = (*packet.packet).data_length - (fragment_offset as usize);
                }
                fragment =
                    enet_malloc(size_of::<ENetOutgoingCommand>()) as *mut ENetOutgoingCommand;
                if fragment.is_null() {
                    while !enet_list_empty(&mut fragments) {
                        fragment = enet_list_remove(enet_list_begin(&mut fragments));
                        enet_free(fragment as *mut c_void);
                    }
                    panic!("malloc() failed");
                }
                (*fragment).fragment_offset = fragment_offset;
                (*fragment).fragment_length = fragment_length as u16;
                (*fragment).packet = packet.packet;
                (*fragment).command.header.command = command_number;
                (*fragment).command.header.channel_id = channel_id;
                (*fragment).command.send_fragment.start_sequence_number = start_sequence_number;
                (*fragment).command.send_fragment.data_length = (fragment_length as u16).to_be();
                (*fragment).command.send_fragment.fragment_count = fragment_count.to_be();
                (*fragment).command.send_fragment.fragment_number = fragment_number.to_be();
                (*fragment).command.send_fragment.total_length =
                    ((*packet.packet).data_length as u32).to_be();
                (*fragment).command.send_fragment.fragment_offset = u32::from_be(fragment_offset);
                enet_list_insert(
                    enet_list_end(&mut fragments),
                    fragment as *mut ENetListNode<ENetOutgoingCommand>,
                );
                fragment_number += 1;
                fragment_offset += fragment_length as u32;
            }
            (*packet.packet).reference_count += fragment_number as usize;
            while !enet_list_empty(&mut fragments) {
                fragment = enet_list_remove(enet_list_begin(&mut fragments));
                enet_peer_setup_outgoing_command(host, peer, fragment);
            }
            return Ok(());
        }
        command.header.channel_id = channel_id;
        if !(*packet.packet).flags.contains(PacketFlag::RELIABLE)
            && (*packet.packet).flags.contains(PacketFlag::UNSEQUENCED)
        {
            command.header.command =
                ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED | ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED;
            command.send_unsequenced.data_length = ((*packet.packet).data_length as u16).to_be();
        } else if (*packet.packet).flags.contains(PacketFlag::RELIABLE)
            || (*channel).outgoing_unreliable_sequence_number == 0xffff
        {
            command.header.command =
                ENET_PROTOCOL_COMMAND_SEND_RELIABLE | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE;
            command.send_reliable.data_length = ((*packet.packet).data_length as u16).to_be();
        } else {
            command.header.command = ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE;
            command.send_unreliable.data_length = ((*packet.packet).data_length as u16).to_be();
        }
        if (enet_peer_queue_outgoing_command(
            host,
            peer,
            &command,
            packet.packet,
            0,
            (*packet.packet).data_length as u16,
        ))
        .is_null()
        {
            Err(crate::Error::Unknown)
        } else {
            Ok(())
        }
    }
}

pub(crate) unsafe fn enet_peer_receive<S: Socket>(
    peer: &mut Peer<S>,
    channel_id: *mut u8,
) -> *mut ENetPacket {
    if enet_list_empty(&mut peer.dispatched_commands) {
        return std::ptr::null_mut();
    }
    let incoming_command = enet_list_remove(enet_list_begin(&mut peer.dispatched_commands));
    if !channel_id.is_null() {
        *channel_id = (*incoming_command).command.header.channel_id;
    }
    let packet = (*incoming_command).packet;
    (*packet).reference_count -= 1;
    if !((*incoming_command).fragments).is_null() {
        enet_free((*incoming_command).fragments as *mut c_void);
    }
    enet_free(incoming_command as *mut c_void);
    peer.total_waiting_data -= (*packet).data_length;
    packet
}
unsafe fn enet_peer_reset_outgoing_commands(queue: *mut ENetList<ENetOutgoingCommand>) {
    let mut outgoing_command: *mut ENetOutgoingCommand;
    while !enet_list_empty(queue) {
        outgoing_command = enet_list_remove(enet_list_begin(queue));
        if !((*outgoing_command).packet).is_null() {
            (*(*outgoing_command).packet).reference_count -= 1;
            if (*(*outgoing_command).packet).reference_count == 0 {
                enet_packet_destroy((*outgoing_command).packet);
            }
        }
        enet_free(outgoing_command as *mut c_void);
    }
}
unsafe fn enet_peer_remove_incoming_commands(
    mut _queue: *mut ENetList<ENetIncomingCommand>,
    start_command: ENetListIterator<ENetIncomingCommand>,
    end_command: ENetListIterator<ENetIncomingCommand>,
    exclude_command: *mut ENetIncomingCommand,
) {
    let mut current_command = start_command;
    while current_command != end_command {
        let incoming_command: *mut ENetIncomingCommand =
            current_command as *mut ENetIncomingCommand;
        current_command = enet_list_next(current_command);
        if incoming_command == exclude_command {
            continue;
        }
        enet_list_remove(&mut (*incoming_command).incoming_command_list);
        if !((*incoming_command).packet).is_null() {
            (*(*incoming_command).packet).reference_count -= 1;
            if (*(*incoming_command).packet).reference_count == 0 {
                enet_packet_destroy((*incoming_command).packet);
            }
        }
        if !((*incoming_command).fragments).is_null() {
            enet_free((*incoming_command).fragments as *mut c_void);
        }
        enet_free(incoming_command as *mut c_void);
    }
}

unsafe fn enet_peer_reset_incoming_commands(queue: *mut ENetList<ENetIncomingCommand>) {
    enet_peer_remove_incoming_commands(
        queue,
        enet_list_begin(queue),
        enet_list_end(queue),
        std::ptr::null_mut(),
    );
}

pub(crate) unsafe fn enet_peer_reset_queues<S: Socket>(host: &mut Host<S>, peer: &mut Peer<S>) {
    let mut channel: *mut Channel;
    if peer.flags & ENET_PEER_FLAG_NEEDS_DISPATCH != 0 {
        host.dispatch_queue.retain(|peer_id| *peer_id != peer.index);
        peer.flags |= !ENET_PEER_FLAG_NEEDS_DISPATCH;
    }
    peer.acknowledgements.clear();
    enet_peer_reset_outgoing_commands(&mut peer.sent_reliable_commands);
    enet_peer_reset_outgoing_commands(&mut peer.outgoing_commands);
    enet_peer_reset_outgoing_commands(&mut peer.outgoing_send_reliable_commands);
    enet_peer_reset_incoming_commands(&mut peer.dispatched_commands);
    if peer.channel_count > 0 {
        channel = peer.channels.as_mut_ptr();
        while channel < &mut *(peer.channels.as_mut_ptr()).add(peer.channel_count) as *mut Channel {
            enet_peer_reset_incoming_commands(&mut (*channel).incoming_reliable_commands);
            enet_peer_reset_incoming_commands(&mut (*channel).incoming_unreliable_commands);
            channel = channel.offset(1);
        }
    }
    peer.channel_count = 0;
}

pub(crate) fn enet_peer_on_connect<S: Socket>(host: &mut Host<S>, peer: &mut Peer<S>) {
    if peer.state != PeerState::Connected && peer.state != PeerState::DisconnectLater {
        if peer.incoming_bandwidth != 0 {
            host.bandwidth_limited_peers += 1;
        }
        host.connected_peers += 1;
    }
}

pub(crate) fn enet_peer_on_disconnect<S: Socket>(host: &mut Host<S>, peer: &mut Peer<S>) {
    if peer.state == PeerState::Connected || peer.state == PeerState::DisconnectLater {
        if peer.incoming_bandwidth != 0 {
            host.bandwidth_limited_peers -= 1;
        }
        host.connected_peers -= 1;
    }
}

pub(crate) fn enet_peer_reset<S: Socket>(host: &mut Host<S>, peer: &mut Peer<S>) {
    enet_peer_on_disconnect(host, peer);
    peer.outgoing_peer_id = ENET_PROTOCOL_MAXIMUM_PEER_ID;
    peer.connect_id = 0;
    peer.state = PeerState::Disconnected;
    peer.incoming_bandwidth = 0;
    peer.outgoing_bandwidth = 0;
    peer.incoming_bandwidth_throttle_epoch = 0;
    peer.outgoing_bandwidth_throttle_epoch = 0;
    peer.incoming_data_total = 0;
    peer.outgoing_data_total = 0;
    peer.last_send_time = 0;
    peer.last_receive_time = 0;
    peer.next_timeout = 0;
    peer.earliest_timeout = 0;
    peer.packet_loss_epoch = 0;
    peer.packets_sent = 0;
    peer.packets_lost = 0;
    peer.packet_loss = 0;
    peer.packet_loss_variance = 0;
    peer.packet_throttle = ENET_PEER_DEFAULT_PACKET_THROTTLE;
    peer.packet_throttle_limit = ENET_PEER_PACKET_THROTTLE_SCALE;
    peer.packet_throttle_counter = 0;
    peer.packet_throttle_epoch = 0;
    peer.packet_throttle_acceleration = ENET_PEER_PACKET_THROTTLE_ACCELERATION;
    peer.packet_throttle_deceleration = ENET_PEER_PACKET_THROTTLE_DECELERATION;
    peer.packet_throttle_interval = ENET_PEER_PACKET_THROTTLE_INTERVAL;
    peer.ping_interval = ENET_PEER_PING_INTERVAL;
    peer.timeout_limit = ENET_PEER_TIMEOUT_LIMIT;
    peer.timeout_minimum = ENET_PEER_TIMEOUT_MINIMUM;
    peer.timeout_maximum = ENET_PEER_TIMEOUT_MAXIMUM;
    peer.last_round_trip_time = ENET_PEER_DEFAULT_ROUND_TRIP_TIME;
    peer.lowest_round_trip_time = ENET_PEER_DEFAULT_ROUND_TRIP_TIME;
    peer.last_round_trip_time_variance = 0;
    peer.highest_round_trip_time_variance = 0;
    peer.round_trip_time = ENET_PEER_DEFAULT_ROUND_TRIP_TIME;
    peer.round_trip_time_variance = 0;
    peer.mtu = host.mtu;
    peer.reliable_data_in_transit = 0;
    peer.outgoing_reliable_sequence_number = 0;
    peer.window_size = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE;
    peer.incoming_unsequenced_group = 0;
    peer.outgoing_unsequenced_group = 0;
    peer.event_data = 0;
    peer.total_waiting_data = 0;
    peer.flags = 0;
    unsafe {
        enet_memset(
            (peer.unsequenced_window).as_mut_ptr() as *mut c_void,
            0,
            size_of::<[u32; ENET_PEER_UNSEQUENCED_WINDOW_SIZE / 32]>(),
        );
        enet_peer_reset_queues(host, peer);
    }
}

pub(crate) unsafe fn enet_peer_ping<S: Socket>(host: &mut Host<S>, peer: &mut Peer<S>) {
    let mut command = ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channel_id: 0,
            reliable_sequence_number: 0,
        },
    };
    if peer.state != PeerState::Connected {
        return;
    }
    command.header.command = ENET_PROTOCOL_COMMAND_PING | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE;
    command.header.channel_id = 0xff;
    enet_peer_queue_outgoing_command(host, peer, &command, std::ptr::null_mut(), 0, 0);
}

pub(crate) fn enet_peer_ping_interval<S: Socket>(peer: &mut Peer<S>, ping_interval: u32) {
    peer.ping_interval = if ping_interval != 0 {
        ping_interval
    } else {
        ENET_PEER_PING_INTERVAL
    };
}

pub(crate) fn enet_peer_timeout<S: Socket>(
    peer: &mut Peer<S>,
    timeout_limit: u32,
    timeout_minimum: u32,
    timeout_maximum: u32,
) {
    peer.timeout_limit = if timeout_limit != 0 {
        timeout_limit
    } else {
        ENET_PEER_TIMEOUT_LIMIT
    };
    peer.timeout_minimum = if timeout_minimum != 0 {
        timeout_minimum
    } else {
        ENET_PEER_TIMEOUT_MINIMUM
    };
    peer.timeout_maximum = if timeout_maximum != 0 {
        timeout_maximum
    } else {
        ENET_PEER_TIMEOUT_MAXIMUM
    };
}

pub(crate) fn enet_peer_disconnect_now<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    data: u32,
) {
    let mut command = ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channel_id: 0,
            reliable_sequence_number: 0,
        },
    };
    if peer.state == PeerState::Disconnected {
        return;
    }
    if peer.state != PeerState::Zombie && peer.state != PeerState::Disconnecting {
        unsafe {
            enet_peer_reset_queues(host, peer);
            command.header.command =
                ENET_PROTOCOL_COMMAND_DISCONNECT | ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED;
            command.header.channel_id = 0xff;
            command.disconnect.data = data.to_be();
            enet_peer_queue_outgoing_command(host, peer, &command, std::ptr::null_mut(), 0, 0);
            enet_host_flush(host);
        }
    }
    enet_peer_reset(host, peer);
}

pub(crate) fn enet_peer_disconnect<S: Socket>(host: &mut Host<S>, peer: &mut Peer<S>, data: u32) {
    let mut command = ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channel_id: 0,
            reliable_sequence_number: 0,
        },
    };
    if peer.state == PeerState::Disconnecting
        || peer.state == PeerState::Disconnected
        || peer.state == PeerState::AcknowledgingDisconnect
        || peer.state == PeerState::Zombie
    {
        return;
    }
    unsafe {
        enet_peer_reset_queues(host, peer);
    }
    command.header.command = ENET_PROTOCOL_COMMAND_DISCONNECT;
    command.header.channel_id = 0xff;
    command.disconnect.data = data.to_be();
    unsafe {
        if peer.state == PeerState::Connected || peer.state == PeerState::DisconnectLater {
            command.header.command |= ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE;
        } else {
            command.header.command |= ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED;
        }
        enet_peer_queue_outgoing_command(host, peer, &command, std::ptr::null_mut(), 0, 0);
    }
    if peer.state == PeerState::Connected || peer.state == PeerState::DisconnectLater {
        enet_peer_on_disconnect(host, peer);
        peer.state = PeerState::Disconnecting;
    } else {
        enet_host_flush(host);
        enet_peer_reset(host, peer);
    };
}

pub(crate) fn enet_peer_has_outgoing_commands<S: Socket>(peer: &mut Peer<S>) -> i32 {
    unsafe {
        if enet_list_empty(&mut peer.outgoing_commands)
            && enet_list_empty(&mut peer.outgoing_send_reliable_commands)
            && enet_list_empty(&mut peer.sent_reliable_commands)
        {
            0
        } else {
            1
        }
    }
}

pub(crate) fn enet_peer_disconnect_later<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    data: u32,
) {
    if (peer.state == PeerState::Connected || peer.state == PeerState::DisconnectLater)
        && enet_peer_has_outgoing_commands(peer) != 0
    {
        peer.state = PeerState::DisconnectLater;
        peer.event_data = data;
    } else {
        enet_peer_disconnect(host, peer, data);
    };
}

pub(crate) unsafe fn enet_peer_queue_acknowledgement<S: Socket>(
    peer: &mut Peer<S>,
    command: *const ENetProtocol,
    sent_time: u16,
) -> Result<(), crate::Error> {
    if ((*command).header.channel_id as usize) < peer.channel_count {
        let channel: *mut Channel = &mut *(peer.channels.as_mut_ptr())
            .offset((*command).header.channel_id as isize)
            as *mut Channel;
        let mut reliable_window: u16 =
            (*command).header.reliable_sequence_number / ENET_PEER_RELIABLE_WINDOW_SIZE;
        let current_window: u16 =
            (*channel).incoming_reliable_sequence_number / ENET_PEER_RELIABLE_WINDOW_SIZE;
        if ((*command).header.reliable_sequence_number)
            < (*channel).incoming_reliable_sequence_number
        {
            reliable_window += ENET_PEER_RELIABLE_WINDOWS;
        }
        if reliable_window >= current_window + ENET_PEER_FREE_RELIABLE_WINDOWS - 1
            && reliable_window <= current_window + ENET_PEER_FREE_RELIABLE_WINDOWS
        {
            return Err(crate::Error::Unknown);
        }
    }
    peer.outgoing_data_total += size_of::<ENetProtocolAcknowledge>() as u32;
    let acknowledgement = ENetAcknowledgement {
        sent_time: sent_time as u32,
        command: *command,
    };
    peer.acknowledgements.push_back(acknowledgement);
    Ok(())
}

pub(crate) unsafe fn enet_peer_setup_outgoing_command<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    outgoing_command: *mut ENetOutgoingCommand,
) {
    peer.outgoing_data_total +=
        ((enet_protocol_command_size((*outgoing_command).command.header.command))
            + ((*outgoing_command).fragment_length as usize)) as u32;

    peer.outgoing_data_total +=
        ((enet_protocol_command_size((*outgoing_command).command.header.command))
            + ((*outgoing_command).fragment_length as usize)) as u32;
    if (*outgoing_command).command.header.channel_id == 0xff {
        peer.outgoing_reliable_sequence_number += 1;
        (*outgoing_command).reliable_sequence_number = peer.outgoing_reliable_sequence_number;
        (*outgoing_command).unreliable_sequence_number = 0;
    } else {
        let channel: *mut Channel = &mut *(peer.channels.as_mut_ptr())
            .offset((*outgoing_command).command.header.channel_id as isize)
            as *mut Channel;
        if (*outgoing_command).command.header.command & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE != 0
        {
            (*channel).outgoing_reliable_sequence_number =
                (*channel).outgoing_reliable_sequence_number.wrapping_add(1);
            (*channel).outgoing_unreliable_sequence_number = 0;
            (*outgoing_command).reliable_sequence_number =
                (*channel).outgoing_reliable_sequence_number;
            (*outgoing_command).unreliable_sequence_number = 0;
        } else if (*outgoing_command).command.header.command
            & ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED
            != 0
        {
            peer.outgoing_unsequenced_group += 1;
            (*outgoing_command).reliable_sequence_number = 0;
            (*outgoing_command).unreliable_sequence_number = 0;
        } else {
            if (*outgoing_command).fragment_offset == 0 {
                (*channel).outgoing_unreliable_sequence_number = (*channel)
                    .outgoing_unreliable_sequence_number
                    .wrapping_add(1);
            }
            (*outgoing_command).reliable_sequence_number =
                (*channel).outgoing_reliable_sequence_number;
            (*outgoing_command).unreliable_sequence_number =
                (*channel).outgoing_unreliable_sequence_number;
        }
    }
    (*outgoing_command).send_attempts = 0;
    (*outgoing_command).sent_time = 0;
    (*outgoing_command).round_trip_timeout = 0;
    (*outgoing_command).command.header.reliable_sequence_number =
        ((*outgoing_command).reliable_sequence_number).to_be();
    host.total_queued += 1;
    (*outgoing_command).queue_time = host.total_queued;
    match (*outgoing_command).command.header.command & ENET_PROTOCOL_COMMAND_MASK {
        7 => {
            (*outgoing_command)
                .command
                .send_unreliable
                .unreliable_sequence_number =
                ((*outgoing_command).unreliable_sequence_number).to_be();
        }
        9 => {
            (*outgoing_command)
                .command
                .send_unsequenced
                .unsequenced_group = (peer.outgoing_unsequenced_group).to_be();
        }
        _ => {}
    }
    if (*outgoing_command).command.header.command & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE != 0
        && !((*outgoing_command).packet).is_null()
    {
        enet_list_insert(
            enet_list_end(&mut peer.outgoing_send_reliable_commands),
            outgoing_command as *mut ENetListNode<ENetOutgoingCommand>,
        );
    } else {
        enet_list_insert(
            enet_list_end(&mut peer.outgoing_commands),
            outgoing_command as *mut ENetListNode<ENetOutgoingCommand>,
        );
    };
}

pub(crate) unsafe fn enet_peer_queue_outgoing_command<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    command: *const ENetProtocol,
    packet: *mut ENetPacket,
    offset: u32,
    length: u16,
) -> *mut ENetOutgoingCommand {
    let outgoing_command: *mut ENetOutgoingCommand =
        enet_malloc(size_of::<ENetOutgoingCommand>()) as *mut ENetOutgoingCommand;
    if outgoing_command.is_null() {
        return std::ptr::null_mut();
    }
    (*outgoing_command).command = *command;
    (*outgoing_command).fragment_offset = offset;
    (*outgoing_command).fragment_length = length;
    (*outgoing_command).packet = packet;
    if !packet.is_null() {
        (*packet).reference_count += 1;
    }
    enet_peer_setup_outgoing_command(host, peer, outgoing_command);
    outgoing_command
}

pub(crate) unsafe fn enet_peer_dispatch_incoming_unreliable_commands<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    channel: *mut Channel,
    queued_command: *mut ENetIncomingCommand,
) {
    let mut dropped_command: ENetListIterator<ENetIncomingCommand>;
    let mut start_command: ENetListIterator<ENetIncomingCommand>;
    let mut current_command: ENetListIterator<ENetIncomingCommand>;
    let mut next;
    current_command = enet_list_begin(&mut (*channel).incoming_unreliable_commands);
    start_command = current_command;
    dropped_command = start_command;
    while current_command != enet_list_end(&mut (*channel).incoming_unreliable_commands) {
        let incoming_command: *mut ENetIncomingCommand =
            current_command as *mut ENetIncomingCommand;
        if (*incoming_command).command.header.command & ENET_PROTOCOL_COMMAND_MASK
            != ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED
        {
            if (*incoming_command).reliable_sequence_number
                == (*channel).incoming_reliable_sequence_number
            {
                if (*incoming_command).fragments_remaining == 0 {
                    (*channel).incoming_unreliable_sequence_number =
                        (*incoming_command).unreliable_sequence_number;
                    next = false;
                } else {
                    if start_command != current_command {
                        enet_list_move(
                            enet_list_end(&mut peer.dispatched_commands),
                            start_command as *mut c_void,
                            enet_list_previous(current_command) as *mut c_void,
                        );
                        if peer.flags & ENET_PEER_FLAG_NEEDS_DISPATCH == 0 {
                            host.dispatch_queue.push_back(peer.index);
                            peer.flags |= ENET_PEER_FLAG_NEEDS_DISPATCH;
                        }
                        dropped_command = current_command;
                    } else if dropped_command != current_command {
                        dropped_command = enet_list_previous(current_command);
                    }
                    next = true;
                }
            } else {
                let mut reliable_window: u16 =
                    (*incoming_command).reliable_sequence_number / ENET_PEER_RELIABLE_WINDOW_SIZE;
                let current_window: u16 =
                    (*channel).incoming_reliable_sequence_number / ENET_PEER_RELIABLE_WINDOW_SIZE;
                if ((*incoming_command).reliable_sequence_number)
                    < (*channel).incoming_reliable_sequence_number
                {
                    reliable_window += ENET_PEER_RELIABLE_WINDOWS;
                }
                if reliable_window >= current_window
                    && (reliable_window) < current_window + ENET_PEER_FREE_RELIABLE_WINDOWS - 1
                {
                    break;
                }
                dropped_command = enet_list_next(current_command);
                if start_command != current_command {
                    enet_list_move(
                        enet_list_end(&mut peer.dispatched_commands),
                        start_command as *mut c_void,
                        enet_list_previous(current_command) as *mut c_void,
                    );
                    if peer.flags & ENET_PEER_FLAG_NEEDS_DISPATCH == 0 {
                        host.dispatch_queue.push_back(peer.index);
                        peer.flags |= ENET_PEER_FLAG_NEEDS_DISPATCH;
                    }
                }
                next = true;
            }
            if next {
                start_command = enet_list_next(current_command);
            }
        }
        current_command = enet_list_next(current_command);
    }
    if start_command != current_command {
        enet_list_move(
            enet_list_end(&mut peer.dispatched_commands),
            start_command as *mut c_void,
            enet_list_previous(current_command) as *mut c_void,
        );
        if peer.flags & ENET_PEER_FLAG_NEEDS_DISPATCH == 0 {
            host.dispatch_queue.push_back(peer.index);
            peer.flags |= ENET_PEER_FLAG_NEEDS_DISPATCH;
        }
        dropped_command = current_command;
    }
    enet_peer_remove_incoming_commands(
        &mut (*channel).incoming_unreliable_commands,
        enet_list_begin(&mut (*channel).incoming_unreliable_commands),
        dropped_command,
        queued_command,
    );
}

pub(crate) unsafe fn enet_peer_dispatch_incoming_reliable_commands<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    channel: *mut Channel,
    queued_command: *mut ENetIncomingCommand,
) {
    let mut current_command = enet_list_begin(&mut (*channel).incoming_reliable_commands);
    while current_command != enet_list_end(&mut (*channel).incoming_reliable_commands) {
        let incoming_command: *mut ENetIncomingCommand =
            current_command as *mut ENetIncomingCommand;
        if (*incoming_command).fragments_remaining > 0
            || (*incoming_command).reliable_sequence_number
                != ((*channel).incoming_reliable_sequence_number + 1)
        {
            break;
        }
        (*channel).incoming_reliable_sequence_number = (*incoming_command).reliable_sequence_number;
        if (*incoming_command).fragment_count > 0 {
            (*channel).incoming_reliable_sequence_number +=
                ((*incoming_command).fragment_count - 1) as u16;
        }
        current_command = enet_list_next(current_command);
    }
    if current_command == enet_list_begin(&mut (*channel).incoming_reliable_commands) {
        return;
    }
    (*channel).incoming_unreliable_sequence_number = 0;
    enet_list_move(
        enet_list_end(&mut peer.dispatched_commands),
        enet_list_begin(&mut (*channel).incoming_reliable_commands) as *mut c_void,
        enet_list_previous(current_command) as *mut c_void,
    );
    if peer.flags & ENET_PEER_FLAG_NEEDS_DISPATCH == 0 {
        host.dispatch_queue.push_back(peer.index);
        peer.flags |= ENET_PEER_FLAG_NEEDS_DISPATCH;
    }
    if !enet_list_empty(&mut (*channel).incoming_unreliable_commands) {
        enet_peer_dispatch_incoming_unreliable_commands(host, peer, channel, queued_command);
    }
}

pub(crate) unsafe fn enet_peer_queue_incoming_command<S: Socket>(
    host: &mut Host<S>,
    peer: &mut Peer<S>,
    command: *const ENetProtocol,
    data: *const c_void,
    data_length: usize,
    flags: PacketFlag,
    fragment_count: u32,
) -> *mut ENetIncomingCommand {
    static mut DUMMY_COMMAND: ENetIncomingCommand = ENetIncomingCommand {
        incoming_command_list: ENetListNode::zeroed(),
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
        fragments: std::ptr::null_mut(),
        packet: std::ptr::null_mut(),
    };

    let channel = &mut peer.channels[(*command).header.channel_id as usize] as *mut Channel;
    let mut unreliable_sequence_number: u32 = 0;
    let mut reliable_sequence_number: u32 = 0;
    let mut reliable_window: u16;
    let current_window: u16;
    let mut incoming_command: *mut ENetIncomingCommand;
    let mut current_command: ENetListIterator<ENetIncomingCommand>;
    let mut packet: *mut ENetPacket = std::ptr::null_mut();

    macro_rules! notify_error {
        () => {
            if !packet.is_null() && (*packet).reference_count == 0 {
                enet_packet_destroy(packet);
            }

            return std::ptr::null_mut();
        };
    }

    macro_rules! discard_command {
        () => {
            if fragment_count > 0 {
                notify_error!();
            }
            if !packet.is_null() && (*packet).reference_count == 0 {
                enet_packet_destroy(packet);
            }
            return &mut DUMMY_COMMAND;
        };
    }

    if peer.state == PeerState::DisconnectLater {
        discard_command!();
    }

    if ((*command).header.command & ENET_PROTOCOL_COMMAND_MASK)
        != ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED
    {
        reliable_sequence_number = (*command).header.reliable_sequence_number as u32;
        reliable_window = reliable_sequence_number as u16 / ENET_PEER_RELIABLE_WINDOW_SIZE;
        current_window =
            (*channel).incoming_reliable_sequence_number / ENET_PEER_RELIABLE_WINDOW_SIZE;

        if reliable_sequence_number < (*channel).incoming_reliable_sequence_number as u32 {
            reliable_window += ENET_PEER_RELIABLE_WINDOWS;
        }

        if reliable_window < current_window
            || reliable_window >= current_window + ENET_PEER_FREE_RELIABLE_WINDOWS - 1
        {
            discard_command!();
        }
    }

    let command_masked = (*command).header.command & ENET_PROTOCOL_COMMAND_MASK;
    if command_masked == ENET_PROTOCOL_COMMAND_SEND_FRAGMENT
        || command_masked == ENET_PROTOCOL_COMMAND_SEND_RELIABLE
    {
        if reliable_sequence_number as u16 == (*channel).incoming_reliable_sequence_number {
            discard_command!();
        }

        current_command =
            enet_list_previous(enet_list_end(&mut (*channel).incoming_reliable_commands));
        while current_command != enet_list_end(&mut (*channel).incoming_reliable_commands) {
            incoming_command = current_command as *mut ENetIncomingCommand;

            if reliable_sequence_number as u16 >= (*channel).incoming_reliable_sequence_number {
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

            if (*incoming_command).reliable_sequence_number <= reliable_sequence_number as u16 {
                if (*incoming_command).reliable_sequence_number < reliable_sequence_number as u16 {
                    break;
                }
                discard_command!();
            }
            current_command = enet_list_previous(current_command);
        }
    } else if command_masked == ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE
        || command_masked == ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT
    {
        unreliable_sequence_number =
            u16::from_be((*command).send_unreliable.unreliable_sequence_number) as u32;

        if reliable_sequence_number == (*channel).incoming_reliable_sequence_number as u32
            && unreliable_sequence_number <= (*channel).incoming_unreliable_sequence_number as u32
        {
            discard_command!();
        }

        current_command =
            enet_list_previous(enet_list_end(&mut (*channel).incoming_unreliable_commands));
        while current_command != enet_list_end(&mut (*channel).incoming_unreliable_commands) {
            incoming_command = current_command as *mut ENetIncomingCommand;

            if ((*command).header.command & ENET_PROTOCOL_COMMAND_MASK)
                == ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED
            {
                current_command = enet_list_previous(current_command);
                continue;
            }

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

            if (*incoming_command).reliable_sequence_number < reliable_sequence_number as u16 {
                break;
            }

            if (*incoming_command).reliable_sequence_number > reliable_sequence_number as u16 {
                current_command = enet_list_previous(current_command);
                continue;
            }

            if (*incoming_command).unreliable_sequence_number <= unreliable_sequence_number as u16 {
                if (*incoming_command).unreliable_sequence_number
                    < unreliable_sequence_number as u16
                {
                    break;
                }

                discard_command!();
            }
            current_command = enet_list_previous(current_command);
        }
    } else if command_masked == ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED {
        current_command = enet_list_end(&mut (*channel).incoming_unreliable_commands);
    } else {
        discard_command!();
    }

    if peer.total_waiting_data >= host.maximum_waiting_data {
        notify_error!();
    }

    packet = enet_packet_create(data, data_length, flags);
    if packet.is_null() {
        notify_error!();
    }

    incoming_command = enet_malloc(size_of::<ENetIncomingCommand>()) as *mut ENetIncomingCommand;
    if incoming_command.is_null() {
        notify_error!();
    }

    (*incoming_command).reliable_sequence_number = (*command).header.reliable_sequence_number;
    (*incoming_command).unreliable_sequence_number = (unreliable_sequence_number & 0xFFF) as u16;
    (*incoming_command).command = *command;
    (*incoming_command).fragment_count = fragment_count;
    (*incoming_command).fragments_remaining = fragment_count;
    (*incoming_command).packet = packet;
    (*incoming_command).fragments = std::ptr::null_mut();

    if fragment_count > 0 {
        if fragment_count <= ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT {
            (*incoming_command).fragments =
                enet_malloc(((fragment_count as usize + 31) / 32) * size_of::<u32>()) as *mut u32;
        }
        if (*incoming_command).fragments.is_null() {
            enet_free(incoming_command as *mut c_void);

            notify_error!();
        }
        enet_memset(
            (*incoming_command).fragments as *mut c_void,
            0,
            ((fragment_count as usize + 31) / 32) * size_of::<u32>(),
        );
    }

    if !packet.is_null() {
        (*packet).reference_count += 1;

        peer.total_waiting_data += (*packet).data_length;
    }

    enet_list_insert(
        enet_list_next(current_command),
        incoming_command as *mut ENetListNode<ENetIncomingCommand>,
    );

    let command_masked = (*command).header.command & ENET_PROTOCOL_COMMAND_MASK;
    if command_masked == ENET_PROTOCOL_COMMAND_SEND_FRAGMENT
        || command_masked == ENET_PROTOCOL_COMMAND_SEND_RELIABLE
    {
        enet_peer_dispatch_incoming_reliable_commands(host, peer, channel, incoming_command);
    } else {
        enet_peer_dispatch_incoming_unreliable_commands(host, peer, channel, incoming_command);
    }

    incoming_command
}
