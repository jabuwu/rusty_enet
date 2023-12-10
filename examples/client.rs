use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket},
    str::{self, FromStr},
    time::Duration,
};

use rusty_enet as enet;

fn main() {
    let socket =
        UdpSocket::bind(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0))).unwrap();
    let mut host = enet::Host::<UdpSocket>::create(
        socket,
        enet::HostSettings {
            peer_limit: 1,
            channel_limit: 2,
            compressor: Some(Box::new(enet::RangeCoder::new())),
            checksum_fn: Some(Box::new(enet::crc32)),
            ..Default::default()
        },
    )
    .unwrap();
    let address = SocketAddr::from_str("127.0.0.1:6060").unwrap();
    let peer = host.connect(address, 2, 0).unwrap();
    peer.set_ping_interval(100);
    loop {
        while let Some(event) = host.service().unwrap() {
            match event {
                enet::Event::Connect { peer, .. } => {
                    println!("Connected");
                    let packet = enet::Packet::reliable("hello world".as_bytes());
                    _ = peer.send(0, packet);
                }
                enet::Event::Disconnect { .. } => {
                    println!("Disconnected");
                }
                enet::Event::Receive { packet, .. } => {
                    if let Ok(message) = str::from_utf8(packet.data()) {
                        println!("Received packet: {:?}", message);
                    }
                }
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}
