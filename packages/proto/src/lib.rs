#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/86496504?s=100")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    rustdoc::bare_urls,
    clippy::derive_partial_eq_without_eq,
    clippy::doc_lazy_continuation,
    clippy::too_long_first_doc_paragraph
)]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

#[cfg(feature = "abstract-any")]
pub mod any;

mod gen;

pub use gen::*;

pub use prost;
pub use prost_types::Any;

pub use tendermint_proto as tendermint;

/// The Archway protocol version (or commit hash) used when generating this library.
pub const ARCHWAY_VERSION: &str = include_str!("gen/ARCHWAY_COMMIT");

/// The Cosmos SDK version (or commit hash) used when generating this library.
pub const COSMOS_SDK_VERSION: &str = include_str!("gen/COSMOS_SDK_COMMIT");

/// The ibc-go version (or commit hash) used when generating this library.
pub const IBC_VERSION: &str = include_str!("gen/IBC_COMMIT");

/// The wasmd version (or commit hash) of the used when generating this library.
pub const WASMD_VERSION: &str = include_str!("gen/WASMD_COMMIT");
