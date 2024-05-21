// @generated
/// Params defines the module parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// inflation_rewards_ratio defines the percentage of minted inflation tokens
    /// that are used for dApp rewards \[0.0, 1.0\]. If set to 0.0, no inflation
    /// rewards are distributed.
    #[prost(string, tag="1")]
    pub inflation_rewards_ratio: ::prost::alloc::string::String,
    /// tx_fee_rebate_ratio defines the percentage of tx fees that are used for
    /// dApp rewards \[0.0, 1.0\]. If set to 0.0, no fee rewards are distributed.
    #[prost(string, tag="2")]
    pub tx_fee_rebate_ratio: ::prost::alloc::string::String,
    /// max_withdraw_records defines the maximum number of RewardsRecord objects
    /// used for the withdrawal operation.
    #[prost(uint64, tag="3")]
    pub max_withdraw_records: u64,
    /// min_price_of_gas defines the minimum price for each single unit of gas in
    /// the network. during the min consensus fee ante handler we will be taking
    /// the max between min consensus fee and minimum price of gas to compute the
    /// minimum tx computational fees, which are independent from contract flat
    /// fees (premiums)
    #[prost(message, optional, tag="4")]
    pub min_price_of_gas: ::core::option::Option<super::super::super::cosmos::base::v1beta1::DecCoin>,
}
/// ContractMetadata defines the contract rewards distribution options for a
/// particular contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractMetadata {
    /// contract_address defines the contract address (bech32 encoded).
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
    /// owner_address is the contract owner address that can modify contract reward
    /// options (bech32 encoded). That could be the contract admin or the contract
    /// itself. If owner_address is set to contract address, contract can modify
    /// the metadata on its own using WASM bindings.
    #[prost(string, tag="2")]
    pub owner_address: ::prost::alloc::string::String,
    /// rewards_address is an address to distribute rewards to (bech32 encoded).
    /// If not set (empty), rewards are not distributed for this contract.
    #[prost(string, tag="3")]
    pub rewards_address: ::prost::alloc::string::String,
    /// withdraw_to_wallet is a flag that defines if rewards should be immediately
    /// withdrawn to the wallet instead of creating a rewards record to be lazily
    /// withdrawn after.
    #[prost(bool, tag="4")]
    pub withdraw_to_wallet: bool,
}
/// BlockRewards defines block related rewards distribution data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockRewards {
    /// height defines the block height.
    #[prost(int64, tag="1")]
    pub height: i64,
    /// inflation_rewards is the rewards to be distributed.
    #[prost(message, optional, tag="2")]
    pub inflation_rewards: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// max_gas defines the maximum gas for the block that is used to distribute
    /// inflation rewards (consensus parameter).
    #[prost(uint64, tag="3")]
    pub max_gas: u64,
}
/// TxRewards defines transaction related rewards distribution data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxRewards {
    /// tx_id is the tracking transaction ID (x/tracking is the data source for
    /// this value).
    #[prost(uint64, tag="1")]
    pub tx_id: u64,
    /// height defines the block height.
    #[prost(int64, tag="2")]
    pub height: i64,
    /// fee_rewards is the rewards to be distributed.
    #[prost(message, repeated, tag="3")]
    pub fee_rewards: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// RewardsRecord defines a record that is used to distribute rewards later (lazy
/// distribution). This record is being created by the x/rewards EndBlocker and
/// pruned after the rewards are distributed. An actual rewards x/bank transfer
/// might be triggered by a Tx (via CLI for example) or by a contract via WASM
/// bindings. For a contract to trigger rewards transfer, contract address must
/// be set as the rewards_address in a corresponding ContractMetadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsRecord {
    /// id is the unique ID of the record.
    #[prost(uint64, tag="1")]
    pub id: u64,
    /// rewards_address is the address to distribute rewards to (bech32 encoded).
    #[prost(string, tag="2")]
    pub rewards_address: ::prost::alloc::string::String,
    /// rewards are the rewards to be transferred later.
    #[prost(message, repeated, tag="3")]
    pub rewards: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// calculated_height defines the block height of rewards calculation event.
    #[prost(int64, tag="4")]
    pub calculated_height: i64,
    /// calculated_time defines the block time of rewards calculation event.
    #[prost(message, optional, tag="5")]
    pub calculated_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// FlatFee defines the flat fee for a particular contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlatFee {
    /// contract_address defines the contract address (bech32 encoded).
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
    /// flat_fee defines the minimum flat fee set by the contract_owner
    #[prost(message, optional, tag="2")]
    pub flat_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// ContractMetadataSetEvent is emitted when the contract metadata is created or
/// updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractMetadataSetEvent {
    /// contract_address defines the contract address.
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
    /// metadata defines the new contract metadata state.
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<ContractMetadata>,
}
/// ContractRewardCalculationEvent is emitted when the contract reward is
/// calculated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractRewardCalculationEvent {
    /// contract_address defines the contract address.
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
    /// gas_consumed defines the total gas consumption by all WASM operations
    /// within one transaction.
    #[prost(uint64, tag="2")]
    pub gas_consumed: u64,
    /// inflation_rewards defines the inflation rewards portions of the rewards.
    #[prost(message, optional, tag="3")]
    pub inflation_rewards: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// fee_rebate_rewards defines the fee rebate rewards portions of the rewards.
    #[prost(message, repeated, tag="4")]
    pub fee_rebate_rewards: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// metadata defines the contract metadata (if set).
    #[prost(message, optional, tag="5")]
    pub metadata: ::core::option::Option<ContractMetadata>,
}
/// RewardsWithdrawEvent is emitted when credited rewards for a specific
/// rewards_address are distributed. Event could be triggered by a transaction
/// (via CLI for example) or by a contract via WASM bindings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsWithdrawEvent {
    /// rewards_address defines the rewards address rewards are distributed to.
    #[prost(string, tag="1")]
    pub reward_address: ::prost::alloc::string::String,
    /// rewards defines the total rewards being distributed.
    #[prost(message, repeated, tag="2")]
    pub rewards: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MinConsensusFeeSetEvent is emitted when the minimum consensus fee is updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MinConsensusFeeSetEvent {
    /// fee defines the updated minimum gas unit price.
    #[prost(message, optional, tag="1")]
    pub fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::DecCoin>,
}
/// ContractFlatFeeSetEvent is emitted when the contract flat fee is updated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractFlatFeeSetEvent {
    /// contract_address defines the bech32 address of the contract for which the
    /// flat fee is set
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
    /// flat_fee defines the amount that has been set as the minimum fee for the
    /// contract
    #[prost(message, optional, tag="2")]
    pub flat_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the initial state of the tracking module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the module parameters.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    /// contracts_metadata defines a list of all contracts metadata.
    #[prost(message, repeated, tag="2")]
    pub contracts_metadata: ::prost::alloc::vec::Vec<ContractMetadata>,
    /// block_rewards defines a list of all block rewards objects.
    #[prost(message, repeated, tag="3")]
    pub block_rewards: ::prost::alloc::vec::Vec<BlockRewards>,
    /// tx_rewards defines a list of all tx rewards objects.
    #[prost(message, repeated, tag="4")]
    pub tx_rewards: ::prost::alloc::vec::Vec<TxRewards>,
    /// min_consensus_fee defines the minimum gas unit price.
    #[prost(message, optional, tag="5")]
    pub min_consensus_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::DecCoin>,
    /// rewards_record_last_id defines the last unique ID for a RewardsRecord objs.
    #[prost(uint64, tag="6")]
    pub rewards_record_last_id: u64,
    /// rewards_records defines a list of all active (undistributed) rewards
    /// records.
    #[prost(message, repeated, tag="7")]
    pub rewards_records: ::prost::alloc::vec::Vec<RewardsRecord>,
    /// flat_fees defines a list of contract flat fee.
    #[prost(message, repeated, tag="8")]
    pub flat_fees: ::prost::alloc::vec::Vec<FlatFee>,
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
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryContractMetadataRequest is the request for Query.ContractMetadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractMetadataRequest {
    /// contract_address is the contract address (bech32 encoded).
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
}
/// QueryContractMetadataResponse is the response for Query.ContractMetadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<ContractMetadata>,
}
/// QueryBlockRewardsTrackingRequest is the request for
/// Query.BlockRewardsTracking.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockRewardsTrackingRequest {
}
/// QueryBlockRewardsTrackingResponse is the response for
/// Query.BlockRewardsTracking.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockRewardsTrackingResponse {
    #[prost(message, optional, tag="1")]
    pub block: ::core::option::Option<BlockTracking>,
}
/// QueryRewardsPoolRequest is the request for Query.RewardsPool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardsPoolRequest {
}
/// QueryRewardsPoolResponse is the response for Query.RewardsPool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardsPoolResponse {
    /// undistributed_funds are undistributed yet tokens (ready for withdrawal).
    #[prost(message, repeated, tag="1")]
    pub undistributed_funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// treasury_funds are treasury tokens available (no mechanism is available to
    /// withdraw ATM). Treasury tokens are collected on a block basis. Those tokens
    /// are unused block rewards.
    #[prost(message, repeated, tag="2")]
    pub treasury_funds: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryEstimateTxFeesRequest is the request for Query.EstimateTxFees.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEstimateTxFeesRequest {
    /// gas_limit is the transaction gas limit.
    #[prost(uint64, tag="1")]
    pub gas_limit: u64,
    /// contract_address whose flat fee is considered when estimating tx fees.
    #[prost(string, tag="2")]
    pub contract_address: ::prost::alloc::string::String,
}
/// QueryEstimateTxFeesResponse is the response for Query.EstimateTxFees.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEstimateTxFeesResponse {
    /// gas_unit_price defines the minimum transaction fee per gas unit.
    #[prost(message, optional, tag="1")]
    pub gas_unit_price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::DecCoin>,
    /// estimated_fee is the estimated transaction fee for a given gas limit.
    #[prost(message, repeated, tag="2")]
    pub estimated_fee: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// BlockTracking is the tracking information for a block.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockTracking {
    /// inflation_rewards defines the inflation rewards for the block.
    #[prost(message, optional, tag="1")]
    pub inflation_rewards: ::core::option::Option<BlockRewards>,
    /// tx_rewards defines the transaction rewards for the block.
    #[prost(message, repeated, tag="2")]
    pub tx_rewards: ::prost::alloc::vec::Vec<TxRewards>,
}
/// QueryRewardsRecordsRequest is the request for Query.RewardsRecords.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardsRecordsRequest {
    /// rewards_address is the target address to query records for (bech32
    /// encoded).
    #[prost(string, tag="1")]
    pub rewards_address: ::prost::alloc::string::String,
    /// pagination is an optional pagination options for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryRewardsRecordsResponse is the response for Query.RewardsRecords.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardsRecordsResponse {
    /// records is the list of rewards records.
    #[prost(message, repeated, tag="1")]
    pub records: ::prost::alloc::vec::Vec<RewardsRecord>,
    /// pagination is the pagination details in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryOutstandingRewardsRequest is the request for Query.OutstandingRewards.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOutstandingRewardsRequest {
    /// rewards_address is the target address to query calculated rewards for
    /// (bech32 encoded).
    #[prost(string, tag="1")]
    pub rewards_address: ::prost::alloc::string::String,
}
/// QueryOutstandingRewardsResponse is the response for Query.OutstandingRewards.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOutstandingRewardsResponse {
    /// total_rewards is the total rewards credited to the rewards_address.
    #[prost(message, repeated, tag="1")]
    pub total_rewards: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// records_num is the total number of RewardsRecord objects stored for the
    /// rewards_address.
    #[prost(uint64, tag="2")]
    pub records_num: u64,
}
/// QueryFlatFeeRequest is the request for Query.FlatFeet
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFlatFeeRequest {
    /// contract_address is the contract address (bech32 encoded).
    #[prost(string, tag="1")]
    pub contract_address: ::prost::alloc::string::String,
}
/// QueryFlatFeeResponse is the response for Query.FlatFee
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFlatFeeResponse {
    /// flat_fee_amount defines the minimum flat fee set by the contract_owner per
    /// contract execution.
    #[prost(message, optional, tag="1")]
    pub flat_fee_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSetContractMetadata is the request for Msg.SetContractMetadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetContractMetadata {
    /// sender_address is the msg sender address (bech32 encoded).
    #[prost(string, tag="1")]
    pub sender_address: ::prost::alloc::string::String,
    /// metadata is the contract metadata to set / update.
    /// If metadata exists, non-empty fields will be updated.
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<ContractMetadata>,
}
/// MsgSetContractMetadataResponse is the response for Msg.SetContractMetadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetContractMetadataResponse {
}
/// MsgWithdrawRewards is the request for Msg.WithdrawRewards.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawRewards {
    /// rewards_address is the address to distribute rewards to (bech32 encoded).
    #[prost(string, tag="1")]
    pub rewards_address: ::prost::alloc::string::String,
    /// mode defines the operation type.
    #[prost(oneof="msg_withdraw_rewards::Mode", tags="2, 3")]
    pub mode: ::core::option::Option<msg_withdraw_rewards::Mode>,
}
/// Nested message and enum types in `MsgWithdrawRewards`.
pub mod msg_withdraw_rewards {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RecordsLimit {
        #[prost(uint64, tag="1")]
        pub limit: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RecordIDs {
        #[prost(uint64, repeated, tag="1")]
        pub ids: ::prost::alloc::vec::Vec<u64>,
    }
    /// mode defines the operation type.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Mode {
        /// records_limit defines the maximum number of RewardsRecord objects to
        /// process. If provided limit is 0, the default limit is used.
        #[prost(message, tag="2")]
        RecordsLimit(RecordsLimit),
        /// record_ids defines specific RewardsRecord object IDs to process.
        #[prost(message, tag="3")]
        RecordIds(RecordIDs),
    }
}
/// MsgWithdrawRewardsResponse is the response for Msg.WithdrawRewards.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawRewardsResponse {
    /// records_num is the number of RewardsRecord objects processed.
    #[prost(uint64, tag="1")]
    pub records_num: u64,
    /// rewards are the total rewards transferred.
    #[prost(message, repeated, tag="2")]
    pub total_rewards: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSetFlatFee is the request for Msg.SetFlatFee.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetFlatFee {
    /// sender_address is the msg sender address (bech32 encoded).
    #[prost(string, tag="1")]
    pub sender_address: ::prost::alloc::string::String,
    /// contract_address is the contract address (bech32 encoded).
    #[prost(string, tag="2")]
    pub contract_address: ::prost::alloc::string::String,
    /// flat_fee_amount defines the minimum flat fee set by the contract_owner
    #[prost(message, optional, tag="3")]
    pub flat_fee_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSetFlatFeeResponse is the response for Msg.SetFlatFee.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetFlatFeeResponse {
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
///
/// Since: archway v5 && cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/rewards parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: archway v5 && cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
include!("archway.rewards.v1.tonic.rs");
// @@protoc_insertion_point(module)