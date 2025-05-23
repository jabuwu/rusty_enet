use core::net::{SocketAddr, SocketAddrV4, SocketAddrV6};
use std::net::{IpAddr, Ipv4Addr};

/// An address type, for use with the [`Socket`](`crate::Socket`) trait.
pub trait Address: Sized + Clone {
    /// Are the two addresses the same host?
    ///
    /// For IP based addresses, this checks if the IP of two addresses are the same.
    fn same_host(&self, other: &Self) -> bool;
    /// Are the two addresses exactly the same?
    ///
    /// For IP based addresses, this checks if the IP and port of two addresses are the same.
    fn same(&self, other: &Self) -> bool;
    /// Is this a broadcast address?
    ///
    /// For IP based addresses, checks if this is the IPv4 broadcast address.
    fn is_broadcast(&self) -> bool;
    fn port(&self) -> u16;
    fn address(&self) -> IpAddr;
}

impl Address for () {
    fn same_host(&self, _other: &()) -> bool {
        true
    }

    fn same(&self, _other: &()) -> bool {
        true
    }

    fn is_broadcast(&self) -> bool {
        false
    }
    fn port(&self) -> u16 {
        0
    }
    fn address(&self) -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
    }
}

impl Address for SocketAddrV4 {
    fn same_host(&self, other: &Self) -> bool {
        self.ip() == other.ip()
    }

    fn same(&self, other: &Self) -> bool {
        *self == *other
    }

    fn is_broadcast(&self) -> bool {
        self.ip().is_broadcast()
    }
}

impl Address for SocketAddrV6 {
    fn same_host(&self, other: &Self) -> bool {
        self.ip() == other.ip()
    }

    fn same(&self, other: &Self) -> bool {
        *self == *other
    }

    fn is_broadcast(&self) -> bool {
        false
    }
}

impl Address for SocketAddr {
    fn same_host(&self, other: &SocketAddr) -> bool {
        self.ip() == other.ip()
    }

    fn same(&self, other: &SocketAddr) -> bool {
        *self == *other
    }

    fn is_broadcast(&self) -> bool {
        match self {
            SocketAddr::V4(self_addr_v4) => self_addr_v4.ip().is_broadcast(),
            _ => false,
        }
    }

    fn port(&self) -> u16 {
        self.port()
    }
    fn address(&self) -> IpAddr {
        self.ip()
    }
}
