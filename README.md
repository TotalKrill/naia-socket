[![Build Status](https://img.shields.io/circleci/project/github/naia-rs/naia-socket.svg)](https://circleci.com/gh/naia-rs/naia-socket)
[![Latest Version](https://img.shields.io/crates/v/naia-server-socket.svg)](https://crates.io/crates/naia-server-socket)
[![API Documentation](https://docs.rs/naia-server-socket/badge.svg)](https://docs.rs/naia-server-socket)
![](https://tokei.rs/b1/github/naia-rs/naia-socket)
[![Discord chat](https://img.shields.io/discord/764975354913619988.svg?label=discord%20chat)](https://discord.gg/fD6QCtX)
[![MIT/Apache][s3]][l3]

[s3]: https://img.shields.io/badge/license-MIT%2FApache-blue.svg
[l3]: docs/LICENSE-MIT

# naia-socket

A cross-platform (including Wasm!) Socket API that wraps unreliable & unordered communication, using WebRTC & UDP.

Utilizes Kyren's wonderful [webrtc-unreliable](https://github.com/kyren/webrtc-unreliable)

`naia-client-socket` is usable with both `wasm-bindgen` and [miniquad](https://github.com/not-fl3/miniquad) (build for these with the feature `wbindgen` & `mquad`, respectively)

## Demo

### Server:

To run a UDP server: (that will be able to communicate with Linux clients)

    1. `cd demo/server`
    2. `cargo run --features "use-udp"`

To run a WebRTC server: (that will be able to communicate with Web clients)

    1. `cd demo/server`
    2. `cargo run --features "use-webrtc"`

### Client:

To run a UDP client: (that will be able to communicate with a UDP server)

    1. `cd demo/client/wasm_bindgen`
    2. `cargo run`

To run a WebRTC client on Web using wasm-bindgen: (that will be able to communicate with a WebRTC server)

    1. `cd demo/client/wasm_bindgen`
    2. `cargo make serve` //this will open a web browser pointing at http://127.0.0.1:4000/

To run a WebRTC client on Web using miniquad: (that will be able to communicate with a WebRTC server)

    1. `cd demo/client/miniquad`
    2. `cargo make serve` //this will open a web browser pointing at http://127.0.0.1:4000/

### Notes:
Hosting a WebRTC server on `127.0.0.1` doesn't seem to work out of the box when using a Firefox browser, try using Chrome for development purposes