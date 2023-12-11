# Archway Proto

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![MSRV][rustc-image]

Rust build of Archway's ProtoBuf definitions.

This package can be used both in CosmWasm smart contracts and gRPC clients.
It is based on the [`cosmos-sdk-proto v0.18.0`][cosmos-rust]
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

## License

This project is licensed under the Apache-2.0 License - see the [NOTICE][notice-link] and [LICENSE][license-link] files for details.

[//]: # "badges"
[crate-image]: https://buildstats.info/crate/archway-proto
[crate-link]: https://crates.io/crates/archway-proto
[docs-image]: https://docs.rs/archway-proto/badge.svg
[docs-link]: https://docs.rs/archway-proto/
[license-image]: https://img.shields.io/github/license/archway-network/arch3.rs?label=License&logo=opensourceinitiative&logoColor=white&color=informational
[license-link]: https://github.com/archway-network/arch3.rs/blob/main/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.70+-blue.svg?logo=rust&logoColor=white&color=informational

[//]: # "links"
[cosmos-rust]: https://github.com/cosmos/cosmos-rust
[notice-link]: https://github.com/archway-network/arch3.rs/blob/main/NOTICE
