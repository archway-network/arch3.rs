/// Params defines the module parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// inflation_rewards_ratio defines the percentage of minted inflation tokens
    /// that are used for dApp rewards [0.0, 1.0]. If set to 0.0, no inflation
    /// rewards are distributed.
    #[prost(string, tag = "1")]
    pub inflation_rewards_ratio: ::prost::alloc::string::String,
    /// tx_fee_rebate_ratio defines the percentage of tx fees that are used for
    /// dApp rewards [0.0, 1.0]. If set to 0.0, no fee rewards are distributed.
    #[prost(string, tag = "2")]
    pub tx_fee_rebate_ratio: ::prost::alloc::string::String,
    /// max_withdraw_records defines the maximum number of RewardsRecord objects
    /// used for the withdrawal operation.
    #[prost(uint64, tag = "3")]
    pub max_withdraw_records: u64,
    /// min_price_of_gas defines the minimum price for each single unit of gas in
    /// the network. during the min consensus fee ante handler we will be taking
    /// the max between min consensus fee and minimum price of gas to compute the
    /// minimum tx computational fees, which are independent from contract flat
    /// fees (premiums)
    #[prost(message, optional, tag = "4")]
    pub min_price_of_gas: ::core::option::Option<
        ::cosmos_sdk_proto::cosmos::base::v1beta1::DecCoin,
    >,
}
/// ContractMetadata defines the contract rewards distribution options for a
/// particular contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractMetadata {
    /// contract_address defines the contract address (bech32 encoded).
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    /// owner_address is the contract owner address that can modify contract reward
    /// options (bech32 encoded). That could be the contract admin or the contract
    /// itself. If owner_address is set to contract address, contract can modify
    /// the metadata on its own using WASM bindings.
    #[prost(string, tag = "2")]
    pub owner_address: ::prost::alloc::string::String,
    /// rewards_address is an address to distribute rewards to (bech32 encoded).
    /// If not set (empty), rewards are not distributed for this contract.
    #[prost(string, tag = "3")]
    pub rewards_address: ::prost::alloc::string::String,
}
/// BlockRewards defines block related rewards distribution data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockRewards {
    /// height defines the block height.
    #[prost(int64, tag = "1")]
    pub height: i64,
    /// inflation_rewards is the rewards to be distributed.
    #[prost(message, optional, tag = "2")]
    pub inflation_rewards: ::core::option::Option<
        ::cosmos_sdk_proto::cosmos::base::v1beta1::Coin,
    >,
    /// max_gas defines the maximum gas for the block that is used to distribute
    /// inflation rewards (consensus parameter).
    #[prost(uint64, tag = "3")]
    pub max_gas: u64,
}
/// TxRewards defines transaction related rewards distribution data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxRewards {
    /// tx_id is the tracking transaction ID (x/tracking is the data source for
    /// this value).
    #[prost(uint64, tag = "1")]
    pub tx_id: u64,
    /// height defines the block height.
    #[prost(int64, tag = "2")]
    pub height: i64,
    /// fee_rewards is the rewards to be distributed.
    #[prost(message, repeated, tag = "3")]
    pub fee_rewards: ::prost::alloc::vec::Vec<
        ::cosmos_sdk_proto::cosmos::base::v1beta1::Coin,
    >,
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
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// rewards_address is the address to distribute rewards to (bech32 encoded).
    #[prost(string, tag = "2")]
    pub rewards_address: ::prost::alloc::string::String,
    /// rewards are the rewards to be transferred later.
    #[prost(message, repeated, tag = "3")]
    pub rewards: ::prost::alloc::vec::Vec<
        ::cosmos_sdk_proto::cosmos::base::v1beta1::Coin,
    >,
    /// calculated_height defines the block height of rewards calculation event.
    #[prost(int64, tag = "4")]
    pub calculated_height: i64,
    /// calculated_time defines the block time of rewards calculation event.
    #[prost(message, optional, tag = "5")]
    pub calculated_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// FlatFee defines the flat fee for a particular contract.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlatFee {
    /// contract_address defines the contract address (bech32 encoded).
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    /// flat_fee defines the minimum flat fee set by the contract_owner
    #[prost(message, optional, tag = "2")]
    pub flat_fee: ::core::option::Option<
        ::cosmos_sdk_proto::cosmos::base::v1beta1::Coin,
    >,
}
/// ContractMetadataSetEvent is emitted when the contract metadata is created or
/// updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractMetadataSetEvent {
    /// contract_address defines the contract address.
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    /// metadata defines the new contract metadata state.
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<ContractMetadata>,
}
/// ContractRewardCalculationEvent is emitted when the contract reward is
/// calculated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractRewardCalculationEvent {
    /// contract_address defines the contract address.
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    /// gas_consumed defines the total gas consumption by all WASM operations
    /// within one transaction.
    #[prost(uint64, tag = "2")]
    pub gas_consumed: u64,
    /// inflation_rewards defines the inflation rewards portions of the rewards.
    #[prost(message, optional, tag = "3")]
    pub inflation_rewards: ::core::option::Option<
        ::cosmos_sdk_proto::cosmos::base::v1beta1::Coin,
    >,
    /// fee_rebate_rewards defines the fee rebate rewards portions of the rewards.
    #[prost(message, repeated, tag = "4")]
    pub fee_rebate_rewards: ::prost::alloc::vec::Vec<
        ::cosmos_sdk_proto::cosmos::base::v1beta1::Coin,
    >,
    /// metadata defines the contract metadata (if set).
    #[prost(message, optional, tag = "5")]
    pub metadata: ::core::option::Option<ContractMetadata>,
}
/// RewardsWithdrawEvent is emitted when credited rewards for a specific
/// rewards_address are distributed. Event could be triggered by a transaction
/// (via CLI for example) or by a contract via WASM bindings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsWithdrawEvent {
    /// rewards_address defines the rewards address rewards are distributed to.
    #[prost(string, tag = "1")]
    pub reward_address: ::prost::alloc::string::String,
    /// rewards defines the total rewards being distributed.
    #[prost(message, repeated, tag = "2")]
    pub rewards: ::prost::alloc::vec::Vec<
        ::cosmos_sdk_proto::cosmos::base::v1beta1::Coin,
    >,
}
/// MinConsensusFeeSetEvent is emitted when the minimum consensus fee is updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MinConsensusFeeSetEvent {
    /// fee defines the updated minimum gas unit price.
    #[prost(message, optional, tag = "1")]
    pub fee: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::v1beta1::DecCoin>,
}
/// ContractFlatFeeSetEvent is emitted when the contract flat fee is updated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractFlatFeeSetEvent {
    /// contract_address defines the bech32 address of the contract for which the
    /// flat fee is set
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    /// flat_fee defines the amount that has been set as the minimum fee for the
    /// contract
    #[prost(message, optional, tag = "2")]
    pub flat_fee: ::core::option::Option<
        ::cosmos_sdk_proto::cosmos::base::v1beta1::Coin,
    >,
}
/// GenesisState defines the initial state of the tracking module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the module parameters.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// contracts_metadata defines a list of all contracts metadata.
    #[prost(message, repeated, tag = "2")]
    pub contracts_metadata: ::prost::alloc::vec::Vec<ContractMetadata>,
    /// block_rewards defines a list of all block rewards objects.
    #[prost(message, repeated, tag = "3")]
    pub block_rewards: ::prost::alloc::vec::Vec<BlockRewards>,
    /// tx_rewards defines a list of all tx rewards objects.
    #[prost(message, repeated, tag = "4")]
    pub tx_rewards: ::prost::alloc::vec::Vec<TxRewards>,
    /// min_consensus_fee defines the minimum gas unit price.
    #[prost(message, optional, tag = "5")]
    pub min_consensus_fee: ::core::option::Option<
        ::cosmos_sdk_proto::cosmos::base::v1beta1::DecCoin,
    >,
    /// rewards_record_last_id defines the last unique ID for a RewardsRecord objs.
    #[prost(uint64, tag = "6")]
    pub rewards_record_last_id: u64,
    /// rewards_records defines a list of all active (undistributed) rewards
    /// records.
    #[prost(message, repeated, tag = "7")]
    pub rewards_records: ::prost::alloc::vec::Vec<RewardsRecord>,
    /// flat_fees defines a list of contract flat fee.
    #[prost(message, repeated, tag = "8")]
    pub flat_fees: ::prost::alloc::vec::Vec<FlatFee>,
}
/// QueryParamsRequest is the request for Query.Params.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response for Query.Params.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryContractMetadataRequest is the request for Query.ContractMetadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractMetadataRequest {
    /// contract_address is the contract address (bech32 encoded).
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
}
/// QueryContractMetadataResponse is the response for Query.ContractMetadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryContractMetadataResponse {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<ContractMetadata>,
}
/// QueryBlockRewardsTrackingRequest is the request for
/// Query.BlockRewardsTracking.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockRewardsTrackingRequest {}
/// QueryBlockRewardsTrackingResponse is the response for
/// Query.BlockRewardsTracking.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockRewardsTrackingResponse {
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<BlockTracking>,
}
/// QueryRewardsPoolRequest is the request for Query.RewardsPool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardsPoolRequest {}
/// QueryRewardsPoolResponse is the response for Query.RewardsPool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardsPoolResponse {
    /// undistributed_funds are undistributed yet tokens (ready for withdrawal).
    #[prost(message, repeated, tag = "1")]
    pub undistributed_funds: ::prost::alloc::vec::Vec<
        ::cosmos_sdk_proto::cosmos::base::v1beta1::Coin,
    >,
    /// treasury_funds are treasury tokens available (no mechanism is available to
    /// withdraw ATM). Treasury tokens are collected on a block basis. Those tokens
    /// are unused block rewards.
    #[prost(message, repeated, tag = "2")]
    pub treasury_funds: ::prost::alloc::vec::Vec<
        ::cosmos_sdk_proto::cosmos::base::v1beta1::Coin,
    >,
}
/// QueryEstimateTxFeesRequest is the request for Query.EstimateTxFees.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEstimateTxFeesRequest {
    /// gas_limit is the transaction gas limit.
    #[prost(uint64, tag = "1")]
    pub gas_limit: u64,
    /// contract_address whose flat fee is considered when estimating tx fees.
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
}
/// QueryEstimateTxFeesResponse is the response for Query.EstimateTxFees.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEstimateTxFeesResponse {
    /// gas_unit_price defines the minimum transaction fee per gas unit.
    #[prost(message, optional, tag = "1")]
    pub gas_unit_price: ::core::option::Option<
        ::cosmos_sdk_proto::cosmos::base::v1beta1::DecCoin,
    >,
    /// estimated_fee is the estimated transaction fee for a given gas limit.
    #[prost(message, repeated, tag = "2")]
    pub estimated_fee: ::prost::alloc::vec::Vec<
        ::cosmos_sdk_proto::cosmos::base::v1beta1::Coin,
    >,
}
/// BlockTracking is the tracking information for a block.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockTracking {
    /// inflation_rewards defines the inflation rewards for the block.
    #[prost(message, optional, tag = "1")]
    pub inflation_rewards: ::core::option::Option<BlockRewards>,
    /// tx_rewards defines the transaction rewards for the block.
    #[prost(message, repeated, tag = "2")]
    pub tx_rewards: ::prost::alloc::vec::Vec<TxRewards>,
}
/// QueryRewardsRecordsRequest is the request for Query.RewardsRecords.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardsRecordsRequest {
    /// rewards_address is the target address to query records for (bech32
    /// encoded).
    #[prost(string, tag = "1")]
    pub rewards_address: ::prost::alloc::string::String,
    /// pagination is an optional pagination options for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        ::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryRewardsRecordsResponse is the response for Query.RewardsRecords.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardsRecordsResponse {
    /// records is the list of rewards records.
    #[prost(message, repeated, tag = "1")]
    pub records: ::prost::alloc::vec::Vec<RewardsRecord>,
    /// pagination is the pagination details in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        ::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// QueryOutstandingRewardsRequest is the request for Query.OutstandingRewards.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOutstandingRewardsRequest {
    /// rewards_address is the target address to query calculated rewards for
    /// (bech32 encoded).
    #[prost(string, tag = "1")]
    pub rewards_address: ::prost::alloc::string::String,
}
/// QueryOutstandingRewardsResponse is the response for Query.OutstandingRewards.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOutstandingRewardsResponse {
    /// total_rewards is the total rewards credited to the rewards_address.
    #[prost(message, repeated, tag = "1")]
    pub total_rewards: ::prost::alloc::vec::Vec<
        ::cosmos_sdk_proto::cosmos::base::v1beta1::Coin,
    >,
    /// records_num is the total number of RewardsRecord objects stored for the
    /// rewards_address.
    #[prost(uint64, tag = "2")]
    pub records_num: u64,
}
/// QueryFlatFeeRequest is the request for Query.FlatFeet
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFlatFeeRequest {
    /// contract_address is the contract address (bech32 encoded).
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
}
/// QueryFlatFeeResponse is the response for Query.FlatFee
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFlatFeeResponse {
    /// flat_fee_amount defines the minimum flat fee set by the contract_owner per
    /// contract execution.
    #[prost(message, optional, tag = "1")]
    pub flat_fee_amount: ::core::option::Option<
        ::cosmos_sdk_proto::cosmos::base::v1beta1::Coin,
    >,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query service for the tracking module.
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Params returns module parameters.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryParamsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/archway.rewards.v1.Query/Params",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("archway.rewards.v1.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
        /// ContractMetadata returns the contract rewards parameters (metadata).
        pub async fn contract_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryContractMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryContractMetadataResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/archway.rewards.v1.Query/ContractMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("archway.rewards.v1.Query", "ContractMetadata"));
            self.inner.unary(req, path, codec).await
        }
        /// BlockRewardsTracking returns block rewards tracking for the current block.
        pub async fn block_rewards_tracking(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBlockRewardsTrackingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBlockRewardsTrackingResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/archway.rewards.v1.Query/BlockRewardsTracking",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("archway.rewards.v1.Query", "BlockRewardsTracking"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// RewardsPool returns the current undistributed rewards pool funds.
        pub async fn rewards_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRewardsPoolRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryRewardsPoolResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/archway.rewards.v1.Query/RewardsPool",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("archway.rewards.v1.Query", "RewardsPool"));
            self.inner.unary(req, path, codec).await
        }
        /// EstimateTxFees returns the estimated transaction fees for the given
        /// transaction gas limit using the minimum consensus fee value for the current
        /// block.
        pub async fn estimate_tx_fees(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryEstimateTxFeesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryEstimateTxFeesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/archway.rewards.v1.Query/EstimateTxFees",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("archway.rewards.v1.Query", "EstimateTxFees"));
            self.inner.unary(req, path, codec).await
        }
        /// RewardsRecords returns the paginated list of RewardsRecord objects stored
        /// for the provided rewards_address.
        pub async fn rewards_records(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRewardsRecordsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryRewardsRecordsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/archway.rewards.v1.Query/RewardsRecords",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("archway.rewards.v1.Query", "RewardsRecords"));
            self.inner.unary(req, path, codec).await
        }
        /// OutstandingRewards returns total rewards credited from different contracts
        /// for the provided rewards_address.
        pub async fn outstanding_rewards(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryOutstandingRewardsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryOutstandingRewardsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/archway.rewards.v1.Query/OutstandingRewards",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("archway.rewards.v1.Query", "OutstandingRewards"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// FlatFee returns the flat fee set by the contract owner for the provided
        /// contract_address
        pub async fn flat_fee(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryFlatFeeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryFlatFeeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/archway.rewards.v1.Query/FlatFee",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("archway.rewards.v1.Query", "FlatFee"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        /// Params returns module parameters.
        async fn params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryParamsResponse>,
            tonic::Status,
        >;
        /// ContractMetadata returns the contract rewards parameters (metadata).
        async fn contract_metadata(
            &self,
            request: tonic::Request<super::QueryContractMetadataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryContractMetadataResponse>,
            tonic::Status,
        >;
        /// BlockRewardsTracking returns block rewards tracking for the current block.
        async fn block_rewards_tracking(
            &self,
            request: tonic::Request<super::QueryBlockRewardsTrackingRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryBlockRewardsTrackingResponse>,
            tonic::Status,
        >;
        /// RewardsPool returns the current undistributed rewards pool funds.
        async fn rewards_pool(
            &self,
            request: tonic::Request<super::QueryRewardsPoolRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryRewardsPoolResponse>,
            tonic::Status,
        >;
        /// EstimateTxFees returns the estimated transaction fees for the given
        /// transaction gas limit using the minimum consensus fee value for the current
        /// block.
        async fn estimate_tx_fees(
            &self,
            request: tonic::Request<super::QueryEstimateTxFeesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryEstimateTxFeesResponse>,
            tonic::Status,
        >;
        /// RewardsRecords returns the paginated list of RewardsRecord objects stored
        /// for the provided rewards_address.
        async fn rewards_records(
            &self,
            request: tonic::Request<super::QueryRewardsRecordsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryRewardsRecordsResponse>,
            tonic::Status,
        >;
        /// OutstandingRewards returns total rewards credited from different contracts
        /// for the provided rewards_address.
        async fn outstanding_rewards(
            &self,
            request: tonic::Request<super::QueryOutstandingRewardsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryOutstandingRewardsResponse>,
            tonic::Status,
        >;
        /// FlatFee returns the flat fee set by the contract owner for the provided
        /// contract_address
        async fn flat_fee(
            &self,
            request: tonic::Request<super::QueryFlatFeeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryFlatFeeResponse>,
            tonic::Status,
        >;
    }
    /// Query service for the tracking module.
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Query> QueryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/archway.rewards.v1.Query/Params" => {
                    #[allow(non_camel_case_types)]
                    struct ParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryParamsRequest>
                    for ParamsSvc<T> {
                        type Response = super::QueryParamsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryParamsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ParamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/archway.rewards.v1.Query/ContractMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct ContractMetadataSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryContractMetadataRequest>
                    for ContractMetadataSvc<T> {
                        type Response = super::QueryContractMetadataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryContractMetadataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).contract_metadata(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ContractMetadataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/archway.rewards.v1.Query/BlockRewardsTracking" => {
                    #[allow(non_camel_case_types)]
                    struct BlockRewardsTrackingSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<
                        super::QueryBlockRewardsTrackingRequest,
                    > for BlockRewardsTrackingSvc<T> {
                        type Response = super::QueryBlockRewardsTrackingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryBlockRewardsTrackingRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).block_rewards_tracking(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BlockRewardsTrackingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/archway.rewards.v1.Query/RewardsPool" => {
                    #[allow(non_camel_case_types)]
                    struct RewardsPoolSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryRewardsPoolRequest>
                    for RewardsPoolSvc<T> {
                        type Response = super::QueryRewardsPoolResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRewardsPoolRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).rewards_pool(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RewardsPoolSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/archway.rewards.v1.Query/EstimateTxFees" => {
                    #[allow(non_camel_case_types)]
                    struct EstimateTxFeesSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryEstimateTxFeesRequest>
                    for EstimateTxFeesSvc<T> {
                        type Response = super::QueryEstimateTxFeesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryEstimateTxFeesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).estimate_tx_fees(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EstimateTxFeesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/archway.rewards.v1.Query/RewardsRecords" => {
                    #[allow(non_camel_case_types)]
                    struct RewardsRecordsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryRewardsRecordsRequest>
                    for RewardsRecordsSvc<T> {
                        type Response = super::QueryRewardsRecordsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRewardsRecordsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).rewards_records(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RewardsRecordsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/archway.rewards.v1.Query/OutstandingRewards" => {
                    #[allow(non_camel_case_types)]
                    struct OutstandingRewardsSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryOutstandingRewardsRequest>
                    for OutstandingRewardsSvc<T> {
                        type Response = super::QueryOutstandingRewardsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryOutstandingRewardsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).outstanding_rewards(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OutstandingRewardsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/archway.rewards.v1.Query/FlatFee" => {
                    #[allow(non_camel_case_types)]
                    struct FlatFeeSvc<T: Query>(pub Arc<T>);
                    impl<
                        T: Query,
                    > tonic::server::UnaryService<super::QueryFlatFeeRequest>
                    for FlatFeeSvc<T> {
                        type Response = super::QueryFlatFeeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryFlatFeeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).flat_fee(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FlatFeeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Query> Clone for QueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "archway.rewards.v1.Query";
    }
}
/// MsgSetContractMetadata is the request for Msg.SetContractMetadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetContractMetadata {
    /// sender_address is the msg sender address (bech32 encoded).
    #[prost(string, tag = "1")]
    pub sender_address: ::prost::alloc::string::String,
    /// metadata is the contract metadata to set / update.
    /// If metadata exists, non-empty fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<ContractMetadata>,
}
/// MsgSetContractMetadataResponse is the response for Msg.SetContractMetadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetContractMetadataResponse {}
/// MsgWithdrawRewards is the request for Msg.WithdrawRewards.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawRewards {
    /// rewards_address is the address to distribute rewards to (bech32 encoded).
    #[prost(string, tag = "1")]
    pub rewards_address: ::prost::alloc::string::String,
    /// mode defines the operation type.
    #[prost(oneof = "msg_withdraw_rewards::Mode", tags = "2, 3")]
    pub mode: ::core::option::Option<msg_withdraw_rewards::Mode>,
}
/// Nested message and enum types in `MsgWithdrawRewards`.
pub mod msg_withdraw_rewards {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RecordsLimit {
        #[prost(uint64, tag = "1")]
        pub limit: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RecordIDs {
        #[prost(uint64, repeated, packed = "false", tag = "1")]
        pub ids: ::prost::alloc::vec::Vec<u64>,
    }
    /// mode defines the operation type.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Mode {
        /// records_limit defines the maximum number of RewardsRecord objects to
        /// process. If provided limit is 0, the default limit is used.
        #[prost(message, tag = "2")]
        RecordsLimit(RecordsLimit),
        /// record_ids defines specific RewardsRecord object IDs to process.
        #[prost(message, tag = "3")]
        RecordIds(RecordIDs),
    }
}
/// MsgWithdrawRewardsResponse is the response for Msg.WithdrawRewards.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawRewardsResponse {
    /// records_num is the number of RewardsRecord objects processed.
    #[prost(uint64, tag = "1")]
    pub records_num: u64,
    /// rewards are the total rewards transferred.
    #[prost(message, repeated, tag = "2")]
    pub total_rewards: ::prost::alloc::vec::Vec<
        ::cosmos_sdk_proto::cosmos::base::v1beta1::Coin,
    >,
}
/// MsgSetFlatFee is the request for Msg.SetFlatFee.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetFlatFee {
    /// sender_address is the msg sender address (bech32 encoded).
    #[prost(string, tag = "1")]
    pub sender_address: ::prost::alloc::string::String,
    /// contract_address is the contract address (bech32 encoded).
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
    /// flat_fee_amount defines the minimum flat fee set by the contract_owner
    #[prost(message, optional, tag = "3")]
    pub flat_fee_amount: ::core::option::Option<
        ::cosmos_sdk_proto::cosmos::base::v1beta1::Coin,
    >,
}
/// MsgSetFlatFeeResponse is the response for Msg.SetFlatFee.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetFlatFeeResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Msg defines the module messaging service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// SetContractMetadata creates or updates an existing contract metadata.
        /// Method is authorized to the contract owner (admin if no metadata exists).
        pub async fn set_contract_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetContractMetadata>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetContractMetadataResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/archway.rewards.v1.Msg/SetContractMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("archway.rewards.v1.Msg", "SetContractMetadata"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// WithdrawRewards performs collected rewards distribution.
        /// Rewards might be credited from multiple contracts (rewards_address must be
        /// set in the corresponding contract metadata).
        pub async fn withdraw_rewards(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdrawRewards>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWithdrawRewardsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/archway.rewards.v1.Msg/WithdrawRewards",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("archway.rewards.v1.Msg", "WithdrawRewards"));
            self.inner.unary(req, path, codec).await
        }
        /// SetFlatFee sets or updates or removes the flat fee to interact with the
        /// contract Method is authorized to the contract owner.
        pub async fn set_flat_fee(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetFlatFee>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetFlatFeeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/archway.rewards.v1.Msg/SetFlatFee",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("archway.rewards.v1.Msg", "SetFlatFee"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MsgServer.
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        /// SetContractMetadata creates or updates an existing contract metadata.
        /// Method is authorized to the contract owner (admin if no metadata exists).
        async fn set_contract_metadata(
            &self,
            request: tonic::Request<super::MsgSetContractMetadata>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetContractMetadataResponse>,
            tonic::Status,
        >;
        /// WithdrawRewards performs collected rewards distribution.
        /// Rewards might be credited from multiple contracts (rewards_address must be
        /// set in the corresponding contract metadata).
        async fn withdraw_rewards(
            &self,
            request: tonic::Request<super::MsgWithdrawRewards>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWithdrawRewardsResponse>,
            tonic::Status,
        >;
        /// SetFlatFee sets or updates or removes the flat fee to interact with the
        /// contract Method is authorized to the contract owner.
        async fn set_flat_fee(
            &self,
            request: tonic::Request<super::MsgSetFlatFee>,
        ) -> std::result::Result<
            tonic::Response<super::MsgSetFlatFeeResponse>,
            tonic::Status,
        >;
    }
    /// Msg defines the module messaging service.
    #[derive(Debug)]
    pub struct MsgServer<T: Msg> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Msg> MsgServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MsgServer<T>
    where
        T: Msg,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/archway.rewards.v1.Msg/SetContractMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct SetContractMetadataSvc<T: Msg>(pub Arc<T>);
                    impl<
                        T: Msg,
                    > tonic::server::UnaryService<super::MsgSetContractMetadata>
                    for SetContractMetadataSvc<T> {
                        type Response = super::MsgSetContractMetadataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSetContractMetadata>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_contract_metadata(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetContractMetadataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/archway.rewards.v1.Msg/WithdrawRewards" => {
                    #[allow(non_camel_case_types)]
                    struct WithdrawRewardsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWithdrawRewards>
                    for WithdrawRewardsSvc<T> {
                        type Response = super::MsgWithdrawRewardsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWithdrawRewards>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).withdraw_rewards(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WithdrawRewardsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/archway.rewards.v1.Msg/SetFlatFee" => {
                    #[allow(non_camel_case_types)]
                    struct SetFlatFeeSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSetFlatFee>
                    for SetFlatFeeSvc<T> {
                        type Response = super::MsgSetFlatFeeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSetFlatFee>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).set_flat_fee(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetFlatFeeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Msg> Clone for MsgServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Msg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Msg> tonic::server::NamedService for MsgServer<T> {
        const NAME: &'static str = "archway.rewards.v1.Msg";
    }
}
