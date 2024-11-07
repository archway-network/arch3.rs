#![crate_name = "archway_bindings"]
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

mod msg;
mod pagination;
mod query;

pub mod types;

#[cfg(not(target_arch = "wasm32"))]
pub mod testing;

pub use msg::ArchwayMsg;
pub use pagination::{PageRequest, PageResponse};
pub use query::ArchwayQuery;

pub type Coins = Vec<cosmwasm_std::Coin>;

/// A result type for [responses](cosmwasm_std::Response<ArchwayMsg>) that contain an ArchwayMsg.
///
/// This is a convenience type to avoid writing out the full type signature.
pub type ArchwayResult<E> = Result<cosmwasm_std::Response<ArchwayMsg>, E>;
