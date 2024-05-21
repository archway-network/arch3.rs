// @generated
/// MsgRegisterAsGranter allows a contract to register itself as a fee granter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterAsGranter {
    #[prost(string, tag="1")]
    pub granting_contract: ::prost::alloc::string::String,
}
/// MsgRegisterAsGranterResponse defines the response of RegisterAsGranter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterAsGranterResponse {
}
/// MsgUnregisterAsGranter can be used by a cosmwasm contract to unregister itself as a fee granter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnregisterAsGranter {
    #[prost(string, tag="1")]
    pub granting_contract: ::prost::alloc::string::String,
}
/// MsgUnregisterAsGranterResponse defines the response of UnregisterAsGranter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnregisterAsGranterResponse {
}
/// IsGrantingContract is the request type of IsGrantingContract RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsGrantingContractRequest {
    /// contract_address defines the address of the contract
    /// which we want to check if it is a granter or not.
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
}
/// IsGrantingContractResponse is the response type of IsGRantingContract RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsGrantingContractResponse {
    /// is_granting_contract report if the contract is a granter or not.
    #[prost(bool, tag="1")]
    pub is_granting_contract: bool,
}
/// GenesisState represents the genesis state of the cwfeesant module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(string, repeated, tag="1")]
    pub granting_contracts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
include!("archway.cwfees.v1.tonic.rs");
// @@protoc_insertion_point(module)