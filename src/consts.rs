pub const PROTOCOL_MAXIMUM_FRAGMENT_COUNT: u32 = 1024 * 1024;
pub const PROTOCOL_MAXIMUM_PEER_ID: u32 = 4095;
pub const PROTOCOL_MAXIMUM_CHANNEL_COUNT: u32 = 255;
pub const PROTOCOL_MINIMUM_CHANNEL_COUNT: u32 = 1;
pub const PROTOCOL_MAXIMUM_WINDOW_SIZE: u32 = 65536;
pub const PROTOCOL_MINIMUM_WINDOW_SIZE: u32 = 4096;
pub const PROTOCOL_MAXIMUM_PACKET_COMMANDS: u32 = 32;
pub const PROTOCOL_MAXIMUM_MTU: usize = 4096;
pub const PROTOCOL_MINIMUM_MTU: usize = 576;
pub const PEER_FREE_RELIABLE_WINDOWS: u32 = 8;
pub const PEER_RELIABLE_WINDOW_SIZE: u32 = 0x1000;
pub const PEER_RELIABLE_WINDOWS: u32 = 16;
pub const PEER_FREE_UNSEQUENCED_WINDOWS: u32 = 32;
pub const PEER_UNSEQUENCED_WINDOW_SIZE: u32 = 1024;
pub const PEER_UNSEQUENCED_WINDOWS: u32 = 64;
pub const PEER_PING_INTERVAL: u32 = 500;
pub const PEER_TIMEOUT_MAXIMUM: u32 = 30000;
pub const PEER_TIMEOUT_MINIMUM: u32 = 5000;
pub const PEER_TIMEOUT_LIMIT: u32 = 32;
pub const PEER_WINDOW_SIZE_SCALE: u32 = 64 * 1024;
pub const PEER_PACKET_LOSS_INTERVAL: u32 = 10000;
pub const PEER_PACKET_LOSS_SCALE: u32 = 1 << 16;
pub const PEER_PACKET_THROTTLE_INTERVAL: u32 = 5000;
pub const PEER_PACKET_THROTTLE_DECELERATION: u32 = 2;
pub const PEER_PACKET_THROTTLE_ACCELERATION: u32 = 2;
pub const PEER_PACKET_THROTTLE_COUNTER: u32 = 7;
pub const PEER_PACKET_THROTTLE_SCALE: u32 = 32;
pub const PEER_DEFAULT_PACKET_THROTTLE: u32 = 32;
pub const PEER_DEFAULT_ROUND_TRIP_TIME: u32 = 500;
pub const HOST_DEFAULT_MAXIMUM_WAITING_DATA: u32 = 32 * 1024 * 1024;
pub const HOST_DEFAULT_MAXIMUM_PACKET_SIZE: u32 = 32 * 1024 * 1024;
pub const HOST_DEFAULT_MTU: u32 = 1392;
pub const HOST_BANDWIDTH_THROTTLE_INTERVAL: u32 = 1000;
pub const HOST_SEND_BUFFER_SIZE: u32 = 256 * 1024;
pub const HOST_RECEIVE_BUFFER_SIZE: u32 = 256 * 1024;

pub const BUFFER_MAXIMUM: u32 = PROTOCOL_MAXIMUM_PACKET_COMMANDS * 2 + 1;
