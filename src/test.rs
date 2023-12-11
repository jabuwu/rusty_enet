use std::time::Duration;

use crate as enet;

mod network;
use network::*;

#[test]
fn events() {
    let mut network = Network::new();
    let mut host1 = network.create_host(enet::HostSettings {
        peer_limit: 1,
        ..Default::default()
    });
    let mut host2 = network.create_host(enet::HostSettings {
        peer_limit: 1,
        ..Default::default()
    });

    network.connect(host1, host2, 255, 5);
    network.update(Duration::from_millis(10));
    let events = network.update(Duration::from_millis(10));
    assert_eq!(events.len(), 2);
    assert!(events[0].is_connect_and(|event| event.to == host1
        && event.from == host2
        && event.peer == enet::PeerID(0)
        && event.data == 0));
    assert!(events[1].is_connect_and(|event| event.to == host2
        && event.from == host1
        && event.peer == enet::PeerID(0)
        && event.data == 5));

    network.send(
        host1,
        host2,
        0,
        enet::Packet::reliable("hello world".as_bytes()),
    );
    let events = network.update(Duration::from_millis(10));
    assert_eq!(events.len(), 1);
    assert!(events[0].is_receive_and(|event| event.from == host1
        && event.to == host2
        && event.channel_id == 0
        && event.packet.data().len() == 11
        && event.packet.kind() == enet::PacketKind::Reliable));

    network.disconnect(host1, host2, 10);
    let events = network.update(Duration::from_millis(10));
    assert_eq!(events.len(), 1);
    assert!(events[0]
        .is_disconnect_and(|event| event.from == host1 && event.to == host2 && event.data == 10));
    let events = network.update(Duration::from_millis(10));
    assert_eq!(events.len(), 1);
    assert!(events[0]
        .is_disconnect_and(|event| event.from == host2 && event.to == host1 && event.data == 0));
}
