use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket},
    str::FromStr,
    time::Duration,
};

use rusty_enet::{Event, Host, Packet};

fn main() {
    let mut network = Host::<UdpSocket>::create(
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)),
        1,
        2,
        0,
        0,
    )
    .unwrap();
    let address = SocketAddr::from_str("127.0.0.1:6060").unwrap();
    let peer = network.connect(address, 2, 0).unwrap();
    _ = network.set_ping_interval(peer, 100);
    loop {
        while let Some(event) = network.service().unwrap() {
            if matches!(event, Event::Connect { .. }) {
                println!("Connection to 127.0.0.1:6060 succeeded.");
                let packet = Packet::reliable("hello world".as_bytes());
                _ = network.send(peer, 0, packet);
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}