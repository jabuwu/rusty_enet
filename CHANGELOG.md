# 0.4.0
- Add missing `Error` impl on `HostNewError`
- Implement `Address` trait for `SocketAddrV4` and `SocketAddrV6`
- Add `ToRawPacket` trait to support zero-copy packet types ([#5](https://github.com/jabuwu/rusty_enet/pull/5))
  - Change `Packet::new` and helper functions to take `impl ToRawPacket` instead of `&[u8]`

# 0.3.3
- Reset peer on connection error in `enet::connected`

# 0.3.2
- Fix a `Duration` underflow panic in `enet::connected`

# 0.3.1
- Add missing `enet::connected::Host::now` function

# 0.3.0
- Add `SocketError` trait
- Add `BadParameter` type in `HostNewError::BadParameter`
- Add `Host::now` to get the current time according to ENet
- Add `connected` feature, providing an easier integration with connection based protocols
- Allow `ReadWrite` in `#![no_std]` environments
- Change `Host::set_mtu` and `Peer::set_mtu` to take `&mut self` instead of `&self`

# 0.2.0
- Compatible with ENet 1.3.18
- Refine some trait requirements and derives
- Reduce allocations introduced by Rust port ([#1](https://github.com/jabuwu/rusty_enet/issues/1))
- Reduce `enet_malloc` overhead
- Adjust `Socket::receive` interface to one which takes a pre-allocated buffer
- Add `#![no_std]` support (by disabling `std` feature)
- Add `MTU_MAX` constant (an alias of `ENET_PROTOCOL_MAXIMUM_MTU`)
- Add functions:
  - `Host::mtu`
  - `Host::set_mtu`
  - `Peer::mtu`
  - `Peer::set_mtu`
- Remove redundant `ENET_` prefix on `consts`

# 0.1.0
- Initial release
