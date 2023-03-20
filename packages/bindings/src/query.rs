use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CustomQuery;

use crate::pagination::PageRequest;
use crate::types::{gov, rewards};

#[cw_serde]
#[derive(QueryResponses)]
pub enum ArchwayQuery {
    #[returns(rewards::ContractMetadataResponse)]
    ContractMetadata { contract_address: String },
    #[returns(rewards::FlatFeeResponse)]
    FlatFee { contract_address: String },
    #[returns(rewards::RewardsRecordsResponse)]
    RewardsRecords {
        rewards_address: String,
        pagination: Option<PageRequest>,
    },
    #[returns(gov::VoteResponse)]
    GovVote { proposal_id: u64, voter: String },
}

impl CustomQuery for ArchwayQuery {}

impl ArchwayQuery {
    pub fn contract_metadata(contract_address: impl Into<String>) -> Self {
        ArchwayQuery::ContractMetadata {
            contract_address: contract_address.into(),
        }
    }

    pub fn flat_fee(contract_address: impl Into<String>) -> Self {
        ArchwayQuery::FlatFee {
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

    pub fn gov_vote(proposal_id: u64, voter: impl Into<String>) -> Self {
        ArchwayQuery::GovVote {
            proposal_id,
            voter: voter.into(),
        }
    }
}
