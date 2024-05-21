// @generated
/// SudoError defines the sudo message for the error callback
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SudoError {
    /// module_name is the name of the module throwing the error
    #[prost(string, tag="1")]
    pub module_name: ::prost::alloc::string::String,
    /// error_code is the module level error code
    #[prost(int32, tag="2")]
    pub error_code: i32,
    /// contract_address is the address of the contract which will receive the
    /// error callback
    #[prost(string, tag="3")]
    pub contract_address: ::prost::alloc::string::String,
    /// input_payload is any input which caused the error
    #[prost(string, tag="4")]
    pub input_payload: ::prost::alloc::string::String,
    /// error_message is the error message
    #[prost(string, tag="5")]
    pub error_message: ::prost::alloc::string::String,
}
/// ModuleErrors defines the module level error codes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModuleErrors {
    /// ERR_UNKNOWN is the default error code
    ErrUnknown = 0,
    /// ERR_CALLBACK_EXECUTION_FAILED is the error code for when the error callback fails
    ErrCallbackExecutionFailed = 1,
}
impl ModuleErrors {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ModuleErrors::ErrUnknown => "ERR_UNKNOWN",
            ModuleErrors::ErrCallbackExecutionFailed => "ERR_CALLBACK_EXECUTION_FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ERR_UNKNOWN" => Some(Self::ErrUnknown),
            "ERR_CALLBACK_EXECUTION_FAILED" => Some(Self::ErrCallbackExecutionFailed),
            _ => None,
        }
    }
}
/// Params defines the set of parameters for the cwerrors module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// error_stored_time is the block height until which error is stored
    #[prost(int64, tag="1")]
    pub error_stored_time: i64,
    /// subsciption_fee is the fee required to subscribe to error callbacks
    #[prost(message, optional, tag="2")]
    pub subscription_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// subscription_period is the period for which the subscription is valid
    #[prost(int64, tag="3")]
    pub subscription_period: i64,
}
/// ParamsUpdatedEvent defines the event which is thrown when the module
/// parameters are updated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsUpdatedEvent {
    /// new_params are the new parameters for the module
    #[prost(message, optional, tag="1")]
    pub new_params: ::core::option::Option<Params>,
    /// authority is the address of the authority that updated the parameters
    #[prost(string, tag="2")]
    pub authority: ::prost::alloc::string::String,
}
/// SubscribedToErrorsEvent defines the event which is thrown when a contract
/// subscribes to errors
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribedToErrorsEvent {
    /// sender is the address which initiated the subscription
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// contract_address is the address of the contract which is subscribed to
    /// errors
    #[prost(string, tag="2")]
    pub contract_address: ::prost::alloc::string::String,
    /// fees_paid is the fees paid for the subscription
    #[prost(message, optional, tag="3")]
    pub fees_paid: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// subscription_valid_till is the block height till which the subscription is
    /// valid
    #[prost(int64, tag="4")]
    pub subscription_valid_till: i64,
}
/// StoringErrorEvent defines the event which is thrown when an error is stored
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoringErrorEvent {
    /// error is the error which is stored
    #[prost(message, optional, tag="1")]
    pub error: ::core::option::Option<SudoError>,
    /// deletion_block_height is the block height at which the error will be pruned
    /// from the state
    #[prost(int64, tag="2")]
    pub deletion_block_height: i64,
}
/// SudoErrorCallbackFailedEvent defines the event which is thrown when a sudo
/// error callback fails
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SudoErrorCallbackFailedEvent {
    /// error is the error for which the callback is executed
    #[prost(message, optional, tag="1")]
    pub error: ::core::option::Option<SudoError>,
    /// callback_error_message is the error message of why the callback failed
    #[prost(string, tag="2")]
    pub callback_error_message: ::prost::alloc::string::String,
}
/// GenesisState defines the cwerrors module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the module parameters.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    /// errors defines all the sudo errors currently registered.
    #[prost(message, repeated, tag="2")]
    pub errors: ::prost::alloc::vec::Vec<SudoError>,
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
/// QueryErrorsRequest is the request for Query.Errors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryErrorsRequest {
    /// contract_address is the address of the contract whose errors to query for
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
}
/// QueryErrorsResponse is the response for Query.Errors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryErrorsResponse {
    /// errors defines all the contract errors which will be returned
    #[prost(message, repeated, tag="1")]
    pub errors: ::prost::alloc::vec::Vec<SudoError>,
}
/// QueryIsSubscribedRequest is the request for Query.IsSubscribed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIsSubscribedRequest {
    /// contract_address is the address of the contract to query if subscribed
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
}
/// QueryIsSubscribedResponse is the response for Query.IsSubscribed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIsSubscribedResponse {
    /// subscribed defines if the contract is subscribed to sudo error callbacks
    #[prost(bool, tag="1")]
    pub subscribed: bool,
    /// subscription_valid_till defines the block height till which the
    /// subscription is valid
    #[prost(int64, tag="2")]
    pub subscription_valid_till: i64,
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless
    /// overwritten).
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/cwerrors parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
/// MsgSubscribeToError is the Msg/SubscribeToError request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubscribeToError {
    /// sender is the address of who is registering the contarcts for callback on
    /// error
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// contract is the address of the contract that will be called on error
    #[prost(string, tag="2")]
    pub contract_address: ::prost::alloc::string::String,
    /// fee is the subscription fee for the feature (current no fee is charged for
    /// this feature)
    #[prost(message, optional, tag="3")]
    pub fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSubscribeToErrorResponse defines the response structure for executing a
/// MsgSubscribeToError message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubscribeToErrorResponse {
    /// subscription_valid_till is the block height till which the subscription is
    /// valid
    #[prost(int64, tag="1")]
    pub subscription_valid_till: i64,
}
include!("archway.cwerrors.v1.tonic.rs");
// @@protoc_insertion_point(module)