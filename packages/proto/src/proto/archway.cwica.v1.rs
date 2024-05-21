// @generated
/// ModuleErrors defines the module level error codes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModuleErrors {
    /// ERR_UNKNOWN is the default error code
    ErrUnknown = 0,
    /// ERR_PACKET_TIMEOUT is the error code for packet timeout
    ErrPacketTimeout = 1,
    /// ERR_EXEC_FAILURE is the error code for tx execution failure
    ErrExecFailure = 2,
}
impl ModuleErrors {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ModuleErrors::ErrUnknown => "ERR_UNKNOWN",
            ModuleErrors::ErrPacketTimeout => "ERR_PACKET_TIMEOUT",
            ModuleErrors::ErrExecFailure => "ERR_EXEC_FAILURE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ERR_UNKNOWN" => Some(Self::ErrUnknown),
            "ERR_PACKET_TIMEOUT" => Some(Self::ErrPacketTimeout),
            "ERR_EXEC_FAILURE" => Some(Self::ErrExecFailure),
            _ => None,
        }
    }
}
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// Defines maximum amount of messages which can be passed in MsgSendTx
    #[prost(uint64, tag="1")]
    pub msg_send_tx_max_messages: u64,
}
/// GenesisState defines the cwica module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters for the module
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// Sudopayload is the payload for the sudo call sent by the cwica module on IBC
/// actions
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SudoPayload {
    /// ICA is the message which carries the success responses
    #[prost(message, optional, tag="1")]
    pub ica: ::core::option::Option<IcaSuccess>,
}
/// ICASuccess is the success message after the ICA operation has taken place
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IcaSuccess {
    /// account_registered is the message which carries the success response after
    /// the ica account has been registered
    #[prost(message, optional, tag="1")]
    pub account_registered: ::core::option::Option<AccountRegistered>,
    /// tx_executed is the message which carries the success response after the ica
    /// tx has been executed
    #[prost(message, optional, tag="2")]
    pub tx_executed: ::core::option::Option<TxExecuted>,
}
/// AccountRegistered is contains the address of the registered account on the
/// counterparty chain
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountRegistered {
    /// counterparty_address is the address of the account on the counterparty
    /// chain
    #[prost(string, tag="1")]
    pub counterparty_address: ::prost::alloc::string::String,
}
/// TxExecuted is the response message after the execute of the ICA tx
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxExecuted {
    /// packet is the ibc packet which was executed
    #[prost(message, optional, tag="1")]
    pub packet: ::core::option::Option<super::super::super::ibc::core::channel::v1::Packet>,
    /// data is the response data after the tx has been executed
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// MsgRegisterInterchainAccount defines the Msg/RegisterInterchainAccount
/// request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterInterchainAccount {
    /// contract_address is the address of the contrat who wants to register an ica
    /// account on the counterparty chain
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
    /// connection_id is the connection id between the two chains
    #[prost(string, tag="2")]
    pub connection_id: ::prost::alloc::string::String,
}
/// MsgRegisterInterchainAccountResponse defines the response for
/// Msg/RegisterInterchainAccount
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterInterchainAccountResponse {
}
/// MsgSendTx defines the Msg/SendTx request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendTx {
    /// contract_address is the address of the who wants to submit a transaction to
    /// the counterparty chain
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
    /// connection_id is the connection id between the two chains
    #[prost(string, tag="2")]
    pub connection_id: ::prost::alloc::string::String,
    /// msgs are the messages to be submitted to the counterparty chain
    #[prost(message, repeated, tag="3")]
    pub msgs: ::prost::alloc::vec::Vec<::prost_types::Any>,
    /// memo is the memo to be included in the packet
    #[prost(string, tag="4")]
    pub memo: ::prost::alloc::string::String,
    /// timeout in seconds after which the packet times out
    #[prost(uint64, tag="5")]
    pub timeout: u64,
}
/// MsgSendTxResponse defines the response for Msg/SendTx
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendTxResponse {
    /// sequence_id is the channel's sequence_id for outgoing ibc packet. Unique
    /// per a channel.
    #[prost(uint64, tag="1")]
    pub sequence_id: u64,
    /// channel is the channel id the transaction was submitted from
    #[prost(string, tag="2")]
    pub channel: ::prost::alloc::string::String,
}
/// MsgUpdateParams is the MsgUpdateParams request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the authority that is allowed to update the
    /// cwica module parameters.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// params deines the module parmeters to update
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse is the MsgUpdateParams response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
include!("archway.cwica.v1.tonic.rs");
// @@protoc_insertion_point(module)