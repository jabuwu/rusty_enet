use std::net::SocketAddr;

pub trait Address: Clone {
    fn same_host(&self, other: &Self) -> bool;
    fn same(&self, other: &Self) -> bool;
    fn is_broadcast(&self) -> bool;
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
}
