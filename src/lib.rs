//! [ENet](https://github.com/lsalzman/enet) transpiled to Rust, and made agnostic to the underlying
//! socket. Supports [`std::net::UdpSocket`] out of the box. Works in WASM if you bring your own WebRTC
//! interface or similar.
//!
//! Much of the docs are copied from the [ENet Website](http://sauerbraten.org/enet/index.html),
//! both for convenience, and in case that resource is unavailable for any reason.
//!
//! > ENet's purpose is to provide a relatively thin, simple and robust network communication layer
//! on top of UDP (User Datagram Protocol). The primary feature it provides is optional reliable,
//! in-order delivery of packets.  
//! >
//! > ENet omits certain higher level networking features such as authentication, lobbying, server
//! discovery, encryption, or other similar tasks that are particularly application specific so that
//! the library remains flexible, portable, and easily embeddable.
//!
//! [See the examples](https://github.com/jabuwu/rusty_enet/tree/main/examples)
//!
//! # Features and Architecture
//!
//! ENet evolved specifically as a UDP networking layer for the multiplayer first person shooter
//! Cube.
//!
//! Cube necessitated low latency communication with data sent out very frequently, so TCP was an
//! unsuitable choice due to its high latency and stream orientation. UDP, however, lacks many
//! sometimes necessary features from TCP such as reliability, sequencing, unrestricted packet
//! sizes, and connection management. So UDP by itself was not suitable as a network protocol
//! either. No suitable freely available networking libraries existed at the time of ENet's creation
//! to fill this niche.
//!
//! UDP and TCP could have been used together in Cube to benefit somewhat from both of their
//! features, however, the resulting combinations of protocols still leaves much to be desired.
//! TCP lacks multiple streams of communication without resorting to opening many sockets and
//! complicates delineation of packets due to its buffering behavior. UDP lacks sequencing,
//! connection management, management of bandwidth resources, and imposes limitations on the size of
//! packets. A significant investment is required to integrate these two protocols, and the end
//! result is worse off in features and performance than the uniform protocol presented by ENet.
//!
//! ENet thus attempts to address these issues and provide a single, uniform protocol layered over
//! UDP to the developer with the best features of UDP and TCP as well as some useful features
//! neither provide, with a much cleaner integration than any resulting from a mixture of UDP and
//! TCP.
//!
//! ## Connection Management
//!
//! ENet provides a simple connection interface over which to communicate with a foreign host. The
//! liveness of the connection is actively monitored by pinging the foreign host at frequent
//! intervals, and also monitors the network conditions from the local host to the foreign host such
//! as the mean round trip time and packet loss in this fashion.
//!
//! ## Sequencing
//!
//! Rather than a single byte stream that complicates the delineation of packets, ENet presents
//! connections as multiple, properly sequenced packet streams that simplify the transfer of various
//! types of data.
//!
//! ENet provides sequencing for all packets by assigning to each sent packet a sequence number that
//! is incremented as packets are sent. ENet guarantees that no packet with a higher sequence number
//! will be delivered before a packet with a lower sequence number, thus ensuring packets are
//! delivered exactly in the order they are sent.
//!
//! For unreliable packets, ENet will simply discard the lower sequence number packet if a packet
//! with a higher sequence number has already been delivered. This allows the packets to be
//! dispatched immediately as they arrive, and reduce latency of unreliable packets to an absolute
//! minimum. For reliable packets, if a higher sequence number packet arrives, but the preceding
//! packets in the sequence have not yet arrived, ENet will stall delivery of the higher sequence
//! number packets until its predecessors have arrived.
//!
//! ## Channels
//!
//! Since ENet will stall delivery of reliable packets to ensure proper sequencing, and consequently
//! any packets of higher sequence number whether reliable or unreliable, in the event the reliable
//! packet's predecessors have not yet arrived, this can introduce latency into the delivery of
//! other packets which may not need to be as strictly ordered with respect to the packet that
//! stalled their delivery.
//!
//! To combat this latency and reduce the ordering restrictions on packets, ENet provides multiple
//! channels of communication over a given connection. Each channel is independently sequenced, and
//! so the delivery status of a packet in one channel will not stall the delivery of other packets
//! in another channel.
//!
//! ## Reliability
//!
//! ENet provides optional reliability of packet delivery by ensuring the foreign host acknowledges
//! receipt of all reliable packets. ENet will attempt to resend the packet up to a reasonable
//! amount of times, if no acknowledgement of the packet's receipt happens within a specified
//! timeout. Retry timeouts are progressive and become more lenient with every failed attempt to
//! allow for temporary turbulence in network conditions.
//!
//! ## Fragmentation and Reassembly
//!
//! ENet will send and deliver packets regardless of size. Large packets are fragmented into many
//! smaller packets of suitable size, and reassembled on the foreign host to recover the original
//! packet for delivery. The process is entirely transparent to the developer.
//!
//! ## Aggregation
//!
//! ENet aggregates all protocol commands, including acknowledgements and packet transfer, into
//! larger protocol packets to ensure the proper utilization of the connection and to limit the
//! opportunities for packet loss that might otherwise result in further delivery latency.
//!
//! ## Adaptability
//!
//! ENet provides an in-flight data window for reliable packets to ensure connections are not
//! overwhelmed by volumes of packets. It also provides a static bandwidth allocation mechanism to
//! ensure the total volume of packets sent and received to a host don't exceed the host's
//! capabilities. Further, ENet also provides a dynamic throttle that responds to deviations from
//! normal network connections to rectify various types of network congestion by further limiting
//! the volume of packets sent.

#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![warn(missing_docs)]

use std::mem::MaybeUninit;

use wasm_timer::{SystemTime, UNIX_EPOCH};

mod address;
mod error;
mod event;
mod host;
mod os;
mod packet;
mod peer;
mod socket;
mod version;

pub use address::*;
pub use error::*;
pub use event::*;
pub use host::*;
pub(crate) use os::*;
pub use packet::*;
pub use peer::*;
pub use socket::*;
pub use version::*;

/// Constants provided by ENet.
#[allow(missing_docs)]
pub mod consts;
use consts::*;

/// A [`Result`](`core::result::Result`) type alias with this crate's [`Error`] type.
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct ENetBuffer {
    pub(crate) data: *mut c_void,
    pub(crate) dataLength: size_t,
}
pub(crate) type enet_uint8 = c_uchar;
pub(crate) type enet_uint16 = c_ushort;
pub(crate) type enet_uint32 = c_uint;
pub(crate) type _ENetProtocolCommand = c_uint;
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
pub(crate) type _ENetProtocolFlag = c_uint;
pub(crate) const ENET_PROTOCOL_HEADER_SESSION_SHIFT: _ENetProtocolFlag = 12;
pub(crate) const ENET_PROTOCOL_HEADER_SESSION_MASK: _ENetProtocolFlag = 12288;
pub(crate) const ENET_PROTOCOL_HEADER_FLAG_MASK: _ENetProtocolFlag = 49152;
pub(crate) const ENET_PROTOCOL_HEADER_FLAG_SENT_TIME: _ENetProtocolFlag = 32768;
pub(crate) const ENET_PROTOCOL_HEADER_FLAG_COMPRESSED: _ENetProtocolFlag = 16384;
pub(crate) const ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED: _ENetProtocolFlag = 64;
pub(crate) const ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE: _ENetProtocolFlag = 128;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct _ENetProtocolHeader {
    pub(crate) peerID: enet_uint16,
    pub(crate) sentTime: enet_uint16,
}
pub(crate) type ENetProtocolHeader = _ENetProtocolHeader;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct _ENetProtocolCommandHeader {
    pub(crate) command: enet_uint8,
    pub(crate) channelID: enet_uint8,
    pub(crate) reliableSequenceNumber: enet_uint16,
}
pub(crate) type ENetProtocolCommandHeader = _ENetProtocolCommandHeader;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct _ENetProtocolAcknowledge {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) receivedReliableSequenceNumber: enet_uint16,
    pub(crate) receivedSentTime: enet_uint16,
}
pub(crate) type ENetProtocolAcknowledge = _ENetProtocolAcknowledge;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct _ENetProtocolConnect {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) outgoingPeerID: enet_uint16,
    pub(crate) incomingSessionID: enet_uint8,
    pub(crate) outgoingSessionID: enet_uint8,
    pub(crate) mtu: enet_uint32,
    pub(crate) windowSize: enet_uint32,
    pub(crate) channelCount: enet_uint32,
    pub(crate) incomingBandwidth: enet_uint32,
    pub(crate) outgoingBandwidth: enet_uint32,
    pub(crate) packetThrottleInterval: enet_uint32,
    pub(crate) packetThrottleAcceleration: enet_uint32,
    pub(crate) packetThrottleDeceleration: enet_uint32,
    pub(crate) connectID: enet_uint32,
    pub(crate) data: enet_uint32,
}
pub(crate) type ENetProtocolConnect = _ENetProtocolConnect;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct _ENetProtocolVerifyConnect {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) outgoingPeerID: enet_uint16,
    pub(crate) incomingSessionID: enet_uint8,
    pub(crate) outgoingSessionID: enet_uint8,
    pub(crate) mtu: enet_uint32,
    pub(crate) windowSize: enet_uint32,
    pub(crate) channelCount: enet_uint32,
    pub(crate) incomingBandwidth: enet_uint32,
    pub(crate) outgoingBandwidth: enet_uint32,
    pub(crate) packetThrottleInterval: enet_uint32,
    pub(crate) packetThrottleAcceleration: enet_uint32,
    pub(crate) packetThrottleDeceleration: enet_uint32,
    pub(crate) connectID: enet_uint32,
}
pub(crate) type ENetProtocolVerifyConnect = _ENetProtocolVerifyConnect;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct _ENetProtocolBandwidthLimit {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) incomingBandwidth: enet_uint32,
    pub(crate) outgoingBandwidth: enet_uint32,
}
pub(crate) type ENetProtocolBandwidthLimit = _ENetProtocolBandwidthLimit;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct _ENetProtocolThrottleConfigure {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) packetThrottleInterval: enet_uint32,
    pub(crate) packetThrottleAcceleration: enet_uint32,
    pub(crate) packetThrottleDeceleration: enet_uint32,
}
pub(crate) type ENetProtocolThrottleConfigure = _ENetProtocolThrottleConfigure;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct _ENetProtocolDisconnect {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) data: enet_uint32,
}
pub(crate) type ENetProtocolDisconnect = _ENetProtocolDisconnect;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct _ENetProtocolPing {
    pub(crate) header: ENetProtocolCommandHeader,
}
pub(crate) type ENetProtocolPing = _ENetProtocolPing;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct _ENetProtocolSendReliable {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) dataLength: enet_uint16,
}
pub(crate) type ENetProtocolSendReliable = _ENetProtocolSendReliable;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct _ENetProtocolSendUnreliable {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) unreliableSequenceNumber: enet_uint16,
    pub(crate) dataLength: enet_uint16,
}
pub(crate) type ENetProtocolSendUnreliable = _ENetProtocolSendUnreliable;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct _ENetProtocolSendUnsequenced {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) unsequencedGroup: enet_uint16,
    pub(crate) dataLength: enet_uint16,
}
pub(crate) type ENetProtocolSendUnsequenced = _ENetProtocolSendUnsequenced;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) struct _ENetProtocolSendFragment {
    pub(crate) header: ENetProtocolCommandHeader,
    pub(crate) startSequenceNumber: enet_uint16,
    pub(crate) dataLength: enet_uint16,
    pub(crate) fragmentCount: enet_uint32,
    pub(crate) fragmentNumber: enet_uint32,
    pub(crate) totalLength: enet_uint32,
    pub(crate) fragmentOffset: enet_uint32,
}
pub(crate) type ENetProtocolSendFragment = _ENetProtocolSendFragment;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub(crate) union _ENetProtocol {
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
pub(crate) type ENetProtocol = _ENetProtocol;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetListNode {
    pub(crate) next: *mut _ENetListNode,
    pub(crate) previous: *mut _ENetListNode,
}
pub(crate) type ENetListNode = _ENetListNode;
pub(crate) type ENetListIterator = *mut ENetListNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetList {
    pub(crate) sentinel: ENetListNode,
}
pub(crate) type ENetList = _ENetList;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetCallbacks {
    pub(crate) malloc: Option<unsafe extern "C" fn(size_t) -> *mut c_void>,
    pub(crate) free: Option<unsafe extern "C" fn(*mut c_void) -> ()>,
    pub(crate) no_memory: Option<unsafe extern "C" fn() -> ()>,
}
pub(crate) type ENetCallbacks = _ENetCallbacks;
pub(crate) type ENetVersion = enet_uint32;
pub(crate) struct _ENetHost<S: Socket> {
    pub(crate) socket: MaybeUninit<S>,
    pub(crate) incomingBandwidth: enet_uint32,
    pub(crate) outgoingBandwidth: enet_uint32,
    pub(crate) bandwidthThrottleEpoch: enet_uint32,
    pub(crate) mtu: enet_uint32,
    pub(crate) randomSeed: enet_uint32,
    pub(crate) recalculateBandwidthLimits: c_int,
    pub(crate) peers: *mut ENetPeer<S>,
    pub(crate) peerCount: size_t,
    pub(crate) channelLimit: size_t,
    pub(crate) serviceTime: enet_uint32,
    pub(crate) dispatchQueue: ENetList,
    pub(crate) totalQueued: enet_uint32,
    pub(crate) packetSize: size_t,
    pub(crate) headerFlags: enet_uint16,
    pub(crate) commands: [ENetProtocol; 32],
    pub(crate) commandCount: size_t,
    pub(crate) buffers: [ENetBuffer; 65],
    pub(crate) bufferCount: size_t,
    pub(crate) checksum: ENetChecksumCallback,
    pub(crate) compressor: ENetCompressor,
    pub(crate) packetData: [[enet_uint8; 4096]; 2],
    pub(crate) receivedAddress: MaybeUninit<Option<S::PeerAddress>>,
    pub(crate) receivedData: *mut enet_uint8,
    pub(crate) receivedDataLength: size_t,
    pub(crate) totalSentData: enet_uint32,
    pub(crate) totalSentPackets: enet_uint32,
    pub(crate) totalReceivedData: enet_uint32,
    pub(crate) totalReceivedPackets: enet_uint32,
    pub(crate) intercept: ENetInterceptCallback<S>,
    pub(crate) connectedPeers: size_t,
    pub(crate) bandwidthLimitedPeers: size_t,
    pub(crate) duplicatePeers: size_t,
    pub(crate) maximumPacketSize: size_t,
    pub(crate) maximumWaitingData: size_t,
}
pub(crate) type ENetInterceptCallback<S> =
    Option<unsafe extern "C" fn(*mut _ENetHost<S>, *mut _ENetEvent<S>) -> c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetEvent<S: Socket> {
    pub(crate) type_0: ENetEventType,
    pub(crate) peer: *mut ENetPeer<S>,
    pub(crate) channelID: enet_uint8,
    pub(crate) data: enet_uint32,
    pub(crate) packet: *mut ENetPacket,
}
pub(crate) type ENetPacket = _ENetPacket;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetPacket {
    pub(crate) referenceCount: size_t,
    pub(crate) flags: enet_uint32,
    pub(crate) data: *mut enet_uint8,
    pub(crate) dataLength: size_t,
    pub(crate) freeCallback: ENetPacketFreeCallback,
    pub(crate) userData: *mut c_void,
}
pub(crate) type ENetPacketFreeCallback = Option<unsafe extern "C" fn(*mut _ENetPacket) -> ()>;
pub(crate) type ENetPeer<S> = _ENetPeer<S>;
#[repr(C)]
pub(crate) struct _ENetPeer<S: Socket> {
    pub(crate) dispatchList: ENetListNode,
    pub(crate) host: *mut _ENetHost<S>,
    pub(crate) outgoingPeerID: enet_uint16,
    pub(crate) incomingPeerID: enet_uint16,
    pub(crate) connectID: enet_uint32,
    pub(crate) outgoingSessionID: enet_uint8,
    pub(crate) incomingSessionID: enet_uint8,
    pub(crate) address: MaybeUninit<Option<S::PeerAddress>>,
    pub(crate) data: *mut c_void,
    pub(crate) state: ENetPeerState,
    pub(crate) channels: *mut ENetChannel,
    pub(crate) channelCount: size_t,
    pub(crate) incomingBandwidth: enet_uint32,
    pub(crate) outgoingBandwidth: enet_uint32,
    pub(crate) incomingBandwidthThrottleEpoch: enet_uint32,
    pub(crate) outgoingBandwidthThrottleEpoch: enet_uint32,
    pub(crate) incomingDataTotal: enet_uint32,
    pub(crate) outgoingDataTotal: enet_uint32,
    pub(crate) lastSendTime: enet_uint32,
    pub(crate) lastReceiveTime: enet_uint32,
    pub(crate) nextTimeout: enet_uint32,
    pub(crate) earliestTimeout: enet_uint32,
    pub(crate) packetLossEpoch: enet_uint32,
    pub(crate) packetsSent: enet_uint32,
    pub(crate) packetsLost: enet_uint32,
    pub(crate) packetLoss: enet_uint32,
    pub(crate) packetLossVariance: enet_uint32,
    pub(crate) packetThrottle: enet_uint32,
    pub(crate) packetThrottleLimit: enet_uint32,
    pub(crate) packetThrottleCounter: enet_uint32,
    pub(crate) packetThrottleEpoch: enet_uint32,
    pub(crate) packetThrottleAcceleration: enet_uint32,
    pub(crate) packetThrottleDeceleration: enet_uint32,
    pub(crate) packetThrottleInterval: enet_uint32,
    pub(crate) pingInterval: enet_uint32,
    pub(crate) timeoutLimit: enet_uint32,
    pub(crate) timeoutMinimum: enet_uint32,
    pub(crate) timeoutMaximum: enet_uint32,
    pub(crate) lastRoundTripTime: enet_uint32,
    pub(crate) lowestRoundTripTime: enet_uint32,
    pub(crate) lastRoundTripTimeVariance: enet_uint32,
    pub(crate) highestRoundTripTimeVariance: enet_uint32,
    pub(crate) roundTripTime: enet_uint32,
    pub(crate) roundTripTimeVariance: enet_uint32,
    pub(crate) mtu: enet_uint32,
    pub(crate) windowSize: enet_uint32,
    pub(crate) reliableDataInTransit: enet_uint32,
    pub(crate) outgoingReliableSequenceNumber: enet_uint16,
    pub(crate) acknowledgements: ENetList,
    pub(crate) sentReliableCommands: ENetList,
    pub(crate) outgoingSendReliableCommands: ENetList,
    pub(crate) outgoingCommands: ENetList,
    pub(crate) dispatchedCommands: ENetList,
    pub(crate) flags: enet_uint16,
    pub(crate) reserved: enet_uint16,
    pub(crate) incomingUnsequencedGroup: enet_uint16,
    pub(crate) outgoingUnsequencedGroup: enet_uint16,
    pub(crate) unsequencedWindow: [enet_uint32; 32],
    pub(crate) eventData: enet_uint32,
    pub(crate) totalWaitingData: size_t,
}
pub(crate) type ENetChannel = _ENetChannel;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetChannel {
    pub(crate) outgoingReliableSequenceNumber: enet_uint16,
    pub(crate) outgoingUnreliableSequenceNumber: enet_uint16,
    pub(crate) usedReliableWindows: enet_uint16,
    pub(crate) reliableWindows: [enet_uint16; 16],
    pub(crate) incomingReliableSequenceNumber: enet_uint16,
    pub(crate) incomingUnreliableSequenceNumber: enet_uint16,
    pub(crate) incomingReliableCommands: ENetList,
    pub(crate) incomingUnreliableCommands: ENetList,
}
pub(crate) type ENetPeerState = _ENetPeerState;
pub(crate) type _ENetPeerState = c_uint;
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
pub(crate) type ENetEventType = _ENetEventType;
pub(crate) type _ENetEventType = c_uint;
pub(crate) const ENET_EVENT_TYPE_RECEIVE: _ENetEventType = 3;
pub(crate) const ENET_EVENT_TYPE_DISCONNECT: _ENetEventType = 2;
pub(crate) const ENET_EVENT_TYPE_CONNECT: _ENetEventType = 1;
pub(crate) const ENET_EVENT_TYPE_NONE: _ENetEventType = 0;
pub(crate) type ENetCompressor = _ENetCompressor;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetCompressor {
    pub(crate) context: *mut c_void,
    pub(crate) compress: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *const ENetBuffer,
            size_t,
            size_t,
            *mut enet_uint8,
            size_t,
        ) -> size_t,
    >,
    pub(crate) decompress: Option<
        unsafe extern "C" fn(
            *mut c_void,
            *const enet_uint8,
            size_t,
            *mut enet_uint8,
            size_t,
        ) -> size_t,
    >,
    pub(crate) destroy: Option<unsafe extern "C" fn(*mut c_void) -> ()>,
}
pub(crate) type ENetChecksumCallback =
    Option<unsafe extern "C" fn(*const ENetBuffer, size_t) -> enet_uint32>;
pub(crate) type _ENetSocketType = c_uint;
pub(crate) const ENET_SOCKET_TYPE_DATAGRAM: _ENetSocketType = 2;
pub(crate) const ENET_SOCKET_TYPE_STREAM: _ENetSocketType = 1;
pub(crate) type ENetSocketType = _ENetSocketType;
pub(crate) type _ENetSocketOption = c_uint;
pub(crate) const ENET_SOCKOPT_TTL: _ENetSocketOption = 10;
pub(crate) const ENET_SOCKOPT_NODELAY: _ENetSocketOption = 9;
pub(crate) const ENET_SOCKOPT_ERROR: _ENetSocketOption = 8;
pub(crate) const ENET_SOCKOPT_SNDTIMEO: _ENetSocketOption = 7;
pub(crate) const ENET_SOCKOPT_RCVTIMEO: _ENetSocketOption = 6;
pub(crate) const ENET_SOCKOPT_REUSEADDR: _ENetSocketOption = 5;
pub(crate) const ENET_SOCKOPT_SNDBUF: _ENetSocketOption = 4;
pub(crate) const ENET_SOCKOPT_RCVBUF: _ENetSocketOption = 3;
pub(crate) const ENET_SOCKOPT_BROADCAST: _ENetSocketOption = 2;
pub(crate) const ENET_SOCKOPT_NONBLOCK: _ENetSocketOption = 1;
pub(crate) type ENetSocketOption = _ENetSocketOption;
pub(crate) type _ENetSocketShutdown = c_uint;
pub(crate) const ENET_SOCKET_SHUTDOWN_READ_WRITE: _ENetSocketShutdown = 2;
pub(crate) const ENET_SOCKET_SHUTDOWN_WRITE: _ENetSocketShutdown = 1;
pub(crate) const ENET_SOCKET_SHUTDOWN_READ: _ENetSocketShutdown = 0;
pub(crate) type ENetSocketShutdown = _ENetSocketShutdown;
pub(crate) type _ENetPacketFlag = c_uint;
pub(crate) const ENET_PACKET_FLAG_SENT: _ENetPacketFlag = 256;
pub(crate) const ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT: _ENetPacketFlag = 8;
pub(crate) const ENET_PACKET_FLAG_NO_ALLOCATE: _ENetPacketFlag = 4;
pub(crate) const ENET_PACKET_FLAG_UNSEQUENCED: _ENetPacketFlag = 2;
pub(crate) const ENET_PACKET_FLAG_RELIABLE: _ENetPacketFlag = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetAcknowledgement {
    pub(crate) acknowledgementList: ENetListNode,
    pub(crate) sentTime: enet_uint32,
    pub(crate) command: ENetProtocol,
}
pub(crate) type ENetAcknowledgement = _ENetAcknowledgement;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetOutgoingCommand {
    pub(crate) outgoingCommandList: ENetListNode,
    pub(crate) reliableSequenceNumber: enet_uint16,
    pub(crate) unreliableSequenceNumber: enet_uint16,
    pub(crate) sentTime: enet_uint32,
    pub(crate) roundTripTimeout: enet_uint32,
    pub(crate) queueTime: enet_uint32,
    pub(crate) fragmentOffset: enet_uint32,
    pub(crate) fragmentLength: enet_uint16,
    pub(crate) sendAttempts: enet_uint16,
    pub(crate) command: ENetProtocol,
    pub(crate) packet: *mut ENetPacket,
}
pub(crate) type ENetOutgoingCommand = _ENetOutgoingCommand;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetIncomingCommand {
    pub(crate) incomingCommandList: ENetListNode,
    pub(crate) reliableSequenceNumber: enet_uint16,
    pub(crate) unreliableSequenceNumber: enet_uint16,
    pub(crate) command: ENetProtocol,
    pub(crate) fragmentCount: enet_uint32,
    pub(crate) fragmentsRemaining: enet_uint32,
    pub(crate) fragments: *mut enet_uint32,
    pub(crate) packet: *mut ENetPacket,
}
pub(crate) type ENetIncomingCommand = _ENetIncomingCommand;
pub(crate) type _ENetPeerFlag = c_uint;
pub(crate) const ENET_PEER_FLAG_CONTINUE_SENDING: _ENetPeerFlag = 2;
pub(crate) const ENET_PEER_FLAG_NEEDS_DISPATCH: _ENetPeerFlag = 1;
pub(crate) type ENetHost<S> = _ENetHost<S>;
pub(crate) type ENetEvent<S> = _ENetEvent<S>;
pub(crate) type ENetRangeCoder = _ENetRangeCoder;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetRangeCoder {
    pub(crate) symbols: [ENetSymbol; 4096],
}
pub(crate) type ENetSymbol = _ENetSymbol;
#[derive(Copy, Clone)]
#[repr(C)]
pub(crate) struct _ENetSymbol {
    pub(crate) value: enet_uint8,
    pub(crate) count: enet_uint8,
    pub(crate) under: enet_uint16,
    pub(crate) left: enet_uint16,
    pub(crate) right: enet_uint16,
    pub(crate) symbols: enet_uint16,
    pub(crate) escapes: enet_uint16,
    pub(crate) total: enet_uint16,
    pub(crate) parent: enet_uint16,
}
pub(crate) const ENET_CONTEXT_SYMBOL_MINIMUM: C2RustUnnamed_3 = 1;
pub(crate) const ENET_CONTEXT_ESCAPE_MINIMUM: C2RustUnnamed_3 = 1;
pub(crate) const ENET_SUBCONTEXT_ORDER: C2RustUnnamed_3 = 2;
pub(crate) const ENET_RANGE_CODER_BOTTOM: C2RustUnnamed_3 = 65536;
pub(crate) const ENET_SUBCONTEXT_SYMBOL_DELTA: C2RustUnnamed_3 = 2;
pub(crate) const ENET_SUBCONTEXT_ESCAPE_DELTA: C2RustUnnamed_3 = 5;
pub(crate) const ENET_CONTEXT_SYMBOL_DELTA: C2RustUnnamed_3 = 3;
pub(crate) const ENET_RANGE_CODER_TOP: C2RustUnnamed_3 = 16777216;
pub(crate) type C2RustUnnamed_3 = c_uint;
static mut callbacks: ENetCallbacks = unsafe {
    {
        let mut init = _ENetCallbacks {
            malloc: Some(_enet_malloc as unsafe extern "C" fn(size_t) -> *mut c_void),
            free: Some(_enet_free as unsafe extern "C" fn(*mut c_void) -> ()),
            no_memory: ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> !>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(_enet_abort as unsafe extern "C" fn() -> !)),
        };
        init
    }
};
pub(crate) unsafe fn enet_initialize_with_callbacks(
    mut version: ENetVersion,
    mut inits: *const ENetCallbacks,
) -> c_int {
    if version < ((1 as c_int) << 16 as c_int | (3 as c_int) << 8 as c_int | 0 as c_int) as c_uint {
        return -(1 as c_int);
    }
    if ((*inits).malloc).is_some() || ((*inits).free).is_some() {
        if ((*inits).malloc).is_none() || ((*inits).free).is_none() {
            return -(1 as c_int);
        }
        callbacks.malloc = (*inits).malloc;
        callbacks.free = (*inits).free;
    }
    if ((*inits).no_memory).is_some() {
        callbacks.no_memory = (*inits).no_memory;
    }
    return 0;
}
pub(crate) unsafe fn enet_linked_version() -> ENetVersion {
    return ((1 as c_int) << 16 as c_int | (3 as c_int) << 8 as c_int | 17 as c_int) as ENetVersion;
}
#[no_mangle]
pub(crate) unsafe extern "C" fn enet_malloc(mut size: size_t) -> *mut c_void {
    let mut memory: *mut c_void = (callbacks.malloc).expect("non-null function pointer")(size);
    if memory.is_null() {
        (callbacks.no_memory).expect("non-null function pointer")();
    }
    return memory;
}
#[no_mangle]
pub(crate) unsafe extern "C" fn enet_free(mut memory: *mut c_void) {
    (callbacks.free).expect("non-null function pointer")(memory);
}
#[no_mangle]
pub(crate) unsafe extern "C" fn enet_range_coder_create() -> *mut c_void {
    let mut rangeCoder: *mut ENetRangeCoder =
        enet_malloc(::core::mem::size_of::<ENetRangeCoder>() as size_t) as *mut ENetRangeCoder;
    if rangeCoder.is_null() {
        return 0 as *mut c_void;
    }
    return rangeCoder as *mut c_void;
}
#[no_mangle]
pub(crate) unsafe extern "C" fn enet_range_coder_destroy(mut context: *mut c_void) {
    let mut rangeCoder: *mut ENetRangeCoder = context as *mut ENetRangeCoder;
    if rangeCoder.is_null() {
        return;
    }
    enet_free(rangeCoder as *mut c_void);
}
unsafe extern "C" fn enet_symbol_rescale(mut symbol: *mut ENetSymbol) -> enet_uint16 {
    let mut total: enet_uint16 = 0 as c_int as enet_uint16;
    loop {
        (*symbol).count =
            ((*symbol).count as c_int - ((*symbol).count as c_int >> 1 as c_int)) as enet_uint8;
        (*symbol).under = (*symbol).count as enet_uint16;
        if (*symbol).left != 0 {
            (*symbol).under = ((*symbol).under as c_int
                + enet_symbol_rescale(symbol.offset((*symbol).left as c_int as isize)) as c_int)
                as enet_uint16;
        }
        total = (total as c_int + (*symbol).under as c_int) as enet_uint16;
        if (*symbol).right == 0 {
            break;
        }
        symbol = symbol.offset((*symbol).right as c_int as isize);
    }
    return total;
}
#[no_mangle]
pub(crate) unsafe extern "C" fn enet_range_coder_compress(
    mut context: *mut c_void,
    mut inBuffers: *const ENetBuffer,
    mut inBufferCount: size_t,
    mut inLimit: size_t,
    mut outData: *mut enet_uint8,
    mut outLimit: size_t,
) -> size_t {
    let mut rangeCoder: *mut ENetRangeCoder = context as *mut ENetRangeCoder;
    let mut outStart: *mut enet_uint8 = outData;
    let mut outEnd: *mut enet_uint8 = &mut *outData.offset(outLimit as isize) as *mut enet_uint8;
    let mut inData: *const enet_uint8 = 0 as *const enet_uint8;
    let mut inEnd: *const enet_uint8 = 0 as *const enet_uint8;
    let mut encodeLow: enet_uint32 = 0 as c_int as enet_uint32;
    let mut encodeRange: enet_uint32 = !(0 as c_int) as enet_uint32;
    let mut root: *mut ENetSymbol = 0 as *mut ENetSymbol;
    let mut predicted: enet_uint16 = 0 as c_int as enet_uint16;
    let mut order: size_t = 0 as c_int as size_t;
    let mut nextSymbol: size_t = 0 as c_int as size_t;
    if rangeCoder.is_null()
        || inBufferCount <= 0 as c_int as size_t
        || inLimit <= 0 as c_int as size_t
    {
        return 0 as c_int as size_t;
    }
    inData = (*inBuffers).data as *const enet_uint8;
    inEnd = &*inData.offset((*inBuffers).dataLength as isize) as *const enet_uint8;
    inBuffers = inBuffers.offset(1);
    inBufferCount = inBufferCount.wrapping_sub(1);
    let fresh0 = nextSymbol;
    nextSymbol = nextSymbol.wrapping_add(1);
    root = &mut *((*rangeCoder).symbols).as_mut_ptr().offset(fresh0 as isize) as *mut ENetSymbol;
    (*root).value = 0 as c_int as enet_uint8;
    (*root).count = 0 as c_int as enet_uint8;
    (*root).under = 0 as c_int as enet_uint16;
    (*root).left = 0 as c_int as enet_uint16;
    (*root).right = 0 as c_int as enet_uint16;
    (*root).symbols = 0 as c_int as enet_uint16;
    (*root).escapes = 0 as c_int as enet_uint16;
    (*root).total = 0 as c_int as enet_uint16;
    (*root).parent = 0 as c_int as enet_uint16;
    (*root).escapes = ENET_CONTEXT_ESCAPE_MINIMUM as c_int as enet_uint16;
    (*root).total = (ENET_CONTEXT_ESCAPE_MINIMUM as c_int
        + 256 as c_int * ENET_CONTEXT_SYMBOL_MINIMUM as c_int) as enet_uint16;
    (*root).symbols = 0 as c_int as enet_uint16;
    let mut current_block_237: u64;
    loop {
        let mut subcontext: *mut ENetSymbol = 0 as *mut ENetSymbol;
        let mut symbol: *mut ENetSymbol = 0 as *mut ENetSymbol;
        let mut value: enet_uint8 = 0;
        let mut count: enet_uint16 = 0;
        let mut under: enet_uint16 = 0;
        let mut parent: *mut enet_uint16 = &mut predicted;
        let mut total: enet_uint16 = 0;
        if inData >= inEnd {
            if inBufferCount <= 0 as c_int as size_t {
                break;
            }
            inData = (*inBuffers).data as *const enet_uint8;
            inEnd = &*inData.offset((*inBuffers).dataLength as isize) as *const enet_uint8;
            inBuffers = inBuffers.offset(1);
            inBufferCount = inBufferCount.wrapping_sub(1);
        }
        let fresh1 = inData;
        inData = inData.offset(1);
        value = *fresh1;
        subcontext = &mut *((*rangeCoder).symbols)
            .as_mut_ptr()
            .offset(predicted as isize) as *mut ENetSymbol;
        loop {
            if !(subcontext != root) {
                current_block_237 = 2463987395154258233;
                break;
            }
            under = (value as c_int * 0 as c_int) as enet_uint16;
            count = 0 as c_int as enet_uint16;
            if (*subcontext).symbols == 0 {
                let fresh2 = nextSymbol;
                nextSymbol = nextSymbol.wrapping_add(1);
                symbol = &mut *((*rangeCoder).symbols).as_mut_ptr().offset(fresh2 as isize)
                    as *mut ENetSymbol;
                (*symbol).value = value;
                (*symbol).count = ENET_SUBCONTEXT_SYMBOL_DELTA as c_int as enet_uint8;
                (*symbol).under = ENET_SUBCONTEXT_SYMBOL_DELTA as c_int as enet_uint16;
                (*symbol).left = 0 as c_int as enet_uint16;
                (*symbol).right = 0 as c_int as enet_uint16;
                (*symbol).symbols = 0 as c_int as enet_uint16;
                (*symbol).escapes = 0 as c_int as enet_uint16;
                (*symbol).total = 0 as c_int as enet_uint16;
                (*symbol).parent = 0 as c_int as enet_uint16;
                (*subcontext).symbols = symbol.offset_from(subcontext) as c_long as enet_uint16;
            } else {
                let mut node: *mut ENetSymbol =
                    subcontext.offset((*subcontext).symbols as c_int as isize);
                loop {
                    if (value as c_int) < (*node).value as c_int {
                        (*node).under = ((*node).under as c_int
                            + ENET_SUBCONTEXT_SYMBOL_DELTA as c_int)
                            as enet_uint16;
                        if (*node).left != 0 {
                            node = node.offset((*node).left as c_int as isize);
                        } else {
                            let fresh3 = nextSymbol;
                            nextSymbol = nextSymbol.wrapping_add(1);
                            symbol =
                                &mut *((*rangeCoder).symbols).as_mut_ptr().offset(fresh3 as isize)
                                    as *mut ENetSymbol;
                            (*symbol).value = value;
                            (*symbol).count = ENET_SUBCONTEXT_SYMBOL_DELTA as c_int as enet_uint8;
                            (*symbol).under = ENET_SUBCONTEXT_SYMBOL_DELTA as c_int as enet_uint16;
                            (*symbol).left = 0 as c_int as enet_uint16;
                            (*symbol).right = 0 as c_int as enet_uint16;
                            (*symbol).symbols = 0 as c_int as enet_uint16;
                            (*symbol).escapes = 0 as c_int as enet_uint16;
                            (*symbol).total = 0 as c_int as enet_uint16;
                            (*symbol).parent = 0 as c_int as enet_uint16;
                            (*node).left = symbol.offset_from(node) as c_long as enet_uint16;
                            break;
                        }
                    } else if value as c_int > (*node).value as c_int {
                        under = (under as c_int + (*node).under as c_int) as enet_uint16;
                        if (*node).right != 0 {
                            node = node.offset((*node).right as c_int as isize);
                        } else {
                            let fresh4 = nextSymbol;
                            nextSymbol = nextSymbol.wrapping_add(1);
                            symbol =
                                &mut *((*rangeCoder).symbols).as_mut_ptr().offset(fresh4 as isize)
                                    as *mut ENetSymbol;
                            (*symbol).value = value;
                            (*symbol).count = ENET_SUBCONTEXT_SYMBOL_DELTA as c_int as enet_uint8;
                            (*symbol).under = ENET_SUBCONTEXT_SYMBOL_DELTA as c_int as enet_uint16;
                            (*symbol).left = 0 as c_int as enet_uint16;
                            (*symbol).right = 0 as c_int as enet_uint16;
                            (*symbol).symbols = 0 as c_int as enet_uint16;
                            (*symbol).escapes = 0 as c_int as enet_uint16;
                            (*symbol).total = 0 as c_int as enet_uint16;
                            (*symbol).parent = 0 as c_int as enet_uint16;
                            (*node).right = symbol.offset_from(node) as c_long as enet_uint16;
                            break;
                        }
                    } else {
                        count = (count as c_int + (*node).count as c_int) as enet_uint16;
                        under = (under as c_int + ((*node).under as c_int - (*node).count as c_int))
                            as enet_uint16;
                        (*node).under = ((*node).under as c_int
                            + ENET_SUBCONTEXT_SYMBOL_DELTA as c_int)
                            as enet_uint16;
                        (*node).count = ((*node).count as c_int
                            + ENET_SUBCONTEXT_SYMBOL_DELTA as c_int)
                            as enet_uint8;
                        symbol = node;
                        break;
                    }
                }
            }
            *parent =
                symbol.offset_from(((*rangeCoder).symbols).as_mut_ptr()) as c_long as enet_uint16;
            parent = &mut (*symbol).parent;
            total = (*subcontext).total;
            if count as c_int > 0 as c_int {
                encodeRange = (encodeRange as c_uint).wrapping_div(total as c_uint) as enet_uint32
                    as enet_uint32;
                encodeLow = (encodeLow as c_uint).wrapping_add(
                    (((*subcontext).escapes as c_int + under as c_int) as c_uint)
                        .wrapping_mul(encodeRange),
                ) as enet_uint32 as enet_uint32;
                encodeRange = (encodeRange as c_uint).wrapping_mul(count as c_uint) as enet_uint32
                    as enet_uint32;
                loop {
                    if encodeLow ^ encodeLow.wrapping_add(encodeRange)
                        >= ENET_RANGE_CODER_TOP as c_int as c_uint
                    {
                        if encodeRange >= ENET_RANGE_CODER_BOTTOM as c_int as c_uint {
                            break;
                        }
                        encodeRange = encodeLow.wrapping_neg()
                            & (ENET_RANGE_CODER_BOTTOM as c_int - 1 as c_int) as c_uint;
                    }
                    if outData >= outEnd {
                        return 0 as c_int as size_t;
                    }
                    let fresh5 = outData;
                    outData = outData.offset(1);
                    *fresh5 = (encodeLow >> 24 as c_int) as enet_uint8;
                    encodeRange <<= 8 as c_int;
                    encodeLow <<= 8 as c_int;
                }
            } else {
                if (*subcontext).escapes as c_int > 0 as c_int
                    && ((*subcontext).escapes as c_int) < total as c_int
                {
                    encodeRange = (encodeRange as c_uint).wrapping_div(total as c_uint)
                        as enet_uint32 as enet_uint32;
                    encodeLow = (encodeLow as c_uint)
                        .wrapping_add((0 as c_int as c_uint).wrapping_mul(encodeRange))
                        as enet_uint32 as enet_uint32;
                    encodeRange = (encodeRange as c_uint)
                        .wrapping_mul((*subcontext).escapes as c_uint)
                        as enet_uint32 as enet_uint32;
                    loop {
                        if encodeLow ^ encodeLow.wrapping_add(encodeRange)
                            >= ENET_RANGE_CODER_TOP as c_int as c_uint
                        {
                            if encodeRange >= ENET_RANGE_CODER_BOTTOM as c_int as c_uint {
                                break;
                            }
                            encodeRange = encodeLow.wrapping_neg()
                                & (ENET_RANGE_CODER_BOTTOM as c_int - 1 as c_int) as c_uint;
                        }
                        if outData >= outEnd {
                            return 0 as c_int as size_t;
                        }
                        let fresh6 = outData;
                        outData = outData.offset(1);
                        *fresh6 = (encodeLow >> 24 as c_int) as enet_uint8;
                        encodeRange <<= 8 as c_int;
                        encodeLow <<= 8 as c_int;
                    }
                }
                (*subcontext).escapes = ((*subcontext).escapes as c_int
                    + ENET_SUBCONTEXT_ESCAPE_DELTA as c_int)
                    as enet_uint16;
                (*subcontext).total = ((*subcontext).total as c_int
                    + ENET_SUBCONTEXT_ESCAPE_DELTA as c_int)
                    as enet_uint16;
            }
            (*subcontext).total = ((*subcontext).total as c_int
                + ENET_SUBCONTEXT_SYMBOL_DELTA as c_int)
                as enet_uint16;
            if count as c_int > 0xff as c_int - 2 as c_int * ENET_SUBCONTEXT_SYMBOL_DELTA as c_int
                || (*subcontext).total as c_int > ENET_RANGE_CODER_BOTTOM as c_int - 0x100 as c_int
            {
                (*subcontext).total = (if (*subcontext).symbols as c_int != 0 {
                    enet_symbol_rescale(subcontext.offset((*subcontext).symbols as c_int as isize))
                        as c_int
                } else {
                    0 as c_int
                }) as enet_uint16;
                (*subcontext).escapes = ((*subcontext).escapes as c_int
                    - ((*subcontext).escapes as c_int >> 1 as c_int))
                    as enet_uint16;
                (*subcontext).total = ((*subcontext).total as c_int
                    + ((*subcontext).escapes as c_int + 256 as c_int * 0 as c_int))
                    as enet_uint16;
            }
            if count as c_int > 0 as c_int {
                current_block_237 = 836937598693885467;
                break;
            }
            subcontext = &mut *((*rangeCoder).symbols)
                .as_mut_ptr()
                .offset((*subcontext).parent as isize) as *mut ENetSymbol;
        }
        match current_block_237 {
            2463987395154258233 => {
                under = (value as c_int * ENET_CONTEXT_SYMBOL_MINIMUM as c_int) as enet_uint16;
                count = ENET_CONTEXT_SYMBOL_MINIMUM as c_int as enet_uint16;
                if (*root).symbols == 0 {
                    let fresh7 = nextSymbol;
                    nextSymbol = nextSymbol.wrapping_add(1);
                    symbol = &mut *((*rangeCoder).symbols).as_mut_ptr().offset(fresh7 as isize)
                        as *mut ENetSymbol;
                    (*symbol).value = value;
                    (*symbol).count = ENET_CONTEXT_SYMBOL_DELTA as c_int as enet_uint8;
                    (*symbol).under = ENET_CONTEXT_SYMBOL_DELTA as c_int as enet_uint16;
                    (*symbol).left = 0 as c_int as enet_uint16;
                    (*symbol).right = 0 as c_int as enet_uint16;
                    (*symbol).symbols = 0 as c_int as enet_uint16;
                    (*symbol).escapes = 0 as c_int as enet_uint16;
                    (*symbol).total = 0 as c_int as enet_uint16;
                    (*symbol).parent = 0 as c_int as enet_uint16;
                    (*root).symbols = symbol.offset_from(root) as c_long as enet_uint16;
                } else {
                    let mut node_0: *mut ENetSymbol =
                        root.offset((*root).symbols as c_int as isize);
                    loop {
                        if (value as c_int) < (*node_0).value as c_int {
                            (*node_0).under = ((*node_0).under as c_int
                                + ENET_CONTEXT_SYMBOL_DELTA as c_int)
                                as enet_uint16;
                            if (*node_0).left != 0 {
                                node_0 = node_0.offset((*node_0).left as c_int as isize);
                            } else {
                                let fresh8 = nextSymbol;
                                nextSymbol = nextSymbol.wrapping_add(1);
                                symbol = &mut *((*rangeCoder).symbols)
                                    .as_mut_ptr()
                                    .offset(fresh8 as isize)
                                    as *mut ENetSymbol;
                                (*symbol).value = value;
                                (*symbol).count = ENET_CONTEXT_SYMBOL_DELTA as c_int as enet_uint8;
                                (*symbol).under = ENET_CONTEXT_SYMBOL_DELTA as c_int as enet_uint16;
                                (*symbol).left = 0 as c_int as enet_uint16;
                                (*symbol).right = 0 as c_int as enet_uint16;
                                (*symbol).symbols = 0 as c_int as enet_uint16;
                                (*symbol).escapes = 0 as c_int as enet_uint16;
                                (*symbol).total = 0 as c_int as enet_uint16;
                                (*symbol).parent = 0 as c_int as enet_uint16;
                                (*node_0).left =
                                    symbol.offset_from(node_0) as c_long as enet_uint16;
                                break;
                            }
                        } else if value as c_int > (*node_0).value as c_int {
                            under = (under as c_int + (*node_0).under as c_int) as enet_uint16;
                            if (*node_0).right != 0 {
                                node_0 = node_0.offset((*node_0).right as c_int as isize);
                            } else {
                                let fresh9 = nextSymbol;
                                nextSymbol = nextSymbol.wrapping_add(1);
                                symbol = &mut *((*rangeCoder).symbols)
                                    .as_mut_ptr()
                                    .offset(fresh9 as isize)
                                    as *mut ENetSymbol;
                                (*symbol).value = value;
                                (*symbol).count = ENET_CONTEXT_SYMBOL_DELTA as c_int as enet_uint8;
                                (*symbol).under = ENET_CONTEXT_SYMBOL_DELTA as c_int as enet_uint16;
                                (*symbol).left = 0 as c_int as enet_uint16;
                                (*symbol).right = 0 as c_int as enet_uint16;
                                (*symbol).symbols = 0 as c_int as enet_uint16;
                                (*symbol).escapes = 0 as c_int as enet_uint16;
                                (*symbol).total = 0 as c_int as enet_uint16;
                                (*symbol).parent = 0 as c_int as enet_uint16;
                                (*node_0).right =
                                    symbol.offset_from(node_0) as c_long as enet_uint16;
                                break;
                            }
                        } else {
                            count = (count as c_int + (*node_0).count as c_int) as enet_uint16;
                            under = (under as c_int
                                + ((*node_0).under as c_int - (*node_0).count as c_int))
                                as enet_uint16;
                            (*node_0).under = ((*node_0).under as c_int
                                + ENET_CONTEXT_SYMBOL_DELTA as c_int)
                                as enet_uint16;
                            (*node_0).count = ((*node_0).count as c_int
                                + ENET_CONTEXT_SYMBOL_DELTA as c_int)
                                as enet_uint8;
                            symbol = node_0;
                            break;
                        }
                    }
                }
                *parent = symbol.offset_from(((*rangeCoder).symbols).as_mut_ptr()) as c_long
                    as enet_uint16;
                parent = &mut (*symbol).parent;
                total = (*root).total;
                encodeRange = (encodeRange as c_uint).wrapping_div(total as c_uint) as enet_uint32
                    as enet_uint32;
                encodeLow = (encodeLow as c_uint).wrapping_add(
                    (((*root).escapes as c_int + under as c_int) as c_uint)
                        .wrapping_mul(encodeRange),
                ) as enet_uint32 as enet_uint32;
                encodeRange = (encodeRange as c_uint).wrapping_mul(count as c_uint) as enet_uint32
                    as enet_uint32;
                loop {
                    if encodeLow ^ encodeLow.wrapping_add(encodeRange)
                        >= ENET_RANGE_CODER_TOP as c_int as c_uint
                    {
                        if encodeRange >= ENET_RANGE_CODER_BOTTOM as c_int as c_uint {
                            break;
                        }
                        encodeRange = encodeLow.wrapping_neg()
                            & (ENET_RANGE_CODER_BOTTOM as c_int - 1 as c_int) as c_uint;
                    }
                    if outData >= outEnd {
                        return 0 as c_int as size_t;
                    }
                    let fresh10 = outData;
                    outData = outData.offset(1);
                    *fresh10 = (encodeLow >> 24 as c_int) as enet_uint8;
                    encodeRange <<= 8 as c_int;
                    encodeLow <<= 8 as c_int;
                }
                (*root).total =
                    ((*root).total as c_int + ENET_CONTEXT_SYMBOL_DELTA as c_int) as enet_uint16;
                if count as c_int
                    > 0xff as c_int - 2 as c_int * ENET_CONTEXT_SYMBOL_DELTA as c_int
                        + ENET_CONTEXT_SYMBOL_MINIMUM as c_int
                    || (*root).total as c_int > ENET_RANGE_CODER_BOTTOM as c_int - 0x100 as c_int
                {
                    (*root).total = (if (*root).symbols as c_int != 0 {
                        enet_symbol_rescale(root.offset((*root).symbols as c_int as isize)) as c_int
                    } else {
                        0 as c_int
                    }) as enet_uint16;
                    (*root).escapes = ((*root).escapes as c_int
                        - ((*root).escapes as c_int >> 1 as c_int))
                        as enet_uint16;
                    (*root).total = ((*root).total as c_int
                        + ((*root).escapes as c_int
                            + 256 as c_int * ENET_CONTEXT_SYMBOL_MINIMUM as c_int))
                        as enet_uint16;
                }
            }
            _ => {}
        }
        if order >= ENET_SUBCONTEXT_ORDER as c_int as size_t {
            predicted = (*rangeCoder).symbols[predicted as usize].parent;
        } else {
            order = order.wrapping_add(1);
        }
        if nextSymbol
            >= (::core::mem::size_of::<[ENetSymbol; 4096]>() as size_t)
                .wrapping_div(::core::mem::size_of::<ENetSymbol>() as size_t)
                .wrapping_sub(ENET_SUBCONTEXT_ORDER as c_int as size_t)
        {
            nextSymbol = 0 as c_int as size_t;
            let fresh11 = nextSymbol;
            nextSymbol = nextSymbol.wrapping_add(1);
            root = &mut *((*rangeCoder).symbols)
                .as_mut_ptr()
                .offset(fresh11 as isize) as *mut ENetSymbol;
            (*root).value = 0 as c_int as enet_uint8;
            (*root).count = 0 as c_int as enet_uint8;
            (*root).under = 0 as c_int as enet_uint16;
            (*root).left = 0 as c_int as enet_uint16;
            (*root).right = 0 as c_int as enet_uint16;
            (*root).symbols = 0 as c_int as enet_uint16;
            (*root).escapes = 0 as c_int as enet_uint16;
            (*root).total = 0 as c_int as enet_uint16;
            (*root).parent = 0 as c_int as enet_uint16;
            (*root).escapes = ENET_CONTEXT_ESCAPE_MINIMUM as c_int as enet_uint16;
            (*root).total = (ENET_CONTEXT_ESCAPE_MINIMUM as c_int
                + 256 as c_int * ENET_CONTEXT_SYMBOL_MINIMUM as c_int)
                as enet_uint16;
            (*root).symbols = 0 as c_int as enet_uint16;
            predicted = 0 as c_int as enet_uint16;
            order = 0 as c_int as size_t;
        }
    }
    while encodeLow != 0 {
        if outData >= outEnd {
            return 0 as c_int as size_t;
        }
        let fresh12 = outData;
        outData = outData.offset(1);
        *fresh12 = (encodeLow >> 24 as c_int) as enet_uint8;
        encodeLow <<= 8 as c_int;
    }
    return outData.offset_from(outStart) as c_long as size_t;
}
#[no_mangle]
pub(crate) unsafe extern "C" fn enet_range_coder_decompress(
    mut context: *mut c_void,
    mut inData: *const enet_uint8,
    mut inLimit: size_t,
    mut outData: *mut enet_uint8,
    mut outLimit: size_t,
) -> size_t {
    let mut rangeCoder: *mut ENetRangeCoder = context as *mut ENetRangeCoder;
    let mut outStart: *mut enet_uint8 = outData;
    let mut outEnd: *mut enet_uint8 = &mut *outData.offset(outLimit as isize) as *mut enet_uint8;
    let mut inEnd: *const enet_uint8 = &*inData.offset(inLimit as isize) as *const enet_uint8;
    let mut decodeLow: enet_uint32 = 0 as c_int as enet_uint32;
    let mut decodeCode: enet_uint32 = 0 as c_int as enet_uint32;
    let mut decodeRange: enet_uint32 = !(0 as c_int) as enet_uint32;
    let mut root: *mut ENetSymbol = 0 as *mut ENetSymbol;
    let mut predicted: enet_uint16 = 0 as c_int as enet_uint16;
    let mut order: size_t = 0 as c_int as size_t;
    let mut nextSymbol: size_t = 0 as c_int as size_t;
    if rangeCoder.is_null() || inLimit <= 0 as c_int as size_t {
        return 0 as c_int as size_t;
    }
    let fresh13 = nextSymbol;
    nextSymbol = nextSymbol.wrapping_add(1);
    root = &mut *((*rangeCoder).symbols)
        .as_mut_ptr()
        .offset(fresh13 as isize) as *mut ENetSymbol;
    (*root).value = 0 as c_int as enet_uint8;
    (*root).count = 0 as c_int as enet_uint8;
    (*root).under = 0 as c_int as enet_uint16;
    (*root).left = 0 as c_int as enet_uint16;
    (*root).right = 0 as c_int as enet_uint16;
    (*root).symbols = 0 as c_int as enet_uint16;
    (*root).escapes = 0 as c_int as enet_uint16;
    (*root).total = 0 as c_int as enet_uint16;
    (*root).parent = 0 as c_int as enet_uint16;
    (*root).escapes = ENET_CONTEXT_ESCAPE_MINIMUM as c_int as enet_uint16;
    (*root).total = (ENET_CONTEXT_ESCAPE_MINIMUM as c_int
        + 256 as c_int * ENET_CONTEXT_SYMBOL_MINIMUM as c_int) as enet_uint16;
    (*root).symbols = 0 as c_int as enet_uint16;
    if inData < inEnd {
        let fresh14 = inData;
        inData = inData.offset(1);
        decodeCode |= ((*fresh14 as c_int) << 24 as c_int) as c_uint;
    }
    if inData < inEnd {
        let fresh15 = inData;
        inData = inData.offset(1);
        decodeCode |= ((*fresh15 as c_int) << 16 as c_int) as c_uint;
    }
    if inData < inEnd {
        let fresh16 = inData;
        inData = inData.offset(1);
        decodeCode |= ((*fresh16 as c_int) << 8 as c_int) as c_uint;
    }
    if inData < inEnd {
        let fresh17 = inData;
        inData = inData.offset(1);
        decodeCode |= *fresh17 as c_uint;
    }
    let mut current_block_297: u64;
    loop {
        let mut subcontext: *mut ENetSymbol = 0 as *mut ENetSymbol;
        let mut symbol: *mut ENetSymbol = 0 as *mut ENetSymbol;
        let mut patch: *mut ENetSymbol = 0 as *mut ENetSymbol;
        let mut value: enet_uint8 = 0 as c_int as enet_uint8;
        let mut code: enet_uint16 = 0;
        let mut under: enet_uint16 = 0;
        let mut count: enet_uint16 = 0;
        let mut bottom: enet_uint16 = 0;
        let mut parent: *mut enet_uint16 = &mut predicted;
        let mut total: enet_uint16 = 0;
        subcontext = &mut *((*rangeCoder).symbols)
            .as_mut_ptr()
            .offset(predicted as isize) as *mut ENetSymbol;
        loop {
            if !(subcontext != root) {
                current_block_297 = 18325745679564279244;
                break;
            }
            if !((*subcontext).escapes as c_int <= 0 as c_int) {
                total = (*subcontext).total;
                if !((*subcontext).escapes as c_int >= total as c_int) {
                    decodeRange = (decodeRange as c_uint).wrapping_div(total as c_uint)
                        as enet_uint32 as enet_uint32;
                    code =
                        decodeCode.wrapping_sub(decodeLow).wrapping_div(decodeRange) as enet_uint16;
                    if (code as c_int) < (*subcontext).escapes as c_int {
                        decodeLow = (decodeLow as c_uint)
                            .wrapping_add((0 as c_int as c_uint).wrapping_mul(decodeRange))
                            as enet_uint32 as enet_uint32;
                        decodeRange = (decodeRange as c_uint)
                            .wrapping_mul((*subcontext).escapes as c_uint)
                            as enet_uint32 as enet_uint32;
                        loop {
                            if decodeLow ^ decodeLow.wrapping_add(decodeRange)
                                >= ENET_RANGE_CODER_TOP as c_int as c_uint
                            {
                                if decodeRange >= ENET_RANGE_CODER_BOTTOM as c_int as c_uint {
                                    break;
                                }
                                decodeRange = decodeLow.wrapping_neg()
                                    & (ENET_RANGE_CODER_BOTTOM as c_int - 1 as c_int) as c_uint;
                            }
                            decodeCode <<= 8 as c_int;
                            if inData < inEnd {
                                let fresh18 = inData;
                                inData = inData.offset(1);
                                decodeCode |= *fresh18 as c_uint;
                            }
                            decodeRange <<= 8 as c_int;
                            decodeLow <<= 8 as c_int;
                        }
                    } else {
                        code = (code as c_int - (*subcontext).escapes as c_int) as enet_uint16;
                        under = 0 as c_int as enet_uint16;
                        count = 0 as c_int as enet_uint16;
                        if (*subcontext).symbols == 0 {
                            return 0 as c_int as size_t;
                        } else {
                            let mut node: *mut ENetSymbol =
                                subcontext.offset((*subcontext).symbols as c_int as isize);
                            loop {
                                let mut after: enet_uint16 = (under as c_int
                                    + (*node).under as c_int
                                    + ((*node).value as c_int + 1 as c_int) * 0 as c_int)
                                    as enet_uint16;
                                let mut before: enet_uint16 =
                                    ((*node).count as c_int + 0 as c_int) as enet_uint16;
                                if code as c_int >= after as c_int {
                                    under =
                                        (under as c_int + (*node).under as c_int) as enet_uint16;
                                    if (*node).right != 0 {
                                        node = node.offset((*node).right as c_int as isize);
                                    } else {
                                        return 0 as c_int as size_t;
                                    }
                                } else if (code as c_int) < after as c_int - before as c_int {
                                    (*node).under = ((*node).under as c_int
                                        + ENET_SUBCONTEXT_SYMBOL_DELTA as c_int)
                                        as enet_uint16;
                                    if (*node).left != 0 {
                                        node = node.offset((*node).left as c_int as isize);
                                    } else {
                                        return 0 as c_int as size_t;
                                    }
                                } else {
                                    value = (*node).value;
                                    count =
                                        (count as c_int + (*node).count as c_int) as enet_uint16;
                                    under = (after as c_int - before as c_int) as enet_uint16;
                                    (*node).under = ((*node).under as c_int
                                        + ENET_SUBCONTEXT_SYMBOL_DELTA as c_int)
                                        as enet_uint16;
                                    (*node).count = ((*node).count as c_int
                                        + ENET_SUBCONTEXT_SYMBOL_DELTA as c_int)
                                        as enet_uint8;
                                    symbol = node;
                                    break;
                                }
                            }
                        }
                        bottom = symbol.offset_from(((*rangeCoder).symbols).as_mut_ptr()) as c_long
                            as enet_uint16;
                        decodeLow = (decodeLow as c_uint).wrapping_add(
                            (((*subcontext).escapes as c_int + under as c_int) as c_uint)
                                .wrapping_mul(decodeRange),
                        ) as enet_uint32 as enet_uint32;
                        decodeRange = (decodeRange as c_uint).wrapping_mul(count as c_uint)
                            as enet_uint32 as enet_uint32;
                        loop {
                            if decodeLow ^ decodeLow.wrapping_add(decodeRange)
                                >= ENET_RANGE_CODER_TOP as c_int as c_uint
                            {
                                if decodeRange >= ENET_RANGE_CODER_BOTTOM as c_int as c_uint {
                                    break;
                                }
                                decodeRange = decodeLow.wrapping_neg()
                                    & (ENET_RANGE_CODER_BOTTOM as c_int - 1 as c_int) as c_uint;
                            }
                            decodeCode <<= 8 as c_int;
                            if inData < inEnd {
                                let fresh19 = inData;
                                inData = inData.offset(1);
                                decodeCode |= *fresh19 as c_uint;
                            }
                            decodeRange <<= 8 as c_int;
                            decodeLow <<= 8 as c_int;
                        }
                        (*subcontext).total = ((*subcontext).total as c_int
                            + ENET_SUBCONTEXT_SYMBOL_DELTA as c_int)
                            as enet_uint16;
                        if count as c_int
                            > 0xff as c_int - 2 as c_int * ENET_SUBCONTEXT_SYMBOL_DELTA as c_int
                            || (*subcontext).total as c_int
                                > ENET_RANGE_CODER_BOTTOM as c_int - 0x100 as c_int
                        {
                            (*subcontext).total = (if (*subcontext).symbols as c_int != 0 {
                                enet_symbol_rescale(
                                    subcontext.offset((*subcontext).symbols as c_int as isize),
                                ) as c_int
                            } else {
                                0 as c_int
                            }) as enet_uint16;
                            (*subcontext).escapes = ((*subcontext).escapes as c_int
                                - ((*subcontext).escapes as c_int >> 1 as c_int))
                                as enet_uint16;
                            (*subcontext).total = ((*subcontext).total as c_int
                                + ((*subcontext).escapes as c_int + 256 as c_int * 0 as c_int))
                                as enet_uint16;
                        }
                        current_block_297 = 16234561804784670422;
                        break;
                    }
                }
            }
            subcontext = &mut *((*rangeCoder).symbols)
                .as_mut_ptr()
                .offset((*subcontext).parent as isize) as *mut ENetSymbol;
        }
        match current_block_297 {
            18325745679564279244 => {
                total = (*root).total;
                decodeRange = (decodeRange as c_uint).wrapping_div(total as c_uint) as enet_uint32
                    as enet_uint32;
                code = decodeCode.wrapping_sub(decodeLow).wrapping_div(decodeRange) as enet_uint16;
                if (code as c_int) < (*root).escapes as c_int {
                    decodeLow = (decodeLow as c_uint)
                        .wrapping_add((0 as c_int as c_uint).wrapping_mul(decodeRange))
                        as enet_uint32 as enet_uint32;
                    decodeRange = (decodeRange as c_uint).wrapping_mul((*root).escapes as c_uint)
                        as enet_uint32 as enet_uint32;
                    loop {
                        if decodeLow ^ decodeLow.wrapping_add(decodeRange)
                            >= ENET_RANGE_CODER_TOP as c_int as c_uint
                        {
                            if decodeRange >= ENET_RANGE_CODER_BOTTOM as c_int as c_uint {
                                break;
                            }
                            decodeRange = decodeLow.wrapping_neg()
                                & (ENET_RANGE_CODER_BOTTOM as c_int - 1 as c_int) as c_uint;
                        }
                        decodeCode <<= 8 as c_int;
                        if inData < inEnd {
                            let fresh20 = inData;
                            inData = inData.offset(1);
                            decodeCode |= *fresh20 as c_uint;
                        }
                        decodeRange <<= 8 as c_int;
                        decodeLow <<= 8 as c_int;
                    }
                    break;
                } else {
                    code = (code as c_int - (*root).escapes as c_int) as enet_uint16;
                    under = 0 as c_int as enet_uint16;
                    count = ENET_CONTEXT_SYMBOL_MINIMUM as c_int as enet_uint16;
                    if (*root).symbols == 0 {
                        value =
                            (code as c_int / ENET_CONTEXT_SYMBOL_MINIMUM as c_int) as enet_uint8;
                        under = (code as c_int
                            - code as c_int % ENET_CONTEXT_SYMBOL_MINIMUM as c_int)
                            as enet_uint16;
                        let fresh21 = nextSymbol;
                        nextSymbol = nextSymbol.wrapping_add(1);
                        symbol = &mut *((*rangeCoder).symbols)
                            .as_mut_ptr()
                            .offset(fresh21 as isize)
                            as *mut ENetSymbol;
                        (*symbol).value = value;
                        (*symbol).count = ENET_CONTEXT_SYMBOL_DELTA as c_int as enet_uint8;
                        (*symbol).under = ENET_CONTEXT_SYMBOL_DELTA as c_int as enet_uint16;
                        (*symbol).left = 0 as c_int as enet_uint16;
                        (*symbol).right = 0 as c_int as enet_uint16;
                        (*symbol).symbols = 0 as c_int as enet_uint16;
                        (*symbol).escapes = 0 as c_int as enet_uint16;
                        (*symbol).total = 0 as c_int as enet_uint16;
                        (*symbol).parent = 0 as c_int as enet_uint16;
                        (*root).symbols = symbol.offset_from(root) as c_long as enet_uint16;
                    } else {
                        let mut node_0: *mut ENetSymbol =
                            root.offset((*root).symbols as c_int as isize);
                        loop {
                            let mut after_0: enet_uint16 = (under as c_int
                                + (*node_0).under as c_int
                                + ((*node_0).value as c_int + 1 as c_int)
                                    * ENET_CONTEXT_SYMBOL_MINIMUM as c_int)
                                as enet_uint16;
                            let mut before_0: enet_uint16 = ((*node_0).count as c_int
                                + ENET_CONTEXT_SYMBOL_MINIMUM as c_int)
                                as enet_uint16;
                            if code as c_int >= after_0 as c_int {
                                under = (under as c_int + (*node_0).under as c_int) as enet_uint16;
                                if (*node_0).right != 0 {
                                    node_0 = node_0.offset((*node_0).right as c_int as isize);
                                } else {
                                    value = ((*node_0).value as c_int
                                        + 1 as c_int
                                        + (code as c_int - after_0 as c_int)
                                            / ENET_CONTEXT_SYMBOL_MINIMUM as c_int)
                                        as enet_uint8;
                                    under = (code as c_int
                                        - (code as c_int - after_0 as c_int)
                                            % ENET_CONTEXT_SYMBOL_MINIMUM as c_int)
                                        as enet_uint16;
                                    let fresh22 = nextSymbol;
                                    nextSymbol = nextSymbol.wrapping_add(1);
                                    symbol = &mut *((*rangeCoder).symbols)
                                        .as_mut_ptr()
                                        .offset(fresh22 as isize)
                                        as *mut ENetSymbol;
                                    (*symbol).value = value;
                                    (*symbol).count =
                                        ENET_CONTEXT_SYMBOL_DELTA as c_int as enet_uint8;
                                    (*symbol).under =
                                        ENET_CONTEXT_SYMBOL_DELTA as c_int as enet_uint16;
                                    (*symbol).left = 0 as c_int as enet_uint16;
                                    (*symbol).right = 0 as c_int as enet_uint16;
                                    (*symbol).symbols = 0 as c_int as enet_uint16;
                                    (*symbol).escapes = 0 as c_int as enet_uint16;
                                    (*symbol).total = 0 as c_int as enet_uint16;
                                    (*symbol).parent = 0 as c_int as enet_uint16;
                                    (*node_0).right =
                                        symbol.offset_from(node_0) as c_long as enet_uint16;
                                    break;
                                }
                            } else if (code as c_int) < after_0 as c_int - before_0 as c_int {
                                (*node_0).under = ((*node_0).under as c_int
                                    + ENET_CONTEXT_SYMBOL_DELTA as c_int)
                                    as enet_uint16;
                                if (*node_0).left != 0 {
                                    node_0 = node_0.offset((*node_0).left as c_int as isize);
                                } else {
                                    value = ((*node_0).value as c_int
                                        - 1 as c_int
                                        - (after_0 as c_int
                                            - before_0 as c_int
                                            - code as c_int
                                            - 1 as c_int)
                                            / ENET_CONTEXT_SYMBOL_MINIMUM as c_int)
                                        as enet_uint8;
                                    under = (code as c_int
                                        - (after_0 as c_int
                                            - before_0 as c_int
                                            - code as c_int
                                            - 1 as c_int)
                                            % ENET_CONTEXT_SYMBOL_MINIMUM as c_int)
                                        as enet_uint16;
                                    let fresh23 = nextSymbol;
                                    nextSymbol = nextSymbol.wrapping_add(1);
                                    symbol = &mut *((*rangeCoder).symbols)
                                        .as_mut_ptr()
                                        .offset(fresh23 as isize)
                                        as *mut ENetSymbol;
                                    (*symbol).value = value;
                                    (*symbol).count =
                                        ENET_CONTEXT_SYMBOL_DELTA as c_int as enet_uint8;
                                    (*symbol).under =
                                        ENET_CONTEXT_SYMBOL_DELTA as c_int as enet_uint16;
                                    (*symbol).left = 0 as c_int as enet_uint16;
                                    (*symbol).right = 0 as c_int as enet_uint16;
                                    (*symbol).symbols = 0 as c_int as enet_uint16;
                                    (*symbol).escapes = 0 as c_int as enet_uint16;
                                    (*symbol).total = 0 as c_int as enet_uint16;
                                    (*symbol).parent = 0 as c_int as enet_uint16;
                                    (*node_0).left =
                                        symbol.offset_from(node_0) as c_long as enet_uint16;
                                    break;
                                }
                            } else {
                                value = (*node_0).value;
                                count = (count as c_int + (*node_0).count as c_int) as enet_uint16;
                                under = (after_0 as c_int - before_0 as c_int) as enet_uint16;
                                (*node_0).under = ((*node_0).under as c_int
                                    + ENET_CONTEXT_SYMBOL_DELTA as c_int)
                                    as enet_uint16;
                                (*node_0).count = ((*node_0).count as c_int
                                    + ENET_CONTEXT_SYMBOL_DELTA as c_int)
                                    as enet_uint8;
                                symbol = node_0;
                                break;
                            }
                        }
                    }
                    bottom = symbol.offset_from(((*rangeCoder).symbols).as_mut_ptr()) as c_long
                        as enet_uint16;
                    decodeLow = (decodeLow as c_uint).wrapping_add(
                        (((*root).escapes as c_int + under as c_int) as c_uint)
                            .wrapping_mul(decodeRange),
                    ) as enet_uint32 as enet_uint32;
                    decodeRange = (decodeRange as c_uint).wrapping_mul(count as c_uint)
                        as enet_uint32 as enet_uint32;
                    loop {
                        if decodeLow ^ decodeLow.wrapping_add(decodeRange)
                            >= ENET_RANGE_CODER_TOP as c_int as c_uint
                        {
                            if decodeRange >= ENET_RANGE_CODER_BOTTOM as c_int as c_uint {
                                break;
                            }
                            decodeRange = decodeLow.wrapping_neg()
                                & (ENET_RANGE_CODER_BOTTOM as c_int - 1 as c_int) as c_uint;
                        }
                        decodeCode <<= 8 as c_int;
                        if inData < inEnd {
                            let fresh24 = inData;
                            inData = inData.offset(1);
                            decodeCode |= *fresh24 as c_uint;
                        }
                        decodeRange <<= 8 as c_int;
                        decodeLow <<= 8 as c_int;
                    }
                    (*root).total = ((*root).total as c_int + ENET_CONTEXT_SYMBOL_DELTA as c_int)
                        as enet_uint16;
                    if count as c_int
                        > 0xff as c_int - 2 as c_int * ENET_CONTEXT_SYMBOL_DELTA as c_int
                            + ENET_CONTEXT_SYMBOL_MINIMUM as c_int
                        || (*root).total as c_int
                            > ENET_RANGE_CODER_BOTTOM as c_int - 0x100 as c_int
                    {
                        (*root).total = (if (*root).symbols as c_int != 0 {
                            enet_symbol_rescale(root.offset((*root).symbols as c_int as isize))
                                as c_int
                        } else {
                            0 as c_int
                        }) as enet_uint16;
                        (*root).escapes = ((*root).escapes as c_int
                            - ((*root).escapes as c_int >> 1 as c_int))
                            as enet_uint16;
                        (*root).total = ((*root).total as c_int
                            + ((*root).escapes as c_int
                                + 256 as c_int * ENET_CONTEXT_SYMBOL_MINIMUM as c_int))
                            as enet_uint16;
                    }
                }
            }
            _ => {}
        }
        patch = &mut *((*rangeCoder).symbols)
            .as_mut_ptr()
            .offset(predicted as isize) as *mut ENetSymbol;
        while patch != subcontext {
            under = (value as c_int * 0 as c_int) as enet_uint16;
            count = 0 as c_int as enet_uint16;
            if (*patch).symbols == 0 {
                let fresh25 = nextSymbol;
                nextSymbol = nextSymbol.wrapping_add(1);
                symbol = &mut *((*rangeCoder).symbols)
                    .as_mut_ptr()
                    .offset(fresh25 as isize) as *mut ENetSymbol;
                (*symbol).value = value;
                (*symbol).count = ENET_SUBCONTEXT_SYMBOL_DELTA as c_int as enet_uint8;
                (*symbol).under = ENET_SUBCONTEXT_SYMBOL_DELTA as c_int as enet_uint16;
                (*symbol).left = 0 as c_int as enet_uint16;
                (*symbol).right = 0 as c_int as enet_uint16;
                (*symbol).symbols = 0 as c_int as enet_uint16;
                (*symbol).escapes = 0 as c_int as enet_uint16;
                (*symbol).total = 0 as c_int as enet_uint16;
                (*symbol).parent = 0 as c_int as enet_uint16;
                (*patch).symbols = symbol.offset_from(patch) as c_long as enet_uint16;
            } else {
                let mut node_1: *mut ENetSymbol = patch.offset((*patch).symbols as c_int as isize);
                loop {
                    if (value as c_int) < (*node_1).value as c_int {
                        (*node_1).under = ((*node_1).under as c_int
                            + ENET_SUBCONTEXT_SYMBOL_DELTA as c_int)
                            as enet_uint16;
                        if (*node_1).left != 0 {
                            node_1 = node_1.offset((*node_1).left as c_int as isize);
                        } else {
                            let fresh26 = nextSymbol;
                            nextSymbol = nextSymbol.wrapping_add(1);
                            symbol = &mut *((*rangeCoder).symbols)
                                .as_mut_ptr()
                                .offset(fresh26 as isize)
                                as *mut ENetSymbol;
                            (*symbol).value = value;
                            (*symbol).count = ENET_SUBCONTEXT_SYMBOL_DELTA as c_int as enet_uint8;
                            (*symbol).under = ENET_SUBCONTEXT_SYMBOL_DELTA as c_int as enet_uint16;
                            (*symbol).left = 0 as c_int as enet_uint16;
                            (*symbol).right = 0 as c_int as enet_uint16;
                            (*symbol).symbols = 0 as c_int as enet_uint16;
                            (*symbol).escapes = 0 as c_int as enet_uint16;
                            (*symbol).total = 0 as c_int as enet_uint16;
                            (*symbol).parent = 0 as c_int as enet_uint16;
                            (*node_1).left = symbol.offset_from(node_1) as c_long as enet_uint16;
                            break;
                        }
                    } else if value as c_int > (*node_1).value as c_int {
                        under = (under as c_int + (*node_1).under as c_int) as enet_uint16;
                        if (*node_1).right != 0 {
                            node_1 = node_1.offset((*node_1).right as c_int as isize);
                        } else {
                            let fresh27 = nextSymbol;
                            nextSymbol = nextSymbol.wrapping_add(1);
                            symbol = &mut *((*rangeCoder).symbols)
                                .as_mut_ptr()
                                .offset(fresh27 as isize)
                                as *mut ENetSymbol;
                            (*symbol).value = value;
                            (*symbol).count = ENET_SUBCONTEXT_SYMBOL_DELTA as c_int as enet_uint8;
                            (*symbol).under = ENET_SUBCONTEXT_SYMBOL_DELTA as c_int as enet_uint16;
                            (*symbol).left = 0 as c_int as enet_uint16;
                            (*symbol).right = 0 as c_int as enet_uint16;
                            (*symbol).symbols = 0 as c_int as enet_uint16;
                            (*symbol).escapes = 0 as c_int as enet_uint16;
                            (*symbol).total = 0 as c_int as enet_uint16;
                            (*symbol).parent = 0 as c_int as enet_uint16;
                            (*node_1).right = symbol.offset_from(node_1) as c_long as enet_uint16;
                            break;
                        }
                    } else {
                        count = (count as c_int + (*node_1).count as c_int) as enet_uint16;
                        under = (under as c_int
                            + ((*node_1).under as c_int - (*node_1).count as c_int))
                            as enet_uint16;
                        (*node_1).under = ((*node_1).under as c_int
                            + ENET_SUBCONTEXT_SYMBOL_DELTA as c_int)
                            as enet_uint16;
                        (*node_1).count = ((*node_1).count as c_int
                            + ENET_SUBCONTEXT_SYMBOL_DELTA as c_int)
                            as enet_uint8;
                        symbol = node_1;
                        break;
                    }
                }
            }
            *parent =
                symbol.offset_from(((*rangeCoder).symbols).as_mut_ptr()) as c_long as enet_uint16;
            parent = &mut (*symbol).parent;
            if count as c_int <= 0 as c_int {
                (*patch).escapes = ((*patch).escapes as c_int
                    + ENET_SUBCONTEXT_ESCAPE_DELTA as c_int)
                    as enet_uint16;
                (*patch).total = ((*patch).total as c_int + ENET_SUBCONTEXT_ESCAPE_DELTA as c_int)
                    as enet_uint16;
            }
            (*patch).total =
                ((*patch).total as c_int + ENET_SUBCONTEXT_SYMBOL_DELTA as c_int) as enet_uint16;
            if count as c_int > 0xff as c_int - 2 as c_int * ENET_SUBCONTEXT_SYMBOL_DELTA as c_int
                || (*patch).total as c_int > ENET_RANGE_CODER_BOTTOM as c_int - 0x100 as c_int
            {
                (*patch).total = (if (*patch).symbols as c_int != 0 {
                    enet_symbol_rescale(patch.offset((*patch).symbols as c_int as isize)) as c_int
                } else {
                    0 as c_int
                }) as enet_uint16;
                (*patch).escapes = ((*patch).escapes as c_int
                    - ((*patch).escapes as c_int >> 1 as c_int))
                    as enet_uint16;
                (*patch).total = ((*patch).total as c_int
                    + ((*patch).escapes as c_int + 256 as c_int * 0 as c_int))
                    as enet_uint16;
            }
            patch = &mut *((*rangeCoder).symbols)
                .as_mut_ptr()
                .offset((*patch).parent as isize) as *mut ENetSymbol;
        }
        *parent = bottom;
        if outData >= outEnd {
            return 0 as c_int as size_t;
        }
        let fresh28 = outData;
        outData = outData.offset(1);
        *fresh28 = value;
        if order >= ENET_SUBCONTEXT_ORDER as c_int as size_t {
            predicted = (*rangeCoder).symbols[predicted as usize].parent;
        } else {
            order = order.wrapping_add(1);
        }
        if nextSymbol
            >= (::core::mem::size_of::<[ENetSymbol; 4096]>() as size_t)
                .wrapping_div(::core::mem::size_of::<ENetSymbol>() as size_t)
                .wrapping_sub(ENET_SUBCONTEXT_ORDER as c_int as size_t)
        {
            nextSymbol = 0 as c_int as size_t;
            let fresh29 = nextSymbol;
            nextSymbol = nextSymbol.wrapping_add(1);
            root = &mut *((*rangeCoder).symbols)
                .as_mut_ptr()
                .offset(fresh29 as isize) as *mut ENetSymbol;
            (*root).value = 0 as c_int as enet_uint8;
            (*root).count = 0 as c_int as enet_uint8;
            (*root).under = 0 as c_int as enet_uint16;
            (*root).left = 0 as c_int as enet_uint16;
            (*root).right = 0 as c_int as enet_uint16;
            (*root).symbols = 0 as c_int as enet_uint16;
            (*root).escapes = 0 as c_int as enet_uint16;
            (*root).total = 0 as c_int as enet_uint16;
            (*root).parent = 0 as c_int as enet_uint16;
            (*root).escapes = ENET_CONTEXT_ESCAPE_MINIMUM as c_int as enet_uint16;
            (*root).total = (ENET_CONTEXT_ESCAPE_MINIMUM as c_int
                + 256 as c_int * ENET_CONTEXT_SYMBOL_MINIMUM as c_int)
                as enet_uint16;
            (*root).symbols = 0 as c_int as enet_uint16;
            predicted = 0 as c_int as enet_uint16;
            order = 0 as c_int as size_t;
        }
    }
    return outData.offset_from(outStart) as c_long as size_t;
}
pub(crate) unsafe fn enet_host_compress_with_range_coder<S: Socket>(
    mut host: *mut ENetHost<S>,
) -> c_int {
    let mut compressor: ENetCompressor = ENetCompressor {
        context: 0 as *mut c_void,
        compress: None,
        decompress: None,
        destroy: None,
    };
    _enet_memset(
        &mut compressor as *mut ENetCompressor as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<ENetCompressor>() as size_t,
    );
    compressor.context = enet_range_coder_create();
    if (compressor.context).is_null() {
        return -(1 as c_int);
    }
    compressor.compress = Some(
        enet_range_coder_compress
            as unsafe extern "C" fn(
                *mut c_void,
                *const ENetBuffer,
                size_t,
                size_t,
                *mut enet_uint8,
                size_t,
            ) -> size_t,
    );
    compressor.decompress = Some(
        enet_range_coder_decompress
            as unsafe extern "C" fn(
                *mut c_void,
                *const enet_uint8,
                size_t,
                *mut enet_uint8,
                size_t,
            ) -> size_t,
    );
    compressor.destroy = Some(enet_range_coder_destroy as unsafe extern "C" fn(*mut c_void) -> ());
    enet_host_compress(host, &mut compressor);
    return 0 as c_int;
}
pub(crate) unsafe fn enet_host_create<S: Socket>(
    mut socket: S,
    mut peerCount: size_t,
    mut channelLimit: size_t,
    mut incomingBandwidth: enet_uint32,
    mut outgoingBandwidth: enet_uint32,
) -> *mut ENetHost<S> {
    let mut host: *mut ENetHost<S> = 0 as *mut ENetHost<S>;
    let mut currentPeer: *mut ENetPeer<S> = 0 as *mut ENetPeer<S>;
    if peerCount > ENET_PROTOCOL_MAXIMUM_PEER_ID as c_int as size_t {
        return 0 as *mut ENetHost<S>;
    }
    host = enet_malloc(::core::mem::size_of::<ENetHost<S>>() as size_t) as *mut ENetHost<S>;
    if host.is_null() {
        return 0 as *mut ENetHost<S>;
    }
    _enet_memset(
        host as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<ENetHost<S>>() as size_t,
    );
    (*host).peers =
        enet_malloc(peerCount.wrapping_mul(::core::mem::size_of::<ENetPeer<S>>() as size_t))
            as *mut ENetPeer<S>;
    if ((*host).peers).is_null() {
        enet_free(host as *mut c_void);
        return 0 as *mut ENetHost<S>;
    }
    _enet_memset(
        (*host).peers as *mut c_void,
        0 as c_int,
        peerCount.wrapping_mul(::core::mem::size_of::<ENetPeer<S>>() as size_t),
    );
    _ = socket.init(SocketOptions {
        receive_buffer: ENET_HOST_RECEIVE_BUFFER_SIZE as usize,
        send_buffer: ENET_HOST_SEND_BUFFER_SIZE as usize,
    });
    (*host).socket.write(socket);
    if channelLimit == 0 || channelLimit > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as c_int as size_t {
        channelLimit = ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as c_int as size_t;
    } else if channelLimit < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as c_int as size_t {
        channelLimit = ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as c_int as size_t;
    }
    (*host).randomSeed = host as size_t as enet_uint32;
    (*host).randomSeed =
        ((*host).randomSeed as c_uint).wrapping_add(enet_time_get()) as enet_uint32 as enet_uint32;
    (*host).randomSeed = (*host).randomSeed << 16 as c_int | (*host).randomSeed >> 16 as c_int;
    (*host).channelLimit = channelLimit;
    (*host).incomingBandwidth = incomingBandwidth;
    (*host).outgoingBandwidth = outgoingBandwidth;
    (*host).bandwidthThrottleEpoch = 0 as c_int as enet_uint32;
    (*host).recalculateBandwidthLimits = 0 as c_int;
    (*host).mtu = ENET_HOST_DEFAULT_MTU as c_int as enet_uint32;
    (*host).peerCount = peerCount;
    (*host).commandCount = 0 as c_int as size_t;
    (*host).bufferCount = 0 as c_int as size_t;
    (*host).checksum = None;
    (*host).receivedAddress.write(None);
    (*host).receivedData = 0 as *mut enet_uint8;
    (*host).receivedDataLength = 0 as c_int as size_t;
    (*host).totalSentData = 0 as c_int as enet_uint32;
    (*host).totalSentPackets = 0 as c_int as enet_uint32;
    (*host).totalReceivedData = 0 as c_int as enet_uint32;
    (*host).totalReceivedPackets = 0 as c_int as enet_uint32;
    (*host).totalQueued = 0 as c_int as enet_uint32;
    (*host).connectedPeers = 0 as c_int as size_t;
    (*host).bandwidthLimitedPeers = 0 as c_int as size_t;
    (*host).duplicatePeers = ENET_PROTOCOL_MAXIMUM_PEER_ID as c_int as size_t;
    (*host).maximumPacketSize = ENET_HOST_DEFAULT_MAXIMUM_PACKET_SIZE as c_int as size_t;
    (*host).maximumWaitingData = ENET_HOST_DEFAULT_MAXIMUM_WAITING_DATA as c_int as size_t;
    (*host).compressor.context = 0 as *mut c_void;
    (*host).compressor.compress = None;
    (*host).compressor.decompress = None;
    (*host).compressor.destroy = None;
    (*host).intercept = None;
    enet_list_clear(&mut (*host).dispatchQueue);
    currentPeer = (*host).peers;
    while currentPeer < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer<S>
    {
        (*currentPeer).host = host;
        (*currentPeer).incomingPeerID =
            currentPeer.offset_from((*host).peers) as c_long as enet_uint16;
        (*currentPeer).incomingSessionID = 0xff as c_int as enet_uint8;
        (*currentPeer).outgoingSessionID = (*currentPeer).incomingSessionID;
        (*currentPeer).address.write(None);
        (*currentPeer).data = 0 as *mut c_void;
        enet_list_clear(&mut (*currentPeer).acknowledgements);
        enet_list_clear(&mut (*currentPeer).sentReliableCommands);
        enet_list_clear(&mut (*currentPeer).outgoingCommands);
        enet_list_clear(&mut (*currentPeer).outgoingSendReliableCommands);
        enet_list_clear(&mut (*currentPeer).dispatchedCommands);
        enet_peer_reset(currentPeer);
        currentPeer = currentPeer.offset(1);
    }
    return host;
}
pub(crate) unsafe fn enet_host_destroy<S: Socket>(mut host: *mut ENetHost<S>) {
    let mut currentPeer: *mut ENetPeer<S> = 0 as *mut ENetPeer<S>;
    if host.is_null() {
        return;
    }
    (*host).socket.assume_init_drop();
    currentPeer = (*host).peers;
    while currentPeer < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer<S>
    {
        enet_peer_reset(currentPeer);
        (*currentPeer).address.assume_init_drop();
        currentPeer = currentPeer.offset(1);
    }
    if !((*host).compressor.context).is_null() && ((*host).compressor.destroy).is_some() {
        (Some(((*host).compressor.destroy).expect("non-null function pointer")))
            .expect("non-null function pointer")((*host).compressor.context);
    }
    (*host).receivedAddress.assume_init_drop();
    enet_free((*host).peers as *mut c_void);
    enet_free(host as *mut c_void);
}
pub(crate) unsafe fn enet_host_random<S: Socket>(mut host: *mut ENetHost<S>) -> enet_uint32 {
    (*host).randomSeed = ((*host).randomSeed as c_uint).wrapping_add(0x6d2b79f5 as c_uint)
        as enet_uint32 as enet_uint32;
    let mut n: enet_uint32 = (*host).randomSeed;
    n = (n ^ n >> 15 as c_int).wrapping_mul(n | 1 as c_uint);
    n ^= n.wrapping_add((n ^ n >> 7 as c_int).wrapping_mul(n | 61 as c_uint));
    return n ^ n >> 14 as c_int;
}
pub(crate) unsafe fn enet_host_connect<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut address: S::PeerAddress,
    mut channelCount: size_t,
    mut data: enet_uint32,
) -> *mut ENetPeer<S> {
    let mut currentPeer: *mut ENetPeer<S> = 0 as *mut ENetPeer<S>;
    let mut channel: *mut ENetChannel = 0 as *mut ENetChannel;
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    if channelCount < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as c_int as size_t {
        channelCount = ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as c_int as size_t;
    } else if channelCount > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as c_int as size_t {
        channelCount = ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as c_int as size_t;
    }
    currentPeer = (*host).peers;
    while currentPeer < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer<S>
    {
        if (*currentPeer).state as c_uint == ENET_PEER_STATE_DISCONNECTED as c_int as c_uint {
            break;
        }
        currentPeer = currentPeer.offset(1);
    }
    if currentPeer >= &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer<S> {
        return 0 as *mut ENetPeer<S>;
    }
    (*currentPeer).channels =
        enet_malloc(channelCount.wrapping_mul(::core::mem::size_of::<ENetChannel>() as size_t))
            as *mut ENetChannel;
    if ((*currentPeer).channels).is_null() {
        return 0 as *mut ENetPeer<S>;
    }
    (*currentPeer).channelCount = channelCount;
    (*currentPeer).state = ENET_PEER_STATE_CONNECTING;
    *(*currentPeer).address.assume_init_mut() = Some(address);
    (*currentPeer).connectID = enet_host_random(host);
    (*currentPeer).mtu = (*host).mtu;
    if (*host).outgoingBandwidth == 0 as c_int as c_uint {
        (*currentPeer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as c_int as enet_uint32;
    } else {
        (*currentPeer).windowSize = ((*host).outgoingBandwidth)
            .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as c_int as c_uint)
            .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as c_int as c_uint);
    }
    if (*currentPeer).windowSize < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as c_int as c_uint {
        (*currentPeer).windowSize = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as c_int as enet_uint32;
    } else if (*currentPeer).windowSize > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as c_int as c_uint {
        (*currentPeer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as c_int as enet_uint32;
    }
    channel = (*currentPeer).channels;
    while channel
        < &mut *((*currentPeer).channels).offset(channelCount as isize) as *mut ENetChannel
    {
        (*channel).outgoingReliableSequenceNumber = 0 as c_int as enet_uint16;
        (*channel).outgoingUnreliableSequenceNumber = 0 as c_int as enet_uint16;
        (*channel).incomingReliableSequenceNumber = 0 as c_int as enet_uint16;
        (*channel).incomingUnreliableSequenceNumber = 0 as c_int as enet_uint16;
        enet_list_clear(&mut (*channel).incomingReliableCommands);
        enet_list_clear(&mut (*channel).incomingUnreliableCommands);
        (*channel).usedReliableWindows = 0 as c_int as enet_uint16;
        _enet_memset(
            ((*channel).reliableWindows).as_mut_ptr() as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<[enet_uint16; 16]>() as size_t,
        );
        channel = channel.offset(1);
    }
    command.header.command = (ENET_PROTOCOL_COMMAND_CONNECT as c_int
        | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as c_int)
        as enet_uint8;
    command.header.channelID = 0xff as c_int as enet_uint8;
    command.connect.outgoingPeerID = htons((*currentPeer).incomingPeerID);
    command.connect.incomingSessionID = (*currentPeer).incomingSessionID;
    command.connect.outgoingSessionID = (*currentPeer).outgoingSessionID;
    command.connect.mtu = htonl((*currentPeer).mtu);
    command.connect.windowSize = htonl((*currentPeer).windowSize);
    command.connect.channelCount = htonl(channelCount as uint32_t);
    command.connect.incomingBandwidth = htonl((*host).incomingBandwidth);
    command.connect.outgoingBandwidth = htonl((*host).outgoingBandwidth);
    command.connect.packetThrottleInterval = htonl((*currentPeer).packetThrottleInterval);
    command.connect.packetThrottleAcceleration = htonl((*currentPeer).packetThrottleAcceleration);
    command.connect.packetThrottleDeceleration = htonl((*currentPeer).packetThrottleDeceleration);
    command.connect.connectID = (*currentPeer).connectID;
    command.connect.data = htonl(data);
    enet_peer_queue_outgoing_command(
        currentPeer,
        &mut command,
        0 as *mut ENetPacket,
        0 as c_int as enet_uint32,
        0 as c_int as enet_uint16,
    );
    return currentPeer;
}
pub(crate) unsafe fn enet_host_broadcast<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut channelID: enet_uint8,
    mut packet: *mut ENetPacket,
) {
    let mut currentPeer: *mut ENetPeer<S> = 0 as *mut ENetPeer<S>;
    currentPeer = (*host).peers;
    while currentPeer < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer<S>
    {
        if !((*currentPeer).state as c_uint != ENET_PEER_STATE_CONNECTED as c_int as c_uint) {
            enet_peer_send(currentPeer, channelID, packet);
        }
        currentPeer = currentPeer.offset(1);
    }
    if (*packet).referenceCount == 0 as c_int as size_t {
        enet_packet_destroy(packet);
    }
}
pub(crate) unsafe fn enet_host_compress<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut compressor: *const ENetCompressor,
) {
    if !((*host).compressor.context).is_null() && ((*host).compressor.destroy).is_some() {
        (Some(((*host).compressor.destroy).expect("non-null function pointer")))
            .expect("non-null function pointer")((*host).compressor.context);
    }
    if !compressor.is_null() {
        (*host).compressor = *compressor;
    } else {
        (*host).compressor.context = 0 as *mut c_void;
    };
}
pub(crate) unsafe fn enet_host_channel_limit<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut channelLimit: size_t,
) {
    if channelLimit == 0 || channelLimit > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as c_int as size_t {
        channelLimit = ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as c_int as size_t;
    } else if channelLimit < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as c_int as size_t {
        channelLimit = ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as c_int as size_t;
    }
    (*host).channelLimit = channelLimit;
}
pub(crate) unsafe fn enet_host_bandwidth_limit<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut incomingBandwidth: enet_uint32,
    mut outgoingBandwidth: enet_uint32,
) {
    (*host).incomingBandwidth = incomingBandwidth;
    (*host).outgoingBandwidth = outgoingBandwidth;
    (*host).recalculateBandwidthLimits = 1 as c_int;
}
pub(crate) unsafe fn enet_host_bandwidth_throttle<S: Socket>(mut host: *mut ENetHost<S>) {
    let mut timeCurrent: enet_uint32 = enet_time_get();
    let mut elapsedTime: enet_uint32 = timeCurrent.wrapping_sub((*host).bandwidthThrottleEpoch);
    let mut peersRemaining: enet_uint32 = (*host).connectedPeers as enet_uint32;
    let mut dataTotal: enet_uint32 = !(0 as c_int) as enet_uint32;
    let mut bandwidth: enet_uint32 = !(0 as c_int) as enet_uint32;
    let mut throttle: enet_uint32 = 0 as c_int as enet_uint32;
    let mut bandwidthLimit: enet_uint32 = 0 as c_int as enet_uint32;
    let mut needsAdjustment: c_int = if (*host).bandwidthLimitedPeers > 0 as c_int as size_t {
        1 as c_int
    } else {
        0 as c_int
    };
    let mut peer: *mut ENetPeer<S> = 0 as *mut ENetPeer<S>;
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    if elapsedTime < ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL as c_int as c_uint {
        return;
    }
    (*host).bandwidthThrottleEpoch = timeCurrent;
    if peersRemaining == 0 as c_int as c_uint {
        return;
    }
    if (*host).outgoingBandwidth != 0 as c_int as c_uint {
        dataTotal = 0 as c_int as enet_uint32;
        bandwidth = ((*host).outgoingBandwidth)
            .wrapping_mul(elapsedTime)
            .wrapping_div(1000 as c_int as c_uint);
        peer = (*host).peers;
        while peer < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer<S> {
            if !((*peer).state as c_uint != ENET_PEER_STATE_CONNECTED as c_int as c_uint
                && (*peer).state as c_uint != ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint)
            {
                dataTotal = (dataTotal as c_uint).wrapping_add((*peer).outgoingDataTotal)
                    as enet_uint32 as enet_uint32;
            }
            peer = peer.offset(1);
        }
    }
    while peersRemaining > 0 as c_int as c_uint && needsAdjustment != 0 as c_int {
        needsAdjustment = 0 as c_int;
        if dataTotal <= bandwidth {
            throttle = ENET_PEER_PACKET_THROTTLE_SCALE as c_int as enet_uint32;
        } else {
            throttle = bandwidth
                .wrapping_mul(ENET_PEER_PACKET_THROTTLE_SCALE as c_int as c_uint)
                .wrapping_div(dataTotal);
        }
        peer = (*host).peers;
        while peer < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer<S> {
            let mut peerBandwidth: enet_uint32 = 0;
            if !((*peer).state as c_uint != ENET_PEER_STATE_CONNECTED as c_int as c_uint
                && (*peer).state as c_uint != ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
                || (*peer).incomingBandwidth == 0 as c_int as c_uint
                || (*peer).outgoingBandwidthThrottleEpoch == timeCurrent)
            {
                peerBandwidth = ((*peer).incomingBandwidth)
                    .wrapping_mul(elapsedTime)
                    .wrapping_div(1000 as c_int as c_uint);
                if !(throttle
                    .wrapping_mul((*peer).outgoingDataTotal)
                    .wrapping_div(ENET_PEER_PACKET_THROTTLE_SCALE as c_int as c_uint)
                    <= peerBandwidth)
                {
                    (*peer).packetThrottleLimit = peerBandwidth
                        .wrapping_mul(ENET_PEER_PACKET_THROTTLE_SCALE as c_int as c_uint)
                        .wrapping_div((*peer).outgoingDataTotal);
                    if (*peer).packetThrottleLimit == 0 as c_int as c_uint {
                        (*peer).packetThrottleLimit = 1 as c_int as enet_uint32;
                    }
                    if (*peer).packetThrottle > (*peer).packetThrottleLimit {
                        (*peer).packetThrottle = (*peer).packetThrottleLimit;
                    }
                    (*peer).outgoingBandwidthThrottleEpoch = timeCurrent;
                    (*peer).incomingDataTotal = 0 as c_int as enet_uint32;
                    (*peer).outgoingDataTotal = 0 as c_int as enet_uint32;
                    needsAdjustment = 1 as c_int;
                    peersRemaining = peersRemaining.wrapping_sub(1);
                    bandwidth = (bandwidth as c_uint).wrapping_sub(peerBandwidth) as enet_uint32
                        as enet_uint32;
                    dataTotal = (dataTotal as c_uint).wrapping_sub(peerBandwidth) as enet_uint32
                        as enet_uint32;
                }
            }
            peer = peer.offset(1);
        }
    }
    if peersRemaining > 0 as c_int as c_uint {
        if dataTotal <= bandwidth {
            throttle = ENET_PEER_PACKET_THROTTLE_SCALE as c_int as enet_uint32;
        } else {
            throttle = bandwidth
                .wrapping_mul(ENET_PEER_PACKET_THROTTLE_SCALE as c_int as c_uint)
                .wrapping_div(dataTotal);
        }
        peer = (*host).peers;
        while peer < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer<S> {
            if !((*peer).state as c_uint != ENET_PEER_STATE_CONNECTED as c_int as c_uint
                && (*peer).state as c_uint != ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
                || (*peer).outgoingBandwidthThrottleEpoch == timeCurrent)
            {
                (*peer).packetThrottleLimit = throttle;
                if (*peer).packetThrottle > (*peer).packetThrottleLimit {
                    (*peer).packetThrottle = (*peer).packetThrottleLimit;
                }
                (*peer).incomingDataTotal = 0 as c_int as enet_uint32;
                (*peer).outgoingDataTotal = 0 as c_int as enet_uint32;
            }
            peer = peer.offset(1);
        }
    }
    if (*host).recalculateBandwidthLimits != 0 {
        (*host).recalculateBandwidthLimits = 0 as c_int;
        peersRemaining = (*host).connectedPeers as enet_uint32;
        bandwidth = (*host).incomingBandwidth;
        needsAdjustment = 1 as c_int;
        if bandwidth == 0 as c_int as c_uint {
            bandwidthLimit = 0 as c_int as enet_uint32;
        } else {
            while peersRemaining > 0 as c_int as c_uint && needsAdjustment != 0 as c_int {
                needsAdjustment = 0 as c_int;
                bandwidthLimit = bandwidth.wrapping_div(peersRemaining);
                peer = (*host).peers;
                while peer
                    < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer<S>
                {
                    if !((*peer).state as c_uint != ENET_PEER_STATE_CONNECTED as c_int as c_uint
                        && (*peer).state as c_uint
                            != ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
                        || (*peer).incomingBandwidthThrottleEpoch == timeCurrent)
                    {
                        if !((*peer).outgoingBandwidth > 0 as c_int as c_uint
                            && (*peer).outgoingBandwidth >= bandwidthLimit)
                        {
                            (*peer).incomingBandwidthThrottleEpoch = timeCurrent;
                            needsAdjustment = 1 as c_int;
                            peersRemaining = peersRemaining.wrapping_sub(1);
                            bandwidth =
                                (bandwidth as c_uint).wrapping_sub((*peer).outgoingBandwidth)
                                    as enet_uint32 as enet_uint32;
                        }
                    }
                    peer = peer.offset(1);
                }
            }
        }
        peer = (*host).peers;
        while peer < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer<S> {
            if !((*peer).state as c_uint != ENET_PEER_STATE_CONNECTED as c_int as c_uint
                && (*peer).state as c_uint != ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint)
            {
                command.header.command = (ENET_PROTOCOL_COMMAND_BANDWIDTH_LIMIT as c_int
                    | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as c_int)
                    as enet_uint8;
                command.header.channelID = 0xff as c_int as enet_uint8;
                command.bandwidthLimit.outgoingBandwidth = htonl((*host).outgoingBandwidth);
                if (*peer).incomingBandwidthThrottleEpoch == timeCurrent {
                    command.bandwidthLimit.incomingBandwidth = htonl((*peer).outgoingBandwidth);
                } else {
                    command.bandwidthLimit.incomingBandwidth = htonl(bandwidthLimit);
                }
                enet_peer_queue_outgoing_command(
                    peer,
                    &mut command,
                    0 as *mut ENetPacket,
                    0 as c_int as enet_uint32,
                    0 as c_int as enet_uint16,
                );
            }
            peer = peer.offset(1);
        }
    }
}
pub(crate) unsafe fn enet_list_clear(mut list: *mut ENetList) {
    (*list).sentinel.next = &mut (*list).sentinel;
    (*list).sentinel.previous = &mut (*list).sentinel;
}
pub(crate) unsafe fn enet_list_insert(
    mut position: ENetListIterator,
    mut data: *mut c_void,
) -> ENetListIterator {
    let mut result: ENetListIterator = data as ENetListIterator;
    (*result).previous = (*position).previous;
    (*result).next = position;
    (*(*result).previous).next = result;
    (*position).previous = result;
    return result;
}
pub(crate) unsafe fn enet_list_remove(mut position: ENetListIterator) -> *mut c_void {
    (*(*position).previous).next = (*position).next;
    (*(*position).next).previous = (*position).previous;
    return position as *mut c_void;
}
pub(crate) unsafe fn enet_list_move(
    mut position: ENetListIterator,
    mut dataFirst: *mut c_void,
    mut dataLast: *mut c_void,
) -> ENetListIterator {
    let mut first: ENetListIterator = dataFirst as ENetListIterator;
    let mut last: ENetListIterator = dataLast as ENetListIterator;
    (*(*first).previous).next = (*last).next;
    (*(*last).next).previous = (*first).previous;
    (*first).previous = (*position).previous;
    (*last).next = position;
    (*(*first).previous).next = first;
    (*position).previous = last;
    return first;
}
pub(crate) unsafe fn enet_list_size(mut list: *mut ENetList) -> size_t {
    let mut size: size_t = 0 as c_int as size_t;
    let mut position: ENetListIterator = 0 as *mut ENetListNode;
    position = (*list).sentinel.next;
    while position != &mut (*list).sentinel as *mut ENetListNode {
        size = size.wrapping_add(1);
        position = (*position).next;
    }
    return size;
}
pub(crate) unsafe fn enet_packet_create(
    mut data: *const c_void,
    mut dataLength: size_t,
    mut flags: enet_uint32,
) -> *mut ENetPacket {
    let mut packet: *mut ENetPacket =
        enet_malloc(::core::mem::size_of::<ENetPacket>() as size_t) as *mut ENetPacket;
    if packet.is_null() {
        return 0 as *mut ENetPacket;
    }
    if flags & ENET_PACKET_FLAG_NO_ALLOCATE as c_int as c_uint != 0 {
        (*packet).data = data as *mut enet_uint8;
    } else if dataLength <= 0 as c_int as size_t {
        (*packet).data = 0 as *mut enet_uint8;
    } else {
        (*packet).data = enet_malloc(dataLength) as *mut enet_uint8;
        if ((*packet).data).is_null() {
            enet_free(packet as *mut c_void);
            return 0 as *mut ENetPacket;
        }
        if !data.is_null() {
            _enet_memcpy((*packet).data as *mut c_void, data, dataLength);
        }
    }
    (*packet).referenceCount = 0 as c_int as size_t;
    (*packet).flags = flags;
    (*packet).dataLength = dataLength;
    (*packet).freeCallback = None;
    (*packet).userData = 0 as *mut c_void;
    return packet;
}
pub(crate) unsafe fn enet_packet_destroy(mut packet: *mut ENetPacket) {
    if packet.is_null() {
        return;
    }
    if ((*packet).freeCallback).is_some() {
        (Some(((*packet).freeCallback).expect("non-null function pointer")))
            .expect("non-null function pointer")(packet);
    }
    if (*packet).flags & ENET_PACKET_FLAG_NO_ALLOCATE as c_int as c_uint == 0
        && !((*packet).data).is_null()
    {
        enet_free((*packet).data as *mut c_void);
    }
    enet_free(packet as *mut c_void);
}
pub(crate) unsafe fn enet_packet_resize(
    mut packet: *mut ENetPacket,
    mut dataLength: size_t,
) -> c_int {
    let mut newData: *mut enet_uint8 = 0 as *mut enet_uint8;
    if dataLength <= (*packet).dataLength
        || (*packet).flags & ENET_PACKET_FLAG_NO_ALLOCATE as c_int as c_uint != 0
    {
        (*packet).dataLength = dataLength;
        return 0 as c_int;
    }
    newData = enet_malloc(dataLength) as *mut enet_uint8;
    if newData.is_null() {
        return -(1 as c_int);
    }
    _enet_memcpy(
        newData as *mut c_void,
        (*packet).data as *const c_void,
        (*packet).dataLength,
    );
    enet_free((*packet).data as *mut c_void);
    (*packet).data = newData;
    (*packet).dataLength = dataLength;
    return 0 as c_int;
}
static mut crcTable: [enet_uint32; 256] = [
    0 as c_int as enet_uint32,
    0x77073096 as c_int as enet_uint32,
    0xee0e612c as c_uint,
    0x990951ba as c_uint,
    0x76dc419 as c_int as enet_uint32,
    0x706af48f as c_int as enet_uint32,
    0xe963a535 as c_uint,
    0x9e6495a3 as c_uint,
    0xedb8832 as c_int as enet_uint32,
    0x79dcb8a4 as c_int as enet_uint32,
    0xe0d5e91e as c_uint,
    0x97d2d988 as c_uint,
    0x9b64c2b as c_int as enet_uint32,
    0x7eb17cbd as c_int as enet_uint32,
    0xe7b82d07 as c_uint,
    0x90bf1d91 as c_uint,
    0x1db71064 as c_int as enet_uint32,
    0x6ab020f2 as c_int as enet_uint32,
    0xf3b97148 as c_uint,
    0x84be41de as c_uint,
    0x1adad47d as c_int as enet_uint32,
    0x6ddde4eb as c_int as enet_uint32,
    0xf4d4b551 as c_uint,
    0x83d385c7 as c_uint,
    0x136c9856 as c_int as enet_uint32,
    0x646ba8c0 as c_int as enet_uint32,
    0xfd62f97a as c_uint,
    0x8a65c9ec as c_uint,
    0x14015c4f as c_int as enet_uint32,
    0x63066cd9 as c_int as enet_uint32,
    0xfa0f3d63 as c_uint,
    0x8d080df5 as c_uint,
    0x3b6e20c8 as c_int as enet_uint32,
    0x4c69105e as c_int as enet_uint32,
    0xd56041e4 as c_uint,
    0xa2677172 as c_uint,
    0x3c03e4d1 as c_int as enet_uint32,
    0x4b04d447 as c_int as enet_uint32,
    0xd20d85fd as c_uint,
    0xa50ab56b as c_uint,
    0x35b5a8fa as c_int as enet_uint32,
    0x42b2986c as c_int as enet_uint32,
    0xdbbbc9d6 as c_uint,
    0xacbcf940 as c_uint,
    0x32d86ce3 as c_int as enet_uint32,
    0x45df5c75 as c_int as enet_uint32,
    0xdcd60dcf as c_uint,
    0xabd13d59 as c_uint,
    0x26d930ac as c_int as enet_uint32,
    0x51de003a as c_int as enet_uint32,
    0xc8d75180 as c_uint,
    0xbfd06116 as c_uint,
    0x21b4f4b5 as c_int as enet_uint32,
    0x56b3c423 as c_int as enet_uint32,
    0xcfba9599 as c_uint,
    0xb8bda50f as c_uint,
    0x2802b89e as c_int as enet_uint32,
    0x5f058808 as c_int as enet_uint32,
    0xc60cd9b2 as c_uint,
    0xb10be924 as c_uint,
    0x2f6f7c87 as c_int as enet_uint32,
    0x58684c11 as c_int as enet_uint32,
    0xc1611dab as c_uint,
    0xb6662d3d as c_uint,
    0x76dc4190 as c_int as enet_uint32,
    0x1db7106 as c_int as enet_uint32,
    0x98d220bc as c_uint,
    0xefd5102a as c_uint,
    0x71b18589 as c_int as enet_uint32,
    0x6b6b51f as c_int as enet_uint32,
    0x9fbfe4a5 as c_uint,
    0xe8b8d433 as c_uint,
    0x7807c9a2 as c_int as enet_uint32,
    0xf00f934 as c_int as enet_uint32,
    0x9609a88e as c_uint,
    0xe10e9818 as c_uint,
    0x7f6a0dbb as c_int as enet_uint32,
    0x86d3d2d as c_int as enet_uint32,
    0x91646c97 as c_uint,
    0xe6635c01 as c_uint,
    0x6b6b51f4 as c_int as enet_uint32,
    0x1c6c6162 as c_int as enet_uint32,
    0x856530d8 as c_uint,
    0xf262004e as c_uint,
    0x6c0695ed as c_int as enet_uint32,
    0x1b01a57b as c_int as enet_uint32,
    0x8208f4c1 as c_uint,
    0xf50fc457 as c_uint,
    0x65b0d9c6 as c_int as enet_uint32,
    0x12b7e950 as c_int as enet_uint32,
    0x8bbeb8ea as c_uint,
    0xfcb9887c as c_uint,
    0x62dd1ddf as c_int as enet_uint32,
    0x15da2d49 as c_int as enet_uint32,
    0x8cd37cf3 as c_uint,
    0xfbd44c65 as c_uint,
    0x4db26158 as c_int as enet_uint32,
    0x3ab551ce as c_int as enet_uint32,
    0xa3bc0074 as c_uint,
    0xd4bb30e2 as c_uint,
    0x4adfa541 as c_int as enet_uint32,
    0x3dd895d7 as c_int as enet_uint32,
    0xa4d1c46d as c_uint,
    0xd3d6f4fb as c_uint,
    0x4369e96a as c_int as enet_uint32,
    0x346ed9fc as c_int as enet_uint32,
    0xad678846 as c_uint,
    0xda60b8d0 as c_uint,
    0x44042d73 as c_int as enet_uint32,
    0x33031de5 as c_int as enet_uint32,
    0xaa0a4c5f as c_uint,
    0xdd0d7cc9 as c_uint,
    0x5005713c as c_int as enet_uint32,
    0x270241aa as c_int as enet_uint32,
    0xbe0b1010 as c_uint,
    0xc90c2086 as c_uint,
    0x5768b525 as c_int as enet_uint32,
    0x206f85b3 as c_int as enet_uint32,
    0xb966d409 as c_uint,
    0xce61e49f as c_uint,
    0x5edef90e as c_int as enet_uint32,
    0x29d9c998 as c_int as enet_uint32,
    0xb0d09822 as c_uint,
    0xc7d7a8b4 as c_uint,
    0x59b33d17 as c_int as enet_uint32,
    0x2eb40d81 as c_int as enet_uint32,
    0xb7bd5c3b as c_uint,
    0xc0ba6cad as c_uint,
    0xedb88320 as c_uint,
    0x9abfb3b6 as c_uint,
    0x3b6e20c as c_int as enet_uint32,
    0x74b1d29a as c_int as enet_uint32,
    0xead54739 as c_uint,
    0x9dd277af as c_uint,
    0x4db2615 as c_int as enet_uint32,
    0x73dc1683 as c_int as enet_uint32,
    0xe3630b12 as c_uint,
    0x94643b84 as c_uint,
    0xd6d6a3e as c_int as enet_uint32,
    0x7a6a5aa8 as c_int as enet_uint32,
    0xe40ecf0b as c_uint,
    0x9309ff9d as c_uint,
    0xa00ae27 as c_int as enet_uint32,
    0x7d079eb1 as c_int as enet_uint32,
    0xf00f9344 as c_uint,
    0x8708a3d2 as c_uint,
    0x1e01f268 as c_int as enet_uint32,
    0x6906c2fe as c_int as enet_uint32,
    0xf762575d as c_uint,
    0x806567cb as c_uint,
    0x196c3671 as c_int as enet_uint32,
    0x6e6b06e7 as c_int as enet_uint32,
    0xfed41b76 as c_uint,
    0x89d32be0 as c_uint,
    0x10da7a5a as c_int as enet_uint32,
    0x67dd4acc as c_int as enet_uint32,
    0xf9b9df6f as c_uint,
    0x8ebeeff9 as c_uint,
    0x17b7be43 as c_int as enet_uint32,
    0x60b08ed5 as c_int as enet_uint32,
    0xd6d6a3e8 as c_uint,
    0xa1d1937e as c_uint,
    0x38d8c2c4 as c_int as enet_uint32,
    0x4fdff252 as c_int as enet_uint32,
    0xd1bb67f1 as c_uint,
    0xa6bc5767 as c_uint,
    0x3fb506dd as c_int as enet_uint32,
    0x48b2364b as c_int as enet_uint32,
    0xd80d2bda as c_uint,
    0xaf0a1b4c as c_uint,
    0x36034af6 as c_int as enet_uint32,
    0x41047a60 as c_int as enet_uint32,
    0xdf60efc3 as c_uint,
    0xa867df55 as c_uint,
    0x316e8eef as c_int as enet_uint32,
    0x4669be79 as c_int as enet_uint32,
    0xcb61b38c as c_uint,
    0xbc66831a as c_uint,
    0x256fd2a0 as c_int as enet_uint32,
    0x5268e236 as c_int as enet_uint32,
    0xcc0c7795 as c_uint,
    0xbb0b4703 as c_uint,
    0x220216b9 as c_int as enet_uint32,
    0x5505262f as c_int as enet_uint32,
    0xc5ba3bbe as c_uint,
    0xb2bd0b28 as c_uint,
    0x2bb45a92 as c_int as enet_uint32,
    0x5cb36a04 as c_int as enet_uint32,
    0xc2d7ffa7 as c_uint,
    0xb5d0cf31 as c_uint,
    0x2cd99e8b as c_int as enet_uint32,
    0x5bdeae1d as c_int as enet_uint32,
    0x9b64c2b0 as c_uint,
    0xec63f226 as c_uint,
    0x756aa39c as c_int as enet_uint32,
    0x26d930a as c_int as enet_uint32,
    0x9c0906a9 as c_uint,
    0xeb0e363f as c_uint,
    0x72076785 as c_int as enet_uint32,
    0x5005713 as c_int as enet_uint32,
    0x95bf4a82 as c_uint,
    0xe2b87a14 as c_uint,
    0x7bb12bae as c_int as enet_uint32,
    0xcb61b38 as c_int as enet_uint32,
    0x92d28e9b as c_uint,
    0xe5d5be0d as c_uint,
    0x7cdcefb7 as c_int as enet_uint32,
    0xbdbdf21 as c_int as enet_uint32,
    0x86d3d2d4 as c_uint,
    0xf1d4e242 as c_uint,
    0x68ddb3f8 as c_int as enet_uint32,
    0x1fda836e as c_int as enet_uint32,
    0x81be16cd as c_uint,
    0xf6b9265b as c_uint,
    0x6fb077e1 as c_int as enet_uint32,
    0x18b74777 as c_int as enet_uint32,
    0x88085ae6 as c_uint,
    0xff0f6a70 as c_uint,
    0x66063bca as c_int as enet_uint32,
    0x11010b5c as c_int as enet_uint32,
    0x8f659eff as c_uint,
    0xf862ae69 as c_uint,
    0x616bffd3 as c_int as enet_uint32,
    0x166ccf45 as c_int as enet_uint32,
    0xa00ae278 as c_uint,
    0xd70dd2ee as c_uint,
    0x4e048354 as c_int as enet_uint32,
    0x3903b3c2 as c_int as enet_uint32,
    0xa7672661 as c_uint,
    0xd06016f7 as c_uint,
    0x4969474d as c_int as enet_uint32,
    0x3e6e77db as c_int as enet_uint32,
    0xaed16a4a as c_uint,
    0xd9d65adc as c_uint,
    0x40df0b66 as c_int as enet_uint32,
    0x37d83bf0 as c_int as enet_uint32,
    0xa9bcae53 as c_uint,
    0xdebb9ec5 as c_uint,
    0x47b2cf7f as c_int as enet_uint32,
    0x30b5ffe9 as c_int as enet_uint32,
    0xbdbdf21c as c_uint,
    0xcabac28a as c_uint,
    0x53b39330 as c_int as enet_uint32,
    0x24b4a3a6 as c_int as enet_uint32,
    0xbad03605 as c_uint,
    0xcdd70693 as c_uint,
    0x54de5729 as c_int as enet_uint32,
    0x23d967bf as c_int as enet_uint32,
    0xb3667a2e as c_uint,
    0xc4614ab8 as c_uint,
    0x5d681b02 as c_int as enet_uint32,
    0x2a6f2b94 as c_int as enet_uint32,
    0xb40bbe37 as c_uint,
    0xc30c8ea1 as c_uint,
    0x5a05df1b as c_int as enet_uint32,
    0x2d02ef8d as c_int as enet_uint32,
];
#[no_mangle]
pub(crate) unsafe extern "C" fn enet_crc32(
    mut buffers: *const ENetBuffer,
    mut bufferCount: size_t,
) -> enet_uint32 {
    let mut crc: enet_uint32 = 0xffffffff as c_uint;
    loop {
        let fresh30 = bufferCount;
        bufferCount = bufferCount.wrapping_sub(1);
        if !(fresh30 > 0 as c_int as size_t) {
            break;
        }
        let mut data: *const enet_uint8 = (*buffers).data as *const enet_uint8;
        let mut dataEnd: *const enet_uint8 =
            &*data.offset((*buffers).dataLength as isize) as *const enet_uint8;
        while data < dataEnd {
            let fresh31 = data;
            data = data.offset(1);
            crc = crc >> 8 as c_int
                ^ crcTable[(crc & 0xff as c_int as c_uint ^ *fresh31 as c_uint) as usize];
        }
        buffers = buffers.offset(1);
    }
    return htonl(!crc);
}
pub(crate) unsafe fn enet_peer_throttle_configure<S: Socket>(
    mut peer: *mut ENetPeer<S>,
    mut interval: enet_uint32,
    mut acceleration: enet_uint32,
    mut deceleration: enet_uint32,
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
    command.header.command = (ENET_PROTOCOL_COMMAND_THROTTLE_CONFIGURE as c_int
        | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as c_int)
        as enet_uint8;
    command.header.channelID = 0xff as c_int as enet_uint8;
    command.throttleConfigure.packetThrottleInterval = htonl(interval);
    command.throttleConfigure.packetThrottleAcceleration = htonl(acceleration);
    command.throttleConfigure.packetThrottleDeceleration = htonl(deceleration);
    enet_peer_queue_outgoing_command(
        peer,
        &mut command,
        0 as *mut ENetPacket,
        0 as c_int as enet_uint32,
        0 as c_int as enet_uint16,
    );
}
pub(crate) unsafe fn enet_peer_throttle<S: Socket>(
    mut peer: *mut ENetPeer<S>,
    mut rtt: enet_uint32,
) -> c_int {
    if (*peer).lastRoundTripTime <= (*peer).lastRoundTripTimeVariance {
        (*peer).packetThrottle = (*peer).packetThrottleLimit;
    } else if rtt <= (*peer).lastRoundTripTime {
        (*peer).packetThrottle = ((*peer).packetThrottle as c_uint)
            .wrapping_add((*peer).packetThrottleAcceleration)
            as enet_uint32 as enet_uint32;
        if (*peer).packetThrottle > (*peer).packetThrottleLimit {
            (*peer).packetThrottle = (*peer).packetThrottleLimit;
        }
        return 1 as c_int;
    } else {
        if rtt
            > ((*peer).lastRoundTripTime).wrapping_add(
                (2 as c_int as c_uint).wrapping_mul((*peer).lastRoundTripTimeVariance),
            )
        {
            if (*peer).packetThrottle > (*peer).packetThrottleDeceleration {
                (*peer).packetThrottle = ((*peer).packetThrottle as c_uint)
                    .wrapping_sub((*peer).packetThrottleDeceleration)
                    as enet_uint32 as enet_uint32;
            } else {
                (*peer).packetThrottle = 0 as c_int as enet_uint32;
            }
            return -(1 as c_int);
        }
    }
    return 0 as c_int;
}
pub(crate) unsafe fn enet_peer_send<S: Socket>(
    mut peer: *mut ENetPeer<S>,
    mut channelID: enet_uint8,
    mut packet: *mut ENetPacket,
) -> c_int {
    let mut channel: *mut ENetChannel = 0 as *mut ENetChannel;
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    let mut fragmentLength: size_t = 0;
    if (*peer).state as c_uint != ENET_PEER_STATE_CONNECTED as c_int as c_uint
        || channelID as size_t >= (*peer).channelCount
        || (*packet).dataLength > (*(*peer).host).maximumPacketSize
    {
        return -(1 as c_int);
    }
    channel = &mut *((*peer).channels).offset(channelID as isize) as *mut ENetChannel;
    fragmentLength = ((*peer).mtu as size_t)
        .wrapping_sub(::core::mem::size_of::<ENetProtocolHeader>() as size_t)
        .wrapping_sub(::core::mem::size_of::<ENetProtocolSendFragment>() as size_t);
    if ((*(*peer).host).checksum).is_some() {
        fragmentLength = (fragmentLength as c_ulong)
            .wrapping_sub(::core::mem::size_of::<enet_uint32>() as c_ulong)
            as size_t as size_t;
    }
    if (*packet).dataLength > fragmentLength {
        let mut fragmentCount: enet_uint32 = ((*packet).dataLength)
            .wrapping_add(fragmentLength)
            .wrapping_sub(1 as c_int as size_t)
            .wrapping_div(fragmentLength)
            as enet_uint32;
        let mut fragmentNumber: enet_uint32 = 0;
        let mut fragmentOffset: enet_uint32 = 0;
        let mut commandNumber: enet_uint8 = 0;
        let mut startSequenceNumber: enet_uint16 = 0;
        let mut fragments: ENetList = ENetList {
            sentinel: ENetListNode {
                next: 0 as *mut _ENetListNode,
                previous: 0 as *mut _ENetListNode,
            },
        };
        let mut fragment: *mut ENetOutgoingCommand = 0 as *mut ENetOutgoingCommand;
        if fragmentCount > ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as c_int as c_uint {
            return -(1 as c_int);
        }
        if (*packet).flags
            & (ENET_PACKET_FLAG_RELIABLE as c_int | ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT as c_int)
                as c_uint
            == ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT as c_int as c_uint
            && ((*channel).outgoingUnreliableSequenceNumber as c_int) < 0xffff as c_int
        {
            commandNumber = ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT as c_int as enet_uint8;
            startSequenceNumber = htons(
                ((*channel).outgoingUnreliableSequenceNumber as c_int + 1 as c_int) as uint16_t,
            );
        } else {
            commandNumber = (ENET_PROTOCOL_COMMAND_SEND_FRAGMENT as c_int
                | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as c_int)
                as enet_uint8;
            startSequenceNumber = htons(
                ((*channel).outgoingReliableSequenceNumber as c_int + 1 as c_int) as uint16_t,
            );
        }
        enet_list_clear(&mut fragments);
        fragmentNumber = 0 as c_int as enet_uint32;
        fragmentOffset = 0 as c_int as enet_uint32;
        while (fragmentOffset as size_t) < (*packet).dataLength {
            if ((*packet).dataLength).wrapping_sub(fragmentOffset as size_t) < fragmentLength {
                fragmentLength = ((*packet).dataLength).wrapping_sub(fragmentOffset as size_t);
            }
            fragment = enet_malloc(::core::mem::size_of::<ENetOutgoingCommand>() as size_t)
                as *mut ENetOutgoingCommand;
            if fragment.is_null() {
                while !(fragments.sentinel.next == &mut fragments.sentinel as *mut ENetListNode) {
                    fragment =
                        enet_list_remove(fragments.sentinel.next) as *mut ENetOutgoingCommand;
                    enet_free(fragment as *mut c_void);
                }
                return -(1 as c_int);
            }
            (*fragment).fragmentOffset = fragmentOffset;
            (*fragment).fragmentLength = fragmentLength as enet_uint16;
            (*fragment).packet = packet;
            (*fragment).command.header.command = commandNumber;
            (*fragment).command.header.channelID = channelID;
            (*fragment).command.sendFragment.startSequenceNumber = startSequenceNumber;
            (*fragment).command.sendFragment.dataLength = htons(fragmentLength as uint16_t);
            (*fragment).command.sendFragment.fragmentCount = htonl(fragmentCount);
            (*fragment).command.sendFragment.fragmentNumber = htonl(fragmentNumber);
            (*fragment).command.sendFragment.totalLength = htonl((*packet).dataLength as uint32_t);
            (*fragment).command.sendFragment.fragmentOffset = ntohl(fragmentOffset);
            enet_list_insert(&mut fragments.sentinel, fragment as *mut c_void);
            fragmentNumber = fragmentNumber.wrapping_add(1);
            fragmentOffset = (fragmentOffset as size_t).wrapping_add(fragmentLength) as enet_uint32
                as enet_uint32;
        }
        (*packet).referenceCount = ((*packet).referenceCount as c_ulong)
            .wrapping_add(fragmentNumber as c_ulong) as size_t
            as size_t;
        while !(fragments.sentinel.next == &mut fragments.sentinel as *mut ENetListNode) {
            fragment = enet_list_remove(fragments.sentinel.next) as *mut ENetOutgoingCommand;
            enet_peer_setup_outgoing_command(peer, fragment);
        }
        return 0 as c_int;
    }
    command.header.channelID = channelID;
    if (*packet).flags
        & (ENET_PACKET_FLAG_RELIABLE as c_int | ENET_PACKET_FLAG_UNSEQUENCED as c_int) as c_uint
        == ENET_PACKET_FLAG_UNSEQUENCED as c_int as c_uint
    {
        command.header.command = (ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as c_int
            | ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED as c_int)
            as enet_uint8;
        command.sendUnsequenced.dataLength = htons((*packet).dataLength as uint16_t);
    } else if (*packet).flags & ENET_PACKET_FLAG_RELIABLE as c_int as c_uint != 0
        || (*channel).outgoingUnreliableSequenceNumber as c_int >= 0xffff as c_int
    {
        command.header.command = (ENET_PROTOCOL_COMMAND_SEND_RELIABLE as c_int
            | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as c_int)
            as enet_uint8;
        command.sendReliable.dataLength = htons((*packet).dataLength as uint16_t);
    } else {
        command.header.command = ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE as c_int as enet_uint8;
        command.sendUnreliable.dataLength = htons((*packet).dataLength as uint16_t);
    }
    if (enet_peer_queue_outgoing_command(
        peer,
        &mut command,
        packet,
        0 as c_int as enet_uint32,
        (*packet).dataLength as enet_uint16,
    ))
    .is_null()
    {
        return -(1 as c_int);
    }
    return 0 as c_int;
}
pub(crate) unsafe fn enet_peer_receive<S: Socket>(
    mut peer: *mut ENetPeer<S>,
    mut channelID: *mut enet_uint8,
) -> *mut ENetPacket {
    let mut incomingCommand: *mut ENetIncomingCommand = 0 as *mut ENetIncomingCommand;
    let mut packet: *mut ENetPacket = 0 as *mut ENetPacket;
    if (*peer).dispatchedCommands.sentinel.next
        == &mut (*peer).dispatchedCommands.sentinel as *mut ENetListNode
    {
        return 0 as *mut ENetPacket;
    }
    incomingCommand =
        enet_list_remove((*peer).dispatchedCommands.sentinel.next) as *mut ENetIncomingCommand;
    if !channelID.is_null() {
        *channelID = (*incomingCommand).command.header.channelID;
    }
    packet = (*incomingCommand).packet;
    (*packet).referenceCount = ((*packet).referenceCount).wrapping_sub(1);
    if !((*incomingCommand).fragments).is_null() {
        enet_free((*incomingCommand).fragments as *mut c_void);
    }
    enet_free(incomingCommand as *mut c_void);
    (*peer).totalWaitingData =
        ((*peer).totalWaitingData as size_t).wrapping_sub((*packet).dataLength) as size_t as size_t;
    return packet;
}
unsafe fn enet_peer_reset_outgoing_commands(mut queue: *mut ENetList) {
    let mut outgoingCommand: *mut ENetOutgoingCommand = 0 as *mut ENetOutgoingCommand;
    while !((*queue).sentinel.next == &mut (*queue).sentinel as *mut ENetListNode) {
        outgoingCommand = enet_list_remove((*queue).sentinel.next) as *mut ENetOutgoingCommand;
        if !((*outgoingCommand).packet).is_null() {
            (*(*outgoingCommand).packet).referenceCount =
                ((*(*outgoingCommand).packet).referenceCount).wrapping_sub(1);
            if (*(*outgoingCommand).packet).referenceCount == 0 as c_int as size_t {
                enet_packet_destroy((*outgoingCommand).packet);
            }
        }
        enet_free(outgoingCommand as *mut c_void);
    }
}
unsafe fn enet_peer_remove_incoming_commands(
    mut _queue: *mut ENetList,
    mut startCommand: ENetListIterator,
    mut endCommand: ENetListIterator,
    mut excludeCommand: *mut ENetIncomingCommand,
) {
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    currentCommand = startCommand;
    while currentCommand != endCommand {
        let mut incomingCommand: *mut ENetIncomingCommand =
            currentCommand as *mut ENetIncomingCommand;
        currentCommand = (*currentCommand).next;
        if incomingCommand == excludeCommand {
            continue;
        }
        enet_list_remove(&mut (*incomingCommand).incomingCommandList);
        if !((*incomingCommand).packet).is_null() {
            (*(*incomingCommand).packet).referenceCount =
                ((*(*incomingCommand).packet).referenceCount).wrapping_sub(1);
            if (*(*incomingCommand).packet).referenceCount == 0 as c_int as size_t {
                enet_packet_destroy((*incomingCommand).packet);
            }
        }
        if !((*incomingCommand).fragments).is_null() {
            enet_free((*incomingCommand).fragments as *mut c_void);
        }
        enet_free(incomingCommand as *mut c_void);
    }
}
unsafe fn enet_peer_reset_incoming_commands(mut queue: *mut ENetList) {
    enet_peer_remove_incoming_commands(
        queue,
        (*queue).sentinel.next,
        &mut (*queue).sentinel,
        0 as *mut ENetIncomingCommand,
    );
}
pub(crate) unsafe fn enet_peer_reset_queues<S: Socket>(mut peer: *mut ENetPeer<S>) {
    let mut channel: *mut ENetChannel = 0 as *mut ENetChannel;
    if (*peer).flags as c_int & ENET_PEER_FLAG_NEEDS_DISPATCH as c_int != 0 {
        enet_list_remove(&mut (*peer).dispatchList);
        (*peer).flags =
            ((*peer).flags as c_int & !(ENET_PEER_FLAG_NEEDS_DISPATCH as c_int)) as enet_uint16;
    }
    while !((*peer).acknowledgements.sentinel.next
        == &mut (*peer).acknowledgements.sentinel as *mut ENetListNode)
    {
        enet_free(enet_list_remove((*peer).acknowledgements.sentinel.next));
    }
    enet_peer_reset_outgoing_commands(&mut (*peer).sentReliableCommands);
    enet_peer_reset_outgoing_commands(&mut (*peer).outgoingCommands);
    enet_peer_reset_outgoing_commands(&mut (*peer).outgoingSendReliableCommands);
    enet_peer_reset_incoming_commands(&mut (*peer).dispatchedCommands);
    if !((*peer).channels).is_null() && (*peer).channelCount > 0 as c_int as size_t {
        channel = (*peer).channels;
        while channel
            < &mut *((*peer).channels).offset((*peer).channelCount as isize) as *mut ENetChannel
        {
            enet_peer_reset_incoming_commands(&mut (*channel).incomingReliableCommands);
            enet_peer_reset_incoming_commands(&mut (*channel).incomingUnreliableCommands);
            channel = channel.offset(1);
        }
        enet_free((*peer).channels as *mut c_void);
    }
    (*peer).channels = 0 as *mut ENetChannel;
    (*peer).channelCount = 0 as c_int as size_t;
}
pub(crate) unsafe fn enet_peer_on_connect<S: Socket>(mut peer: *mut ENetPeer<S>) {
    if (*peer).state as c_uint != ENET_PEER_STATE_CONNECTED as c_int as c_uint
        && (*peer).state as c_uint != ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
    {
        if (*peer).incomingBandwidth != 0 as c_int as c_uint {
            (*(*peer).host).bandwidthLimitedPeers =
                ((*(*peer).host).bandwidthLimitedPeers).wrapping_add(1);
        }
        (*(*peer).host).connectedPeers = ((*(*peer).host).connectedPeers).wrapping_add(1);
    }
}
pub(crate) unsafe fn enet_peer_on_disconnect<S: Socket>(mut peer: *mut ENetPeer<S>) {
    if (*peer).state as c_uint == ENET_PEER_STATE_CONNECTED as c_int as c_uint
        || (*peer).state as c_uint == ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
    {
        if (*peer).incomingBandwidth != 0 as c_int as c_uint {
            (*(*peer).host).bandwidthLimitedPeers =
                ((*(*peer).host).bandwidthLimitedPeers).wrapping_sub(1);
        }
        (*(*peer).host).connectedPeers = ((*(*peer).host).connectedPeers).wrapping_sub(1);
    }
}
pub(crate) unsafe fn enet_peer_reset<S: Socket>(mut peer: *mut ENetPeer<S>) {
    enet_peer_on_disconnect(peer);
    (*peer).outgoingPeerID = ENET_PROTOCOL_MAXIMUM_PEER_ID as c_int as enet_uint16;
    (*peer).connectID = 0 as c_int as enet_uint32;
    (*peer).state = ENET_PEER_STATE_DISCONNECTED;
    (*peer).incomingBandwidth = 0 as c_int as enet_uint32;
    (*peer).outgoingBandwidth = 0 as c_int as enet_uint32;
    (*peer).incomingBandwidthThrottleEpoch = 0 as c_int as enet_uint32;
    (*peer).outgoingBandwidthThrottleEpoch = 0 as c_int as enet_uint32;
    (*peer).incomingDataTotal = 0 as c_int as enet_uint32;
    (*peer).outgoingDataTotal = 0 as c_int as enet_uint32;
    (*peer).lastSendTime = 0 as c_int as enet_uint32;
    (*peer).lastReceiveTime = 0 as c_int as enet_uint32;
    (*peer).nextTimeout = 0 as c_int as enet_uint32;
    (*peer).earliestTimeout = 0 as c_int as enet_uint32;
    (*peer).packetLossEpoch = 0 as c_int as enet_uint32;
    (*peer).packetsSent = 0 as c_int as enet_uint32;
    (*peer).packetsLost = 0 as c_int as enet_uint32;
    (*peer).packetLoss = 0 as c_int as enet_uint32;
    (*peer).packetLossVariance = 0 as c_int as enet_uint32;
    (*peer).packetThrottle = ENET_PEER_DEFAULT_PACKET_THROTTLE as c_int as enet_uint32;
    (*peer).packetThrottleLimit = ENET_PEER_PACKET_THROTTLE_SCALE as c_int as enet_uint32;
    (*peer).packetThrottleCounter = 0 as c_int as enet_uint32;
    (*peer).packetThrottleEpoch = 0 as c_int as enet_uint32;
    (*peer).packetThrottleAcceleration =
        ENET_PEER_PACKET_THROTTLE_ACCELERATION as c_int as enet_uint32;
    (*peer).packetThrottleDeceleration =
        ENET_PEER_PACKET_THROTTLE_DECELERATION as c_int as enet_uint32;
    (*peer).packetThrottleInterval = ENET_PEER_PACKET_THROTTLE_INTERVAL as c_int as enet_uint32;
    (*peer).pingInterval = ENET_PEER_PING_INTERVAL as c_int as enet_uint32;
    (*peer).timeoutLimit = ENET_PEER_TIMEOUT_LIMIT as c_int as enet_uint32;
    (*peer).timeoutMinimum = ENET_PEER_TIMEOUT_MINIMUM as c_int as enet_uint32;
    (*peer).timeoutMaximum = ENET_PEER_TIMEOUT_MAXIMUM as c_int as enet_uint32;
    (*peer).lastRoundTripTime = ENET_PEER_DEFAULT_ROUND_TRIP_TIME as c_int as enet_uint32;
    (*peer).lowestRoundTripTime = ENET_PEER_DEFAULT_ROUND_TRIP_TIME as c_int as enet_uint32;
    (*peer).lastRoundTripTimeVariance = 0 as c_int as enet_uint32;
    (*peer).highestRoundTripTimeVariance = 0 as c_int as enet_uint32;
    (*peer).roundTripTime = ENET_PEER_DEFAULT_ROUND_TRIP_TIME as c_int as enet_uint32;
    (*peer).roundTripTimeVariance = 0 as c_int as enet_uint32;
    (*peer).mtu = (*(*peer).host).mtu;
    (*peer).reliableDataInTransit = 0 as c_int as enet_uint32;
    (*peer).outgoingReliableSequenceNumber = 0 as c_int as enet_uint16;
    (*peer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as c_int as enet_uint32;
    (*peer).incomingUnsequencedGroup = 0 as c_int as enet_uint16;
    (*peer).outgoingUnsequencedGroup = 0 as c_int as enet_uint16;
    (*peer).eventData = 0 as c_int as enet_uint32;
    (*peer).totalWaitingData = 0 as c_int as size_t;
    (*peer).flags = 0 as c_int as enet_uint16;
    _enet_memset(
        ((*peer).unsequencedWindow).as_mut_ptr() as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<[enet_uint32; 32]>() as size_t,
    );
    enet_peer_reset_queues(peer);
}
pub(crate) unsafe fn enet_peer_ping<S: Socket>(mut peer: *mut ENetPeer<S>) {
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    if (*peer).state as c_uint != ENET_PEER_STATE_CONNECTED as c_int as c_uint {
        return;
    }
    command.header.command = (ENET_PROTOCOL_COMMAND_PING as c_int
        | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as c_int)
        as enet_uint8;
    command.header.channelID = 0xff as c_int as enet_uint8;
    enet_peer_queue_outgoing_command(
        peer,
        &mut command,
        0 as *mut ENetPacket,
        0 as c_int as enet_uint32,
        0 as c_int as enet_uint16,
    );
}
pub(crate) unsafe fn enet_peer_ping_interval<S: Socket>(
    mut peer: *mut ENetPeer<S>,
    mut pingInterval: enet_uint32,
) {
    (*peer).pingInterval = if pingInterval != 0 {
        pingInterval
    } else {
        ENET_PEER_PING_INTERVAL as c_int as c_uint
    };
}
pub(crate) unsafe fn enet_peer_timeout<S: Socket>(
    mut peer: *mut ENetPeer<S>,
    mut timeoutLimit: enet_uint32,
    mut timeoutMinimum: enet_uint32,
    mut timeoutMaximum: enet_uint32,
) {
    (*peer).timeoutLimit = if timeoutLimit != 0 {
        timeoutLimit
    } else {
        ENET_PEER_TIMEOUT_LIMIT as c_int as c_uint
    };
    (*peer).timeoutMinimum = if timeoutMinimum != 0 {
        timeoutMinimum
    } else {
        ENET_PEER_TIMEOUT_MINIMUM as c_int as c_uint
    };
    (*peer).timeoutMaximum = if timeoutMaximum != 0 {
        timeoutMaximum
    } else {
        ENET_PEER_TIMEOUT_MAXIMUM as c_int as c_uint
    };
}
pub(crate) unsafe fn enet_peer_disconnect_now<S: Socket>(
    mut peer: *mut ENetPeer<S>,
    mut data: enet_uint32,
) {
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    if (*peer).state as c_uint == ENET_PEER_STATE_DISCONNECTED as c_int as c_uint {
        return;
    }
    if (*peer).state as c_uint != ENET_PEER_STATE_ZOMBIE as c_int as c_uint
        && (*peer).state as c_uint != ENET_PEER_STATE_DISCONNECTING as c_int as c_uint
    {
        enet_peer_reset_queues(peer);
        command.header.command = (ENET_PROTOCOL_COMMAND_DISCONNECT as c_int
            | ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED as c_int)
            as enet_uint8;
        command.header.channelID = 0xff as c_int as enet_uint8;
        command.disconnect.data = htonl(data);
        enet_peer_queue_outgoing_command(
            peer,
            &mut command,
            0 as *mut ENetPacket,
            0 as c_int as enet_uint32,
            0 as c_int as enet_uint16,
        );
        enet_host_flush((*peer).host);
    }
    enet_peer_reset(peer);
}
pub(crate) unsafe fn enet_peer_disconnect<S: Socket>(
    mut peer: *mut ENetPeer<S>,
    mut data: enet_uint32,
) {
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    if (*peer).state as c_uint == ENET_PEER_STATE_DISCONNECTING as c_int as c_uint
        || (*peer).state as c_uint == ENET_PEER_STATE_DISCONNECTED as c_int as c_uint
        || (*peer).state as c_uint == ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT as c_int as c_uint
        || (*peer).state as c_uint == ENET_PEER_STATE_ZOMBIE as c_int as c_uint
    {
        return;
    }
    enet_peer_reset_queues(peer);
    command.header.command = ENET_PROTOCOL_COMMAND_DISCONNECT as c_int as enet_uint8;
    command.header.channelID = 0xff as c_int as enet_uint8;
    command.disconnect.data = htonl(data);
    if (*peer).state as c_uint == ENET_PEER_STATE_CONNECTED as c_int as c_uint
        || (*peer).state as c_uint == ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
    {
        command.header.command = (command.header.command as c_int
            | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as c_int)
            as enet_uint8;
    } else {
        command.header.command = (command.header.command as c_int
            | ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED as c_int)
            as enet_uint8;
    }
    enet_peer_queue_outgoing_command(
        peer,
        &mut command,
        0 as *mut ENetPacket,
        0 as c_int as enet_uint32,
        0 as c_int as enet_uint16,
    );
    if (*peer).state as c_uint == ENET_PEER_STATE_CONNECTED as c_int as c_uint
        || (*peer).state as c_uint == ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
    {
        enet_peer_on_disconnect(peer);
        (*peer).state = ENET_PEER_STATE_DISCONNECTING;
    } else {
        enet_host_flush((*peer).host);
        enet_peer_reset(peer);
    };
}
pub(crate) unsafe fn enet_peer_has_outgoing_commands<S: Socket>(
    mut peer: *mut ENetPeer<S>,
) -> c_int {
    if (*peer).outgoingCommands.sentinel.next
        == &mut (*peer).outgoingCommands.sentinel as *mut ENetListNode
        && (*peer).outgoingSendReliableCommands.sentinel.next
            == &mut (*peer).outgoingSendReliableCommands.sentinel as *mut ENetListNode
        && (*peer).sentReliableCommands.sentinel.next
            == &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode
    {
        return 0 as c_int;
    }
    return 1 as c_int;
}
pub(crate) unsafe fn enet_peer_disconnect_later<S: Socket>(
    mut peer: *mut ENetPeer<S>,
    mut data: enet_uint32,
) {
    if ((*peer).state as c_uint == ENET_PEER_STATE_CONNECTED as c_int as c_uint
        || (*peer).state as c_uint == ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint)
        && enet_peer_has_outgoing_commands(peer) != 0
    {
        (*peer).state = ENET_PEER_STATE_DISCONNECT_LATER;
        (*peer).eventData = data;
    } else {
        enet_peer_disconnect(peer, data);
    };
}
pub(crate) unsafe fn enet_peer_queue_acknowledgement<S: Socket>(
    mut peer: *mut ENetPeer<S>,
    mut command: *const ENetProtocol,
    mut sentTime: enet_uint16,
) -> *mut ENetAcknowledgement {
    let mut acknowledgement: *mut ENetAcknowledgement = 0 as *mut ENetAcknowledgement;
    if ((*command).header.channelID as size_t) < (*peer).channelCount {
        let mut channel: *mut ENetChannel = &mut *((*peer).channels)
            .offset((*command).header.channelID as isize)
            as *mut ENetChannel;
        let mut reliableWindow: enet_uint16 = ((*command).header.reliableSequenceNumber as c_int
            / ENET_PEER_RELIABLE_WINDOW_SIZE as c_int)
            as enet_uint16;
        let mut currentWindow: enet_uint16 = ((*channel).incomingReliableSequenceNumber as c_int
            / ENET_PEER_RELIABLE_WINDOW_SIZE as c_int)
            as enet_uint16;
        if ((*command).header.reliableSequenceNumber as c_int)
            < (*channel).incomingReliableSequenceNumber as c_int
        {
            reliableWindow =
                (reliableWindow as c_int + ENET_PEER_RELIABLE_WINDOWS as c_int) as enet_uint16;
        }
        if reliableWindow as c_int
            >= currentWindow as c_int + ENET_PEER_FREE_RELIABLE_WINDOWS as c_int - 1 as c_int
            && reliableWindow as c_int
                <= currentWindow as c_int + ENET_PEER_FREE_RELIABLE_WINDOWS as c_int
        {
            return 0 as *mut ENetAcknowledgement;
        }
    }
    acknowledgement = enet_malloc(::core::mem::size_of::<ENetAcknowledgement>() as size_t)
        as *mut ENetAcknowledgement;
    if acknowledgement.is_null() {
        return 0 as *mut ENetAcknowledgement;
    }
    (*peer).outgoingDataTotal = ((*peer).outgoingDataTotal as c_ulong)
        .wrapping_add(::core::mem::size_of::<ENetProtocolAcknowledge>() as c_ulong)
        as enet_uint32 as enet_uint32;
    (*acknowledgement).sentTime = sentTime as enet_uint32;
    (*acknowledgement).command = *command;
    enet_list_insert(
        &mut (*peer).acknowledgements.sentinel,
        acknowledgement as *mut c_void,
    );
    return acknowledgement;
}
pub(crate) unsafe fn enet_peer_setup_outgoing_command<S: Socket>(
    mut peer: *mut ENetPeer<S>,
    mut outgoingCommand: *mut ENetOutgoingCommand,
) {
    (*peer).outgoingDataTotal = ((*peer).outgoingDataTotal as size_t).wrapping_add(
        (enet_protocol_command_size((*outgoingCommand).command.header.command))
            .wrapping_add((*outgoingCommand).fragmentLength as size_t),
    ) as enet_uint32 as enet_uint32;
    if (*outgoingCommand).command.header.channelID as c_int == 0xff as c_int {
        (*peer).outgoingReliableSequenceNumber =
            ((*peer).outgoingReliableSequenceNumber).wrapping_add(1);
        (*outgoingCommand).reliableSequenceNumber = (*peer).outgoingReliableSequenceNumber;
        (*outgoingCommand).unreliableSequenceNumber = 0 as c_int as enet_uint16;
    } else {
        let mut channel: *mut ENetChannel = &mut *((*peer).channels)
            .offset((*outgoingCommand).command.header.channelID as isize)
            as *mut ENetChannel;
        if (*outgoingCommand).command.header.command as c_int
            & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as c_int
            != 0
        {
            (*channel).outgoingReliableSequenceNumber =
                ((*channel).outgoingReliableSequenceNumber).wrapping_add(1);
            (*channel).outgoingUnreliableSequenceNumber = 0 as c_int as enet_uint16;
            (*outgoingCommand).reliableSequenceNumber = (*channel).outgoingReliableSequenceNumber;
            (*outgoingCommand).unreliableSequenceNumber = 0 as c_int as enet_uint16;
        } else if (*outgoingCommand).command.header.command as c_int
            & ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED as c_int
            != 0
        {
            (*peer).outgoingUnsequencedGroup = ((*peer).outgoingUnsequencedGroup).wrapping_add(1);
            (*outgoingCommand).reliableSequenceNumber = 0 as c_int as enet_uint16;
            (*outgoingCommand).unreliableSequenceNumber = 0 as c_int as enet_uint16;
        } else {
            if (*outgoingCommand).fragmentOffset == 0 as c_int as c_uint {
                (*channel).outgoingUnreliableSequenceNumber =
                    ((*channel).outgoingUnreliableSequenceNumber).wrapping_add(1);
            }
            (*outgoingCommand).reliableSequenceNumber = (*channel).outgoingReliableSequenceNumber;
            (*outgoingCommand).unreliableSequenceNumber =
                (*channel).outgoingUnreliableSequenceNumber;
        }
    }
    (*outgoingCommand).sendAttempts = 0 as c_int as enet_uint16;
    (*outgoingCommand).sentTime = 0 as c_int as enet_uint32;
    (*outgoingCommand).roundTripTimeout = 0 as c_int as enet_uint32;
    (*outgoingCommand).command.header.reliableSequenceNumber =
        htons((*outgoingCommand).reliableSequenceNumber);
    (*(*peer).host).totalQueued = ((*(*peer).host).totalQueued).wrapping_add(1);
    (*outgoingCommand).queueTime = (*(*peer).host).totalQueued;
    match (*outgoingCommand).command.header.command as c_int & ENET_PROTOCOL_COMMAND_MASK as c_int {
        7 => {
            (*outgoingCommand)
                .command
                .sendUnreliable
                .unreliableSequenceNumber = htons((*outgoingCommand).unreliableSequenceNumber);
        }
        9 => {
            (*outgoingCommand).command.sendUnsequenced.unsequencedGroup =
                htons((*peer).outgoingUnsequencedGroup);
        }
        _ => {}
    }
    if (*outgoingCommand).command.header.command as c_int
        & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as c_int
        != 0 as c_int
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
    mut peer: *mut ENetPeer<S>,
    mut command: *const ENetProtocol,
    mut packet: *mut ENetPacket,
    mut offset: enet_uint32,
    mut length: enet_uint16,
) -> *mut ENetOutgoingCommand {
    let mut outgoingCommand: *mut ENetOutgoingCommand =
        enet_malloc(::core::mem::size_of::<ENetOutgoingCommand>() as size_t)
            as *mut ENetOutgoingCommand;
    if outgoingCommand.is_null() {
        return 0 as *mut ENetOutgoingCommand;
    }
    (*outgoingCommand).command = *command;
    (*outgoingCommand).fragmentOffset = offset;
    (*outgoingCommand).fragmentLength = length;
    (*outgoingCommand).packet = packet;
    if !packet.is_null() {
        (*packet).referenceCount = ((*packet).referenceCount).wrapping_add(1);
    }
    enet_peer_setup_outgoing_command(peer, outgoingCommand);
    return outgoingCommand;
}
pub(crate) unsafe fn enet_peer_dispatch_incoming_unreliable_commands<S: Socket>(
    mut peer: *mut ENetPeer<S>,
    mut channel: *mut ENetChannel,
    mut queuedCommand: *mut ENetIncomingCommand,
) {
    let mut droppedCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut startCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut current_block_22: u64;
    currentCommand = (*channel).incomingUnreliableCommands.sentinel.next;
    startCommand = currentCommand;
    droppedCommand = startCommand;
    while currentCommand != &mut (*channel).incomingUnreliableCommands.sentinel as *mut ENetListNode
    {
        let mut incomingCommand: *mut ENetIncomingCommand =
            currentCommand as *mut ENetIncomingCommand;
        if !((*incomingCommand).command.header.command as c_int
            & ENET_PROTOCOL_COMMAND_MASK as c_int
            == ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as c_int)
        {
            if (*incomingCommand).reliableSequenceNumber as c_int
                == (*channel).incomingReliableSequenceNumber as c_int
            {
                if (*incomingCommand).fragmentsRemaining <= 0 as c_int as c_uint {
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
                        if (*peer).flags as c_int & ENET_PEER_FLAG_NEEDS_DISPATCH as c_int == 0 {
                            enet_list_insert(
                                &mut (*(*peer).host).dispatchQueue.sentinel,
                                &mut (*peer).dispatchList as *mut ENetListNode as *mut c_void,
                            );
                            (*peer).flags = ((*peer).flags as c_int
                                | ENET_PEER_FLAG_NEEDS_DISPATCH as c_int)
                                as enet_uint16;
                        }
                        droppedCommand = currentCommand;
                    } else if droppedCommand != currentCommand {
                        droppedCommand = (*currentCommand).previous;
                    }
                    current_block_22 = 13472856163611868459;
                }
            } else {
                let mut reliableWindow: enet_uint16 = ((*incomingCommand).reliableSequenceNumber
                    as c_int
                    / ENET_PEER_RELIABLE_WINDOW_SIZE as c_int)
                    as enet_uint16;
                let mut currentWindow: enet_uint16 = ((*channel).incomingReliableSequenceNumber
                    as c_int
                    / ENET_PEER_RELIABLE_WINDOW_SIZE as c_int)
                    as enet_uint16;
                if ((*incomingCommand).reliableSequenceNumber as c_int)
                    < (*channel).incomingReliableSequenceNumber as c_int
                {
                    reliableWindow = (reliableWindow as c_int + ENET_PEER_RELIABLE_WINDOWS as c_int)
                        as enet_uint16;
                }
                if reliableWindow as c_int >= currentWindow as c_int
                    && (reliableWindow as c_int)
                        < currentWindow as c_int + ENET_PEER_FREE_RELIABLE_WINDOWS as c_int
                            - 1 as c_int
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
                    if (*peer).flags as c_int & ENET_PEER_FLAG_NEEDS_DISPATCH as c_int == 0 {
                        enet_list_insert(
                            &mut (*(*peer).host).dispatchQueue.sentinel,
                            &mut (*peer).dispatchList as *mut ENetListNode as *mut c_void,
                        );
                        (*peer).flags = ((*peer).flags as c_int
                            | ENET_PEER_FLAG_NEEDS_DISPATCH as c_int)
                            as enet_uint16;
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
        if (*peer).flags as c_int & ENET_PEER_FLAG_NEEDS_DISPATCH as c_int == 0 {
            enet_list_insert(
                &mut (*(*peer).host).dispatchQueue.sentinel,
                &mut (*peer).dispatchList as *mut ENetListNode as *mut c_void,
            );
            (*peer).flags =
                ((*peer).flags as c_int | ENET_PEER_FLAG_NEEDS_DISPATCH as c_int) as enet_uint16;
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
    mut peer: *mut ENetPeer<S>,
    mut channel: *mut ENetChannel,
    mut queuedCommand: *mut ENetIncomingCommand,
) {
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    currentCommand = (*channel).incomingReliableCommands.sentinel.next;
    while currentCommand != &mut (*channel).incomingReliableCommands.sentinel as *mut ENetListNode {
        let mut incomingCommand: *mut ENetIncomingCommand =
            currentCommand as *mut ENetIncomingCommand;
        if (*incomingCommand).fragmentsRemaining > 0 as c_int as c_uint
            || (*incomingCommand).reliableSequenceNumber as c_int
                != ((*channel).incomingReliableSequenceNumber as c_int + 1 as c_int) as enet_uint16
                    as c_int
        {
            break;
        }
        (*channel).incomingReliableSequenceNumber = (*incomingCommand).reliableSequenceNumber;
        if (*incomingCommand).fragmentCount > 0 as c_int as c_uint {
            (*channel).incomingReliableSequenceNumber =
                ((*channel).incomingReliableSequenceNumber as c_uint).wrapping_add(
                    ((*incomingCommand).fragmentCount).wrapping_sub(1 as c_int as c_uint),
                ) as enet_uint16 as enet_uint16;
        }
        currentCommand = (*currentCommand).next;
    }
    if currentCommand == (*channel).incomingReliableCommands.sentinel.next {
        return;
    }
    (*channel).incomingUnreliableSequenceNumber = 0 as c_int as enet_uint16;
    enet_list_move(
        &mut (*peer).dispatchedCommands.sentinel,
        (*channel).incomingReliableCommands.sentinel.next as *mut c_void,
        (*currentCommand).previous as *mut c_void,
    );
    if (*peer).flags as c_int & ENET_PEER_FLAG_NEEDS_DISPATCH as c_int == 0 {
        enet_list_insert(
            &mut (*(*peer).host).dispatchQueue.sentinel,
            &mut (*peer).dispatchList as *mut ENetListNode as *mut c_void,
        );
        (*peer).flags =
            ((*peer).flags as c_int | ENET_PEER_FLAG_NEEDS_DISPATCH as c_int) as enet_uint16;
    }
    if !((*channel).incomingUnreliableCommands.sentinel.next
        == &mut (*channel).incomingUnreliableCommands.sentinel as *mut ENetListNode)
    {
        enet_peer_dispatch_incoming_unreliable_commands(peer, channel, queuedCommand);
    }
}
pub(crate) unsafe fn enet_peer_queue_incoming_command<S: Socket>(
    mut peer: *mut ENetPeer<S>,
    mut command: *const ENetProtocol,
    mut data: *const c_void,
    mut dataLength: size_t,
    mut flags: enet_uint32,
    mut fragmentCount: enet_uint32,
) -> *mut ENetIncomingCommand {
    let mut current_block: u64;
    static mut dummyCommand: ENetIncomingCommand = ENetIncomingCommand {
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
        fragments: 0 as *const enet_uint32 as *mut enet_uint32,
        packet: 0 as *const ENetPacket as *mut ENetPacket,
    };
    let mut channel: *mut ENetChannel =
        &mut *((*peer).channels).offset((*command).header.channelID as isize) as *mut ENetChannel;
    let mut unreliableSequenceNumber: enet_uint32 = 0 as c_int as enet_uint32;
    let mut reliableSequenceNumber: enet_uint32 = 0 as c_int as enet_uint32;
    let mut reliableWindow: enet_uint16 = 0;
    let mut currentWindow: enet_uint16 = 0;
    let mut incomingCommand: *mut ENetIncomingCommand = 0 as *mut ENetIncomingCommand;
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut packet: *mut ENetPacket = 0 as *mut ENetPacket;
    if (*peer).state as c_uint == ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint {
        current_block = 9207730764507465628;
    } else {
        if (*command).header.command as c_int & ENET_PROTOCOL_COMMAND_MASK as c_int
            != ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as c_int
        {
            reliableSequenceNumber = (*command).header.reliableSequenceNumber as enet_uint32;
            reliableWindow = reliableSequenceNumber
                .wrapping_div(ENET_PEER_RELIABLE_WINDOW_SIZE as c_int as c_uint)
                as enet_uint16;
            currentWindow = ((*channel).incomingReliableSequenceNumber as c_int
                / ENET_PEER_RELIABLE_WINDOW_SIZE as c_int)
                as enet_uint16;
            if reliableSequenceNumber < (*channel).incomingReliableSequenceNumber as c_uint {
                reliableWindow =
                    (reliableWindow as c_int + ENET_PEER_RELIABLE_WINDOWS as c_int) as enet_uint16;
            }
            if (reliableWindow as c_int) < currentWindow as c_int
                || reliableWindow as c_int
                    >= currentWindow as c_int + ENET_PEER_FREE_RELIABLE_WINDOWS as c_int
                        - 1 as c_int
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
            _ => match (*command).header.command as c_int & ENET_PROTOCOL_COMMAND_MASK as c_int {
                8 | 6 => {
                    current_block = 4379360700607281851;
                    match current_block {
                        10107555224945550073 => {
                            currentCommand = &mut (*channel).incomingUnreliableCommands.sentinel;
                            current_block = 7746103178988627676;
                        }
                        4379360700607281851 => {
                            if reliableSequenceNumber
                                == (*channel).incomingReliableSequenceNumber as c_uint
                            {
                                current_block = 9207730764507465628;
                            } else {
                                currentCommand =
                                    (*channel).incomingReliableCommands.sentinel.previous;
                                loop {
                                    if !(currentCommand
                                        != &mut (*channel).incomingReliableCommands.sentinel
                                            as *mut ENetListNode)
                                    {
                                        current_block = 7746103178988627676;
                                        break;
                                    }
                                    incomingCommand = currentCommand as *mut ENetIncomingCommand;
                                    if reliableSequenceNumber
                                        >= (*channel).incomingReliableSequenceNumber as c_uint
                                    {
                                        if ((*incomingCommand).reliableSequenceNumber as c_int)
                                            < (*channel).incomingReliableSequenceNumber as c_int
                                        {
                                            current_block = 1856101646708284338;
                                        } else {
                                            current_block = 8457315219000651999;
                                        }
                                    } else {
                                        if (*incomingCommand).reliableSequenceNumber as c_int
                                            >= (*channel).incomingReliableSequenceNumber as c_int
                                        {
                                            current_block = 7746103178988627676;
                                            break;
                                        }
                                        current_block = 8457315219000651999;
                                    }
                                    match current_block {
                                        8457315219000651999 => {
                                            if (*incomingCommand).reliableSequenceNumber as c_uint
                                                <= reliableSequenceNumber
                                            {
                                                if ((*incomingCommand).reliableSequenceNumber
                                                    as c_uint)
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
                                        _ => {}
                                    }
                                    currentCommand = (*currentCommand).previous;
                                }
                            }
                        }
                        _ => {
                            unreliableSequenceNumber =
                                ntohs((*command).sendUnreliable.unreliableSequenceNumber)
                                    as enet_uint32;
                            if reliableSequenceNumber
                                == (*channel).incomingReliableSequenceNumber as c_uint
                                && unreliableSequenceNumber
                                    <= (*channel).incomingUnreliableSequenceNumber as c_uint
                            {
                                current_block = 9207730764507465628;
                            } else {
                                currentCommand =
                                    (*channel).incomingUnreliableCommands.sentinel.previous;
                                loop {
                                    if !(currentCommand
                                        != &mut (*channel).incomingUnreliableCommands.sentinel
                                            as *mut ENetListNode)
                                    {
                                        current_block = 7746103178988627676;
                                        break;
                                    }
                                    incomingCommand = currentCommand as *mut ENetIncomingCommand;
                                    if !((*command).header.command as c_int
                                        & ENET_PROTOCOL_COMMAND_MASK as c_int
                                        == ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as c_int)
                                    {
                                        if reliableSequenceNumber
                                            >= (*channel).incomingReliableSequenceNumber as c_uint
                                        {
                                            if ((*incomingCommand).reliableSequenceNumber as c_int)
                                                < (*channel).incomingReliableSequenceNumber as c_int
                                            {
                                                current_block = 17478428563724192186;
                                            } else {
                                                current_block = 11459959175219260272;
                                            }
                                        } else {
                                            if (*incomingCommand).reliableSequenceNumber as c_int
                                                >= (*channel).incomingReliableSequenceNumber
                                                    as c_int
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
                                                    as c_uint)
                                                    < reliableSequenceNumber
                                                {
                                                    current_block = 7746103178988627676;
                                                    break;
                                                }
                                                if !((*incomingCommand).reliableSequenceNumber
                                                    as c_uint
                                                    > reliableSequenceNumber)
                                                {
                                                    if (*incomingCommand).unreliableSequenceNumber
                                                        as c_uint
                                                        <= unreliableSequenceNumber
                                                    {
                                                        if ((*incomingCommand)
                                                            .unreliableSequenceNumber
                                                            as c_uint)
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
                                        enet_malloc(
                                            ::core::mem::size_of::<ENetIncomingCommand>() as size_t
                                        )
                                            as *mut ENetIncomingCommand;
                                    if incomingCommand.is_null() {
                                        current_block = 15492018734234176694;
                                    } else {
                                        (*incomingCommand).reliableSequenceNumber =
                                            (*command).header.reliableSequenceNumber;
                                        (*incomingCommand).unreliableSequenceNumber =
                                            (unreliableSequenceNumber & 0xffff as c_int as c_uint)
                                                as enet_uint16;
                                        (*incomingCommand).command = *command;
                                        (*incomingCommand).fragmentCount = fragmentCount;
                                        (*incomingCommand).fragmentsRemaining = fragmentCount;
                                        (*incomingCommand).packet = packet;
                                        (*incomingCommand).fragments = 0 as *mut enet_uint32;
                                        if fragmentCount > 0 as c_int as c_uint {
                                            if fragmentCount
                                                <= ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as c_int
                                                    as c_uint
                                            {
                                                (*incomingCommand).fragments = enet_malloc(
                                                    (fragmentCount
                                                        .wrapping_add(31 as c_int as c_uint)
                                                        .wrapping_div(32 as c_int as c_uint)
                                                        as size_t)
                                                        .wrapping_mul(::core::mem::size_of::<
                                                            enet_uint32,
                                                        >(
                                                        )
                                                            as size_t),
                                                )
                                                    as *mut enet_uint32;
                                            }
                                            if ((*incomingCommand).fragments).is_null() {
                                                enet_free(incomingCommand as *mut c_void);
                                                current_block = 15492018734234176694;
                                            } else {
                                                _enet_memset(
                                                    (*incomingCommand).fragments as *mut c_void,
                                                    0 as c_int,
                                                    (fragmentCount
                                                        .wrapping_add(31 as c_int as c_uint)
                                                        .wrapping_div(32 as c_int as c_uint)
                                                        as size_t)
                                                        .wrapping_mul(::core::mem::size_of::<
                                                            enet_uint32,
                                                        >(
                                                        )
                                                            as size_t),
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
                                                    (*peer).totalWaitingData =
                                                        ((*peer).totalWaitingData as size_t)
                                                            .wrapping_add((*packet).dataLength)
                                                            as size_t
                                                            as size_t;
                                                }
                                                enet_list_insert(
                                                    (*currentCommand).next,
                                                    incomingCommand as *mut c_void,
                                                );
                                                match (*command).header.command as c_int
                                                    & ENET_PROTOCOL_COMMAND_MASK as c_int
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
                                == (*channel).incomingReliableSequenceNumber as c_uint
                            {
                                current_block = 9207730764507465628;
                            } else {
                                currentCommand =
                                    (*channel).incomingReliableCommands.sentinel.previous;
                                loop {
                                    if !(currentCommand
                                        != &mut (*channel).incomingReliableCommands.sentinel
                                            as *mut ENetListNode)
                                    {
                                        current_block = 7746103178988627676;
                                        break;
                                    }
                                    incomingCommand = currentCommand as *mut ENetIncomingCommand;
                                    if reliableSequenceNumber
                                        >= (*channel).incomingReliableSequenceNumber as c_uint
                                    {
                                        if ((*incomingCommand).reliableSequenceNumber as c_int)
                                            < (*channel).incomingReliableSequenceNumber as c_int
                                        {
                                            current_block = 1856101646708284338;
                                        } else {
                                            current_block = 8457315219000651999;
                                        }
                                    } else {
                                        if (*incomingCommand).reliableSequenceNumber as c_int
                                            >= (*channel).incomingReliableSequenceNumber as c_int
                                        {
                                            current_block = 7746103178988627676;
                                            break;
                                        }
                                        current_block = 8457315219000651999;
                                    }
                                    match current_block {
                                        8457315219000651999 => {
                                            if (*incomingCommand).reliableSequenceNumber as c_uint
                                                <= reliableSequenceNumber
                                            {
                                                if ((*incomingCommand).reliableSequenceNumber
                                                    as c_uint)
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
                                        _ => {}
                                    }
                                    currentCommand = (*currentCommand).previous;
                                }
                            }
                        }
                        _ => {
                            unreliableSequenceNumber =
                                ntohs((*command).sendUnreliable.unreliableSequenceNumber)
                                    as enet_uint32;
                            if reliableSequenceNumber
                                == (*channel).incomingReliableSequenceNumber as c_uint
                                && unreliableSequenceNumber
                                    <= (*channel).incomingUnreliableSequenceNumber as c_uint
                            {
                                current_block = 9207730764507465628;
                            } else {
                                currentCommand =
                                    (*channel).incomingUnreliableCommands.sentinel.previous;
                                loop {
                                    if !(currentCommand
                                        != &mut (*channel).incomingUnreliableCommands.sentinel
                                            as *mut ENetListNode)
                                    {
                                        current_block = 7746103178988627676;
                                        break;
                                    }
                                    incomingCommand = currentCommand as *mut ENetIncomingCommand;
                                    if !((*command).header.command as c_int
                                        & ENET_PROTOCOL_COMMAND_MASK as c_int
                                        == ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as c_int)
                                    {
                                        if reliableSequenceNumber
                                            >= (*channel).incomingReliableSequenceNumber as c_uint
                                        {
                                            if ((*incomingCommand).reliableSequenceNumber as c_int)
                                                < (*channel).incomingReliableSequenceNumber as c_int
                                            {
                                                current_block = 17478428563724192186;
                                            } else {
                                                current_block = 11459959175219260272;
                                            }
                                        } else {
                                            if (*incomingCommand).reliableSequenceNumber as c_int
                                                >= (*channel).incomingReliableSequenceNumber
                                                    as c_int
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
                                                    as c_uint)
                                                    < reliableSequenceNumber
                                                {
                                                    current_block = 7746103178988627676;
                                                    break;
                                                }
                                                if !((*incomingCommand).reliableSequenceNumber
                                                    as c_uint
                                                    > reliableSequenceNumber)
                                                {
                                                    if (*incomingCommand).unreliableSequenceNumber
                                                        as c_uint
                                                        <= unreliableSequenceNumber
                                                    {
                                                        if ((*incomingCommand)
                                                            .unreliableSequenceNumber
                                                            as c_uint)
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
                                        enet_malloc(
                                            ::core::mem::size_of::<ENetIncomingCommand>() as size_t
                                        )
                                            as *mut ENetIncomingCommand;
                                    if incomingCommand.is_null() {
                                        current_block = 15492018734234176694;
                                    } else {
                                        (*incomingCommand).reliableSequenceNumber =
                                            (*command).header.reliableSequenceNumber;
                                        (*incomingCommand).unreliableSequenceNumber =
                                            (unreliableSequenceNumber & 0xffff as c_int as c_uint)
                                                as enet_uint16;
                                        (*incomingCommand).command = *command;
                                        (*incomingCommand).fragmentCount = fragmentCount;
                                        (*incomingCommand).fragmentsRemaining = fragmentCount;
                                        (*incomingCommand).packet = packet;
                                        (*incomingCommand).fragments = 0 as *mut enet_uint32;
                                        if fragmentCount > 0 as c_int as c_uint {
                                            if fragmentCount
                                                <= ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as c_int
                                                    as c_uint
                                            {
                                                (*incomingCommand).fragments = enet_malloc(
                                                    (fragmentCount
                                                        .wrapping_add(31 as c_int as c_uint)
                                                        .wrapping_div(32 as c_int as c_uint)
                                                        as size_t)
                                                        .wrapping_mul(::core::mem::size_of::<
                                                            enet_uint32,
                                                        >(
                                                        )
                                                            as size_t),
                                                )
                                                    as *mut enet_uint32;
                                            }
                                            if ((*incomingCommand).fragments).is_null() {
                                                enet_free(incomingCommand as *mut c_void);
                                                current_block = 15492018734234176694;
                                            } else {
                                                _enet_memset(
                                                    (*incomingCommand).fragments as *mut c_void,
                                                    0 as c_int,
                                                    (fragmentCount
                                                        .wrapping_add(31 as c_int as c_uint)
                                                        .wrapping_div(32 as c_int as c_uint)
                                                        as size_t)
                                                        .wrapping_mul(::core::mem::size_of::<
                                                            enet_uint32,
                                                        >(
                                                        )
                                                            as size_t),
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
                                                    (*peer).totalWaitingData =
                                                        ((*peer).totalWaitingData as size_t)
                                                            .wrapping_add((*packet).dataLength)
                                                            as size_t
                                                            as size_t;
                                                }
                                                enet_list_insert(
                                                    (*currentCommand).next,
                                                    incomingCommand as *mut c_void,
                                                );
                                                match (*command).header.command as c_int
                                                    & ENET_PROTOCOL_COMMAND_MASK as c_int
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
                                == (*channel).incomingReliableSequenceNumber as c_uint
                            {
                                current_block = 9207730764507465628;
                            } else {
                                currentCommand =
                                    (*channel).incomingReliableCommands.sentinel.previous;
                                loop {
                                    if !(currentCommand
                                        != &mut (*channel).incomingReliableCommands.sentinel
                                            as *mut ENetListNode)
                                    {
                                        current_block = 7746103178988627676;
                                        break;
                                    }
                                    incomingCommand = currentCommand as *mut ENetIncomingCommand;
                                    if reliableSequenceNumber
                                        >= (*channel).incomingReliableSequenceNumber as c_uint
                                    {
                                        if ((*incomingCommand).reliableSequenceNumber as c_int)
                                            < (*channel).incomingReliableSequenceNumber as c_int
                                        {
                                            current_block = 1856101646708284338;
                                        } else {
                                            current_block = 8457315219000651999;
                                        }
                                    } else {
                                        if (*incomingCommand).reliableSequenceNumber as c_int
                                            >= (*channel).incomingReliableSequenceNumber as c_int
                                        {
                                            current_block = 7746103178988627676;
                                            break;
                                        }
                                        current_block = 8457315219000651999;
                                    }
                                    match current_block {
                                        8457315219000651999 => {
                                            if (*incomingCommand).reliableSequenceNumber as c_uint
                                                <= reliableSequenceNumber
                                            {
                                                if ((*incomingCommand).reliableSequenceNumber
                                                    as c_uint)
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
                                        _ => {}
                                    }
                                    currentCommand = (*currentCommand).previous;
                                }
                            }
                        }
                        _ => {
                            unreliableSequenceNumber =
                                ntohs((*command).sendUnreliable.unreliableSequenceNumber)
                                    as enet_uint32;
                            if reliableSequenceNumber
                                == (*channel).incomingReliableSequenceNumber as c_uint
                                && unreliableSequenceNumber
                                    <= (*channel).incomingUnreliableSequenceNumber as c_uint
                            {
                                current_block = 9207730764507465628;
                            } else {
                                currentCommand =
                                    (*channel).incomingUnreliableCommands.sentinel.previous;
                                loop {
                                    if !(currentCommand
                                        != &mut (*channel).incomingUnreliableCommands.sentinel
                                            as *mut ENetListNode)
                                    {
                                        current_block = 7746103178988627676;
                                        break;
                                    }
                                    incomingCommand = currentCommand as *mut ENetIncomingCommand;
                                    if !((*command).header.command as c_int
                                        & ENET_PROTOCOL_COMMAND_MASK as c_int
                                        == ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as c_int)
                                    {
                                        if reliableSequenceNumber
                                            >= (*channel).incomingReliableSequenceNumber as c_uint
                                        {
                                            if ((*incomingCommand).reliableSequenceNumber as c_int)
                                                < (*channel).incomingReliableSequenceNumber as c_int
                                            {
                                                current_block = 17478428563724192186;
                                            } else {
                                                current_block = 11459959175219260272;
                                            }
                                        } else {
                                            if (*incomingCommand).reliableSequenceNumber as c_int
                                                >= (*channel).incomingReliableSequenceNumber
                                                    as c_int
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
                                                    as c_uint)
                                                    < reliableSequenceNumber
                                                {
                                                    current_block = 7746103178988627676;
                                                    break;
                                                }
                                                if !((*incomingCommand).reliableSequenceNumber
                                                    as c_uint
                                                    > reliableSequenceNumber)
                                                {
                                                    if (*incomingCommand).unreliableSequenceNumber
                                                        as c_uint
                                                        <= unreliableSequenceNumber
                                                    {
                                                        if ((*incomingCommand)
                                                            .unreliableSequenceNumber
                                                            as c_uint)
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
                                        enet_malloc(
                                            ::core::mem::size_of::<ENetIncomingCommand>() as size_t
                                        )
                                            as *mut ENetIncomingCommand;
                                    if incomingCommand.is_null() {
                                        current_block = 15492018734234176694;
                                    } else {
                                        (*incomingCommand).reliableSequenceNumber =
                                            (*command).header.reliableSequenceNumber;
                                        (*incomingCommand).unreliableSequenceNumber =
                                            (unreliableSequenceNumber & 0xffff as c_int as c_uint)
                                                as enet_uint16;
                                        (*incomingCommand).command = *command;
                                        (*incomingCommand).fragmentCount = fragmentCount;
                                        (*incomingCommand).fragmentsRemaining = fragmentCount;
                                        (*incomingCommand).packet = packet;
                                        (*incomingCommand).fragments = 0 as *mut enet_uint32;
                                        if fragmentCount > 0 as c_int as c_uint {
                                            if fragmentCount
                                                <= ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as c_int
                                                    as c_uint
                                            {
                                                (*incomingCommand).fragments = enet_malloc(
                                                    (fragmentCount
                                                        .wrapping_add(31 as c_int as c_uint)
                                                        .wrapping_div(32 as c_int as c_uint)
                                                        as size_t)
                                                        .wrapping_mul(::core::mem::size_of::<
                                                            enet_uint32,
                                                        >(
                                                        )
                                                            as size_t),
                                                )
                                                    as *mut enet_uint32;
                                            }
                                            if ((*incomingCommand).fragments).is_null() {
                                                enet_free(incomingCommand as *mut c_void);
                                                current_block = 15492018734234176694;
                                            } else {
                                                _enet_memset(
                                                    (*incomingCommand).fragments as *mut c_void,
                                                    0 as c_int,
                                                    (fragmentCount
                                                        .wrapping_add(31 as c_int as c_uint)
                                                        .wrapping_div(32 as c_int as c_uint)
                                                        as size_t)
                                                        .wrapping_mul(::core::mem::size_of::<
                                                            enet_uint32,
                                                        >(
                                                        )
                                                            as size_t),
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
                                                    (*peer).totalWaitingData =
                                                        ((*peer).totalWaitingData as size_t)
                                                            .wrapping_add((*packet).dataLength)
                                                            as size_t
                                                            as size_t;
                                                }
                                                enet_list_insert(
                                                    (*currentCommand).next,
                                                    incomingCommand as *mut c_void,
                                                );
                                                match (*command).header.command as c_int
                                                    & ENET_PROTOCOL_COMMAND_MASK as c_int
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
    match current_block {
        9207730764507465628 => {
            if !(fragmentCount > 0 as c_int as c_uint) {
                if !packet.is_null() && (*packet).referenceCount == 0 as c_int as size_t {
                    enet_packet_destroy(packet);
                }
                return &mut dummyCommand;
            }
        }
        _ => {}
    }
    if !packet.is_null() && (*packet).referenceCount == 0 as c_int as size_t {
        enet_packet_destroy(packet);
    }
    return 0 as *mut ENetIncomingCommand;
}
static mut commandSizes: [size_t; 13] = [
    0 as c_int as size_t,
    ::core::mem::size_of::<ENetProtocolAcknowledge>() as size_t,
    ::core::mem::size_of::<ENetProtocolConnect>() as size_t,
    ::core::mem::size_of::<ENetProtocolVerifyConnect>() as size_t,
    ::core::mem::size_of::<ENetProtocolDisconnect>() as size_t,
    ::core::mem::size_of::<ENetProtocolPing>() as size_t,
    ::core::mem::size_of::<ENetProtocolSendReliable>() as size_t,
    ::core::mem::size_of::<ENetProtocolSendUnreliable>() as size_t,
    ::core::mem::size_of::<ENetProtocolSendFragment>() as size_t,
    ::core::mem::size_of::<ENetProtocolSendUnsequenced>() as size_t,
    ::core::mem::size_of::<ENetProtocolBandwidthLimit>() as size_t,
    ::core::mem::size_of::<ENetProtocolThrottleConfigure>() as size_t,
    ::core::mem::size_of::<ENetProtocolSendFragment>() as size_t,
];
pub(crate) unsafe fn enet_protocol_command_size(mut commandNumber: enet_uint8) -> size_t {
    return commandSizes[(commandNumber as c_int & ENET_PROTOCOL_COMMAND_MASK as c_int) as usize];
}
unsafe fn enet_protocol_change_state<S: Socket>(
    mut _host: *mut ENetHost<S>,
    mut peer: *mut ENetPeer<S>,
    mut state: ENetPeerState,
) {
    if state as c_uint == ENET_PEER_STATE_CONNECTED as c_int as c_uint
        || state as c_uint == ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
    {
        enet_peer_on_connect(peer);
    } else {
        enet_peer_on_disconnect(peer);
    }
    (*peer).state = state;
}
unsafe fn enet_protocol_dispatch_state<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut peer: *mut ENetPeer<S>,
    mut state: ENetPeerState,
) {
    enet_protocol_change_state(host, peer, state);
    if (*peer).flags as c_int & ENET_PEER_FLAG_NEEDS_DISPATCH as c_int == 0 {
        enet_list_insert(
            &mut (*host).dispatchQueue.sentinel,
            &mut (*peer).dispatchList as *mut ENetListNode as *mut c_void,
        );
        (*peer).flags =
            ((*peer).flags as c_int | ENET_PEER_FLAG_NEEDS_DISPATCH as c_int) as enet_uint16;
    }
}
unsafe fn enet_protocol_dispatch_incoming_commands<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut event: *mut ENetEvent<S>,
) -> c_int {
    while !((*host).dispatchQueue.sentinel.next
        == &mut (*host).dispatchQueue.sentinel as *mut ENetListNode)
    {
        let mut peer: *mut ENetPeer<S> =
            enet_list_remove((*host).dispatchQueue.sentinel.next) as *mut ENetPeer<S>;
        (*peer).flags =
            ((*peer).flags as c_int & !(ENET_PEER_FLAG_NEEDS_DISPATCH as c_int)) as enet_uint16;
        match (*peer).state as c_uint {
            3 | 4 => {
                enet_protocol_change_state(host, peer, ENET_PEER_STATE_CONNECTED);
                (*event).type_0 = ENET_EVENT_TYPE_CONNECT;
                (*event).peer = peer;
                (*event).data = (*peer).eventData;
                return 1 as c_int;
            }
            9 => {
                (*host).recalculateBandwidthLimits = 1 as c_int;
                (*event).type_0 = ENET_EVENT_TYPE_DISCONNECT;
                (*event).peer = peer;
                (*event).data = (*peer).eventData;
                enet_peer_reset(peer);
                return 1 as c_int;
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
                if !((*peer).dispatchedCommands.sentinel.next
                    == &mut (*peer).dispatchedCommands.sentinel as *mut ENetListNode)
                {
                    (*peer).flags = ((*peer).flags as c_int
                        | ENET_PEER_FLAG_NEEDS_DISPATCH as c_int)
                        as enet_uint16;
                    enet_list_insert(
                        &mut (*host).dispatchQueue.sentinel,
                        &mut (*peer).dispatchList as *mut ENetListNode as *mut c_void,
                    );
                }
                return 1 as c_int;
            }
            _ => {}
        }
    }
    return 0 as c_int;
}
unsafe fn enet_protocol_notify_connect<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut peer: *mut ENetPeer<S>,
    mut event: *mut ENetEvent<S>,
) {
    (*host).recalculateBandwidthLimits = 1 as c_int;
    if !event.is_null() {
        enet_protocol_change_state(host, peer, ENET_PEER_STATE_CONNECTED);
        (*event).type_0 = ENET_EVENT_TYPE_CONNECT;
        (*event).peer = peer;
        (*event).data = (*peer).eventData;
    } else {
        enet_protocol_dispatch_state(
            host,
            peer,
            (if (*peer).state as c_uint == ENET_PEER_STATE_CONNECTING as c_int as c_uint {
                ENET_PEER_STATE_CONNECTION_SUCCEEDED as c_int
            } else {
                ENET_PEER_STATE_CONNECTION_PENDING as c_int
            }) as ENetPeerState,
        );
    };
}
unsafe fn enet_protocol_notify_disconnect<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut peer: *mut ENetPeer<S>,
    mut event: *mut ENetEvent<S>,
) {
    if (*peer).state as c_uint >= ENET_PEER_STATE_CONNECTION_PENDING as c_int as c_uint {
        (*host).recalculateBandwidthLimits = 1 as c_int;
    }
    if (*peer).state as c_uint != ENET_PEER_STATE_CONNECTING as c_int as c_uint
        && ((*peer).state as c_uint) < ENET_PEER_STATE_CONNECTION_SUCCEEDED as c_int as c_uint
    {
        enet_peer_reset(peer);
    } else if !event.is_null() {
        (*event).type_0 = ENET_EVENT_TYPE_DISCONNECT;
        (*event).peer = peer;
        (*event).data = 0 as c_int as enet_uint32;
        enet_peer_reset(peer);
    } else {
        (*peer).eventData = 0 as c_int as enet_uint32;
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
    };
}
unsafe fn enet_protocol_remove_sent_unreliable_commands<S: Socket>(
    mut peer: *mut ENetPeer<S>,
    mut sentUnreliableCommands: *mut ENetList,
) {
    let mut outgoingCommand: *mut ENetOutgoingCommand = 0 as *mut ENetOutgoingCommand;
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
            if (*(*outgoingCommand).packet).referenceCount == 0 as c_int as size_t {
                (*(*outgoingCommand).packet).flags |= ENET_PACKET_FLAG_SENT as c_int as c_uint;
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
    if (*peer).state as c_uint == ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
        && enet_peer_has_outgoing_commands(peer) == 0
    {
        enet_peer_disconnect(peer, (*peer).eventData);
    }
}
unsafe fn enet_protocol_find_sent_reliable_command(
    mut list: *mut ENetList,
    mut reliableSequenceNumber: enet_uint16,
    mut channelID: enet_uint8,
) -> *mut ENetOutgoingCommand {
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    currentCommand = (*list).sentinel.next;
    while currentCommand != &mut (*list).sentinel as *mut ENetListNode {
        let mut outgoingCommand: *mut ENetOutgoingCommand =
            currentCommand as *mut ENetOutgoingCommand;
        if !((*outgoingCommand).command.header.command as c_int
            & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as c_int
            == 0)
        {
            if ((*outgoingCommand).sendAttempts as c_int) < 1 as c_int {
                break;
            }
            if (*outgoingCommand).reliableSequenceNumber as c_int == reliableSequenceNumber as c_int
                && (*outgoingCommand).command.header.channelID as c_int == channelID as c_int
            {
                return outgoingCommand;
            }
        }
        currentCommand = (*currentCommand).next;
    }
    return 0 as *mut ENetOutgoingCommand;
}
unsafe fn enet_protocol_remove_sent_reliable_command<S: Socket>(
    mut peer: *mut ENetPeer<S>,
    mut reliableSequenceNumber: enet_uint16,
    mut channelID: enet_uint8,
) -> ENetProtocolCommand {
    let mut outgoingCommand: *mut ENetOutgoingCommand = 0 as *mut ENetOutgoingCommand;
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut commandNumber: ENetProtocolCommand = ENET_PROTOCOL_COMMAND_NONE;
    let mut wasSent: c_int = 1 as c_int;
    currentCommand = (*peer).sentReliableCommands.sentinel.next;
    while currentCommand != &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode {
        outgoingCommand = currentCommand as *mut ENetOutgoingCommand;
        if (*outgoingCommand).reliableSequenceNumber as c_int == reliableSequenceNumber as c_int
            && (*outgoingCommand).command.header.channelID as c_int == channelID as c_int
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
        wasSent = 0 as c_int;
    }
    if outgoingCommand.is_null() {
        return ENET_PROTOCOL_COMMAND_NONE;
    }
    if (channelID as size_t) < (*peer).channelCount {
        let mut channel: *mut ENetChannel =
            &mut *((*peer).channels).offset(channelID as isize) as *mut ENetChannel;
        let mut reliableWindow: enet_uint16 = (reliableSequenceNumber as c_int
            / ENET_PEER_RELIABLE_WINDOW_SIZE as c_int)
            as enet_uint16;
        if (*channel).reliableWindows[reliableWindow as usize] as c_int > 0 as c_int {
            (*channel).reliableWindows[reliableWindow as usize] =
                ((*channel).reliableWindows[reliableWindow as usize]).wrapping_sub(1);
            if (*channel).reliableWindows[reliableWindow as usize] == 0 {
                (*channel).usedReliableWindows = ((*channel).usedReliableWindows as c_int
                    & !((1 as c_int) << reliableWindow as c_int))
                    as enet_uint16;
            }
        }
    }
    commandNumber = ((*outgoingCommand).command.header.command as c_int
        & ENET_PROTOCOL_COMMAND_MASK as c_int) as ENetProtocolCommand;
    enet_list_remove(&mut (*outgoingCommand).outgoingCommandList);
    if !((*outgoingCommand).packet).is_null() {
        if wasSent != 0 {
            (*peer).reliableDataInTransit = ((*peer).reliableDataInTransit as c_uint)
                .wrapping_sub((*outgoingCommand).fragmentLength as c_uint)
                as enet_uint32 as enet_uint32;
        }
        (*(*outgoingCommand).packet).referenceCount =
            ((*(*outgoingCommand).packet).referenceCount).wrapping_sub(1);
        if (*(*outgoingCommand).packet).referenceCount == 0 as c_int as size_t {
            (*(*outgoingCommand).packet).flags |= ENET_PACKET_FLAG_SENT as c_int as c_uint;
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
    return commandNumber;
}
unsafe fn enet_protocol_handle_connect<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut _header: *mut ENetProtocolHeader,
    mut command: *mut ENetProtocol,
) -> *mut ENetPeer<S> {
    let mut incomingSessionID: enet_uint8 = 0;
    let mut outgoingSessionID: enet_uint8 = 0;
    let mut mtu: enet_uint32 = 0;
    let mut windowSize: enet_uint32 = 0;
    let mut channel: *mut ENetChannel = 0 as *mut ENetChannel;
    let mut channelCount: size_t = 0;
    let mut duplicatePeers: size_t = 0 as c_int as size_t;
    let mut currentPeer: *mut ENetPeer<S> = 0 as *mut ENetPeer<S>;
    let mut peer: *mut ENetPeer<S> = 0 as *mut ENetPeer<S>;
    let mut verifyCommand: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    channelCount = ntohl((*command).connect.channelCount) as size_t;
    if channelCount < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as c_int as size_t
        || channelCount > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as c_int as size_t
    {
        return 0 as *mut ENetPeer<S>;
    }
    currentPeer = (*host).peers;
    while currentPeer < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer<S>
    {
        if (*currentPeer).state as c_uint == ENET_PEER_STATE_DISCONNECTED as c_int as c_uint {
            if peer.is_null() {
                peer = currentPeer;
            }
        } else if (*currentPeer).state as c_uint != ENET_PEER_STATE_CONNECTING as c_int as c_uint
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
                return 0 as *mut ENetPeer<S>;
            }
            duplicatePeers = duplicatePeers.wrapping_add(1);
        }
        currentPeer = currentPeer.offset(1);
    }
    if peer.is_null() || duplicatePeers >= (*host).duplicatePeers {
        return 0 as *mut ENetPeer<S>;
    }
    if channelCount > (*host).channelLimit {
        channelCount = (*host).channelLimit;
    }
    (*peer).channels =
        enet_malloc(channelCount.wrapping_mul(::core::mem::size_of::<ENetChannel>() as size_t))
            as *mut ENetChannel;
    if ((*peer).channels).is_null() {
        return 0 as *mut ENetPeer<S>;
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
    (*peer).outgoingPeerID = ntohs((*command).connect.outgoingPeerID);
    (*peer).incomingBandwidth = ntohl((*command).connect.incomingBandwidth);
    (*peer).outgoingBandwidth = ntohl((*command).connect.outgoingBandwidth);
    (*peer).packetThrottleInterval = ntohl((*command).connect.packetThrottleInterval);
    (*peer).packetThrottleAcceleration = ntohl((*command).connect.packetThrottleAcceleration);
    (*peer).packetThrottleDeceleration = ntohl((*command).connect.packetThrottleDeceleration);
    (*peer).eventData = ntohl((*command).connect.data);
    incomingSessionID = (if (*command).connect.incomingSessionID as c_int == 0xff as c_int {
        (*peer).outgoingSessionID as c_int
    } else {
        (*command).connect.incomingSessionID as c_int
    }) as enet_uint8;
    incomingSessionID = (incomingSessionID as c_int + 1 as c_int
        & ENET_PROTOCOL_HEADER_SESSION_MASK as c_int >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as c_int)
        as enet_uint8;
    if incomingSessionID as c_int == (*peer).outgoingSessionID as c_int {
        incomingSessionID = (incomingSessionID as c_int + 1 as c_int
            & ENET_PROTOCOL_HEADER_SESSION_MASK as c_int
                >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as c_int)
            as enet_uint8;
    }
    (*peer).outgoingSessionID = incomingSessionID;
    outgoingSessionID = (if (*command).connect.outgoingSessionID as c_int == 0xff as c_int {
        (*peer).incomingSessionID as c_int
    } else {
        (*command).connect.outgoingSessionID as c_int
    }) as enet_uint8;
    outgoingSessionID = (outgoingSessionID as c_int + 1 as c_int
        & ENET_PROTOCOL_HEADER_SESSION_MASK as c_int >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as c_int)
        as enet_uint8;
    if outgoingSessionID as c_int == (*peer).incomingSessionID as c_int {
        outgoingSessionID = (outgoingSessionID as c_int + 1 as c_int
            & ENET_PROTOCOL_HEADER_SESSION_MASK as c_int
                >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as c_int)
            as enet_uint8;
    }
    (*peer).incomingSessionID = outgoingSessionID;
    channel = (*peer).channels;
    while channel < &mut *((*peer).channels).offset(channelCount as isize) as *mut ENetChannel {
        (*channel).outgoingReliableSequenceNumber = 0 as c_int as enet_uint16;
        (*channel).outgoingUnreliableSequenceNumber = 0 as c_int as enet_uint16;
        (*channel).incomingReliableSequenceNumber = 0 as c_int as enet_uint16;
        (*channel).incomingUnreliableSequenceNumber = 0 as c_int as enet_uint16;
        enet_list_clear(&mut (*channel).incomingReliableCommands);
        enet_list_clear(&mut (*channel).incomingUnreliableCommands);
        (*channel).usedReliableWindows = 0 as c_int as enet_uint16;
        _enet_memset(
            ((*channel).reliableWindows).as_mut_ptr() as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<[enet_uint16; 16]>() as size_t,
        );
        channel = channel.offset(1);
    }
    mtu = ntohl((*command).connect.mtu);
    if mtu < ENET_PROTOCOL_MINIMUM_MTU as c_int as c_uint {
        mtu = ENET_PROTOCOL_MINIMUM_MTU as c_int as enet_uint32;
    } else if mtu > ENET_PROTOCOL_MAXIMUM_MTU as c_int as c_uint {
        mtu = ENET_PROTOCOL_MAXIMUM_MTU as c_int as enet_uint32;
    }
    if mtu < (*peer).mtu {
        (*peer).mtu = mtu;
    }
    if (*host).outgoingBandwidth == 0 as c_int as c_uint
        && (*peer).incomingBandwidth == 0 as c_int as c_uint
    {
        (*peer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as c_int as enet_uint32;
    } else if (*host).outgoingBandwidth == 0 as c_int as c_uint
        || (*peer).incomingBandwidth == 0 as c_int as c_uint
    {
        (*peer).windowSize = (if (*host).outgoingBandwidth > (*peer).incomingBandwidth {
            (*host).outgoingBandwidth
        } else {
            (*peer).incomingBandwidth
        })
        .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as c_int as c_uint)
        .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as c_int as c_uint);
    } else {
        (*peer).windowSize = (if (*host).outgoingBandwidth < (*peer).incomingBandwidth {
            (*host).outgoingBandwidth
        } else {
            (*peer).incomingBandwidth
        })
        .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as c_int as c_uint)
        .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as c_int as c_uint);
    }
    if (*peer).windowSize < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as c_int as c_uint {
        (*peer).windowSize = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as c_int as enet_uint32;
    } else if (*peer).windowSize > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as c_int as c_uint {
        (*peer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as c_int as enet_uint32;
    }
    if (*host).incomingBandwidth == 0 as c_int as c_uint {
        windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as c_int as enet_uint32;
    } else {
        windowSize = ((*host).incomingBandwidth)
            .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as c_int as c_uint)
            .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as c_int as c_uint);
    }
    if windowSize > ntohl((*command).connect.windowSize) {
        windowSize = ntohl((*command).connect.windowSize);
    }
    if windowSize < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as c_int as c_uint {
        windowSize = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as c_int as enet_uint32;
    } else if windowSize > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as c_int as c_uint {
        windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as c_int as enet_uint32;
    }
    verifyCommand.header.command = (ENET_PROTOCOL_COMMAND_VERIFY_CONNECT as c_int
        | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as c_int)
        as enet_uint8;
    verifyCommand.header.channelID = 0xff as c_int as enet_uint8;
    verifyCommand.verifyConnect.outgoingPeerID = htons((*peer).incomingPeerID);
    verifyCommand.verifyConnect.incomingSessionID = incomingSessionID;
    verifyCommand.verifyConnect.outgoingSessionID = outgoingSessionID;
    verifyCommand.verifyConnect.mtu = htonl((*peer).mtu);
    verifyCommand.verifyConnect.windowSize = htonl(windowSize);
    verifyCommand.verifyConnect.channelCount = htonl(channelCount as uint32_t);
    verifyCommand.verifyConnect.incomingBandwidth = htonl((*host).incomingBandwidth);
    verifyCommand.verifyConnect.outgoingBandwidth = htonl((*host).outgoingBandwidth);
    verifyCommand.verifyConnect.packetThrottleInterval = htonl((*peer).packetThrottleInterval);
    verifyCommand.verifyConnect.packetThrottleAcceleration =
        htonl((*peer).packetThrottleAcceleration);
    verifyCommand.verifyConnect.packetThrottleDeceleration =
        htonl((*peer).packetThrottleDeceleration);
    verifyCommand.verifyConnect.connectID = (*peer).connectID;
    enet_peer_queue_outgoing_command(
        peer,
        &mut verifyCommand,
        0 as *mut ENetPacket,
        0 as c_int as enet_uint32,
        0 as c_int as enet_uint16,
    );
    return peer;
}
unsafe fn enet_protocol_handle_send_reliable<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut peer: *mut ENetPeer<S>,
    mut command: *const ENetProtocol,
    mut currentData: *mut *mut enet_uint8,
) -> c_int {
    let mut dataLength: size_t = 0;
    if (*command).header.channelID as size_t >= (*peer).channelCount
        || (*peer).state as c_uint != ENET_PEER_STATE_CONNECTED as c_int as c_uint
            && (*peer).state as c_uint != ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
    {
        return -(1 as c_int);
    }
    dataLength = ntohs((*command).sendReliable.dataLength) as size_t;
    *currentData = (*currentData).offset(dataLength as isize);
    if dataLength > (*host).maximumPacketSize
        || *currentData < (*host).receivedData
        || *currentData
            > &mut *((*host).receivedData).offset((*host).receivedDataLength as isize)
                as *mut enet_uint8
    {
        return -(1 as c_int);
    }
    if (enet_peer_queue_incoming_command(
        peer,
        command,
        (command as *const enet_uint8)
            .offset(::core::mem::size_of::<ENetProtocolSendReliable>() as c_ulong as isize)
            as *const c_void,
        dataLength,
        ENET_PACKET_FLAG_RELIABLE as c_int as enet_uint32,
        0 as c_int as enet_uint32,
    ))
    .is_null()
    {
        return -(1 as c_int);
    }
    return 0 as c_int;
}
unsafe fn enet_protocol_handle_send_unsequenced<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut peer: *mut ENetPeer<S>,
    mut command: *const ENetProtocol,
    mut currentData: *mut *mut enet_uint8,
) -> c_int {
    let mut unsequencedGroup: enet_uint32 = 0;
    let mut index: enet_uint32 = 0;
    let mut dataLength: size_t = 0;
    if (*command).header.channelID as size_t >= (*peer).channelCount
        || (*peer).state as c_uint != ENET_PEER_STATE_CONNECTED as c_int as c_uint
            && (*peer).state as c_uint != ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
    {
        return -(1 as c_int);
    }
    dataLength = ntohs((*command).sendUnsequenced.dataLength) as size_t;
    *currentData = (*currentData).offset(dataLength as isize);
    if dataLength > (*host).maximumPacketSize
        || *currentData < (*host).receivedData
        || *currentData
            > &mut *((*host).receivedData).offset((*host).receivedDataLength as isize)
                as *mut enet_uint8
    {
        return -(1 as c_int);
    }
    unsequencedGroup = ntohs((*command).sendUnsequenced.unsequencedGroup) as enet_uint32;
    index = unsequencedGroup.wrapping_rem(ENET_PEER_UNSEQUENCED_WINDOW_SIZE as c_int as c_uint);
    if unsequencedGroup < (*peer).incomingUnsequencedGroup as c_uint {
        unsequencedGroup = (unsequencedGroup as c_uint).wrapping_add(0x10000 as c_int as c_uint)
            as enet_uint32 as enet_uint32;
    }
    if unsequencedGroup
        >= ((*peer).incomingUnsequencedGroup as enet_uint32).wrapping_add(
            (ENET_PEER_FREE_UNSEQUENCED_WINDOWS as c_int
                * ENET_PEER_UNSEQUENCED_WINDOW_SIZE as c_int) as c_uint,
        )
    {
        return 0 as c_int;
    }
    unsequencedGroup &= 0xffff as c_int as c_uint;
    if unsequencedGroup.wrapping_sub(index) != (*peer).incomingUnsequencedGroup as c_uint {
        (*peer).incomingUnsequencedGroup = unsequencedGroup.wrapping_sub(index) as enet_uint16;
        _enet_memset(
            ((*peer).unsequencedWindow).as_mut_ptr() as *mut c_void,
            0 as c_int,
            ::core::mem::size_of::<[enet_uint32; 32]>() as size_t,
        );
    } else if (*peer).unsequencedWindow[index.wrapping_div(32 as c_int as c_uint) as usize]
        & ((1 as c_int) << index.wrapping_rem(32 as c_int as c_uint)) as c_uint
        != 0
    {
        return 0 as c_int;
    }
    if (enet_peer_queue_incoming_command(
        peer,
        command,
        (command as *const enet_uint8)
            .offset(::core::mem::size_of::<ENetProtocolSendUnsequenced>() as c_ulong as isize)
            as *const c_void,
        dataLength,
        ENET_PACKET_FLAG_UNSEQUENCED as c_int as enet_uint32,
        0 as c_int as enet_uint32,
    ))
    .is_null()
    {
        return -(1 as c_int);
    }
    (*peer).unsequencedWindow[index.wrapping_div(32 as c_int as c_uint) as usize] |=
        ((1 as c_int) << index.wrapping_rem(32 as c_int as c_uint)) as c_uint;
    return 0 as c_int;
}
unsafe fn enet_protocol_handle_send_unreliable<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut peer: *mut ENetPeer<S>,
    mut command: *const ENetProtocol,
    mut currentData: *mut *mut enet_uint8,
) -> c_int {
    let mut dataLength: size_t = 0;
    if (*command).header.channelID as size_t >= (*peer).channelCount
        || (*peer).state as c_uint != ENET_PEER_STATE_CONNECTED as c_int as c_uint
            && (*peer).state as c_uint != ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
    {
        return -(1 as c_int);
    }
    dataLength = ntohs((*command).sendUnreliable.dataLength) as size_t;
    *currentData = (*currentData).offset(dataLength as isize);
    if dataLength > (*host).maximumPacketSize
        || *currentData < (*host).receivedData
        || *currentData
            > &mut *((*host).receivedData).offset((*host).receivedDataLength as isize)
                as *mut enet_uint8
    {
        return -(1 as c_int);
    }
    if (enet_peer_queue_incoming_command(
        peer,
        command,
        (command as *const enet_uint8)
            .offset(::core::mem::size_of::<ENetProtocolSendUnreliable>() as c_ulong as isize)
            as *const c_void,
        dataLength,
        0 as c_int as enet_uint32,
        0 as c_int as enet_uint32,
    ))
    .is_null()
    {
        return -(1 as c_int);
    }
    return 0 as c_int;
}
unsafe fn enet_protocol_handle_send_fragment<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut peer: *mut ENetPeer<S>,
    mut command: *const ENetProtocol,
    mut currentData: *mut *mut enet_uint8,
) -> c_int {
    let mut fragmentNumber: enet_uint32 = 0;
    let mut fragmentCount: enet_uint32 = 0;
    let mut fragmentOffset: enet_uint32 = 0;
    let mut fragmentLength: enet_uint32 = 0;
    let mut startSequenceNumber: enet_uint32 = 0;
    let mut totalLength: enet_uint32 = 0;
    let mut channel: *mut ENetChannel = 0 as *mut ENetChannel;
    let mut startWindow: enet_uint16 = 0;
    let mut currentWindow: enet_uint16 = 0;
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut startCommand: *mut ENetIncomingCommand = 0 as *mut ENetIncomingCommand;
    if (*command).header.channelID as size_t >= (*peer).channelCount
        || (*peer).state as c_uint != ENET_PEER_STATE_CONNECTED as c_int as c_uint
            && (*peer).state as c_uint != ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
    {
        return -(1 as c_int);
    }
    fragmentLength = ntohs((*command).sendFragment.dataLength) as enet_uint32;
    *currentData = (*currentData).offset(fragmentLength as isize);
    if fragmentLength <= 0 as c_int as c_uint
        || fragmentLength as size_t > (*host).maximumPacketSize
        || *currentData < (*host).receivedData
        || *currentData
            > &mut *((*host).receivedData).offset((*host).receivedDataLength as isize)
                as *mut enet_uint8
    {
        return -(1 as c_int);
    }
    channel =
        &mut *((*peer).channels).offset((*command).header.channelID as isize) as *mut ENetChannel;
    startSequenceNumber = ntohs((*command).sendFragment.startSequenceNumber) as enet_uint32;
    startWindow = startSequenceNumber
        .wrapping_div(ENET_PEER_RELIABLE_WINDOW_SIZE as c_int as c_uint)
        as enet_uint16;
    currentWindow = ((*channel).incomingReliableSequenceNumber as c_int
        / ENET_PEER_RELIABLE_WINDOW_SIZE as c_int) as enet_uint16;
    if startSequenceNumber < (*channel).incomingReliableSequenceNumber as c_uint {
        startWindow = (startWindow as c_int + ENET_PEER_RELIABLE_WINDOWS as c_int) as enet_uint16;
    }
    if (startWindow as c_int) < currentWindow as c_int
        || startWindow as c_int
            >= currentWindow as c_int + ENET_PEER_FREE_RELIABLE_WINDOWS as c_int - 1 as c_int
    {
        return 0 as c_int;
    }
    fragmentNumber = ntohl((*command).sendFragment.fragmentNumber);
    fragmentCount = ntohl((*command).sendFragment.fragmentCount);
    fragmentOffset = ntohl((*command).sendFragment.fragmentOffset);
    totalLength = ntohl((*command).sendFragment.totalLength);
    if fragmentCount > ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as c_int as c_uint
        || fragmentNumber >= fragmentCount
        || totalLength as size_t > (*host).maximumPacketSize
        || totalLength < fragmentCount
        || fragmentOffset >= totalLength
        || fragmentLength > totalLength.wrapping_sub(fragmentOffset)
    {
        return -(1 as c_int);
    }
    let mut current_block_23: u64;
    currentCommand = (*channel).incomingReliableCommands.sentinel.previous;
    while currentCommand != &mut (*channel).incomingReliableCommands.sentinel as *mut ENetListNode {
        let mut incomingCommand: *mut ENetIncomingCommand =
            currentCommand as *mut ENetIncomingCommand;
        if startSequenceNumber >= (*channel).incomingReliableSequenceNumber as c_uint {
            if ((*incomingCommand).reliableSequenceNumber as c_int)
                < (*channel).incomingReliableSequenceNumber as c_int
            {
                current_block_23 = 13056961889198038528;
            } else {
                current_block_23 = 12147880666119273379;
            }
        } else {
            if (*incomingCommand).reliableSequenceNumber as c_int
                >= (*channel).incomingReliableSequenceNumber as c_int
            {
                break;
            }
            current_block_23 = 12147880666119273379;
        }
        match current_block_23 {
            12147880666119273379 => {
                if (*incomingCommand).reliableSequenceNumber as c_uint <= startSequenceNumber {
                    if ((*incomingCommand).reliableSequenceNumber as c_uint) < startSequenceNumber {
                        break;
                    }
                    if (*incomingCommand).command.header.command as c_int
                        & ENET_PROTOCOL_COMMAND_MASK as c_int
                        != ENET_PROTOCOL_COMMAND_SEND_FRAGMENT as c_int
                        || totalLength as size_t != (*(*incomingCommand).packet).dataLength
                        || fragmentCount != (*incomingCommand).fragmentCount
                    {
                        return -(1 as c_int);
                    }
                    startCommand = incomingCommand;
                    break;
                }
            }
            _ => {}
        }
        currentCommand = (*currentCommand).previous;
    }
    if startCommand.is_null() {
        let mut hostCommand: ENetProtocol = *command;
        hostCommand.header.reliableSequenceNumber = startSequenceNumber as enet_uint16;
        startCommand = enet_peer_queue_incoming_command(
            peer,
            &mut hostCommand,
            0 as *const c_void,
            totalLength as size_t,
            ENET_PACKET_FLAG_RELIABLE as c_int as enet_uint32,
            fragmentCount,
        );
        if startCommand.is_null() {
            return -(1 as c_int);
        }
    }
    if *((*startCommand).fragments)
        .offset(fragmentNumber.wrapping_div(32 as c_int as c_uint) as isize)
        & ((1 as c_int) << fragmentNumber.wrapping_rem(32 as c_int as c_uint)) as c_uint
        == 0 as c_int as c_uint
    {
        (*startCommand).fragmentsRemaining = ((*startCommand).fragmentsRemaining).wrapping_sub(1);
        let ref mut fresh32 = *((*startCommand).fragments)
            .offset(fragmentNumber.wrapping_div(32 as c_int as c_uint) as isize);
        *fresh32 |= ((1 as c_int) << fragmentNumber.wrapping_rem(32 as c_int as c_uint)) as c_uint;
        if fragmentOffset.wrapping_add(fragmentLength) as size_t
            > (*(*startCommand).packet).dataLength
        {
            fragmentLength = ((*(*startCommand).packet).dataLength)
                .wrapping_sub(fragmentOffset as size_t) as enet_uint32;
        }
        _enet_memcpy(
            ((*(*startCommand).packet).data).offset(fragmentOffset as isize) as *mut c_void,
            (command as *mut enet_uint8)
                .offset(::core::mem::size_of::<ENetProtocolSendFragment>() as c_ulong as isize)
                as *const c_void,
            fragmentLength as size_t,
        );
        if (*startCommand).fragmentsRemaining <= 0 as c_int as c_uint {
            enet_peer_dispatch_incoming_reliable_commands(
                peer,
                channel,
                0 as *mut ENetIncomingCommand,
            );
        }
    }
    return 0 as c_int;
}
unsafe fn enet_protocol_handle_send_unreliable_fragment<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut peer: *mut ENetPeer<S>,
    mut command: *const ENetProtocol,
    mut currentData: *mut *mut enet_uint8,
) -> c_int {
    let mut fragmentNumber: enet_uint32 = 0;
    let mut fragmentCount: enet_uint32 = 0;
    let mut fragmentOffset: enet_uint32 = 0;
    let mut fragmentLength: enet_uint32 = 0;
    let mut reliableSequenceNumber: enet_uint32 = 0;
    let mut startSequenceNumber: enet_uint32 = 0;
    let mut totalLength: enet_uint32 = 0;
    let mut reliableWindow: enet_uint16 = 0;
    let mut currentWindow: enet_uint16 = 0;
    let mut channel: *mut ENetChannel = 0 as *mut ENetChannel;
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut startCommand: *mut ENetIncomingCommand = 0 as *mut ENetIncomingCommand;
    if (*command).header.channelID as size_t >= (*peer).channelCount
        || (*peer).state as c_uint != ENET_PEER_STATE_CONNECTED as c_int as c_uint
            && (*peer).state as c_uint != ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
    {
        return -(1 as c_int);
    }
    fragmentLength = ntohs((*command).sendFragment.dataLength) as enet_uint32;
    *currentData = (*currentData).offset(fragmentLength as isize);
    if fragmentLength as size_t > (*host).maximumPacketSize
        || *currentData < (*host).receivedData
        || *currentData
            > &mut *((*host).receivedData).offset((*host).receivedDataLength as isize)
                as *mut enet_uint8
    {
        return -(1 as c_int);
    }
    channel =
        &mut *((*peer).channels).offset((*command).header.channelID as isize) as *mut ENetChannel;
    reliableSequenceNumber = (*command).header.reliableSequenceNumber as enet_uint32;
    startSequenceNumber = ntohs((*command).sendFragment.startSequenceNumber) as enet_uint32;
    reliableWindow = reliableSequenceNumber
        .wrapping_div(ENET_PEER_RELIABLE_WINDOW_SIZE as c_int as c_uint)
        as enet_uint16;
    currentWindow = ((*channel).incomingReliableSequenceNumber as c_int
        / ENET_PEER_RELIABLE_WINDOW_SIZE as c_int) as enet_uint16;
    if reliableSequenceNumber < (*channel).incomingReliableSequenceNumber as c_uint {
        reliableWindow =
            (reliableWindow as c_int + ENET_PEER_RELIABLE_WINDOWS as c_int) as enet_uint16;
    }
    if (reliableWindow as c_int) < currentWindow as c_int
        || reliableWindow as c_int
            >= currentWindow as c_int + ENET_PEER_FREE_RELIABLE_WINDOWS as c_int - 1 as c_int
    {
        return 0 as c_int;
    }
    if reliableSequenceNumber == (*channel).incomingReliableSequenceNumber as c_uint
        && startSequenceNumber <= (*channel).incomingUnreliableSequenceNumber as c_uint
    {
        return 0 as c_int;
    }
    fragmentNumber = ntohl((*command).sendFragment.fragmentNumber);
    fragmentCount = ntohl((*command).sendFragment.fragmentCount);
    fragmentOffset = ntohl((*command).sendFragment.fragmentOffset);
    totalLength = ntohl((*command).sendFragment.totalLength);
    if fragmentCount > ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as c_int as c_uint
        || fragmentNumber >= fragmentCount
        || totalLength as size_t > (*host).maximumPacketSize
        || fragmentOffset >= totalLength
        || fragmentLength > totalLength.wrapping_sub(fragmentOffset)
    {
        return -(1 as c_int);
    }
    let mut current_block_26: u64;
    currentCommand = (*channel).incomingUnreliableCommands.sentinel.previous;
    while currentCommand != &mut (*channel).incomingUnreliableCommands.sentinel as *mut ENetListNode
    {
        let mut incomingCommand: *mut ENetIncomingCommand =
            currentCommand as *mut ENetIncomingCommand;
        if reliableSequenceNumber >= (*channel).incomingReliableSequenceNumber as c_uint {
            if ((*incomingCommand).reliableSequenceNumber as c_int)
                < (*channel).incomingReliableSequenceNumber as c_int
            {
                current_block_26 = 8457315219000651999;
            } else {
                current_block_26 = 1109700713171191020;
            }
        } else {
            if (*incomingCommand).reliableSequenceNumber as c_int
                >= (*channel).incomingReliableSequenceNumber as c_int
            {
                break;
            }
            current_block_26 = 1109700713171191020;
        }
        match current_block_26 {
            1109700713171191020 => {
                if ((*incomingCommand).reliableSequenceNumber as c_uint) < reliableSequenceNumber {
                    break;
                }
                if !((*incomingCommand).reliableSequenceNumber as c_uint > reliableSequenceNumber) {
                    if (*incomingCommand).unreliableSequenceNumber as c_uint <= startSequenceNumber
                    {
                        if ((*incomingCommand).unreliableSequenceNumber as c_uint)
                            < startSequenceNumber
                        {
                            break;
                        }
                        if (*incomingCommand).command.header.command as c_int
                            & ENET_PROTOCOL_COMMAND_MASK as c_int
                            != ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT as c_int
                            || totalLength as size_t != (*(*incomingCommand).packet).dataLength
                            || fragmentCount != (*incomingCommand).fragmentCount
                        {
                            return -(1 as c_int);
                        }
                        startCommand = incomingCommand;
                        break;
                    }
                }
            }
            _ => {}
        }
        currentCommand = (*currentCommand).previous;
    }
    if startCommand.is_null() {
        startCommand = enet_peer_queue_incoming_command(
            peer,
            command,
            0 as *const c_void,
            totalLength as size_t,
            ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT as c_int as enet_uint32,
            fragmentCount,
        );
        if startCommand.is_null() {
            return -(1 as c_int);
        }
    }
    if *((*startCommand).fragments)
        .offset(fragmentNumber.wrapping_div(32 as c_int as c_uint) as isize)
        & ((1 as c_int) << fragmentNumber.wrapping_rem(32 as c_int as c_uint)) as c_uint
        == 0 as c_int as c_uint
    {
        (*startCommand).fragmentsRemaining = ((*startCommand).fragmentsRemaining).wrapping_sub(1);
        let ref mut fresh33 = *((*startCommand).fragments)
            .offset(fragmentNumber.wrapping_div(32 as c_int as c_uint) as isize);
        *fresh33 |= ((1 as c_int) << fragmentNumber.wrapping_rem(32 as c_int as c_uint)) as c_uint;
        if fragmentOffset.wrapping_add(fragmentLength) as size_t
            > (*(*startCommand).packet).dataLength
        {
            fragmentLength = ((*(*startCommand).packet).dataLength)
                .wrapping_sub(fragmentOffset as size_t) as enet_uint32;
        }
        _enet_memcpy(
            ((*(*startCommand).packet).data).offset(fragmentOffset as isize) as *mut c_void,
            (command as *mut enet_uint8)
                .offset(::core::mem::size_of::<ENetProtocolSendFragment>() as c_ulong as isize)
                as *const c_void,
            fragmentLength as size_t,
        );
        if (*startCommand).fragmentsRemaining <= 0 as c_int as c_uint {
            enet_peer_dispatch_incoming_unreliable_commands(
                peer,
                channel,
                0 as *mut ENetIncomingCommand,
            );
        }
    }
    return 0 as c_int;
}
unsafe fn enet_protocol_handle_ping<S: Socket>(
    mut _host: *mut ENetHost<S>,
    mut peer: *mut ENetPeer<S>,
    mut _command: *const ENetProtocol,
) -> c_int {
    if (*peer).state as c_uint != ENET_PEER_STATE_CONNECTED as c_int as c_uint
        && (*peer).state as c_uint != ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
    {
        return -(1 as c_int);
    }
    return 0 as c_int;
}
unsafe fn enet_protocol_handle_bandwidth_limit<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut peer: *mut ENetPeer<S>,
    mut command: *const ENetProtocol,
) -> c_int {
    if (*peer).state as c_uint != ENET_PEER_STATE_CONNECTED as c_int as c_uint
        && (*peer).state as c_uint != ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
    {
        return -(1 as c_int);
    }
    if (*peer).incomingBandwidth != 0 as c_int as c_uint {
        (*host).bandwidthLimitedPeers = ((*host).bandwidthLimitedPeers).wrapping_sub(1);
    }
    (*peer).incomingBandwidth = ntohl((*command).bandwidthLimit.incomingBandwidth);
    (*peer).outgoingBandwidth = ntohl((*command).bandwidthLimit.outgoingBandwidth);
    if (*peer).incomingBandwidth != 0 as c_int as c_uint {
        (*host).bandwidthLimitedPeers = ((*host).bandwidthLimitedPeers).wrapping_add(1);
    }
    if (*peer).incomingBandwidth == 0 as c_int as c_uint
        && (*host).outgoingBandwidth == 0 as c_int as c_uint
    {
        (*peer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as c_int as enet_uint32;
    } else if (*peer).incomingBandwidth == 0 as c_int as c_uint
        || (*host).outgoingBandwidth == 0 as c_int as c_uint
    {
        (*peer).windowSize = (if (*peer).incomingBandwidth > (*host).outgoingBandwidth {
            (*peer).incomingBandwidth
        } else {
            (*host).outgoingBandwidth
        })
        .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as c_int as c_uint)
        .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as c_int as c_uint);
    } else {
        (*peer).windowSize = (if (*peer).incomingBandwidth < (*host).outgoingBandwidth {
            (*peer).incomingBandwidth
        } else {
            (*host).outgoingBandwidth
        })
        .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as c_int as c_uint)
        .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as c_int as c_uint);
    }
    if (*peer).windowSize < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as c_int as c_uint {
        (*peer).windowSize = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as c_int as enet_uint32;
    } else if (*peer).windowSize > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as c_int as c_uint {
        (*peer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as c_int as enet_uint32;
    }
    return 0 as c_int;
}
unsafe fn enet_protocol_handle_throttle_configure<S: Socket>(
    mut _host: *mut ENetHost<S>,
    mut peer: *mut ENetPeer<S>,
    mut command: *const ENetProtocol,
) -> c_int {
    if (*peer).state as c_uint != ENET_PEER_STATE_CONNECTED as c_int as c_uint
        && (*peer).state as c_uint != ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
    {
        return -(1 as c_int);
    }
    (*peer).packetThrottleInterval = ntohl((*command).throttleConfigure.packetThrottleInterval);
    (*peer).packetThrottleAcceleration =
        ntohl((*command).throttleConfigure.packetThrottleAcceleration);
    (*peer).packetThrottleDeceleration =
        ntohl((*command).throttleConfigure.packetThrottleDeceleration);
    return 0 as c_int;
}
unsafe fn enet_protocol_handle_disconnect<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut peer: *mut ENetPeer<S>,
    mut command: *const ENetProtocol,
) -> c_int {
    if (*peer).state as c_uint == ENET_PEER_STATE_DISCONNECTED as c_int as c_uint
        || (*peer).state as c_uint == ENET_PEER_STATE_ZOMBIE as c_int as c_uint
        || (*peer).state as c_uint == ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT as c_int as c_uint
    {
        return 0 as c_int;
    }
    enet_peer_reset_queues(peer);
    if (*peer).state as c_uint == ENET_PEER_STATE_CONNECTION_SUCCEEDED as c_int as c_uint
        || (*peer).state as c_uint == ENET_PEER_STATE_DISCONNECTING as c_int as c_uint
        || (*peer).state as c_uint == ENET_PEER_STATE_CONNECTING as c_int as c_uint
    {
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
    } else if (*peer).state as c_uint != ENET_PEER_STATE_CONNECTED as c_int as c_uint
        && (*peer).state as c_uint != ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
    {
        if (*peer).state as c_uint == ENET_PEER_STATE_CONNECTION_PENDING as c_int as c_uint {
            (*host).recalculateBandwidthLimits = 1 as c_int;
        }
        enet_peer_reset(peer);
    } else if (*command).header.command as c_int & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as c_int
        != 0
    {
        enet_protocol_change_state(host, peer, ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT);
    } else {
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
    }
    if (*peer).state as c_uint != ENET_PEER_STATE_DISCONNECTED as c_int as c_uint {
        (*peer).eventData = ntohl((*command).disconnect.data);
    }
    return 0 as c_int;
}
unsafe fn enet_protocol_handle_acknowledge<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut event: *mut ENetEvent<S>,
    mut peer: *mut ENetPeer<S>,
    mut command: *const ENetProtocol,
) -> c_int {
    let mut roundTripTime: enet_uint32 = 0;
    let mut receivedSentTime: enet_uint32 = 0;
    let mut receivedReliableSequenceNumber: enet_uint32 = 0;
    let mut commandNumber: ENetProtocolCommand = ENET_PROTOCOL_COMMAND_NONE;
    if (*peer).state as c_uint == ENET_PEER_STATE_DISCONNECTED as c_int as c_uint
        || (*peer).state as c_uint == ENET_PEER_STATE_ZOMBIE as c_int as c_uint
    {
        return 0 as c_int;
    }
    receivedSentTime = ntohs((*command).acknowledge.receivedSentTime) as enet_uint32;
    receivedSentTime |= (*host).serviceTime & 0xffff0000 as c_uint;
    if receivedSentTime & 0x8000 as c_int as c_uint
        > (*host).serviceTime & 0x8000 as c_int as c_uint
    {
        receivedSentTime = (receivedSentTime as c_uint).wrapping_sub(0x10000 as c_int as c_uint)
            as enet_uint32 as enet_uint32;
    }
    if ((*host).serviceTime).wrapping_sub(receivedSentTime) >= 86400000 as c_int as c_uint {
        return 0 as c_int;
    }
    roundTripTime =
        if ((*host).serviceTime).wrapping_sub(receivedSentTime) >= 86400000 as c_int as c_uint {
            receivedSentTime.wrapping_sub((*host).serviceTime)
        } else {
            ((*host).serviceTime).wrapping_sub(receivedSentTime)
        };
    roundTripTime = if roundTripTime > 1 as c_int as c_uint {
        roundTripTime
    } else {
        1 as c_int as c_uint
    };
    if (*peer).lastReceiveTime > 0 as c_int as c_uint {
        enet_peer_throttle(peer, roundTripTime);
        (*peer).roundTripTimeVariance = ((*peer).roundTripTimeVariance as c_uint)
            .wrapping_sub(((*peer).roundTripTimeVariance).wrapping_div(4 as c_int as c_uint))
            as enet_uint32 as enet_uint32;
        if roundTripTime >= (*peer).roundTripTime {
            let mut diff: enet_uint32 = roundTripTime.wrapping_sub((*peer).roundTripTime);
            (*peer).roundTripTimeVariance = ((*peer).roundTripTimeVariance as c_uint)
                .wrapping_add(diff.wrapping_div(4 as c_int as c_uint))
                as enet_uint32 as enet_uint32;
            (*peer).roundTripTime = ((*peer).roundTripTime as c_uint)
                .wrapping_add(diff.wrapping_div(8 as c_int as c_uint))
                as enet_uint32 as enet_uint32;
        } else {
            let mut diff_0: enet_uint32 = ((*peer).roundTripTime).wrapping_sub(roundTripTime);
            (*peer).roundTripTimeVariance = ((*peer).roundTripTimeVariance as c_uint)
                .wrapping_add(diff_0.wrapping_div(4 as c_int as c_uint))
                as enet_uint32 as enet_uint32;
            (*peer).roundTripTime = ((*peer).roundTripTime as c_uint)
                .wrapping_sub(diff_0.wrapping_div(8 as c_int as c_uint))
                as enet_uint32 as enet_uint32;
        }
    } else {
        (*peer).roundTripTime = roundTripTime;
        (*peer).roundTripTimeVariance = roundTripTime
            .wrapping_add(1 as c_int as c_uint)
            .wrapping_div(2 as c_int as c_uint);
    }
    if (*peer).roundTripTime < (*peer).lowestRoundTripTime {
        (*peer).lowestRoundTripTime = (*peer).roundTripTime;
    }
    if (*peer).roundTripTimeVariance > (*peer).highestRoundTripTimeVariance {
        (*peer).highestRoundTripTimeVariance = (*peer).roundTripTimeVariance;
    }
    if (*peer).packetThrottleEpoch == 0 as c_int as c_uint
        || (if ((*host).serviceTime).wrapping_sub((*peer).packetThrottleEpoch)
            >= 86400000 as c_int as c_uint
        {
            ((*peer).packetThrottleEpoch).wrapping_sub((*host).serviceTime)
        } else {
            ((*host).serviceTime).wrapping_sub((*peer).packetThrottleEpoch)
        }) >= (*peer).packetThrottleInterval
    {
        (*peer).lastRoundTripTime = (*peer).lowestRoundTripTime;
        (*peer).lastRoundTripTimeVariance =
            if (*peer).highestRoundTripTimeVariance > 1 as c_int as c_uint {
                (*peer).highestRoundTripTimeVariance
            } else {
                1 as c_int as c_uint
            };
        (*peer).lowestRoundTripTime = (*peer).roundTripTime;
        (*peer).highestRoundTripTimeVariance = (*peer).roundTripTimeVariance;
        (*peer).packetThrottleEpoch = (*host).serviceTime;
    }
    (*peer).lastReceiveTime = if (*host).serviceTime > 1 as c_int as c_uint {
        (*host).serviceTime
    } else {
        1 as c_int as c_uint
    };
    (*peer).earliestTimeout = 0 as c_int as enet_uint32;
    receivedReliableSequenceNumber =
        ntohs((*command).acknowledge.receivedReliableSequenceNumber) as enet_uint32;
    commandNumber = enet_protocol_remove_sent_reliable_command(
        peer,
        receivedReliableSequenceNumber as enet_uint16,
        (*command).header.channelID,
    );
    match (*peer).state as c_uint {
        2 => {
            if commandNumber as c_uint != ENET_PROTOCOL_COMMAND_VERIFY_CONNECT as c_int as c_uint {
                return -(1 as c_int);
            }
            enet_protocol_notify_connect(host, peer, event);
        }
        7 => {
            if commandNumber as c_uint != ENET_PROTOCOL_COMMAND_DISCONNECT as c_int as c_uint {
                return -(1 as c_int);
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
    return 0 as c_int;
}
unsafe fn enet_protocol_handle_verify_connect<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut event: *mut ENetEvent<S>,
    mut peer: *mut ENetPeer<S>,
    mut command: *const ENetProtocol,
) -> c_int {
    let mut mtu: enet_uint32 = 0;
    let mut windowSize: enet_uint32 = 0;
    let mut channelCount: size_t = 0;
    if (*peer).state as c_uint != ENET_PEER_STATE_CONNECTING as c_int as c_uint {
        return 0 as c_int;
    }
    channelCount = ntohl((*command).verifyConnect.channelCount) as size_t;
    if channelCount < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as c_int as size_t
        || channelCount > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as c_int as size_t
        || ntohl((*command).verifyConnect.packetThrottleInterval) != (*peer).packetThrottleInterval
        || ntohl((*command).verifyConnect.packetThrottleAcceleration)
            != (*peer).packetThrottleAcceleration
        || ntohl((*command).verifyConnect.packetThrottleDeceleration)
            != (*peer).packetThrottleDeceleration
        || (*command).verifyConnect.connectID != (*peer).connectID
    {
        (*peer).eventData = 0 as c_int as enet_uint32;
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
        return -(1 as c_int);
    }
    enet_protocol_remove_sent_reliable_command(
        peer,
        1 as c_int as enet_uint16,
        0xff as c_int as enet_uint8,
    );
    if channelCount < (*peer).channelCount {
        (*peer).channelCount = channelCount;
    }
    (*peer).outgoingPeerID = ntohs((*command).verifyConnect.outgoingPeerID);
    (*peer).incomingSessionID = (*command).verifyConnect.incomingSessionID;
    (*peer).outgoingSessionID = (*command).verifyConnect.outgoingSessionID;
    mtu = ntohl((*command).verifyConnect.mtu);
    if mtu < ENET_PROTOCOL_MINIMUM_MTU as c_int as c_uint {
        mtu = ENET_PROTOCOL_MINIMUM_MTU as c_int as enet_uint32;
    } else if mtu > ENET_PROTOCOL_MAXIMUM_MTU as c_int as c_uint {
        mtu = ENET_PROTOCOL_MAXIMUM_MTU as c_int as enet_uint32;
    }
    if mtu < (*peer).mtu {
        (*peer).mtu = mtu;
    }
    windowSize = ntohl((*command).verifyConnect.windowSize);
    if windowSize < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as c_int as c_uint {
        windowSize = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as c_int as enet_uint32;
    }
    if windowSize > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as c_int as c_uint {
        windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as c_int as enet_uint32;
    }
    if windowSize < (*peer).windowSize {
        (*peer).windowSize = windowSize;
    }
    (*peer).incomingBandwidth = ntohl((*command).verifyConnect.incomingBandwidth);
    (*peer).outgoingBandwidth = ntohl((*command).verifyConnect.outgoingBandwidth);
    enet_protocol_notify_connect(host, peer, event);
    return 0 as c_int;
}
unsafe fn enet_protocol_handle_incoming_commands<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut event: *mut ENetEvent<S>,
) -> c_int {
    let mut header: *mut ENetProtocolHeader = 0 as *mut ENetProtocolHeader;
    let mut command: *mut ENetProtocol = 0 as *mut ENetProtocol;
    let mut peer: *mut ENetPeer<S> = 0 as *mut ENetPeer<S>;
    let mut currentData: *mut enet_uint8 = 0 as *mut enet_uint8;
    let mut headerSize: size_t = 0;
    let mut peerID: enet_uint16 = 0;
    let mut flags: enet_uint16 = 0;
    let mut sessionID: enet_uint8 = 0;
    if (*host).receivedDataLength < 2 as size_t {
        return 0 as c_int;
    }
    header = (*host).receivedData as *mut ENetProtocolHeader;
    peerID = ntohs((*header).peerID);
    sessionID = ((peerID as c_int & ENET_PROTOCOL_HEADER_SESSION_MASK as c_int)
        >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as c_int) as enet_uint8;
    flags = (peerID as c_int & ENET_PROTOCOL_HEADER_FLAG_MASK as c_int) as enet_uint16;
    peerID = (peerID as c_int
        & !(ENET_PROTOCOL_HEADER_FLAG_MASK as c_int | ENET_PROTOCOL_HEADER_SESSION_MASK as c_int))
        as enet_uint16;
    headerSize = if flags as c_int & ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as c_int != 0 {
        ::core::mem::size_of::<ENetProtocolHeader>() as size_t
    } else {
        2 as size_t
    };
    if ((*host).checksum).is_some() {
        headerSize = (headerSize as c_ulong)
            .wrapping_add(::core::mem::size_of::<enet_uint32>() as c_ulong)
            as size_t as size_t;
    }
    if peerID as c_int == ENET_PROTOCOL_MAXIMUM_PEER_ID as c_int {
        peer = 0 as *mut ENetPeer<S>;
    } else if peerID as size_t >= (*host).peerCount {
        return 0 as c_int;
    } else {
        peer = &mut *((*host).peers).offset(peerID as isize) as *mut ENetPeer<S>;
        if (*peer).state as c_uint == ENET_PEER_STATE_DISCONNECTED as c_int as c_uint
            || (*peer).state as c_uint == ENET_PEER_STATE_ZOMBIE as c_int as c_uint
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
            || ((*peer).outgoingPeerID as c_int) < ENET_PROTOCOL_MAXIMUM_PEER_ID as c_int
                && sessionID as c_int != (*peer).incomingSessionID as c_int
        {
            return 0 as c_int;
        }
    }
    if flags as c_int & ENET_PROTOCOL_HEADER_FLAG_COMPRESSED as c_int != 0 {
        let mut originalSize: size_t = 0;
        if ((*host).compressor.context).is_null() || ((*host).compressor.decompress).is_none() {
            return 0 as c_int;
        }
        originalSize = ((*host).compressor.decompress).expect("non-null function pointer")(
            (*host).compressor.context,
            ((*host).receivedData).offset(headerSize as isize),
            ((*host).receivedDataLength).wrapping_sub(headerSize),
            ((*host).packetData[1 as c_int as usize])
                .as_mut_ptr()
                .offset(headerSize as isize),
            (::core::mem::size_of::<[enet_uint8; 4096]>() as size_t).wrapping_sub(headerSize),
        );
        if originalSize <= 0 as c_int as size_t
            || originalSize
                > (::core::mem::size_of::<[enet_uint8; 4096]>() as size_t).wrapping_sub(headerSize)
        {
            return 0 as c_int;
        }
        _enet_memcpy(
            ((*host).packetData[1 as c_int as usize]).as_mut_ptr() as *mut c_void,
            header as *const c_void,
            headerSize,
        );
        (*host).receivedData = ((*host).packetData[1 as c_int as usize]).as_mut_ptr();
        (*host).receivedDataLength = headerSize.wrapping_add(originalSize);
    }
    if ((*host).checksum).is_some() {
        let mut checksum: *mut enet_uint32 = &mut *((*host).receivedData).offset(
            headerSize.wrapping_sub(::core::mem::size_of::<enet_uint32>() as size_t) as isize,
        ) as *mut enet_uint8 as *mut enet_uint32;
        let mut desiredChecksum: enet_uint32 = *checksum;
        let mut buffer: ENetBuffer = ENetBuffer {
            data: 0 as *mut c_void,
            dataLength: 0,
        };
        *checksum = if !peer.is_null() {
            (*peer).connectID
        } else {
            0 as c_int as c_uint
        };
        buffer.data = (*host).receivedData as *mut c_void;
        buffer.dataLength = (*host).receivedDataLength;
        if ((*host).checksum).expect("non-null function pointer")(&mut buffer, 1 as c_int as size_t)
            != desiredChecksum
        {
            return 0 as c_int;
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
        (*peer).incomingDataTotal = ((*peer).incomingDataTotal as size_t)
            .wrapping_add((*host).receivedDataLength)
            as enet_uint32 as enet_uint32;
    }
    currentData = ((*host).receivedData).offset(headerSize as isize);
    while currentData
        < &mut *((*host).receivedData).offset((*host).receivedDataLength as isize)
            as *mut enet_uint8
    {
        let mut commandNumber: enet_uint8 = 0;
        let mut commandSize: size_t = 0;
        command = currentData as *mut ENetProtocol;
        if currentData
            .offset(::core::mem::size_of::<ENetProtocolCommandHeader>() as c_ulong as isize)
            > &mut *((*host).receivedData).offset((*host).receivedDataLength as isize)
                as *mut enet_uint8
        {
            break;
        }
        commandNumber = ((*command).header.command as c_int & ENET_PROTOCOL_COMMAND_MASK as c_int)
            as enet_uint8;
        if commandNumber as c_int >= ENET_PROTOCOL_COMMAND_COUNT as c_int {
            break;
        }
        commandSize = commandSizes[commandNumber as usize];
        if commandSize == 0 as c_int as size_t
            || currentData.offset(commandSize as isize)
                > &mut *((*host).receivedData).offset((*host).receivedDataLength as isize)
                    as *mut enet_uint8
        {
            break;
        }
        currentData = currentData.offset(commandSize as isize);
        if peer.is_null() && commandNumber as c_int != ENET_PROTOCOL_COMMAND_CONNECT as c_int {
            break;
        }
        (*command).header.reliableSequenceNumber = ntohs((*command).header.reliableSequenceNumber);
        match commandNumber as c_int {
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
        if !(!peer.is_null()
            && (*command).header.command as c_int & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as c_int
                != 0 as c_int)
        {
            continue;
        }
        let mut sentTime: enet_uint16 = 0;
        if flags as c_int & ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as c_int == 0 {
            break;
        }
        sentTime = ntohs((*header).sentTime);
        match (*peer).state as c_uint {
            7 | 2 | 0 | 9 => {}
            8 => {
                if (*command).header.command as c_int & ENET_PROTOCOL_COMMAND_MASK as c_int
                    == ENET_PROTOCOL_COMMAND_DISCONNECT as c_int
                {
                    enet_peer_queue_acknowledgement(peer, command, sentTime);
                }
            }
            _ => {
                enet_peer_queue_acknowledgement(peer, command, sentTime);
            }
        }
    }
    if !event.is_null() && (*event).type_0 as c_uint != ENET_EVENT_TYPE_NONE as c_int as c_uint {
        return 1 as c_int;
    }
    return 0 as c_int;
}
unsafe fn enet_protocol_receive_incoming_commands<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut event: *mut ENetEvent<S>,
) -> c_int {
    let mut packets: c_int = 0;
    let mut current_block_17: u64;
    packets = 0 as c_int;
    while packets < 256 as c_int {
        let mut receivedLength: c_int = 0;
        let mut buffer: ENetBuffer = ENetBuffer {
            data: 0 as *mut c_void,
            dataLength: 0,
        };
        buffer.data = ((*host).packetData[0 as c_int as usize]).as_mut_ptr() as *mut c_void;
        const MTU: usize = 4096;
        buffer.dataLength = ::core::mem::size_of::<[enet_uint8; MTU]>() as size_t;
        receivedLength = match (*host)
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
                        received_data.len() as size_t,
                    );
                    received_data.len() as c_int
                } else {
                    -2
                }
            }
            Ok(Some((_, PacketReceived::Partial))) => -2,
            Ok(None) => 0,
            Err(_) => -1,
        };
        if receivedLength == -2 as c_int {
            continue;
        }
        if receivedLength < 0 as c_int {
            return -(1 as c_int);
        }
        if receivedLength == 0 as c_int {
            return 0 as c_int;
        }
        (*host).receivedData = ((*host).packetData[0 as c_int as usize]).as_mut_ptr();
        (*host).receivedDataLength = receivedLength as size_t;
        (*host).totalReceivedData = ((*host).totalReceivedData as c_uint)
            .wrapping_add(receivedLength as c_uint)
            as enet_uint32 as enet_uint32;
        (*host).totalReceivedPackets = ((*host).totalReceivedPackets).wrapping_add(1);
        if ((*host).intercept).is_some() {
            match ((*host).intercept).expect("non-null function pointer")(host, event) {
                1 => {
                    current_block_17 = 11187707480244993007;
                    match current_block_17 {
                        15717549315443811277 => return -(1 as c_int),
                        _ => {
                            if !event.is_null()
                                && (*event).type_0 as c_uint
                                    != ENET_EVENT_TYPE_NONE as c_int as c_uint
                            {
                                return 1 as c_int;
                            }
                        }
                    }
                    current_block_17 = 11174649648027449784;
                }
                -1 => {
                    current_block_17 = 15717549315443811277;
                    match current_block_17 {
                        15717549315443811277 => return -(1 as c_int),
                        _ => {
                            if !event.is_null()
                                && (*event).type_0 as c_uint
                                    != ENET_EVENT_TYPE_NONE as c_int as c_uint
                            {
                                return 1 as c_int;
                            }
                        }
                    }
                    current_block_17 = 11174649648027449784;
                }
                _ => {
                    current_block_17 = 5143058163439228106;
                }
            }
        } else {
            current_block_17 = 5143058163439228106;
        }
        match current_block_17 {
            11174649648027449784 => {}
            _ => match enet_protocol_handle_incoming_commands(host, event) {
                1 => return 1 as c_int,
                -1 => return -(1 as c_int),
                _ => {}
            },
        }
        packets += 1;
    }
    return 0 as c_int;
}
unsafe fn enet_protocol_send_acknowledgements<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut peer: *mut ENetPeer<S>,
) {
    let mut command: *mut ENetProtocol = &mut *((*host).commands)
        .as_mut_ptr()
        .offset((*host).commandCount as isize)
        as *mut ENetProtocol;
    let mut buffer: *mut ENetBuffer = &mut *((*host).buffers)
        .as_mut_ptr()
        .offset((*host).bufferCount as isize)
        as *mut ENetBuffer;
    let mut acknowledgement: *mut ENetAcknowledgement = 0 as *mut ENetAcknowledgement;
    let mut currentAcknowledgement: ENetListIterator = 0 as *mut ENetListNode;
    let mut reliableSequenceNumber: enet_uint16 = 0;
    currentAcknowledgement = (*peer).acknowledgements.sentinel.next;
    while currentAcknowledgement != &mut (*peer).acknowledgements.sentinel as *mut ENetListNode {
        if command
            >= &mut *((*host).commands).as_mut_ptr().offset(
                (::core::mem::size_of::<[ENetProtocol; 32]>() as c_ulong)
                    .wrapping_div(::core::mem::size_of::<ENetProtocol>() as c_ulong)
                    as isize,
            ) as *mut ENetProtocol
            || buffer
                >= &mut *((*host).buffers).as_mut_ptr().offset(
                    (::core::mem::size_of::<[ENetBuffer; 65]>() as c_ulong)
                        .wrapping_div(::core::mem::size_of::<ENetBuffer>() as c_ulong)
                        as isize,
                ) as *mut ENetBuffer
            || ((*peer).mtu as size_t).wrapping_sub((*host).packetSize)
                < ::core::mem::size_of::<ENetProtocolAcknowledge>() as size_t
        {
            (*peer).flags =
                ((*peer).flags as c_int | ENET_PEER_FLAG_CONTINUE_SENDING as c_int) as enet_uint16;
            break;
        } else {
            acknowledgement = currentAcknowledgement as *mut ENetAcknowledgement;
            currentAcknowledgement = (*currentAcknowledgement).next;
            (*buffer).data = command as *mut c_void;
            (*buffer).dataLength = ::core::mem::size_of::<ENetProtocolAcknowledge>() as size_t;
            (*host).packetSize = ((*host).packetSize as size_t).wrapping_add((*buffer).dataLength)
                as size_t as size_t;
            reliableSequenceNumber =
                htons((*acknowledgement).command.header.reliableSequenceNumber);
            (*command).header.command = ENET_PROTOCOL_COMMAND_ACKNOWLEDGE as c_int as enet_uint8;
            (*command).header.channelID = (*acknowledgement).command.header.channelID;
            (*command).header.reliableSequenceNumber = reliableSequenceNumber;
            (*command).acknowledge.receivedReliableSequenceNumber = reliableSequenceNumber;
            (*command).acknowledge.receivedSentTime =
                htons((*acknowledgement).sentTime as uint16_t);
            if (*acknowledgement).command.header.command as c_int
                & ENET_PROTOCOL_COMMAND_MASK as c_int
                == ENET_PROTOCOL_COMMAND_DISCONNECT as c_int
            {
                enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
            }
            enet_list_remove(&mut (*acknowledgement).acknowledgementList);
            enet_free(acknowledgement as *mut c_void);
            command = command.offset(1);
            buffer = buffer.offset(1);
        }
    }
    (*host).commandCount = command.offset_from(((*host).commands).as_mut_ptr()) as c_long as size_t;
    (*host).bufferCount = buffer.offset_from(((*host).buffers).as_mut_ptr()) as c_long as size_t;
}
unsafe fn enet_protocol_check_timeouts<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut peer: *mut ENetPeer<S>,
    mut event: *mut ENetEvent<S>,
) -> c_int {
    let mut outgoingCommand: *mut ENetOutgoingCommand = 0 as *mut ENetOutgoingCommand;
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut insertPosition: ENetListIterator = 0 as *mut ENetListNode;
    let mut insertSendReliablePosition: ENetListIterator = 0 as *mut ENetListNode;
    currentCommand = (*peer).sentReliableCommands.sentinel.next;
    insertPosition = (*peer).outgoingCommands.sentinel.next;
    insertSendReliablePosition = (*peer).outgoingSendReliableCommands.sentinel.next;
    while currentCommand != &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode {
        outgoingCommand = currentCommand as *mut ENetOutgoingCommand;
        currentCommand = (*currentCommand).next;
        if (if ((*host).serviceTime).wrapping_sub((*outgoingCommand).sentTime)
            >= 86400000 as c_int as c_uint
        {
            ((*outgoingCommand).sentTime).wrapping_sub((*host).serviceTime)
        } else {
            ((*host).serviceTime).wrapping_sub((*outgoingCommand).sentTime)
        }) < (*outgoingCommand).roundTripTimeout
        {
            continue;
        }
        if (*peer).earliestTimeout == 0 as c_int as c_uint
            || ((*outgoingCommand).sentTime).wrapping_sub((*peer).earliestTimeout)
                >= 86400000 as c_int as c_uint
        {
            (*peer).earliestTimeout = (*outgoingCommand).sentTime;
        }
        if (*peer).earliestTimeout != 0 as c_int as c_uint
            && ((if ((*host).serviceTime).wrapping_sub((*peer).earliestTimeout)
                >= 86400000 as c_int as c_uint
            {
                ((*peer).earliestTimeout).wrapping_sub((*host).serviceTime)
            } else {
                ((*host).serviceTime).wrapping_sub((*peer).earliestTimeout)
            }) >= (*peer).timeoutMaximum
                || ((1 as c_int) << (*outgoingCommand).sendAttempts as c_int - 1 as c_int)
                    as c_uint
                    >= (*peer).timeoutLimit
                    && (if ((*host).serviceTime).wrapping_sub((*peer).earliestTimeout)
                        >= 86400000 as c_int as c_uint
                    {
                        ((*peer).earliestTimeout).wrapping_sub((*host).serviceTime)
                    } else {
                        ((*host).serviceTime).wrapping_sub((*peer).earliestTimeout)
                    }) >= (*peer).timeoutMinimum)
        {
            enet_protocol_notify_disconnect(host, peer, event);
            return 1 as c_int;
        }
        (*peer).packetsLost = ((*peer).packetsLost).wrapping_add(1);
        (*outgoingCommand).roundTripTimeout = ((*outgoingCommand).roundTripTimeout as c_uint)
            .wrapping_mul(2 as c_int as c_uint)
            as enet_uint32 as enet_uint32;
        if !((*outgoingCommand).packet).is_null() {
            (*peer).reliableDataInTransit = ((*peer).reliableDataInTransit as c_uint)
                .wrapping_sub((*outgoingCommand).fragmentLength as c_uint)
                as enet_uint32 as enet_uint32;
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
            && !((*peer).sentReliableCommands.sentinel.next
                == &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode)
        {
            outgoingCommand = currentCommand as *mut ENetOutgoingCommand;
            (*peer).nextTimeout =
                ((*outgoingCommand).sentTime).wrapping_add((*outgoingCommand).roundTripTimeout);
        }
    }
    return 0 as c_int;
}
unsafe fn enet_protocol_check_outgoing_commands<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut peer: *mut ENetPeer<S>,
    mut sentUnreliableCommands: *mut ENetList,
) -> c_int {
    let mut command: *mut ENetProtocol = &mut *((*host).commands)
        .as_mut_ptr()
        .offset((*host).commandCount as isize)
        as *mut ENetProtocol;
    let mut buffer: *mut ENetBuffer = &mut *((*host).buffers)
        .as_mut_ptr()
        .offset((*host).bufferCount as isize)
        as *mut ENetBuffer;
    let mut outgoingCommand: *mut ENetOutgoingCommand = 0 as *mut ENetOutgoingCommand;
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut currentSendReliableCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut channel: *mut ENetChannel = 0 as *mut ENetChannel;
    let mut reliableWindow: enet_uint16 = 0 as c_int as enet_uint16;
    let mut commandSize: size_t = 0;
    let mut windowWrap: c_int = 0 as c_int;
    let mut canPing: c_int = 1 as c_int;
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
                    >= 86400000 as c_int as c_uint
            {
                current_block_55 = 13678975718891345113;
            } else {
                currentCommand = (*currentCommand).next;
                current_block_55 = 1856101646708284338;
            }
        } else {
            if !(currentSendReliableCommand
                != &mut (*peer).outgoingSendReliableCommands.sentinel as *mut ENetListNode)
            {
                break;
            }
            current_block_55 = 13678975718891345113;
        }
        match current_block_55 {
            13678975718891345113 => {
                outgoingCommand = currentSendReliableCommand as *mut ENetOutgoingCommand;
                currentSendReliableCommand = (*currentSendReliableCommand).next;
            }
            _ => {}
        }
        if (*outgoingCommand).command.header.command as c_int
            & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as c_int
            != 0
        {
            channel =
                if ((*outgoingCommand).command.header.channelID as size_t) < (*peer).channelCount {
                    &mut *((*peer).channels)
                        .offset((*outgoingCommand).command.header.channelID as isize)
                        as *mut ENetChannel
                } else {
                    0 as *mut ENetChannel
                };
            reliableWindow = ((*outgoingCommand).reliableSequenceNumber as c_int
                / ENET_PEER_RELIABLE_WINDOW_SIZE as c_int)
                as enet_uint16;
            if !channel.is_null() {
                if windowWrap != 0 {
                    continue;
                }
                if ((*outgoingCommand).sendAttempts as c_int) < 1 as c_int
                    && (*outgoingCommand).reliableSequenceNumber as c_int
                        % ENET_PEER_RELIABLE_WINDOW_SIZE as c_int
                        == 0
                    && ((*channel).reliableWindows[((reliableWindow as c_int
                        + ENET_PEER_RELIABLE_WINDOWS as c_int
                        - 1 as c_int)
                        % ENET_PEER_RELIABLE_WINDOWS as c_int)
                        as usize] as c_int
                        >= ENET_PEER_RELIABLE_WINDOW_SIZE as c_int
                        || (*channel).usedReliableWindows as c_int
                            & ((((1 as c_int)
                                << ENET_PEER_FREE_RELIABLE_WINDOWS as c_int + 2 as c_int)
                                - 1 as c_int)
                                << reliableWindow as c_int
                                | ((1 as c_int)
                                    << ENET_PEER_FREE_RELIABLE_WINDOWS as c_int + 2 as c_int)
                                    - 1 as c_int
                                    >> ENET_PEER_RELIABLE_WINDOWS as c_int
                                        - reliableWindow as c_int)
                            != 0)
                {
                    windowWrap = 1 as c_int;
                    currentSendReliableCommand = &mut (*peer).outgoingSendReliableCommands.sentinel;
                    continue;
                }
            }
            if !((*outgoingCommand).packet).is_null() {
                let mut windowSize: enet_uint32 = ((*peer).packetThrottle)
                    .wrapping_mul((*peer).windowSize)
                    .wrapping_div(ENET_PEER_PACKET_THROTTLE_SCALE as c_int as c_uint);
                if ((*peer).reliableDataInTransit)
                    .wrapping_add((*outgoingCommand).fragmentLength as c_uint)
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
            canPing = 0 as c_int;
        }
        commandSize = commandSizes[((*outgoingCommand).command.header.command as c_int
            & ENET_PROTOCOL_COMMAND_MASK as c_int) as usize];
        if command
            >= &mut *((*host).commands).as_mut_ptr().offset(
                (::core::mem::size_of::<[ENetProtocol; 32]>() as c_ulong)
                    .wrapping_div(::core::mem::size_of::<ENetProtocol>() as c_ulong)
                    as isize,
            ) as *mut ENetProtocol
            || buffer.offset(1 as c_int as isize)
                >= &mut *((*host).buffers).as_mut_ptr().offset(
                    (::core::mem::size_of::<[ENetBuffer; 65]>() as c_ulong)
                        .wrapping_div(::core::mem::size_of::<ENetBuffer>() as c_ulong)
                        as isize,
                ) as *mut ENetBuffer
            || ((*peer).mtu as size_t).wrapping_sub((*host).packetSize) < commandSize
            || !((*outgoingCommand).packet).is_null()
                && (((*peer).mtu as size_t).wrapping_sub((*host).packetSize) as enet_uint16
                    as c_int)
                    < commandSize.wrapping_add((*outgoingCommand).fragmentLength as size_t)
                        as enet_uint16 as c_int
        {
            (*peer).flags =
                ((*peer).flags as c_int | ENET_PEER_FLAG_CONTINUE_SENDING as c_int) as enet_uint16;
            break;
        } else {
            if (*outgoingCommand).command.header.command as c_int
                & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as c_int
                != 0
            {
                if !channel.is_null() && ((*outgoingCommand).sendAttempts as c_int) < 1 as c_int {
                    (*channel).usedReliableWindows = ((*channel).usedReliableWindows as c_int
                        | (1 as c_int) << reliableWindow as c_int)
                        as enet_uint16;
                    (*channel).reliableWindows[reliableWindow as usize] =
                        ((*channel).reliableWindows[reliableWindow as usize]).wrapping_add(1);
                }
                (*outgoingCommand).sendAttempts = ((*outgoingCommand).sendAttempts).wrapping_add(1);
                if (*outgoingCommand).roundTripTimeout == 0 as c_int as c_uint {
                    (*outgoingCommand).roundTripTimeout = ((*peer).roundTripTime).wrapping_add(
                        (4 as c_int as c_uint).wrapping_mul((*peer).roundTripTimeVariance),
                    );
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
                (*host).headerFlags = ((*host).headerFlags as c_int
                    | ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as c_int)
                    as enet_uint16;
                (*peer).reliableDataInTransit = ((*peer).reliableDataInTransit as c_uint)
                    .wrapping_add((*outgoingCommand).fragmentLength as c_uint)
                    as enet_uint32 as enet_uint32;
            } else {
                if !((*outgoingCommand).packet).is_null()
                    && (*outgoingCommand).fragmentOffset == 0 as c_int as c_uint
                {
                    (*peer).packetThrottleCounter = ((*peer).packetThrottleCounter as c_uint)
                        .wrapping_add(ENET_PEER_PACKET_THROTTLE_COUNTER as c_int as c_uint)
                        as enet_uint32
                        as enet_uint32;
                    (*peer).packetThrottleCounter = ((*peer).packetThrottleCounter as c_uint)
                        .wrapping_rem(ENET_PEER_PACKET_THROTTLE_SCALE as c_int as c_uint)
                        as enet_uint32
                        as enet_uint32;
                    if (*peer).packetThrottleCounter > (*peer).packetThrottle {
                        let mut reliableSequenceNumber: enet_uint16 =
                            (*outgoingCommand).reliableSequenceNumber;
                        let mut unreliableSequenceNumber: enet_uint16 =
                            (*outgoingCommand).unreliableSequenceNumber;
                        loop {
                            (*(*outgoingCommand).packet).referenceCount =
                                ((*(*outgoingCommand).packet).referenceCount).wrapping_sub(1);
                            if (*(*outgoingCommand).packet).referenceCount == 0 as c_int as size_t {
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
                            if (*outgoingCommand).reliableSequenceNumber as c_int
                                != reliableSequenceNumber as c_int
                                || (*outgoingCommand).unreliableSequenceNumber as c_int
                                    != unreliableSequenceNumber as c_int
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
            (*host).packetSize = ((*host).packetSize as size_t).wrapping_add((*buffer).dataLength)
                as size_t as size_t;
            *command = (*outgoingCommand).command;
            if !((*outgoingCommand).packet).is_null() {
                buffer = buffer.offset(1);
                (*buffer).data = ((*(*outgoingCommand).packet).data)
                    .offset((*outgoingCommand).fragmentOffset as isize)
                    as *mut c_void;
                (*buffer).dataLength = (*outgoingCommand).fragmentLength as size_t;
                (*host).packetSize = ((*host).packetSize as c_ulong)
                    .wrapping_add((*outgoingCommand).fragmentLength as c_ulong)
                    as size_t as size_t;
            } else if (*outgoingCommand).command.header.command as c_int
                & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as c_int
                == 0
            {
                enet_free(outgoingCommand as *mut c_void);
            }
            (*peer).packetsSent = ((*peer).packetsSent).wrapping_add(1);
            command = command.offset(1);
            buffer = buffer.offset(1);
        }
    }
    (*host).commandCount = command.offset_from(((*host).commands).as_mut_ptr()) as c_long as size_t;
    (*host).bufferCount = buffer.offset_from(((*host).buffers).as_mut_ptr()) as c_long as size_t;
    if (*peer).state as c_uint == ENET_PEER_STATE_DISCONNECT_LATER as c_int as c_uint
        && enet_peer_has_outgoing_commands(peer) == 0
        && (*sentUnreliableCommands).sentinel.next
            == &mut (*sentUnreliableCommands).sentinel as *mut ENetListNode
    {
        enet_peer_disconnect(peer, (*peer).eventData);
    }
    return canPing;
}
unsafe fn enet_protocol_send_outgoing_commands<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut event: *mut ENetEvent<S>,
    mut checkForTimeouts: c_int,
) -> c_int {
    let mut headerData: [enet_uint8; 8] = [0; 8];
    let mut header: *mut ENetProtocolHeader = headerData.as_mut_ptr() as *mut ENetProtocolHeader;
    let mut sentLength: c_int = 0 as c_int;
    let mut shouldCompress: size_t = 0 as c_int as size_t;
    let mut sentUnreliableCommands: ENetList = ENetList {
        sentinel: ENetListNode {
            next: 0 as *mut _ENetListNode,
            previous: 0 as *mut _ENetListNode,
        },
    };
    enet_list_clear(&mut sentUnreliableCommands);
    let mut sendPass: c_int = 0 as c_int;
    let mut continueSending: c_int = 0 as c_int;
    while sendPass <= continueSending {
        let mut currentPeer: *mut ENetPeer<S> = (*host).peers;
        while currentPeer
            < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer<S>
        {
            if !((*currentPeer).state as c_uint == ENET_PEER_STATE_DISCONNECTED as c_int as c_uint
                || (*currentPeer).state as c_uint == ENET_PEER_STATE_ZOMBIE as c_int as c_uint
                || sendPass > 0 as c_int
                    && (*currentPeer).flags as c_int & ENET_PEER_FLAG_CONTINUE_SENDING as c_int
                        == 0)
            {
                (*currentPeer).flags = ((*currentPeer).flags as c_int
                    & !(ENET_PEER_FLAG_CONTINUE_SENDING as c_int))
                    as enet_uint16;
                (*host).headerFlags = 0 as c_int as enet_uint16;
                (*host).commandCount = 0 as c_int as size_t;
                (*host).bufferCount = 1 as c_int as size_t;
                (*host).packetSize = ::core::mem::size_of::<ENetProtocolHeader>() as size_t;
                if !((*currentPeer).acknowledgements.sentinel.next
                    == &mut (*currentPeer).acknowledgements.sentinel as *mut ENetListNode)
                {
                    enet_protocol_send_acknowledgements(host, currentPeer);
                }
                if checkForTimeouts != 0 as c_int
                    && !((*currentPeer).sentReliableCommands.sentinel.next
                        == &mut (*currentPeer).sentReliableCommands.sentinel as *mut ENetListNode)
                    && !(((*host).serviceTime).wrapping_sub((*currentPeer).nextTimeout)
                        >= 86400000 as c_int as c_uint)
                    && enet_protocol_check_timeouts(host, currentPeer, event) == 1 as c_int
                {
                    if !event.is_null()
                        && (*event).type_0 as c_uint != ENET_EVENT_TYPE_NONE as c_int as c_uint
                    {
                        return 1 as c_int;
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
                            >= 86400000 as c_int as c_uint
                        {
                            ((*currentPeer).lastReceiveTime).wrapping_sub((*host).serviceTime)
                        } else {
                            ((*host).serviceTime).wrapping_sub((*currentPeer).lastReceiveTime)
                        }) >= (*currentPeer).pingInterval
                        && ((*currentPeer).mtu as size_t).wrapping_sub((*host).packetSize)
                            >= ::core::mem::size_of::<ENetProtocolPing>() as size_t
                    {
                        enet_peer_ping(currentPeer);
                        enet_protocol_check_outgoing_commands(
                            host,
                            currentPeer,
                            &mut sentUnreliableCommands,
                        );
                    }
                    if !((*host).commandCount == 0 as c_int as size_t) {
                        if (*currentPeer).packetLossEpoch == 0 as c_int as c_uint {
                            (*currentPeer).packetLossEpoch = (*host).serviceTime;
                        } else if (if ((*host).serviceTime)
                            .wrapping_sub((*currentPeer).packetLossEpoch)
                            >= 86400000 as c_int as c_uint
                        {
                            ((*currentPeer).packetLossEpoch).wrapping_sub((*host).serviceTime)
                        } else {
                            ((*host).serviceTime).wrapping_sub((*currentPeer).packetLossEpoch)
                        }) >= ENET_PEER_PACKET_LOSS_INTERVAL as c_int as c_uint
                            && (*currentPeer).packetsSent > 0 as c_int as c_uint
                        {
                            let mut packetLoss: enet_uint32 = ((*currentPeer).packetsLost)
                                .wrapping_mul(ENET_PEER_PACKET_LOSS_SCALE as c_int as c_uint)
                                .wrapping_div((*currentPeer).packetsSent);
                            (*currentPeer).packetLossVariance = ((*currentPeer).packetLossVariance)
                                .wrapping_mul(3 as c_int as c_uint)
                                .wrapping_add(if packetLoss < (*currentPeer).packetLoss {
                                    ((*currentPeer).packetLoss).wrapping_sub(packetLoss)
                                } else {
                                    packetLoss.wrapping_sub((*currentPeer).packetLoss)
                                })
                                .wrapping_div(4 as c_int as c_uint);
                            (*currentPeer).packetLoss = ((*currentPeer).packetLoss)
                                .wrapping_mul(7 as c_int as c_uint)
                                .wrapping_add(packetLoss)
                                .wrapping_div(8 as c_int as c_uint);
                            (*currentPeer).packetLossEpoch = (*host).serviceTime;
                            (*currentPeer).packetsSent = 0 as c_int as enet_uint32;
                            (*currentPeer).packetsLost = 0 as c_int as enet_uint32;
                        }
                        let ref mut fresh34 = (*((*host).buffers).as_mut_ptr()).data;
                        *fresh34 = headerData.as_mut_ptr() as *mut c_void;
                        if (*host).headerFlags as c_int
                            & ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as c_int
                            != 0
                        {
                            (*header).sentTime = htons(
                                ((*host).serviceTime & 0xffff as c_int as c_uint) as uint16_t,
                            );
                            (*((*host).buffers).as_mut_ptr()).dataLength =
                                ::core::mem::size_of::<ENetProtocolHeader>() as size_t;
                        } else {
                            (*((*host).buffers).as_mut_ptr()).dataLength = 2 as size_t;
                        }
                        shouldCompress = 0 as c_int as size_t;
                        if !((*host).compressor.context).is_null()
                            && ((*host).compressor.compress).is_some()
                        {
                            let mut originalSize: size_t =
                                ((*host).packetSize).wrapping_sub(::core::mem::size_of::<
                                    ENetProtocolHeader,
                                >(
                                )
                                    as size_t);
                            let mut compressedSize: size_t = ((*host).compressor.compress)
                                .expect("non-null function pointer")(
                                (*host).compressor.context,
                                &mut *((*host).buffers).as_mut_ptr().offset(1 as c_int as isize),
                                ((*host).bufferCount).wrapping_sub(1 as c_int as size_t),
                                originalSize,
                                ((*host).packetData[1 as c_int as usize]).as_mut_ptr(),
                                originalSize,
                            );
                            if compressedSize > 0 as c_int as size_t
                                && compressedSize < originalSize
                            {
                                (*host).headerFlags = ((*host).headerFlags as c_int
                                    | ENET_PROTOCOL_HEADER_FLAG_COMPRESSED as c_int)
                                    as enet_uint16;
                                shouldCompress = compressedSize;
                            }
                        }
                        if ((*currentPeer).outgoingPeerID as c_int)
                            < ENET_PROTOCOL_MAXIMUM_PEER_ID as c_int
                        {
                            (*host).headerFlags = ((*host).headerFlags as c_int
                                | ((*currentPeer).outgoingSessionID as c_int)
                                    << ENET_PROTOCOL_HEADER_SESSION_SHIFT as c_int)
                                as enet_uint16;
                        }
                        (*header).peerID = htons(
                            ((*currentPeer).outgoingPeerID as c_int | (*host).headerFlags as c_int)
                                as uint16_t,
                        );
                        if ((*host).checksum).is_some() {
                            let mut checksum: *mut enet_uint32 = &mut *headerData
                                .as_mut_ptr()
                                .offset((*((*host).buffers).as_mut_ptr()).dataLength as isize)
                                as *mut enet_uint8
                                as *mut enet_uint32;
                            *checksum = if ((*currentPeer).outgoingPeerID as c_int)
                                < ENET_PROTOCOL_MAXIMUM_PEER_ID as c_int
                            {
                                (*currentPeer).connectID
                            } else {
                                0 as c_int as c_uint
                            };
                            let ref mut fresh35 = (*((*host).buffers).as_mut_ptr()).dataLength;
                            *fresh35 =
                                (*fresh35 as c_ulong)
                                    .wrapping_add(::core::mem::size_of::<enet_uint32>() as c_ulong)
                                    as size_t as size_t;
                            *checksum = ((*host).checksum).expect("non-null function pointer")(
                                ((*host).buffers).as_mut_ptr(),
                                (*host).bufferCount,
                            );
                        }
                        if shouldCompress > 0 as c_int as size_t {
                            (*host).buffers[1 as c_int as usize].data =
                                ((*host).packetData[1 as c_int as usize]).as_mut_ptr()
                                    as *mut c_void;
                            (*host).buffers[1 as c_int as usize].dataLength = shouldCompress;
                            (*host).bufferCount = 2 as c_int as size_t;
                        }
                        (*currentPeer).lastSendTime = (*host).serviceTime;
                        let mut conglomerate_buffer = vec![];
                        for buffer_index in 0..(*host).bufferCount {
                            let buffer = &(*host).buffers[buffer_index as usize];
                            conglomerate_buffer.extend_from_slice(std::slice::from_raw_parts(
                                buffer.data as *mut u8,
                                buffer.dataLength as usize,
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
                            Ok(sent) => sent as c_int,
                            Err(_) => -1,
                        };
                        enet_protocol_remove_sent_unreliable_commands(
                            currentPeer,
                            &mut sentUnreliableCommands,
                        );
                        if sentLength < 0 as c_int {
                            return -(1 as c_int);
                        }
                        (*host).totalSentData =
                            ((*host).totalSentData as c_uint).wrapping_add(sentLength as c_uint)
                                as enet_uint32 as enet_uint32;
                        (*host).totalSentPackets = ((*host).totalSentPackets).wrapping_add(1);
                    }
                }
                if (*currentPeer).flags as c_int & ENET_PEER_FLAG_CONTINUE_SENDING as c_int != 0 {
                    continueSending = sendPass + 1 as c_int;
                }
            }
            currentPeer = currentPeer.offset(1);
        }
        sendPass += 1;
    }
    return 0 as c_int;
}
pub(crate) unsafe fn enet_host_flush<S: Socket>(mut host: *mut ENetHost<S>) {
    (*host).serviceTime = enet_time_get();
    enet_protocol_send_outgoing_commands(host, 0 as *mut ENetEvent<S>, 0 as c_int);
}
pub(crate) unsafe fn enet_host_check_events<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut event: *mut ENetEvent<S>,
) -> c_int {
    if event.is_null() {
        return -(1 as c_int);
    }
    (*event).type_0 = ENET_EVENT_TYPE_NONE;
    (*event).peer = 0 as *mut ENetPeer<S>;
    (*event).packet = 0 as *mut ENetPacket;
    return enet_protocol_dispatch_incoming_commands(host, event);
}
pub(crate) unsafe fn enet_host_service<S: Socket>(
    mut host: *mut ENetHost<S>,
    mut event: *mut ENetEvent<S>,
) -> c_int {
    if !event.is_null() {
        (*event).type_0 = ENET_EVENT_TYPE_NONE;
        (*event).peer = 0 as *mut ENetPeer<S>;
        (*event).packet = 0 as *mut ENetPacket;
        match enet_protocol_dispatch_incoming_commands(host, event) {
            1 => return 1 as c_int,
            -1 => return -(1 as c_int),
            _ => {}
        }
    }
    (*host).serviceTime = enet_time_get();
    if (if ((*host).serviceTime).wrapping_sub((*host).bandwidthThrottleEpoch)
        >= 86400000 as c_int as c_uint
    {
        ((*host).bandwidthThrottleEpoch).wrapping_sub((*host).serviceTime)
    } else {
        ((*host).serviceTime).wrapping_sub((*host).bandwidthThrottleEpoch)
    }) >= ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL as c_int as c_uint
    {
        enet_host_bandwidth_throttle(host);
    }
    match enet_protocol_send_outgoing_commands(host, event, 1 as c_int) {
        1 => return 1 as c_int,
        -1 => return -(1 as c_int),
        _ => {}
    }
    match enet_protocol_receive_incoming_commands(host, event) {
        1 => return 1 as c_int,
        -1 => return -(1 as c_int),
        _ => {}
    }
    match enet_protocol_send_outgoing_commands(host, event, 1 as c_int) {
        1 => return 1 as c_int,
        -1 => return -(1 as c_int),
        _ => {}
    }
    if !event.is_null() {
        match enet_protocol_dispatch_incoming_commands(host, event) {
            1 => return 1 as c_int,
            -1 => return -(1 as c_int),
            _ => {}
        }
    }
    return 0 as c_int;
}
pub(crate) unsafe fn enet_host_random_seed() -> enet_uint32 {
    enet_time_get()
}
pub(crate) unsafe fn enet_time_get() -> enet_uint32 {
    (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        % u32::MAX as u128) as enet_uint32
}
