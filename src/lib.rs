// https://github.com/rust-lang/rust-clippy/issues/11382
#![allow(clippy::arc_with_non_send_sync)]

pub(crate) const ENET_HOST_DEFAULT_MAXIMUM_WAITING_DATA: u32 = 33554432;
pub(crate) const ENET_HOST_DEFAULT_MAXIMUM_PACKET_SIZE: u32 = 33554432;
pub(crate) const ENET_HOST_DEFAULT_MTU: u32 = 1392;
pub(crate) const ENET_HOST_BANDWIDTH_THROTTLE_INTERVAL: u32 = 1000;
pub(crate) const ENET_HOST_SEND_BUFFER_SIZE: i32 = 262144;
pub(crate) const ENET_HOST_RECEIVE_BUFFER_SIZE: i32 = 262144;

pub(crate) const ENET_PEER_FREE_RELIABLE_WINDOWS: u16 = 8;
pub(crate) const ENET_PEER_RELIABLE_WINDOW_SIZE: u16 = 4096;
pub(crate) const ENET_PEER_RELIABLE_WINDOWS: u16 = 16;
pub(crate) const ENET_PEER_FREE_UNSEQUENCED_WINDOWS: u32 = 32;
pub(crate) const ENET_PEER_UNSEQUENCED_WINDOW_SIZE: usize = 1024;
pub(crate) const ENET_PEER_PING_INTERVAL: u32 = 500;
pub(crate) const ENET_PEER_TIMEOUT_MAXIMUM: u32 = 30000;
pub(crate) const ENET_PEER_TIMEOUT_MINIMUM: u32 = 5000;
pub(crate) const ENET_PEER_TIMEOUT_LIMIT: u32 = 32;
pub(crate) const ENET_PEER_WINDOW_SIZE_SCALE: u32 = 65536;
pub(crate) const ENET_PEER_PACKET_LOSS_INTERVAL: u32 = 10000;
pub(crate) const ENET_PEER_PACKET_LOSS_SCALE: u32 = 65536;
pub(crate) const ENET_PEER_PACKET_THROTTLE_INTERVAL: u32 = 5000;
pub(crate) const ENET_PEER_PACKET_THROTTLE_DECELERATION: u32 = 2;
pub(crate) const ENET_PEER_PACKET_THROTTLE_ACCELERATION: u32 = 2;
pub(crate) const ENET_PEER_PACKET_THROTTLE_COUNTER: u32 = 7;
pub(crate) const ENET_PEER_PACKET_THROTTLE_SCALE: u32 = 32;
pub(crate) const ENET_PEER_DEFAULT_PACKET_THROTTLE: u32 = 32;
pub(crate) const ENET_PEER_DEFAULT_ROUND_TRIP_TIME: u32 = 500;

pub(crate) const ENET_PROTOCOL_MAXIMUM_FRAGMENT_COUNT: u32 = 1048576;
pub(crate) const ENET_PROTOCOL_MAXIMUM_PEER_ID: u16 = 4095;
pub(crate) const ENET_PROTOCOL_MAXIMUM_CHANNEL_COUNT: usize = 255;
pub(crate) const ENET_PROTOCOL_MINIMUM_CHANNEL_COUNT: usize = 1;
pub(crate) const ENET_PROTOCOL_MAXIMUM_WINDOW_SIZE: u32 = 65536;
pub(crate) const ENET_PROTOCOL_MINIMUM_WINDOW_SIZE: u32 = 4096;
pub(crate) const ENET_PROTOCOL_MAXIMUM_PACKET_COMMANDS: usize = 32;
pub(crate) const ENET_PROTOCOL_MAXIMUM_MTU: u32 = 4096;
pub(crate) const ENET_PROTOCOL_MINIMUM_MTU: u32 = 576;

mod address;
mod buffer;
mod channel;
mod error;
mod event;
mod host;
mod list;
mod os;
mod packet;
mod peer;
mod protocol;
mod socket;
mod time;

pub use address::*;
pub(crate) use buffer::*;
pub(crate) use channel::*;
pub use error::*;
pub use event::*;
pub use host::*;
pub(crate) use list::*;
pub(crate) use os::*;
pub use packet::*;
pub use peer::*;
pub(crate) use protocol::*;
pub use socket::*;
pub(crate) use time::*;

pub(crate) const SENT_TIME_OFFSET: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl Version {
    pub fn current() -> Self {
        Self {
            major: 1,
            minor: 3,
            patch: 17,
        }
    }
}
