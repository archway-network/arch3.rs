// @generated
pub mod archway {
    pub mod callback {
        // @@protoc_insertion_point(attribute:archway.callback.v1)
        pub mod v1 {
            include!("archway.callback.v1.rs");
            // @@protoc_insertion_point(archway.callback.v1)
        }
    }
    pub mod cwerrors {
        // @@protoc_insertion_point(attribute:archway.cwerrors.v1)
        pub mod v1 {
            include!("archway.cwerrors.v1.rs");
            // @@protoc_insertion_point(archway.cwerrors.v1)
        }
    }
    pub mod cwfees {
        // @@protoc_insertion_point(attribute:archway.cwfees.v1)
        pub mod v1 {
            include!("archway.cwfees.v1.rs");
            // @@protoc_insertion_point(archway.cwfees.v1)
        }
    }
    pub mod cwica {
        // @@protoc_insertion_point(attribute:archway.cwica.v1)
        pub mod v1 {
            #[cfg(not(feature = "abstract-any"))]
            include!("archway.cwica.v1.rs");
            #[cfg(feature = "abstract-any")]
            include!("archway.cwica.v1.abstract.rs");
            // @@protoc_insertion_point(archway.cwica.v1)
        }
    }
    pub mod genmsg {
        // @@protoc_insertion_point(attribute:archway.genmsg.v1)
        pub mod v1 {
            #[cfg(not(feature = "abstract-any"))]
            include!("archway.genmsg.v1.rs");
            #[cfg(feature = "abstract-any")]
            include!("archway.genmsg.v1.abstract.rs");
            // @@protoc_insertion_point(archway.genmsg.v1)
        }
    }
    pub mod rewards {
        // @@protoc_insertion_point(attribute:archway.rewards.v1)
        pub mod v1 {
            include!("archway.rewards.v1.rs");
            // @@protoc_insertion_point(archway.rewards.v1)
        }
    }
    pub mod tracking {
        // @@protoc_insertion_point(attribute:archway.tracking.v1)
        pub mod v1 {
            include!("archway.tracking.v1.rs");
            // @@protoc_insertion_point(archway.tracking.v1)
        }
    }
}
pub mod cosmwasm {
    pub mod wasm {
        // @@protoc_insertion_point(attribute:cosmwasm.wasm.v1)
        pub mod v1 {
            #[cfg(not(feature = "abstract-any"))]
            include!("cosmwasm.wasm.v1.rs");
            #[cfg(feature = "abstract-any")]
            include!("cosmwasm.wasm.v1.abstract.rs");
            // @@protoc_insertion_point(cosmwasm.wasm.v1)
        }
    }
}
