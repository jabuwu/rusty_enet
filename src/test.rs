use crate as enet;

#[allow(dead_code)]
mod network;
use network::*;

#[test]
fn events() {
    let mut network = Network::new();
    let host1 = network.create_host(enet::HostSettings {
        peer_limit: 1,
        ..Default::default()
    });
    let host2 = network.create_host(enet::HostSettings {
        peer_limit: 1,
        ..Default::default()
    });

    network.connect(host1, host2, 255, 5);
    network.update(1);
    let events = network.update(1);
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
        &enet::Packet::reliable("hello world".as_bytes()),
    );
    let events = network.update(1);
    assert_eq!(events.len(), 1);
    assert!(events[0].is_receive_and(|event| event.from == host1
        && event.to == host2
        && event.channel_id == 0
        && event.packet.data().len() == 11
        && event.packet.kind() == enet::PacketKind::Reliable));

    network.disconnect(host1, host2, 10);
    let events = network.update(1);
    assert_eq!(events.len(), 1);
    assert!(events[0]
        .is_disconnect_and(|event| event.from == host1 && event.to == host2 && event.data == 10));
    let events = network.update(1);
    assert_eq!(events.len(), 1);
    assert!(events[0]
        .is_disconnect_and(|event| event.from == host2 && event.to == host1 && event.data == 0));
}

#[test]
fn resend_reliable_packet() {
    let mut network = Network::new();
    let host1 = network.create_host(enet::HostSettings {
        peer_limit: 1,
        ..Default::default()
    });
    let host2 = network.create_host(enet::HostSettings {
        peer_limit: 1,
        ..Default::default()
    });

    network.connect(host1, host2, 255, 5);
    network.update(2);

    network.send(
        host1,
        host2,
        0,
        &enet::Packet::reliable("reliable".as_bytes()),
    );
    network.send(
        host1,
        host2,
        0,
        &enet::Packet::unreliable("unreliable".as_bytes()),
    );
    let events = network.update(1);
    assert_eq!(events.len(), 2);
    assert!(events[0].is_receive_and(|event| event.from == host1
        && event.to == host2
        && event.channel_id == 0
        && event.packet.data() == "reliable".as_bytes()));
    assert!(events[1].is_receive_and(|event| event.from == host1
        && event.to == host2
        && event.channel_id == 0
        && event.packet.data() == "unreliable".as_bytes()));

    network.conditions(host1, host2, NetworkConditions::disconnected());
    network.send(
        host1,
        host2,
        0,
        &enet::Packet::reliable("reliable1".as_bytes()),
    );
    network.send(
        host1,
        host2,
        0,
        &enet::Packet::reliable("reliable2".as_bytes()),
    );
    network.send(
        host1,
        host2,
        0,
        &enet::Packet::unreliable("unreliable".as_bytes()),
    );
    let events = network.update(1);
    assert_eq!(events.len(), 0);
    assert!(events.is_empty());
    network.conditions(host1, host2, NetworkConditions::perfect());
    let events = network.update(enet::consts::PEER_DEFAULT_ROUND_TRIP_TIME as usize - 1);
    assert_eq!(events.len(), 0);

    let events = network.update(1);
    assert_eq!(events.len(), 2);
    assert!(events[0].is_receive_and(|event| event.from == host1
        && event.to == host2
        && event.channel_id == 0
        && event.packet.data() == "reliable1".as_bytes()));
    assert!(events[1].is_receive_and(|event| event.from == host1
        && event.to == host2
        && event.channel_id == 0
        && event.packet.data() == "reliable2".as_bytes()));
    let events = network.update(10000);
    assert_eq!(events.len(), 0);
}

#[test]
fn round_trip_time() {
    let mut network = Network::new();
    let host1 = network.create_host(enet::HostSettings {
        peer_limit: 1,
        ..Default::default()
    });
    let host2 = network.create_host(enet::HostSettings {
        peer_limit: 1,
        ..Default::default()
    });

    network.connect(host1, host2, 255, 5);
    network.update(2);

    network.conditions(host1, host2, NetworkConditions::perfect());
    network.update(10000);
    assert_eq!(network.round_trip_time(host1, host2).as_millis(), 1);

    network.conditions(host1, host2, NetworkConditions::good());
    network.update(10000);
    assert_eq!(network.round_trip_time(host1, host2).as_millis(), 93);

    network.conditions(host1, host2, NetworkConditions::bad());
    network.update(10000);
    assert_eq!(network.round_trip_time(host1, host2).as_millis(), 302);
}

#[test]
fn timeout() {
    let mut network = Network::new();
    let host1 = network.create_host(enet::HostSettings {
        peer_limit: 1,
        ..Default::default()
    });
    let host2 = network.create_host(enet::HostSettings {
        peer_limit: 1,
        ..Default::default()
    });

    network.connect(host1, host2, 255, 5);
    network.update(2);

    network.conditions(host1, host2, NetworkConditions::disconnected());
    let events = network.update(5614);
    assert_eq!(events.len(), 0);
    let events = network.update(1);
    assert_eq!(events.len(), 1);
    assert!(events[0].is_disconnect_and(|event| event.from == 0));
    let events = network.update(26383);
    assert_eq!(events.len(), 0);
    let events = network.update(1);
    assert_eq!(events.len(), 1);
    assert!(events[0].is_disconnect_and(|event| event.from == 1));
}
