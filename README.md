# arch3.rs

[![Build Status][build-image]][build-link]
[![Codacy Badge][codacy-image]][codacy-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![MSRV][rustc-image]

Rust SDK for Archway.

## Packages

| Package                           | Description                                           |                    Version                    |                         Docs                          |
|-----------------------------------|:------------------------------------------------------|:---------------------------------------------:|:-----------------------------------------------------:|
| [`bindings`](./packages/bindings) | CosmWasm bindings to interact with Archway's modules. | [![crates.io][bindings-image]][bindings-link] | [![docs.rs][bindings-docs-image]][bindings-docs-link] |
| [`proto`](./packages/proto)       | Rust build of Archway's ProtoBuf definitions.         |    [![crates.io][proto-image]][proto-link]    |    [![docs.rs][proto-docs-image]][proto-docs-link]    |

## License

This project is licensed under the Apache-2.0 License - see the [NOTICE][notice-link] and [LICENSE][license-link] files for details.

[//]: # "badges"
[build-image]: https://github.com/archway-network/arch3.rs/actions/workflows/check.yml/badge.svg
[build-link]: https://github.com/archway-network/arch3.rs/actions/workflows/check.yml
[codacy-image]: https://app.codacy.com/project/badge/Grade/6fedce8e3af541718dba59bde1f38375
[codacy-link]: https://app.codacy.com/gh/archway-network/arch3.rs/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade
[license-image]: https://img.shields.io/github/license/archway-network/arch3.rs?label=License&logo=opensourceinitiative&logoColor=white
[license-link]: https://github.com/archway-network/arch3.rs/blob/main/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.70+-red.svg?logo=rust
[bindings-image]: https://img.shields.io/crates/v/archway-bindings.svg
[bindings-link]: https://crates.io/crates/archway-bindings
[bindings-docs-image]: https://docs.rs/archway-bindings/badge.svg
[bindings-docs-link]: https://docs.rs/archway-bindings/
[proto-image]: https://img.shields.io/crates/v/archway-proto.svg
[proto-link]: https://crates.io/crates/archway-proto
[proto-docs-image]: https://docs.rs/archway-proto/badge.svg
[proto-docs-link]: https://docs.rs/archway-proto/

[//]: # "links"
[notice-link]: https://github.com/archway-network/arch3.rs/blob/main/NOTICE
