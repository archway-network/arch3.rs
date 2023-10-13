# Archway Proto

Rust build of Archway's ProtoBuf definitions.

This package can be used both in CosmWasm smart contracts and gRPC clients.
It is based on the [`cosmos-sdk-proto v0.18.0`](https://github.com/cosmos/cosmos-rust)
crate to provide the Cosmos SDK `v0.45.x` messages and follows the same feature
definitions, with the `tonic` gRPC clients enabled by default.

## Usage

### In CosmWasm smart contracts

```toml
archway-proto = { version = "0.1.0", default-features = false, features = ["cosmwasm"] }
```

### For the gRPC client functionality (won't work in CosmWasm)

```toml
archway-proto = "0.1.0"
```
