# rusty_enet

[ENet](https://github.com/lsalzman/enet) transpiled to Rust, and made agnostic to the underlying socket. Supports `std::net::UdpSocket` out of the box. Works in WASM if you bring your own WebRTC interface or similar.

## Why?

From [ENet's website](http://sauerbraten.org/enet/):

> ENet's purpose is to provide a relatively thin, simple and robust network communication layer on top of UDP (User Datagram Protocol). The primary feature it provides is optional reliable, in-order delivery of packets.
>
> ENet omits certain higher level networking features such as authentication, lobbying, server discovery, encryption, or other similar tasks that are particularly application specific so that the library remains flexible, portable, and easily embeddable.

This Rust port allows using ENet with more than just UDP sockets. Most noteably, in WASM environments.

## Work in progress features

These features were transpiled, but aren't accessible or configurable in the provided Rust API, yet.

- Compression
- Checksums