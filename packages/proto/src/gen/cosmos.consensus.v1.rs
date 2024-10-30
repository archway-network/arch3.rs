// @generated
/// QueryParamsRequest defines the request type for querying x/consensus parameters.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
impl ::prost::Name for QueryParamsRequest {
    const NAME: &'static str = "QueryParamsRequest";
    const PACKAGE: &'static str = "cosmos.consensus.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.consensus.v1.{}", Self::NAME)
    }
}
/// QueryParamsResponse defines the response type for querying x/consensus parameters.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params are the tendermint consensus params stored in the consensus module.
    /// Please note that `params.version` is not populated in this response, it is
    /// tracked separately in the x/upgrade module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<::tendermint_proto::v0_37::types::ConsensusParams>,
}
impl ::prost::Name for QueryParamsResponse {
    const NAME: &'static str = "QueryParamsResponse";
    const PACKAGE: &'static str = "cosmos.consensus.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.consensus.v1.{}", Self::NAME)
    }
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/consensus parameters to update.
    /// VersionsParams is not included in this Msg because it is tracked
    /// separarately in x/upgrade.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<::tendermint_proto::v0_37::types::BlockParams>,
    #[prost(message, optional, tag = "3")]
    pub evidence: ::core::option::Option<::tendermint_proto::v0_37::types::EvidenceParams>,
    #[prost(message, optional, tag = "4")]
    pub validator: ::core::option::Option<::tendermint_proto::v0_37::types::ValidatorParams>,
}
impl ::prost::Name for MsgUpdateParams {
    const NAME: &'static str = "MsgUpdateParams";
    const PACKAGE: &'static str = "cosmos.consensus.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.consensus.v1.{}", Self::NAME)
    }
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
#[derive(::serde::Serialize, ::serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
impl ::prost::Name for MsgUpdateParamsResponse {
    const NAME: &'static str = "MsgUpdateParamsResponse";
    const PACKAGE: &'static str = "cosmos.consensus.v1";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.consensus.v1.{}", Self::NAME)
    }
}
/// Encoded file descriptor set for the `cosmos.consensus.v1` package
#[cfg(feature = "proto-descriptor")]
#[cfg_attr(docsrs, doc(cfg(feature = "proto-descriptor")))]
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xb9, 0x09, 0x0a, 0x1f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x6f, 0x6e, 0x73,
    0x65, 0x6e, 0x73, 0x75, 0x73, 0x2f, 0x76, 0x31, 0x2f, 0x71, 0x75, 0x65, 0x72, 0x79, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x63, 0x6f, 0x6e,
    0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x2e, 0x76, 0x31, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67, 0x6c,
    0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x6d,
    0x69, 0x6e, 0x74, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x14, 0x0a, 0x12, 0x51, 0x75, 0x65, 0x72, 0x79, 0x50,
    0x61, 0x72, 0x61, 0x6d, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x50, 0x0a, 0x13,
    0x51, 0x75, 0x65, 0x72, 0x79, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x39, 0x0a, 0x06, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x74,
    0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x43, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73,
    0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x52, 0x06, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x32, 0x8a,
    0x01, 0x0a, 0x05, 0x51, 0x75, 0x65, 0x72, 0x79, 0x12, 0x80, 0x01, 0x0a, 0x06, 0x50, 0x61, 0x72,
    0x61, 0x6d, 0x73, 0x12, 0x27, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x63, 0x6f, 0x6e,
    0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x51, 0x75, 0x65, 0x72, 0x79, 0x50,
    0x61, 0x72, 0x61, 0x6d, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x28, 0x2e, 0x63,
    0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x2e,
    0x76, 0x31, 0x2e, 0x51, 0x75, 0x65, 0x72, 0x79, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x23, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x1d, 0x12, 0x1b,
    0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75,
    0x73, 0x2f, 0x76, 0x31, 0x2f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x42, 0x30, 0x5a, 0x2e, 0x67,
    0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73,
    0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x78, 0x2f, 0x63, 0x6f,
    0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x4a, 0x94, 0x06,
    0x0a, 0x06, 0x12, 0x04, 0x01, 0x00, 0x1a, 0x01, 0x0a, 0x22, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x01,
    0x00, 0x12, 0x1a, 0x18, 0x20, 0x53, 0x69, 0x6e, 0x63, 0x65, 0x3a, 0x20, 0x63, 0x6f, 0x73, 0x6d,
    0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x20, 0x30, 0x2e, 0x34, 0x37, 0x0a, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x02, 0x00, 0x1c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00,
    0x26, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x27, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x07, 0x00, 0x45, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x07, 0x00,
    0x45, 0x0a, 0x35, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x0a, 0x00, 0x0f, 0x01, 0x1a, 0x29, 0x20,
    0x51, 0x75, 0x65, 0x72, 0x79, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x67, 0x52, 0x50, 0x43, 0x20, 0x71, 0x75, 0x65, 0x72, 0x69, 0x65, 0x72, 0x20, 0x73,
    0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12,
    0x03, 0x0a, 0x08, 0x0d, 0x0a, 0x4a, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x04, 0x0c, 0x02,
    0x0e, 0x03, 0x1a, 0x3c, 0x20, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x20, 0x71, 0x75, 0x65, 0x72,
    0x69, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65,
    0x72, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x78, 0x2f, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75,
    0x73, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x06, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0c, 0x0d, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x2a, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x0d, 0x04, 0x41, 0x0a, 0x11, 0x0a, 0x0a, 0x06, 0x00, 0x02, 0x00,
    0x04, 0xb0, 0xca, 0xbc, 0x22, 0x02, 0x12, 0x03, 0x0d, 0x04, 0x41, 0x0a, 0x5d, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x03, 0x12, 0x00, 0x1d, 0x1a, 0x52, 0x20, 0x51, 0x75, 0x65, 0x72, 0x79, 0x50, 0x61,
    0x72, 0x61, 0x6d, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x64, 0x65, 0x66, 0x69,
    0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20,
    0x74, 0x79, 0x70, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x71, 0x75, 0x65, 0x72, 0x79, 0x69, 0x6e,
    0x67, 0x20, 0x78, 0x2f, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x20, 0x70, 0x61,
    0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x12, 0x08, 0x1a, 0x0a, 0x60, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x15, 0x00,
    0x1a, 0x01, 0x1a, 0x54, 0x20, 0x51, 0x75, 0x65, 0x72, 0x79, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x79,
    0x70, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x71, 0x75, 0x65, 0x72, 0x79, 0x69, 0x6e, 0x67, 0x20,
    0x78, 0x2f, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x20, 0x70, 0x61, 0x72, 0x61,
    0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12,
    0x03, 0x15, 0x08, 0x1b, 0x0a, 0xd3, 0x01, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x19,
    0x02, 0x2e, 0x1a, 0xc5, 0x01, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x20, 0x61, 0x72, 0x65,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x74, 0x20,
    0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73,
    0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63,
    0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e,
    0x0a, 0x20, 0x50, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x20, 0x6e, 0x6f, 0x74, 0x65, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x20, 0x60, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x2e, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x60, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x70, 0x6f, 0x70, 0x75, 0x6c,
    0x61, 0x74, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2c, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x0a, 0x20, 0x74, 0x72,
    0x61, 0x63, 0x6b, 0x65, 0x64, 0x20, 0x73, 0x65, 0x70, 0x61, 0x72, 0x61, 0x74, 0x65, 0x6c, 0x79,
    0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x78, 0x2f, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64,
    0x65, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x19, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x19, 0x23, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x19, 0x2c, 0x2d, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33, 0x0a, 0xe4, 0x0d, 0x0a,
    0x1c, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75,
    0x73, 0x2f, 0x76, 0x31, 0x2f, 0x74, 0x78, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x63,
    0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x2e,
    0x76, 0x31, 0x1a, 0x19, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x63,
    0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2f, 0x6d, 0x73, 0x67, 0x2f, 0x76, 0x31, 0x2f, 0x6d, 0x73, 0x67,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x6d, 0x69,
    0x6e, 0x74, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x8d, 0x02, 0x0a, 0x0f, 0x4d, 0x73, 0x67, 0x55, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x12, 0x36, 0x0a, 0x09, 0x61, 0x75, 0x74,
    0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x18, 0xd2, 0xb4,
    0x2d, 0x14, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x52, 0x09, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74,
    0x79, 0x12, 0x33, 0x0a, 0x05, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1d, 0x2e, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x74, 0x2e, 0x74, 0x79,
    0x70, 0x65, 0x73, 0x2e, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x52,
    0x05, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x3c, 0x0a, 0x08, 0x65, 0x76, 0x69, 0x64, 0x65, 0x6e,
    0x63, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x74, 0x65, 0x6e, 0x64, 0x65,
    0x72, 0x6d, 0x69, 0x6e, 0x74, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x45, 0x76, 0x69, 0x64,
    0x65, 0x6e, 0x63, 0x65, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x52, 0x08, 0x65, 0x76, 0x69, 0x64,
    0x65, 0x6e, 0x63, 0x65, 0x12, 0x3f, 0x0a, 0x09, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x6f,
    0x72, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x72,
    0x6d, 0x69, 0x6e, 0x74, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x56, 0x61, 0x6c, 0x69, 0x64,
    0x61, 0x74, 0x6f, 0x72, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x52, 0x09, 0x76, 0x61, 0x6c, 0x69,
    0x64, 0x61, 0x74, 0x6f, 0x72, 0x3a, 0x0e, 0x82, 0xe7, 0xb0, 0x2a, 0x09, 0x61, 0x75, 0x74, 0x68,
    0x6f, 0x72, 0x69, 0x74, 0x79, 0x22, 0x19, 0x0a, 0x17, 0x4d, 0x73, 0x67, 0x55, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x32, 0x69, 0x0a, 0x03, 0x4d, 0x73, 0x67, 0x12, 0x62, 0x0a, 0x0c, 0x55, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x12, 0x24, 0x2e, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73,
    0x2e, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x4d, 0x73,
    0x67, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x1a, 0x2c, 0x2e,
    0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2e, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73,
    0x2e, 0x76, 0x31, 0x2e, 0x4d, 0x73, 0x67, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72,
    0x61, 0x6d, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x30, 0x5a, 0x2e, 0x67,
    0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73,
    0x2f, 0x63, 0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x2f, 0x78, 0x2f, 0x63, 0x6f,
    0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x4a, 0x8b, 0x09,
    0x0a, 0x06, 0x12, 0x04, 0x01, 0x00, 0x26, 0x22, 0x0a, 0x22, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x01,
    0x00, 0x12, 0x1a, 0x18, 0x20, 0x53, 0x69, 0x6e, 0x63, 0x65, 0x3a, 0x20, 0x63, 0x6f, 0x73, 0x6d,
    0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x20, 0x30, 0x2e, 0x34, 0x37, 0x0a, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x02, 0x00, 0x1c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00,
    0x23, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x21, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x02, 0x12, 0x03, 0x06, 0x00, 0x27, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00,
    0x45, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x08, 0x00, 0x45, 0x0a, 0x2f, 0x0a, 0x02,
    0x06, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x11, 0x01, 0x1a, 0x23, 0x20, 0x4d, 0x73, 0x67, 0x20, 0x64,
    0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x61, 0x6e, 0x6b, 0x20,
    0x4d, 0x73, 0x67, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x0b, 0x0a, 0xb3, 0x01, 0x0a, 0x04, 0x06, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x10, 0x02, 0x46, 0x1a, 0xa5, 0x01, 0x20, 0x55, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20,
    0x61, 0x20, 0x67, 0x6f, 0x76, 0x65, 0x72, 0x6e, 0x61, 0x6e, 0x63, 0x65, 0x20, 0x6f, 0x70, 0x65,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74,
    0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x78, 0x2f, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e,
    0x73, 0x75, 0x73, 0x5f, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65,
    0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x0a, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x20, 0x69, 0x73, 0x20, 0x64,
    0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65,
    0x65, 0x70, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x20, 0x53, 0x69, 0x6e, 0x63, 0x65, 0x3a, 0x20, 0x63,
    0x6f, 0x73, 0x6d, 0x6f, 0x73, 0x2d, 0x73, 0x64, 0x6b, 0x20, 0x30, 0x2e, 0x34, 0x37, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x06, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x10, 0x13, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x10, 0x2d, 0x44, 0x0a, 0x43, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x14, 0x00, 0x22, 0x01, 0x1a, 0x37, 0x20, 0x4d, 0x73, 0x67, 0x55, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4d,
    0x73, 0x67, 0x2f, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x14, 0x08, 0x17, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x07, 0x12, 0x03, 0x15, 0x02, 0x2e, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x07, 0xf0, 0x8c, 0xa6,
    0x05, 0x00, 0x12, 0x03, 0x15, 0x02, 0x2e, 0x0a, 0x68, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x18, 0x02, 0x48, 0x1a, 0x5b, 0x20, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79,
    0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20,
    0x74, 0x68, 0x61, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x73, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x20, 0x28, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c,
    0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x78, 0x2f, 0x67, 0x6f, 0x76, 0x20, 0x75, 0x6e, 0x6c, 0x65,
    0x73, 0x73, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x77, 0x72, 0x69, 0x74, 0x74, 0x65, 0x6e, 0x29, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x18, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x09, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x15, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x18, 0x17, 0x47, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02,
    0x00, 0x08, 0xca, 0xd6, 0x05, 0x12, 0x03, 0x18, 0x18, 0x46, 0x0a, 0xcb, 0x01, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x02, 0x31, 0x1a, 0xbd, 0x01, 0x20, 0x70, 0x61, 0x72, 0x61,
    0x6d, 0x73, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x78,
    0x2f, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d,
    0x65, 0x74, 0x65, 0x72, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x2e,
    0x0a, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73,
    0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x64,
    0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x4d, 0x73, 0x67, 0x20, 0x62, 0x65, 0x63,
    0x61, 0x75, 0x73, 0x65, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x74, 0x72, 0x61, 0x63, 0x6b,
    0x65, 0x64, 0x0a, 0x20, 0x73, 0x65, 0x70, 0x61, 0x72, 0x61, 0x72, 0x61, 0x74, 0x65, 0x6c, 0x79,
    0x20, 0x69, 0x6e, 0x20, 0x78, 0x2f, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x2e, 0x0a, 0x0a,
    0x20, 0x4e, 0x4f, 0x54, 0x45, 0x3a, 0x20, 0x41, 0x6c, 0x6c, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d,
    0x65, 0x74, 0x65, 0x72, 0x73, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x62, 0x65, 0x20, 0x73, 0x75,
    0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x1f, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x1f, 0x23, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1f,
    0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x20, 0x02, 0x31, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x20, 0x02, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x20, 0x23, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x20, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x21, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12,
    0x03, 0x21, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x21,
    0x23, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x21, 0x2f, 0x30,
    0x0a, 0x6d, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x03, 0x26, 0x00, 0x22, 0x1a, 0x62, 0x20, 0x4d, 0x73,
    0x67, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x73, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x73, 0x74, 0x72, 0x75, 0x63,
    0x74, 0x75, 0x72, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69,
    0x6e, 0x67, 0x20, 0x61, 0x0a, 0x20, 0x4d, 0x73, 0x67, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50,
    0x61, 0x72, 0x61, 0x6d, 0x73, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x26, 0x08, 0x1f, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
];
include!("cosmos.consensus.v1.tonic.rs");
// @@protoc_insertion_point(module)