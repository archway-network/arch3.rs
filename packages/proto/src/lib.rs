#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/86496504?s=100")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(rustdoc::bare_urls, clippy::derive_partial_eq_without_eq)]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

mod type_urls;

pub use prost;
pub use prost_types::Any;

pub use tendermint_proto as tendermint;

/// The version (commit hash) of the Archway protocol used when generating this library.
pub const ARCHWAY_VERSION: &str = include_str!("proto/ARCHWAY_COMMIT");
pub const COSMOS_SDK_VERSION: &str = include_str!("proto/COSMOS_SDK_COMMIT");
pub const IBC_VERSION: &str = include_str!("proto/IBC_COMMIT");
pub const WASMD_VERSION: &str = include_str!("proto/WASMD_COMMIT");

pub mod archway {
    pub mod genmsg {
        pub mod v1 {
            include!("proto/archway.genmsg.v1.rs");
        }
    }

    pub mod rewards {
        pub mod v1 {
            include!("proto/archway.rewards.v1.rs");
        }
    }

    pub mod tracking {
        pub mod v1 {
            include!("proto/archway.tracking.v1.rs");
        }
    }
}
