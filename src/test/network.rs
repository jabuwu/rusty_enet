use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::{mpsc, Arc, RwLock},
    time::Duration,
};

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

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

#[derive(Debug, Clone, Copy)]
pub struct NetworkConditions {
    round_trip_time: u32,
    round_trip_time_variance: u32,
    loss_chance: f32,
}

impl NetworkConditions {
    pub const fn perfect() -> Self {
        Self {
            round_trip_time: 0,
            round_trip_time_variance: 0,
            loss_chance: 0.,
        }
    }

    pub const fn good() -> Self {
        Self {
            round_trip_time: 50,
            round_trip_time_variance: 30,
            loss_chance: 0.05,
        }
    }

    pub const fn bad() -> Self {
        Self {
            round_trip_time: 300,
            round_trip_time_variance: 100,
            loss_chance: 0.2,
        }
    }

    pub const fn disconnected() -> Self {
        Self {
            round_trip_time: 0,
            round_trip_time_variance: 0,
            loss_chance: 1.,
        }
    }
}

pub struct NetworkEvent {
    sent: bool,
    send_time: u32,
    from: usize,
    to: usize,
    data: Vec<u8>,
}

pub struct Network {
    rng: ChaCha20Rng,
    sockets: Vec<Socket>,
    events: Vec<NetworkEvent>,
    hosts: Vec<enet::Host<Socket>>,
    conditions: HashMap<(usize, usize), NetworkConditions>,
    connections: HashMap<(usize, usize), enet::PeerID>,
    time: Arc<RwLock<u32>>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            rng: ChaCha20Rng::seed_from_u64(0),
            sockets: Vec::default(),
            events: Vec::default(),
            hosts: Vec::default(),
            conditions: HashMap::default(),
            connections: HashMap::default(),
            time: Arc::default(),
        }
    }

    fn send_and_receive(&mut self, time: u32) {
        for (from, socket) in self.sockets.iter_mut().enumerate() {
            while let Some((to, data)) = socket.receive() {
                if let Some(conditions) = self.conditions.get(&(to, from)).copied() {
                    if self.rng.gen_bool(1. - conditions.loss_chance as f64) {
                        self.events.push(NetworkEvent {
                            sent: false,
                            send_time: time
                                + (conditions.round_trip_time as i32
                                    + self.rng.gen_range(
                                        -(conditions.round_trip_time_variance as i32)
                                            ..=conditions.round_trip_time_variance as i32,
                                    ))
                                .max(0) as u32,
                            from,
                            to,
                            data,
                        });
                    }
                }
            }
        }
        for event in &mut self.events {
            if time >= event.send_time {
                self.sockets[event.to].send(event.from, &event.data);
                event.sent = true;
            }
        }
        self.events.retain(|event| !event.sent);
    }

    /// Update n frames, where each frame is 1ms.
    pub fn update(&mut self, frames: usize) -> Vec<Event> {
        let mut events = vec![];
        for _ in 0..frames {
            let now = *self.time.write().unwrap();
            for host_index in 0..self.hosts.len() {
                loop {
                    self.send_and_receive(now);
                    let host = &mut self.hosts[host_index];
                    if let Some(event) = host.service().unwrap() {
                        let peer_index: usize;
                        match &event {
                            enet::Event::Connect { peer, .. } => {
                                peer_index = peer.address().unwrap();
                                self.connections.insert((host_index, peer_index), peer.id());
                            }
                            enet::Event::Disconnect { peer, .. } => {
                                peer_index = peer.address().unwrap();
                                self.conditions.remove(&(host_index, peer_index));
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
                    } else {
                        break;
                    }
                }
            }
            let mut time = self.time.write().unwrap();
            *time = time.wrapping_add(1);
        }
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
        self.conditions(from, to, NetworkConditions::perfect());
    }

    pub fn conditions(&mut self, host1: usize, host2: usize, conditions: NetworkConditions) {
        self.conditions.insert((host1, host2), conditions);
        self.conditions.insert((host2, host1), conditions);
    }

    pub fn disconnect(&mut self, from: usize, to: usize, data: u32) {
        let peer = self.resolve_peer(from, to);
        self.hosts[from].peer_mut(peer).disconnect(data);
    }

    pub fn disconnect_later(&mut self, from: usize, to: usize, data: u32) {
        let peer = self.resolve_peer(from, to);
        self.hosts[from].peer_mut(peer).disconnect_later(data);
    }

    pub fn disconnect_now(&mut self, from: usize, to: usize, data: u32) {
        let peer = self.resolve_peer(from, to);
        self.hosts[from].peer_mut(peer).disconnect_now(data);
        self.conditions.remove(&(from, to));
    }

    pub fn send(&mut self, from: usize, to: usize, channel_id: u8, packet: &enet::Packet) {
        let peer = self.resolve_peer(from, to);
        self.hosts[from]
            .peer_mut(peer)
            .send(channel_id, packet)
            .unwrap();
    }

    pub fn round_trip_time(&self, from: usize, to: usize) -> Duration {
        let peer = self.resolve_peer(from, to);
        self.hosts[from].peer(peer).round_trip_time()
    }
}

pub struct Host {
    host: enet::Host<Socket>,
    address: usize,
}

impl Host {
    pub const fn address(&self) -> usize {
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

#[derive(Debug, Clone)]
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
    pub const fn from(&self) -> usize {
        self.from
    }

    pub const fn to(&self) -> usize {
        self.to
    }

    pub const fn is_connect(&self) -> bool {
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

    pub const fn is_disconnect(&self) -> bool {
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

    pub const fn is_receive(&self) -> bool {
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
