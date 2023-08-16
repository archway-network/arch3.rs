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
