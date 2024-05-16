# rusty_enet

[ENet](https://github.com/lsalzman/enet) transpiled to Rust, and made agnostic to the underlying socket. Supports `std::net::UdpSocket` out of the box. Works in WASM if you bring your own WebRTC interface or similar.

```
[dependencies]
rusty_enet = "0.1"
```

## ENet Versions

| rusty_enet  | ENet    | Commit
| ----------- | ------- | ------
| main        | 1.3.18  | ([enet/2662c0d](https://github.com/lsalzman/enet/commit/2662c0de09e36f2a2030ccc2c528a3e4c9e8138a))
| 0.1         | 1.3.17* | ([enet/2a85cd6](https://github.com/lsalzman/enet/commit/2a85cd64459f6ba038d233a634d9440490dbba12))

\* indicates non-exact version (see commit)

## Why?

From [ENet's website](http://sauerbraten.org/enet/):

> ENet's purpose is to provide a relatively thin, simple and robust network communication layer on top of UDP (User Datagram Protocol). The primary feature it provides is optional reliable, in-order delivery of packets.
>
> ENet omits certain higher level networking features such as authentication, lobbying, server discovery, encryption, or other similar tasks that are particularly application specific so that the library remains flexible, portable, and easily embeddable.

This Rust port allows using ENet with more than just UDP sockets. Most noteably, in WASM environments.

## Project Status

The entire API has been wrapped in safe Rust bindings, and I've tested things pretty thoroughly in my own projects. Despite the low semver version, this project couldn't be much further from "ready for serious use".

There may have been some bugs introduced during the C -> Rust transpilation and cleanup, but I've been diligent to keep changes to the original code minimal, and double check those that were necessary.
