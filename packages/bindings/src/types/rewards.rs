use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Coins, PageResponse};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct WithdrawRewardsResponse {
    pub records_num: u64,
    pub total_rewards: Coins,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ContractMetadataResponse {
    pub owner_address: String,
    pub rewards_address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct RewardsRecordsResponse {
    pub records: Vec<RewardsRecord>,
    pub pagination: Option<PageResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct RewardsRecord {
    pub id: u64,
    pub rewards_address: String,
    pub rewards: Coins,
    pub calculated_height: i64,
    pub calculated_time: String,
}
