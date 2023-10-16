# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## archway-proto v0.1.0 (2023-10-15)

### Added

- **proto:** generated ProtoBuf structs from the protocol (#18)
- **proto:** tonic gRPC client and server (#18)

## archway-bindings v0.2.1 (2023-10-16)

### Fixed

- **bindings:** fixed docs.rs build (#27)

## archway-bindings v0.2.0 (2023-10-15)

### Added

- **bindings:** message and query for contract premiums (flat fees) (#9)
- **bindings:** helper to withdraw max rewards (#22)
- **contracts:** rewards staking example (#17)
- **contracts:** sample usage for flat fee (#9)

### Changed

- **bindings:** allow a contract to update another contract's metadata (#11)
- **bindings:** allow importing cosmwasm-std staking feature (#16)

### Fixed

- **bindings:** `contract_address` is mandatory on `SetFlatFee` (#21)

## archway-bindings v0.1.0 (2023-01-24)

### Added

- **bindings:** `ArchwayMsg` and `ArchwayQuery` (#4)
- **bindings:** custom gov vote query (#6)
- **contracts:** sample increment contract (#5)
