use std::{
    ffi::CStr,
    mem::zeroed,
    net::{SocketAddr, UdpSocket},
    str::FromStr,
    time::Duration,
};

use rusty_enet::{
    enet_host_create, enet_host_service, enet_peer_send, ENET_EVENT_TYPE_CONNECT,
    ENET_EVENT_TYPE_DISCONNECT, ENET_EVENT_TYPE_RECEIVE,
};

fn make_address(ip: &str, port: u16) -> SocketAddr {
    SocketAddr::from_str(&format!("{}:{}", ip, port)).unwrap()
}

fn main() {
    unsafe {
        let bind_address = make_address("127.0.0.1", 6060);
        let host = enet_host_create::<UdpSocket>(bind_address, 32, 2, 0, 0);
        loop {
            let mut event = zeroed();
            let result = enet_host_service(host, &mut event);
            if result > 0 {
                match event.type_0 {
                    ENET_EVENT_TYPE_CONNECT => {
                        println!("Peer {} connected!", event.peer.offset_from((*host).peers));
                    }
                    ENET_EVENT_TYPE_DISCONNECT => {
                        println!(
                            "Peer {} disconnected!",
                            event.peer.offset_from((*host).peers)
                        );
                    }
                    ENET_EVENT_TYPE_RECEIVE => {
                        let message = CStr::from_ptr((*event.packet).data as *const i8);
                        println!("Received packet: {:?}", message);
                        enet_peer_send(event.peer, event.channelID, event.packet);
                    }
                    _ => unreachable!(),
                }
            } else if result < 0 {
                panic!("Error");
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}
