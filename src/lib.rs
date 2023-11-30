#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn socket(__domain: libc::c_int, __type: libc::c_int, __protocol: libc::c_int) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn getsockname(__fd: libc::c_int, __addr: *mut sockaddr, __len: *mut socklen_t) -> libc::c_int;
    fn connect(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn sendmsg(__fd: libc::c_int, __message: *const msghdr, __flags: libc::c_int) -> ssize_t;
    fn recvmsg(__fd: libc::c_int, __message: *mut msghdr, __flags: libc::c_int) -> ssize_t;
    fn getsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *mut libc::c_void,
        __optlen: *mut socklen_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(__fd: libc::c_int, __addr: *mut sockaddr, __addr_len: *mut socklen_t) -> libc::c_int;
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    fn ntohl(__netlong: uint32_t) -> uint32_t;
    fn ntohs(__netshort: uint16_t) -> uint16_t;
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn getnameinfo(
        __sa: *const sockaddr,
        __salen: socklen_t,
        __host: *mut libc::c_char,
        __hostlen: socklen_t,
        __serv: *mut libc::c_char,
        __servlen: socklen_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type C2RustUnnamed = libc::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
pub const MSG_BATCH: C2RustUnnamed = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut libc::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut libc::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: libc::c_int,
}
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_0 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
pub type ENetSocket = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENetBuffer {
    pub data: *mut libc::c_void,
    pub dataLength: size_t,
}
pub type ENetSocketSet = fd_set;
pub type enet_uint8 = libc::c_uchar;
pub type enet_uint16 = libc::c_ushort;
pub type enet_uint32 = libc::c_uint;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT: C2RustUnnamed_1 = 1048576;
pub const ENET_PROTOCOL_MAXIMUM_PEER_ID: C2RustUnnamed_1 = 4095;
pub const ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT: C2RustUnnamed_1 = 255;
pub const ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT: C2RustUnnamed_1 = 1;
pub const ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE: C2RustUnnamed_1 = 65536;
pub const ENET_PROTOCOL_MINIMUM_WINDOW_SIZE: C2RustUnnamed_1 = 4096;
pub const ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS: C2RustUnnamed_1 = 32;
pub const ENET_PROTOCOL_MAXIMUM_MTU: C2RustUnnamed_1 = 4096;
pub const ENET_PROTOCOL_MINIMUM_MTU: C2RustUnnamed_1 = 576;
pub type _ENetProtocolCommand = libc::c_uint;
pub const ENET_PROTOCOL_COMMAND_MASK: _ENetProtocolCommand = 15;
pub const ENET_PROTOCOL_COMMAND_COUNT: _ENetProtocolCommand = 13;
pub const ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT: _ENetProtocolCommand = 12;
pub const ENET_PROTOCOL_COMMAND_THROTTLE_CONFIGURE: _ENetProtocolCommand = 11;
pub const ENET_PROTOCOL_COMMAND_BANDWIDTH_LIMIT: _ENetProtocolCommand = 10;
pub const ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED: _ENetProtocolCommand = 9;
pub const ENET_PROTOCOL_COMMAND_SEND_FRAGMENT: _ENetProtocolCommand = 8;
pub const ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE: _ENetProtocolCommand = 7;
pub const ENET_PROTOCOL_COMMAND_SEND_RELIABLE: _ENetProtocolCommand = 6;
pub const ENET_PROTOCOL_COMMAND_PING: _ENetProtocolCommand = 5;
pub const ENET_PROTOCOL_COMMAND_DISCONNECT: _ENetProtocolCommand = 4;
pub const ENET_PROTOCOL_COMMAND_VERIFY_CONNECT: _ENetProtocolCommand = 3;
pub const ENET_PROTOCOL_COMMAND_CONNECT: _ENetProtocolCommand = 2;
pub const ENET_PROTOCOL_COMMAND_ACKNOWLEDGE: _ENetProtocolCommand = 1;
pub const ENET_PROTOCOL_COMMAND_NONE: _ENetProtocolCommand = 0;
pub type ENetProtocolCommand = _ENetProtocolCommand;
pub type _ENetProtocolFlag = libc::c_uint;
pub const ENET_PROTOCOL_HEADER_SESSION_SHIFT: _ENetProtocolFlag = 12;
pub const ENET_PROTOCOL_HEADER_SESSION_MASK: _ENetProtocolFlag = 12288;
pub const ENET_PROTOCOL_HEADER_FLAG_MASK: _ENetProtocolFlag = 49152;
pub const ENET_PROTOCOL_HEADER_FLAG_SENT_TIME: _ENetProtocolFlag = 32768;
pub const ENET_PROTOCOL_HEADER_FLAG_COMPRESSED: _ENetProtocolFlag = 16384;
pub const ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED: _ENetProtocolFlag = 64;
pub const ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE: _ENetProtocolFlag = 128;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct _ENetProtocolHeader {
    pub peerID: enet_uint16,
    pub sentTime: enet_uint16,
}
pub type ENetProtocolHeader = _ENetProtocolHeader;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct _ENetProtocolCommandHeader {
    pub command: enet_uint8,
    pub channelID: enet_uint8,
    pub reliableSequenceNumber: enet_uint16,
}
pub type ENetProtocolCommandHeader = _ENetProtocolCommandHeader;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct _ENetProtocolAcknowledge {
    pub header: ENetProtocolCommandHeader,
    pub receivedReliableSequenceNumber: enet_uint16,
    pub receivedSentTime: enet_uint16,
}
pub type ENetProtocolAcknowledge = _ENetProtocolAcknowledge;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct _ENetProtocolConnect {
    pub header: ENetProtocolCommandHeader,
    pub outgoingPeerID: enet_uint16,
    pub incomingSessionID: enet_uint8,
    pub outgoingSessionID: enet_uint8,
    pub mtu: enet_uint32,
    pub windowSize: enet_uint32,
    pub channelCount: enet_uint32,
    pub incomingBandwidth: enet_uint32,
    pub outgoingBandwidth: enet_uint32,
    pub packetThrottleInterval: enet_uint32,
    pub packetThrottleAcceleration: enet_uint32,
    pub packetThrottleDeceleration: enet_uint32,
    pub connectID: enet_uint32,
    pub data: enet_uint32,
}
pub type ENetProtocolConnect = _ENetProtocolConnect;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct _ENetProtocolVerifyConnect {
    pub header: ENetProtocolCommandHeader,
    pub outgoingPeerID: enet_uint16,
    pub incomingSessionID: enet_uint8,
    pub outgoingSessionID: enet_uint8,
    pub mtu: enet_uint32,
    pub windowSize: enet_uint32,
    pub channelCount: enet_uint32,
    pub incomingBandwidth: enet_uint32,
    pub outgoingBandwidth: enet_uint32,
    pub packetThrottleInterval: enet_uint32,
    pub packetThrottleAcceleration: enet_uint32,
    pub packetThrottleDeceleration: enet_uint32,
    pub connectID: enet_uint32,
}
pub type ENetProtocolVerifyConnect = _ENetProtocolVerifyConnect;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct _ENetProtocolBandwidthLimit {
    pub header: ENetProtocolCommandHeader,
    pub incomingBandwidth: enet_uint32,
    pub outgoingBandwidth: enet_uint32,
}
pub type ENetProtocolBandwidthLimit = _ENetProtocolBandwidthLimit;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct _ENetProtocolThrottleConfigure {
    pub header: ENetProtocolCommandHeader,
    pub packetThrottleInterval: enet_uint32,
    pub packetThrottleAcceleration: enet_uint32,
    pub packetThrottleDeceleration: enet_uint32,
}
pub type ENetProtocolThrottleConfigure = _ENetProtocolThrottleConfigure;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct _ENetProtocolDisconnect {
    pub header: ENetProtocolCommandHeader,
    pub data: enet_uint32,
}
pub type ENetProtocolDisconnect = _ENetProtocolDisconnect;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct _ENetProtocolPing {
    pub header: ENetProtocolCommandHeader,
}
pub type ENetProtocolPing = _ENetProtocolPing;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct _ENetProtocolSendReliable {
    pub header: ENetProtocolCommandHeader,
    pub dataLength: enet_uint16,
}
pub type ENetProtocolSendReliable = _ENetProtocolSendReliable;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct _ENetProtocolSendUnreliable {
    pub header: ENetProtocolCommandHeader,
    pub unreliableSequenceNumber: enet_uint16,
    pub dataLength: enet_uint16,
}
pub type ENetProtocolSendUnreliable = _ENetProtocolSendUnreliable;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct _ENetProtocolSendUnsequenced {
    pub header: ENetProtocolCommandHeader,
    pub unsequencedGroup: enet_uint16,
    pub dataLength: enet_uint16,
}
pub type ENetProtocolSendUnsequenced = _ENetProtocolSendUnsequenced;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct _ENetProtocolSendFragment {
    pub header: ENetProtocolCommandHeader,
    pub startSequenceNumber: enet_uint16,
    pub dataLength: enet_uint16,
    pub fragmentCount: enet_uint32,
    pub fragmentNumber: enet_uint32,
    pub totalLength: enet_uint32,
    pub fragmentOffset: enet_uint32,
}
pub type ENetProtocolSendFragment = _ENetProtocolSendFragment;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union _ENetProtocol {
    pub header: ENetProtocolCommandHeader,
    pub acknowledge: ENetProtocolAcknowledge,
    pub connect: ENetProtocolConnect,
    pub verifyConnect: ENetProtocolVerifyConnect,
    pub disconnect: ENetProtocolDisconnect,
    pub ping: ENetProtocolPing,
    pub sendReliable: ENetProtocolSendReliable,
    pub sendUnreliable: ENetProtocolSendUnreliable,
    pub sendUnsequenced: ENetProtocolSendUnsequenced,
    pub sendFragment: ENetProtocolSendFragment,
    pub bandwidthLimit: ENetProtocolBandwidthLimit,
    pub throttleConfigure: ENetProtocolThrottleConfigure,
}
pub type ENetProtocol = _ENetProtocol;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ENetListNode {
    pub next: *mut _ENetListNode,
    pub previous: *mut _ENetListNode,
}
pub type ENetListNode = _ENetListNode;
pub type ENetListIterator = *mut ENetListNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ENetList {
    pub sentinel: ENetListNode,
}
pub type ENetList = _ENetList;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ENetCallbacks {
    pub malloc: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub no_memory: Option<unsafe extern "C" fn() -> ()>,
}
pub type ENetCallbacks = _ENetCallbacks;
pub type ENetVersion = enet_uint32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ENetHost {
    pub socket: ENetSocket,
    pub address: ENetAddress,
    pub incomingBandwidth: enet_uint32,
    pub outgoingBandwidth: enet_uint32,
    pub bandwidthThrottleEpoch: enet_uint32,
    pub mtu: enet_uint32,
    pub randomSeed: enet_uint32,
    pub recalculateBandwidthLimits: libc::c_int,
    pub peers: *mut ENetPeer,
    pub peerCount: size_t,
    pub channelLimit: size_t,
    pub serviceTime: enet_uint32,
    pub dispatchQueue: ENetList,
    pub totalQueued: enet_uint32,
    pub packetSize: size_t,
    pub headerFlags: enet_uint16,
    pub commands: [ENetProtocol; 32],
    pub commandCount: size_t,
    pub buffers: [ENetBuffer; 65],
    pub bufferCount: size_t,
    pub checksum: ENetChecksumCallback,
    pub compressor: ENetCompressor,
    pub packetData: [[enet_uint8; 4096]; 2],
    pub receivedAddress: ENetAddress,
    pub receivedData: *mut enet_uint8,
    pub receivedDataLength: size_t,
    pub totalSentData: enet_uint32,
    pub totalSentPackets: enet_uint32,
    pub totalReceivedData: enet_uint32,
    pub totalReceivedPackets: enet_uint32,
    pub intercept: ENetInterceptCallback,
    pub connectedPeers: size_t,
    pub bandwidthLimitedPeers: size_t,
    pub duplicatePeers: size_t,
    pub maximumPacketSize: size_t,
    pub maximumWaitingData: size_t,
}
pub type ENetInterceptCallback =
    Option<unsafe extern "C" fn(*mut _ENetHost, *mut _ENetEvent) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ENetEvent {
    pub type_0: ENetEventType,
    pub peer: *mut ENetPeer,
    pub channelID: enet_uint8,
    pub data: enet_uint32,
    pub packet: *mut ENetPacket,
}
pub type ENetPacket = _ENetPacket;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ENetPacket {
    pub referenceCount: size_t,
    pub flags: enet_uint32,
    pub data: *mut enet_uint8,
    pub dataLength: size_t,
    pub freeCallback: ENetPacketFreeCallback,
    pub userData: *mut libc::c_void,
}
pub type ENetPacketFreeCallback = Option<unsafe extern "C" fn(*mut _ENetPacket) -> ()>;
pub type ENetPeer = _ENetPeer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ENetPeer {
    pub dispatchList: ENetListNode,
    pub host: *mut _ENetHost,
    pub outgoingPeerID: enet_uint16,
    pub incomingPeerID: enet_uint16,
    pub connectID: enet_uint32,
    pub outgoingSessionID: enet_uint8,
    pub incomingSessionID: enet_uint8,
    pub address: ENetAddress,
    pub data: *mut libc::c_void,
    pub state: ENetPeerState,
    pub channels: *mut ENetChannel,
    pub channelCount: size_t,
    pub incomingBandwidth: enet_uint32,
    pub outgoingBandwidth: enet_uint32,
    pub incomingBandwidthThrottleEpoch: enet_uint32,
    pub outgoingBandwidthThrottleEpoch: enet_uint32,
    pub incomingDataTotal: enet_uint32,
    pub outgoingDataTotal: enet_uint32,
    pub lastSendTime: enet_uint32,
    pub lastReceiveTime: enet_uint32,
    pub nextTimeout: enet_uint32,
    pub earliestTimeout: enet_uint32,
    pub packetLossEpoch: enet_uint32,
    pub packetsSent: enet_uint32,
    pub packetsLost: enet_uint32,
    pub packetLoss: enet_uint32,
    pub packetLossVariance: enet_uint32,
    pub packetThrottle: enet_uint32,
    pub packetThrottleLimit: enet_uint32,
    pub packetThrottleCounter: enet_uint32,
    pub packetThrottleEpoch: enet_uint32,
    pub packetThrottleAcceleration: enet_uint32,
    pub packetThrottleDeceleration: enet_uint32,
    pub packetThrottleInterval: enet_uint32,
    pub pingInterval: enet_uint32,
    pub timeoutLimit: enet_uint32,
    pub timeoutMinimum: enet_uint32,
    pub timeoutMaximum: enet_uint32,
    pub lastRoundTripTime: enet_uint32,
    pub lowestRoundTripTime: enet_uint32,
    pub lastRoundTripTimeVariance: enet_uint32,
    pub highestRoundTripTimeVariance: enet_uint32,
    pub roundTripTime: enet_uint32,
    pub roundTripTimeVariance: enet_uint32,
    pub mtu: enet_uint32,
    pub windowSize: enet_uint32,
    pub reliableDataInTransit: enet_uint32,
    pub outgoingReliableSequenceNumber: enet_uint16,
    pub acknowledgements: ENetList,
    pub sentReliableCommands: ENetList,
    pub outgoingSendReliableCommands: ENetList,
    pub outgoingCommands: ENetList,
    pub dispatchedCommands: ENetList,
    pub flags: enet_uint16,
    pub reserved: enet_uint16,
    pub incomingUnsequencedGroup: enet_uint16,
    pub outgoingUnsequencedGroup: enet_uint16,
    pub unsequencedWindow: [enet_uint32; 32],
    pub eventData: enet_uint32,
    pub totalWaitingData: size_t,
}
pub type ENetChannel = _ENetChannel;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ENetChannel {
    pub outgoingReliableSequenceNumber: enet_uint16,
    pub outgoingUnreliableSequenceNumber: enet_uint16,
    pub usedReliableWindows: enet_uint16,
    pub reliableWindows: [enet_uint16; 16],
    pub incomingReliableSequenceNumber: enet_uint16,
    pub incomingUnreliableSequenceNumber: enet_uint16,
    pub incomingReliableCommands: ENetList,
    pub incomingUnreliableCommands: ENetList,
}
pub type ENetPeerState = _ENetPeerState;
pub type _ENetPeerState = libc::c_uint;
pub const ENET_PEER_STATE_ZOMBIE: _ENetPeerState = 9;
pub const ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT: _ENetPeerState = 8;
pub const ENET_PEER_STATE_DISCONNECTING: _ENetPeerState = 7;
pub const ENET_PEER_STATE_DISCONNECT_LATER: _ENetPeerState = 6;
pub const ENET_PEER_STATE_CONNECTED: _ENetPeerState = 5;
pub const ENET_PEER_STATE_CONNECTION_SUCCEEDED: _ENetPeerState = 4;
pub const ENET_PEER_STATE_CONNECTION_PENDING: _ENetPeerState = 3;
pub const ENET_PEER_STATE_ACKNOWLEDGING_CONNECT: _ENetPeerState = 2;
pub const ENET_PEER_STATE_CONNECTING: _ENetPeerState = 1;
pub const ENET_PEER_STATE_DISCONNECTED: _ENetPeerState = 0;
pub type ENetAddress = _ENetAddress;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ENetAddress {
    pub host: enet_uint32,
    pub port: enet_uint16,
}
pub type ENetEventType = _ENetEventType;
pub type _ENetEventType = libc::c_uint;
pub const ENET_EVENT_TYPE_RECEIVE: _ENetEventType = 3;
pub const ENET_EVENT_TYPE_DISCONNECT: _ENetEventType = 2;
pub const ENET_EVENT_TYPE_CONNECT: _ENetEventType = 1;
pub const ENET_EVENT_TYPE_NONE: _ENetEventType = 0;
pub type ENetCompressor = _ENetCompressor;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ENetCompressor {
    pub context: *mut libc::c_void,
    pub compress: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const ENetBuffer,
            size_t,
            size_t,
            *mut enet_uint8,
            size_t,
        ) -> size_t,
    >,
    pub decompress: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const enet_uint8,
            size_t,
            *mut enet_uint8,
            size_t,
        ) -> size_t,
    >,
    pub destroy: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
pub type ENetChecksumCallback =
    Option<unsafe extern "C" fn(*const ENetBuffer, size_t) -> enet_uint32>;
pub type _ENetSocketType = libc::c_uint;
pub const ENET_SOCKET_TYPE_DATAGRAM: _ENetSocketType = 2;
pub const ENET_SOCKET_TYPE_STREAM: _ENetSocketType = 1;
pub type ENetSocketType = _ENetSocketType;
pub type _ENetSocketWait = libc::c_uint;
pub const ENET_SOCKET_WAIT_INTERRUPT: _ENetSocketWait = 4;
pub const ENET_SOCKET_WAIT_RECEIVE: _ENetSocketWait = 2;
pub const ENET_SOCKET_WAIT_SEND: _ENetSocketWait = 1;
pub const ENET_SOCKET_WAIT_NONE: _ENetSocketWait = 0;
pub type _ENetSocketOption = libc::c_uint;
pub const ENET_SOCKOPT_TTL: _ENetSocketOption = 10;
pub const ENET_SOCKOPT_NODELAY: _ENetSocketOption = 9;
pub const ENET_SOCKOPT_ERROR: _ENetSocketOption = 8;
pub const ENET_SOCKOPT_SNDTIMEO: _ENetSocketOption = 7;
pub const ENET_SOCKOPT_RCVTIMEO: _ENetSocketOption = 6;
pub const ENET_SOCKOPT_REUSEADDR: _ENetSocketOption = 5;
pub const ENET_SOCKOPT_SNDBUF: _ENetSocketOption = 4;
pub const ENET_SOCKOPT_RCVBUF: _ENetSocketOption = 3;
pub const ENET_SOCKOPT_BROADCAST: _ENetSocketOption = 2;
pub const ENET_SOCKOPT_NONBLOCK: _ENetSocketOption = 1;
pub type ENetSocketOption = _ENetSocketOption;
pub type _ENetSocketShutdown = libc::c_uint;
pub const ENET_SOCKET_SHUTDOWN_READ_WRITE: _ENetSocketShutdown = 2;
pub const ENET_SOCKET_SHUTDOWN_WRITE: _ENetSocketShutdown = 1;
pub const ENET_SOCKET_SHUTDOWN_READ: _ENetSocketShutdown = 0;
pub type ENetSocketShutdown = _ENetSocketShutdown;
pub type _ENetPacketFlag = libc::c_uint;
pub const ENET_PACKET_FLAG_SENT: _ENetPacketFlag = 256;
pub const ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT: _ENetPacketFlag = 8;
pub const ENET_PACKET_FLAG_NO_ALLOCATE: _ENetPacketFlag = 4;
pub const ENET_PACKET_FLAG_UNSEQUENCED: _ENetPacketFlag = 2;
pub const ENET_PACKET_FLAG_RELIABLE: _ENetPacketFlag = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ENetAcknowledgement {
    pub acknowledgementList: ENetListNode,
    pub sentTime: enet_uint32,
    pub command: ENetProtocol,
}
pub type ENetAcknowledgement = _ENetAcknowledgement;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ENetOutgoingCommand {
    pub outgoingCommandList: ENetListNode,
    pub reliableSequenceNumber: enet_uint16,
    pub unreliableSequenceNumber: enet_uint16,
    pub sentTime: enet_uint32,
    pub roundTripTimeout: enet_uint32,
    pub queueTime: enet_uint32,
    pub fragmentOffset: enet_uint32,
    pub fragmentLength: enet_uint16,
    pub sendAttempts: enet_uint16,
    pub command: ENetProtocol,
    pub packet: *mut ENetPacket,
}
pub type ENetOutgoingCommand = _ENetOutgoingCommand;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ENetIncomingCommand {
    pub incomingCommandList: ENetListNode,
    pub reliableSequenceNumber: enet_uint16,
    pub unreliableSequenceNumber: enet_uint16,
    pub command: ENetProtocol,
    pub fragmentCount: enet_uint32,
    pub fragmentsRemaining: enet_uint32,
    pub fragments: *mut enet_uint32,
    pub packet: *mut ENetPacket,
}
pub type ENetIncomingCommand = _ENetIncomingCommand;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const ENET_PEER_FREE_RELIABLE_WINDOWS: C2RustUnnamed_2 = 8;
pub const ENET_PEER_RELIABLE_WINDOW_SIZE: C2RustUnnamed_2 = 4096;
pub const ENET_PEER_RELIABLE_WINDOWS: C2RustUnnamed_2 = 16;
pub const ENET_PEER_FREE_UNSEQUENCED_WINDOWS: C2RustUnnamed_2 = 32;
pub const ENET_PEER_UNSEQUENCED_WINDOW_SIZE: C2RustUnnamed_2 = 1024;
pub const ENET_PEER_UNSEQUENCED_WINDOWS: C2RustUnnamed_2 = 64;
pub const ENET_PEER_PING_INTERVAL: C2RustUnnamed_2 = 500;
pub const ENET_PEER_TIMEOUT_MAXIMUM: C2RustUnnamed_2 = 30000;
pub const ENET_PEER_TIMEOUT_MINIMUM: C2RustUnnamed_2 = 5000;
pub const ENET_PEER_TIMEOUT_LIMIT: C2RustUnnamed_2 = 32;
pub const ENET_PEER_WINDOW_SIZE_SCALE: C2RustUnnamed_2 = 65536;
pub const ENET_PEER_PACKET_LOSS_INTERVAL: C2RustUnnamed_2 = 10000;
pub const ENET_PEER_PACKET_LOSS_SCALE: C2RustUnnamed_2 = 65536;
pub const ENET_PEER_PACKET_THROTTLE_INTERVAL: C2RustUnnamed_2 = 5000;
pub const ENET_PEER_PACKET_THROTTLE_DECELERATION: C2RustUnnamed_2 = 2;
pub const ENET_PEER_PACKET_THROTTLE_ACCELERATION: C2RustUnnamed_2 = 2;
pub const ENET_PEER_PACKET_THROTTLE_COUNTER: C2RustUnnamed_2 = 7;
pub const ENET_PEER_PACKET_THROTTLE_SCALE: C2RustUnnamed_2 = 32;
pub const ENET_PEER_DEFAULT_PACKET_THROTTLE: C2RustUnnamed_2 = 32;
pub const ENET_PEER_DEFAULT_ROUND_TRIP_TIME: C2RustUnnamed_2 = 500;
pub const ENET_HOST_DEFAULT_MAXIMUM_WAITING_DATA: C2RustUnnamed_2 = 33554432;
pub const ENET_HOST_DEFAULT_MAXIMUM_PACKET_SIZE: C2RustUnnamed_2 = 33554432;
pub const ENET_HOST_DEFAULT_MTU: C2RustUnnamed_2 = 1392;
pub const ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL: C2RustUnnamed_2 = 1000;
pub const ENET_HOST_SEND_BUFFER_SIZE: C2RustUnnamed_2 = 262144;
pub const ENET_HOST_RECEIVE_BUFFER_SIZE: C2RustUnnamed_2 = 262144;
pub type _ENetPeerFlag = libc::c_uint;
pub const ENET_PEER_FLAG_CONTINUE_SENDING: _ENetPeerFlag = 2;
pub const ENET_PEER_FLAG_NEEDS_DISPATCH: _ENetPeerFlag = 1;
pub type ENetHost = _ENetHost;
pub type ENetEvent = _ENetEvent;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
pub type ENetRangeCoder = _ENetRangeCoder;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ENetRangeCoder {
    pub symbols: [ENetSymbol; 4096],
}
pub type ENetSymbol = _ENetSymbol;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ENetSymbol {
    pub value: enet_uint8,
    pub count: enet_uint8,
    pub under: enet_uint16,
    pub left: enet_uint16,
    pub right: enet_uint16,
    pub symbols: enet_uint16,
    pub escapes: enet_uint16,
    pub total: enet_uint16,
    pub parent: enet_uint16,
}
pub const ENET_CONTEXT_SYMBOL_MINIMUM: C2RustUnnamed_3 = 1;
pub const ENET_CONTEXT_ESCAPE_MINIMUM: C2RustUnnamed_3 = 1;
pub const ENET_SUBCONTEXT_ORDER: C2RustUnnamed_3 = 2;
pub const ENET_RANGE_CODER_BOTTOM: C2RustUnnamed_3 = 65536;
pub const ENET_SUBCONTEXT_SYMBOL_DELTA: C2RustUnnamed_3 = 2;
pub const ENET_SUBCONTEXT_ESCAPE_DELTA: C2RustUnnamed_3 = 5;
pub const ENET_CONTEXT_SYMBOL_DELTA: C2RustUnnamed_3 = 3;
pub const ENET_RANGE_CODER_TOP: C2RustUnnamed_3 = 16777216;
pub type C2RustUnnamed_3 = libc::c_uint;
static mut callbacks: ENetCallbacks = unsafe {
    {
        let mut init = _ENetCallbacks {
            malloc: Some(malloc as unsafe extern "C" fn(libc::c_ulong) -> *mut libc::c_void),
            free: Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            no_memory: ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> !>,
                Option<unsafe extern "C" fn() -> ()>,
            >(Some(abort as unsafe extern "C" fn() -> !)),
        };
        init
    }
};
pub unsafe fn enet_initialize_with_callbacks(
    mut version: ENetVersion,
    mut inits: *const ENetCallbacks,
) -> libc::c_int {
    if version
        < ((1 as libc::c_int) << 16 as libc::c_int
            | (3 as libc::c_int) << 8 as libc::c_int
            | 0 as libc::c_int) as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    if ((*inits).malloc).is_some() || ((*inits).free).is_some() {
        if ((*inits).malloc).is_none() || ((*inits).free).is_none() {
            return -(1 as libc::c_int);
        }
        callbacks.malloc = (*inits).malloc;
        callbacks.free = (*inits).free;
    }
    if ((*inits).no_memory).is_some() {
        callbacks.no_memory = (*inits).no_memory;
    }
    return enet_initialize();
}
pub unsafe fn enet_linked_version() -> ENetVersion {
    return ((1 as libc::c_int) << 16 as libc::c_int
        | (3 as libc::c_int) << 8 as libc::c_int
        | 17 as libc::c_int) as ENetVersion;
}
#[no_mangle]
pub unsafe extern "C" fn enet_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut memory: *mut libc::c_void =
        (callbacks.malloc).expect("non-null function pointer")(size);
    if memory.is_null() {
        (callbacks.no_memory).expect("non-null function pointer")();
    }
    return memory;
}
#[no_mangle]
pub unsafe extern "C" fn enet_free(mut memory: *mut libc::c_void) {
    (callbacks.free).expect("non-null function pointer")(memory);
}
#[no_mangle]
pub unsafe extern "C" fn enet_range_coder_create() -> *mut libc::c_void {
    let mut rangeCoder: *mut ENetRangeCoder =
        enet_malloc(::core::mem::size_of::<ENetRangeCoder>() as libc::c_ulong)
            as *mut ENetRangeCoder;
    if rangeCoder.is_null() {
        return 0 as *mut libc::c_void;
    }
    return rangeCoder as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn enet_range_coder_destroy(mut context: *mut libc::c_void) {
    let mut rangeCoder: *mut ENetRangeCoder = context as *mut ENetRangeCoder;
    if rangeCoder.is_null() {
        return;
    }
    enet_free(rangeCoder as *mut libc::c_void);
}
unsafe extern "C" fn enet_symbol_rescale(mut symbol: *mut ENetSymbol) -> enet_uint16 {
    let mut total: enet_uint16 = 0 as libc::c_int as enet_uint16;
    loop {
        (*symbol).count = ((*symbol).count as libc::c_int
            - ((*symbol).count as libc::c_int >> 1 as libc::c_int))
            as enet_uint8;
        (*symbol).under = (*symbol).count as enet_uint16;
        if (*symbol).left != 0 {
            (*symbol).under = ((*symbol).under as libc::c_int
                + enet_symbol_rescale(symbol.offset((*symbol).left as libc::c_int as isize))
                    as libc::c_int) as enet_uint16;
        }
        total = (total as libc::c_int + (*symbol).under as libc::c_int) as enet_uint16;
        if (*symbol).right == 0 {
            break;
        }
        symbol = symbol.offset((*symbol).right as libc::c_int as isize);
    }
    return total;
}
#[no_mangle]
pub unsafe extern "C" fn enet_range_coder_compress(
    mut context: *mut libc::c_void,
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
    let mut encodeLow: enet_uint32 = 0 as libc::c_int as enet_uint32;
    let mut encodeRange: enet_uint32 = !(0 as libc::c_int) as enet_uint32;
    let mut root: *mut ENetSymbol = 0 as *mut ENetSymbol;
    let mut predicted: enet_uint16 = 0 as libc::c_int as enet_uint16;
    let mut order: size_t = 0 as libc::c_int as size_t;
    let mut nextSymbol: size_t = 0 as libc::c_int as size_t;
    if rangeCoder.is_null()
        || inBufferCount <= 0 as libc::c_int as libc::c_ulong
        || inLimit <= 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int as size_t;
    }
    inData = (*inBuffers).data as *const enet_uint8;
    inEnd = &*inData.offset((*inBuffers).dataLength as isize) as *const enet_uint8;
    inBuffers = inBuffers.offset(1);
    inBufferCount = inBufferCount.wrapping_sub(1);
    let fresh0 = nextSymbol;
    nextSymbol = nextSymbol.wrapping_add(1);
    root = &mut *((*rangeCoder).symbols).as_mut_ptr().offset(fresh0 as isize) as *mut ENetSymbol;
    (*root).value = 0 as libc::c_int as enet_uint8;
    (*root).count = 0 as libc::c_int as enet_uint8;
    (*root).under = 0 as libc::c_int as enet_uint16;
    (*root).left = 0 as libc::c_int as enet_uint16;
    (*root).right = 0 as libc::c_int as enet_uint16;
    (*root).symbols = 0 as libc::c_int as enet_uint16;
    (*root).escapes = 0 as libc::c_int as enet_uint16;
    (*root).total = 0 as libc::c_int as enet_uint16;
    (*root).parent = 0 as libc::c_int as enet_uint16;
    (*root).escapes = ENET_CONTEXT_ESCAPE_MINIMUM as libc::c_int as enet_uint16;
    (*root).total = (ENET_CONTEXT_ESCAPE_MINIMUM as libc::c_int
        + 256 as libc::c_int * ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int)
        as enet_uint16;
    (*root).symbols = 0 as libc::c_int as enet_uint16;
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
            if inBufferCount <= 0 as libc::c_int as libc::c_ulong {
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
            under = (value as libc::c_int * 0 as libc::c_int) as enet_uint16;
            count = 0 as libc::c_int as enet_uint16;
            if (*subcontext).symbols == 0 {
                let fresh2 = nextSymbol;
                nextSymbol = nextSymbol.wrapping_add(1);
                symbol = &mut *((*rangeCoder).symbols).as_mut_ptr().offset(fresh2 as isize)
                    as *mut ENetSymbol;
                (*symbol).value = value;
                (*symbol).count = ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint8;
                (*symbol).under = ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint16;
                (*symbol).left = 0 as libc::c_int as enet_uint16;
                (*symbol).right = 0 as libc::c_int as enet_uint16;
                (*symbol).symbols = 0 as libc::c_int as enet_uint16;
                (*symbol).escapes = 0 as libc::c_int as enet_uint16;
                (*symbol).total = 0 as libc::c_int as enet_uint16;
                (*symbol).parent = 0 as libc::c_int as enet_uint16;
                (*subcontext).symbols =
                    symbol.offset_from(subcontext) as libc::c_long as enet_uint16;
            } else {
                let mut node: *mut ENetSymbol =
                    subcontext.offset((*subcontext).symbols as libc::c_int as isize);
                loop {
                    if (value as libc::c_int) < (*node).value as libc::c_int {
                        (*node).under = ((*node).under as libc::c_int
                            + ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                            as enet_uint16;
                        if (*node).left != 0 {
                            node = node.offset((*node).left as libc::c_int as isize);
                        } else {
                            let fresh3 = nextSymbol;
                            nextSymbol = nextSymbol.wrapping_add(1);
                            symbol =
                                &mut *((*rangeCoder).symbols).as_mut_ptr().offset(fresh3 as isize)
                                    as *mut ENetSymbol;
                            (*symbol).value = value;
                            (*symbol).count =
                                ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint8;
                            (*symbol).under =
                                ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint16;
                            (*symbol).left = 0 as libc::c_int as enet_uint16;
                            (*symbol).right = 0 as libc::c_int as enet_uint16;
                            (*symbol).symbols = 0 as libc::c_int as enet_uint16;
                            (*symbol).escapes = 0 as libc::c_int as enet_uint16;
                            (*symbol).total = 0 as libc::c_int as enet_uint16;
                            (*symbol).parent = 0 as libc::c_int as enet_uint16;
                            (*node).left = symbol.offset_from(node) as libc::c_long as enet_uint16;
                            break;
                        }
                    } else if value as libc::c_int > (*node).value as libc::c_int {
                        under =
                            (under as libc::c_int + (*node).under as libc::c_int) as enet_uint16;
                        if (*node).right != 0 {
                            node = node.offset((*node).right as libc::c_int as isize);
                        } else {
                            let fresh4 = nextSymbol;
                            nextSymbol = nextSymbol.wrapping_add(1);
                            symbol =
                                &mut *((*rangeCoder).symbols).as_mut_ptr().offset(fresh4 as isize)
                                    as *mut ENetSymbol;
                            (*symbol).value = value;
                            (*symbol).count =
                                ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint8;
                            (*symbol).under =
                                ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint16;
                            (*symbol).left = 0 as libc::c_int as enet_uint16;
                            (*symbol).right = 0 as libc::c_int as enet_uint16;
                            (*symbol).symbols = 0 as libc::c_int as enet_uint16;
                            (*symbol).escapes = 0 as libc::c_int as enet_uint16;
                            (*symbol).total = 0 as libc::c_int as enet_uint16;
                            (*symbol).parent = 0 as libc::c_int as enet_uint16;
                            (*node).right = symbol.offset_from(node) as libc::c_long as enet_uint16;
                            break;
                        }
                    } else {
                        count =
                            (count as libc::c_int + (*node).count as libc::c_int) as enet_uint16;
                        under = (under as libc::c_int
                            + ((*node).under as libc::c_int - (*node).count as libc::c_int))
                            as enet_uint16;
                        (*node).under = ((*node).under as libc::c_int
                            + ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                            as enet_uint16;
                        (*node).count = ((*node).count as libc::c_int
                            + ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                            as enet_uint8;
                        symbol = node;
                        break;
                    }
                }
            }
            *parent = symbol.offset_from(((*rangeCoder).symbols).as_mut_ptr()) as libc::c_long
                as enet_uint16;
            parent = &mut (*symbol).parent;
            total = (*subcontext).total;
            if count as libc::c_int > 0 as libc::c_int {
                encodeRange = (encodeRange as libc::c_uint).wrapping_div(total as libc::c_uint)
                    as enet_uint32 as enet_uint32;
                encodeLow = (encodeLow as libc::c_uint).wrapping_add(
                    (((*subcontext).escapes as libc::c_int + under as libc::c_int) as libc::c_uint)
                        .wrapping_mul(encodeRange),
                ) as enet_uint32 as enet_uint32;
                encodeRange = (encodeRange as libc::c_uint).wrapping_mul(count as libc::c_uint)
                    as enet_uint32 as enet_uint32;
                loop {
                    if encodeLow ^ encodeLow.wrapping_add(encodeRange)
                        >= ENET_RANGE_CODER_TOP as libc::c_int as libc::c_uint
                    {
                        if encodeRange >= ENET_RANGE_CODER_BOTTOM as libc::c_int as libc::c_uint {
                            break;
                        }
                        encodeRange = encodeLow.wrapping_neg()
                            & (ENET_RANGE_CODER_BOTTOM as libc::c_int - 1 as libc::c_int)
                                as libc::c_uint;
                    }
                    if outData >= outEnd {
                        return 0 as libc::c_int as size_t;
                    }
                    let fresh5 = outData;
                    outData = outData.offset(1);
                    *fresh5 = (encodeLow >> 24 as libc::c_int) as enet_uint8;
                    encodeRange <<= 8 as libc::c_int;
                    encodeLow <<= 8 as libc::c_int;
                }
            } else {
                if (*subcontext).escapes as libc::c_int > 0 as libc::c_int
                    && ((*subcontext).escapes as libc::c_int) < total as libc::c_int
                {
                    encodeRange = (encodeRange as libc::c_uint).wrapping_div(total as libc::c_uint)
                        as enet_uint32 as enet_uint32;
                    encodeLow = (encodeLow as libc::c_uint)
                        .wrapping_add((0 as libc::c_int as libc::c_uint).wrapping_mul(encodeRange))
                        as enet_uint32 as enet_uint32;
                    encodeRange = (encodeRange as libc::c_uint)
                        .wrapping_mul((*subcontext).escapes as libc::c_uint)
                        as enet_uint32 as enet_uint32;
                    loop {
                        if encodeLow ^ encodeLow.wrapping_add(encodeRange)
                            >= ENET_RANGE_CODER_TOP as libc::c_int as libc::c_uint
                        {
                            if encodeRange >= ENET_RANGE_CODER_BOTTOM as libc::c_int as libc::c_uint
                            {
                                break;
                            }
                            encodeRange = encodeLow.wrapping_neg()
                                & (ENET_RANGE_CODER_BOTTOM as libc::c_int - 1 as libc::c_int)
                                    as libc::c_uint;
                        }
                        if outData >= outEnd {
                            return 0 as libc::c_int as size_t;
                        }
                        let fresh6 = outData;
                        outData = outData.offset(1);
                        *fresh6 = (encodeLow >> 24 as libc::c_int) as enet_uint8;
                        encodeRange <<= 8 as libc::c_int;
                        encodeLow <<= 8 as libc::c_int;
                    }
                }
                (*subcontext).escapes = ((*subcontext).escapes as libc::c_int
                    + ENET_SUBCONTEXT_ESCAPE_DELTA as libc::c_int)
                    as enet_uint16;
                (*subcontext).total = ((*subcontext).total as libc::c_int
                    + ENET_SUBCONTEXT_ESCAPE_DELTA as libc::c_int)
                    as enet_uint16;
            }
            (*subcontext).total = ((*subcontext).total as libc::c_int
                + ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                as enet_uint16;
            if count as libc::c_int
                > 0xff as libc::c_int
                    - 2 as libc::c_int * ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int
                || (*subcontext).total as libc::c_int
                    > ENET_RANGE_CODER_BOTTOM as libc::c_int - 0x100 as libc::c_int
            {
                (*subcontext).total = (if (*subcontext).symbols as libc::c_int != 0 {
                    enet_symbol_rescale(
                        subcontext.offset((*subcontext).symbols as libc::c_int as isize),
                    ) as libc::c_int
                } else {
                    0 as libc::c_int
                }) as enet_uint16;
                (*subcontext).escapes = ((*subcontext).escapes as libc::c_int
                    - ((*subcontext).escapes as libc::c_int >> 1 as libc::c_int))
                    as enet_uint16;
                (*subcontext).total = ((*subcontext).total as libc::c_int
                    + ((*subcontext).escapes as libc::c_int
                        + 256 as libc::c_int * 0 as libc::c_int))
                    as enet_uint16;
            }
            if count as libc::c_int > 0 as libc::c_int {
                current_block_237 = 836937598693885467;
                break;
            }
            subcontext = &mut *((*rangeCoder).symbols)
                .as_mut_ptr()
                .offset((*subcontext).parent as isize) as *mut ENetSymbol;
        }
        match current_block_237 {
            2463987395154258233 => {
                under = (value as libc::c_int * ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int)
                    as enet_uint16;
                count = ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int as enet_uint16;
                if (*root).symbols == 0 {
                    let fresh7 = nextSymbol;
                    nextSymbol = nextSymbol.wrapping_add(1);
                    symbol = &mut *((*rangeCoder).symbols).as_mut_ptr().offset(fresh7 as isize)
                        as *mut ENetSymbol;
                    (*symbol).value = value;
                    (*symbol).count = ENET_CONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint8;
                    (*symbol).under = ENET_CONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint16;
                    (*symbol).left = 0 as libc::c_int as enet_uint16;
                    (*symbol).right = 0 as libc::c_int as enet_uint16;
                    (*symbol).symbols = 0 as libc::c_int as enet_uint16;
                    (*symbol).escapes = 0 as libc::c_int as enet_uint16;
                    (*symbol).total = 0 as libc::c_int as enet_uint16;
                    (*symbol).parent = 0 as libc::c_int as enet_uint16;
                    (*root).symbols = symbol.offset_from(root) as libc::c_long as enet_uint16;
                } else {
                    let mut node_0: *mut ENetSymbol =
                        root.offset((*root).symbols as libc::c_int as isize);
                    loop {
                        if (value as libc::c_int) < (*node_0).value as libc::c_int {
                            (*node_0).under = ((*node_0).under as libc::c_int
                                + ENET_CONTEXT_SYMBOL_DELTA as libc::c_int)
                                as enet_uint16;
                            if (*node_0).left != 0 {
                                node_0 = node_0.offset((*node_0).left as libc::c_int as isize);
                            } else {
                                let fresh8 = nextSymbol;
                                nextSymbol = nextSymbol.wrapping_add(1);
                                symbol = &mut *((*rangeCoder).symbols)
                                    .as_mut_ptr()
                                    .offset(fresh8 as isize)
                                    as *mut ENetSymbol;
                                (*symbol).value = value;
                                (*symbol).count =
                                    ENET_CONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint8;
                                (*symbol).under =
                                    ENET_CONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint16;
                                (*symbol).left = 0 as libc::c_int as enet_uint16;
                                (*symbol).right = 0 as libc::c_int as enet_uint16;
                                (*symbol).symbols = 0 as libc::c_int as enet_uint16;
                                (*symbol).escapes = 0 as libc::c_int as enet_uint16;
                                (*symbol).total = 0 as libc::c_int as enet_uint16;
                                (*symbol).parent = 0 as libc::c_int as enet_uint16;
                                (*node_0).left =
                                    symbol.offset_from(node_0) as libc::c_long as enet_uint16;
                                break;
                            }
                        } else if value as libc::c_int > (*node_0).value as libc::c_int {
                            under = (under as libc::c_int + (*node_0).under as libc::c_int)
                                as enet_uint16;
                            if (*node_0).right != 0 {
                                node_0 = node_0.offset((*node_0).right as libc::c_int as isize);
                            } else {
                                let fresh9 = nextSymbol;
                                nextSymbol = nextSymbol.wrapping_add(1);
                                symbol = &mut *((*rangeCoder).symbols)
                                    .as_mut_ptr()
                                    .offset(fresh9 as isize)
                                    as *mut ENetSymbol;
                                (*symbol).value = value;
                                (*symbol).count =
                                    ENET_CONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint8;
                                (*symbol).under =
                                    ENET_CONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint16;
                                (*symbol).left = 0 as libc::c_int as enet_uint16;
                                (*symbol).right = 0 as libc::c_int as enet_uint16;
                                (*symbol).symbols = 0 as libc::c_int as enet_uint16;
                                (*symbol).escapes = 0 as libc::c_int as enet_uint16;
                                (*symbol).total = 0 as libc::c_int as enet_uint16;
                                (*symbol).parent = 0 as libc::c_int as enet_uint16;
                                (*node_0).right =
                                    symbol.offset_from(node_0) as libc::c_long as enet_uint16;
                                break;
                            }
                        } else {
                            count = (count as libc::c_int + (*node_0).count as libc::c_int)
                                as enet_uint16;
                            under = (under as libc::c_int
                                + ((*node_0).under as libc::c_int - (*node_0).count as libc::c_int))
                                as enet_uint16;
                            (*node_0).under = ((*node_0).under as libc::c_int
                                + ENET_CONTEXT_SYMBOL_DELTA as libc::c_int)
                                as enet_uint16;
                            (*node_0).count = ((*node_0).count as libc::c_int
                                + ENET_CONTEXT_SYMBOL_DELTA as libc::c_int)
                                as enet_uint8;
                            symbol = node_0;
                            break;
                        }
                    }
                }
                *parent = symbol.offset_from(((*rangeCoder).symbols).as_mut_ptr()) as libc::c_long
                    as enet_uint16;
                parent = &mut (*symbol).parent;
                total = (*root).total;
                encodeRange = (encodeRange as libc::c_uint).wrapping_div(total as libc::c_uint)
                    as enet_uint32 as enet_uint32;
                encodeLow = (encodeLow as libc::c_uint).wrapping_add(
                    (((*root).escapes as libc::c_int + under as libc::c_int) as libc::c_uint)
                        .wrapping_mul(encodeRange),
                ) as enet_uint32 as enet_uint32;
                encodeRange = (encodeRange as libc::c_uint).wrapping_mul(count as libc::c_uint)
                    as enet_uint32 as enet_uint32;
                loop {
                    if encodeLow ^ encodeLow.wrapping_add(encodeRange)
                        >= ENET_RANGE_CODER_TOP as libc::c_int as libc::c_uint
                    {
                        if encodeRange >= ENET_RANGE_CODER_BOTTOM as libc::c_int as libc::c_uint {
                            break;
                        }
                        encodeRange = encodeLow.wrapping_neg()
                            & (ENET_RANGE_CODER_BOTTOM as libc::c_int - 1 as libc::c_int)
                                as libc::c_uint;
                    }
                    if outData >= outEnd {
                        return 0 as libc::c_int as size_t;
                    }
                    let fresh10 = outData;
                    outData = outData.offset(1);
                    *fresh10 = (encodeLow >> 24 as libc::c_int) as enet_uint8;
                    encodeRange <<= 8 as libc::c_int;
                    encodeLow <<= 8 as libc::c_int;
                }
                (*root).total = ((*root).total as libc::c_int
                    + ENET_CONTEXT_SYMBOL_DELTA as libc::c_int)
                    as enet_uint16;
                if count as libc::c_int
                    > 0xff as libc::c_int
                        - 2 as libc::c_int * ENET_CONTEXT_SYMBOL_DELTA as libc::c_int
                        + ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int
                    || (*root).total as libc::c_int
                        > ENET_RANGE_CODER_BOTTOM as libc::c_int - 0x100 as libc::c_int
                {
                    (*root).total = (if (*root).symbols as libc::c_int != 0 {
                        enet_symbol_rescale(root.offset((*root).symbols as libc::c_int as isize))
                            as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as enet_uint16;
                    (*root).escapes = ((*root).escapes as libc::c_int
                        - ((*root).escapes as libc::c_int >> 1 as libc::c_int))
                        as enet_uint16;
                    (*root).total = ((*root).total as libc::c_int
                        + ((*root).escapes as libc::c_int
                            + 256 as libc::c_int * ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int))
                        as enet_uint16;
                }
            }
            _ => {}
        }
        if order >= ENET_SUBCONTEXT_ORDER as libc::c_int as libc::c_ulong {
            predicted = (*rangeCoder).symbols[predicted as usize].parent;
        } else {
            order = order.wrapping_add(1);
        }
        if nextSymbol
            >= (::core::mem::size_of::<[ENetSymbol; 4096]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<ENetSymbol>() as libc::c_ulong)
                .wrapping_sub(ENET_SUBCONTEXT_ORDER as libc::c_int as libc::c_ulong)
        {
            nextSymbol = 0 as libc::c_int as size_t;
            let fresh11 = nextSymbol;
            nextSymbol = nextSymbol.wrapping_add(1);
            root = &mut *((*rangeCoder).symbols)
                .as_mut_ptr()
                .offset(fresh11 as isize) as *mut ENetSymbol;
            (*root).value = 0 as libc::c_int as enet_uint8;
            (*root).count = 0 as libc::c_int as enet_uint8;
            (*root).under = 0 as libc::c_int as enet_uint16;
            (*root).left = 0 as libc::c_int as enet_uint16;
            (*root).right = 0 as libc::c_int as enet_uint16;
            (*root).symbols = 0 as libc::c_int as enet_uint16;
            (*root).escapes = 0 as libc::c_int as enet_uint16;
            (*root).total = 0 as libc::c_int as enet_uint16;
            (*root).parent = 0 as libc::c_int as enet_uint16;
            (*root).escapes = ENET_CONTEXT_ESCAPE_MINIMUM as libc::c_int as enet_uint16;
            (*root).total = (ENET_CONTEXT_ESCAPE_MINIMUM as libc::c_int
                + 256 as libc::c_int * ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int)
                as enet_uint16;
            (*root).symbols = 0 as libc::c_int as enet_uint16;
            predicted = 0 as libc::c_int as enet_uint16;
            order = 0 as libc::c_int as size_t;
        }
    }
    while encodeLow != 0 {
        if outData >= outEnd {
            return 0 as libc::c_int as size_t;
        }
        let fresh12 = outData;
        outData = outData.offset(1);
        *fresh12 = (encodeLow >> 24 as libc::c_int) as enet_uint8;
        encodeLow <<= 8 as libc::c_int;
    }
    return outData.offset_from(outStart) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn enet_range_coder_decompress(
    mut context: *mut libc::c_void,
    mut inData: *const enet_uint8,
    mut inLimit: size_t,
    mut outData: *mut enet_uint8,
    mut outLimit: size_t,
) -> size_t {
    let mut rangeCoder: *mut ENetRangeCoder = context as *mut ENetRangeCoder;
    let mut outStart: *mut enet_uint8 = outData;
    let mut outEnd: *mut enet_uint8 = &mut *outData.offset(outLimit as isize) as *mut enet_uint8;
    let mut inEnd: *const enet_uint8 = &*inData.offset(inLimit as isize) as *const enet_uint8;
    let mut decodeLow: enet_uint32 = 0 as libc::c_int as enet_uint32;
    let mut decodeCode: enet_uint32 = 0 as libc::c_int as enet_uint32;
    let mut decodeRange: enet_uint32 = !(0 as libc::c_int) as enet_uint32;
    let mut root: *mut ENetSymbol = 0 as *mut ENetSymbol;
    let mut predicted: enet_uint16 = 0 as libc::c_int as enet_uint16;
    let mut order: size_t = 0 as libc::c_int as size_t;
    let mut nextSymbol: size_t = 0 as libc::c_int as size_t;
    if rangeCoder.is_null() || inLimit <= 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    let fresh13 = nextSymbol;
    nextSymbol = nextSymbol.wrapping_add(1);
    root = &mut *((*rangeCoder).symbols)
        .as_mut_ptr()
        .offset(fresh13 as isize) as *mut ENetSymbol;
    (*root).value = 0 as libc::c_int as enet_uint8;
    (*root).count = 0 as libc::c_int as enet_uint8;
    (*root).under = 0 as libc::c_int as enet_uint16;
    (*root).left = 0 as libc::c_int as enet_uint16;
    (*root).right = 0 as libc::c_int as enet_uint16;
    (*root).symbols = 0 as libc::c_int as enet_uint16;
    (*root).escapes = 0 as libc::c_int as enet_uint16;
    (*root).total = 0 as libc::c_int as enet_uint16;
    (*root).parent = 0 as libc::c_int as enet_uint16;
    (*root).escapes = ENET_CONTEXT_ESCAPE_MINIMUM as libc::c_int as enet_uint16;
    (*root).total = (ENET_CONTEXT_ESCAPE_MINIMUM as libc::c_int
        + 256 as libc::c_int * ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int)
        as enet_uint16;
    (*root).symbols = 0 as libc::c_int as enet_uint16;
    if inData < inEnd {
        let fresh14 = inData;
        inData = inData.offset(1);
        decodeCode |= ((*fresh14 as libc::c_int) << 24 as libc::c_int) as libc::c_uint;
    }
    if inData < inEnd {
        let fresh15 = inData;
        inData = inData.offset(1);
        decodeCode |= ((*fresh15 as libc::c_int) << 16 as libc::c_int) as libc::c_uint;
    }
    if inData < inEnd {
        let fresh16 = inData;
        inData = inData.offset(1);
        decodeCode |= ((*fresh16 as libc::c_int) << 8 as libc::c_int) as libc::c_uint;
    }
    if inData < inEnd {
        let fresh17 = inData;
        inData = inData.offset(1);
        decodeCode |= *fresh17 as libc::c_uint;
    }
    let mut current_block_297: u64;
    loop {
        let mut subcontext: *mut ENetSymbol = 0 as *mut ENetSymbol;
        let mut symbol: *mut ENetSymbol = 0 as *mut ENetSymbol;
        let mut patch: *mut ENetSymbol = 0 as *mut ENetSymbol;
        let mut value: enet_uint8 = 0 as libc::c_int as enet_uint8;
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
            if !((*subcontext).escapes as libc::c_int <= 0 as libc::c_int) {
                total = (*subcontext).total;
                if !((*subcontext).escapes as libc::c_int >= total as libc::c_int) {
                    decodeRange = (decodeRange as libc::c_uint).wrapping_div(total as libc::c_uint)
                        as enet_uint32 as enet_uint32;
                    code =
                        decodeCode.wrapping_sub(decodeLow).wrapping_div(decodeRange) as enet_uint16;
                    if (code as libc::c_int) < (*subcontext).escapes as libc::c_int {
                        decodeLow = (decodeLow as libc::c_uint).wrapping_add(
                            (0 as libc::c_int as libc::c_uint).wrapping_mul(decodeRange),
                        ) as enet_uint32 as enet_uint32;
                        decodeRange = (decodeRange as libc::c_uint)
                            .wrapping_mul((*subcontext).escapes as libc::c_uint)
                            as enet_uint32 as enet_uint32;
                        loop {
                            if decodeLow ^ decodeLow.wrapping_add(decodeRange)
                                >= ENET_RANGE_CODER_TOP as libc::c_int as libc::c_uint
                            {
                                if decodeRange
                                    >= ENET_RANGE_CODER_BOTTOM as libc::c_int as libc::c_uint
                                {
                                    break;
                                }
                                decodeRange = decodeLow.wrapping_neg()
                                    & (ENET_RANGE_CODER_BOTTOM as libc::c_int - 1 as libc::c_int)
                                        as libc::c_uint;
                            }
                            decodeCode <<= 8 as libc::c_int;
                            if inData < inEnd {
                                let fresh18 = inData;
                                inData = inData.offset(1);
                                decodeCode |= *fresh18 as libc::c_uint;
                            }
                            decodeRange <<= 8 as libc::c_int;
                            decodeLow <<= 8 as libc::c_int;
                        }
                    } else {
                        code = (code as libc::c_int - (*subcontext).escapes as libc::c_int)
                            as enet_uint16;
                        under = 0 as libc::c_int as enet_uint16;
                        count = 0 as libc::c_int as enet_uint16;
                        if (*subcontext).symbols == 0 {
                            return 0 as libc::c_int as size_t;
                        } else {
                            let mut node: *mut ENetSymbol =
                                subcontext.offset((*subcontext).symbols as libc::c_int as isize);
                            loop {
                                let mut after: enet_uint16 = (under as libc::c_int
                                    + (*node).under as libc::c_int
                                    + ((*node).value as libc::c_int + 1 as libc::c_int)
                                        * 0 as libc::c_int)
                                    as enet_uint16;
                                let mut before: enet_uint16 = ((*node).count as libc::c_int
                                    + 0 as libc::c_int)
                                    as enet_uint16;
                                if code as libc::c_int >= after as libc::c_int {
                                    under = (under as libc::c_int + (*node).under as libc::c_int)
                                        as enet_uint16;
                                    if (*node).right != 0 {
                                        node = node.offset((*node).right as libc::c_int as isize);
                                    } else {
                                        return 0 as libc::c_int as size_t;
                                    }
                                } else if (code as libc::c_int)
                                    < after as libc::c_int - before as libc::c_int
                                {
                                    (*node).under = ((*node).under as libc::c_int
                                        + ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                                        as enet_uint16;
                                    if (*node).left != 0 {
                                        node = node.offset((*node).left as libc::c_int as isize);
                                    } else {
                                        return 0 as libc::c_int as size_t;
                                    }
                                } else {
                                    value = (*node).value;
                                    count = (count as libc::c_int + (*node).count as libc::c_int)
                                        as enet_uint16;
                                    under = (after as libc::c_int - before as libc::c_int)
                                        as enet_uint16;
                                    (*node).under = ((*node).under as libc::c_int
                                        + ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                                        as enet_uint16;
                                    (*node).count = ((*node).count as libc::c_int
                                        + ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                                        as enet_uint8;
                                    symbol = node;
                                    break;
                                }
                            }
                        }
                        bottom = symbol.offset_from(((*rangeCoder).symbols).as_mut_ptr())
                            as libc::c_long as enet_uint16;
                        decodeLow = (decodeLow as libc::c_uint).wrapping_add(
                            (((*subcontext).escapes as libc::c_int + under as libc::c_int)
                                as libc::c_uint)
                                .wrapping_mul(decodeRange),
                        ) as enet_uint32 as enet_uint32;
                        decodeRange = (decodeRange as libc::c_uint)
                            .wrapping_mul(count as libc::c_uint)
                            as enet_uint32 as enet_uint32;
                        loop {
                            if decodeLow ^ decodeLow.wrapping_add(decodeRange)
                                >= ENET_RANGE_CODER_TOP as libc::c_int as libc::c_uint
                            {
                                if decodeRange
                                    >= ENET_RANGE_CODER_BOTTOM as libc::c_int as libc::c_uint
                                {
                                    break;
                                }
                                decodeRange = decodeLow.wrapping_neg()
                                    & (ENET_RANGE_CODER_BOTTOM as libc::c_int - 1 as libc::c_int)
                                        as libc::c_uint;
                            }
                            decodeCode <<= 8 as libc::c_int;
                            if inData < inEnd {
                                let fresh19 = inData;
                                inData = inData.offset(1);
                                decodeCode |= *fresh19 as libc::c_uint;
                            }
                            decodeRange <<= 8 as libc::c_int;
                            decodeLow <<= 8 as libc::c_int;
                        }
                        (*subcontext).total = ((*subcontext).total as libc::c_int
                            + ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                            as enet_uint16;
                        if count as libc::c_int
                            > 0xff as libc::c_int
                                - 2 as libc::c_int * ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int
                            || (*subcontext).total as libc::c_int
                                > ENET_RANGE_CODER_BOTTOM as libc::c_int - 0x100 as libc::c_int
                        {
                            (*subcontext).total = (if (*subcontext).symbols as libc::c_int != 0 {
                                enet_symbol_rescale(
                                    subcontext
                                        .offset((*subcontext).symbols as libc::c_int as isize),
                                ) as libc::c_int
                            } else {
                                0 as libc::c_int
                            }) as enet_uint16;
                            (*subcontext).escapes = ((*subcontext).escapes as libc::c_int
                                - ((*subcontext).escapes as libc::c_int >> 1 as libc::c_int))
                                as enet_uint16;
                            (*subcontext).total = ((*subcontext).total as libc::c_int
                                + ((*subcontext).escapes as libc::c_int
                                    + 256 as libc::c_int * 0 as libc::c_int))
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
                decodeRange = (decodeRange as libc::c_uint).wrapping_div(total as libc::c_uint)
                    as enet_uint32 as enet_uint32;
                code = decodeCode.wrapping_sub(decodeLow).wrapping_div(decodeRange) as enet_uint16;
                if (code as libc::c_int) < (*root).escapes as libc::c_int {
                    decodeLow = (decodeLow as libc::c_uint)
                        .wrapping_add((0 as libc::c_int as libc::c_uint).wrapping_mul(decodeRange))
                        as enet_uint32 as enet_uint32;
                    decodeRange = (decodeRange as libc::c_uint)
                        .wrapping_mul((*root).escapes as libc::c_uint)
                        as enet_uint32 as enet_uint32;
                    loop {
                        if decodeLow ^ decodeLow.wrapping_add(decodeRange)
                            >= ENET_RANGE_CODER_TOP as libc::c_int as libc::c_uint
                        {
                            if decodeRange >= ENET_RANGE_CODER_BOTTOM as libc::c_int as libc::c_uint
                            {
                                break;
                            }
                            decodeRange = decodeLow.wrapping_neg()
                                & (ENET_RANGE_CODER_BOTTOM as libc::c_int - 1 as libc::c_int)
                                    as libc::c_uint;
                        }
                        decodeCode <<= 8 as libc::c_int;
                        if inData < inEnd {
                            let fresh20 = inData;
                            inData = inData.offset(1);
                            decodeCode |= *fresh20 as libc::c_uint;
                        }
                        decodeRange <<= 8 as libc::c_int;
                        decodeLow <<= 8 as libc::c_int;
                    }
                    break;
                } else {
                    code = (code as libc::c_int - (*root).escapes as libc::c_int) as enet_uint16;
                    under = 0 as libc::c_int as enet_uint16;
                    count = ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int as enet_uint16;
                    if (*root).symbols == 0 {
                        value = (code as libc::c_int / ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int)
                            as enet_uint8;
                        under = (code as libc::c_int
                            - code as libc::c_int % ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int)
                            as enet_uint16;
                        let fresh21 = nextSymbol;
                        nextSymbol = nextSymbol.wrapping_add(1);
                        symbol = &mut *((*rangeCoder).symbols)
                            .as_mut_ptr()
                            .offset(fresh21 as isize)
                            as *mut ENetSymbol;
                        (*symbol).value = value;
                        (*symbol).count = ENET_CONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint8;
                        (*symbol).under = ENET_CONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint16;
                        (*symbol).left = 0 as libc::c_int as enet_uint16;
                        (*symbol).right = 0 as libc::c_int as enet_uint16;
                        (*symbol).symbols = 0 as libc::c_int as enet_uint16;
                        (*symbol).escapes = 0 as libc::c_int as enet_uint16;
                        (*symbol).total = 0 as libc::c_int as enet_uint16;
                        (*symbol).parent = 0 as libc::c_int as enet_uint16;
                        (*root).symbols = symbol.offset_from(root) as libc::c_long as enet_uint16;
                    } else {
                        let mut node_0: *mut ENetSymbol =
                            root.offset((*root).symbols as libc::c_int as isize);
                        loop {
                            let mut after_0: enet_uint16 = (under as libc::c_int
                                + (*node_0).under as libc::c_int
                                + ((*node_0).value as libc::c_int + 1 as libc::c_int)
                                    * ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int)
                                as enet_uint16;
                            let mut before_0: enet_uint16 = ((*node_0).count as libc::c_int
                                + ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int)
                                as enet_uint16;
                            if code as libc::c_int >= after_0 as libc::c_int {
                                under = (under as libc::c_int + (*node_0).under as libc::c_int)
                                    as enet_uint16;
                                if (*node_0).right != 0 {
                                    node_0 = node_0.offset((*node_0).right as libc::c_int as isize);
                                } else {
                                    value = ((*node_0).value as libc::c_int
                                        + 1 as libc::c_int
                                        + (code as libc::c_int - after_0 as libc::c_int)
                                            / ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int)
                                        as enet_uint8;
                                    under = (code as libc::c_int
                                        - (code as libc::c_int - after_0 as libc::c_int)
                                            % ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int)
                                        as enet_uint16;
                                    let fresh22 = nextSymbol;
                                    nextSymbol = nextSymbol.wrapping_add(1);
                                    symbol = &mut *((*rangeCoder).symbols)
                                        .as_mut_ptr()
                                        .offset(fresh22 as isize)
                                        as *mut ENetSymbol;
                                    (*symbol).value = value;
                                    (*symbol).count =
                                        ENET_CONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint8;
                                    (*symbol).under =
                                        ENET_CONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint16;
                                    (*symbol).left = 0 as libc::c_int as enet_uint16;
                                    (*symbol).right = 0 as libc::c_int as enet_uint16;
                                    (*symbol).symbols = 0 as libc::c_int as enet_uint16;
                                    (*symbol).escapes = 0 as libc::c_int as enet_uint16;
                                    (*symbol).total = 0 as libc::c_int as enet_uint16;
                                    (*symbol).parent = 0 as libc::c_int as enet_uint16;
                                    (*node_0).right =
                                        symbol.offset_from(node_0) as libc::c_long as enet_uint16;
                                    break;
                                }
                            } else if (code as libc::c_int)
                                < after_0 as libc::c_int - before_0 as libc::c_int
                            {
                                (*node_0).under = ((*node_0).under as libc::c_int
                                    + ENET_CONTEXT_SYMBOL_DELTA as libc::c_int)
                                    as enet_uint16;
                                if (*node_0).left != 0 {
                                    node_0 = node_0.offset((*node_0).left as libc::c_int as isize);
                                } else {
                                    value = ((*node_0).value as libc::c_int
                                        - 1 as libc::c_int
                                        - (after_0 as libc::c_int
                                            - before_0 as libc::c_int
                                            - code as libc::c_int
                                            - 1 as libc::c_int)
                                            / ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int)
                                        as enet_uint8;
                                    under = (code as libc::c_int
                                        - (after_0 as libc::c_int
                                            - before_0 as libc::c_int
                                            - code as libc::c_int
                                            - 1 as libc::c_int)
                                            % ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int)
                                        as enet_uint16;
                                    let fresh23 = nextSymbol;
                                    nextSymbol = nextSymbol.wrapping_add(1);
                                    symbol = &mut *((*rangeCoder).symbols)
                                        .as_mut_ptr()
                                        .offset(fresh23 as isize)
                                        as *mut ENetSymbol;
                                    (*symbol).value = value;
                                    (*symbol).count =
                                        ENET_CONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint8;
                                    (*symbol).under =
                                        ENET_CONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint16;
                                    (*symbol).left = 0 as libc::c_int as enet_uint16;
                                    (*symbol).right = 0 as libc::c_int as enet_uint16;
                                    (*symbol).symbols = 0 as libc::c_int as enet_uint16;
                                    (*symbol).escapes = 0 as libc::c_int as enet_uint16;
                                    (*symbol).total = 0 as libc::c_int as enet_uint16;
                                    (*symbol).parent = 0 as libc::c_int as enet_uint16;
                                    (*node_0).left =
                                        symbol.offset_from(node_0) as libc::c_long as enet_uint16;
                                    break;
                                }
                            } else {
                                value = (*node_0).value;
                                count = (count as libc::c_int + (*node_0).count as libc::c_int)
                                    as enet_uint16;
                                under = (after_0 as libc::c_int - before_0 as libc::c_int)
                                    as enet_uint16;
                                (*node_0).under = ((*node_0).under as libc::c_int
                                    + ENET_CONTEXT_SYMBOL_DELTA as libc::c_int)
                                    as enet_uint16;
                                (*node_0).count = ((*node_0).count as libc::c_int
                                    + ENET_CONTEXT_SYMBOL_DELTA as libc::c_int)
                                    as enet_uint8;
                                symbol = node_0;
                                break;
                            }
                        }
                    }
                    bottom = symbol.offset_from(((*rangeCoder).symbols).as_mut_ptr())
                        as libc::c_long as enet_uint16;
                    decodeLow = (decodeLow as libc::c_uint).wrapping_add(
                        (((*root).escapes as libc::c_int + under as libc::c_int) as libc::c_uint)
                            .wrapping_mul(decodeRange),
                    ) as enet_uint32 as enet_uint32;
                    decodeRange = (decodeRange as libc::c_uint).wrapping_mul(count as libc::c_uint)
                        as enet_uint32 as enet_uint32;
                    loop {
                        if decodeLow ^ decodeLow.wrapping_add(decodeRange)
                            >= ENET_RANGE_CODER_TOP as libc::c_int as libc::c_uint
                        {
                            if decodeRange >= ENET_RANGE_CODER_BOTTOM as libc::c_int as libc::c_uint
                            {
                                break;
                            }
                            decodeRange = decodeLow.wrapping_neg()
                                & (ENET_RANGE_CODER_BOTTOM as libc::c_int - 1 as libc::c_int)
                                    as libc::c_uint;
                        }
                        decodeCode <<= 8 as libc::c_int;
                        if inData < inEnd {
                            let fresh24 = inData;
                            inData = inData.offset(1);
                            decodeCode |= *fresh24 as libc::c_uint;
                        }
                        decodeRange <<= 8 as libc::c_int;
                        decodeLow <<= 8 as libc::c_int;
                    }
                    (*root).total = ((*root).total as libc::c_int
                        + ENET_CONTEXT_SYMBOL_DELTA as libc::c_int)
                        as enet_uint16;
                    if count as libc::c_int
                        > 0xff as libc::c_int
                            - 2 as libc::c_int * ENET_CONTEXT_SYMBOL_DELTA as libc::c_int
                            + ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int
                        || (*root).total as libc::c_int
                            > ENET_RANGE_CODER_BOTTOM as libc::c_int - 0x100 as libc::c_int
                    {
                        (*root).total = (if (*root).symbols as libc::c_int != 0 {
                            enet_symbol_rescale(
                                root.offset((*root).symbols as libc::c_int as isize),
                            ) as libc::c_int
                        } else {
                            0 as libc::c_int
                        }) as enet_uint16;
                        (*root).escapes = ((*root).escapes as libc::c_int
                            - ((*root).escapes as libc::c_int >> 1 as libc::c_int))
                            as enet_uint16;
                        (*root).total = ((*root).total as libc::c_int
                            + ((*root).escapes as libc::c_int
                                + 256 as libc::c_int * ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int))
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
            under = (value as libc::c_int * 0 as libc::c_int) as enet_uint16;
            count = 0 as libc::c_int as enet_uint16;
            if (*patch).symbols == 0 {
                let fresh25 = nextSymbol;
                nextSymbol = nextSymbol.wrapping_add(1);
                symbol = &mut *((*rangeCoder).symbols)
                    .as_mut_ptr()
                    .offset(fresh25 as isize) as *mut ENetSymbol;
                (*symbol).value = value;
                (*symbol).count = ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint8;
                (*symbol).under = ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint16;
                (*symbol).left = 0 as libc::c_int as enet_uint16;
                (*symbol).right = 0 as libc::c_int as enet_uint16;
                (*symbol).symbols = 0 as libc::c_int as enet_uint16;
                (*symbol).escapes = 0 as libc::c_int as enet_uint16;
                (*symbol).total = 0 as libc::c_int as enet_uint16;
                (*symbol).parent = 0 as libc::c_int as enet_uint16;
                (*patch).symbols = symbol.offset_from(patch) as libc::c_long as enet_uint16;
            } else {
                let mut node_1: *mut ENetSymbol =
                    patch.offset((*patch).symbols as libc::c_int as isize);
                loop {
                    if (value as libc::c_int) < (*node_1).value as libc::c_int {
                        (*node_1).under = ((*node_1).under as libc::c_int
                            + ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                            as enet_uint16;
                        if (*node_1).left != 0 {
                            node_1 = node_1.offset((*node_1).left as libc::c_int as isize);
                        } else {
                            let fresh26 = nextSymbol;
                            nextSymbol = nextSymbol.wrapping_add(1);
                            symbol = &mut *((*rangeCoder).symbols)
                                .as_mut_ptr()
                                .offset(fresh26 as isize)
                                as *mut ENetSymbol;
                            (*symbol).value = value;
                            (*symbol).count =
                                ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint8;
                            (*symbol).under =
                                ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint16;
                            (*symbol).left = 0 as libc::c_int as enet_uint16;
                            (*symbol).right = 0 as libc::c_int as enet_uint16;
                            (*symbol).symbols = 0 as libc::c_int as enet_uint16;
                            (*symbol).escapes = 0 as libc::c_int as enet_uint16;
                            (*symbol).total = 0 as libc::c_int as enet_uint16;
                            (*symbol).parent = 0 as libc::c_int as enet_uint16;
                            (*node_1).left =
                                symbol.offset_from(node_1) as libc::c_long as enet_uint16;
                            break;
                        }
                    } else if value as libc::c_int > (*node_1).value as libc::c_int {
                        under =
                            (under as libc::c_int + (*node_1).under as libc::c_int) as enet_uint16;
                        if (*node_1).right != 0 {
                            node_1 = node_1.offset((*node_1).right as libc::c_int as isize);
                        } else {
                            let fresh27 = nextSymbol;
                            nextSymbol = nextSymbol.wrapping_add(1);
                            symbol = &mut *((*rangeCoder).symbols)
                                .as_mut_ptr()
                                .offset(fresh27 as isize)
                                as *mut ENetSymbol;
                            (*symbol).value = value;
                            (*symbol).count =
                                ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint8;
                            (*symbol).under =
                                ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int as enet_uint16;
                            (*symbol).left = 0 as libc::c_int as enet_uint16;
                            (*symbol).right = 0 as libc::c_int as enet_uint16;
                            (*symbol).symbols = 0 as libc::c_int as enet_uint16;
                            (*symbol).escapes = 0 as libc::c_int as enet_uint16;
                            (*symbol).total = 0 as libc::c_int as enet_uint16;
                            (*symbol).parent = 0 as libc::c_int as enet_uint16;
                            (*node_1).right =
                                symbol.offset_from(node_1) as libc::c_long as enet_uint16;
                            break;
                        }
                    } else {
                        count =
                            (count as libc::c_int + (*node_1).count as libc::c_int) as enet_uint16;
                        under = (under as libc::c_int
                            + ((*node_1).under as libc::c_int - (*node_1).count as libc::c_int))
                            as enet_uint16;
                        (*node_1).under = ((*node_1).under as libc::c_int
                            + ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                            as enet_uint16;
                        (*node_1).count = ((*node_1).count as libc::c_int
                            + ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                            as enet_uint8;
                        symbol = node_1;
                        break;
                    }
                }
            }
            *parent = symbol.offset_from(((*rangeCoder).symbols).as_mut_ptr()) as libc::c_long
                as enet_uint16;
            parent = &mut (*symbol).parent;
            if count as libc::c_int <= 0 as libc::c_int {
                (*patch).escapes = ((*patch).escapes as libc::c_int
                    + ENET_SUBCONTEXT_ESCAPE_DELTA as libc::c_int)
                    as enet_uint16;
                (*patch).total = ((*patch).total as libc::c_int
                    + ENET_SUBCONTEXT_ESCAPE_DELTA as libc::c_int)
                    as enet_uint16;
            }
            (*patch).total = ((*patch).total as libc::c_int
                + ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int)
                as enet_uint16;
            if count as libc::c_int
                > 0xff as libc::c_int
                    - 2 as libc::c_int * ENET_SUBCONTEXT_SYMBOL_DELTA as libc::c_int
                || (*patch).total as libc::c_int
                    > ENET_RANGE_CODER_BOTTOM as libc::c_int - 0x100 as libc::c_int
            {
                (*patch).total = (if (*patch).symbols as libc::c_int != 0 {
                    enet_symbol_rescale(patch.offset((*patch).symbols as libc::c_int as isize))
                        as libc::c_int
                } else {
                    0 as libc::c_int
                }) as enet_uint16;
                (*patch).escapes = ((*patch).escapes as libc::c_int
                    - ((*patch).escapes as libc::c_int >> 1 as libc::c_int))
                    as enet_uint16;
                (*patch).total = ((*patch).total as libc::c_int
                    + ((*patch).escapes as libc::c_int + 256 as libc::c_int * 0 as libc::c_int))
                    as enet_uint16;
            }
            patch = &mut *((*rangeCoder).symbols)
                .as_mut_ptr()
                .offset((*patch).parent as isize) as *mut ENetSymbol;
        }
        *parent = bottom;
        if outData >= outEnd {
            return 0 as libc::c_int as size_t;
        }
        let fresh28 = outData;
        outData = outData.offset(1);
        *fresh28 = value;
        if order >= ENET_SUBCONTEXT_ORDER as libc::c_int as libc::c_ulong {
            predicted = (*rangeCoder).symbols[predicted as usize].parent;
        } else {
            order = order.wrapping_add(1);
        }
        if nextSymbol
            >= (::core::mem::size_of::<[ENetSymbol; 4096]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<ENetSymbol>() as libc::c_ulong)
                .wrapping_sub(ENET_SUBCONTEXT_ORDER as libc::c_int as libc::c_ulong)
        {
            nextSymbol = 0 as libc::c_int as size_t;
            let fresh29 = nextSymbol;
            nextSymbol = nextSymbol.wrapping_add(1);
            root = &mut *((*rangeCoder).symbols)
                .as_mut_ptr()
                .offset(fresh29 as isize) as *mut ENetSymbol;
            (*root).value = 0 as libc::c_int as enet_uint8;
            (*root).count = 0 as libc::c_int as enet_uint8;
            (*root).under = 0 as libc::c_int as enet_uint16;
            (*root).left = 0 as libc::c_int as enet_uint16;
            (*root).right = 0 as libc::c_int as enet_uint16;
            (*root).symbols = 0 as libc::c_int as enet_uint16;
            (*root).escapes = 0 as libc::c_int as enet_uint16;
            (*root).total = 0 as libc::c_int as enet_uint16;
            (*root).parent = 0 as libc::c_int as enet_uint16;
            (*root).escapes = ENET_CONTEXT_ESCAPE_MINIMUM as libc::c_int as enet_uint16;
            (*root).total = (ENET_CONTEXT_ESCAPE_MINIMUM as libc::c_int
                + 256 as libc::c_int * ENET_CONTEXT_SYMBOL_MINIMUM as libc::c_int)
                as enet_uint16;
            (*root).symbols = 0 as libc::c_int as enet_uint16;
            predicted = 0 as libc::c_int as enet_uint16;
            order = 0 as libc::c_int as size_t;
        }
    }
    return outData.offset_from(outStart) as libc::c_long as size_t;
}
pub unsafe fn enet_host_compress_with_range_coder(mut host: *mut ENetHost) -> libc::c_int {
    let mut compressor: ENetCompressor = ENetCompressor {
        context: 0 as *mut libc::c_void,
        compress: None,
        decompress: None,
        destroy: None,
    };
    memset(
        &mut compressor as *mut ENetCompressor as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ENetCompressor>() as libc::c_ulong,
    );
    compressor.context = enet_range_coder_create();
    if (compressor.context).is_null() {
        return -(1 as libc::c_int);
    }
    compressor.compress = Some(
        enet_range_coder_compress
            as unsafe extern "C" fn(
                *mut libc::c_void,
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
                *mut libc::c_void,
                *const enet_uint8,
                size_t,
                *mut enet_uint8,
                size_t,
            ) -> size_t,
    );
    compressor.destroy =
        Some(enet_range_coder_destroy as unsafe extern "C" fn(*mut libc::c_void) -> ());
    enet_host_compress(host, &mut compressor);
    return 0 as libc::c_int;
}
pub unsafe fn enet_host_create(
    mut address: *const ENetAddress,
    mut peerCount: size_t,
    mut channelLimit: size_t,
    mut incomingBandwidth: enet_uint32,
    mut outgoingBandwidth: enet_uint32,
) -> *mut ENetHost {
    let mut host: *mut ENetHost = 0 as *mut ENetHost;
    let mut currentPeer: *mut ENetPeer = 0 as *mut ENetPeer;
    if peerCount > ENET_PROTOCOL_MAXIMUM_PEER_ID as libc::c_int as libc::c_ulong {
        return 0 as *mut ENetHost;
    }
    host = enet_malloc(::core::mem::size_of::<ENetHost>() as libc::c_ulong) as *mut ENetHost;
    if host.is_null() {
        return 0 as *mut ENetHost;
    }
    memset(
        host as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ENetHost>() as libc::c_ulong,
    );
    (*host).peers =
        enet_malloc(peerCount.wrapping_mul(::core::mem::size_of::<ENetPeer>() as libc::c_ulong))
            as *mut ENetPeer;
    if ((*host).peers).is_null() {
        enet_free(host as *mut libc::c_void);
        return 0 as *mut ENetHost;
    }
    memset(
        (*host).peers as *mut libc::c_void,
        0 as libc::c_int,
        peerCount.wrapping_mul(::core::mem::size_of::<ENetPeer>() as libc::c_ulong),
    );
    (*host).socket = enet_socket_create(ENET_SOCKET_TYPE_DATAGRAM);
    if (*host).socket == -(1 as libc::c_int)
        || !address.is_null() && enet_socket_bind((*host).socket, address) < 0 as libc::c_int
    {
        if (*host).socket != -(1 as libc::c_int) {
            enet_socket_destroy((*host).socket);
        }
        enet_free((*host).peers as *mut libc::c_void);
        enet_free(host as *mut libc::c_void);
        return 0 as *mut ENetHost;
    }
    enet_socket_set_option((*host).socket, ENET_SOCKOPT_NONBLOCK, 1 as libc::c_int);
    enet_socket_set_option((*host).socket, ENET_SOCKOPT_BROADCAST, 1 as libc::c_int);
    enet_socket_set_option(
        (*host).socket,
        ENET_SOCKOPT_RCVBUF,
        ENET_HOST_RECEIVE_BUFFER_SIZE as libc::c_int,
    );
    enet_socket_set_option(
        (*host).socket,
        ENET_SOCKOPT_SNDBUF,
        ENET_HOST_SEND_BUFFER_SIZE as libc::c_int,
    );
    if !address.is_null()
        && enet_socket_get_address((*host).socket, &mut (*host).address) < 0 as libc::c_int
    {
        (*host).address = *address;
    }
    if channelLimit == 0
        || channelLimit > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as libc::c_int as libc::c_ulong
    {
        channelLimit = ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as libc::c_int as size_t;
    } else if channelLimit < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as libc::c_int as libc::c_ulong {
        channelLimit = ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as libc::c_int as size_t;
    }
    (*host).randomSeed = host as size_t as enet_uint32;
    (*host).randomSeed = ((*host).randomSeed as libc::c_uint).wrapping_add(enet_host_random_seed())
        as enet_uint32 as enet_uint32;
    (*host).randomSeed =
        (*host).randomSeed << 16 as libc::c_int | (*host).randomSeed >> 16 as libc::c_int;
    (*host).channelLimit = channelLimit;
    (*host).incomingBandwidth = incomingBandwidth;
    (*host).outgoingBandwidth = outgoingBandwidth;
    (*host).bandwidthThrottleEpoch = 0 as libc::c_int as enet_uint32;
    (*host).recalculateBandwidthLimits = 0 as libc::c_int;
    (*host).mtu = ENET_HOST_DEFAULT_MTU as libc::c_int as enet_uint32;
    (*host).peerCount = peerCount;
    (*host).commandCount = 0 as libc::c_int as size_t;
    (*host).bufferCount = 0 as libc::c_int as size_t;
    (*host).checksum = None;
    (*host).receivedAddress.host = 0 as libc::c_int as enet_uint32;
    (*host).receivedAddress.port = 0 as libc::c_int as enet_uint16;
    (*host).receivedData = 0 as *mut enet_uint8;
    (*host).receivedDataLength = 0 as libc::c_int as size_t;
    (*host).totalSentData = 0 as libc::c_int as enet_uint32;
    (*host).totalSentPackets = 0 as libc::c_int as enet_uint32;
    (*host).totalReceivedData = 0 as libc::c_int as enet_uint32;
    (*host).totalReceivedPackets = 0 as libc::c_int as enet_uint32;
    (*host).totalQueued = 0 as libc::c_int as enet_uint32;
    (*host).connectedPeers = 0 as libc::c_int as size_t;
    (*host).bandwidthLimitedPeers = 0 as libc::c_int as size_t;
    (*host).duplicatePeers = ENET_PROTOCOL_MAXIMUM_PEER_ID as libc::c_int as size_t;
    (*host).maximumPacketSize = ENET_HOST_DEFAULT_MAXIMUM_PACKET_SIZE as libc::c_int as size_t;
    (*host).maximumWaitingData = ENET_HOST_DEFAULT_MAXIMUM_WAITING_DATA as libc::c_int as size_t;
    (*host).compressor.context = 0 as *mut libc::c_void;
    (*host).compressor.compress = None;
    (*host).compressor.decompress = None;
    (*host).compressor.destroy = None;
    (*host).intercept = None;
    enet_list_clear(&mut (*host).dispatchQueue);
    currentPeer = (*host).peers;
    while currentPeer < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer {
        (*currentPeer).host = host;
        (*currentPeer).incomingPeerID =
            currentPeer.offset_from((*host).peers) as libc::c_long as enet_uint16;
        (*currentPeer).incomingSessionID = 0xff as libc::c_int as enet_uint8;
        (*currentPeer).outgoingSessionID = (*currentPeer).incomingSessionID;
        (*currentPeer).data = 0 as *mut libc::c_void;
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
pub unsafe fn enet_host_destroy(mut host: *mut ENetHost) {
    let mut currentPeer: *mut ENetPeer = 0 as *mut ENetPeer;
    if host.is_null() {
        return;
    }
    enet_socket_destroy((*host).socket);
    currentPeer = (*host).peers;
    while currentPeer < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer {
        enet_peer_reset(currentPeer);
        currentPeer = currentPeer.offset(1);
    }
    if !((*host).compressor.context).is_null() && ((*host).compressor.destroy).is_some() {
        (Some(((*host).compressor.destroy).expect("non-null function pointer")))
            .expect("non-null function pointer")((*host).compressor.context);
    }
    enet_free((*host).peers as *mut libc::c_void);
    enet_free(host as *mut libc::c_void);
}
pub unsafe fn enet_host_random(mut host: *mut ENetHost) -> enet_uint32 {
    (*host).randomSeed = ((*host).randomSeed as libc::c_uint)
        .wrapping_add(0x6d2b79f5 as libc::c_uint) as enet_uint32
        as enet_uint32;
    let mut n: enet_uint32 = (*host).randomSeed;
    n = (n ^ n >> 15 as libc::c_int).wrapping_mul(n | 1 as libc::c_uint);
    n ^= n.wrapping_add((n ^ n >> 7 as libc::c_int).wrapping_mul(n | 61 as libc::c_uint));
    return n ^ n >> 14 as libc::c_int;
}
pub unsafe fn enet_host_connect(
    mut host: *mut ENetHost,
    mut address: *const ENetAddress,
    mut channelCount: size_t,
    mut data: enet_uint32,
) -> *mut ENetPeer {
    let mut currentPeer: *mut ENetPeer = 0 as *mut ENetPeer;
    let mut channel: *mut ENetChannel = 0 as *mut ENetChannel;
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    if channelCount < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as libc::c_int as libc::c_ulong {
        channelCount = ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as libc::c_int as size_t;
    } else if channelCount > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as libc::c_int as libc::c_ulong {
        channelCount = ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as libc::c_int as size_t;
    }
    currentPeer = (*host).peers;
    while currentPeer < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer {
        if (*currentPeer).state as libc::c_uint
            == ENET_PEER_STATE_DISCONNECTED as libc::c_int as libc::c_uint
        {
            break;
        }
        currentPeer = currentPeer.offset(1);
    }
    if currentPeer >= &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer {
        return 0 as *mut ENetPeer;
    }
    (*currentPeer).channels = enet_malloc(
        channelCount.wrapping_mul(::core::mem::size_of::<ENetChannel>() as libc::c_ulong),
    ) as *mut ENetChannel;
    if ((*currentPeer).channels).is_null() {
        return 0 as *mut ENetPeer;
    }
    (*currentPeer).channelCount = channelCount;
    (*currentPeer).state = ENET_PEER_STATE_CONNECTING;
    (*currentPeer).address = *address;
    (*currentPeer).connectID = enet_host_random(host);
    (*currentPeer).mtu = (*host).mtu;
    if (*host).outgoingBandwidth == 0 as libc::c_int as libc::c_uint {
        (*currentPeer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as enet_uint32;
    } else {
        (*currentPeer).windowSize = ((*host).outgoingBandwidth)
            .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as libc::c_int as libc::c_uint)
            .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint);
    }
    if (*currentPeer).windowSize < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint
    {
        (*currentPeer).windowSize = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as enet_uint32;
    } else if (*currentPeer).windowSize
        > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint
    {
        (*currentPeer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as enet_uint32;
    }
    channel = (*currentPeer).channels;
    while channel
        < &mut *((*currentPeer).channels).offset(channelCount as isize) as *mut ENetChannel
    {
        (*channel).outgoingReliableSequenceNumber = 0 as libc::c_int as enet_uint16;
        (*channel).outgoingUnreliableSequenceNumber = 0 as libc::c_int as enet_uint16;
        (*channel).incomingReliableSequenceNumber = 0 as libc::c_int as enet_uint16;
        (*channel).incomingUnreliableSequenceNumber = 0 as libc::c_int as enet_uint16;
        enet_list_clear(&mut (*channel).incomingReliableCommands);
        enet_list_clear(&mut (*channel).incomingUnreliableCommands);
        (*channel).usedReliableWindows = 0 as libc::c_int as enet_uint16;
        memset(
            ((*channel).reliableWindows).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[enet_uint16; 16]>() as libc::c_ulong,
        );
        channel = channel.offset(1);
    }
    command.header.command = (ENET_PROTOCOL_COMMAND_CONNECT as libc::c_int
        | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int)
        as enet_uint8;
    command.header.channelID = 0xff as libc::c_int as enet_uint8;
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
        0 as libc::c_int as enet_uint32,
        0 as libc::c_int as enet_uint16,
    );
    return currentPeer;
}
pub unsafe fn enet_host_broadcast(
    mut host: *mut ENetHost,
    mut channelID: enet_uint8,
    mut packet: *mut ENetPacket,
) {
    let mut currentPeer: *mut ENetPeer = 0 as *mut ENetPeer;
    currentPeer = (*host).peers;
    while currentPeer < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer {
        if !((*currentPeer).state as libc::c_uint
            != ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint)
        {
            enet_peer_send(currentPeer, channelID, packet);
        }
        currentPeer = currentPeer.offset(1);
    }
    if (*packet).referenceCount == 0 as libc::c_int as libc::c_ulong {
        enet_packet_destroy(packet);
    }
}
pub unsafe fn enet_host_compress(mut host: *mut ENetHost, mut compressor: *const ENetCompressor) {
    if !((*host).compressor.context).is_null() && ((*host).compressor.destroy).is_some() {
        (Some(((*host).compressor.destroy).expect("non-null function pointer")))
            .expect("non-null function pointer")((*host).compressor.context);
    }
    if !compressor.is_null() {
        (*host).compressor = *compressor;
    } else {
        (*host).compressor.context = 0 as *mut libc::c_void;
    };
}
pub unsafe fn enet_host_channel_limit(mut host: *mut ENetHost, mut channelLimit: size_t) {
    if channelLimit == 0
        || channelLimit > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as libc::c_int as libc::c_ulong
    {
        channelLimit = ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as libc::c_int as size_t;
    } else if channelLimit < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as libc::c_int as libc::c_ulong {
        channelLimit = ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as libc::c_int as size_t;
    }
    (*host).channelLimit = channelLimit;
}
pub unsafe fn enet_host_bandwidth_limit(
    mut host: *mut ENetHost,
    mut incomingBandwidth: enet_uint32,
    mut outgoingBandwidth: enet_uint32,
) {
    (*host).incomingBandwidth = incomingBandwidth;
    (*host).outgoingBandwidth = outgoingBandwidth;
    (*host).recalculateBandwidthLimits = 1 as libc::c_int;
}
pub unsafe fn enet_host_bandwidth_throttle(mut host: *mut ENetHost) {
    let mut timeCurrent: enet_uint32 = enet_time_get();
    let mut elapsedTime: enet_uint32 = timeCurrent.wrapping_sub((*host).bandwidthThrottleEpoch);
    let mut peersRemaining: enet_uint32 = (*host).connectedPeers as enet_uint32;
    let mut dataTotal: enet_uint32 = !(0 as libc::c_int) as enet_uint32;
    let mut bandwidth: enet_uint32 = !(0 as libc::c_int) as enet_uint32;
    let mut throttle: enet_uint32 = 0 as libc::c_int as enet_uint32;
    let mut bandwidthLimit: enet_uint32 = 0 as libc::c_int as enet_uint32;
    let mut needsAdjustment: libc::c_int =
        if (*host).bandwidthLimitedPeers > 0 as libc::c_int as libc::c_ulong {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    let mut peer: *mut ENetPeer = 0 as *mut ENetPeer;
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    if elapsedTime < ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL as libc::c_int as libc::c_uint {
        return;
    }
    (*host).bandwidthThrottleEpoch = timeCurrent;
    if peersRemaining == 0 as libc::c_int as libc::c_uint {
        return;
    }
    if (*host).outgoingBandwidth != 0 as libc::c_int as libc::c_uint {
        dataTotal = 0 as libc::c_int as enet_uint32;
        bandwidth = ((*host).outgoingBandwidth)
            .wrapping_mul(elapsedTime)
            .wrapping_div(1000 as libc::c_int as libc::c_uint);
        peer = (*host).peers;
        while peer < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer {
            if !((*peer).state as libc::c_uint
                != ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
                && (*peer).state as libc::c_uint
                    != ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint)
            {
                dataTotal = (dataTotal as libc::c_uint).wrapping_add((*peer).outgoingDataTotal)
                    as enet_uint32 as enet_uint32;
            }
            peer = peer.offset(1);
        }
    }
    while peersRemaining > 0 as libc::c_int as libc::c_uint && needsAdjustment != 0 as libc::c_int {
        needsAdjustment = 0 as libc::c_int;
        if dataTotal <= bandwidth {
            throttle = ENET_PEER_PACKET_THROTTLE_SCALE as libc::c_int as enet_uint32;
        } else {
            throttle = bandwidth
                .wrapping_mul(ENET_PEER_PACKET_THROTTLE_SCALE as libc::c_int as libc::c_uint)
                .wrapping_div(dataTotal);
        }
        peer = (*host).peers;
        while peer < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer {
            let mut peerBandwidth: enet_uint32 = 0;
            if !((*peer).state as libc::c_uint
                != ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
                && (*peer).state as libc::c_uint
                    != ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
                || (*peer).incomingBandwidth == 0 as libc::c_int as libc::c_uint
                || (*peer).outgoingBandwidthThrottleEpoch == timeCurrent)
            {
                peerBandwidth = ((*peer).incomingBandwidth)
                    .wrapping_mul(elapsedTime)
                    .wrapping_div(1000 as libc::c_int as libc::c_uint);
                if !(throttle
                    .wrapping_mul((*peer).outgoingDataTotal)
                    .wrapping_div(ENET_PEER_PACKET_THROTTLE_SCALE as libc::c_int as libc::c_uint)
                    <= peerBandwidth)
                {
                    (*peer).packetThrottleLimit = peerBandwidth
                        .wrapping_mul(
                            ENET_PEER_PACKET_THROTTLE_SCALE as libc::c_int as libc::c_uint,
                        )
                        .wrapping_div((*peer).outgoingDataTotal);
                    if (*peer).packetThrottleLimit == 0 as libc::c_int as libc::c_uint {
                        (*peer).packetThrottleLimit = 1 as libc::c_int as enet_uint32;
                    }
                    if (*peer).packetThrottle > (*peer).packetThrottleLimit {
                        (*peer).packetThrottle = (*peer).packetThrottleLimit;
                    }
                    (*peer).outgoingBandwidthThrottleEpoch = timeCurrent;
                    (*peer).incomingDataTotal = 0 as libc::c_int as enet_uint32;
                    (*peer).outgoingDataTotal = 0 as libc::c_int as enet_uint32;
                    needsAdjustment = 1 as libc::c_int;
                    peersRemaining = peersRemaining.wrapping_sub(1);
                    bandwidth = (bandwidth as libc::c_uint).wrapping_sub(peerBandwidth)
                        as enet_uint32 as enet_uint32;
                    dataTotal = (dataTotal as libc::c_uint).wrapping_sub(peerBandwidth)
                        as enet_uint32 as enet_uint32;
                }
            }
            peer = peer.offset(1);
        }
    }
    if peersRemaining > 0 as libc::c_int as libc::c_uint {
        if dataTotal <= bandwidth {
            throttle = ENET_PEER_PACKET_THROTTLE_SCALE as libc::c_int as enet_uint32;
        } else {
            throttle = bandwidth
                .wrapping_mul(ENET_PEER_PACKET_THROTTLE_SCALE as libc::c_int as libc::c_uint)
                .wrapping_div(dataTotal);
        }
        peer = (*host).peers;
        while peer < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer {
            if !((*peer).state as libc::c_uint
                != ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
                && (*peer).state as libc::c_uint
                    != ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
                || (*peer).outgoingBandwidthThrottleEpoch == timeCurrent)
            {
                (*peer).packetThrottleLimit = throttle;
                if (*peer).packetThrottle > (*peer).packetThrottleLimit {
                    (*peer).packetThrottle = (*peer).packetThrottleLimit;
                }
                (*peer).incomingDataTotal = 0 as libc::c_int as enet_uint32;
                (*peer).outgoingDataTotal = 0 as libc::c_int as enet_uint32;
            }
            peer = peer.offset(1);
        }
    }
    if (*host).recalculateBandwidthLimits != 0 {
        (*host).recalculateBandwidthLimits = 0 as libc::c_int;
        peersRemaining = (*host).connectedPeers as enet_uint32;
        bandwidth = (*host).incomingBandwidth;
        needsAdjustment = 1 as libc::c_int;
        if bandwidth == 0 as libc::c_int as libc::c_uint {
            bandwidthLimit = 0 as libc::c_int as enet_uint32;
        } else {
            while peersRemaining > 0 as libc::c_int as libc::c_uint
                && needsAdjustment != 0 as libc::c_int
            {
                needsAdjustment = 0 as libc::c_int;
                bandwidthLimit = bandwidth.wrapping_div(peersRemaining);
                peer = (*host).peers;
                while peer
                    < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer
                {
                    if !((*peer).state as libc::c_uint
                        != ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
                        && (*peer).state as libc::c_uint
                            != ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
                        || (*peer).incomingBandwidthThrottleEpoch == timeCurrent)
                    {
                        if !((*peer).outgoingBandwidth > 0 as libc::c_int as libc::c_uint
                            && (*peer).outgoingBandwidth >= bandwidthLimit)
                        {
                            (*peer).incomingBandwidthThrottleEpoch = timeCurrent;
                            needsAdjustment = 1 as libc::c_int;
                            peersRemaining = peersRemaining.wrapping_sub(1);
                            bandwidth =
                                (bandwidth as libc::c_uint).wrapping_sub((*peer).outgoingBandwidth)
                                    as enet_uint32 as enet_uint32;
                        }
                    }
                    peer = peer.offset(1);
                }
            }
        }
        peer = (*host).peers;
        while peer < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer {
            if !((*peer).state as libc::c_uint
                != ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
                && (*peer).state as libc::c_uint
                    != ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint)
            {
                command.header.command = (ENET_PROTOCOL_COMMAND_BANDWIDTH_LIMIT as libc::c_int
                    | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int)
                    as enet_uint8;
                command.header.channelID = 0xff as libc::c_int as enet_uint8;
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
                    0 as libc::c_int as enet_uint32,
                    0 as libc::c_int as enet_uint16,
                );
            }
            peer = peer.offset(1);
        }
    }
}
pub unsafe fn enet_list_clear(mut list: *mut ENetList) {
    (*list).sentinel.next = &mut (*list).sentinel;
    (*list).sentinel.previous = &mut (*list).sentinel;
}
pub unsafe fn enet_list_insert(
    mut position: ENetListIterator,
    mut data: *mut libc::c_void,
) -> ENetListIterator {
    let mut result: ENetListIterator = data as ENetListIterator;
    (*result).previous = (*position).previous;
    (*result).next = position;
    (*(*result).previous).next = result;
    (*position).previous = result;
    return result;
}
pub unsafe fn enet_list_remove(mut position: ENetListIterator) -> *mut libc::c_void {
    (*(*position).previous).next = (*position).next;
    (*(*position).next).previous = (*position).previous;
    return position as *mut libc::c_void;
}
pub unsafe fn enet_list_move(
    mut position: ENetListIterator,
    mut dataFirst: *mut libc::c_void,
    mut dataLast: *mut libc::c_void,
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
pub unsafe fn enet_list_size(mut list: *mut ENetList) -> size_t {
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut position: ENetListIterator = 0 as *mut ENetListNode;
    position = (*list).sentinel.next;
    while position != &mut (*list).sentinel as *mut ENetListNode {
        size = size.wrapping_add(1);
        position = (*position).next;
    }
    return size;
}
pub unsafe fn enet_packet_create(
    mut data: *const libc::c_void,
    mut dataLength: size_t,
    mut flags: enet_uint32,
) -> *mut ENetPacket {
    let mut packet: *mut ENetPacket =
        enet_malloc(::core::mem::size_of::<ENetPacket>() as libc::c_ulong) as *mut ENetPacket;
    if packet.is_null() {
        return 0 as *mut ENetPacket;
    }
    if flags & ENET_PACKET_FLAG_NO_ALLOCATE as libc::c_int as libc::c_uint != 0 {
        (*packet).data = data as *mut enet_uint8;
    } else if dataLength <= 0 as libc::c_int as libc::c_ulong {
        (*packet).data = 0 as *mut enet_uint8;
    } else {
        (*packet).data = enet_malloc(dataLength) as *mut enet_uint8;
        if ((*packet).data).is_null() {
            enet_free(packet as *mut libc::c_void);
            return 0 as *mut ENetPacket;
        }
        if !data.is_null() {
            memcpy((*packet).data as *mut libc::c_void, data, dataLength);
        }
    }
    (*packet).referenceCount = 0 as libc::c_int as size_t;
    (*packet).flags = flags;
    (*packet).dataLength = dataLength;
    (*packet).freeCallback = None;
    (*packet).userData = 0 as *mut libc::c_void;
    return packet;
}
pub unsafe fn enet_packet_destroy(mut packet: *mut ENetPacket) {
    if packet.is_null() {
        return;
    }
    if ((*packet).freeCallback).is_some() {
        (Some(((*packet).freeCallback).expect("non-null function pointer")))
            .expect("non-null function pointer")(packet);
    }
    if (*packet).flags & ENET_PACKET_FLAG_NO_ALLOCATE as libc::c_int as libc::c_uint == 0
        && !((*packet).data).is_null()
    {
        enet_free((*packet).data as *mut libc::c_void);
    }
    enet_free(packet as *mut libc::c_void);
}
pub unsafe fn enet_packet_resize(
    mut packet: *mut ENetPacket,
    mut dataLength: size_t,
) -> libc::c_int {
    let mut newData: *mut enet_uint8 = 0 as *mut enet_uint8;
    if dataLength <= (*packet).dataLength
        || (*packet).flags & ENET_PACKET_FLAG_NO_ALLOCATE as libc::c_int as libc::c_uint != 0
    {
        (*packet).dataLength = dataLength;
        return 0 as libc::c_int;
    }
    newData = enet_malloc(dataLength) as *mut enet_uint8;
    if newData.is_null() {
        return -(1 as libc::c_int);
    }
    memcpy(
        newData as *mut libc::c_void,
        (*packet).data as *const libc::c_void,
        (*packet).dataLength,
    );
    enet_free((*packet).data as *mut libc::c_void);
    (*packet).data = newData;
    (*packet).dataLength = dataLength;
    return 0 as libc::c_int;
}
static mut crcTable: [enet_uint32; 256] = [
    0 as libc::c_int as enet_uint32,
    0x77073096 as libc::c_int as enet_uint32,
    0xee0e612c as libc::c_uint,
    0x990951ba as libc::c_uint,
    0x76dc419 as libc::c_int as enet_uint32,
    0x706af48f as libc::c_int as enet_uint32,
    0xe963a535 as libc::c_uint,
    0x9e6495a3 as libc::c_uint,
    0xedb8832 as libc::c_int as enet_uint32,
    0x79dcb8a4 as libc::c_int as enet_uint32,
    0xe0d5e91e as libc::c_uint,
    0x97d2d988 as libc::c_uint,
    0x9b64c2b as libc::c_int as enet_uint32,
    0x7eb17cbd as libc::c_int as enet_uint32,
    0xe7b82d07 as libc::c_uint,
    0x90bf1d91 as libc::c_uint,
    0x1db71064 as libc::c_int as enet_uint32,
    0x6ab020f2 as libc::c_int as enet_uint32,
    0xf3b97148 as libc::c_uint,
    0x84be41de as libc::c_uint,
    0x1adad47d as libc::c_int as enet_uint32,
    0x6ddde4eb as libc::c_int as enet_uint32,
    0xf4d4b551 as libc::c_uint,
    0x83d385c7 as libc::c_uint,
    0x136c9856 as libc::c_int as enet_uint32,
    0x646ba8c0 as libc::c_int as enet_uint32,
    0xfd62f97a as libc::c_uint,
    0x8a65c9ec as libc::c_uint,
    0x14015c4f as libc::c_int as enet_uint32,
    0x63066cd9 as libc::c_int as enet_uint32,
    0xfa0f3d63 as libc::c_uint,
    0x8d080df5 as libc::c_uint,
    0x3b6e20c8 as libc::c_int as enet_uint32,
    0x4c69105e as libc::c_int as enet_uint32,
    0xd56041e4 as libc::c_uint,
    0xa2677172 as libc::c_uint,
    0x3c03e4d1 as libc::c_int as enet_uint32,
    0x4b04d447 as libc::c_int as enet_uint32,
    0xd20d85fd as libc::c_uint,
    0xa50ab56b as libc::c_uint,
    0x35b5a8fa as libc::c_int as enet_uint32,
    0x42b2986c as libc::c_int as enet_uint32,
    0xdbbbc9d6 as libc::c_uint,
    0xacbcf940 as libc::c_uint,
    0x32d86ce3 as libc::c_int as enet_uint32,
    0x45df5c75 as libc::c_int as enet_uint32,
    0xdcd60dcf as libc::c_uint,
    0xabd13d59 as libc::c_uint,
    0x26d930ac as libc::c_int as enet_uint32,
    0x51de003a as libc::c_int as enet_uint32,
    0xc8d75180 as libc::c_uint,
    0xbfd06116 as libc::c_uint,
    0x21b4f4b5 as libc::c_int as enet_uint32,
    0x56b3c423 as libc::c_int as enet_uint32,
    0xcfba9599 as libc::c_uint,
    0xb8bda50f as libc::c_uint,
    0x2802b89e as libc::c_int as enet_uint32,
    0x5f058808 as libc::c_int as enet_uint32,
    0xc60cd9b2 as libc::c_uint,
    0xb10be924 as libc::c_uint,
    0x2f6f7c87 as libc::c_int as enet_uint32,
    0x58684c11 as libc::c_int as enet_uint32,
    0xc1611dab as libc::c_uint,
    0xb6662d3d as libc::c_uint,
    0x76dc4190 as libc::c_int as enet_uint32,
    0x1db7106 as libc::c_int as enet_uint32,
    0x98d220bc as libc::c_uint,
    0xefd5102a as libc::c_uint,
    0x71b18589 as libc::c_int as enet_uint32,
    0x6b6b51f as libc::c_int as enet_uint32,
    0x9fbfe4a5 as libc::c_uint,
    0xe8b8d433 as libc::c_uint,
    0x7807c9a2 as libc::c_int as enet_uint32,
    0xf00f934 as libc::c_int as enet_uint32,
    0x9609a88e as libc::c_uint,
    0xe10e9818 as libc::c_uint,
    0x7f6a0dbb as libc::c_int as enet_uint32,
    0x86d3d2d as libc::c_int as enet_uint32,
    0x91646c97 as libc::c_uint,
    0xe6635c01 as libc::c_uint,
    0x6b6b51f4 as libc::c_int as enet_uint32,
    0x1c6c6162 as libc::c_int as enet_uint32,
    0x856530d8 as libc::c_uint,
    0xf262004e as libc::c_uint,
    0x6c0695ed as libc::c_int as enet_uint32,
    0x1b01a57b as libc::c_int as enet_uint32,
    0x8208f4c1 as libc::c_uint,
    0xf50fc457 as libc::c_uint,
    0x65b0d9c6 as libc::c_int as enet_uint32,
    0x12b7e950 as libc::c_int as enet_uint32,
    0x8bbeb8ea as libc::c_uint,
    0xfcb9887c as libc::c_uint,
    0x62dd1ddf as libc::c_int as enet_uint32,
    0x15da2d49 as libc::c_int as enet_uint32,
    0x8cd37cf3 as libc::c_uint,
    0xfbd44c65 as libc::c_uint,
    0x4db26158 as libc::c_int as enet_uint32,
    0x3ab551ce as libc::c_int as enet_uint32,
    0xa3bc0074 as libc::c_uint,
    0xd4bb30e2 as libc::c_uint,
    0x4adfa541 as libc::c_int as enet_uint32,
    0x3dd895d7 as libc::c_int as enet_uint32,
    0xa4d1c46d as libc::c_uint,
    0xd3d6f4fb as libc::c_uint,
    0x4369e96a as libc::c_int as enet_uint32,
    0x346ed9fc as libc::c_int as enet_uint32,
    0xad678846 as libc::c_uint,
    0xda60b8d0 as libc::c_uint,
    0x44042d73 as libc::c_int as enet_uint32,
    0x33031de5 as libc::c_int as enet_uint32,
    0xaa0a4c5f as libc::c_uint,
    0xdd0d7cc9 as libc::c_uint,
    0x5005713c as libc::c_int as enet_uint32,
    0x270241aa as libc::c_int as enet_uint32,
    0xbe0b1010 as libc::c_uint,
    0xc90c2086 as libc::c_uint,
    0x5768b525 as libc::c_int as enet_uint32,
    0x206f85b3 as libc::c_int as enet_uint32,
    0xb966d409 as libc::c_uint,
    0xce61e49f as libc::c_uint,
    0x5edef90e as libc::c_int as enet_uint32,
    0x29d9c998 as libc::c_int as enet_uint32,
    0xb0d09822 as libc::c_uint,
    0xc7d7a8b4 as libc::c_uint,
    0x59b33d17 as libc::c_int as enet_uint32,
    0x2eb40d81 as libc::c_int as enet_uint32,
    0xb7bd5c3b as libc::c_uint,
    0xc0ba6cad as libc::c_uint,
    0xedb88320 as libc::c_uint,
    0x9abfb3b6 as libc::c_uint,
    0x3b6e20c as libc::c_int as enet_uint32,
    0x74b1d29a as libc::c_int as enet_uint32,
    0xead54739 as libc::c_uint,
    0x9dd277af as libc::c_uint,
    0x4db2615 as libc::c_int as enet_uint32,
    0x73dc1683 as libc::c_int as enet_uint32,
    0xe3630b12 as libc::c_uint,
    0x94643b84 as libc::c_uint,
    0xd6d6a3e as libc::c_int as enet_uint32,
    0x7a6a5aa8 as libc::c_int as enet_uint32,
    0xe40ecf0b as libc::c_uint,
    0x9309ff9d as libc::c_uint,
    0xa00ae27 as libc::c_int as enet_uint32,
    0x7d079eb1 as libc::c_int as enet_uint32,
    0xf00f9344 as libc::c_uint,
    0x8708a3d2 as libc::c_uint,
    0x1e01f268 as libc::c_int as enet_uint32,
    0x6906c2fe as libc::c_int as enet_uint32,
    0xf762575d as libc::c_uint,
    0x806567cb as libc::c_uint,
    0x196c3671 as libc::c_int as enet_uint32,
    0x6e6b06e7 as libc::c_int as enet_uint32,
    0xfed41b76 as libc::c_uint,
    0x89d32be0 as libc::c_uint,
    0x10da7a5a as libc::c_int as enet_uint32,
    0x67dd4acc as libc::c_int as enet_uint32,
    0xf9b9df6f as libc::c_uint,
    0x8ebeeff9 as libc::c_uint,
    0x17b7be43 as libc::c_int as enet_uint32,
    0x60b08ed5 as libc::c_int as enet_uint32,
    0xd6d6a3e8 as libc::c_uint,
    0xa1d1937e as libc::c_uint,
    0x38d8c2c4 as libc::c_int as enet_uint32,
    0x4fdff252 as libc::c_int as enet_uint32,
    0xd1bb67f1 as libc::c_uint,
    0xa6bc5767 as libc::c_uint,
    0x3fb506dd as libc::c_int as enet_uint32,
    0x48b2364b as libc::c_int as enet_uint32,
    0xd80d2bda as libc::c_uint,
    0xaf0a1b4c as libc::c_uint,
    0x36034af6 as libc::c_int as enet_uint32,
    0x41047a60 as libc::c_int as enet_uint32,
    0xdf60efc3 as libc::c_uint,
    0xa867df55 as libc::c_uint,
    0x316e8eef as libc::c_int as enet_uint32,
    0x4669be79 as libc::c_int as enet_uint32,
    0xcb61b38c as libc::c_uint,
    0xbc66831a as libc::c_uint,
    0x256fd2a0 as libc::c_int as enet_uint32,
    0x5268e236 as libc::c_int as enet_uint32,
    0xcc0c7795 as libc::c_uint,
    0xbb0b4703 as libc::c_uint,
    0x220216b9 as libc::c_int as enet_uint32,
    0x5505262f as libc::c_int as enet_uint32,
    0xc5ba3bbe as libc::c_uint,
    0xb2bd0b28 as libc::c_uint,
    0x2bb45a92 as libc::c_int as enet_uint32,
    0x5cb36a04 as libc::c_int as enet_uint32,
    0xc2d7ffa7 as libc::c_uint,
    0xb5d0cf31 as libc::c_uint,
    0x2cd99e8b as libc::c_int as enet_uint32,
    0x5bdeae1d as libc::c_int as enet_uint32,
    0x9b64c2b0 as libc::c_uint,
    0xec63f226 as libc::c_uint,
    0x756aa39c as libc::c_int as enet_uint32,
    0x26d930a as libc::c_int as enet_uint32,
    0x9c0906a9 as libc::c_uint,
    0xeb0e363f as libc::c_uint,
    0x72076785 as libc::c_int as enet_uint32,
    0x5005713 as libc::c_int as enet_uint32,
    0x95bf4a82 as libc::c_uint,
    0xe2b87a14 as libc::c_uint,
    0x7bb12bae as libc::c_int as enet_uint32,
    0xcb61b38 as libc::c_int as enet_uint32,
    0x92d28e9b as libc::c_uint,
    0xe5d5be0d as libc::c_uint,
    0x7cdcefb7 as libc::c_int as enet_uint32,
    0xbdbdf21 as libc::c_int as enet_uint32,
    0x86d3d2d4 as libc::c_uint,
    0xf1d4e242 as libc::c_uint,
    0x68ddb3f8 as libc::c_int as enet_uint32,
    0x1fda836e as libc::c_int as enet_uint32,
    0x81be16cd as libc::c_uint,
    0xf6b9265b as libc::c_uint,
    0x6fb077e1 as libc::c_int as enet_uint32,
    0x18b74777 as libc::c_int as enet_uint32,
    0x88085ae6 as libc::c_uint,
    0xff0f6a70 as libc::c_uint,
    0x66063bca as libc::c_int as enet_uint32,
    0x11010b5c as libc::c_int as enet_uint32,
    0x8f659eff as libc::c_uint,
    0xf862ae69 as libc::c_uint,
    0x616bffd3 as libc::c_int as enet_uint32,
    0x166ccf45 as libc::c_int as enet_uint32,
    0xa00ae278 as libc::c_uint,
    0xd70dd2ee as libc::c_uint,
    0x4e048354 as libc::c_int as enet_uint32,
    0x3903b3c2 as libc::c_int as enet_uint32,
    0xa7672661 as libc::c_uint,
    0xd06016f7 as libc::c_uint,
    0x4969474d as libc::c_int as enet_uint32,
    0x3e6e77db as libc::c_int as enet_uint32,
    0xaed16a4a as libc::c_uint,
    0xd9d65adc as libc::c_uint,
    0x40df0b66 as libc::c_int as enet_uint32,
    0x37d83bf0 as libc::c_int as enet_uint32,
    0xa9bcae53 as libc::c_uint,
    0xdebb9ec5 as libc::c_uint,
    0x47b2cf7f as libc::c_int as enet_uint32,
    0x30b5ffe9 as libc::c_int as enet_uint32,
    0xbdbdf21c as libc::c_uint,
    0xcabac28a as libc::c_uint,
    0x53b39330 as libc::c_int as enet_uint32,
    0x24b4a3a6 as libc::c_int as enet_uint32,
    0xbad03605 as libc::c_uint,
    0xcdd70693 as libc::c_uint,
    0x54de5729 as libc::c_int as enet_uint32,
    0x23d967bf as libc::c_int as enet_uint32,
    0xb3667a2e as libc::c_uint,
    0xc4614ab8 as libc::c_uint,
    0x5d681b02 as libc::c_int as enet_uint32,
    0x2a6f2b94 as libc::c_int as enet_uint32,
    0xb40bbe37 as libc::c_uint,
    0xc30c8ea1 as libc::c_uint,
    0x5a05df1b as libc::c_int as enet_uint32,
    0x2d02ef8d as libc::c_int as enet_uint32,
];
#[no_mangle]
pub unsafe extern "C" fn enet_crc32(
    mut buffers: *const ENetBuffer,
    mut bufferCount: size_t,
) -> enet_uint32 {
    let mut crc: enet_uint32 = 0xffffffff as libc::c_uint;
    loop {
        let fresh30 = bufferCount;
        bufferCount = bufferCount.wrapping_sub(1);
        if !(fresh30 > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        let mut data: *const enet_uint8 = (*buffers).data as *const enet_uint8;
        let mut dataEnd: *const enet_uint8 =
            &*data.offset((*buffers).dataLength as isize) as *const enet_uint8;
        while data < dataEnd {
            let fresh31 = data;
            data = data.offset(1);
            crc = crc >> 8 as libc::c_int
                ^ crcTable[(crc & 0xff as libc::c_int as libc::c_uint ^ *fresh31 as libc::c_uint)
                    as usize];
        }
        buffers = buffers.offset(1);
    }
    return htonl(!crc);
}
pub unsafe fn enet_peer_throttle_configure(
    mut peer: *mut ENetPeer,
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
    command.header.command = (ENET_PROTOCOL_COMMAND_THROTTLE_CONFIGURE as libc::c_int
        | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int)
        as enet_uint8;
    command.header.channelID = 0xff as libc::c_int as enet_uint8;
    command.throttleConfigure.packetThrottleInterval = htonl(interval);
    command.throttleConfigure.packetThrottleAcceleration = htonl(acceleration);
    command.throttleConfigure.packetThrottleDeceleration = htonl(deceleration);
    enet_peer_queue_outgoing_command(
        peer,
        &mut command,
        0 as *mut ENetPacket,
        0 as libc::c_int as enet_uint32,
        0 as libc::c_int as enet_uint16,
    );
}
pub unsafe fn enet_peer_throttle(mut peer: *mut ENetPeer, mut rtt: enet_uint32) -> libc::c_int {
    if (*peer).lastRoundTripTime <= (*peer).lastRoundTripTimeVariance {
        (*peer).packetThrottle = (*peer).packetThrottleLimit;
    } else if rtt <= (*peer).lastRoundTripTime {
        (*peer).packetThrottle = ((*peer).packetThrottle as libc::c_uint)
            .wrapping_add((*peer).packetThrottleAcceleration)
            as enet_uint32 as enet_uint32;
        if (*peer).packetThrottle > (*peer).packetThrottleLimit {
            (*peer).packetThrottle = (*peer).packetThrottleLimit;
        }
        return 1 as libc::c_int;
    } else {
        if rtt
            > ((*peer).lastRoundTripTime).wrapping_add(
                (2 as libc::c_int as libc::c_uint).wrapping_mul((*peer).lastRoundTripTimeVariance),
            )
        {
            if (*peer).packetThrottle > (*peer).packetThrottleDeceleration {
                (*peer).packetThrottle = ((*peer).packetThrottle as libc::c_uint)
                    .wrapping_sub((*peer).packetThrottleDeceleration)
                    as enet_uint32 as enet_uint32;
            } else {
                (*peer).packetThrottle = 0 as libc::c_int as enet_uint32;
            }
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe fn enet_peer_send(
    mut peer: *mut ENetPeer,
    mut channelID: enet_uint8,
    mut packet: *mut ENetPacket,
) -> libc::c_int {
    let mut channel: *mut ENetChannel = 0 as *mut ENetChannel;
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    let mut fragmentLength: size_t = 0;
    if (*peer).state as libc::c_uint != ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
        || channelID as libc::c_ulong >= (*peer).channelCount
        || (*packet).dataLength > (*(*peer).host).maximumPacketSize
    {
        return -(1 as libc::c_int);
    }
    channel = &mut *((*peer).channels).offset(channelID as isize) as *mut ENetChannel;
    fragmentLength = ((*peer).mtu as libc::c_ulong)
        .wrapping_sub(::core::mem::size_of::<ENetProtocolHeader>() as libc::c_ulong)
        .wrapping_sub(::core::mem::size_of::<ENetProtocolSendFragment>() as libc::c_ulong);
    if ((*(*peer).host).checksum).is_some() {
        fragmentLength = (fragmentLength as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<enet_uint32>() as libc::c_ulong)
            as size_t as size_t;
    }
    if (*packet).dataLength > fragmentLength {
        let mut fragmentCount: enet_uint32 = ((*packet).dataLength)
            .wrapping_add(fragmentLength)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
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
        if fragmentCount > ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as libc::c_int as libc::c_uint {
            return -(1 as libc::c_int);
        }
        if (*packet).flags
            & (ENET_PACKET_FLAG_RELIABLE as libc::c_int
                | ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT as libc::c_int) as libc::c_uint
            == ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT as libc::c_int as libc::c_uint
            && ((*channel).outgoingUnreliableSequenceNumber as libc::c_int) < 0xffff as libc::c_int
        {
            commandNumber =
                ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT as libc::c_int as enet_uint8;
            startSequenceNumber = htons(
                ((*channel).outgoingUnreliableSequenceNumber as libc::c_int + 1 as libc::c_int)
                    as uint16_t,
            );
        } else {
            commandNumber = (ENET_PROTOCOL_COMMAND_SEND_FRAGMENT as libc::c_int
                | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int)
                as enet_uint8;
            startSequenceNumber = htons(
                ((*channel).outgoingReliableSequenceNumber as libc::c_int + 1 as libc::c_int)
                    as uint16_t,
            );
        }
        enet_list_clear(&mut fragments);
        fragmentNumber = 0 as libc::c_int as enet_uint32;
        fragmentOffset = 0 as libc::c_int as enet_uint32;
        while (fragmentOffset as libc::c_ulong) < (*packet).dataLength {
            if ((*packet).dataLength).wrapping_sub(fragmentOffset as libc::c_ulong) < fragmentLength
            {
                fragmentLength =
                    ((*packet).dataLength).wrapping_sub(fragmentOffset as libc::c_ulong);
            }
            fragment = enet_malloc(::core::mem::size_of::<ENetOutgoingCommand>() as libc::c_ulong)
                as *mut ENetOutgoingCommand;
            if fragment.is_null() {
                while !(fragments.sentinel.next == &mut fragments.sentinel as *mut ENetListNode) {
                    fragment =
                        enet_list_remove(fragments.sentinel.next) as *mut ENetOutgoingCommand;
                    enet_free(fragment as *mut libc::c_void);
                }
                return -(1 as libc::c_int);
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
            enet_list_insert(&mut fragments.sentinel, fragment as *mut libc::c_void);
            fragmentNumber = fragmentNumber.wrapping_add(1);
            fragmentOffset = (fragmentOffset as libc::c_ulong).wrapping_add(fragmentLength)
                as enet_uint32 as enet_uint32;
        }
        (*packet).referenceCount = ((*packet).referenceCount as libc::c_ulong)
            .wrapping_add(fragmentNumber as libc::c_ulong)
            as size_t as size_t;
        while !(fragments.sentinel.next == &mut fragments.sentinel as *mut ENetListNode) {
            fragment = enet_list_remove(fragments.sentinel.next) as *mut ENetOutgoingCommand;
            enet_peer_setup_outgoing_command(peer, fragment);
        }
        return 0 as libc::c_int;
    }
    command.header.channelID = channelID;
    if (*packet).flags
        & (ENET_PACKET_FLAG_RELIABLE as libc::c_int | ENET_PACKET_FLAG_UNSEQUENCED as libc::c_int)
            as libc::c_uint
        == ENET_PACKET_FLAG_UNSEQUENCED as libc::c_int as libc::c_uint
    {
        command.header.command = (ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as libc::c_int
            | ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED as libc::c_int)
            as enet_uint8;
        command.sendUnsequenced.dataLength = htons((*packet).dataLength as uint16_t);
    } else if (*packet).flags & ENET_PACKET_FLAG_RELIABLE as libc::c_int as libc::c_uint != 0
        || (*channel).outgoingUnreliableSequenceNumber as libc::c_int >= 0xffff as libc::c_int
    {
        command.header.command = (ENET_PROTOCOL_COMMAND_SEND_RELIABLE as libc::c_int
            | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int)
            as enet_uint8;
        command.sendReliable.dataLength = htons((*packet).dataLength as uint16_t);
    } else {
        command.header.command = ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE as libc::c_int as enet_uint8;
        command.sendUnreliable.dataLength = htons((*packet).dataLength as uint16_t);
    }
    if (enet_peer_queue_outgoing_command(
        peer,
        &mut command,
        packet,
        0 as libc::c_int as enet_uint32,
        (*packet).dataLength as enet_uint16,
    ))
    .is_null()
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe fn enet_peer_receive(
    mut peer: *mut ENetPeer,
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
        enet_free((*incomingCommand).fragments as *mut libc::c_void);
    }
    enet_free(incomingCommand as *mut libc::c_void);
    (*peer).totalWaitingData = ((*peer).totalWaitingData as libc::c_ulong)
        .wrapping_sub((*packet).dataLength) as size_t as size_t;
    return packet;
}
unsafe fn enet_peer_reset_outgoing_commands(mut queue: *mut ENetList) {
    let mut outgoingCommand: *mut ENetOutgoingCommand = 0 as *mut ENetOutgoingCommand;
    while !((*queue).sentinel.next == &mut (*queue).sentinel as *mut ENetListNode) {
        outgoingCommand = enet_list_remove((*queue).sentinel.next) as *mut ENetOutgoingCommand;
        if !((*outgoingCommand).packet).is_null() {
            (*(*outgoingCommand).packet).referenceCount =
                ((*(*outgoingCommand).packet).referenceCount).wrapping_sub(1);
            if (*(*outgoingCommand).packet).referenceCount == 0 as libc::c_int as libc::c_ulong {
                enet_packet_destroy((*outgoingCommand).packet);
            }
        }
        enet_free(outgoingCommand as *mut libc::c_void);
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
            if (*(*incomingCommand).packet).referenceCount == 0 as libc::c_int as libc::c_ulong {
                enet_packet_destroy((*incomingCommand).packet);
            }
        }
        if !((*incomingCommand).fragments).is_null() {
            enet_free((*incomingCommand).fragments as *mut libc::c_void);
        }
        enet_free(incomingCommand as *mut libc::c_void);
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
pub unsafe fn enet_peer_reset_queues(mut peer: *mut ENetPeer) {
    let mut channel: *mut ENetChannel = 0 as *mut ENetChannel;
    if (*peer).flags as libc::c_int & ENET_PEER_FLAG_NEEDS_DISPATCH as libc::c_int != 0 {
        enet_list_remove(&mut (*peer).dispatchList);
        (*peer).flags = ((*peer).flags as libc::c_int
            & !(ENET_PEER_FLAG_NEEDS_DISPATCH as libc::c_int))
            as enet_uint16;
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
    if !((*peer).channels).is_null() && (*peer).channelCount > 0 as libc::c_int as libc::c_ulong {
        channel = (*peer).channels;
        while channel
            < &mut *((*peer).channels).offset((*peer).channelCount as isize) as *mut ENetChannel
        {
            enet_peer_reset_incoming_commands(&mut (*channel).incomingReliableCommands);
            enet_peer_reset_incoming_commands(&mut (*channel).incomingUnreliableCommands);
            channel = channel.offset(1);
        }
        enet_free((*peer).channels as *mut libc::c_void);
    }
    (*peer).channels = 0 as *mut ENetChannel;
    (*peer).channelCount = 0 as libc::c_int as size_t;
}
pub unsafe fn enet_peer_on_connect(mut peer: *mut ENetPeer) {
    if (*peer).state as libc::c_uint != ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
        && (*peer).state as libc::c_uint
            != ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
    {
        if (*peer).incomingBandwidth != 0 as libc::c_int as libc::c_uint {
            (*(*peer).host).bandwidthLimitedPeers =
                ((*(*peer).host).bandwidthLimitedPeers).wrapping_add(1);
        }
        (*(*peer).host).connectedPeers = ((*(*peer).host).connectedPeers).wrapping_add(1);
    }
}
pub unsafe fn enet_peer_on_disconnect(mut peer: *mut ENetPeer) {
    if (*peer).state as libc::c_uint == ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
        || (*peer).state as libc::c_uint
            == ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
    {
        if (*peer).incomingBandwidth != 0 as libc::c_int as libc::c_uint {
            (*(*peer).host).bandwidthLimitedPeers =
                ((*(*peer).host).bandwidthLimitedPeers).wrapping_sub(1);
        }
        (*(*peer).host).connectedPeers = ((*(*peer).host).connectedPeers).wrapping_sub(1);
    }
}
pub unsafe fn enet_peer_reset(mut peer: *mut ENetPeer) {
    enet_peer_on_disconnect(peer);
    (*peer).outgoingPeerID = ENET_PROTOCOL_MAXIMUM_PEER_ID as libc::c_int as enet_uint16;
    (*peer).connectID = 0 as libc::c_int as enet_uint32;
    (*peer).state = ENET_PEER_STATE_DISCONNECTED;
    (*peer).incomingBandwidth = 0 as libc::c_int as enet_uint32;
    (*peer).outgoingBandwidth = 0 as libc::c_int as enet_uint32;
    (*peer).incomingBandwidthThrottleEpoch = 0 as libc::c_int as enet_uint32;
    (*peer).outgoingBandwidthThrottleEpoch = 0 as libc::c_int as enet_uint32;
    (*peer).incomingDataTotal = 0 as libc::c_int as enet_uint32;
    (*peer).outgoingDataTotal = 0 as libc::c_int as enet_uint32;
    (*peer).lastSendTime = 0 as libc::c_int as enet_uint32;
    (*peer).lastReceiveTime = 0 as libc::c_int as enet_uint32;
    (*peer).nextTimeout = 0 as libc::c_int as enet_uint32;
    (*peer).earliestTimeout = 0 as libc::c_int as enet_uint32;
    (*peer).packetLossEpoch = 0 as libc::c_int as enet_uint32;
    (*peer).packetsSent = 0 as libc::c_int as enet_uint32;
    (*peer).packetsLost = 0 as libc::c_int as enet_uint32;
    (*peer).packetLoss = 0 as libc::c_int as enet_uint32;
    (*peer).packetLossVariance = 0 as libc::c_int as enet_uint32;
    (*peer).packetThrottle = ENET_PEER_DEFAULT_PACKET_THROTTLE as libc::c_int as enet_uint32;
    (*peer).packetThrottleLimit = ENET_PEER_PACKET_THROTTLE_SCALE as libc::c_int as enet_uint32;
    (*peer).packetThrottleCounter = 0 as libc::c_int as enet_uint32;
    (*peer).packetThrottleEpoch = 0 as libc::c_int as enet_uint32;
    (*peer).packetThrottleAcceleration =
        ENET_PEER_PACKET_THROTTLE_ACCELERATION as libc::c_int as enet_uint32;
    (*peer).packetThrottleDeceleration =
        ENET_PEER_PACKET_THROTTLE_DECELERATION as libc::c_int as enet_uint32;
    (*peer).packetThrottleInterval =
        ENET_PEER_PACKET_THROTTLE_INTERVAL as libc::c_int as enet_uint32;
    (*peer).pingInterval = ENET_PEER_PING_INTERVAL as libc::c_int as enet_uint32;
    (*peer).timeoutLimit = ENET_PEER_TIMEOUT_LIMIT as libc::c_int as enet_uint32;
    (*peer).timeoutMinimum = ENET_PEER_TIMEOUT_MINIMUM as libc::c_int as enet_uint32;
    (*peer).timeoutMaximum = ENET_PEER_TIMEOUT_MAXIMUM as libc::c_int as enet_uint32;
    (*peer).lastRoundTripTime = ENET_PEER_DEFAULT_ROUND_TRIP_TIME as libc::c_int as enet_uint32;
    (*peer).lowestRoundTripTime = ENET_PEER_DEFAULT_ROUND_TRIP_TIME as libc::c_int as enet_uint32;
    (*peer).lastRoundTripTimeVariance = 0 as libc::c_int as enet_uint32;
    (*peer).highestRoundTripTimeVariance = 0 as libc::c_int as enet_uint32;
    (*peer).roundTripTime = ENET_PEER_DEFAULT_ROUND_TRIP_TIME as libc::c_int as enet_uint32;
    (*peer).roundTripTimeVariance = 0 as libc::c_int as enet_uint32;
    (*peer).mtu = (*(*peer).host).mtu;
    (*peer).reliableDataInTransit = 0 as libc::c_int as enet_uint32;
    (*peer).outgoingReliableSequenceNumber = 0 as libc::c_int as enet_uint16;
    (*peer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as enet_uint32;
    (*peer).incomingUnsequencedGroup = 0 as libc::c_int as enet_uint16;
    (*peer).outgoingUnsequencedGroup = 0 as libc::c_int as enet_uint16;
    (*peer).eventData = 0 as libc::c_int as enet_uint32;
    (*peer).totalWaitingData = 0 as libc::c_int as size_t;
    (*peer).flags = 0 as libc::c_int as enet_uint16;
    memset(
        ((*peer).unsequencedWindow).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[enet_uint32; 32]>() as libc::c_ulong,
    );
    enet_peer_reset_queues(peer);
}
pub unsafe fn enet_peer_ping(mut peer: *mut ENetPeer) {
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    if (*peer).state as libc::c_uint != ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint {
        return;
    }
    command.header.command = (ENET_PROTOCOL_COMMAND_PING as libc::c_int
        | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int)
        as enet_uint8;
    command.header.channelID = 0xff as libc::c_int as enet_uint8;
    enet_peer_queue_outgoing_command(
        peer,
        &mut command,
        0 as *mut ENetPacket,
        0 as libc::c_int as enet_uint32,
        0 as libc::c_int as enet_uint16,
    );
}
pub unsafe fn enet_peer_ping_interval(mut peer: *mut ENetPeer, mut pingInterval: enet_uint32) {
    (*peer).pingInterval = if pingInterval != 0 {
        pingInterval
    } else {
        ENET_PEER_PING_INTERVAL as libc::c_int as libc::c_uint
    };
}
pub unsafe fn enet_peer_timeout(
    mut peer: *mut ENetPeer,
    mut timeoutLimit: enet_uint32,
    mut timeoutMinimum: enet_uint32,
    mut timeoutMaximum: enet_uint32,
) {
    (*peer).timeoutLimit = if timeoutLimit != 0 {
        timeoutLimit
    } else {
        ENET_PEER_TIMEOUT_LIMIT as libc::c_int as libc::c_uint
    };
    (*peer).timeoutMinimum = if timeoutMinimum != 0 {
        timeoutMinimum
    } else {
        ENET_PEER_TIMEOUT_MINIMUM as libc::c_int as libc::c_uint
    };
    (*peer).timeoutMaximum = if timeoutMaximum != 0 {
        timeoutMaximum
    } else {
        ENET_PEER_TIMEOUT_MAXIMUM as libc::c_int as libc::c_uint
    };
}
pub unsafe fn enet_peer_disconnect_now(mut peer: *mut ENetPeer, mut data: enet_uint32) {
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    if (*peer).state as libc::c_uint == ENET_PEER_STATE_DISCONNECTED as libc::c_int as libc::c_uint
    {
        return;
    }
    if (*peer).state as libc::c_uint != ENET_PEER_STATE_ZOMBIE as libc::c_int as libc::c_uint
        && (*peer).state as libc::c_uint
            != ENET_PEER_STATE_DISCONNECTING as libc::c_int as libc::c_uint
    {
        enet_peer_reset_queues(peer);
        command.header.command = (ENET_PROTOCOL_COMMAND_DISCONNECT as libc::c_int
            | ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED as libc::c_int)
            as enet_uint8;
        command.header.channelID = 0xff as libc::c_int as enet_uint8;
        command.disconnect.data = htonl(data);
        enet_peer_queue_outgoing_command(
            peer,
            &mut command,
            0 as *mut ENetPacket,
            0 as libc::c_int as enet_uint32,
            0 as libc::c_int as enet_uint16,
        );
        enet_host_flush((*peer).host);
    }
    enet_peer_reset(peer);
}
pub unsafe fn enet_peer_disconnect(mut peer: *mut ENetPeer, mut data: enet_uint32) {
    let mut command: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    if (*peer).state as libc::c_uint == ENET_PEER_STATE_DISCONNECTING as libc::c_int as libc::c_uint
        || (*peer).state as libc::c_uint
            == ENET_PEER_STATE_DISCONNECTED as libc::c_int as libc::c_uint
        || (*peer).state as libc::c_uint
            == ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT as libc::c_int as libc::c_uint
        || (*peer).state as libc::c_uint == ENET_PEER_STATE_ZOMBIE as libc::c_int as libc::c_uint
    {
        return;
    }
    enet_peer_reset_queues(peer);
    command.header.command = ENET_PROTOCOL_COMMAND_DISCONNECT as libc::c_int as enet_uint8;
    command.header.channelID = 0xff as libc::c_int as enet_uint8;
    command.disconnect.data = htonl(data);
    if (*peer).state as libc::c_uint == ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
        || (*peer).state as libc::c_uint
            == ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
    {
        command.header.command = (command.header.command as libc::c_int
            | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int)
            as enet_uint8;
    } else {
        command.header.command = (command.header.command as libc::c_int
            | ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED as libc::c_int)
            as enet_uint8;
    }
    enet_peer_queue_outgoing_command(
        peer,
        &mut command,
        0 as *mut ENetPacket,
        0 as libc::c_int as enet_uint32,
        0 as libc::c_int as enet_uint16,
    );
    if (*peer).state as libc::c_uint == ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
        || (*peer).state as libc::c_uint
            == ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
    {
        enet_peer_on_disconnect(peer);
        (*peer).state = ENET_PEER_STATE_DISCONNECTING;
    } else {
        enet_host_flush((*peer).host);
        enet_peer_reset(peer);
    };
}
pub unsafe fn enet_peer_has_outgoing_commands(mut peer: *mut ENetPeer) -> libc::c_int {
    if (*peer).outgoingCommands.sentinel.next
        == &mut (*peer).outgoingCommands.sentinel as *mut ENetListNode
        && (*peer).outgoingSendReliableCommands.sentinel.next
            == &mut (*peer).outgoingSendReliableCommands.sentinel as *mut ENetListNode
        && (*peer).sentReliableCommands.sentinel.next
            == &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe fn enet_peer_disconnect_later(mut peer: *mut ENetPeer, mut data: enet_uint32) {
    if ((*peer).state as libc::c_uint == ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
        || (*peer).state as libc::c_uint
            == ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint)
        && enet_peer_has_outgoing_commands(peer) != 0
    {
        (*peer).state = ENET_PEER_STATE_DISCONNECT_LATER;
        (*peer).eventData = data;
    } else {
        enet_peer_disconnect(peer, data);
    };
}
pub unsafe fn enet_peer_queue_acknowledgement(
    mut peer: *mut ENetPeer,
    mut command: *const ENetProtocol,
    mut sentTime: enet_uint16,
) -> *mut ENetAcknowledgement {
    let mut acknowledgement: *mut ENetAcknowledgement = 0 as *mut ENetAcknowledgement;
    if ((*command).header.channelID as libc::c_ulong) < (*peer).channelCount {
        let mut channel: *mut ENetChannel = &mut *((*peer).channels)
            .offset((*command).header.channelID as isize)
            as *mut ENetChannel;
        let mut reliableWindow: enet_uint16 =
            ((*command).header.reliableSequenceNumber as libc::c_int
                / ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int) as enet_uint16;
        let mut currentWindow: enet_uint16 =
            ((*channel).incomingReliableSequenceNumber as libc::c_int
                / ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int) as enet_uint16;
        if ((*command).header.reliableSequenceNumber as libc::c_int)
            < (*channel).incomingReliableSequenceNumber as libc::c_int
        {
            reliableWindow = (reliableWindow as libc::c_int
                + ENET_PEER_RELIABLE_WINDOWS as libc::c_int)
                as enet_uint16;
        }
        if reliableWindow as libc::c_int
            >= currentWindow as libc::c_int + ENET_PEER_FREE_RELIABLE_WINDOWS as libc::c_int
                - 1 as libc::c_int
            && reliableWindow as libc::c_int
                <= currentWindow as libc::c_int + ENET_PEER_FREE_RELIABLE_WINDOWS as libc::c_int
        {
            return 0 as *mut ENetAcknowledgement;
        }
    }
    acknowledgement = enet_malloc(::core::mem::size_of::<ENetAcknowledgement>() as libc::c_ulong)
        as *mut ENetAcknowledgement;
    if acknowledgement.is_null() {
        return 0 as *mut ENetAcknowledgement;
    }
    (*peer).outgoingDataTotal = ((*peer).outgoingDataTotal as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<ENetProtocolAcknowledge>() as libc::c_ulong)
        as enet_uint32 as enet_uint32;
    (*acknowledgement).sentTime = sentTime as enet_uint32;
    (*acknowledgement).command = *command;
    enet_list_insert(
        &mut (*peer).acknowledgements.sentinel,
        acknowledgement as *mut libc::c_void,
    );
    return acknowledgement;
}
pub unsafe fn enet_peer_setup_outgoing_command(
    mut peer: *mut ENetPeer,
    mut outgoingCommand: *mut ENetOutgoingCommand,
) {
    (*peer).outgoingDataTotal = ((*peer).outgoingDataTotal as libc::c_ulong).wrapping_add(
        (enet_protocol_command_size((*outgoingCommand).command.header.command))
            .wrapping_add((*outgoingCommand).fragmentLength as libc::c_ulong),
    ) as enet_uint32 as enet_uint32;
    if (*outgoingCommand).command.header.channelID as libc::c_int == 0xff as libc::c_int {
        (*peer).outgoingReliableSequenceNumber =
            ((*peer).outgoingReliableSequenceNumber).wrapping_add(1);
        (*outgoingCommand).reliableSequenceNumber = (*peer).outgoingReliableSequenceNumber;
        (*outgoingCommand).unreliableSequenceNumber = 0 as libc::c_int as enet_uint16;
    } else {
        let mut channel: *mut ENetChannel = &mut *((*peer).channels)
            .offset((*outgoingCommand).command.header.channelID as isize)
            as *mut ENetChannel;
        if (*outgoingCommand).command.header.command as libc::c_int
            & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int
            != 0
        {
            (*channel).outgoingReliableSequenceNumber =
                ((*channel).outgoingReliableSequenceNumber).wrapping_add(1);
            (*channel).outgoingUnreliableSequenceNumber = 0 as libc::c_int as enet_uint16;
            (*outgoingCommand).reliableSequenceNumber = (*channel).outgoingReliableSequenceNumber;
            (*outgoingCommand).unreliableSequenceNumber = 0 as libc::c_int as enet_uint16;
        } else if (*outgoingCommand).command.header.command as libc::c_int
            & ENET_PROTOCOL_COMMAND_FLAG_UNSEQUENCED as libc::c_int
            != 0
        {
            (*peer).outgoingUnsequencedGroup = ((*peer).outgoingUnsequencedGroup).wrapping_add(1);
            (*outgoingCommand).reliableSequenceNumber = 0 as libc::c_int as enet_uint16;
            (*outgoingCommand).unreliableSequenceNumber = 0 as libc::c_int as enet_uint16;
        } else {
            if (*outgoingCommand).fragmentOffset == 0 as libc::c_int as libc::c_uint {
                (*channel).outgoingUnreliableSequenceNumber =
                    ((*channel).outgoingUnreliableSequenceNumber).wrapping_add(1);
            }
            (*outgoingCommand).reliableSequenceNumber = (*channel).outgoingReliableSequenceNumber;
            (*outgoingCommand).unreliableSequenceNumber =
                (*channel).outgoingUnreliableSequenceNumber;
        }
    }
    (*outgoingCommand).sendAttempts = 0 as libc::c_int as enet_uint16;
    (*outgoingCommand).sentTime = 0 as libc::c_int as enet_uint32;
    (*outgoingCommand).roundTripTimeout = 0 as libc::c_int as enet_uint32;
    (*outgoingCommand).command.header.reliableSequenceNumber =
        htons((*outgoingCommand).reliableSequenceNumber);
    (*(*peer).host).totalQueued = ((*(*peer).host).totalQueued).wrapping_add(1);
    (*outgoingCommand).queueTime = (*(*peer).host).totalQueued;
    match (*outgoingCommand).command.header.command as libc::c_int
        & ENET_PROTOCOL_COMMAND_MASK as libc::c_int
    {
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
    if (*outgoingCommand).command.header.command as libc::c_int
        & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int
        != 0 as libc::c_int
        && !((*outgoingCommand).packet).is_null()
    {
        enet_list_insert(
            &mut (*peer).outgoingSendReliableCommands.sentinel,
            outgoingCommand as *mut libc::c_void,
        );
    } else {
        enet_list_insert(
            &mut (*peer).outgoingCommands.sentinel,
            outgoingCommand as *mut libc::c_void,
        );
    };
}
pub unsafe fn enet_peer_queue_outgoing_command(
    mut peer: *mut ENetPeer,
    mut command: *const ENetProtocol,
    mut packet: *mut ENetPacket,
    mut offset: enet_uint32,
    mut length: enet_uint16,
) -> *mut ENetOutgoingCommand {
    let mut outgoingCommand: *mut ENetOutgoingCommand =
        enet_malloc(::core::mem::size_of::<ENetOutgoingCommand>() as libc::c_ulong)
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
pub unsafe fn enet_peer_dispatch_incoming_unreliable_commands(
    mut peer: *mut ENetPeer,
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
        if !((*incomingCommand).command.header.command as libc::c_int
            & ENET_PROTOCOL_COMMAND_MASK as libc::c_int
            == ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as libc::c_int)
        {
            if (*incomingCommand).reliableSequenceNumber as libc::c_int
                == (*channel).incomingReliableSequenceNumber as libc::c_int
            {
                if (*incomingCommand).fragmentsRemaining <= 0 as libc::c_int as libc::c_uint {
                    (*channel).incomingUnreliableSequenceNumber =
                        (*incomingCommand).unreliableSequenceNumber;
                    current_block_22 = 11174649648027449784;
                } else {
                    if startCommand != currentCommand {
                        enet_list_move(
                            &mut (*peer).dispatchedCommands.sentinel,
                            startCommand as *mut libc::c_void,
                            (*currentCommand).previous as *mut libc::c_void,
                        );
                        if (*peer).flags as libc::c_int
                            & ENET_PEER_FLAG_NEEDS_DISPATCH as libc::c_int
                            == 0
                        {
                            enet_list_insert(
                                &mut (*(*peer).host).dispatchQueue.sentinel,
                                &mut (*peer).dispatchList as *mut ENetListNode as *mut libc::c_void,
                            );
                            (*peer).flags = ((*peer).flags as libc::c_int
                                | ENET_PEER_FLAG_NEEDS_DISPATCH as libc::c_int)
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
                    as libc::c_int
                    / ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int)
                    as enet_uint16;
                let mut currentWindow: enet_uint16 = ((*channel).incomingReliableSequenceNumber
                    as libc::c_int
                    / ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int)
                    as enet_uint16;
                if ((*incomingCommand).reliableSequenceNumber as libc::c_int)
                    < (*channel).incomingReliableSequenceNumber as libc::c_int
                {
                    reliableWindow = (reliableWindow as libc::c_int
                        + ENET_PEER_RELIABLE_WINDOWS as libc::c_int)
                        as enet_uint16;
                }
                if reliableWindow as libc::c_int >= currentWindow as libc::c_int
                    && (reliableWindow as libc::c_int)
                        < currentWindow as libc::c_int
                            + ENET_PEER_FREE_RELIABLE_WINDOWS as libc::c_int
                            - 1 as libc::c_int
                {
                    break;
                }
                droppedCommand = (*currentCommand).next;
                if startCommand != currentCommand {
                    enet_list_move(
                        &mut (*peer).dispatchedCommands.sentinel,
                        startCommand as *mut libc::c_void,
                        (*currentCommand).previous as *mut libc::c_void,
                    );
                    if (*peer).flags as libc::c_int & ENET_PEER_FLAG_NEEDS_DISPATCH as libc::c_int
                        == 0
                    {
                        enet_list_insert(
                            &mut (*(*peer).host).dispatchQueue.sentinel,
                            &mut (*peer).dispatchList as *mut ENetListNode as *mut libc::c_void,
                        );
                        (*peer).flags = ((*peer).flags as libc::c_int
                            | ENET_PEER_FLAG_NEEDS_DISPATCH as libc::c_int)
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
            startCommand as *mut libc::c_void,
            (*currentCommand).previous as *mut libc::c_void,
        );
        if (*peer).flags as libc::c_int & ENET_PEER_FLAG_NEEDS_DISPATCH as libc::c_int == 0 {
            enet_list_insert(
                &mut (*(*peer).host).dispatchQueue.sentinel,
                &mut (*peer).dispatchList as *mut ENetListNode as *mut libc::c_void,
            );
            (*peer).flags = ((*peer).flags as libc::c_int
                | ENET_PEER_FLAG_NEEDS_DISPATCH as libc::c_int)
                as enet_uint16;
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
pub unsafe fn enet_peer_dispatch_incoming_reliable_commands(
    mut peer: *mut ENetPeer,
    mut channel: *mut ENetChannel,
    mut queuedCommand: *mut ENetIncomingCommand,
) {
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    currentCommand = (*channel).incomingReliableCommands.sentinel.next;
    while currentCommand != &mut (*channel).incomingReliableCommands.sentinel as *mut ENetListNode {
        let mut incomingCommand: *mut ENetIncomingCommand =
            currentCommand as *mut ENetIncomingCommand;
        if (*incomingCommand).fragmentsRemaining > 0 as libc::c_int as libc::c_uint
            || (*incomingCommand).reliableSequenceNumber as libc::c_int
                != ((*channel).incomingReliableSequenceNumber as libc::c_int + 1 as libc::c_int)
                    as enet_uint16 as libc::c_int
        {
            break;
        }
        (*channel).incomingReliableSequenceNumber = (*incomingCommand).reliableSequenceNumber;
        if (*incomingCommand).fragmentCount > 0 as libc::c_int as libc::c_uint {
            (*channel).incomingReliableSequenceNumber =
                ((*channel).incomingReliableSequenceNumber as libc::c_uint).wrapping_add(
                    ((*incomingCommand).fragmentCount)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                ) as enet_uint16 as enet_uint16;
        }
        currentCommand = (*currentCommand).next;
    }
    if currentCommand == (*channel).incomingReliableCommands.sentinel.next {
        return;
    }
    (*channel).incomingUnreliableSequenceNumber = 0 as libc::c_int as enet_uint16;
    enet_list_move(
        &mut (*peer).dispatchedCommands.sentinel,
        (*channel).incomingReliableCommands.sentinel.next as *mut libc::c_void,
        (*currentCommand).previous as *mut libc::c_void,
    );
    if (*peer).flags as libc::c_int & ENET_PEER_FLAG_NEEDS_DISPATCH as libc::c_int == 0 {
        enet_list_insert(
            &mut (*(*peer).host).dispatchQueue.sentinel,
            &mut (*peer).dispatchList as *mut ENetListNode as *mut libc::c_void,
        );
        (*peer).flags = ((*peer).flags as libc::c_int
            | ENET_PEER_FLAG_NEEDS_DISPATCH as libc::c_int) as enet_uint16;
    }
    if !((*channel).incomingUnreliableCommands.sentinel.next
        == &mut (*channel).incomingUnreliableCommands.sentinel as *mut ENetListNode)
    {
        enet_peer_dispatch_incoming_unreliable_commands(peer, channel, queuedCommand);
    }
}
pub unsafe fn enet_peer_queue_incoming_command(
    mut peer: *mut ENetPeer,
    mut command: *const ENetProtocol,
    mut data: *const libc::c_void,
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
    let mut unreliableSequenceNumber: enet_uint32 = 0 as libc::c_int as enet_uint32;
    let mut reliableSequenceNumber: enet_uint32 = 0 as libc::c_int as enet_uint32;
    let mut reliableWindow: enet_uint16 = 0;
    let mut currentWindow: enet_uint16 = 0;
    let mut incomingCommand: *mut ENetIncomingCommand = 0 as *mut ENetIncomingCommand;
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut packet: *mut ENetPacket = 0 as *mut ENetPacket;
    if (*peer).state as libc::c_uint
        == ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
    {
        current_block = 9207730764507465628;
    } else {
        if (*command).header.command as libc::c_int & ENET_PROTOCOL_COMMAND_MASK as libc::c_int
            != ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED as libc::c_int
        {
            reliableSequenceNumber = (*command).header.reliableSequenceNumber as enet_uint32;
            reliableWindow = reliableSequenceNumber
                .wrapping_div(ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int as libc::c_uint)
                as enet_uint16;
            currentWindow = ((*channel).incomingReliableSequenceNumber as libc::c_int
                / ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int)
                as enet_uint16;
            if reliableSequenceNumber < (*channel).incomingReliableSequenceNumber as libc::c_uint {
                reliableWindow = (reliableWindow as libc::c_int
                    + ENET_PEER_RELIABLE_WINDOWS as libc::c_int)
                    as enet_uint16;
            }
            if (reliableWindow as libc::c_int) < currentWindow as libc::c_int
                || reliableWindow as libc::c_int
                    >= currentWindow as libc::c_int + ENET_PEER_FREE_RELIABLE_WINDOWS as libc::c_int
                        - 1 as libc::c_int
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
            _ => {
                match (*command).header.command as libc::c_int
                    & ENET_PROTOCOL_COMMAND_MASK as libc::c_int
                {
                    8 | 6 => {
                        current_block = 4379360700607281851;
                        match current_block {
                            10107555224945550073 => {
                                currentCommand =
                                    &mut (*channel).incomingUnreliableCommands.sentinel;
                                current_block = 7746103178988627676;
                            }
                            4379360700607281851 => {
                                if reliableSequenceNumber
                                    == (*channel).incomingReliableSequenceNumber as libc::c_uint
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
                                        incomingCommand =
                                            currentCommand as *mut ENetIncomingCommand;
                                        if reliableSequenceNumber
                                            >= (*channel).incomingReliableSequenceNumber
                                                as libc::c_uint
                                        {
                                            if ((*incomingCommand).reliableSequenceNumber
                                                as libc::c_int)
                                                < (*channel).incomingReliableSequenceNumber
                                                    as libc::c_int
                                            {
                                                current_block = 1856101646708284338;
                                            } else {
                                                current_block = 8457315219000651999;
                                            }
                                        } else {
                                            if (*incomingCommand).reliableSequenceNumber
                                                as libc::c_int
                                                >= (*channel).incomingReliableSequenceNumber
                                                    as libc::c_int
                                            {
                                                current_block = 7746103178988627676;
                                                break;
                                            }
                                            current_block = 8457315219000651999;
                                        }
                                        match current_block {
                                            8457315219000651999 => {
                                                if (*incomingCommand).reliableSequenceNumber
                                                    as libc::c_uint
                                                    <= reliableSequenceNumber
                                                {
                                                    if ((*incomingCommand).reliableSequenceNumber
                                                        as libc::c_uint)
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
                                    == (*channel).incomingReliableSequenceNumber as libc::c_uint
                                    && unreliableSequenceNumber
                                        <= (*channel).incomingUnreliableSequenceNumber
                                            as libc::c_uint
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
                                        incomingCommand =
                                            currentCommand as *mut ENetIncomingCommand;
                                        if !((*command).header.command as libc::c_int
                                            & ENET_PROTOCOL_COMMAND_MASK as libc::c_int
                                            == ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED
                                                as libc::c_int)
                                        {
                                            if reliableSequenceNumber
                                                >= (*channel).incomingReliableSequenceNumber
                                                    as libc::c_uint
                                            {
                                                if ((*incomingCommand).reliableSequenceNumber
                                                    as libc::c_int)
                                                    < (*channel).incomingReliableSequenceNumber
                                                        as libc::c_int
                                                {
                                                    current_block = 17478428563724192186;
                                                } else {
                                                    current_block = 11459959175219260272;
                                                }
                                            } else {
                                                if (*incomingCommand).reliableSequenceNumber
                                                    as libc::c_int
                                                    >= (*channel).incomingReliableSequenceNumber
                                                        as libc::c_int
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
                                                        as libc::c_uint)
                                                        < reliableSequenceNumber
                                                    {
                                                        current_block = 7746103178988627676;
                                                        break;
                                                    }
                                                    if !((*incomingCommand).reliableSequenceNumber
                                                        as libc::c_uint
                                                        > reliableSequenceNumber)
                                                    {
                                                        if (*incomingCommand)
                                                            .unreliableSequenceNumber
                                                            as libc::c_uint
                                                            <= unreliableSequenceNumber
                                                        {
                                                            if ((*incomingCommand)
                                                                .unreliableSequenceNumber
                                                                as libc::c_uint)
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
                                        incomingCommand = enet_malloc(::core::mem::size_of::<
                                            ENetIncomingCommand,
                                        >(
                                        )
                                            as libc::c_ulong)
                                            as *mut ENetIncomingCommand;
                                        if incomingCommand.is_null() {
                                            current_block = 15492018734234176694;
                                        } else {
                                            (*incomingCommand).reliableSequenceNumber =
                                                (*command).header.reliableSequenceNumber;
                                            (*incomingCommand).unreliableSequenceNumber =
                                                (unreliableSequenceNumber
                                                    & 0xffff as libc::c_int as libc::c_uint)
                                                    as enet_uint16;
                                            (*incomingCommand).command = *command;
                                            (*incomingCommand).fragmentCount = fragmentCount;
                                            (*incomingCommand).fragmentsRemaining = fragmentCount;
                                            (*incomingCommand).packet = packet;
                                            (*incomingCommand).fragments = 0 as *mut enet_uint32;
                                            if fragmentCount > 0 as libc::c_int as libc::c_uint {
                                                if fragmentCount
                                                    <= ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT
                                                        as libc::c_int
                                                        as libc::c_uint
                                                {
                                                    (*incomingCommand).fragments = enet_malloc(
                                                        (fragmentCount
                                                            .wrapping_add(
                                                                31 as libc::c_int as libc::c_uint,
                                                            )
                                                            .wrapping_div(
                                                                32 as libc::c_int as libc::c_uint,
                                                            )
                                                            as libc::c_ulong)
                                                            .wrapping_mul(::core::mem::size_of::<
                                                                enet_uint32,
                                                            >(
                                                            )
                                                                as libc::c_ulong),
                                                    )
                                                        as *mut enet_uint32;
                                                }
                                                if ((*incomingCommand).fragments).is_null() {
                                                    enet_free(incomingCommand as *mut libc::c_void);
                                                    current_block = 15492018734234176694;
                                                } else {
                                                    memset(
                                                        (*incomingCommand).fragments
                                                            as *mut libc::c_void,
                                                        0 as libc::c_int,
                                                        (fragmentCount
                                                            .wrapping_add(
                                                                31 as libc::c_int as libc::c_uint,
                                                            )
                                                            .wrapping_div(
                                                                32 as libc::c_int as libc::c_uint,
                                                            )
                                                            as libc::c_ulong)
                                                            .wrapping_mul(::core::mem::size_of::<
                                                                enet_uint32,
                                                            >(
                                                            )
                                                                as libc::c_ulong),
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
                                                        (*packet).referenceCount = ((*packet)
                                                            .referenceCount)
                                                            .wrapping_add(1);
                                                        (*peer).totalWaitingData = ((*peer)
                                                            .totalWaitingData
                                                            as libc::c_ulong)
                                                            .wrapping_add((*packet).dataLength)
                                                            as size_t
                                                            as size_t;
                                                    }
                                                    enet_list_insert(
                                                        (*currentCommand).next,
                                                        incomingCommand as *mut libc::c_void,
                                                    );
                                                    match (*command).header.command as libc::c_int
                                                        & ENET_PROTOCOL_COMMAND_MASK as libc::c_int
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
                                currentCommand =
                                    &mut (*channel).incomingUnreliableCommands.sentinel;
                                current_block = 7746103178988627676;
                            }
                            4379360700607281851 => {
                                if reliableSequenceNumber
                                    == (*channel).incomingReliableSequenceNumber as libc::c_uint
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
                                        incomingCommand =
                                            currentCommand as *mut ENetIncomingCommand;
                                        if reliableSequenceNumber
                                            >= (*channel).incomingReliableSequenceNumber
                                                as libc::c_uint
                                        {
                                            if ((*incomingCommand).reliableSequenceNumber
                                                as libc::c_int)
                                                < (*channel).incomingReliableSequenceNumber
                                                    as libc::c_int
                                            {
                                                current_block = 1856101646708284338;
                                            } else {
                                                current_block = 8457315219000651999;
                                            }
                                        } else {
                                            if (*incomingCommand).reliableSequenceNumber
                                                as libc::c_int
                                                >= (*channel).incomingReliableSequenceNumber
                                                    as libc::c_int
                                            {
                                                current_block = 7746103178988627676;
                                                break;
                                            }
                                            current_block = 8457315219000651999;
                                        }
                                        match current_block {
                                            8457315219000651999 => {
                                                if (*incomingCommand).reliableSequenceNumber
                                                    as libc::c_uint
                                                    <= reliableSequenceNumber
                                                {
                                                    if ((*incomingCommand).reliableSequenceNumber
                                                        as libc::c_uint)
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
                                    == (*channel).incomingReliableSequenceNumber as libc::c_uint
                                    && unreliableSequenceNumber
                                        <= (*channel).incomingUnreliableSequenceNumber
                                            as libc::c_uint
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
                                        incomingCommand =
                                            currentCommand as *mut ENetIncomingCommand;
                                        if !((*command).header.command as libc::c_int
                                            & ENET_PROTOCOL_COMMAND_MASK as libc::c_int
                                            == ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED
                                                as libc::c_int)
                                        {
                                            if reliableSequenceNumber
                                                >= (*channel).incomingReliableSequenceNumber
                                                    as libc::c_uint
                                            {
                                                if ((*incomingCommand).reliableSequenceNumber
                                                    as libc::c_int)
                                                    < (*channel).incomingReliableSequenceNumber
                                                        as libc::c_int
                                                {
                                                    current_block = 17478428563724192186;
                                                } else {
                                                    current_block = 11459959175219260272;
                                                }
                                            } else {
                                                if (*incomingCommand).reliableSequenceNumber
                                                    as libc::c_int
                                                    >= (*channel).incomingReliableSequenceNumber
                                                        as libc::c_int
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
                                                        as libc::c_uint)
                                                        < reliableSequenceNumber
                                                    {
                                                        current_block = 7746103178988627676;
                                                        break;
                                                    }
                                                    if !((*incomingCommand).reliableSequenceNumber
                                                        as libc::c_uint
                                                        > reliableSequenceNumber)
                                                    {
                                                        if (*incomingCommand)
                                                            .unreliableSequenceNumber
                                                            as libc::c_uint
                                                            <= unreliableSequenceNumber
                                                        {
                                                            if ((*incomingCommand)
                                                                .unreliableSequenceNumber
                                                                as libc::c_uint)
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
                                        incomingCommand = enet_malloc(::core::mem::size_of::<
                                            ENetIncomingCommand,
                                        >(
                                        )
                                            as libc::c_ulong)
                                            as *mut ENetIncomingCommand;
                                        if incomingCommand.is_null() {
                                            current_block = 15492018734234176694;
                                        } else {
                                            (*incomingCommand).reliableSequenceNumber =
                                                (*command).header.reliableSequenceNumber;
                                            (*incomingCommand).unreliableSequenceNumber =
                                                (unreliableSequenceNumber
                                                    & 0xffff as libc::c_int as libc::c_uint)
                                                    as enet_uint16;
                                            (*incomingCommand).command = *command;
                                            (*incomingCommand).fragmentCount = fragmentCount;
                                            (*incomingCommand).fragmentsRemaining = fragmentCount;
                                            (*incomingCommand).packet = packet;
                                            (*incomingCommand).fragments = 0 as *mut enet_uint32;
                                            if fragmentCount > 0 as libc::c_int as libc::c_uint {
                                                if fragmentCount
                                                    <= ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT
                                                        as libc::c_int
                                                        as libc::c_uint
                                                {
                                                    (*incomingCommand).fragments = enet_malloc(
                                                        (fragmentCount
                                                            .wrapping_add(
                                                                31 as libc::c_int as libc::c_uint,
                                                            )
                                                            .wrapping_div(
                                                                32 as libc::c_int as libc::c_uint,
                                                            )
                                                            as libc::c_ulong)
                                                            .wrapping_mul(::core::mem::size_of::<
                                                                enet_uint32,
                                                            >(
                                                            )
                                                                as libc::c_ulong),
                                                    )
                                                        as *mut enet_uint32;
                                                }
                                                if ((*incomingCommand).fragments).is_null() {
                                                    enet_free(incomingCommand as *mut libc::c_void);
                                                    current_block = 15492018734234176694;
                                                } else {
                                                    memset(
                                                        (*incomingCommand).fragments
                                                            as *mut libc::c_void,
                                                        0 as libc::c_int,
                                                        (fragmentCount
                                                            .wrapping_add(
                                                                31 as libc::c_int as libc::c_uint,
                                                            )
                                                            .wrapping_div(
                                                                32 as libc::c_int as libc::c_uint,
                                                            )
                                                            as libc::c_ulong)
                                                            .wrapping_mul(::core::mem::size_of::<
                                                                enet_uint32,
                                                            >(
                                                            )
                                                                as libc::c_ulong),
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
                                                        (*packet).referenceCount = ((*packet)
                                                            .referenceCount)
                                                            .wrapping_add(1);
                                                        (*peer).totalWaitingData = ((*peer)
                                                            .totalWaitingData
                                                            as libc::c_ulong)
                                                            .wrapping_add((*packet).dataLength)
                                                            as size_t
                                                            as size_t;
                                                    }
                                                    enet_list_insert(
                                                        (*currentCommand).next,
                                                        incomingCommand as *mut libc::c_void,
                                                    );
                                                    match (*command).header.command as libc::c_int
                                                        & ENET_PROTOCOL_COMMAND_MASK as libc::c_int
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
                                currentCommand =
                                    &mut (*channel).incomingUnreliableCommands.sentinel;
                                current_block = 7746103178988627676;
                            }
                            4379360700607281851 => {
                                if reliableSequenceNumber
                                    == (*channel).incomingReliableSequenceNumber as libc::c_uint
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
                                        incomingCommand =
                                            currentCommand as *mut ENetIncomingCommand;
                                        if reliableSequenceNumber
                                            >= (*channel).incomingReliableSequenceNumber
                                                as libc::c_uint
                                        {
                                            if ((*incomingCommand).reliableSequenceNumber
                                                as libc::c_int)
                                                < (*channel).incomingReliableSequenceNumber
                                                    as libc::c_int
                                            {
                                                current_block = 1856101646708284338;
                                            } else {
                                                current_block = 8457315219000651999;
                                            }
                                        } else {
                                            if (*incomingCommand).reliableSequenceNumber
                                                as libc::c_int
                                                >= (*channel).incomingReliableSequenceNumber
                                                    as libc::c_int
                                            {
                                                current_block = 7746103178988627676;
                                                break;
                                            }
                                            current_block = 8457315219000651999;
                                        }
                                        match current_block {
                                            8457315219000651999 => {
                                                if (*incomingCommand).reliableSequenceNumber
                                                    as libc::c_uint
                                                    <= reliableSequenceNumber
                                                {
                                                    if ((*incomingCommand).reliableSequenceNumber
                                                        as libc::c_uint)
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
                                    == (*channel).incomingReliableSequenceNumber as libc::c_uint
                                    && unreliableSequenceNumber
                                        <= (*channel).incomingUnreliableSequenceNumber
                                            as libc::c_uint
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
                                        incomingCommand =
                                            currentCommand as *mut ENetIncomingCommand;
                                        if !((*command).header.command as libc::c_int
                                            & ENET_PROTOCOL_COMMAND_MASK as libc::c_int
                                            == ENET_PROTOCOL_COMMAND_SEND_UNSEQUENCED
                                                as libc::c_int)
                                        {
                                            if reliableSequenceNumber
                                                >= (*channel).incomingReliableSequenceNumber
                                                    as libc::c_uint
                                            {
                                                if ((*incomingCommand).reliableSequenceNumber
                                                    as libc::c_int)
                                                    < (*channel).incomingReliableSequenceNumber
                                                        as libc::c_int
                                                {
                                                    current_block = 17478428563724192186;
                                                } else {
                                                    current_block = 11459959175219260272;
                                                }
                                            } else {
                                                if (*incomingCommand).reliableSequenceNumber
                                                    as libc::c_int
                                                    >= (*channel).incomingReliableSequenceNumber
                                                        as libc::c_int
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
                                                        as libc::c_uint)
                                                        < reliableSequenceNumber
                                                    {
                                                        current_block = 7746103178988627676;
                                                        break;
                                                    }
                                                    if !((*incomingCommand).reliableSequenceNumber
                                                        as libc::c_uint
                                                        > reliableSequenceNumber)
                                                    {
                                                        if (*incomingCommand)
                                                            .unreliableSequenceNumber
                                                            as libc::c_uint
                                                            <= unreliableSequenceNumber
                                                        {
                                                            if ((*incomingCommand)
                                                                .unreliableSequenceNumber
                                                                as libc::c_uint)
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
                                        incomingCommand = enet_malloc(::core::mem::size_of::<
                                            ENetIncomingCommand,
                                        >(
                                        )
                                            as libc::c_ulong)
                                            as *mut ENetIncomingCommand;
                                        if incomingCommand.is_null() {
                                            current_block = 15492018734234176694;
                                        } else {
                                            (*incomingCommand).reliableSequenceNumber =
                                                (*command).header.reliableSequenceNumber;
                                            (*incomingCommand).unreliableSequenceNumber =
                                                (unreliableSequenceNumber
                                                    & 0xffff as libc::c_int as libc::c_uint)
                                                    as enet_uint16;
                                            (*incomingCommand).command = *command;
                                            (*incomingCommand).fragmentCount = fragmentCount;
                                            (*incomingCommand).fragmentsRemaining = fragmentCount;
                                            (*incomingCommand).packet = packet;
                                            (*incomingCommand).fragments = 0 as *mut enet_uint32;
                                            if fragmentCount > 0 as libc::c_int as libc::c_uint {
                                                if fragmentCount
                                                    <= ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT
                                                        as libc::c_int
                                                        as libc::c_uint
                                                {
                                                    (*incomingCommand).fragments = enet_malloc(
                                                        (fragmentCount
                                                            .wrapping_add(
                                                                31 as libc::c_int as libc::c_uint,
                                                            )
                                                            .wrapping_div(
                                                                32 as libc::c_int as libc::c_uint,
                                                            )
                                                            as libc::c_ulong)
                                                            .wrapping_mul(::core::mem::size_of::<
                                                                enet_uint32,
                                                            >(
                                                            )
                                                                as libc::c_ulong),
                                                    )
                                                        as *mut enet_uint32;
                                                }
                                                if ((*incomingCommand).fragments).is_null() {
                                                    enet_free(incomingCommand as *mut libc::c_void);
                                                    current_block = 15492018734234176694;
                                                } else {
                                                    memset(
                                                        (*incomingCommand).fragments
                                                            as *mut libc::c_void,
                                                        0 as libc::c_int,
                                                        (fragmentCount
                                                            .wrapping_add(
                                                                31 as libc::c_int as libc::c_uint,
                                                            )
                                                            .wrapping_div(
                                                                32 as libc::c_int as libc::c_uint,
                                                            )
                                                            as libc::c_ulong)
                                                            .wrapping_mul(::core::mem::size_of::<
                                                                enet_uint32,
                                                            >(
                                                            )
                                                                as libc::c_ulong),
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
                                                        (*packet).referenceCount = ((*packet)
                                                            .referenceCount)
                                                            .wrapping_add(1);
                                                        (*peer).totalWaitingData = ((*peer)
                                                            .totalWaitingData
                                                            as libc::c_ulong)
                                                            .wrapping_add((*packet).dataLength)
                                                            as size_t
                                                            as size_t;
                                                    }
                                                    enet_list_insert(
                                                        (*currentCommand).next,
                                                        incomingCommand as *mut libc::c_void,
                                                    );
                                                    match (*command).header.command as libc::c_int
                                                        & ENET_PROTOCOL_COMMAND_MASK as libc::c_int
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
                }
            }
        }
    }
    match current_block {
        9207730764507465628 => {
            if !(fragmentCount > 0 as libc::c_int as libc::c_uint) {
                if !packet.is_null()
                    && (*packet).referenceCount == 0 as libc::c_int as libc::c_ulong
                {
                    enet_packet_destroy(packet);
                }
                return &mut dummyCommand;
            }
        }
        _ => {}
    }
    if !packet.is_null() && (*packet).referenceCount == 0 as libc::c_int as libc::c_ulong {
        enet_packet_destroy(packet);
    }
    return 0 as *mut ENetIncomingCommand;
}
static mut commandSizes: [size_t; 13] = [
    0 as libc::c_int as size_t,
    ::core::mem::size_of::<ENetProtocolAcknowledge>() as libc::c_ulong,
    ::core::mem::size_of::<ENetProtocolConnect>() as libc::c_ulong,
    ::core::mem::size_of::<ENetProtocolVerifyConnect>() as libc::c_ulong,
    ::core::mem::size_of::<ENetProtocolDisconnect>() as libc::c_ulong,
    ::core::mem::size_of::<ENetProtocolPing>() as libc::c_ulong,
    ::core::mem::size_of::<ENetProtocolSendReliable>() as libc::c_ulong,
    ::core::mem::size_of::<ENetProtocolSendUnreliable>() as libc::c_ulong,
    ::core::mem::size_of::<ENetProtocolSendFragment>() as libc::c_ulong,
    ::core::mem::size_of::<ENetProtocolSendUnsequenced>() as libc::c_ulong,
    ::core::mem::size_of::<ENetProtocolBandwidthLimit>() as libc::c_ulong,
    ::core::mem::size_of::<ENetProtocolThrottleConfigure>() as libc::c_ulong,
    ::core::mem::size_of::<ENetProtocolSendFragment>() as libc::c_ulong,
];
pub unsafe fn enet_protocol_command_size(mut commandNumber: enet_uint8) -> size_t {
    return commandSizes
        [(commandNumber as libc::c_int & ENET_PROTOCOL_COMMAND_MASK as libc::c_int) as usize];
}
unsafe fn enet_protocol_change_state(
    mut _host: *mut ENetHost,
    mut peer: *mut ENetPeer,
    mut state: ENetPeerState,
) {
    if state as libc::c_uint == ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
        || state as libc::c_uint == ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
    {
        enet_peer_on_connect(peer);
    } else {
        enet_peer_on_disconnect(peer);
    }
    (*peer).state = state;
}
unsafe fn enet_protocol_dispatch_state(
    mut host: *mut ENetHost,
    mut peer: *mut ENetPeer,
    mut state: ENetPeerState,
) {
    enet_protocol_change_state(host, peer, state);
    if (*peer).flags as libc::c_int & ENET_PEER_FLAG_NEEDS_DISPATCH as libc::c_int == 0 {
        enet_list_insert(
            &mut (*host).dispatchQueue.sentinel,
            &mut (*peer).dispatchList as *mut ENetListNode as *mut libc::c_void,
        );
        (*peer).flags = ((*peer).flags as libc::c_int
            | ENET_PEER_FLAG_NEEDS_DISPATCH as libc::c_int) as enet_uint16;
    }
}
unsafe fn enet_protocol_dispatch_incoming_commands(
    mut host: *mut ENetHost,
    mut event: *mut ENetEvent,
) -> libc::c_int {
    while !((*host).dispatchQueue.sentinel.next
        == &mut (*host).dispatchQueue.sentinel as *mut ENetListNode)
    {
        let mut peer: *mut ENetPeer =
            enet_list_remove((*host).dispatchQueue.sentinel.next) as *mut ENetPeer;
        (*peer).flags = ((*peer).flags as libc::c_int
            & !(ENET_PEER_FLAG_NEEDS_DISPATCH as libc::c_int))
            as enet_uint16;
        match (*peer).state as libc::c_uint {
            3 | 4 => {
                enet_protocol_change_state(host, peer, ENET_PEER_STATE_CONNECTED);
                (*event).type_0 = ENET_EVENT_TYPE_CONNECT;
                (*event).peer = peer;
                (*event).data = (*peer).eventData;
                return 1 as libc::c_int;
            }
            9 => {
                (*host).recalculateBandwidthLimits = 1 as libc::c_int;
                (*event).type_0 = ENET_EVENT_TYPE_DISCONNECT;
                (*event).peer = peer;
                (*event).data = (*peer).eventData;
                enet_peer_reset(peer);
                return 1 as libc::c_int;
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
                    (*peer).flags = ((*peer).flags as libc::c_int
                        | ENET_PEER_FLAG_NEEDS_DISPATCH as libc::c_int)
                        as enet_uint16;
                    enet_list_insert(
                        &mut (*host).dispatchQueue.sentinel,
                        &mut (*peer).dispatchList as *mut ENetListNode as *mut libc::c_void,
                    );
                }
                return 1 as libc::c_int;
            }
            _ => {}
        }
    }
    return 0 as libc::c_int;
}
unsafe fn enet_protocol_notify_connect(
    mut host: *mut ENetHost,
    mut peer: *mut ENetPeer,
    mut event: *mut ENetEvent,
) {
    (*host).recalculateBandwidthLimits = 1 as libc::c_int;
    if !event.is_null() {
        enet_protocol_change_state(host, peer, ENET_PEER_STATE_CONNECTED);
        (*event).type_0 = ENET_EVENT_TYPE_CONNECT;
        (*event).peer = peer;
        (*event).data = (*peer).eventData;
    } else {
        enet_protocol_dispatch_state(
            host,
            peer,
            (if (*peer).state as libc::c_uint
                == ENET_PEER_STATE_CONNECTING as libc::c_int as libc::c_uint
            {
                ENET_PEER_STATE_CONNECTION_SUCCEEDED as libc::c_int
            } else {
                ENET_PEER_STATE_CONNECTION_PENDING as libc::c_int
            }) as ENetPeerState,
        );
    };
}
unsafe fn enet_protocol_notify_disconnect(
    mut host: *mut ENetHost,
    mut peer: *mut ENetPeer,
    mut event: *mut ENetEvent,
) {
    if (*peer).state as libc::c_uint
        >= ENET_PEER_STATE_CONNECTION_PENDING as libc::c_int as libc::c_uint
    {
        (*host).recalculateBandwidthLimits = 1 as libc::c_int;
    }
    if (*peer).state as libc::c_uint != ENET_PEER_STATE_CONNECTING as libc::c_int as libc::c_uint
        && ((*peer).state as libc::c_uint)
            < ENET_PEER_STATE_CONNECTION_SUCCEEDED as libc::c_int as libc::c_uint
    {
        enet_peer_reset(peer);
    } else if !event.is_null() {
        (*event).type_0 = ENET_EVENT_TYPE_DISCONNECT;
        (*event).peer = peer;
        (*event).data = 0 as libc::c_int as enet_uint32;
        enet_peer_reset(peer);
    } else {
        (*peer).eventData = 0 as libc::c_int as enet_uint32;
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
    };
}
unsafe fn enet_protocol_remove_sent_unreliable_commands(
    mut peer: *mut ENetPeer,
    mut sentUnreliableCommands: *mut ENetList,
) {
    let mut outgoingCommand: *mut ENetOutgoingCommand = 0 as *mut ENetOutgoingCommand;
    if (*sentUnreliableCommands).sentinel.next
        == &mut (*sentUnreliableCommands).sentinel as *mut ENetListNode
    {
        return;
    }
    loop {
        outgoingCommand = (*sentUnreliableCommands).sentinel.next as *mut libc::c_void
            as *mut ENetOutgoingCommand;
        enet_list_remove(&mut (*outgoingCommand).outgoingCommandList);
        if !((*outgoingCommand).packet).is_null() {
            (*(*outgoingCommand).packet).referenceCount =
                ((*(*outgoingCommand).packet).referenceCount).wrapping_sub(1);
            if (*(*outgoingCommand).packet).referenceCount == 0 as libc::c_int as libc::c_ulong {
                (*(*outgoingCommand).packet).flags |=
                    ENET_PACKET_FLAG_SENT as libc::c_int as libc::c_uint;
                enet_packet_destroy((*outgoingCommand).packet);
            }
        }
        enet_free(outgoingCommand as *mut libc::c_void);
        if (*sentUnreliableCommands).sentinel.next
            == &mut (*sentUnreliableCommands).sentinel as *mut ENetListNode
        {
            break;
        }
    }
    if (*peer).state as libc::c_uint
        == ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
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
        if !((*outgoingCommand).command.header.command as libc::c_int
            & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int
            == 0)
        {
            if ((*outgoingCommand).sendAttempts as libc::c_int) < 1 as libc::c_int {
                break;
            }
            if (*outgoingCommand).reliableSequenceNumber as libc::c_int
                == reliableSequenceNumber as libc::c_int
                && (*outgoingCommand).command.header.channelID as libc::c_int
                    == channelID as libc::c_int
            {
                return outgoingCommand;
            }
        }
        currentCommand = (*currentCommand).next;
    }
    return 0 as *mut ENetOutgoingCommand;
}
unsafe fn enet_protocol_remove_sent_reliable_command(
    mut peer: *mut ENetPeer,
    mut reliableSequenceNumber: enet_uint16,
    mut channelID: enet_uint8,
) -> ENetProtocolCommand {
    let mut outgoingCommand: *mut ENetOutgoingCommand = 0 as *mut ENetOutgoingCommand;
    let mut currentCommand: ENetListIterator = 0 as *mut ENetListNode;
    let mut commandNumber: ENetProtocolCommand = ENET_PROTOCOL_COMMAND_NONE;
    let mut wasSent: libc::c_int = 1 as libc::c_int;
    currentCommand = (*peer).sentReliableCommands.sentinel.next;
    while currentCommand != &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode {
        outgoingCommand = currentCommand as *mut ENetOutgoingCommand;
        if (*outgoingCommand).reliableSequenceNumber as libc::c_int
            == reliableSequenceNumber as libc::c_int
            && (*outgoingCommand).command.header.channelID as libc::c_int
                == channelID as libc::c_int
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
        wasSent = 0 as libc::c_int;
    }
    if outgoingCommand.is_null() {
        return ENET_PROTOCOL_COMMAND_NONE;
    }
    if (channelID as libc::c_ulong) < (*peer).channelCount {
        let mut channel: *mut ENetChannel =
            &mut *((*peer).channels).offset(channelID as isize) as *mut ENetChannel;
        let mut reliableWindow: enet_uint16 = (reliableSequenceNumber as libc::c_int
            / ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int)
            as enet_uint16;
        if (*channel).reliableWindows[reliableWindow as usize] as libc::c_int > 0 as libc::c_int {
            (*channel).reliableWindows[reliableWindow as usize] =
                ((*channel).reliableWindows[reliableWindow as usize]).wrapping_sub(1);
            if (*channel).reliableWindows[reliableWindow as usize] == 0 {
                (*channel).usedReliableWindows = ((*channel).usedReliableWindows as libc::c_int
                    & !((1 as libc::c_int) << reliableWindow as libc::c_int))
                    as enet_uint16;
            }
        }
    }
    commandNumber = ((*outgoingCommand).command.header.command as libc::c_int
        & ENET_PROTOCOL_COMMAND_MASK as libc::c_int) as ENetProtocolCommand;
    enet_list_remove(&mut (*outgoingCommand).outgoingCommandList);
    if !((*outgoingCommand).packet).is_null() {
        if wasSent != 0 {
            (*peer).reliableDataInTransit = ((*peer).reliableDataInTransit as libc::c_uint)
                .wrapping_sub((*outgoingCommand).fragmentLength as libc::c_uint)
                as enet_uint32 as enet_uint32;
        }
        (*(*outgoingCommand).packet).referenceCount =
            ((*(*outgoingCommand).packet).referenceCount).wrapping_sub(1);
        if (*(*outgoingCommand).packet).referenceCount == 0 as libc::c_int as libc::c_ulong {
            (*(*outgoingCommand).packet).flags |=
                ENET_PACKET_FLAG_SENT as libc::c_int as libc::c_uint;
            enet_packet_destroy((*outgoingCommand).packet);
        }
    }
    enet_free(outgoingCommand as *mut libc::c_void);
    if (*peer).sentReliableCommands.sentinel.next
        == &mut (*peer).sentReliableCommands.sentinel as *mut ENetListNode
    {
        return commandNumber;
    }
    outgoingCommand =
        (*peer).sentReliableCommands.sentinel.next as *mut libc::c_void as *mut ENetOutgoingCommand;
    (*peer).nextTimeout =
        ((*outgoingCommand).sentTime).wrapping_add((*outgoingCommand).roundTripTimeout);
    return commandNumber;
}
unsafe fn enet_protocol_handle_connect(
    mut host: *mut ENetHost,
    mut _header: *mut ENetProtocolHeader,
    mut command: *mut ENetProtocol,
) -> *mut ENetPeer {
    let mut incomingSessionID: enet_uint8 = 0;
    let mut outgoingSessionID: enet_uint8 = 0;
    let mut mtu: enet_uint32 = 0;
    let mut windowSize: enet_uint32 = 0;
    let mut channel: *mut ENetChannel = 0 as *mut ENetChannel;
    let mut channelCount: size_t = 0;
    let mut duplicatePeers: size_t = 0 as libc::c_int as size_t;
    let mut currentPeer: *mut ENetPeer = 0 as *mut ENetPeer;
    let mut peer: *mut ENetPeer = 0 as *mut ENetPeer;
    let mut verifyCommand: ENetProtocol = _ENetProtocol {
        header: ENetProtocolCommandHeader {
            command: 0,
            channelID: 0,
            reliableSequenceNumber: 0,
        },
    };
    channelCount = ntohl((*command).connect.channelCount) as size_t;
    if channelCount < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as libc::c_int as libc::c_ulong
        || channelCount > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as libc::c_int as libc::c_ulong
    {
        return 0 as *mut ENetPeer;
    }
    currentPeer = (*host).peers;
    while currentPeer < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer {
        if (*currentPeer).state as libc::c_uint
            == ENET_PEER_STATE_DISCONNECTED as libc::c_int as libc::c_uint
        {
            if peer.is_null() {
                peer = currentPeer;
            }
        } else if (*currentPeer).state as libc::c_uint
            != ENET_PEER_STATE_CONNECTING as libc::c_int as libc::c_uint
            && (*currentPeer).address.host == (*host).receivedAddress.host
        {
            if (*currentPeer).address.port as libc::c_int
                == (*host).receivedAddress.port as libc::c_int
                && (*currentPeer).connectID == (*command).connect.connectID
            {
                return 0 as *mut ENetPeer;
            }
            duplicatePeers = duplicatePeers.wrapping_add(1);
        }
        currentPeer = currentPeer.offset(1);
    }
    if peer.is_null() || duplicatePeers >= (*host).duplicatePeers {
        return 0 as *mut ENetPeer;
    }
    if channelCount > (*host).channelLimit {
        channelCount = (*host).channelLimit;
    }
    (*peer).channels = enet_malloc(
        channelCount.wrapping_mul(::core::mem::size_of::<ENetChannel>() as libc::c_ulong),
    ) as *mut ENetChannel;
    if ((*peer).channels).is_null() {
        return 0 as *mut ENetPeer;
    }
    (*peer).channelCount = channelCount;
    (*peer).state = ENET_PEER_STATE_ACKNOWLEDGING_CONNECT;
    (*peer).connectID = (*command).connect.connectID;
    (*peer).address = (*host).receivedAddress;
    (*peer).mtu = (*host).mtu;
    (*peer).outgoingPeerID = ntohs((*command).connect.outgoingPeerID);
    (*peer).incomingBandwidth = ntohl((*command).connect.incomingBandwidth);
    (*peer).outgoingBandwidth = ntohl((*command).connect.outgoingBandwidth);
    (*peer).packetThrottleInterval = ntohl((*command).connect.packetThrottleInterval);
    (*peer).packetThrottleAcceleration = ntohl((*command).connect.packetThrottleAcceleration);
    (*peer).packetThrottleDeceleration = ntohl((*command).connect.packetThrottleDeceleration);
    (*peer).eventData = ntohl((*command).connect.data);
    incomingSessionID =
        (if (*command).connect.incomingSessionID as libc::c_int == 0xff as libc::c_int {
            (*peer).outgoingSessionID as libc::c_int
        } else {
            (*command).connect.incomingSessionID as libc::c_int
        }) as enet_uint8;
    incomingSessionID = (incomingSessionID as libc::c_int + 1 as libc::c_int
        & ENET_PROTOCOL_HEADER_SESSION_MASK as libc::c_int
            >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as libc::c_int)
        as enet_uint8;
    if incomingSessionID as libc::c_int == (*peer).outgoingSessionID as libc::c_int {
        incomingSessionID = (incomingSessionID as libc::c_int + 1 as libc::c_int
            & ENET_PROTOCOL_HEADER_SESSION_MASK as libc::c_int
                >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as libc::c_int)
            as enet_uint8;
    }
    (*peer).outgoingSessionID = incomingSessionID;
    outgoingSessionID =
        (if (*command).connect.outgoingSessionID as libc::c_int == 0xff as libc::c_int {
            (*peer).incomingSessionID as libc::c_int
        } else {
            (*command).connect.outgoingSessionID as libc::c_int
        }) as enet_uint8;
    outgoingSessionID = (outgoingSessionID as libc::c_int + 1 as libc::c_int
        & ENET_PROTOCOL_HEADER_SESSION_MASK as libc::c_int
            >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as libc::c_int)
        as enet_uint8;
    if outgoingSessionID as libc::c_int == (*peer).incomingSessionID as libc::c_int {
        outgoingSessionID = (outgoingSessionID as libc::c_int + 1 as libc::c_int
            & ENET_PROTOCOL_HEADER_SESSION_MASK as libc::c_int
                >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as libc::c_int)
            as enet_uint8;
    }
    (*peer).incomingSessionID = outgoingSessionID;
    channel = (*peer).channels;
    while channel < &mut *((*peer).channels).offset(channelCount as isize) as *mut ENetChannel {
        (*channel).outgoingReliableSequenceNumber = 0 as libc::c_int as enet_uint16;
        (*channel).outgoingUnreliableSequenceNumber = 0 as libc::c_int as enet_uint16;
        (*channel).incomingReliableSequenceNumber = 0 as libc::c_int as enet_uint16;
        (*channel).incomingUnreliableSequenceNumber = 0 as libc::c_int as enet_uint16;
        enet_list_clear(&mut (*channel).incomingReliableCommands);
        enet_list_clear(&mut (*channel).incomingUnreliableCommands);
        (*channel).usedReliableWindows = 0 as libc::c_int as enet_uint16;
        memset(
            ((*channel).reliableWindows).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[enet_uint16; 16]>() as libc::c_ulong,
        );
        channel = channel.offset(1);
    }
    mtu = ntohl((*command).connect.mtu);
    if mtu < ENET_PROTOCOL_MINIMUM_MTU as libc::c_int as libc::c_uint {
        mtu = ENET_PROTOCOL_MINIMUM_MTU as libc::c_int as enet_uint32;
    } else if mtu > ENET_PROTOCOL_MAXIMUM_MTU as libc::c_int as libc::c_uint {
        mtu = ENET_PROTOCOL_MAXIMUM_MTU as libc::c_int as enet_uint32;
    }
    if mtu < (*peer).mtu {
        (*peer).mtu = mtu;
    }
    if (*host).outgoingBandwidth == 0 as libc::c_int as libc::c_uint
        && (*peer).incomingBandwidth == 0 as libc::c_int as libc::c_uint
    {
        (*peer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as enet_uint32;
    } else if (*host).outgoingBandwidth == 0 as libc::c_int as libc::c_uint
        || (*peer).incomingBandwidth == 0 as libc::c_int as libc::c_uint
    {
        (*peer).windowSize = (if (*host).outgoingBandwidth > (*peer).incomingBandwidth {
            (*host).outgoingBandwidth
        } else {
            (*peer).incomingBandwidth
        })
        .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as libc::c_int as libc::c_uint)
        .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint);
    } else {
        (*peer).windowSize = (if (*host).outgoingBandwidth < (*peer).incomingBandwidth {
            (*host).outgoingBandwidth
        } else {
            (*peer).incomingBandwidth
        })
        .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as libc::c_int as libc::c_uint)
        .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint);
    }
    if (*peer).windowSize < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint {
        (*peer).windowSize = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as enet_uint32;
    } else if (*peer).windowSize > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint
    {
        (*peer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as enet_uint32;
    }
    if (*host).incomingBandwidth == 0 as libc::c_int as libc::c_uint {
        windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as enet_uint32;
    } else {
        windowSize = ((*host).incomingBandwidth)
            .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as libc::c_int as libc::c_uint)
            .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint);
    }
    if windowSize > ntohl((*command).connect.windowSize) {
        windowSize = ntohl((*command).connect.windowSize);
    }
    if windowSize < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint {
        windowSize = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as enet_uint32;
    } else if windowSize > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint {
        windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as enet_uint32;
    }
    verifyCommand.header.command = (ENET_PROTOCOL_COMMAND_VERIFY_CONNECT as libc::c_int
        | ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int)
        as enet_uint8;
    verifyCommand.header.channelID = 0xff as libc::c_int as enet_uint8;
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
        0 as libc::c_int as enet_uint32,
        0 as libc::c_int as enet_uint16,
    );
    return peer;
}
unsafe fn enet_protocol_handle_send_reliable(
    mut host: *mut ENetHost,
    mut peer: *mut ENetPeer,
    mut command: *const ENetProtocol,
    mut currentData: *mut *mut enet_uint8,
) -> libc::c_int {
    let mut dataLength: size_t = 0;
    if (*command).header.channelID as libc::c_ulong >= (*peer).channelCount
        || (*peer).state as libc::c_uint != ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
            && (*peer).state as libc::c_uint
                != ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    dataLength = ntohs((*command).sendReliable.dataLength) as size_t;
    *currentData = (*currentData).offset(dataLength as isize);
    if dataLength > (*host).maximumPacketSize
        || *currentData < (*host).receivedData
        || *currentData
            > &mut *((*host).receivedData).offset((*host).receivedDataLength as isize)
                as *mut enet_uint8
    {
        return -(1 as libc::c_int);
    }
    if (enet_peer_queue_incoming_command(
        peer,
        command,
        (command as *const enet_uint8)
            .offset(::core::mem::size_of::<ENetProtocolSendReliable>() as libc::c_ulong as isize)
            as *const libc::c_void,
        dataLength,
        ENET_PACKET_FLAG_RELIABLE as libc::c_int as enet_uint32,
        0 as libc::c_int as enet_uint32,
    ))
    .is_null()
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe fn enet_protocol_handle_send_unsequenced(
    mut host: *mut ENetHost,
    mut peer: *mut ENetPeer,
    mut command: *const ENetProtocol,
    mut currentData: *mut *mut enet_uint8,
) -> libc::c_int {
    let mut unsequencedGroup: enet_uint32 = 0;
    let mut index: enet_uint32 = 0;
    let mut dataLength: size_t = 0;
    if (*command).header.channelID as libc::c_ulong >= (*peer).channelCount
        || (*peer).state as libc::c_uint != ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
            && (*peer).state as libc::c_uint
                != ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    dataLength = ntohs((*command).sendUnsequenced.dataLength) as size_t;
    *currentData = (*currentData).offset(dataLength as isize);
    if dataLength > (*host).maximumPacketSize
        || *currentData < (*host).receivedData
        || *currentData
            > &mut *((*host).receivedData).offset((*host).receivedDataLength as isize)
                as *mut enet_uint8
    {
        return -(1 as libc::c_int);
    }
    unsequencedGroup = ntohs((*command).sendUnsequenced.unsequencedGroup) as enet_uint32;
    index = unsequencedGroup
        .wrapping_rem(ENET_PEER_UNSEQUENCED_WINDOW_SIZE as libc::c_int as libc::c_uint);
    if unsequencedGroup < (*peer).incomingUnsequencedGroup as libc::c_uint {
        unsequencedGroup = (unsequencedGroup as libc::c_uint)
            .wrapping_add(0x10000 as libc::c_int as libc::c_uint)
            as enet_uint32 as enet_uint32;
    }
    if unsequencedGroup
        >= ((*peer).incomingUnsequencedGroup as enet_uint32).wrapping_add(
            (ENET_PEER_FREE_UNSEQUENCED_WINDOWS as libc::c_int
                * ENET_PEER_UNSEQUENCED_WINDOW_SIZE as libc::c_int) as libc::c_uint,
        )
    {
        return 0 as libc::c_int;
    }
    unsequencedGroup &= 0xffff as libc::c_int as libc::c_uint;
    if unsequencedGroup.wrapping_sub(index) != (*peer).incomingUnsequencedGroup as libc::c_uint {
        (*peer).incomingUnsequencedGroup = unsequencedGroup.wrapping_sub(index) as enet_uint16;
        memset(
            ((*peer).unsequencedWindow).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[enet_uint32; 32]>() as libc::c_ulong,
        );
    } else if (*peer).unsequencedWindow
        [index.wrapping_div(32 as libc::c_int as libc::c_uint) as usize]
        & ((1 as libc::c_int) << index.wrapping_rem(32 as libc::c_int as libc::c_uint))
            as libc::c_uint
        != 0
    {
        return 0 as libc::c_int;
    }
    if (enet_peer_queue_incoming_command(
        peer,
        command,
        (command as *const enet_uint8)
            .offset(::core::mem::size_of::<ENetProtocolSendUnsequenced>() as libc::c_ulong as isize)
            as *const libc::c_void,
        dataLength,
        ENET_PACKET_FLAG_UNSEQUENCED as libc::c_int as enet_uint32,
        0 as libc::c_int as enet_uint32,
    ))
    .is_null()
    {
        return -(1 as libc::c_int);
    }
    (*peer).unsequencedWindow[index.wrapping_div(32 as libc::c_int as libc::c_uint) as usize] |=
        ((1 as libc::c_int) << index.wrapping_rem(32 as libc::c_int as libc::c_uint))
            as libc::c_uint;
    return 0 as libc::c_int;
}
unsafe fn enet_protocol_handle_send_unreliable(
    mut host: *mut ENetHost,
    mut peer: *mut ENetPeer,
    mut command: *const ENetProtocol,
    mut currentData: *mut *mut enet_uint8,
) -> libc::c_int {
    let mut dataLength: size_t = 0;
    if (*command).header.channelID as libc::c_ulong >= (*peer).channelCount
        || (*peer).state as libc::c_uint != ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
            && (*peer).state as libc::c_uint
                != ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    dataLength = ntohs((*command).sendUnreliable.dataLength) as size_t;
    *currentData = (*currentData).offset(dataLength as isize);
    if dataLength > (*host).maximumPacketSize
        || *currentData < (*host).receivedData
        || *currentData
            > &mut *((*host).receivedData).offset((*host).receivedDataLength as isize)
                as *mut enet_uint8
    {
        return -(1 as libc::c_int);
    }
    if (enet_peer_queue_incoming_command(
        peer,
        command,
        (command as *const enet_uint8)
            .offset(::core::mem::size_of::<ENetProtocolSendUnreliable>() as libc::c_ulong as isize)
            as *const libc::c_void,
        dataLength,
        0 as libc::c_int as enet_uint32,
        0 as libc::c_int as enet_uint32,
    ))
    .is_null()
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe fn enet_protocol_handle_send_fragment(
    mut host: *mut ENetHost,
    mut peer: *mut ENetPeer,
    mut command: *const ENetProtocol,
    mut currentData: *mut *mut enet_uint8,
) -> libc::c_int {
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
    if (*command).header.channelID as libc::c_ulong >= (*peer).channelCount
        || (*peer).state as libc::c_uint != ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
            && (*peer).state as libc::c_uint
                != ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    fragmentLength = ntohs((*command).sendFragment.dataLength) as enet_uint32;
    *currentData = (*currentData).offset(fragmentLength as isize);
    if fragmentLength <= 0 as libc::c_int as libc::c_uint
        || fragmentLength as libc::c_ulong > (*host).maximumPacketSize
        || *currentData < (*host).receivedData
        || *currentData
            > &mut *((*host).receivedData).offset((*host).receivedDataLength as isize)
                as *mut enet_uint8
    {
        return -(1 as libc::c_int);
    }
    channel =
        &mut *((*peer).channels).offset((*command).header.channelID as isize) as *mut ENetChannel;
    startSequenceNumber = ntohs((*command).sendFragment.startSequenceNumber) as enet_uint32;
    startWindow = startSequenceNumber
        .wrapping_div(ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int as libc::c_uint)
        as enet_uint16;
    currentWindow = ((*channel).incomingReliableSequenceNumber as libc::c_int
        / ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int) as enet_uint16;
    if startSequenceNumber < (*channel).incomingReliableSequenceNumber as libc::c_uint {
        startWindow =
            (startWindow as libc::c_int + ENET_PEER_RELIABLE_WINDOWS as libc::c_int) as enet_uint16;
    }
    if (startWindow as libc::c_int) < currentWindow as libc::c_int
        || startWindow as libc::c_int
            >= currentWindow as libc::c_int + ENET_PEER_FREE_RELIABLE_WINDOWS as libc::c_int
                - 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    fragmentNumber = ntohl((*command).sendFragment.fragmentNumber);
    fragmentCount = ntohl((*command).sendFragment.fragmentCount);
    fragmentOffset = ntohl((*command).sendFragment.fragmentOffset);
    totalLength = ntohl((*command).sendFragment.totalLength);
    if fragmentCount > ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as libc::c_int as libc::c_uint
        || fragmentNumber >= fragmentCount
        || totalLength as libc::c_ulong > (*host).maximumPacketSize
        || totalLength < fragmentCount
        || fragmentOffset >= totalLength
        || fragmentLength > totalLength.wrapping_sub(fragmentOffset)
    {
        return -(1 as libc::c_int);
    }
    let mut current_block_23: u64;
    currentCommand = (*channel).incomingReliableCommands.sentinel.previous;
    while currentCommand != &mut (*channel).incomingReliableCommands.sentinel as *mut ENetListNode {
        let mut incomingCommand: *mut ENetIncomingCommand =
            currentCommand as *mut ENetIncomingCommand;
        if startSequenceNumber >= (*channel).incomingReliableSequenceNumber as libc::c_uint {
            if ((*incomingCommand).reliableSequenceNumber as libc::c_int)
                < (*channel).incomingReliableSequenceNumber as libc::c_int
            {
                current_block_23 = 13056961889198038528;
            } else {
                current_block_23 = 12147880666119273379;
            }
        } else {
            if (*incomingCommand).reliableSequenceNumber as libc::c_int
                >= (*channel).incomingReliableSequenceNumber as libc::c_int
            {
                break;
            }
            current_block_23 = 12147880666119273379;
        }
        match current_block_23 {
            12147880666119273379 => {
                if (*incomingCommand).reliableSequenceNumber as libc::c_uint <= startSequenceNumber
                {
                    if ((*incomingCommand).reliableSequenceNumber as libc::c_uint)
                        < startSequenceNumber
                    {
                        break;
                    }
                    if (*incomingCommand).command.header.command as libc::c_int
                        & ENET_PROTOCOL_COMMAND_MASK as libc::c_int
                        != ENET_PROTOCOL_COMMAND_SEND_FRAGMENT as libc::c_int
                        || totalLength as libc::c_ulong != (*(*incomingCommand).packet).dataLength
                        || fragmentCount != (*incomingCommand).fragmentCount
                    {
                        return -(1 as libc::c_int);
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
            0 as *const libc::c_void,
            totalLength as size_t,
            ENET_PACKET_FLAG_RELIABLE as libc::c_int as enet_uint32,
            fragmentCount,
        );
        if startCommand.is_null() {
            return -(1 as libc::c_int);
        }
    }
    if *((*startCommand).fragments)
        .offset(fragmentNumber.wrapping_div(32 as libc::c_int as libc::c_uint) as isize)
        & ((1 as libc::c_int) << fragmentNumber.wrapping_rem(32 as libc::c_int as libc::c_uint))
            as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        (*startCommand).fragmentsRemaining = ((*startCommand).fragmentsRemaining).wrapping_sub(1);
        let ref mut fresh32 = *((*startCommand).fragments)
            .offset(fragmentNumber.wrapping_div(32 as libc::c_int as libc::c_uint) as isize);
        *fresh32 |= ((1 as libc::c_int)
            << fragmentNumber.wrapping_rem(32 as libc::c_int as libc::c_uint))
            as libc::c_uint;
        if fragmentOffset.wrapping_add(fragmentLength) as libc::c_ulong
            > (*(*startCommand).packet).dataLength
        {
            fragmentLength = ((*(*startCommand).packet).dataLength)
                .wrapping_sub(fragmentOffset as libc::c_ulong)
                as enet_uint32;
        }
        memcpy(
            ((*(*startCommand).packet).data).offset(fragmentOffset as isize) as *mut libc::c_void,
            (command as *mut enet_uint8).offset(
                ::core::mem::size_of::<ENetProtocolSendFragment>() as libc::c_ulong as isize
            ) as *const libc::c_void,
            fragmentLength as libc::c_ulong,
        );
        if (*startCommand).fragmentsRemaining <= 0 as libc::c_int as libc::c_uint {
            enet_peer_dispatch_incoming_reliable_commands(
                peer,
                channel,
                0 as *mut ENetIncomingCommand,
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe fn enet_protocol_handle_send_unreliable_fragment(
    mut host: *mut ENetHost,
    mut peer: *mut ENetPeer,
    mut command: *const ENetProtocol,
    mut currentData: *mut *mut enet_uint8,
) -> libc::c_int {
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
    if (*command).header.channelID as libc::c_ulong >= (*peer).channelCount
        || (*peer).state as libc::c_uint != ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
            && (*peer).state as libc::c_uint
                != ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    fragmentLength = ntohs((*command).sendFragment.dataLength) as enet_uint32;
    *currentData = (*currentData).offset(fragmentLength as isize);
    if fragmentLength as libc::c_ulong > (*host).maximumPacketSize
        || *currentData < (*host).receivedData
        || *currentData
            > &mut *((*host).receivedData).offset((*host).receivedDataLength as isize)
                as *mut enet_uint8
    {
        return -(1 as libc::c_int);
    }
    channel =
        &mut *((*peer).channels).offset((*command).header.channelID as isize) as *mut ENetChannel;
    reliableSequenceNumber = (*command).header.reliableSequenceNumber as enet_uint32;
    startSequenceNumber = ntohs((*command).sendFragment.startSequenceNumber) as enet_uint32;
    reliableWindow = reliableSequenceNumber
        .wrapping_div(ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int as libc::c_uint)
        as enet_uint16;
    currentWindow = ((*channel).incomingReliableSequenceNumber as libc::c_int
        / ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int) as enet_uint16;
    if reliableSequenceNumber < (*channel).incomingReliableSequenceNumber as libc::c_uint {
        reliableWindow = (reliableWindow as libc::c_int + ENET_PEER_RELIABLE_WINDOWS as libc::c_int)
            as enet_uint16;
    }
    if (reliableWindow as libc::c_int) < currentWindow as libc::c_int
        || reliableWindow as libc::c_int
            >= currentWindow as libc::c_int + ENET_PEER_FREE_RELIABLE_WINDOWS as libc::c_int
                - 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if reliableSequenceNumber == (*channel).incomingReliableSequenceNumber as libc::c_uint
        && startSequenceNumber <= (*channel).incomingUnreliableSequenceNumber as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    fragmentNumber = ntohl((*command).sendFragment.fragmentNumber);
    fragmentCount = ntohl((*command).sendFragment.fragmentCount);
    fragmentOffset = ntohl((*command).sendFragment.fragmentOffset);
    totalLength = ntohl((*command).sendFragment.totalLength);
    if fragmentCount > ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT as libc::c_int as libc::c_uint
        || fragmentNumber >= fragmentCount
        || totalLength as libc::c_ulong > (*host).maximumPacketSize
        || fragmentOffset >= totalLength
        || fragmentLength > totalLength.wrapping_sub(fragmentOffset)
    {
        return -(1 as libc::c_int);
    }
    let mut current_block_26: u64;
    currentCommand = (*channel).incomingUnreliableCommands.sentinel.previous;
    while currentCommand != &mut (*channel).incomingUnreliableCommands.sentinel as *mut ENetListNode
    {
        let mut incomingCommand: *mut ENetIncomingCommand =
            currentCommand as *mut ENetIncomingCommand;
        if reliableSequenceNumber >= (*channel).incomingReliableSequenceNumber as libc::c_uint {
            if ((*incomingCommand).reliableSequenceNumber as libc::c_int)
                < (*channel).incomingReliableSequenceNumber as libc::c_int
            {
                current_block_26 = 8457315219000651999;
            } else {
                current_block_26 = 1109700713171191020;
            }
        } else {
            if (*incomingCommand).reliableSequenceNumber as libc::c_int
                >= (*channel).incomingReliableSequenceNumber as libc::c_int
            {
                break;
            }
            current_block_26 = 1109700713171191020;
        }
        match current_block_26 {
            1109700713171191020 => {
                if ((*incomingCommand).reliableSequenceNumber as libc::c_uint)
                    < reliableSequenceNumber
                {
                    break;
                }
                if !((*incomingCommand).reliableSequenceNumber as libc::c_uint
                    > reliableSequenceNumber)
                {
                    if (*incomingCommand).unreliableSequenceNumber as libc::c_uint
                        <= startSequenceNumber
                    {
                        if ((*incomingCommand).unreliableSequenceNumber as libc::c_uint)
                            < startSequenceNumber
                        {
                            break;
                        }
                        if (*incomingCommand).command.header.command as libc::c_int
                            & ENET_PROTOCOL_COMMAND_MASK as libc::c_int
                            != ENET_PROTOCOL_COMMAND_SEND_UNRELIABLE_FRAGMENT as libc::c_int
                            || totalLength as libc::c_ulong
                                != (*(*incomingCommand).packet).dataLength
                            || fragmentCount != (*incomingCommand).fragmentCount
                        {
                            return -(1 as libc::c_int);
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
            0 as *const libc::c_void,
            totalLength as size_t,
            ENET_PACKET_FLAG_UNRELIABLE_FRAGMENT as libc::c_int as enet_uint32,
            fragmentCount,
        );
        if startCommand.is_null() {
            return -(1 as libc::c_int);
        }
    }
    if *((*startCommand).fragments)
        .offset(fragmentNumber.wrapping_div(32 as libc::c_int as libc::c_uint) as isize)
        & ((1 as libc::c_int) << fragmentNumber.wrapping_rem(32 as libc::c_int as libc::c_uint))
            as libc::c_uint
        == 0 as libc::c_int as libc::c_uint
    {
        (*startCommand).fragmentsRemaining = ((*startCommand).fragmentsRemaining).wrapping_sub(1);
        let ref mut fresh33 = *((*startCommand).fragments)
            .offset(fragmentNumber.wrapping_div(32 as libc::c_int as libc::c_uint) as isize);
        *fresh33 |= ((1 as libc::c_int)
            << fragmentNumber.wrapping_rem(32 as libc::c_int as libc::c_uint))
            as libc::c_uint;
        if fragmentOffset.wrapping_add(fragmentLength) as libc::c_ulong
            > (*(*startCommand).packet).dataLength
        {
            fragmentLength = ((*(*startCommand).packet).dataLength)
                .wrapping_sub(fragmentOffset as libc::c_ulong)
                as enet_uint32;
        }
        memcpy(
            ((*(*startCommand).packet).data).offset(fragmentOffset as isize) as *mut libc::c_void,
            (command as *mut enet_uint8).offset(
                ::core::mem::size_of::<ENetProtocolSendFragment>() as libc::c_ulong as isize
            ) as *const libc::c_void,
            fragmentLength as libc::c_ulong,
        );
        if (*startCommand).fragmentsRemaining <= 0 as libc::c_int as libc::c_uint {
            enet_peer_dispatch_incoming_unreliable_commands(
                peer,
                channel,
                0 as *mut ENetIncomingCommand,
            );
        }
    }
    return 0 as libc::c_int;
}
unsafe fn enet_protocol_handle_ping(
    mut _host: *mut ENetHost,
    mut peer: *mut ENetPeer,
    mut _command: *const ENetProtocol,
) -> libc::c_int {
    if (*peer).state as libc::c_uint != ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
        && (*peer).state as libc::c_uint
            != ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe fn enet_protocol_handle_bandwidth_limit(
    mut host: *mut ENetHost,
    mut peer: *mut ENetPeer,
    mut command: *const ENetProtocol,
) -> libc::c_int {
    if (*peer).state as libc::c_uint != ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
        && (*peer).state as libc::c_uint
            != ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    if (*peer).incomingBandwidth != 0 as libc::c_int as libc::c_uint {
        (*host).bandwidthLimitedPeers = ((*host).bandwidthLimitedPeers).wrapping_sub(1);
    }
    (*peer).incomingBandwidth = ntohl((*command).bandwidthLimit.incomingBandwidth);
    (*peer).outgoingBandwidth = ntohl((*command).bandwidthLimit.outgoingBandwidth);
    if (*peer).incomingBandwidth != 0 as libc::c_int as libc::c_uint {
        (*host).bandwidthLimitedPeers = ((*host).bandwidthLimitedPeers).wrapping_add(1);
    }
    if (*peer).incomingBandwidth == 0 as libc::c_int as libc::c_uint
        && (*host).outgoingBandwidth == 0 as libc::c_int as libc::c_uint
    {
        (*peer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as enet_uint32;
    } else if (*peer).incomingBandwidth == 0 as libc::c_int as libc::c_uint
        || (*host).outgoingBandwidth == 0 as libc::c_int as libc::c_uint
    {
        (*peer).windowSize = (if (*peer).incomingBandwidth > (*host).outgoingBandwidth {
            (*peer).incomingBandwidth
        } else {
            (*host).outgoingBandwidth
        })
        .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as libc::c_int as libc::c_uint)
        .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint);
    } else {
        (*peer).windowSize = (if (*peer).incomingBandwidth < (*host).outgoingBandwidth {
            (*peer).incomingBandwidth
        } else {
            (*host).outgoingBandwidth
        })
        .wrapping_div(ENET_PEER_WINDOW_SIZE_SCALE as libc::c_int as libc::c_uint)
        .wrapping_mul(ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint);
    }
    if (*peer).windowSize < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint {
        (*peer).windowSize = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as enet_uint32;
    } else if (*peer).windowSize > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint
    {
        (*peer).windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as enet_uint32;
    }
    return 0 as libc::c_int;
}
unsafe fn enet_protocol_handle_throttle_configure(
    mut _host: *mut ENetHost,
    mut peer: *mut ENetPeer,
    mut command: *const ENetProtocol,
) -> libc::c_int {
    if (*peer).state as libc::c_uint != ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
        && (*peer).state as libc::c_uint
            != ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
    {
        return -(1 as libc::c_int);
    }
    (*peer).packetThrottleInterval = ntohl((*command).throttleConfigure.packetThrottleInterval);
    (*peer).packetThrottleAcceleration =
        ntohl((*command).throttleConfigure.packetThrottleAcceleration);
    (*peer).packetThrottleDeceleration =
        ntohl((*command).throttleConfigure.packetThrottleDeceleration);
    return 0 as libc::c_int;
}
unsafe fn enet_protocol_handle_disconnect(
    mut host: *mut ENetHost,
    mut peer: *mut ENetPeer,
    mut command: *const ENetProtocol,
) -> libc::c_int {
    if (*peer).state as libc::c_uint == ENET_PEER_STATE_DISCONNECTED as libc::c_int as libc::c_uint
        || (*peer).state as libc::c_uint == ENET_PEER_STATE_ZOMBIE as libc::c_int as libc::c_uint
        || (*peer).state as libc::c_uint
            == ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    enet_peer_reset_queues(peer);
    if (*peer).state as libc::c_uint
        == ENET_PEER_STATE_CONNECTION_SUCCEEDED as libc::c_int as libc::c_uint
        || (*peer).state as libc::c_uint
            == ENET_PEER_STATE_DISCONNECTING as libc::c_int as libc::c_uint
        || (*peer).state as libc::c_uint
            == ENET_PEER_STATE_CONNECTING as libc::c_int as libc::c_uint
    {
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
    } else if (*peer).state as libc::c_uint
        != ENET_PEER_STATE_CONNECTED as libc::c_int as libc::c_uint
        && (*peer).state as libc::c_uint
            != ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
    {
        if (*peer).state as libc::c_uint
            == ENET_PEER_STATE_CONNECTION_PENDING as libc::c_int as libc::c_uint
        {
            (*host).recalculateBandwidthLimits = 1 as libc::c_int;
        }
        enet_peer_reset(peer);
    } else if (*command).header.command as libc::c_int
        & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int
        != 0
    {
        enet_protocol_change_state(host, peer, ENET_PEER_STATE_ACKNOWLEDGING_DISCONNECT);
    } else {
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
    }
    if (*peer).state as libc::c_uint != ENET_PEER_STATE_DISCONNECTED as libc::c_int as libc::c_uint
    {
        (*peer).eventData = ntohl((*command).disconnect.data);
    }
    return 0 as libc::c_int;
}
unsafe fn enet_protocol_handle_acknowledge(
    mut host: *mut ENetHost,
    mut event: *mut ENetEvent,
    mut peer: *mut ENetPeer,
    mut command: *const ENetProtocol,
) -> libc::c_int {
    let mut roundTripTime: enet_uint32 = 0;
    let mut receivedSentTime: enet_uint32 = 0;
    let mut receivedReliableSequenceNumber: enet_uint32 = 0;
    let mut commandNumber: ENetProtocolCommand = ENET_PROTOCOL_COMMAND_NONE;
    if (*peer).state as libc::c_uint == ENET_PEER_STATE_DISCONNECTED as libc::c_int as libc::c_uint
        || (*peer).state as libc::c_uint == ENET_PEER_STATE_ZOMBIE as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    receivedSentTime = ntohs((*command).acknowledge.receivedSentTime) as enet_uint32;
    receivedSentTime |= (*host).serviceTime & 0xffff0000 as libc::c_uint;
    if receivedSentTime & 0x8000 as libc::c_int as libc::c_uint
        > (*host).serviceTime & 0x8000 as libc::c_int as libc::c_uint
    {
        receivedSentTime = (receivedSentTime as libc::c_uint)
            .wrapping_sub(0x10000 as libc::c_int as libc::c_uint)
            as enet_uint32 as enet_uint32;
    }
    if ((*host).serviceTime).wrapping_sub(receivedSentTime)
        >= 86400000 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    roundTripTime = if ((*host).serviceTime).wrapping_sub(receivedSentTime)
        >= 86400000 as libc::c_int as libc::c_uint
    {
        receivedSentTime.wrapping_sub((*host).serviceTime)
    } else {
        ((*host).serviceTime).wrapping_sub(receivedSentTime)
    };
    roundTripTime = if roundTripTime > 1 as libc::c_int as libc::c_uint {
        roundTripTime
    } else {
        1 as libc::c_int as libc::c_uint
    };
    if (*peer).lastReceiveTime > 0 as libc::c_int as libc::c_uint {
        enet_peer_throttle(peer, roundTripTime);
        (*peer).roundTripTimeVariance = ((*peer).roundTripTimeVariance as libc::c_uint)
            .wrapping_sub(
                ((*peer).roundTripTimeVariance).wrapping_div(4 as libc::c_int as libc::c_uint),
            ) as enet_uint32 as enet_uint32;
        if roundTripTime >= (*peer).roundTripTime {
            let mut diff: enet_uint32 = roundTripTime.wrapping_sub((*peer).roundTripTime);
            (*peer).roundTripTimeVariance = ((*peer).roundTripTimeVariance as libc::c_uint)
                .wrapping_add(diff.wrapping_div(4 as libc::c_int as libc::c_uint))
                as enet_uint32 as enet_uint32;
            (*peer).roundTripTime = ((*peer).roundTripTime as libc::c_uint)
                .wrapping_add(diff.wrapping_div(8 as libc::c_int as libc::c_uint))
                as enet_uint32 as enet_uint32;
        } else {
            let mut diff_0: enet_uint32 = ((*peer).roundTripTime).wrapping_sub(roundTripTime);
            (*peer).roundTripTimeVariance = ((*peer).roundTripTimeVariance as libc::c_uint)
                .wrapping_add(diff_0.wrapping_div(4 as libc::c_int as libc::c_uint))
                as enet_uint32 as enet_uint32;
            (*peer).roundTripTime = ((*peer).roundTripTime as libc::c_uint)
                .wrapping_sub(diff_0.wrapping_div(8 as libc::c_int as libc::c_uint))
                as enet_uint32 as enet_uint32;
        }
    } else {
        (*peer).roundTripTime = roundTripTime;
        (*peer).roundTripTimeVariance = roundTripTime
            .wrapping_add(1 as libc::c_int as libc::c_uint)
            .wrapping_div(2 as libc::c_int as libc::c_uint);
    }
    if (*peer).roundTripTime < (*peer).lowestRoundTripTime {
        (*peer).lowestRoundTripTime = (*peer).roundTripTime;
    }
    if (*peer).roundTripTimeVariance > (*peer).highestRoundTripTimeVariance {
        (*peer).highestRoundTripTimeVariance = (*peer).roundTripTimeVariance;
    }
    if (*peer).packetThrottleEpoch == 0 as libc::c_int as libc::c_uint
        || (if ((*host).serviceTime).wrapping_sub((*peer).packetThrottleEpoch)
            >= 86400000 as libc::c_int as libc::c_uint
        {
            ((*peer).packetThrottleEpoch).wrapping_sub((*host).serviceTime)
        } else {
            ((*host).serviceTime).wrapping_sub((*peer).packetThrottleEpoch)
        }) >= (*peer).packetThrottleInterval
    {
        (*peer).lastRoundTripTime = (*peer).lowestRoundTripTime;
        (*peer).lastRoundTripTimeVariance =
            if (*peer).highestRoundTripTimeVariance > 1 as libc::c_int as libc::c_uint {
                (*peer).highestRoundTripTimeVariance
            } else {
                1 as libc::c_int as libc::c_uint
            };
        (*peer).lowestRoundTripTime = (*peer).roundTripTime;
        (*peer).highestRoundTripTimeVariance = (*peer).roundTripTimeVariance;
        (*peer).packetThrottleEpoch = (*host).serviceTime;
    }
    (*peer).lastReceiveTime = if (*host).serviceTime > 1 as libc::c_int as libc::c_uint {
        (*host).serviceTime
    } else {
        1 as libc::c_int as libc::c_uint
    };
    (*peer).earliestTimeout = 0 as libc::c_int as enet_uint32;
    receivedReliableSequenceNumber =
        ntohs((*command).acknowledge.receivedReliableSequenceNumber) as enet_uint32;
    commandNumber = enet_protocol_remove_sent_reliable_command(
        peer,
        receivedReliableSequenceNumber as enet_uint16,
        (*command).header.channelID,
    );
    match (*peer).state as libc::c_uint {
        2 => {
            if commandNumber as libc::c_uint
                != ENET_PROTOCOL_COMMAND_VERIFY_CONNECT as libc::c_int as libc::c_uint
            {
                return -(1 as libc::c_int);
            }
            enet_protocol_notify_connect(host, peer, event);
        }
        7 => {
            if commandNumber as libc::c_uint
                != ENET_PROTOCOL_COMMAND_DISCONNECT as libc::c_int as libc::c_uint
            {
                return -(1 as libc::c_int);
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
    return 0 as libc::c_int;
}
unsafe fn enet_protocol_handle_verify_connect(
    mut host: *mut ENetHost,
    mut event: *mut ENetEvent,
    mut peer: *mut ENetPeer,
    mut command: *const ENetProtocol,
) -> libc::c_int {
    let mut mtu: enet_uint32 = 0;
    let mut windowSize: enet_uint32 = 0;
    let mut channelCount: size_t = 0;
    if (*peer).state as libc::c_uint != ENET_PEER_STATE_CONNECTING as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    channelCount = ntohl((*command).verifyConnect.channelCount) as size_t;
    if channelCount < ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT as libc::c_int as libc::c_ulong
        || channelCount > ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT as libc::c_int as libc::c_ulong
        || ntohl((*command).verifyConnect.packetThrottleInterval) != (*peer).packetThrottleInterval
        || ntohl((*command).verifyConnect.packetThrottleAcceleration)
            != (*peer).packetThrottleAcceleration
        || ntohl((*command).verifyConnect.packetThrottleDeceleration)
            != (*peer).packetThrottleDeceleration
        || (*command).verifyConnect.connectID != (*peer).connectID
    {
        (*peer).eventData = 0 as libc::c_int as enet_uint32;
        enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
        return -(1 as libc::c_int);
    }
    enet_protocol_remove_sent_reliable_command(
        peer,
        1 as libc::c_int as enet_uint16,
        0xff as libc::c_int as enet_uint8,
    );
    if channelCount < (*peer).channelCount {
        (*peer).channelCount = channelCount;
    }
    (*peer).outgoingPeerID = ntohs((*command).verifyConnect.outgoingPeerID);
    (*peer).incomingSessionID = (*command).verifyConnect.incomingSessionID;
    (*peer).outgoingSessionID = (*command).verifyConnect.outgoingSessionID;
    mtu = ntohl((*command).verifyConnect.mtu);
    if mtu < ENET_PROTOCOL_MINIMUM_MTU as libc::c_int as libc::c_uint {
        mtu = ENET_PROTOCOL_MINIMUM_MTU as libc::c_int as enet_uint32;
    } else if mtu > ENET_PROTOCOL_MAXIMUM_MTU as libc::c_int as libc::c_uint {
        mtu = ENET_PROTOCOL_MAXIMUM_MTU as libc::c_int as enet_uint32;
    }
    if mtu < (*peer).mtu {
        (*peer).mtu = mtu;
    }
    windowSize = ntohl((*command).verifyConnect.windowSize);
    if windowSize < ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint {
        windowSize = ENET_PROTOCOL_MINIMUM_WINDOW_SIZE as libc::c_int as enet_uint32;
    }
    if windowSize > ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as libc::c_uint {
        windowSize = ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE as libc::c_int as enet_uint32;
    }
    if windowSize < (*peer).windowSize {
        (*peer).windowSize = windowSize;
    }
    (*peer).incomingBandwidth = ntohl((*command).verifyConnect.incomingBandwidth);
    (*peer).outgoingBandwidth = ntohl((*command).verifyConnect.outgoingBandwidth);
    enet_protocol_notify_connect(host, peer, event);
    return 0 as libc::c_int;
}
unsafe fn enet_protocol_handle_incoming_commands(
    mut host: *mut ENetHost,
    mut event: *mut ENetEvent,
) -> libc::c_int {
    let mut header: *mut ENetProtocolHeader = 0 as *mut ENetProtocolHeader;
    let mut command: *mut ENetProtocol = 0 as *mut ENetProtocol;
    let mut peer: *mut ENetPeer = 0 as *mut ENetPeer;
    let mut currentData: *mut enet_uint8 = 0 as *mut enet_uint8;
    let mut headerSize: size_t = 0;
    let mut peerID: enet_uint16 = 0;
    let mut flags: enet_uint16 = 0;
    let mut sessionID: enet_uint8 = 0;
    if (*host).receivedDataLength < 2 as size_t {
        return 0 as libc::c_int;
    }
    header = (*host).receivedData as *mut ENetProtocolHeader;
    peerID = ntohs((*header).peerID);
    sessionID = ((peerID as libc::c_int & ENET_PROTOCOL_HEADER_SESSION_MASK as libc::c_int)
        >> ENET_PROTOCOL_HEADER_SESSION_SHIFT as libc::c_int) as enet_uint8;
    flags = (peerID as libc::c_int & ENET_PROTOCOL_HEADER_FLAG_MASK as libc::c_int) as enet_uint16;
    peerID = (peerID as libc::c_int
        & !(ENET_PROTOCOL_HEADER_FLAG_MASK as libc::c_int
            | ENET_PROTOCOL_HEADER_SESSION_MASK as libc::c_int)) as enet_uint16;
    headerSize = if flags as libc::c_int & ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as libc::c_int != 0 {
        ::core::mem::size_of::<ENetProtocolHeader>() as libc::c_ulong
    } else {
        2 as size_t
    };
    if ((*host).checksum).is_some() {
        headerSize = (headerSize as libc::c_ulong)
            .wrapping_add(::core::mem::size_of::<enet_uint32>() as libc::c_ulong)
            as size_t as size_t;
    }
    if peerID as libc::c_int == ENET_PROTOCOL_MAXIMUM_PEER_ID as libc::c_int {
        peer = 0 as *mut ENetPeer;
    } else if peerID as libc::c_ulong >= (*host).peerCount {
        return 0 as libc::c_int;
    } else {
        peer = &mut *((*host).peers).offset(peerID as isize) as *mut ENetPeer;
        if (*peer).state as libc::c_uint
            == ENET_PEER_STATE_DISCONNECTED as libc::c_int as libc::c_uint
            || (*peer).state as libc::c_uint
                == ENET_PEER_STATE_ZOMBIE as libc::c_int as libc::c_uint
            || ((*host).receivedAddress.host != (*peer).address.host
                || (*host).receivedAddress.port as libc::c_int
                    != (*peer).address.port as libc::c_int)
                && (*peer).address.host != 0xffffffff as libc::c_uint
            || ((*peer).outgoingPeerID as libc::c_int)
                < ENET_PROTOCOL_MAXIMUM_PEER_ID as libc::c_int
                && sessionID as libc::c_int != (*peer).incomingSessionID as libc::c_int
        {
            return 0 as libc::c_int;
        }
    }
    if flags as libc::c_int & ENET_PROTOCOL_HEADER_FLAG_COMPRESSED as libc::c_int != 0 {
        let mut originalSize: size_t = 0;
        if ((*host).compressor.context).is_null() || ((*host).compressor.decompress).is_none() {
            return 0 as libc::c_int;
        }
        originalSize = ((*host).compressor.decompress).expect("non-null function pointer")(
            (*host).compressor.context,
            ((*host).receivedData).offset(headerSize as isize),
            ((*host).receivedDataLength).wrapping_sub(headerSize),
            ((*host).packetData[1 as libc::c_int as usize])
                .as_mut_ptr()
                .offset(headerSize as isize),
            (::core::mem::size_of::<[enet_uint8; 4096]>() as libc::c_ulong)
                .wrapping_sub(headerSize),
        );
        if originalSize <= 0 as libc::c_int as libc::c_ulong
            || originalSize
                > (::core::mem::size_of::<[enet_uint8; 4096]>() as libc::c_ulong)
                    .wrapping_sub(headerSize)
        {
            return 0 as libc::c_int;
        }
        memcpy(
            ((*host).packetData[1 as libc::c_int as usize]).as_mut_ptr() as *mut libc::c_void,
            header as *const libc::c_void,
            headerSize,
        );
        (*host).receivedData = ((*host).packetData[1 as libc::c_int as usize]).as_mut_ptr();
        (*host).receivedDataLength = headerSize.wrapping_add(originalSize);
    }
    if ((*host).checksum).is_some() {
        let mut checksum: *mut enet_uint32 = &mut *((*host).receivedData).offset(
            headerSize.wrapping_sub(::core::mem::size_of::<enet_uint32>() as libc::c_ulong)
                as isize,
        ) as *mut enet_uint8 as *mut enet_uint32;
        let mut desiredChecksum: enet_uint32 = *checksum;
        let mut buffer: ENetBuffer = ENetBuffer {
            data: 0 as *mut libc::c_void,
            dataLength: 0,
        };
        *checksum = if !peer.is_null() {
            (*peer).connectID
        } else {
            0 as libc::c_int as libc::c_uint
        };
        buffer.data = (*host).receivedData as *mut libc::c_void;
        buffer.dataLength = (*host).receivedDataLength;
        if ((*host).checksum).expect("non-null function pointer")(
            &mut buffer,
            1 as libc::c_int as size_t,
        ) != desiredChecksum
        {
            return 0 as libc::c_int;
        }
    }
    if !peer.is_null() {
        (*peer).address.host = (*host).receivedAddress.host;
        (*peer).address.port = (*host).receivedAddress.port;
        (*peer).incomingDataTotal = ((*peer).incomingDataTotal as libc::c_ulong)
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
            .offset(::core::mem::size_of::<ENetProtocolCommandHeader>() as libc::c_ulong as isize)
            > &mut *((*host).receivedData).offset((*host).receivedDataLength as isize)
                as *mut enet_uint8
        {
            break;
        }
        commandNumber = ((*command).header.command as libc::c_int
            & ENET_PROTOCOL_COMMAND_MASK as libc::c_int) as enet_uint8;
        if commandNumber as libc::c_int >= ENET_PROTOCOL_COMMAND_COUNT as libc::c_int {
            break;
        }
        commandSize = commandSizes[commandNumber as usize];
        if commandSize == 0 as libc::c_int as libc::c_ulong
            || currentData.offset(commandSize as isize)
                > &mut *((*host).receivedData).offset((*host).receivedDataLength as isize)
                    as *mut enet_uint8
        {
            break;
        }
        currentData = currentData.offset(commandSize as isize);
        if peer.is_null()
            && commandNumber as libc::c_int != ENET_PROTOCOL_COMMAND_CONNECT as libc::c_int
        {
            break;
        }
        (*command).header.reliableSequenceNumber = ntohs((*command).header.reliableSequenceNumber);
        match commandNumber as libc::c_int {
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
            && (*command).header.command as libc::c_int
                & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int
                != 0 as libc::c_int)
        {
            continue;
        }
        let mut sentTime: enet_uint16 = 0;
        if flags as libc::c_int & ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as libc::c_int == 0 {
            break;
        }
        sentTime = ntohs((*header).sentTime);
        match (*peer).state as libc::c_uint {
            7 | 2 | 0 | 9 => {}
            8 => {
                if (*command).header.command as libc::c_int
                    & ENET_PROTOCOL_COMMAND_MASK as libc::c_int
                    == ENET_PROTOCOL_COMMAND_DISCONNECT as libc::c_int
                {
                    enet_peer_queue_acknowledgement(peer, command, sentTime);
                }
            }
            _ => {
                enet_peer_queue_acknowledgement(peer, command, sentTime);
            }
        }
    }
    if !event.is_null()
        && (*event).type_0 as libc::c_uint != ENET_EVENT_TYPE_NONE as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe fn enet_protocol_receive_incoming_commands(
    mut host: *mut ENetHost,
    mut event: *mut ENetEvent,
) -> libc::c_int {
    let mut packets: libc::c_int = 0;
    let mut current_block_17: u64;
    packets = 0 as libc::c_int;
    while packets < 256 as libc::c_int {
        let mut receivedLength: libc::c_int = 0;
        let mut buffer: ENetBuffer = ENetBuffer {
            data: 0 as *mut libc::c_void,
            dataLength: 0,
        };
        buffer.data =
            ((*host).packetData[0 as libc::c_int as usize]).as_mut_ptr() as *mut libc::c_void;
        buffer.dataLength = ::core::mem::size_of::<[enet_uint8; 4096]>() as libc::c_ulong;
        receivedLength = enet_socket_receive(
            (*host).socket,
            &mut (*host).receivedAddress,
            &mut buffer,
            1 as libc::c_int as size_t,
        );
        if !(receivedLength == -(2 as libc::c_int)) {
            if receivedLength < 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            if receivedLength == 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            (*host).receivedData = ((*host).packetData[0 as libc::c_int as usize]).as_mut_ptr();
            (*host).receivedDataLength = receivedLength as size_t;
            (*host).totalReceivedData = ((*host).totalReceivedData as libc::c_uint)
                .wrapping_add(receivedLength as libc::c_uint)
                as enet_uint32 as enet_uint32;
            (*host).totalReceivedPackets = ((*host).totalReceivedPackets).wrapping_add(1);
            if ((*host).intercept).is_some() {
                match ((*host).intercept).expect("non-null function pointer")(host, event) {
                    1 => {
                        current_block_17 = 11187707480244993007;
                        match current_block_17 {
                            15717549315443811277 => return -(1 as libc::c_int),
                            _ => {
                                if !event.is_null()
                                    && (*event).type_0 as libc::c_uint
                                        != ENET_EVENT_TYPE_NONE as libc::c_int as libc::c_uint
                                {
                                    return 1 as libc::c_int;
                                }
                            }
                        }
                        current_block_17 = 11174649648027449784;
                    }
                    -1 => {
                        current_block_17 = 15717549315443811277;
                        match current_block_17 {
                            15717549315443811277 => return -(1 as libc::c_int),
                            _ => {
                                if !event.is_null()
                                    && (*event).type_0 as libc::c_uint
                                        != ENET_EVENT_TYPE_NONE as libc::c_int as libc::c_uint
                                {
                                    return 1 as libc::c_int;
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
                    1 => return 1 as libc::c_int,
                    -1 => return -(1 as libc::c_int),
                    _ => {}
                },
            }
        }
        packets += 1;
    }
    return 0 as libc::c_int;
}
unsafe fn enet_protocol_send_acknowledgements(mut host: *mut ENetHost, mut peer: *mut ENetPeer) {
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
                (::core::mem::size_of::<[ENetProtocol; 32]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<ENetProtocol>() as libc::c_ulong)
                    as isize,
            ) as *mut ENetProtocol
            || buffer
                >= &mut *((*host).buffers).as_mut_ptr().offset(
                    (::core::mem::size_of::<[ENetBuffer; 65]>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<ENetBuffer>() as libc::c_ulong)
                        as isize,
                ) as *mut ENetBuffer
            || ((*peer).mtu as libc::c_ulong).wrapping_sub((*host).packetSize)
                < ::core::mem::size_of::<ENetProtocolAcknowledge>() as libc::c_ulong
        {
            (*peer).flags = ((*peer).flags as libc::c_int
                | ENET_PEER_FLAG_CONTINUE_SENDING as libc::c_int)
                as enet_uint16;
            break;
        } else {
            acknowledgement = currentAcknowledgement as *mut ENetAcknowledgement;
            currentAcknowledgement = (*currentAcknowledgement).next;
            (*buffer).data = command as *mut libc::c_void;
            (*buffer).dataLength =
                ::core::mem::size_of::<ENetProtocolAcknowledge>() as libc::c_ulong;
            (*host).packetSize = ((*host).packetSize as libc::c_ulong)
                .wrapping_add((*buffer).dataLength) as size_t
                as size_t;
            reliableSequenceNumber =
                htons((*acknowledgement).command.header.reliableSequenceNumber);
            (*command).header.command =
                ENET_PROTOCOL_COMMAND_ACKNOWLEDGE as libc::c_int as enet_uint8;
            (*command).header.channelID = (*acknowledgement).command.header.channelID;
            (*command).header.reliableSequenceNumber = reliableSequenceNumber;
            (*command).acknowledge.receivedReliableSequenceNumber = reliableSequenceNumber;
            (*command).acknowledge.receivedSentTime =
                htons((*acknowledgement).sentTime as uint16_t);
            if (*acknowledgement).command.header.command as libc::c_int
                & ENET_PROTOCOL_COMMAND_MASK as libc::c_int
                == ENET_PROTOCOL_COMMAND_DISCONNECT as libc::c_int
            {
                enet_protocol_dispatch_state(host, peer, ENET_PEER_STATE_ZOMBIE);
            }
            enet_list_remove(&mut (*acknowledgement).acknowledgementList);
            enet_free(acknowledgement as *mut libc::c_void);
            command = command.offset(1);
            buffer = buffer.offset(1);
        }
    }
    (*host).commandCount =
        command.offset_from(((*host).commands).as_mut_ptr()) as libc::c_long as size_t;
    (*host).bufferCount =
        buffer.offset_from(((*host).buffers).as_mut_ptr()) as libc::c_long as size_t;
}
unsafe fn enet_protocol_check_timeouts(
    mut host: *mut ENetHost,
    mut peer: *mut ENetPeer,
    mut event: *mut ENetEvent,
) -> libc::c_int {
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
            >= 86400000 as libc::c_int as libc::c_uint
        {
            ((*outgoingCommand).sentTime).wrapping_sub((*host).serviceTime)
        } else {
            ((*host).serviceTime).wrapping_sub((*outgoingCommand).sentTime)
        }) < (*outgoingCommand).roundTripTimeout
        {
            continue;
        }
        if (*peer).earliestTimeout == 0 as libc::c_int as libc::c_uint
            || ((*outgoingCommand).sentTime).wrapping_sub((*peer).earliestTimeout)
                >= 86400000 as libc::c_int as libc::c_uint
        {
            (*peer).earliestTimeout = (*outgoingCommand).sentTime;
        }
        if (*peer).earliestTimeout != 0 as libc::c_int as libc::c_uint
            && ((if ((*host).serviceTime).wrapping_sub((*peer).earliestTimeout)
                >= 86400000 as libc::c_int as libc::c_uint
            {
                ((*peer).earliestTimeout).wrapping_sub((*host).serviceTime)
            } else {
                ((*host).serviceTime).wrapping_sub((*peer).earliestTimeout)
            }) >= (*peer).timeoutMaximum
                || ((1 as libc::c_int)
                    << (*outgoingCommand).sendAttempts as libc::c_int - 1 as libc::c_int)
                    as libc::c_uint
                    >= (*peer).timeoutLimit
                    && (if ((*host).serviceTime).wrapping_sub((*peer).earliestTimeout)
                        >= 86400000 as libc::c_int as libc::c_uint
                    {
                        ((*peer).earliestTimeout).wrapping_sub((*host).serviceTime)
                    } else {
                        ((*host).serviceTime).wrapping_sub((*peer).earliestTimeout)
                    }) >= (*peer).timeoutMinimum)
        {
            enet_protocol_notify_disconnect(host, peer, event);
            return 1 as libc::c_int;
        }
        (*peer).packetsLost = ((*peer).packetsLost).wrapping_add(1);
        (*outgoingCommand).roundTripTimeout = ((*outgoingCommand).roundTripTimeout as libc::c_uint)
            .wrapping_mul(2 as libc::c_int as libc::c_uint)
            as enet_uint32 as enet_uint32;
        if !((*outgoingCommand).packet).is_null() {
            (*peer).reliableDataInTransit = ((*peer).reliableDataInTransit as libc::c_uint)
                .wrapping_sub((*outgoingCommand).fragmentLength as libc::c_uint)
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
    return 0 as libc::c_int;
}
unsafe fn enet_protocol_check_outgoing_commands(
    mut host: *mut ENetHost,
    mut peer: *mut ENetPeer,
    mut sentUnreliableCommands: *mut ENetList,
) -> libc::c_int {
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
    let mut reliableWindow: enet_uint16 = 0 as libc::c_int as enet_uint16;
    let mut commandSize: size_t = 0;
    let mut windowWrap: libc::c_int = 0 as libc::c_int;
    let mut canPing: libc::c_int = 1 as libc::c_int;
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
                    >= 86400000 as libc::c_int as libc::c_uint
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
        if (*outgoingCommand).command.header.command as libc::c_int
            & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int
            != 0
        {
            channel = if ((*outgoingCommand).command.header.channelID as libc::c_ulong)
                < (*peer).channelCount
            {
                &mut *((*peer).channels)
                    .offset((*outgoingCommand).command.header.channelID as isize)
                    as *mut ENetChannel
            } else {
                0 as *mut ENetChannel
            };
            reliableWindow = ((*outgoingCommand).reliableSequenceNumber as libc::c_int
                / ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int)
                as enet_uint16;
            if !channel.is_null() {
                if windowWrap != 0 {
                    continue;
                }
                if ((*outgoingCommand).sendAttempts as libc::c_int) < 1 as libc::c_int
                    && (*outgoingCommand).reliableSequenceNumber as libc::c_int
                        % ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int
                        == 0
                    && ((*channel).reliableWindows[((reliableWindow as libc::c_int
                        + ENET_PEER_RELIABLE_WINDOWS as libc::c_int
                        - 1 as libc::c_int)
                        % ENET_PEER_RELIABLE_WINDOWS as libc::c_int)
                        as usize] as libc::c_int
                        >= ENET_PEER_RELIABLE_WINDOW_SIZE as libc::c_int
                        || (*channel).usedReliableWindows as libc::c_int
                            & ((((1 as libc::c_int)
                                << ENET_PEER_FREE_RELIABLE_WINDOWS as libc::c_int
                                    + 2 as libc::c_int)
                                - 1 as libc::c_int)
                                << reliableWindow as libc::c_int
                                | ((1 as libc::c_int)
                                    << ENET_PEER_FREE_RELIABLE_WINDOWS as libc::c_int
                                        + 2 as libc::c_int)
                                    - 1 as libc::c_int
                                    >> ENET_PEER_RELIABLE_WINDOWS as libc::c_int
                                        - reliableWindow as libc::c_int)
                            != 0)
                {
                    windowWrap = 1 as libc::c_int;
                    currentSendReliableCommand = &mut (*peer).outgoingSendReliableCommands.sentinel;
                    continue;
                }
            }
            if !((*outgoingCommand).packet).is_null() {
                let mut windowSize: enet_uint32 = ((*peer).packetThrottle)
                    .wrapping_mul((*peer).windowSize)
                    .wrapping_div(ENET_PEER_PACKET_THROTTLE_SCALE as libc::c_int as libc::c_uint);
                if ((*peer).reliableDataInTransit)
                    .wrapping_add((*outgoingCommand).fragmentLength as libc::c_uint)
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
            canPing = 0 as libc::c_int;
        }
        commandSize = commandSizes[((*outgoingCommand).command.header.command as libc::c_int
            & ENET_PROTOCOL_COMMAND_MASK as libc::c_int)
            as usize];
        if command
            >= &mut *((*host).commands).as_mut_ptr().offset(
                (::core::mem::size_of::<[ENetProtocol; 32]>() as libc::c_ulong)
                    .wrapping_div(::core::mem::size_of::<ENetProtocol>() as libc::c_ulong)
                    as isize,
            ) as *mut ENetProtocol
            || buffer.offset(1 as libc::c_int as isize)
                >= &mut *((*host).buffers).as_mut_ptr().offset(
                    (::core::mem::size_of::<[ENetBuffer; 65]>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<ENetBuffer>() as libc::c_ulong)
                        as isize,
                ) as *mut ENetBuffer
            || ((*peer).mtu as libc::c_ulong).wrapping_sub((*host).packetSize) < commandSize
            || !((*outgoingCommand).packet).is_null()
                && (((*peer).mtu as libc::c_ulong).wrapping_sub((*host).packetSize) as enet_uint16
                    as libc::c_int)
                    < commandSize.wrapping_add((*outgoingCommand).fragmentLength as libc::c_ulong)
                        as enet_uint16 as libc::c_int
        {
            (*peer).flags = ((*peer).flags as libc::c_int
                | ENET_PEER_FLAG_CONTINUE_SENDING as libc::c_int)
                as enet_uint16;
            break;
        } else {
            if (*outgoingCommand).command.header.command as libc::c_int
                & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int
                != 0
            {
                if !channel.is_null()
                    && ((*outgoingCommand).sendAttempts as libc::c_int) < 1 as libc::c_int
                {
                    (*channel).usedReliableWindows = ((*channel).usedReliableWindows as libc::c_int
                        | (1 as libc::c_int) << reliableWindow as libc::c_int)
                        as enet_uint16;
                    (*channel).reliableWindows[reliableWindow as usize] =
                        ((*channel).reliableWindows[reliableWindow as usize]).wrapping_add(1);
                }
                (*outgoingCommand).sendAttempts = ((*outgoingCommand).sendAttempts).wrapping_add(1);
                if (*outgoingCommand).roundTripTimeout == 0 as libc::c_int as libc::c_uint {
                    (*outgoingCommand).roundTripTimeout = ((*peer).roundTripTime).wrapping_add(
                        (4 as libc::c_int as libc::c_uint)
                            .wrapping_mul((*peer).roundTripTimeVariance),
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
                (*host).headerFlags = ((*host).headerFlags as libc::c_int
                    | ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as libc::c_int)
                    as enet_uint16;
                (*peer).reliableDataInTransit = ((*peer).reliableDataInTransit as libc::c_uint)
                    .wrapping_add((*outgoingCommand).fragmentLength as libc::c_uint)
                    as enet_uint32 as enet_uint32;
            } else {
                if !((*outgoingCommand).packet).is_null()
                    && (*outgoingCommand).fragmentOffset == 0 as libc::c_int as libc::c_uint
                {
                    (*peer).packetThrottleCounter =
                        ((*peer).packetThrottleCounter as libc::c_uint).wrapping_add(
                            ENET_PEER_PACKET_THROTTLE_COUNTER as libc::c_int as libc::c_uint,
                        ) as enet_uint32 as enet_uint32;
                    (*peer).packetThrottleCounter =
                        ((*peer).packetThrottleCounter as libc::c_uint).wrapping_rem(
                            ENET_PEER_PACKET_THROTTLE_SCALE as libc::c_int as libc::c_uint,
                        ) as enet_uint32 as enet_uint32;
                    if (*peer).packetThrottleCounter > (*peer).packetThrottle {
                        let mut reliableSequenceNumber: enet_uint16 =
                            (*outgoingCommand).reliableSequenceNumber;
                        let mut unreliableSequenceNumber: enet_uint16 =
                            (*outgoingCommand).unreliableSequenceNumber;
                        loop {
                            (*(*outgoingCommand).packet).referenceCount =
                                ((*(*outgoingCommand).packet).referenceCount).wrapping_sub(1);
                            if (*(*outgoingCommand).packet).referenceCount
                                == 0 as libc::c_int as libc::c_ulong
                            {
                                enet_packet_destroy((*outgoingCommand).packet);
                            }
                            enet_list_remove(&mut (*outgoingCommand).outgoingCommandList);
                            enet_free(outgoingCommand as *mut libc::c_void);
                            if currentCommand
                                == &mut (*peer).outgoingCommands.sentinel as *mut ENetListNode
                            {
                                break;
                            }
                            outgoingCommand = currentCommand as *mut ENetOutgoingCommand;
                            if (*outgoingCommand).reliableSequenceNumber as libc::c_int
                                != reliableSequenceNumber as libc::c_int
                                || (*outgoingCommand).unreliableSequenceNumber as libc::c_int
                                    != unreliableSequenceNumber as libc::c_int
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
                        outgoingCommand as *mut libc::c_void,
                    );
                }
            }
            (*buffer).data = command as *mut libc::c_void;
            (*buffer).dataLength = commandSize;
            (*host).packetSize = ((*host).packetSize as libc::c_ulong)
                .wrapping_add((*buffer).dataLength) as size_t
                as size_t;
            *command = (*outgoingCommand).command;
            if !((*outgoingCommand).packet).is_null() {
                buffer = buffer.offset(1);
                (*buffer).data = ((*(*outgoingCommand).packet).data)
                    .offset((*outgoingCommand).fragmentOffset as isize)
                    as *mut libc::c_void;
                (*buffer).dataLength = (*outgoingCommand).fragmentLength as size_t;
                (*host).packetSize = ((*host).packetSize as libc::c_ulong)
                    .wrapping_add((*outgoingCommand).fragmentLength as libc::c_ulong)
                    as size_t as size_t;
            } else if (*outgoingCommand).command.header.command as libc::c_int
                & ENET_PROTOCOL_COMMAND_FLAG_ACKNOWLEDGE as libc::c_int
                == 0
            {
                enet_free(outgoingCommand as *mut libc::c_void);
            }
            (*peer).packetsSent = ((*peer).packetsSent).wrapping_add(1);
            command = command.offset(1);
            buffer = buffer.offset(1);
        }
    }
    (*host).commandCount =
        command.offset_from(((*host).commands).as_mut_ptr()) as libc::c_long as size_t;
    (*host).bufferCount =
        buffer.offset_from(((*host).buffers).as_mut_ptr()) as libc::c_long as size_t;
    if (*peer).state as libc::c_uint
        == ENET_PEER_STATE_DISCONNECT_LATER as libc::c_int as libc::c_uint
        && enet_peer_has_outgoing_commands(peer) == 0
        && (*sentUnreliableCommands).sentinel.next
            == &mut (*sentUnreliableCommands).sentinel as *mut ENetListNode
    {
        enet_peer_disconnect(peer, (*peer).eventData);
    }
    return canPing;
}
unsafe fn enet_protocol_send_outgoing_commands(
    mut host: *mut ENetHost,
    mut event: *mut ENetEvent,
    mut checkForTimeouts: libc::c_int,
) -> libc::c_int {
    let mut headerData: [enet_uint8; 8] = [0; 8];
    let mut header: *mut ENetProtocolHeader = headerData.as_mut_ptr() as *mut ENetProtocolHeader;
    let mut sentLength: libc::c_int = 0 as libc::c_int;
    let mut shouldCompress: size_t = 0 as libc::c_int as size_t;
    let mut sentUnreliableCommands: ENetList = ENetList {
        sentinel: ENetListNode {
            next: 0 as *mut _ENetListNode,
            previous: 0 as *mut _ENetListNode,
        },
    };
    enet_list_clear(&mut sentUnreliableCommands);
    let mut sendPass: libc::c_int = 0 as libc::c_int;
    let mut continueSending: libc::c_int = 0 as libc::c_int;
    while sendPass <= continueSending {
        let mut currentPeer: *mut ENetPeer = (*host).peers;
        while currentPeer
            < &mut *((*host).peers).offset((*host).peerCount as isize) as *mut ENetPeer
        {
            if !((*currentPeer).state as libc::c_uint
                == ENET_PEER_STATE_DISCONNECTED as libc::c_int as libc::c_uint
                || (*currentPeer).state as libc::c_uint
                    == ENET_PEER_STATE_ZOMBIE as libc::c_int as libc::c_uint
                || sendPass > 0 as libc::c_int
                    && (*currentPeer).flags as libc::c_int
                        & ENET_PEER_FLAG_CONTINUE_SENDING as libc::c_int
                        == 0)
            {
                (*currentPeer).flags = ((*currentPeer).flags as libc::c_int
                    & !(ENET_PEER_FLAG_CONTINUE_SENDING as libc::c_int))
                    as enet_uint16;
                (*host).headerFlags = 0 as libc::c_int as enet_uint16;
                (*host).commandCount = 0 as libc::c_int as size_t;
                (*host).bufferCount = 1 as libc::c_int as size_t;
                (*host).packetSize = ::core::mem::size_of::<ENetProtocolHeader>() as libc::c_ulong;
                if !((*currentPeer).acknowledgements.sentinel.next
                    == &mut (*currentPeer).acknowledgements.sentinel as *mut ENetListNode)
                {
                    enet_protocol_send_acknowledgements(host, currentPeer);
                }
                if checkForTimeouts != 0 as libc::c_int
                    && !((*currentPeer).sentReliableCommands.sentinel.next
                        == &mut (*currentPeer).sentReliableCommands.sentinel as *mut ENetListNode)
                    && !(((*host).serviceTime).wrapping_sub((*currentPeer).nextTimeout)
                        >= 86400000 as libc::c_int as libc::c_uint)
                    && enet_protocol_check_timeouts(host, currentPeer, event) == 1 as libc::c_int
                {
                    if !event.is_null()
                        && (*event).type_0 as libc::c_uint
                            != ENET_EVENT_TYPE_NONE as libc::c_int as libc::c_uint
                    {
                        return 1 as libc::c_int;
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
                            >= 86400000 as libc::c_int as libc::c_uint
                        {
                            ((*currentPeer).lastReceiveTime).wrapping_sub((*host).serviceTime)
                        } else {
                            ((*host).serviceTime).wrapping_sub((*currentPeer).lastReceiveTime)
                        }) >= (*currentPeer).pingInterval
                        && ((*currentPeer).mtu as libc::c_ulong).wrapping_sub((*host).packetSize)
                            >= ::core::mem::size_of::<ENetProtocolPing>() as libc::c_ulong
                    {
                        enet_peer_ping(currentPeer);
                        enet_protocol_check_outgoing_commands(
                            host,
                            currentPeer,
                            &mut sentUnreliableCommands,
                        );
                    }
                    if !((*host).commandCount == 0 as libc::c_int as libc::c_ulong) {
                        if (*currentPeer).packetLossEpoch == 0 as libc::c_int as libc::c_uint {
                            (*currentPeer).packetLossEpoch = (*host).serviceTime;
                        } else if (if ((*host).serviceTime)
                            .wrapping_sub((*currentPeer).packetLossEpoch)
                            >= 86400000 as libc::c_int as libc::c_uint
                        {
                            ((*currentPeer).packetLossEpoch).wrapping_sub((*host).serviceTime)
                        } else {
                            ((*host).serviceTime).wrapping_sub((*currentPeer).packetLossEpoch)
                        }) >= ENET_PEER_PACKET_LOSS_INTERVAL as libc::c_int
                            as libc::c_uint
                            && (*currentPeer).packetsSent > 0 as libc::c_int as libc::c_uint
                        {
                            let mut packetLoss: enet_uint32 = ((*currentPeer).packetsLost)
                                .wrapping_mul(
                                    ENET_PEER_PACKET_LOSS_SCALE as libc::c_int as libc::c_uint,
                                )
                                .wrapping_div((*currentPeer).packetsSent);
                            (*currentPeer).packetLossVariance = ((*currentPeer).packetLossVariance)
                                .wrapping_mul(3 as libc::c_int as libc::c_uint)
                                .wrapping_add(if packetLoss < (*currentPeer).packetLoss {
                                    ((*currentPeer).packetLoss).wrapping_sub(packetLoss)
                                } else {
                                    packetLoss.wrapping_sub((*currentPeer).packetLoss)
                                })
                                .wrapping_div(4 as libc::c_int as libc::c_uint);
                            (*currentPeer).packetLoss = ((*currentPeer).packetLoss)
                                .wrapping_mul(7 as libc::c_int as libc::c_uint)
                                .wrapping_add(packetLoss)
                                .wrapping_div(8 as libc::c_int as libc::c_uint);
                            (*currentPeer).packetLossEpoch = (*host).serviceTime;
                            (*currentPeer).packetsSent = 0 as libc::c_int as enet_uint32;
                            (*currentPeer).packetsLost = 0 as libc::c_int as enet_uint32;
                        }
                        let ref mut fresh34 = (*((*host).buffers).as_mut_ptr()).data;
                        *fresh34 = headerData.as_mut_ptr() as *mut libc::c_void;
                        if (*host).headerFlags as libc::c_int
                            & ENET_PROTOCOL_HEADER_FLAG_SENT_TIME as libc::c_int
                            != 0
                        {
                            (*header).sentTime = htons(
                                ((*host).serviceTime & 0xffff as libc::c_int as libc::c_uint)
                                    as uint16_t,
                            );
                            (*((*host).buffers).as_mut_ptr()).dataLength =
                                ::core::mem::size_of::<ENetProtocolHeader>() as libc::c_ulong;
                        } else {
                            (*((*host).buffers).as_mut_ptr()).dataLength = 2 as size_t;
                        }
                        shouldCompress = 0 as libc::c_int as size_t;
                        if !((*host).compressor.context).is_null()
                            && ((*host).compressor.compress).is_some()
                        {
                            let mut originalSize: size_t =
                                ((*host).packetSize).wrapping_sub(::core::mem::size_of::<
                                    ENetProtocolHeader,
                                >(
                                )
                                    as libc::c_ulong);
                            let mut compressedSize: size_t = ((*host).compressor.compress)
                                .expect("non-null function pointer")(
                                (*host).compressor.context,
                                &mut *((*host).buffers)
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize),
                                ((*host).bufferCount)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                originalSize,
                                ((*host).packetData[1 as libc::c_int as usize]).as_mut_ptr(),
                                originalSize,
                            );
                            if compressedSize > 0 as libc::c_int as libc::c_ulong
                                && compressedSize < originalSize
                            {
                                (*host).headerFlags = ((*host).headerFlags as libc::c_int
                                    | ENET_PROTOCOL_HEADER_FLAG_COMPRESSED as libc::c_int)
                                    as enet_uint16;
                                shouldCompress = compressedSize;
                            }
                        }
                        if ((*currentPeer).outgoingPeerID as libc::c_int)
                            < ENET_PROTOCOL_MAXIMUM_PEER_ID as libc::c_int
                        {
                            (*host).headerFlags = ((*host).headerFlags as libc::c_int
                                | ((*currentPeer).outgoingSessionID as libc::c_int)
                                    << ENET_PROTOCOL_HEADER_SESSION_SHIFT as libc::c_int)
                                as enet_uint16;
                        }
                        (*header).peerID = htons(
                            ((*currentPeer).outgoingPeerID as libc::c_int
                                | (*host).headerFlags as libc::c_int)
                                as uint16_t,
                        );
                        if ((*host).checksum).is_some() {
                            let mut checksum: *mut enet_uint32 = &mut *headerData
                                .as_mut_ptr()
                                .offset((*((*host).buffers).as_mut_ptr()).dataLength as isize)
                                as *mut enet_uint8
                                as *mut enet_uint32;
                            *checksum = if ((*currentPeer).outgoingPeerID as libc::c_int)
                                < ENET_PROTOCOL_MAXIMUM_PEER_ID as libc::c_int
                            {
                                (*currentPeer).connectID
                            } else {
                                0 as libc::c_int as libc::c_uint
                            };
                            let ref mut fresh35 = (*((*host).buffers).as_mut_ptr()).dataLength;
                            *fresh35 =
                                (*fresh35 as libc::c_ulong).wrapping_add(::core::mem::size_of::<
                                    enet_uint32,
                                >(
                                )
                                    as libc::c_ulong) as size_t
                                    as size_t;
                            *checksum = ((*host).checksum).expect("non-null function pointer")(
                                ((*host).buffers).as_mut_ptr(),
                                (*host).bufferCount,
                            );
                        }
                        if shouldCompress > 0 as libc::c_int as libc::c_ulong {
                            (*host).buffers[1 as libc::c_int as usize].data =
                                ((*host).packetData[1 as libc::c_int as usize]).as_mut_ptr()
                                    as *mut libc::c_void;
                            (*host).buffers[1 as libc::c_int as usize].dataLength = shouldCompress;
                            (*host).bufferCount = 2 as libc::c_int as size_t;
                        }
                        (*currentPeer).lastSendTime = (*host).serviceTime;
                        sentLength = enet_socket_send(
                            (*host).socket,
                            &mut (*currentPeer).address,
                            ((*host).buffers).as_mut_ptr(),
                            (*host).bufferCount,
                        );
                        enet_protocol_remove_sent_unreliable_commands(
                            currentPeer,
                            &mut sentUnreliableCommands,
                        );
                        if sentLength < 0 as libc::c_int {
                            return -(1 as libc::c_int);
                        }
                        (*host).totalSentData = ((*host).totalSentData as libc::c_uint)
                            .wrapping_add(sentLength as libc::c_uint)
                            as enet_uint32
                            as enet_uint32;
                        (*host).totalSentPackets = ((*host).totalSentPackets).wrapping_add(1);
                    }
                }
                if (*currentPeer).flags as libc::c_int
                    & ENET_PEER_FLAG_CONTINUE_SENDING as libc::c_int
                    != 0
                {
                    continueSending = sendPass + 1 as libc::c_int;
                }
            }
            currentPeer = currentPeer.offset(1);
        }
        sendPass += 1;
    }
    return 0 as libc::c_int;
}
pub unsafe fn enet_host_flush(mut host: *mut ENetHost) {
    (*host).serviceTime = enet_time_get();
    enet_protocol_send_outgoing_commands(host, 0 as *mut ENetEvent, 0 as libc::c_int);
}
pub unsafe fn enet_host_check_events(
    mut host: *mut ENetHost,
    mut event: *mut ENetEvent,
) -> libc::c_int {
    if event.is_null() {
        return -(1 as libc::c_int);
    }
    (*event).type_0 = ENET_EVENT_TYPE_NONE;
    (*event).peer = 0 as *mut ENetPeer;
    (*event).packet = 0 as *mut ENetPacket;
    return enet_protocol_dispatch_incoming_commands(host, event);
}
pub unsafe fn enet_host_service(
    mut host: *mut ENetHost,
    mut event: *mut ENetEvent,
    mut timeout: enet_uint32,
) -> libc::c_int {
    let mut waitCondition: enet_uint32 = 0;
    if !event.is_null() {
        (*event).type_0 = ENET_EVENT_TYPE_NONE;
        (*event).peer = 0 as *mut ENetPeer;
        (*event).packet = 0 as *mut ENetPacket;
        match enet_protocol_dispatch_incoming_commands(host, event) {
            1 => return 1 as libc::c_int,
            -1 => return -(1 as libc::c_int),
            _ => {}
        }
    }
    (*host).serviceTime = enet_time_get();
    timeout =
        (timeout as libc::c_uint).wrapping_add((*host).serviceTime) as enet_uint32 as enet_uint32;
    loop {
        if (if ((*host).serviceTime).wrapping_sub((*host).bandwidthThrottleEpoch)
            >= 86400000 as libc::c_int as libc::c_uint
        {
            ((*host).bandwidthThrottleEpoch).wrapping_sub((*host).serviceTime)
        } else {
            ((*host).serviceTime).wrapping_sub((*host).bandwidthThrottleEpoch)
        }) >= ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL as libc::c_int as libc::c_uint
        {
            enet_host_bandwidth_throttle(host);
        }
        match enet_protocol_send_outgoing_commands(host, event, 1 as libc::c_int) {
            1 => return 1 as libc::c_int,
            -1 => return -(1 as libc::c_int),
            _ => {}
        }
        match enet_protocol_receive_incoming_commands(host, event) {
            1 => return 1 as libc::c_int,
            -1 => return -(1 as libc::c_int),
            _ => {}
        }
        match enet_protocol_send_outgoing_commands(host, event, 1 as libc::c_int) {
            1 => return 1 as libc::c_int,
            -1 => return -(1 as libc::c_int),
            _ => {}
        }
        if !event.is_null() {
            match enet_protocol_dispatch_incoming_commands(host, event) {
                1 => return 1 as libc::c_int,
                -1 => return -(1 as libc::c_int),
                _ => {}
            }
        }
        if !(((*host).serviceTime).wrapping_sub(timeout) >= 86400000 as libc::c_int as libc::c_uint)
        {
            return 0 as libc::c_int;
        }
        loop {
            (*host).serviceTime = enet_time_get();
            if !(((*host).serviceTime).wrapping_sub(timeout)
                >= 86400000 as libc::c_int as libc::c_uint)
            {
                return 0 as libc::c_int;
            }
            waitCondition = (ENET_SOCKET_WAIT_RECEIVE as libc::c_int
                | ENET_SOCKET_WAIT_INTERRUPT as libc::c_int)
                as enet_uint32;
            if enet_socket_wait(
                (*host).socket,
                &mut waitCondition,
                if timeout.wrapping_sub((*host).serviceTime)
                    >= 86400000 as libc::c_int as libc::c_uint
                {
                    ((*host).serviceTime).wrapping_sub(timeout)
                } else {
                    timeout.wrapping_sub((*host).serviceTime)
                },
            ) != 0 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            if !(waitCondition & ENET_SOCKET_WAIT_INTERRUPT as libc::c_int as libc::c_uint != 0) {
                break;
            }
        }
        (*host).serviceTime = enet_time_get();
        if !(waitCondition & ENET_SOCKET_WAIT_RECEIVE as libc::c_int as libc::c_uint != 0) {
            break;
        }
    }
    return 0 as libc::c_int;
}
static mut timeBase: enet_uint32 = 0 as libc::c_int as enet_uint32;
pub unsafe fn enet_initialize() -> libc::c_int {
    return 0 as libc::c_int;
}
pub unsafe fn enet_deinitialize() {}
pub unsafe fn enet_host_random_seed() -> enet_uint32 {
    return time(0 as *mut time_t) as enet_uint32;
}
pub unsafe fn enet_time_get() -> enet_uint32 {
    let mut timeVal: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    gettimeofday(&mut timeVal, 0 as *mut libc::c_void);
    return (timeVal.tv_sec * 1000 as libc::c_int as libc::c_long
        + timeVal.tv_usec / 1000 as libc::c_int as libc::c_long
        - timeBase as libc::c_long) as enet_uint32;
}
pub unsafe fn enet_time_set(mut newTimeBase: enet_uint32) {
    let mut timeVal: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    gettimeofday(&mut timeVal, 0 as *mut libc::c_void);
    timeBase = (timeVal.tv_sec * 1000 as libc::c_int as libc::c_long
        + timeVal.tv_usec / 1000 as libc::c_int as libc::c_long
        - newTimeBase as libc::c_long) as enet_uint32;
}
pub unsafe fn enet_address_set_host_ip(
    mut address: *mut ENetAddress,
    mut name: *const libc::c_char,
) -> libc::c_int {
    if inet_pton(
        2 as libc::c_int,
        name,
        &mut (*address).host as *mut enet_uint32 as *mut libc::c_void,
    ) == 0
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe fn enet_address_set_host(
    mut address: *mut ENetAddress,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut resultList: *mut addrinfo = 0 as *mut addrinfo;
    let mut result: *mut addrinfo = 0 as *mut addrinfo;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 2 as libc::c_int;
    if getaddrinfo(
        name,
        0 as *const libc::c_char,
        0 as *const addrinfo,
        &mut resultList,
    ) != 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    result = resultList;
    while !result.is_null() {
        if (*result).ai_family == 2 as libc::c_int
            && !((*result).ai_addr).is_null()
            && (*result).ai_addrlen as libc::c_ulong
                >= ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong
        {
            let mut sin: *mut sockaddr_in = (*result).ai_addr as *mut sockaddr_in;
            (*address).host = (*sin).sin_addr.s_addr;
            freeaddrinfo(resultList);
            return 0 as libc::c_int;
        }
        result = (*result).ai_next;
    }
    if !resultList.is_null() {
        freeaddrinfo(resultList);
    }
    return enet_address_set_host_ip(address, name);
}
pub unsafe fn enet_address_get_host_ip(
    mut address: *const ENetAddress,
    mut name: *mut libc::c_char,
    mut nameLength: size_t,
) -> libc::c_int {
    if (inet_ntop(
        2 as libc::c_int,
        &(*address).host as *const enet_uint32 as *const libc::c_void,
        name,
        nameLength as socklen_t,
    ))
    .is_null()
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe fn enet_address_get_host(
    mut address: *const ENetAddress,
    mut name: *mut libc::c_char,
    mut nameLength: size_t,
) -> libc::c_int {
    let mut sin: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut err: libc::c_int = 0;
    memset(
        &mut sin as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    sin.sin_family = 2 as libc::c_int as sa_family_t;
    sin.sin_port = htons((*address).port);
    sin.sin_addr.s_addr = (*address).host;
    err = getnameinfo(
        &mut sin as *mut sockaddr_in as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
        name,
        nameLength as socklen_t,
        0 as *mut libc::c_char,
        0 as libc::c_int as socklen_t,
        8 as libc::c_int,
    );
    if err == 0 {
        if !name.is_null()
            && nameLength > 0 as libc::c_int as libc::c_ulong
            && (memchr(name as *const libc::c_void, '\0' as i32, nameLength)).is_null()
        {
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    }
    if err != -(2 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return enet_address_get_host_ip(address, name, nameLength);
}
pub unsafe fn enet_socket_bind(
    mut socket_0: ENetSocket,
    mut address: *const ENetAddress,
) -> libc::c_int {
    let mut sin: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    memset(
        &mut sin as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    sin.sin_family = 2 as libc::c_int as sa_family_t;
    if !address.is_null() {
        sin.sin_port = htons((*address).port);
        sin.sin_addr.s_addr = (*address).host;
    } else {
        sin.sin_port = 0 as libc::c_int as in_port_t;
        sin.sin_addr.s_addr = 0 as libc::c_int as in_addr_t;
    }
    return bind(
        socket_0,
        &mut sin as *mut sockaddr_in as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
}
pub unsafe fn enet_socket_get_address(
    mut socket_0: ENetSocket,
    mut address: *mut ENetAddress,
) -> libc::c_int {
    let mut sin: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut sinLength: socklen_t =
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    if getsockname(
        socket_0,
        &mut sin as *mut sockaddr_in as *mut sockaddr,
        &mut sinLength,
    ) == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    (*address).host = sin.sin_addr.s_addr;
    (*address).port = ntohs(sin.sin_port);
    return 0 as libc::c_int;
}
pub unsafe fn enet_socket_listen(
    mut socket_0: ENetSocket,
    mut backlog: libc::c_int,
) -> libc::c_int {
    return listen(
        socket_0,
        if backlog < 0 as libc::c_int {
            4096 as libc::c_int
        } else {
            backlog
        },
    );
}
pub unsafe fn enet_socket_create(mut type_0: ENetSocketType) -> ENetSocket {
    return socket(
        2 as libc::c_int,
        if type_0 as libc::c_uint == ENET_SOCKET_TYPE_DATAGRAM as libc::c_int as libc::c_uint {
            SOCK_DGRAM as libc::c_int
        } else {
            SOCK_STREAM as libc::c_int
        },
        0 as libc::c_int,
    );
}
pub unsafe fn enet_socket_set_option(
    mut socket_0: ENetSocket,
    mut option: ENetSocketOption,
    mut value: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = -(1 as libc::c_int);
    match option as libc::c_uint {
        1 => {
            result = fcntl(
                socket_0,
                4 as libc::c_int,
                (if value != 0 {
                    0o4000 as libc::c_int
                } else {
                    0 as libc::c_int
                }) | fcntl(socket_0, 3 as libc::c_int) & !(0o4000 as libc::c_int),
            );
        }
        2 => {
            result = setsockopt(
                socket_0,
                1 as libc::c_int,
                6 as libc::c_int,
                &mut value as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
            );
        }
        5 => {
            result = setsockopt(
                socket_0,
                1 as libc::c_int,
                2 as libc::c_int,
                &mut value as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
            );
        }
        3 => {
            result = setsockopt(
                socket_0,
                1 as libc::c_int,
                8 as libc::c_int,
                &mut value as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
            );
        }
        4 => {
            result = setsockopt(
                socket_0,
                1 as libc::c_int,
                7 as libc::c_int,
                &mut value as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
            );
        }
        6 => {
            let mut timeVal: timeval = timeval {
                tv_sec: 0,
                tv_usec: 0,
            };
            timeVal.tv_sec = (value / 1000 as libc::c_int) as __time_t;
            timeVal.tv_usec = (value % 1000 as libc::c_int * 1000 as libc::c_int) as __suseconds_t;
            result = setsockopt(
                socket_0,
                1 as libc::c_int,
                20 as libc::c_int,
                &mut timeVal as *mut timeval as *mut libc::c_char as *const libc::c_void,
                ::core::mem::size_of::<timeval>() as libc::c_ulong as socklen_t,
            );
        }
        7 => {
            let mut timeVal_0: timeval = timeval {
                tv_sec: 0,
                tv_usec: 0,
            };
            timeVal_0.tv_sec = (value / 1000 as libc::c_int) as __time_t;
            timeVal_0.tv_usec =
                (value % 1000 as libc::c_int * 1000 as libc::c_int) as __suseconds_t;
            result = setsockopt(
                socket_0,
                1 as libc::c_int,
                21 as libc::c_int,
                &mut timeVal_0 as *mut timeval as *mut libc::c_char as *const libc::c_void,
                ::core::mem::size_of::<timeval>() as libc::c_ulong as socklen_t,
            );
        }
        9 => {
            result = setsockopt(
                socket_0,
                IPPROTO_TCP as libc::c_int,
                1 as libc::c_int,
                &mut value as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
            );
        }
        10 => {
            result = setsockopt(
                socket_0,
                IPPROTO_IP as libc::c_int,
                2 as libc::c_int,
                &mut value as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
            );
        }
        _ => {}
    }
    return if result == -(1 as libc::c_int) {
        -(1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
}
pub unsafe fn enet_socket_get_option(
    mut socket_0: ENetSocket,
    mut option: ENetSocketOption,
    mut value: *mut libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = -(1 as libc::c_int);
    let mut len: socklen_t = 0;
    match option as libc::c_uint {
        8 => {
            len = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
            result = getsockopt(
                socket_0,
                1 as libc::c_int,
                4 as libc::c_int,
                value as *mut libc::c_void,
                &mut len,
            );
        }
        10 => {
            len = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
            result = getsockopt(
                socket_0,
                IPPROTO_IP as libc::c_int,
                2 as libc::c_int,
                value as *mut libc::c_char as *mut libc::c_void,
                &mut len,
            );
        }
        _ => {}
    }
    return if result == -(1 as libc::c_int) {
        -(1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
}
pub unsafe fn enet_socket_connect(
    mut socket_0: ENetSocket,
    mut address: *const ENetAddress,
) -> libc::c_int {
    let mut sin: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut result: libc::c_int = 0;
    memset(
        &mut sin as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    sin.sin_family = 2 as libc::c_int as sa_family_t;
    sin.sin_port = htons((*address).port);
    sin.sin_addr.s_addr = (*address).host;
    result = connect(
        socket_0,
        &mut sin as *mut sockaddr_in as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    if result == -(1 as libc::c_int) && *__errno_location() == 115 as libc::c_int {
        return 0 as libc::c_int;
    }
    return result;
}
pub unsafe fn enet_socket_accept(
    mut socket_0: ENetSocket,
    mut address: *mut ENetAddress,
) -> ENetSocket {
    let mut result: libc::c_int = 0;
    let mut sin: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut sinLength: socklen_t =
        ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    result = accept(
        socket_0,
        if !address.is_null() {
            &mut sin as *mut sockaddr_in as *mut sockaddr
        } else {
            0 as *mut sockaddr
        },
        if !address.is_null() {
            &mut sinLength
        } else {
            0 as *mut socklen_t
        },
    );
    if result == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if !address.is_null() {
        (*address).host = sin.sin_addr.s_addr;
        (*address).port = ntohs(sin.sin_port);
    }
    return result;
}
pub unsafe fn enet_socket_shutdown(
    mut socket_0: ENetSocket,
    mut how: ENetSocketShutdown,
) -> libc::c_int {
    return shutdown(socket_0, how as libc::c_int);
}
pub unsafe fn enet_socket_destroy(mut socket_0: ENetSocket) {
    if socket_0 != -(1 as libc::c_int) {
        close(socket_0);
    }
}
pub unsafe fn enet_socket_send(
    mut socket_0: ENetSocket,
    mut address: *const ENetAddress,
    mut buffers: *const ENetBuffer,
    mut bufferCount: size_t,
) -> libc::c_int {
    let mut msgHdr: msghdr = msghdr {
        msg_name: 0 as *mut libc::c_void,
        msg_namelen: 0,
        msg_iov: 0 as *mut iovec,
        msg_iovlen: 0,
        msg_control: 0 as *mut libc::c_void,
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut sin: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut sentLength: libc::c_int = 0;
    memset(
        &mut msgHdr as *mut msghdr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<msghdr>() as libc::c_ulong,
    );
    if !address.is_null() {
        memset(
            &mut sin as *mut sockaddr_in as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong,
        );
        sin.sin_family = 2 as libc::c_int as sa_family_t;
        sin.sin_port = htons((*address).port);
        sin.sin_addr.s_addr = (*address).host;
        msgHdr.msg_name = &mut sin as *mut sockaddr_in as *mut libc::c_void;
        msgHdr.msg_namelen = ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    }
    msgHdr.msg_iov = buffers as *mut iovec;
    msgHdr.msg_iovlen = bufferCount;
    sentLength = sendmsg(socket_0, &mut msgHdr, MSG_NOSIGNAL as libc::c_int) as libc::c_int;
    if sentLength == -(1 as libc::c_int) {
        if *__errno_location() == 11 as libc::c_int {
            return 0 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    return sentLength;
}
pub unsafe fn enet_socket_receive(
    mut socket_0: ENetSocket,
    mut address: *mut ENetAddress,
    mut buffers: *mut ENetBuffer,
    mut bufferCount: size_t,
) -> libc::c_int {
    let mut msgHdr: msghdr = msghdr {
        msg_name: 0 as *mut libc::c_void,
        msg_namelen: 0,
        msg_iov: 0 as *mut iovec,
        msg_iovlen: 0,
        msg_control: 0 as *mut libc::c_void,
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut sin: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut recvLength: libc::c_int = 0;
    memset(
        &mut msgHdr as *mut msghdr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<msghdr>() as libc::c_ulong,
    );
    if !address.is_null() {
        msgHdr.msg_name = &mut sin as *mut sockaddr_in as *mut libc::c_void;
        msgHdr.msg_namelen = ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t;
    }
    msgHdr.msg_iov = buffers as *mut iovec;
    msgHdr.msg_iovlen = bufferCount;
    recvLength = recvmsg(socket_0, &mut msgHdr, MSG_NOSIGNAL as libc::c_int) as libc::c_int;
    if recvLength == -(1 as libc::c_int) {
        if *__errno_location() == 11 as libc::c_int {
            return 0 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    if msgHdr.msg_flags & MSG_TRUNC as libc::c_int != 0 {
        return -(2 as libc::c_int);
    }
    if !address.is_null() {
        (*address).host = sin.sin_addr.s_addr;
        (*address).port = ntohs(sin.sin_port);
    }
    return recvLength;
}
pub unsafe fn enet_socketset_select(
    mut maxSocket: ENetSocket,
    mut readSet: *mut ENetSocketSet,
    mut writeSet: *mut ENetSocketSet,
    mut timeout: enet_uint32,
) -> libc::c_int {
    let mut timeVal: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    timeVal.tv_sec = timeout.wrapping_div(1000 as libc::c_int as libc::c_uint) as __time_t;
    timeVal.tv_usec = timeout
        .wrapping_rem(1000 as libc::c_int as libc::c_uint)
        .wrapping_mul(1000 as libc::c_int as libc::c_uint) as __suseconds_t;
    return select(
        maxSocket + 1 as libc::c_int,
        readSet,
        writeSet,
        0 as *mut fd_set,
        &mut timeVal,
    );
}
pub unsafe fn enet_socket_wait(
    mut socket_0: ENetSocket,
    mut condition: *mut enet_uint32,
    mut timeout: enet_uint32,
) -> libc::c_int {
    let mut pollSocket: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut pollCount: libc::c_int = 0;
    pollSocket.fd = socket_0;
    pollSocket.events = 0 as libc::c_int as libc::c_short;
    if *condition & ENET_SOCKET_WAIT_SEND as libc::c_int as libc::c_uint != 0 {
        pollSocket.events =
            (pollSocket.events as libc::c_int | 0x4 as libc::c_int) as libc::c_short;
    }
    if *condition & ENET_SOCKET_WAIT_RECEIVE as libc::c_int as libc::c_uint != 0 {
        pollSocket.events =
            (pollSocket.events as libc::c_int | 0x1 as libc::c_int) as libc::c_short;
    }
    pollCount = poll(
        &mut pollSocket,
        1 as libc::c_int as nfds_t,
        timeout as libc::c_int,
    );
    if pollCount < 0 as libc::c_int {
        if *__errno_location() == 4 as libc::c_int
            && *condition & ENET_SOCKET_WAIT_INTERRUPT as libc::c_int as libc::c_uint != 0
        {
            *condition = ENET_SOCKET_WAIT_INTERRUPT as libc::c_int as enet_uint32;
            return 0 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    *condition = ENET_SOCKET_WAIT_NONE as libc::c_int as enet_uint32;
    if pollCount == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if pollSocket.revents as libc::c_int & 0x4 as libc::c_int != 0 {
        *condition |= ENET_SOCKET_WAIT_SEND as libc::c_int as libc::c_uint;
    }
    if pollSocket.revents as libc::c_int & 0x1 as libc::c_int != 0 {
        *condition |= ENET_SOCKET_WAIT_RECEIVE as libc::c_int as libc::c_uint;
    }
    return 0 as libc::c_int;
}
