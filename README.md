# Akri Discovery Handler Template
A template for accelerating creating a Discovery Handler for Akri in Rust. 

## About
A Discovery Handler is anything that implements Akri's the `Discover` service and `Registration` client defined in Akri's discovery gRPC interface. This can be done in any language using Akri's proto file. This template creates a `DiscoveryHandler` that implements the `Discover` service and registers it with the Akri Agent.

## Usage
## Use `cargo generate` to clone this template
This template is pulled via the [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) developer tool.
1. Install `cargo-generate`
    `cargo install cargo-generate`
2. Pull down this template
    `cargo generate --git https://github.com/kate-goldenring/akri-discovery-handler-template.git`
3. Fill in the [`discover`](src/discovery_handler.rs) logic of your Discovery Handler