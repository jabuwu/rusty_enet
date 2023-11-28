use std::{
    net::{SocketAddr, UdpSocket},
    str::{self, FromStr},
    time::Duration,
};

use rusty_enet::{Event, Host};

fn main() {
    let address = SocketAddr::from_str("127.0.0.1:6060").unwrap();
    let mut network = Host::<UdpSocket>::create(address, 32, 2, 0, 0).unwrap();
    loop {
        while let Some(event) = network.service().unwrap() {
            match &event {
                Event::Connect { peer, .. } => {
                    println!("Peer {} connected", peer.0);
                }
                Event::Disconnect { peer, .. } => {
                    println!("Peer {} disconnected", peer.0);
                }
                Event::Receive { packet, .. } => {
                    println!("{}", str::from_utf8(packet.data()).unwrap());
                }
                _ => (),
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}
