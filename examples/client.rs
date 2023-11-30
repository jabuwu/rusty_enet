use std::{
    ffi::{CStr, CString},
    mem::zeroed,
    time::Duration,
};

use rusty_enet::{
    enet_address_set_host_ip, enet_host_connect, enet_host_create, enet_host_service,
    enet_packet_create, enet_peer_send, ENetAddress, ENET_EVENT_TYPE_CONNECT,
    ENET_EVENT_TYPE_DISCONNECT, ENET_EVENT_TYPE_RECEIVE, ENET_PACKET_FLAG_RELIABLE, c_void,
};

fn make_address(ip: &str, port: u16) -> ENetAddress {
    unsafe {
        let mut address: ENetAddress = zeroed();
        let ip = CString::new(ip).unwrap();
        enet_address_set_host_ip(&mut address, ip.as_ptr());
        address.port = port;
        address
    }
}

fn main() {
    unsafe {
        let mut bind_address = make_address("0.0.0.0", 0);
        let host = enet_host_create(&mut bind_address, 1, 2, 0, 0);
        assert!(!host.is_null());
        let mut connect_address = make_address("127.0.0.1", 6060);
        let peer = enet_host_connect(host, &mut connect_address, 2, 0);
        assert!(!peer.is_null());
        loop {
            let mut event = zeroed();
            let result = enet_host_service(host, &mut event, 0);
            if result > 0 {
                match event.type_0 {
                    ENET_EVENT_TYPE_CONNECT => {
                        println!("Connected!");
                        let message = CString::new("hello world").unwrap();
                        let packet = enet_packet_create(
                            message.as_ptr() as *mut c_void,
                            12,
                            ENET_PACKET_FLAG_RELIABLE,
                        );
                        enet_peer_send(peer, 0, packet);
                    }
                    ENET_EVENT_TYPE_DISCONNECT => {
                        println!("Disconnected!");
                    }
                    ENET_EVENT_TYPE_RECEIVE => {
                        let message = CStr::from_ptr((*event.packet).data as *const i8);
                        println!("Received packet: {:?}", message);
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
