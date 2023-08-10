use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;

use crate::{Coins, PageResponse};

#[cw_serde]
pub struct WithdrawRewardsResponse {
    pub records_num: u64,
    pub total_rewards: Coins,
}

#[cw_serde]
pub struct ContractMetadataResponse {
    pub owner_address: String,
    pub rewards_address: String,
}

#[cw_serde]
pub struct FlatFeeResponse {
    pub flat_fee_amount: Coin,
}

#[cw_serde]
pub struct RewardsRecordsResponse {
    pub records: Vec<RewardsRecord>,
    pub pagination: Option<PageResponse>,
}

#[cw_serde]
pub struct RewardsRecord {
    pub id: u64,
    pub rewards_address: String,
    pub rewards: Coins,
    pub calculated_height: i64,
    pub calculated_time: String,
}
