use crate::{archway, cosmos::traits::TypeUrl};

impl TypeUrl for archway::genmsg::v1::GenesisState {
    const TYPE_URL: &'static str = "/archway.genmsg.v1.GenesisState";
}
impl TypeUrl for archway::rewards::v1::BlockRewards {
    const TYPE_URL: &'static str = "/archway.rewards.v1.BlockRewards";
}
impl TypeUrl for archway::rewards::v1::BlockTracking {
    const TYPE_URL: &'static str = "/archway.rewards.v1.BlockTracking";
}
impl TypeUrl for archway::rewards::v1::ContractFlatFeeSetEvent {
    const TYPE_URL: &'static str = "/archway.rewards.v1.ContractFlatFeeSetEvent";
}
impl TypeUrl for archway::rewards::v1::ContractMetadata {
    const TYPE_URL: &'static str = "/archway.rewards.v1.ContractMetadata";
}
impl TypeUrl for archway::rewards::v1::ContractMetadataSetEvent {
    const TYPE_URL: &'static str = "/archway.rewards.v1.ContractMetadataSetEvent";
}
impl TypeUrl for archway::rewards::v1::ContractRewardCalculationEvent {
    const TYPE_URL: &'static str = "/archway.rewards.v1.ContractRewardCalculationEvent";
}
impl TypeUrl for archway::rewards::v1::FlatFee {
    const TYPE_URL: &'static str = "/archway.rewards.v1.FlatFee";
}
impl TypeUrl for archway::rewards::v1::GenesisState {
    const TYPE_URL: &'static str = "/archway.rewards.v1.GenesisState";
}
impl TypeUrl for archway::rewards::v1::MinConsensusFeeSetEvent {
    const TYPE_URL: &'static str = "/archway.rewards.v1.MinConsensusFeeSetEvent";
}
impl TypeUrl for archway::rewards::v1::MsgSetContractMetadata {
    const TYPE_URL: &'static str = "/archway.rewards.v1.MsgSetContractMetadata";
}
impl TypeUrl for archway::rewards::v1::MsgSetContractMetadataResponse {
    const TYPE_URL: &'static str = "/archway.rewards.v1.MsgSetContractMetadataResponse";
}
impl TypeUrl for archway::rewards::v1::MsgSetFlatFee {
    const TYPE_URL: &'static str = "/archway.rewards.v1.MsgSetFlatFee";
}
impl TypeUrl for archway::rewards::v1::MsgSetFlatFeeResponse {
    const TYPE_URL: &'static str = "/archway.rewards.v1.MsgSetFlatFeeResponse";
}
impl TypeUrl for archway::rewards::v1::MsgWithdrawRewards {
    const TYPE_URL: &'static str = "/archway.rewards.v1.MsgWithdrawRewards";
}
impl TypeUrl for archway::rewards::v1::MsgWithdrawRewardsResponse {
    const TYPE_URL: &'static str = "/archway.rewards.v1.MsgWithdrawRewardsResponse";
}
impl TypeUrl for archway::rewards::v1::Params {
    const TYPE_URL: &'static str = "/archway.rewards.v1.Params";
}
impl TypeUrl for archway::rewards::v1::QueryBlockRewardsTrackingRequest {
    const TYPE_URL: &'static str = "/archway.rewards.v1.QueryBlockRewardsTrackingRequest";
}
impl TypeUrl for archway::rewards::v1::QueryBlockRewardsTrackingResponse {
    const TYPE_URL: &'static str = "/archway.rewards.v1.QueryBlockRewardsTrackingResponse";
}
impl TypeUrl for archway::rewards::v1::QueryContractMetadataRequest {
    const TYPE_URL: &'static str = "/archway.rewards.v1.QueryContractMetadataRequest";
}
impl TypeUrl for archway::rewards::v1::QueryContractMetadataResponse {
    const TYPE_URL: &'static str = "/archway.rewards.v1.QueryContractMetadataResponse";
}
impl TypeUrl for archway::rewards::v1::QueryEstimateTxFeesRequest {
    const TYPE_URL: &'static str = "/archway.rewards.v1.QueryEstimateTxFeesRequest";
}
impl TypeUrl for archway::rewards::v1::QueryEstimateTxFeesResponse {
    const TYPE_URL: &'static str = "/archway.rewards.v1.QueryEstimateTxFeesResponse";
}
impl TypeUrl for archway::rewards::v1::QueryFlatFeeRequest {
    const TYPE_URL: &'static str = "/archway.rewards.v1.QueryFlatFeeRequest";
}
impl TypeUrl for archway::rewards::v1::QueryFlatFeeResponse {
    const TYPE_URL: &'static str = "/archway.rewards.v1.QueryFlatFeeResponse";
}
impl TypeUrl for archway::rewards::v1::QueryOutstandingRewardsRequest {
    const TYPE_URL: &'static str = "/archway.rewards.v1.QueryOutstandingRewardsRequest";
}
impl TypeUrl for archway::rewards::v1::QueryOutstandingRewardsResponse {
    const TYPE_URL: &'static str = "/archway.rewards.v1.QueryOutstandingRewardsResponse";
}
impl TypeUrl for archway::rewards::v1::QueryParamsRequest {
    const TYPE_URL: &'static str = "/archway.rewards.v1.QueryParamsRequest";
}
impl TypeUrl for archway::rewards::v1::QueryParamsResponse {
    const TYPE_URL: &'static str = "/archway.rewards.v1.QueryParamsResponse";
}
impl TypeUrl for archway::rewards::v1::QueryRewardsPoolRequest {
    const TYPE_URL: &'static str = "/archway.rewards.v1.QueryRewardsPoolRequest";
}
impl TypeUrl for archway::rewards::v1::QueryRewardsPoolResponse {
    const TYPE_URL: &'static str = "/archway.rewards.v1.QueryRewardsPoolResponse";
}
impl TypeUrl for archway::rewards::v1::QueryRewardsRecordsRequest {
    const TYPE_URL: &'static str = "/archway.rewards.v1.QueryRewardsRecordsRequest";
}
impl TypeUrl for archway::rewards::v1::QueryRewardsRecordsResponse {
    const TYPE_URL: &'static str = "/archway.rewards.v1.QueryRewardsRecordsResponse";
}
impl TypeUrl for archway::rewards::v1::RewardsRecord {
    const TYPE_URL: &'static str = "/archway.rewards.v1.RewardsRecord";
}
impl TypeUrl for archway::rewards::v1::RewardsWithdrawEvent {
    const TYPE_URL: &'static str = "/archway.rewards.v1.RewardsWithdrawEvent";
}
impl TypeUrl for archway::rewards::v1::TxRewards {
    const TYPE_URL: &'static str = "/archway.rewards.v1.TxRewards";
}
impl TypeUrl for archway::tracking::v1::BlockTracking {
    const TYPE_URL: &'static str = "/archway.tracking.v1.BlockTracking";
}
impl TypeUrl for archway::tracking::v1::ContractOperationInfo {
    const TYPE_URL: &'static str = "/archway.tracking.v1.ContractOperationInfo";
}
impl TypeUrl for archway::tracking::v1::GenesisState {
    const TYPE_URL: &'static str = "/archway.tracking.v1.GenesisState";
}
impl TypeUrl for archway::tracking::v1::QueryBlockGasTrackingRequest {
    const TYPE_URL: &'static str = "/archway.tracking.v1.QueryBlockGasTrackingRequest";
}
impl TypeUrl for archway::tracking::v1::QueryBlockGasTrackingResponse {
    const TYPE_URL: &'static str = "/archway.tracking.v1.QueryBlockGasTrackingResponse";
}
impl TypeUrl for archway::tracking::v1::TxInfo {
    const TYPE_URL: &'static str = "/archway.tracking.v1.TxInfo";
}
impl TypeUrl for archway::tracking::v1::TxTracking {
    const TYPE_URL: &'static str = "/archway.tracking.v1.TxTracking";
}
