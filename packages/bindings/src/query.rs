use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{pagination::PageRequest, Coins, PageResponse};

#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ArchwayQuery {
    ContractMetadata {
        contract_address: String,
    },
    RewardsRecords {
        rewards_address: String,
        pagination: Option<PageRequest>,
    },
}

impl CustomQuery for ArchwayQuery {}

impl ArchwayQuery {
    pub fn contract_metadata(contract_address: impl Into<String>) -> Self {
        ArchwayQuery::ContractMetadata {
            contract_address: contract_address.into(),
        }
    }

    pub fn rewards_records(rewards_address: impl Into<String>) -> Self {
        ArchwayQuery::RewardsRecords {
            rewards_address: rewards_address.into(),
            pagination: None,
        }
    }

    pub fn rewards_records_with_pagination(
        rewards_address: impl Into<String>,
        pagination: PageRequest,
    ) -> Self {
        ArchwayQuery::RewardsRecords {
            rewards_address: rewards_address.into(),
            pagination: Some(pagination),
        }
    }
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
