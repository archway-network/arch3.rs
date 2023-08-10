use archway_bindings::{
    types::{gov, rewards},
    Coins,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
    UpdateRewardsAddress { rewards_address: Option<Addr> },
    SetFlatFee { amount: Uint128 },
    WithdrawRewards {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(CountResponse)]
    GetCount {},
    #[returns(rewards::ContractMetadataResponse)]
    Metadata { contract_address: Option<Addr> },
    #[returns(rewards::FlatFeeResponse)]
    FlatFee {},
    #[returns(OutstandingRewardsResponse)]
    OutstandingRewards {},
    #[returns(gov::VoteResponse)]
    GovVote { proposal_id: u64, voter: Addr },
}

#[cw_serde]
pub struct CountResponse {
    pub count: i32,
}

#[cw_serde]
pub struct OutstandingRewardsResponse {
    pub rewards_balance: Coins,
    pub total_records: u64,
}
