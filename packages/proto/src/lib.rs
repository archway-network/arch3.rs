#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/86496504?s=100")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(rustdoc::bare_urls, clippy::derive_partial_eq_without_eq)]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

#[cfg(feature = "abstract-any")]
pub mod any;

#[allow(clippy::all)]
mod gen;

pub use gen::*;

pub use prost;
pub use prost_types::Any;

pub use cosmos_sdk_proto::cosmos;
pub use cosmos_sdk_proto::tendermint;
pub use ibc_proto::ibc;

/// The Archway protocol version (or commit hash) used when generating this library.
pub const ARCHWAY_VERSION: &str = include_str!("gen/ARCHWAY_COMMIT");

/// The wasmd version (or commit hash) of the used when generating this library.
pub const WASMD_VERSION: &str = include_str!("gen/WASMD_COMMIT");

pub use cosmos_sdk_proto::COSMOS_SDK_VERSION;
pub use ibc_proto::IBC_GO_COMMIT;
