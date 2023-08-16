#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/86496504?s=100")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(rustdoc::bare_urls, clippy::derive_partial_eq_without_eq)]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

mod type_urls;

pub use prost;
pub use prost_types::Any;

pub use cosmos_sdk_proto as cosmos;
pub use cosmos_sdk_proto::tendermint;

/// The version (commit hash) of the Archway protocol used when generating this library.
pub const ARCHWAY_VERSION: &str = include_str!("proto/ARCHWAY_COMMIT");

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
