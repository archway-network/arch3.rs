// @generated
/// MsgRegisterAsGranter allows a contract to register itself as a fee granter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterAsGranter {
    #[prost(string, tag = "1")]
    pub granting_contract: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgRegisterAsGranter {
    const NAME: &'static str = "MsgRegisterAsGranter";
    const PACKAGE: &'static str = "archway.cwfees.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("archway.cwfees.v1.{}", Self::NAME)
    }
}
/// MsgRegisterAsGranterResponse defines the response of RegisterAsGranter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterAsGranterResponse {}
impl ::prost::Name for MsgRegisterAsGranterResponse {
    const NAME: &'static str = "MsgRegisterAsGranterResponse";
    const PACKAGE: &'static str = "archway.cwfees.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("archway.cwfees.v1.{}", Self::NAME)
    }
}
/// MsgUnregisterAsGranter can be used by a cosmwasm contract to unregister itself as a fee granter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnregisterAsGranter {
    #[prost(string, tag = "1")]
    pub granting_contract: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgUnregisterAsGranter {
    const NAME: &'static str = "MsgUnregisterAsGranter";
    const PACKAGE: &'static str = "archway.cwfees.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("archway.cwfees.v1.{}", Self::NAME)
    }
}
/// MsgUnregisterAsGranterResponse defines the response of UnregisterAsGranter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnregisterAsGranterResponse {}
impl ::prost::Name for MsgUnregisterAsGranterResponse {
    const NAME: &'static str = "MsgUnregisterAsGranterResponse";
    const PACKAGE: &'static str = "archway.cwfees.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("archway.cwfees.v1.{}", Self::NAME)
    }
}
/// IsGrantingContract is the request type of IsGrantingContract RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsGrantingContractRequest {
    /// contract_address defines the address of the contract
    /// which we want to check if it is a granter or not.
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
}
impl ::prost::Name for IsGrantingContractRequest {
    const NAME: &'static str = "IsGrantingContractRequest";
    const PACKAGE: &'static str = "archway.cwfees.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("archway.cwfees.v1.{}", Self::NAME)
    }
}
/// IsGrantingContractResponse is the response type of IsGRantingContract RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsGrantingContractResponse {
    /// is_granting_contract report if the contract is a granter or not.
    #[prost(bool, tag = "1")]
    pub is_granting_contract: bool,
}
impl ::prost::Name for IsGrantingContractResponse {
    const NAME: &'static str = "IsGrantingContractResponse";
    const PACKAGE: &'static str = "archway.cwfees.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("archway.cwfees.v1.{}", Self::NAME)
    }
}
/// GenesisState represents the genesis state of the cwfeesant module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(string, repeated, tag = "1")]
    pub granting_contracts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "archway.cwfees.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("archway.cwfees.v1.{}", Self::NAME)
    }
}
/// Encoded file descriptor set for the `archway.cwfees.v1` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xd5, 0x15, 0x0a, 0x1e, 0x61, 0x72, 0x63, 0x68, 0x77, 0x61, 0x79, 0x2f, 0x63, 0x77, 0x66,
    0x65, 0x65, 0x73, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x77, 0x66, 0x65, 0x65, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x11, 0x61, 0x72, 0x63, 0x68, 0x77, 0x61, 0x79, 0x2e, 0x63, 0x77, 0x66,
    0x65, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x1a, 0x19, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x61, 0x6e, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x17, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x6d, 0x73, 0x67, 0x2f, 0x76, 0x31,
    0x2f, 0x6d, 0x73, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x5b, 0x0a, 0x14, 0x4d, 0x73,
    0x67, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x41, 0x73, 0x47, 0x72, 0x61, 0x6e, 0x74,
    0x65, 0x72, 0x12, 0x2b, 0x0a, 0x11, 0x67, 0x72, 0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x63,
    0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x10, 0x67,
    0x72, 0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x3a,
    0x16, 0x82, 0xe7, 0xb0, 0x2a, 0x11, 0x67, 0x72, 0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x63,
    0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x22, 0x1e, 0x0a, 0x1c, 0x4d, 0x73, 0x67, 0x52, 0x65,
    0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x41, 0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x72, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x5d, 0x0a, 0x16, 0x4d, 0x73, 0x67, 0x55, 0x6e,
    0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x41, 0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x65,
    0x72, 0x12, 0x2b, 0x0a, 0x11, 0x67, 0x72, 0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x10, 0x67, 0x72,
    0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x3a, 0x16,
    0x82, 0xe7, 0xb0, 0x2a, 0x11, 0x67, 0x72, 0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x22, 0x20, 0x0a, 0x1e, 0x4d, 0x73, 0x67, 0x55, 0x6e, 0x72,
    0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x41, 0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x72,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x46, 0x0a, 0x19, 0x49, 0x73, 0x47, 0x72,
    0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x29, 0x0a, 0x10, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63,
    0x74, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x22, 0x4e, 0x0a, 0x1a, 0x49, 0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6f,
    0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x30,
    0x0a, 0x14, 0x69, 0x73, 0x5f, 0x67, 0x72, 0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x12, 0x69, 0x73,
    0x47, 0x72, 0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74,
    0x22, 0x3d, 0x0a, 0x0c, 0x47, 0x65, 0x6e, 0x65, 0x73, 0x69, 0x73, 0x53, 0x74, 0x61, 0x74, 0x65,
    0x12, 0x2d, 0x0a, 0x12, 0x67, 0x72, 0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x63, 0x6f, 0x6e,
    0x74, 0x72, 0x61, 0x63, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x52, 0x11, 0x67, 0x72,
    0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x73, 0x32,
    0xe9, 0x01, 0x0a, 0x03, 0x4d, 0x73, 0x67, 0x12, 0x6d, 0x0a, 0x11, 0x52, 0x65, 0x67, 0x69, 0x73,
    0x74, 0x65, 0x72, 0x41, 0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x72, 0x12, 0x27, 0x2e, 0x61,
    0x72, 0x63, 0x68, 0x77, 0x61, 0x79, 0x2e, 0x63, 0x77, 0x66, 0x65, 0x65, 0x73, 0x2e, 0x76, 0x31,
    0x2e, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x41, 0x73, 0x47, 0x72,
    0x61, 0x6e, 0x74, 0x65, 0x72, 0x1a, 0x2f, 0x2e, 0x61, 0x72, 0x63, 0x68, 0x77, 0x61, 0x79, 0x2e,
    0x63, 0x77, 0x66, 0x65, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x67,
    0x69, 0x73, 0x74, 0x65, 0x72, 0x41, 0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x72, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x73, 0x0a, 0x13, 0x55, 0x6e, 0x72, 0x65, 0x67, 0x69,
    0x73, 0x74, 0x65, 0x72, 0x41, 0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x72, 0x12, 0x29, 0x2e,
    0x61, 0x72, 0x63, 0x68, 0x77, 0x61, 0x79, 0x2e, 0x63, 0x77, 0x66, 0x65, 0x65, 0x73, 0x2e, 0x76,
    0x31, 0x2e, 0x4d, 0x73, 0x67, 0x55, 0x6e, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x41,
    0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x72, 0x1a, 0x31, 0x2e, 0x61, 0x72, 0x63, 0x68, 0x77,
    0x61, 0x79, 0x2e, 0x63, 0x77, 0x66, 0x65, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x4d, 0x73, 0x67,
    0x55, 0x6e, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x41, 0x73, 0x47, 0x72, 0x61, 0x6e,
    0x74, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x32, 0x7a, 0x0a, 0x05, 0x51,
    0x75, 0x65, 0x72, 0x79, 0x12, 0x71, 0x0a, 0x12, 0x49, 0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x69,
    0x6e, 0x67, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x12, 0x2c, 0x2e, 0x61, 0x72, 0x63,
    0x68, 0x77, 0x61, 0x79, 0x2e, 0x63, 0x77, 0x66, 0x65, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x49,
    0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63,
    0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x2d, 0x2e, 0x61, 0x72, 0x63, 0x68, 0x77,
    0x61, 0x79, 0x2e, 0x63, 0x77, 0x66, 0x65, 0x65, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x49, 0x73, 0x47,
    0x72, 0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x33, 0x5a, 0x31, 0x67, 0x69, 0x74, 0x68, 0x75,
    0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x61, 0x72, 0x63, 0x68, 0x77, 0x61, 0x79, 0x2d, 0x6e, 0x65,
    0x74, 0x77, 0x6f, 0x72, 0x6b, 0x2f, 0x61, 0x72, 0x63, 0x68, 0x77, 0x61, 0x79, 0x2f, 0x78, 0x2f,
    0x63, 0x77, 0x66, 0x65, 0x65, 0x73, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x4a, 0xf1, 0x0d, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x38, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00,
    0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x00, 0x1a, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x00, 0x12, 0x03, 0x03, 0x00, 0x23, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00,
    0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x48, 0x0a, 0x09, 0x0a, 0x02, 0x08,
    0x0b, 0x12, 0x03, 0x08, 0x00, 0x48, 0x0a, 0x0a, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x0a, 0x00,
    0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x0b, 0x0a, 0x60,
    0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x02, 0x55, 0x1a, 0x53, 0x20, 0x52, 0x65,
    0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x41, 0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x72, 0x20,
    0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x73, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x73, 0x6d, 0x77, 0x61, 0x73,
    0x6d, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65,
    0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x20, 0x69, 0x74, 0x73, 0x65, 0x6c, 0x66, 0x20, 0x61, 0x73,
    0x20, 0x61, 0x20, 0x66, 0x65, 0x65, 0x20, 0x67, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x72, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x06, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0c, 0x18, 0x2c, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x37, 0x53, 0x0a, 0x64, 0x0a, 0x04, 0x06, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x0e, 0x02, 0x5b, 0x1a, 0x57, 0x20, 0x55, 0x6e, 0x72, 0x65, 0x67, 0x69,
    0x73, 0x74, 0x65, 0x72, 0x41, 0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x72, 0x20, 0x61, 0x6c,
    0x6c, 0x6f, 0x77, 0x73, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x73, 0x6d, 0x77, 0x61, 0x73, 0x6d, 0x20,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x6e, 0x72, 0x65,
    0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x20, 0x69, 0x74, 0x73, 0x65, 0x6c, 0x66, 0x20, 0x61, 0x73,
    0x20, 0x61, 0x20, 0x66, 0x65, 0x65, 0x20, 0x67, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x72, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x06, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0e, 0x1a, 0x30, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0e, 0x3b, 0x59, 0x0a, 0x59, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x12, 0x00, 0x15, 0x01, 0x1a, 0x4d, 0x20, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x67, 0x69,
    0x73, 0x74, 0x65, 0x72, 0x41, 0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x72, 0x20, 0x61, 0x6c,
    0x6c, 0x6f, 0x77, 0x73, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x20,
    0x74, 0x6f, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x20, 0x69, 0x74, 0x73, 0x65,
    0x6c, 0x66, 0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x66, 0x65, 0x65, 0x20, 0x67, 0x72, 0x61, 0x6e,
    0x74, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x12, 0x08,
    0x1c, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x07, 0x12, 0x03, 0x13, 0x02, 0x36, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x00, 0x07, 0xf0, 0x8c, 0xa6, 0x05, 0x00, 0x12, 0x03, 0x13, 0x02, 0x36, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x14, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x14, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x14, 0x09, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x14, 0x1d, 0x1e, 0x0a, 0x54, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x03, 0x18, 0x00, 0x27,
    0x1a, 0x49, 0x20, 0x4d, 0x73, 0x67, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x41, 0x73,
    0x47, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20,
    0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72,
    0x41, 0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x18, 0x08, 0x24, 0x0a, 0x6e, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x1b,
    0x00, 0x1e, 0x01, 0x1a, 0x62, 0x20, 0x4d, 0x73, 0x67, 0x55, 0x6e, 0x72, 0x65, 0x67, 0x69, 0x73,
    0x74, 0x65, 0x72, 0x41, 0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x72, 0x20, 0x63, 0x61, 0x6e,
    0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x61, 0x20, 0x63, 0x6f,
    0x73, 0x6d, 0x77, 0x61, 0x73, 0x6d, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x20,
    0x74, 0x6f, 0x20, 0x75, 0x6e, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x20, 0x69, 0x74,
    0x73, 0x65, 0x6c, 0x66, 0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x66, 0x65, 0x65, 0x20, 0x67, 0x72,
    0x61, 0x6e, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x1b, 0x08, 0x1e, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x07, 0x12, 0x03, 0x1c, 0x02, 0x36, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x02, 0x07, 0xf0, 0x8c, 0xa6, 0x05, 0x00, 0x12, 0x03, 0x1c, 0x02, 0x36,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x02, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x09, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x1d, 0x1d, 0x1e, 0x0a, 0x58, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x03, 0x21,
    0x00, 0x29, 0x1a, 0x4d, 0x20, 0x4d, 0x73, 0x67, 0x55, 0x6e, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74,
    0x65, 0x72, 0x41, 0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x55, 0x6e, 0x72, 0x65,
    0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x41, 0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x72, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x21, 0x08, 0x26, 0x0a, 0x0a, 0x0a,
    0x02, 0x06, 0x01, 0x12, 0x04, 0x23, 0x00, 0x26, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x01, 0x01,
    0x12, 0x03, 0x23, 0x08, 0x0d, 0x0a, 0x5c, 0x0a, 0x04, 0x06, 0x01, 0x02, 0x00, 0x12, 0x03, 0x25,
    0x02, 0x59, 0x1a, 0x4f, 0x20, 0x49, 0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x43,
    0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75,
    0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x69, 0x66, 0x20,
    0x61, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20,
    0x67, 0x72, 0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63,
    0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x25, 0x06,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x25, 0x19, 0x32, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x25, 0x3d, 0x57, 0x0a, 0x4f, 0x0a,
    0x02, 0x04, 0x04, 0x12, 0x04, 0x29, 0x00, 0x2d, 0x01, 0x1a, 0x43, 0x20, 0x49, 0x73, 0x47, 0x72,
    0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x20, 0x69,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x79,
    0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x49, 0x73, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67,
    0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x20, 0x52, 0x50, 0x43, 0x2e, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x29, 0x08, 0x21, 0x0a, 0x76, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x00, 0x12, 0x03, 0x2c, 0x02, 0x1e, 0x1a, 0x69, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61,
    0x63, 0x74, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e,
    0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x0a, 0x20,
    0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x77, 0x65, 0x20, 0x77, 0x61, 0x6e, 0x74, 0x20, 0x74, 0x6f,
    0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x69, 0x66, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20,
    0x61, 0x20, 0x67, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x72, 0x20, 0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x74,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2c, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x09, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x1c, 0x1d, 0x0a, 0x58, 0x0a, 0x02,
    0x04, 0x05, 0x12, 0x04, 0x30, 0x00, 0x33, 0x01, 0x1a, 0x4c, 0x20, 0x49, 0x73, 0x47, 0x72, 0x61,
    0x6e, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x49, 0x73,
    0x47, 0x52, 0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74,
    0x20, 0x52, 0x50, 0x43, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x30,
    0x08, 0x22, 0x0a, 0x4f, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x32, 0x02, 0x20, 0x1a,
    0x42, 0x20, 0x69, 0x73, 0x5f, 0x67, 0x72, 0x61, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x69, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x61, 0x63, 0x74, 0x20, 0x69, 0x73,
    0x20, 0x61, 0x20, 0x67, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x72, 0x20, 0x6f, 0x72, 0x20, 0x6e, 0x6f,
    0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x32, 0x02,
    0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x32, 0x07, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x32, 0x1e, 0x1f, 0x0a, 0x50, 0x0a,
    0x02, 0x04, 0x06, 0x12, 0x04, 0x36, 0x00, 0x38, 0x01, 0x1a, 0x44, 0x20, 0x47, 0x65, 0x6e, 0x65,
    0x73, 0x69, 0x73, 0x53, 0x74, 0x61, 0x74, 0x65, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65,
    0x6e, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x73, 0x69, 0x73, 0x20,
    0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x77, 0x66,
    0x65, 0x65, 0x73, 0x61, 0x6e, 0x74, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x36, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x00, 0x12, 0x03, 0x37, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x37, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x37, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x37,
    0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x37, 0x27, 0x28,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("archway.cwfees.v1.serde.rs");
include!("archway.cwfees.v1.tonic.rs");
// @@protoc_insertion_point(module)
