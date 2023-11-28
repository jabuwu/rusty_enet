use std::{
    io::{self, ErrorKind},
    net::{SocketAddr, UdpSocket},
};

use crate::Address;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketOption {
    SendBuffer,
    ReceiveBuffer,
    Broadcast,
    NonBlocking,
}

#[allow(clippy::type_complexity)]
pub trait Socket: Sized {
    type BindAddress;
    type PeerAddress: Address;
    type Error: std::error::Error + 'static;

    fn bind(address: Self::BindAddress) -> Result<Self, Self::Error>;
    fn set_option(&mut self, option: SocketOption, value: i32) -> Result<(), Self::Error>;
    fn send(&mut self, address: Self::PeerAddress, buffer: &[u8]) -> Result<usize, Self::Error>;
    fn receive(&mut self, mtu: usize) -> Result<Option<(Self::PeerAddress, Vec<u8>)>, Self::Error>;
}

impl Socket for UdpSocket {
    type BindAddress = SocketAddr;
    type PeerAddress = SocketAddr;
    type Error = io::Error;

    fn bind(address: SocketAddr) -> Result<Self, io::Error> {
        UdpSocket::bind(address)
    }

    fn set_option(&mut self, option: SocketOption, value: i32) -> Result<(), io::Error> {
        match option {
            SocketOption::NonBlocking => Ok(self.set_nonblocking(value != 0)?),
            SocketOption::Broadcast => Ok(self.set_broadcast(value != 0)?),
            SocketOption::ReceiveBuffer => Ok(()), // TODO?
            SocketOption::SendBuffer => Ok(()),    // TODO?
        }
    }

    fn send(&mut self, address: SocketAddr, buffer: &[u8]) -> Result<usize, io::Error> {
        match self.send_to(buffer, address) {
            Ok(sent_length) => Ok(sent_length),
            Err(err) if err.kind() == ErrorKind::WouldBlock => Ok(0),
            Err(err) => Err(err),
        }
    }

    fn receive(&mut self, mtu: usize) -> Result<Option<(SocketAddr, Vec<u8>)>, io::Error> {
        let mut buffer = vec![0; mtu];
        match self.recv_from(&mut buffer) {
            Ok((recv_length, recv_addr)) => {
                // TODO: MSG_TRUNC?
                Ok(Some((recv_addr, Vec::from(&buffer[0..recv_length]))))
            }
            Err(err) if err.kind() == ErrorKind::WouldBlock => Ok(None),
            Err(err) => Err(err),
        }
    }
}
