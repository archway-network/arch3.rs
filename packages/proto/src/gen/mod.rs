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
pub mod cosmos {
    pub mod app {
        pub mod runtime {
            // @@protoc_insertion_point(attribute:cosmos.app.runtime.v1alpha1)
            pub mod v1alpha1 {
                include!("cosmos.app.runtime.v1alpha1.rs");
                // @@protoc_insertion_point(cosmos.app.runtime.v1alpha1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.app.v1alpha1)
        pub mod v1alpha1 {
            #[cfg(not(feature = "abstract-any"))]
            include!("cosmos.app.v1alpha1.rs");
            #[cfg(feature = "abstract-any")]
            include!("cosmos.app.v1alpha1.abstract.rs");
            // @@protoc_insertion_point(cosmos.app.v1alpha1)
        }
    }
    pub mod auth {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.auth.module.v1)
            pub mod v1 {
                include!("cosmos.auth.module.v1.rs");
                // @@protoc_insertion_point(cosmos.auth.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.auth.v1beta1)
        pub mod v1beta1 {
            #[cfg(not(feature = "abstract-any"))]
            include!("cosmos.auth.v1beta1.rs");
            #[cfg(feature = "abstract-any")]
            include!("cosmos.auth.v1beta1.abstract.rs");
            // @@protoc_insertion_point(cosmos.auth.v1beta1)
        }
    }
    pub mod authz {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.authz.module.v1)
            pub mod v1 {
                include!("cosmos.authz.module.v1.rs");
                // @@protoc_insertion_point(cosmos.authz.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.authz.v1beta1)
        pub mod v1beta1 {
            #[cfg(not(feature = "abstract-any"))]
            include!("cosmos.authz.v1beta1.rs");
            #[cfg(feature = "abstract-any")]
            include!("cosmos.authz.v1beta1.abstract.rs");
            // @@protoc_insertion_point(cosmos.authz.v1beta1)
        }
    }
    pub mod autocli {
        // @@protoc_insertion_point(attribute:cosmos.autocli.v1)
        pub mod v1 {
            include!("cosmos.autocli.v1.rs");
            // @@protoc_insertion_point(cosmos.autocli.v1)
        }
    }
    pub mod bank {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.bank.module.v1)
            pub mod v1 {
                include!("cosmos.bank.module.v1.rs");
                // @@protoc_insertion_point(cosmos.bank.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.bank.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.bank.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.bank.v1beta1)
        }
    }
    pub mod base {
        pub mod abci {
            // @@protoc_insertion_point(attribute:cosmos.base.abci.v1beta1)
            pub mod v1beta1 {
                #[cfg(not(feature = "abstract-any"))]
                include!("cosmos.base.abci.v1beta1.rs");
                #[cfg(feature = "abstract-any")]
                include!("cosmos.base.abci.v1beta1.abstract.rs");
                // @@protoc_insertion_point(cosmos.base.abci.v1beta1)
            }
        }
        pub mod kv {
            // @@protoc_insertion_point(attribute:cosmos.base.kv.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.base.kv.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.base.kv.v1beta1)
            }
        }
        pub mod node {
            // @@protoc_insertion_point(attribute:cosmos.base.node.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.base.node.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.base.node.v1beta1)
            }
        }
        pub mod query {
            // @@protoc_insertion_point(attribute:cosmos.base.query.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.base.query.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.base.query.v1beta1)
            }
        }
        pub mod reflection {
            // @@protoc_insertion_point(attribute:cosmos.base.reflection.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.base.reflection.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.base.reflection.v1beta1)
            }
            // @@protoc_insertion_point(attribute:cosmos.base.reflection.v2alpha1)
            pub mod v2alpha1 {
                include!("cosmos.base.reflection.v2alpha1.rs");
                // @@protoc_insertion_point(cosmos.base.reflection.v2alpha1)
            }
        }
        pub mod snapshots {
            // @@protoc_insertion_point(attribute:cosmos.base.snapshots.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.base.snapshots.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.base.snapshots.v1beta1)
            }
        }
        pub mod store {
            // @@protoc_insertion_point(attribute:cosmos.base.store.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.base.store.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.base.store.v1beta1)
            }
        }
        pub mod tendermint {
            // @@protoc_insertion_point(attribute:cosmos.base.tendermint.v1beta1)
            pub mod v1beta1 {
                #[cfg(not(feature = "abstract-any"))]
                include!("cosmos.base.tendermint.v1beta1.rs");
                #[cfg(feature = "abstract-any")]
                include!("cosmos.base.tendermint.v1beta1.abstract.rs");
                // @@protoc_insertion_point(cosmos.base.tendermint.v1beta1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.base.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.base.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.base.v1beta1)
        }
    }
    pub mod capability {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.capability.module.v1)
            pub mod v1 {
                include!("cosmos.capability.module.v1.rs");
                // @@protoc_insertion_point(cosmos.capability.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.capability.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.capability.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.capability.v1beta1)
        }
    }
    pub mod consensus {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.consensus.module.v1)
            pub mod v1 {
                include!("cosmos.consensus.module.v1.rs");
                // @@protoc_insertion_point(cosmos.consensus.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.consensus.v1)
        pub mod v1 {
            include!("cosmos.consensus.v1.rs");
            // @@protoc_insertion_point(cosmos.consensus.v1)
        }
    }
    pub mod crisis {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.crisis.module.v1)
            pub mod v1 {
                include!("cosmos.crisis.module.v1.rs");
                // @@protoc_insertion_point(cosmos.crisis.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.crisis.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.crisis.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.crisis.v1beta1)
        }
    }
    pub mod crypto {
        // @@protoc_insertion_point(attribute:cosmos.crypto.ed25519)
        pub mod ed25519 {
            include!("cosmos.crypto.ed25519.rs");
            // @@protoc_insertion_point(cosmos.crypto.ed25519)
        }
        pub mod hd {
            // @@protoc_insertion_point(attribute:cosmos.crypto.hd.v1)
            pub mod v1 {
                include!("cosmos.crypto.hd.v1.rs");
                // @@protoc_insertion_point(cosmos.crypto.hd.v1)
            }
        }
        pub mod keyring {
            // @@protoc_insertion_point(attribute:cosmos.crypto.keyring.v1)
            pub mod v1 {
                #[cfg(not(feature = "abstract-any"))]
                include!("cosmos.crypto.keyring.v1.rs");
                #[cfg(feature = "abstract-any")]
                include!("cosmos.crypto.keyring.v1.abstract.rs");
                // @@protoc_insertion_point(cosmos.crypto.keyring.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.crypto.multisig)
        pub mod multisig {
            #[cfg(not(feature = "abstract-any"))]
            include!("cosmos.crypto.multisig.rs");
            #[cfg(feature = "abstract-any")]
            include!("cosmos.crypto.multisig.abstract.rs");
            // @@protoc_insertion_point(cosmos.crypto.multisig)
            // @@protoc_insertion_point(attribute:cosmos.crypto.multisig.v1beta1)
            pub mod v1beta1 {
                include!("cosmos.crypto.multisig.v1beta1.rs");
                // @@protoc_insertion_point(cosmos.crypto.multisig.v1beta1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.crypto.secp256k1)
        pub mod secp256k1 {
            include!("cosmos.crypto.secp256k1.rs");
            // @@protoc_insertion_point(cosmos.crypto.secp256k1)
        }
        // @@protoc_insertion_point(attribute:cosmos.crypto.secp256r1)
        pub mod secp256r1 {
            include!("cosmos.crypto.secp256r1.rs");
            // @@protoc_insertion_point(cosmos.crypto.secp256r1)
        }
    }
    pub mod distribution {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.distribution.module.v1)
            pub mod v1 {
                include!("cosmos.distribution.module.v1.rs");
                // @@protoc_insertion_point(cosmos.distribution.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.distribution.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.distribution.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.distribution.v1beta1)
        }
    }
    pub mod evidence {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.evidence.module.v1)
            pub mod v1 {
                include!("cosmos.evidence.module.v1.rs");
                // @@protoc_insertion_point(cosmos.evidence.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.evidence.v1beta1)
        pub mod v1beta1 {
            #[cfg(not(feature = "abstract-any"))]
            include!("cosmos.evidence.v1beta1.rs");
            #[cfg(feature = "abstract-any")]
            include!("cosmos.evidence.v1beta1.abstract.rs");
            // @@protoc_insertion_point(cosmos.evidence.v1beta1)
        }
    }
    pub mod feegrant {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.feegrant.module.v1)
            pub mod v1 {
                include!("cosmos.feegrant.module.v1.rs");
                // @@protoc_insertion_point(cosmos.feegrant.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.feegrant.v1beta1)
        pub mod v1beta1 {
            #[cfg(not(feature = "abstract-any"))]
            include!("cosmos.feegrant.v1beta1.rs");
            #[cfg(feature = "abstract-any")]
            include!("cosmos.feegrant.v1beta1.abstract.rs");
            // @@protoc_insertion_point(cosmos.feegrant.v1beta1)
        }
    }
    pub mod genutil {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.genutil.module.v1)
            pub mod v1 {
                include!("cosmos.genutil.module.v1.rs");
                // @@protoc_insertion_point(cosmos.genutil.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.genutil.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.genutil.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.genutil.v1beta1)
        }
    }
    pub mod gov {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.gov.module.v1)
            pub mod v1 {
                include!("cosmos.gov.module.v1.rs");
                // @@protoc_insertion_point(cosmos.gov.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.gov.v1)
        pub mod v1 {
            #[cfg(not(feature = "abstract-any"))]
            include!("cosmos.gov.v1.rs");
            #[cfg(feature = "abstract-any")]
            include!("cosmos.gov.v1.abstract.rs");
            // @@protoc_insertion_point(cosmos.gov.v1)
        }
        // @@protoc_insertion_point(attribute:cosmos.gov.v1beta1)
        pub mod v1beta1 {
            #[cfg(not(feature = "abstract-any"))]
            include!("cosmos.gov.v1beta1.rs");
            #[cfg(feature = "abstract-any")]
            include!("cosmos.gov.v1beta1.abstract.rs");
            // @@protoc_insertion_point(cosmos.gov.v1beta1)
        }
    }
    pub mod group {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.group.module.v1)
            pub mod v1 {
                include!("cosmos.group.module.v1.rs");
                // @@protoc_insertion_point(cosmos.group.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.group.v1)
        pub mod v1 {
            #[cfg(not(feature = "abstract-any"))]
            include!("cosmos.group.v1.rs");
            #[cfg(feature = "abstract-any")]
            include!("cosmos.group.v1.abstract.rs");
            // @@protoc_insertion_point(cosmos.group.v1)
        }
    }
    pub mod ics23 {
        // @@protoc_insertion_point(attribute:cosmos.ics23.v1)
        pub mod v1 {
            include!("cosmos.ics23.v1.rs");
            // @@protoc_insertion_point(cosmos.ics23.v1)
        }
    }
    pub mod mint {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.mint.module.v1)
            pub mod v1 {
                include!("cosmos.mint.module.v1.rs");
                // @@protoc_insertion_point(cosmos.mint.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.mint.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.mint.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.mint.v1beta1)
        }
    }
    pub mod msg {
        // @@protoc_insertion_point(attribute:cosmos.msg.v1)
        pub mod v1 {
            include!("cosmos.msg.v1.rs");
            // @@protoc_insertion_point(cosmos.msg.v1)
        }
    }
    pub mod nft {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.nft.module.v1)
            pub mod v1 {
                include!("cosmos.nft.module.v1.rs");
                // @@protoc_insertion_point(cosmos.nft.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.nft.v1beta1)
        pub mod v1beta1 {
            #[cfg(not(feature = "abstract-any"))]
            include!("cosmos.nft.v1beta1.rs");
            #[cfg(feature = "abstract-any")]
            include!("cosmos.nft.v1beta1.abstract.rs");
            // @@protoc_insertion_point(cosmos.nft.v1beta1)
        }
    }
    pub mod orm {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.orm.module.v1alpha1)
            pub mod v1alpha1 {
                include!("cosmos.orm.module.v1alpha1.rs");
                // @@protoc_insertion_point(cosmos.orm.module.v1alpha1)
            }
        }
        pub mod query {
            // @@protoc_insertion_point(attribute:cosmos.orm.query.v1alpha1)
            pub mod v1alpha1 {
                #[cfg(not(feature = "abstract-any"))]
                include!("cosmos.orm.query.v1alpha1.rs");
                #[cfg(feature = "abstract-any")]
                include!("cosmos.orm.query.v1alpha1.abstract.rs");
                // @@protoc_insertion_point(cosmos.orm.query.v1alpha1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.orm.v1)
        pub mod v1 {
            include!("cosmos.orm.v1.rs");
            // @@protoc_insertion_point(cosmos.orm.v1)
        }
        // @@protoc_insertion_point(attribute:cosmos.orm.v1alpha1)
        pub mod v1alpha1 {
            include!("cosmos.orm.v1alpha1.rs");
            // @@protoc_insertion_point(cosmos.orm.v1alpha1)
        }
    }
    pub mod params {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.params.module.v1)
            pub mod v1 {
                include!("cosmos.params.module.v1.rs");
                // @@protoc_insertion_point(cosmos.params.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.params.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.params.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.params.v1beta1)
        }
    }
    pub mod query {
        // @@protoc_insertion_point(attribute:cosmos.query.v1)
        pub mod v1 {
            include!("cosmos.query.v1.rs");
            // @@protoc_insertion_point(cosmos.query.v1)
        }
    }
    pub mod reflection {
        // @@protoc_insertion_point(attribute:cosmos.reflection.v1)
        pub mod v1 {
            include!("cosmos.reflection.v1.rs");
            // @@protoc_insertion_point(cosmos.reflection.v1)
        }
    }
    pub mod slashing {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.slashing.module.v1)
            pub mod v1 {
                include!("cosmos.slashing.module.v1.rs");
                // @@protoc_insertion_point(cosmos.slashing.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.slashing.v1beta1)
        pub mod v1beta1 {
            include!("cosmos.slashing.v1beta1.rs");
            // @@protoc_insertion_point(cosmos.slashing.v1beta1)
        }
    }
    pub mod staking {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.staking.module.v1)
            pub mod v1 {
                include!("cosmos.staking.module.v1.rs");
                // @@protoc_insertion_point(cosmos.staking.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.staking.v1beta1)
        pub mod v1beta1 {
            #[cfg(not(feature = "abstract-any"))]
            include!("cosmos.staking.v1beta1.rs");
            #[cfg(feature = "abstract-any")]
            include!("cosmos.staking.v1beta1.abstract.rs");
            // @@protoc_insertion_point(cosmos.staking.v1beta1)
        }
    }
    pub mod tx {
        pub mod config {
            // @@protoc_insertion_point(attribute:cosmos.tx.config.v1)
            pub mod v1 {
                include!("cosmos.tx.config.v1.rs");
                // @@protoc_insertion_point(cosmos.tx.config.v1)
            }
        }
        pub mod signing {
            // @@protoc_insertion_point(attribute:cosmos.tx.signing.v1beta1)
            pub mod v1beta1 {
                #[cfg(not(feature = "abstract-any"))]
                include!("cosmos.tx.signing.v1beta1.rs");
                #[cfg(feature = "abstract-any")]
                include!("cosmos.tx.signing.v1beta1.abstract.rs");
                // @@protoc_insertion_point(cosmos.tx.signing.v1beta1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.tx.v1beta1)
        pub mod v1beta1 {
            #[cfg(not(feature = "abstract-any"))]
            include!("cosmos.tx.v1beta1.rs");
            #[cfg(feature = "abstract-any")]
            include!("cosmos.tx.v1beta1.abstract.rs");
            // @@protoc_insertion_point(cosmos.tx.v1beta1)
        }
    }
    pub mod upgrade {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.upgrade.module.v1)
            pub mod v1 {
                include!("cosmos.upgrade.module.v1.rs");
                // @@protoc_insertion_point(cosmos.upgrade.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.upgrade.v1beta1)
        pub mod v1beta1 {
            #[cfg(not(feature = "abstract-any"))]
            include!("cosmos.upgrade.v1beta1.rs");
            #[cfg(feature = "abstract-any")]
            include!("cosmos.upgrade.v1beta1.abstract.rs");
            // @@protoc_insertion_point(cosmos.upgrade.v1beta1)
        }
    }
    pub mod vesting {
        pub mod module {
            // @@protoc_insertion_point(attribute:cosmos.vesting.module.v1)
            pub mod v1 {
                include!("cosmos.vesting.module.v1.rs");
                // @@protoc_insertion_point(cosmos.vesting.module.v1)
            }
        }
        // @@protoc_insertion_point(attribute:cosmos.vesting.v1beta1)
        pub mod v1beta1 {
            #[cfg(not(feature = "abstract-any"))]
            include!("cosmos.vesting.v1beta1.rs");
            #[cfg(feature = "abstract-any")]
            include!("cosmos.vesting.v1beta1.abstract.rs");
            // @@protoc_insertion_point(cosmos.vesting.v1beta1)
        }
    }
}
// @@protoc_insertion_point(attribute:cosmos_proto)
pub mod cosmos_proto {
    include!("cosmos_proto.rs");
    // @@protoc_insertion_point(cosmos_proto)
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
pub mod ibc {
    pub mod applications {
        pub mod fee {
            // @@protoc_insertion_point(attribute:ibc.applications.fee.v1)
            pub mod v1 {
                include!("ibc.applications.fee.v1.rs");
                // @@protoc_insertion_point(ibc.applications.fee.v1)
            }
        }
        pub mod interchain_accounts {
            pub mod controller {
                // @@protoc_insertion_point(attribute:ibc.applications.interchain_accounts.controller.v1)
                pub mod v1 {
                    include!("ibc.applications.interchain_accounts.controller.v1.rs");
                    // @@protoc_insertion_point(ibc.applications.interchain_accounts.controller.v1)
                }
            }
            pub mod genesis {
                // @@protoc_insertion_point(attribute:ibc.applications.interchain_accounts.genesis.v1)
                pub mod v1 {
                    include!("ibc.applications.interchain_accounts.genesis.v1.rs");
                    // @@protoc_insertion_point(ibc.applications.interchain_accounts.genesis.v1)
                }
            }
            pub mod host {
                // @@protoc_insertion_point(attribute:ibc.applications.interchain_accounts.host.v1)
                pub mod v1 {
                    include!("ibc.applications.interchain_accounts.host.v1.rs");
                    // @@protoc_insertion_point(ibc.applications.interchain_accounts.host.v1)
                }
            }
            // @@protoc_insertion_point(attribute:ibc.applications.interchain_accounts.v1)
            pub mod v1 {
                #[cfg(not(feature = "abstract-any"))]
                include!("ibc.applications.interchain_accounts.v1.rs");
                #[cfg(feature = "abstract-any")]
                include!("ibc.applications.interchain_accounts.v1.abstract.rs");
                // @@protoc_insertion_point(ibc.applications.interchain_accounts.v1)
            }
        }
        pub mod transfer {
            // @@protoc_insertion_point(attribute:ibc.applications.transfer.v1)
            pub mod v1 {
                include!("ibc.applications.transfer.v1.rs");
                // @@protoc_insertion_point(ibc.applications.transfer.v1)
            }
            // @@protoc_insertion_point(attribute:ibc.applications.transfer.v2)
            pub mod v2 {
                include!("ibc.applications.transfer.v2.rs");
                // @@protoc_insertion_point(ibc.applications.transfer.v2)
            }
        }
    }
    pub mod core {
        pub mod channel {
            // @@protoc_insertion_point(attribute:ibc.core.channel.v1)
            pub mod v1 {
                #[cfg(not(feature = "abstract-any"))]
                include!("ibc.core.channel.v1.rs");
                #[cfg(feature = "abstract-any")]
                include!("ibc.core.channel.v1.abstract.rs");
                // @@protoc_insertion_point(ibc.core.channel.v1)
            }
        }
        pub mod client {
            // @@protoc_insertion_point(attribute:ibc.core.client.v1)
            pub mod v1 {
                #[cfg(not(feature = "abstract-any"))]
                include!("ibc.core.client.v1.rs");
                #[cfg(feature = "abstract-any")]
                include!("ibc.core.client.v1.abstract.rs");
                // @@protoc_insertion_point(ibc.core.client.v1)
            }
        }
        pub mod commitment {
            // @@protoc_insertion_point(attribute:ibc.core.commitment.v1)
            pub mod v1 {
                include!("ibc.core.commitment.v1.rs");
                // @@protoc_insertion_point(ibc.core.commitment.v1)
            }
        }
        pub mod connection {
            // @@protoc_insertion_point(attribute:ibc.core.connection.v1)
            pub mod v1 {
                #[cfg(not(feature = "abstract-any"))]
                include!("ibc.core.connection.v1.rs");
                #[cfg(feature = "abstract-any")]
                include!("ibc.core.connection.v1.abstract.rs");
                // @@protoc_insertion_point(ibc.core.connection.v1)
            }
        }
        pub mod types {
            // @@protoc_insertion_point(attribute:ibc.core.types.v1)
            pub mod v1 {
                #[cfg(not(feature = "abstract-any"))]
                include!("ibc.core.types.v1.rs");
                #[cfg(feature = "abstract-any")]
                include!("ibc.core.types.v1.abstract.rs");
                // @@protoc_insertion_point(ibc.core.types.v1)
            }
        }
    }
    pub mod lightclients {
        pub mod localhost {
            // @@protoc_insertion_point(attribute:ibc.lightclients.localhost.v2)
            pub mod v2 {
                include!("ibc.lightclients.localhost.v2.rs");
                // @@protoc_insertion_point(ibc.lightclients.localhost.v2)
            }
        }
        pub mod solomachine {
            // @@protoc_insertion_point(attribute:ibc.lightclients.solomachine.v2)
            pub mod v2 {
                #[cfg(not(feature = "abstract-any"))]
                include!("ibc.lightclients.solomachine.v2.rs");
                #[cfg(feature = "abstract-any")]
                include!("ibc.lightclients.solomachine.v2.abstract.rs");
                // @@protoc_insertion_point(ibc.lightclients.solomachine.v2)
            }
            // @@protoc_insertion_point(attribute:ibc.lightclients.solomachine.v3)
            pub mod v3 {
                #[cfg(not(feature = "abstract-any"))]
                include!("ibc.lightclients.solomachine.v3.rs");
                #[cfg(feature = "abstract-any")]
                include!("ibc.lightclients.solomachine.v3.abstract.rs");
                // @@protoc_insertion_point(ibc.lightclients.solomachine.v3)
            }
        }
        pub mod tendermint {
            // @@protoc_insertion_point(attribute:ibc.lightclients.tendermint.v1)
            pub mod v1 {
                include!("ibc.lightclients.tendermint.v1.rs");
                // @@protoc_insertion_point(ibc.lightclients.tendermint.v1)
            }
        }
    }
}
