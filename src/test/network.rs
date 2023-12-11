use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::{mpsc, Arc, RwLock},
    time::Duration,
};

use crate as enet;

pub struct Socket {
    sender: mpsc::Sender<(usize, Vec<u8>)>,
    receiver: mpsc::Receiver<(usize, Vec<u8>)>,
}

impl Socket {
    fn connect() -> (Socket, Socket) {
        let (sender1, receiver2) = mpsc::channel();
        let (sender2, receiver1) = mpsc::channel();
        (
            Socket {
                sender: sender1,
                receiver: receiver1,
            },
            Socket {
                sender: sender2,
                receiver: receiver2,
            },
        )
    }

    fn send(&mut self, address: usize, data: &[u8]) {
        self.sender.send((address, data.to_vec())).unwrap();
    }

    fn receive(&mut self) -> Option<(usize, Vec<u8>)> {
        match self.receiver.recv_timeout(Duration::ZERO) {
            Ok((address, data)) => Some((address, data)),
            Err(mpsc::RecvTimeoutError::Timeout) => None,
            Err(mpsc::RecvTimeoutError::Disconnected) => unreachable!(),
        }
    }
}

impl enet::Socket for Socket {
    type PeerAddress = usize;
    type Error = enet::Error;

    fn init(&mut self, _socket_options: enet::SocketOptions) -> Result<(), Self::Error> {
        Ok(())
    }

    fn send(&mut self, address: Self::PeerAddress, buffer: &[u8]) -> Result<usize, Self::Error> {
        Socket::send(self, address, buffer);
        Ok(buffer.len())
    }

    fn receive(
        &mut self,
        _mtu: usize,
    ) -> Result<Option<(Self::PeerAddress, enet::PacketReceived)>, Self::Error> {
        if let Some((address, data)) = Socket::receive(self) {
            Ok(Some((address, enet::PacketReceived::Complete(data))))
        } else {
            Ok(None)
        }
    }
}

impl enet::Address for usize {
    fn same_host(&self, other: &usize) -> bool {
        *self == *other
    }

    fn same(&self, other: &usize) -> bool {
        *self == *other
    }

    fn is_broadcast(&self) -> bool {
        false
    }
}

#[derive(Default)]
pub struct Network {
    sockets: Vec<Socket>,
    hosts: Vec<enet::Host<Socket>>,
    connections: HashMap<(usize, usize), enet::PeerID>,
    time: Arc<RwLock<u32>>,
}

impl Network {
    pub fn new() -> Self {
        Self::default()
    }

    fn send_and_receive(&mut self) {
        let mut events = vec![];
        for (from, socket) in self.sockets.iter_mut().enumerate() {
            while let Some(event) = socket.receive() {
                events.push((from, event.0, event.1));
            }
        }
        for (from, to, data) in events {
            self.sockets[to].send(from, &data);
        }
    }

    pub fn update(&mut self, time: Duration) -> Vec<Event> {
        macro_rules! send_and_receive {
            () => {
                let mut events = vec![];
                for (from, socket) in self.sockets.iter_mut().enumerate() {
                    while let Some(event) = socket.receive() {
                        events.push((from, event.0, event.1));
                    }
                }
                for (from, to, data) in events {
                    self.sockets[to].send(from, &data);
                }
            };
        }
        let mut events = vec![];
        for (host_index, host) in self.hosts.iter_mut().enumerate() {
            send_and_receive!();
            while let Some(event) = host.service().unwrap() {
                let peer_index: usize;
                match &event {
                    enet::Event::Connect { peer, .. } => {
                        peer_index = peer.address().unwrap();
                        self.connections.insert((host_index, peer_index), peer.id());
                    }
                    enet::Event::Disconnect { peer, .. } => {
                        peer_index = peer.address().unwrap();
                        self.connections.remove(&(host_index, peer_index));
                    }
                    enet::Event::Receive { peer, .. } => {
                        peer_index = peer.address().unwrap();
                    }
                }
                events.push(Event {
                    from: peer_index,
                    to: host_index,
                    event: event.no_ref(),
                });
                send_and_receive!();
            }
        }
        *self.time.write().unwrap() += (time.as_millis() % u32::MAX as u128) as u32;
        events
    }

    pub fn create_host(&mut self, mut settings: enet::HostSettings) -> usize {
        let index = self.hosts.len();
        let time = self.time.clone();
        settings.time = Some(Box::new(move || {
            Duration::from_millis(*time.read().unwrap() as u64)
        }));
        settings.seed = Some(0);
        let (network_socket, host_socket) = Socket::connect();
        self.sockets.push(network_socket);
        self.hosts
            .push(enet::Host::create(host_socket, settings).unwrap());
        index
    }

    pub fn resolve_peer(&self, from: usize, to: usize) -> enet::PeerID {
        self.connections[&(from, to)]
    }

    pub fn connect(&mut self, from: usize, to: usize, channel_count: usize, data: u32) {
        self.hosts[from].connect(to, channel_count, data).unwrap();
    }

    pub fn disconnect(&mut self, from: usize, to: usize, data: u32) {
        let peer = self.resolve_peer(from, to);
        self.hosts[from].peer_mut(peer).disconnect(data)
    }

    pub fn send(&mut self, from: usize, to: usize, channel_id: u8, packet: enet::Packet) {
        let peer = self.resolve_peer(from, to);
        self.hosts[from]
            .peer_mut(peer)
            .send(channel_id, packet)
            .unwrap();
    }
}

pub struct Host {
    host: enet::Host<Socket>,
    address: usize,
}

impl Host {
    pub fn address(&self) -> usize {
        self.address
    }
}

impl Deref for Host {
    type Target = enet::Host<Socket>;

    fn deref(&self) -> &Self::Target {
        &self.host
    }
}

impl DerefMut for Host {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.host
    }
}

#[derive(Clone)]
pub struct Event {
    from: usize,
    to: usize,
    event: enet::EventNoRef,
}

#[derive(Debug, Clone)]
pub struct EventConnect {
    pub from: usize,
    pub to: usize,
    pub peer: enet::PeerID,
    pub data: u32,
}

#[derive(Debug, Clone)]
pub struct EventDisconnect {
    pub from: usize,
    pub to: usize,
    pub peer: enet::PeerID,
    pub data: u32,
}

#[derive(Debug, Clone)]
pub struct EventReceive {
    pub from: usize,
    pub to: usize,
    pub peer: enet::PeerID,
    pub channel_id: u8,
    pub packet: enet::Packet,
}

impl Event {
    pub fn from(&self) -> usize {
        self.from
    }

    pub fn to(&self) -> usize {
        self.to
    }

    pub fn is_connect(&self) -> bool {
        matches!(&self.event, enet::EventNoRef::Connect { .. })
    }

    pub fn is_connect_and(&self, and: impl Fn(EventConnect) -> bool) -> bool {
        if let enet::EventNoRef::Connect { peer, data } = &self.event {
            and(EventConnect {
                from: self.from,
                to: self.to,
                peer: *peer,
                data: *data,
            })
        } else {
            false
        }
    }

    pub fn is_disconnect(&self) -> bool {
        matches!(&self.event, enet::EventNoRef::Disconnect { .. })
    }

    pub fn is_disconnect_and(&self, and: impl Fn(EventDisconnect) -> bool) -> bool {
        if let enet::EventNoRef::Disconnect { peer, data } = &self.event {
            and(EventDisconnect {
                from: self.from,
                to: self.to,
                peer: *peer,
                data: *data,
            })
        } else {
            false
        }
    }

    pub fn is_receive(&self) -> bool {
        matches!(&self.event, enet::EventNoRef::Receive { .. })
    }

    pub fn is_receive_and(&self, and: impl Fn(EventReceive) -> bool) -> bool {
        if let enet::EventNoRef::Receive {
            peer,
            channel_id,
            packet,
        } = &self.event
        {
            and(EventReceive {
                from: self.from,
                to: self.to,
                peer: *peer,
                channel_id: *channel_id,
                packet: packet.clone(),
            })
        } else {
            false
        }
    }
}
