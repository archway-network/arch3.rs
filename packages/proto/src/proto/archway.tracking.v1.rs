// @generated
/// TxInfo keeps a transaction gas tracking data.
/// Object is being created at the module EndBlocker.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxInfo {
    /// id defines the unique transaction ID.
    #[prost(uint64, tag="1")]
    pub id: u64,
    /// height defines the block height of the transaction.
    #[prost(int64, tag="2")]
    pub height: i64,
    /// total_gas defines total gas consumption by the transaction.
    /// It is the sum of gas consumed by all contract operations (VM + SDK gas).
    #[prost(uint64, tag="3")]
    pub total_gas: u64,
}
/// ContractOperationInfo keeps a single contract operation gas consumption data.
/// Object is being created by the IngestGasRecord call from the wasmd.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractOperationInfo {
    /// id defines the unique operation ID.
    #[prost(uint64, tag="1")]
    pub id: u64,
    /// tx_id defines a transaction ID operation relates to (TxInfo.id).
    #[prost(uint64, tag="2")]
    pub tx_id: u64,
    /// contract_address defines the contract address operation relates to.
    #[prost(string, tag="3")]
    pub contract_address: ::prost::alloc::string::String,
    /// operation_type defines the gas consumption type.
    #[prost(enumeration="ContractOperation", tag="4")]
    pub operation_type: i32,
    /// vm_gas is the gas consumption reported by the WASM VM.
    /// Value is adjusted by this module (CalculateUpdatedGas func).
    #[prost(uint64, tag="5")]
    pub vm_gas: u64,
    /// sdk_gas is the gas consumption reported by the SDK gas meter and the WASM
    /// GasRegister (cost of Execute/Query/etc). Value is adjusted by this module
    /// (CalculateUpdatedGas func).
    #[prost(uint64, tag="6")]
    pub sdk_gas: u64,
}
/// BlockTracking is the tracking information for a block.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockTracking {
    /// txs defines the list of transactions tracked in the block.
    #[prost(message, repeated, tag="1")]
    pub txs: ::prost::alloc::vec::Vec<TxTracking>,
}
/// TxTracking is the tracking information for a single transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxTracking {
    /// info defines the transaction details.
    #[prost(message, optional, tag="1")]
    pub info: ::core::option::Option<TxInfo>,
    /// contract_operations defines the list of contract operations consumed by the
    /// transaction.
    #[prost(message, repeated, tag="2")]
    pub contract_operations: ::prost::alloc::vec::Vec<ContractOperationInfo>,
}
/// ContractOperation denotes which operation consumed gas.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContractOperation {
    /// Invalid or unknown operation
    Unspecified = 0,
    /// Instantiate operation
    Instantiation = 1,
    /// Execute operation
    Execution = 2,
    /// Query
    Query = 3,
    /// Migrate operation
    Migrate = 4,
    /// IBC operations
    Ibc = 5,
    /// Sudo operation
    Sudo = 6,
    /// Reply callback operation
    Reply = 7,
}
impl ContractOperation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContractOperation::Unspecified => "CONTRACT_OPERATION_UNSPECIFIED",
            ContractOperation::Instantiation => "CONTRACT_OPERATION_INSTANTIATION",
            ContractOperation::Execution => "CONTRACT_OPERATION_EXECUTION",
            ContractOperation::Query => "CONTRACT_OPERATION_QUERY",
            ContractOperation::Migrate => "CONTRACT_OPERATION_MIGRATE",
            ContractOperation::Ibc => "CONTRACT_OPERATION_IBC",
            ContractOperation::Sudo => "CONTRACT_OPERATION_SUDO",
            ContractOperation::Reply => "CONTRACT_OPERATION_REPLY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CONTRACT_OPERATION_UNSPECIFIED" => Some(Self::Unspecified),
            "CONTRACT_OPERATION_INSTANTIATION" => Some(Self::Instantiation),
            "CONTRACT_OPERATION_EXECUTION" => Some(Self::Execution),
            "CONTRACT_OPERATION_QUERY" => Some(Self::Query),
            "CONTRACT_OPERATION_MIGRATE" => Some(Self::Migrate),
            "CONTRACT_OPERATION_IBC" => Some(Self::Ibc),
            "CONTRACT_OPERATION_SUDO" => Some(Self::Sudo),
            "CONTRACT_OPERATION_REPLY" => Some(Self::Reply),
            _ => None,
        }
    }
}
/// GenesisState defines the initial state of the tracking module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// tx_info_last_id defines the last unique ID for a TxInfo objs.
    #[prost(uint64, tag="1")]
    pub tx_info_last_id: u64,
    /// tx_infos defines a list of all the tracked transactions.
    #[prost(message, repeated, tag="2")]
    pub tx_infos: ::prost::alloc::vec::Vec<TxInfo>,
    /// contract_op_info_last_id defines the last unique ID for
    /// ContractOperationInfo objs.
    #[prost(uint64, tag="3")]
    pub contract_op_info_last_id: u64,
    /// contract_op_infos defines a list of all the tracked contract operations.
    #[prost(message, repeated, tag="4")]
    pub contract_op_infos: ::prost::alloc::vec::Vec<ContractOperationInfo>,
}
/// QueryBlockGasTrackingRequest is the request for Query.BlockGasTracking.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockGasTrackingRequest {
}
/// QueryBlockGasTrackingResponse is the response for Query.BlockGasTracking.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockGasTrackingResponse {
    #[prost(message, optional, tag="1")]
    pub block: ::core::option::Option<BlockTracking>,
}
include!("archway.tracking.v1.tonic.rs");
// @@protoc_insertion_point(module)