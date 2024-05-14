use std::{convert::Infallible, time::Duration};

use rusty_enet as enet;

fn main() {
    let mut host1 = enet::Host::new(
        enet::ReadWrite::<(), Infallible>::new(),
        enet::HostSettings::default(),
    )
    .unwrap();
    let mut host2 = enet::Host::new(
        enet::ReadWrite::<(), Infallible>::new(),
        enet::HostSettings::default(),
    )
    .unwrap();

    macro_rules! update {
        () => {
            for _ in 0..10 {
                std::thread::sleep(Duration::from_millis(1));
                update_host("Host 1", &mut host1, &mut host2);
                update_host("Host 2", &mut host2, &mut host1);
            }
        };
    }

    host1.connect((), 255, 100).unwrap();

    update!();

    host1
        .peer_mut(enet::PeerID(0))
        .send(0, &enet::Packet::reliable("Hello!".as_bytes()))
        .unwrap();

    update!();

    host1.peer_mut(enet::PeerID(0)).disconnect(32);

    update!();
}

fn update_host(
    name: &str,
    host: &mut enet::Host<enet::ReadWrite<(), Infallible>>,
    other_host: &mut enet::Host<enet::ReadWrite<(), Infallible>>,
) {
    while let Some(event) = host.service().unwrap() {
        match event {
            enet::Event::Connect { peer, data } => {
                println!(
                    "[{}] Connected to {:?} with data: {}",
                    name,
                    peer.id(),
                    data
                );
            }
            enet::Event::Disconnect { peer, data } => {
                println!(
                    "[{}] Disconnected from {:?} with data: {}",
                    name,
                    peer.id(),
                    data
                );
            }
            enet::Event::Receive {
                peer,
                packet,
                channel_id,
            } => {
                let message = std::str::from_utf8(packet.data()).unwrap();
                println!(
                    "[{}] Received message from {:?} on channel {}: {}",
                    name,
                    peer.id(),
                    channel_id,
                    message
                );
            }
        }
    }
    if let Some((_, packet)) = host.socket_mut().read() {
        other_host.socket_mut().write((), packet);
    }
}
