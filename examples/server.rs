use std::{
    ffi::{CStr, CString},
    mem::zeroed,
    time::Duration,
};

use rusty_enet::{
    enet_address_set_host_ip, enet_host_create, enet_host_service, enet_peer_send, ENetAddress,
    ENET_EVENT_TYPE_CONNECT, ENET_EVENT_TYPE_DISCONNECT, ENET_EVENT_TYPE_RECEIVE,
};

fn make_address(ip: &str, port: u16) -> ENetAddress {
    unsafe {
        let mut address: ENetAddress = zeroed();
        let ip = CString::new(ip).unwrap();
        assert_eq!(enet_address_set_host_ip(&mut address, ip.as_ptr()), 0);
        address.port = port;
        address
    }
}

fn main() {
    unsafe {
        let mut bind_address = make_address("127.0.0.1", 6060);
        let host = enet_host_create(&mut bind_address, 32, 2, 0, 0);
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
