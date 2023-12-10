use std::{
    net::{SocketAddr, UdpSocket},
    str::{self, FromStr},
    time::Duration,
};

use rusty_enet as enet;

fn main() {
    let socket = UdpSocket::bind(SocketAddr::from_str("127.0.0.1:6060").unwrap()).unwrap();
    let mut host = enet::Host::create(
        socket,
        enet::HostSettings {
            peer_limit: 32,
            channel_limit: 2,
            ..Default::default()
        },
    )
    .unwrap();

    loop {
        while let Some(event) = host.service().unwrap() {
            match event {
                enet::Event::Connect { peer, .. } => {
                    println!("Peer {} connected", peer.id().0);
                }
                enet::Event::Disconnect { peer, .. } => {
                    println!("Peer {} disconnected", peer.id().0);
                }
                enet::Event::Receive {
                    peer,
                    channel_id,
                    packet,
                } => {
                    if let Ok(message) = str::from_utf8(packet.data()) {
                        println!("Received packet: {:?}", message);
                    }
                    _ = peer.send(channel_id, packet);
                }
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}
