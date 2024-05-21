// @generated
/// Callback defines the callback structure.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Callback {
    /// contract_address is the address of the contract which is requesting the callback (bech32 encoded).
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
    /// job_id is an identifier the callback requestor can pass in to identify the callback when it happens. 
    #[prost(uint64, tag="2")]
    pub job_id: u64,
    /// callback_height is the height at which the callback is executed.
    #[prost(int64, tag="3")]
    pub callback_height: i64,
    /// fee_split is the breakdown of the fees paid by the contract to reserve the callback
    #[prost(message, optional, tag="4")]
    pub fee_split: ::core::option::Option<CallbackFeesFeeSplit>,
    /// reserved_by is the address which reserved the callback (bech32 encoded).
    #[prost(string, tag="5")]
    pub reserved_by: ::prost::alloc::string::String,
    /// callback_gas_limit is the maximum gas that can be consumed by this callback.
    #[prost(uint64, tag="6")]
    pub max_gas_limit: u64,
}
/// CallbackFeesFeeSplit is the breakdown of all the fees that need to be paid by the contract to reserve a callback
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallbackFeesFeeSplit {
    /// transaction_fees is the transaction fees for the callback based on its gas consumption
    #[prost(message, optional, tag="1")]
    pub transaction_fees: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// block_reservation_fees is the block reservation fees portion of the callback reservation fees
    #[prost(message, optional, tag="2")]
    pub block_reservation_fees: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// future_reservation_fees is the future reservation fees portion of the callback reservation fees
    #[prost(message, optional, tag="3")]
    pub future_reservation_fees: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// surplus_fees is any extra fees passed in for the registration of the callback
    #[prost(message, optional, tag="4")]
    pub surplus_fees: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// Params defines the module parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// callback_gas_limit is the maximum gas that can be consumed by a callback.
    #[prost(uint64, tag="1")]
    pub callback_gas_limit: u64,
    /// max_block_reservation_limit is the maximum number of callbacks which can be registered in a given block. 
    #[prost(uint64, tag="2")]
    pub max_block_reservation_limit: u64,
    /// max_future_reservation_limit is the maximum number of blocks in the future that a contract can request a callback in.
    #[prost(uint64, tag="3")]
    pub max_future_reservation_limit: u64,
    /// block_reservation_fee_multiplier is used to calculate a part of the reservation fees which will need to be paid when requesting the callback. 
    #[prost(string, tag="4")]
    pub block_reservation_fee_multiplier: ::prost::alloc::string::String,
    /// future_reservation_fee_multiplier is used to calculate a part of the reservation fees which will need to be paid while requesting the callback. 
    #[prost(string, tag="5")]
    pub future_reservation_fee_multiplier: ::prost::alloc::string::String,
}
/// ModuleErrors defines the module level error codes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModuleErrors {
    /// ERR_UNKNOWN is the default error code
    ErrUnknown = 0,
    /// ERR_OUT_OF_GAS is the error code when the contract callback exceeds the gas limit allowed by the module
    ErrOutOfGas = 1,
    /// ERR_CONTRACT_EXECUTION_FAILED is the error code when the contract callback execution fails
    ErrContractExecutionFailed = 2,
}
impl ModuleErrors {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ModuleErrors::ErrUnknown => "ERR_UNKNOWN",
            ModuleErrors::ErrOutOfGas => "ERR_OUT_OF_GAS",
            ModuleErrors::ErrContractExecutionFailed => "ERR_CONTRACT_EXECUTION_FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ERR_UNKNOWN" => Some(Self::ErrUnknown),
            "ERR_OUT_OF_GAS" => Some(Self::ErrOutOfGas),
            "ERR_CONTRACT_EXECUTION_FAILED" => Some(Self::ErrContractExecutionFailed),
            _ => None,
        }
    }
}
/// CallbackRegisteredEvent is emitted when a callback is registered.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallbackRegisteredEvent {
    /// contract_address is the address of the contract for which callback is being registered (bech32 encoded).
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
    /// job_id is an identifier of the callback. 
    #[prost(uint64, tag="2")]
    pub job_id: u64,
    /// callback_height is the height at which the callback is executed.
    #[prost(int64, tag="3")]
    pub callback_height: i64,
    /// fee_split is the breakdown of the fees paid by the contract to reserve the callback
    #[prost(message, optional, tag="4")]
    pub fee_split: ::core::option::Option<CallbackFeesFeeSplit>,
    /// reserved_by is the address which reserved the callback (bech32 encoded).
    #[prost(string, tag="5")]
    pub reserved_by: ::prost::alloc::string::String,
}
/// CallbackCancelledEvent is emitted when a callback is cancelled.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallbackCancelledEvent {
    /// cancelled_by is the address of the contract whose callback is being cancelled (bech32 encoded)
    #[prost(string, tag="1")]
    pub cancelled_by: ::prost::alloc::string::String,
    /// contract_address is the address of the contract (bech32 encoded)
    #[prost(string, tag="2")]
    pub contract_address: ::prost::alloc::string::String,
    /// job_id is an identifier the callback requestor had passed during registration of the callback
    #[prost(uint64, tag="3")]
    pub job_id: u64,
    /// callback_height is the height at which the callback requestor had registered the callback
    #[prost(int64, tag="4")]
    pub callback_height: i64,
    /// refund_amount is the amount of fees which was refunded on cancellation
    #[prost(message, optional, tag="5")]
    pub refund_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// CallbackExecutedSuccessEvent is emitted when a callback is executed successfully.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallbackExecutedSuccessEvent {
    /// contract_address is the address of the contract for which callback is being executed (bech32 encoded).
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
    /// job_id is an identifier of the callback.
    #[prost(uint64, tag="2")]
    pub job_id: u64,
    /// sudo_msg is the input passed by the module to the contract
    #[prost(string, tag="3")]
    pub sudo_msg: ::prost::alloc::string::String,
    /// gas_used is the amount of gas consumed during the callback execution
    #[prost(uint64, tag="4")]
    pub gas_used: u64,
}
/// CallbackExecutedFailedEvent is emitted when a callback execution fails.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallbackExecutedFailedEvent {
    /// contract_address is the address of the contract for which callback is being executed (bech32 encoded).
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
    /// job_id is an identifier of the callback.
    #[prost(uint64, tag="2")]
    pub job_id: u64,
    /// sudo_msg is the input passed by the module to the contract
    #[prost(string, tag="3")]
    pub sudo_msg: ::prost::alloc::string::String,
    /// gas_used is the amount of gas consumed during the callback execution
    #[prost(uint64, tag="4")]
    pub gas_used: u64,
    /// error is the error returned during the callback execution
    #[prost(string, tag="5")]
    pub error: ::prost::alloc::string::String,
}
/// GenesisState defines the initial state of the callback module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the module parameters.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    /// callbacks defines all the callbacks which are yet to be executed
    #[prost(message, repeated, tag="2")]
    pub callbacks: ::prost::alloc::vec::Vec<Callback>,
}
/// QueryParamsRequest is the request for Query.Params.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is the response for Query.Params.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines all the module parameters.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryEstimateCallbackFeesRequest is the request for Query.EstimateCallbackFees.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEstimateCallbackFeesRequest {
    /// block_height is the height at which to estimate the callback fees
    #[prost(int64, tag="1")]
    pub block_height: i64,
}
/// QueryEstimateCallbackFeesResponse is the response for Query.EstimateCallbackFees.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEstimateCallbackFeesResponse {
    /// total_fees is the total fees that needs to be paid by the contract to reserve a callback
    #[prost(message, optional, tag="1")]
    pub total_fees: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// fee_split is the breakdown of the total_fees
    #[prost(message, optional, tag="2")]
    pub fee_split: ::core::option::Option<CallbackFeesFeeSplit>,
}
/// QueryCallbacksRequest is the request for Query.Callbacks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCallbacksRequest {
    /// block_height is the height at which to query the callbacks
    #[prost(int64, tag="1")]
    pub block_height: i64,
}
/// QueryCallbacksResponse is the response for Query.Callbacks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCallbacksResponse {
    /// callbacks is the list of callbacks registered at the given height
    #[prost(message, repeated, tag="1")]
    pub callbacks: ::prost::alloc::vec::Vec<Callback>,
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/callback parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a MsgUpdateParams message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
/// MsgRequestCallback is the Msg/RequestCallback request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestCallback {
    /// sender is the address who is requesting the callback (bech32 encoded)
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// contract_address is the address of the contract which is requesting the callback (bech32 encoded)
    #[prost(string, tag="2")]
    pub contract_address: ::prost::alloc::string::String,
    /// job_id is an identifier the callback requestor can pass in to identify the callback when it happens
    #[prost(uint64, tag="3")]
    pub job_id: u64,
    /// callback_height is the height at which the callback is executed.
    #[prost(int64, tag="4")]
    pub callback_height: i64,
    /// fees is the amount of fees being paid to register the contract
    #[prost(message, optional, tag="5")]
    pub fees: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgRequestCallbackResponse defines the response structure for executing a MsgRequestCallback message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRequestCallbackResponse {
}
/// MsgCancelCallback is the Msg/CancelCallback request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelCallback {
    /// sender is the address of the contract which is cancelling the callback (bech32 encoded)
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// contract_address is the address of the contract (bech32 encoded)
    #[prost(string, tag="2")]
    pub contract_address: ::prost::alloc::string::String,
    /// job_id is an identifier the callback requestor had passed during registration of the callback
    #[prost(uint64, tag="3")]
    pub job_id: u64,
    /// callback_height is the height at which the callback requestor had registered the callback
    #[prost(int64, tag="4")]
    pub callback_height: i64,
}
/// MsgCancelCallbackResponse defines the response structure for executing a MsgCancelCallback message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelCallbackResponse {
    /// refund is the amount of fees being refunded due to the cancellation of the callback
    #[prost(message, optional, tag="1")]
    pub refund: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
include!("archway.callback.v1.tonic.rs");
// @@protoc_insertion_point(module)