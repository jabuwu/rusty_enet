use std::{
    ffi::{CStr, CString},
    mem::zeroed,
    net::{SocketAddr, UdpSocket},
    str::FromStr,
    time::Duration,
};

use rusty_enet::{
    c_void, enet_host_connect, enet_host_create, enet_host_service, enet_packet_create,
    enet_peer_send, ENET_EVENT_TYPE_CONNECT, ENET_EVENT_TYPE_DISCONNECT, ENET_EVENT_TYPE_RECEIVE,
    ENET_PACKET_FLAG_RELIABLE,
};

fn make_address(ip: &str, port: u16) -> SocketAddr {
    SocketAddr::from_str(&format!("{}:{}", ip, port)).unwrap()
}

fn main() {
    unsafe {
        let bind_address = make_address("0.0.0.0", 0);
        let host = enet_host_create::<UdpSocket>(bind_address, 1, 2, 0, 0);
        assert!(!host.is_null());
        let connect_address = make_address("127.0.0.1", 6060);
        let peer = enet_host_connect(host, connect_address, 2, 0);
        assert!(!peer.is_null());
        loop {
            let mut event = zeroed();
            let result = enet_host_service(host, &mut event);
            if result > 0 {
                match event.type_0 {
                    ENET_EVENT_TYPE_CONNECT => {
                        println!("Connected!");
                        let message = CString::new("hello world").unwrap();
                        let packet = enet_packet_create(
                            message.as_ptr() as *mut c_void,
                            message.to_bytes().len() as u64,
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
