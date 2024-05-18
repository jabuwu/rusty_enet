# UNRELEASED
- Compatible with ENet 1.3.18
- Refine some trait requirements and derives
- Reduce allocations introduced by Rust port ([#1](https://github.com/jabuwu/rusty_enet/issues/1))
- Adjust `Socket::receive` interface to one which takes a pre-allocated buffer
- Add `MTU_MAX` constant (an alias of `ENET_PROTOCOL_MAXIMUM_MTU`)
- Add functions:
  - [`Host::mtu`]
  - [`Host::set_mtu`]
  - [`Peer::mtu`]
  - [`Peer::set_mtu`]

# 0.1.0
- Initial release
