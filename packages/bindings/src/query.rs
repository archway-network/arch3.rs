use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::CustomQuery;

use crate::pagination::PageRequest;
use crate::types::{gov, rewards};

/// Queries for Archway's bindings.
#[cw_serde]
#[derive(QueryResponses)]
pub enum ArchwayQuery {
    /// Returns a [rewards::ContractMetadataResponse] for the given contract address.
    #[returns(rewards::ContractMetadataResponse)]
    ContractMetadata { contract_address: String },
    /// Returns a [rewards::FlatFeeResponse for the given contract address.
    #[returns(rewards::FlatFeeResponse)]
    FlatFee { contract_address: String },
    /// Returns a [rewards::RewardsRecordsResponse] containing a list of [rewards::RewardsRecord]
    /// objects that are credited for the account and are ready to be withdrawn. The request is
    /// paginated. If the limit field is not set, the governance param `rewards.MaxWithdrawRecords`
    /// is used.
    #[returns(rewards::RewardsRecordsResponse)]
    RewardsRecords {
        rewards_address: String,
        pagination: Option<PageRequest>,
    },
    /// Returns a [gov::VoteResponse] for the given proposal ID and voter.
    #[returns(gov::VoteResponse)]
    GovVote { proposal_id: u64, voter: String },
}

impl CustomQuery for ArchwayQuery {}

impl ArchwayQuery {
    /// Builds a query to get the contract metadata for the given contract address.
    pub fn contract_metadata(contract_address: impl Into<String>) -> Self {
        ArchwayQuery::ContractMetadata {
            contract_address: contract_address.into(),
        }
    }

    /// Builds a query to get the flat fee for the given contract address.
    pub fn flat_fee(contract_address: impl Into<String>) -> Self {
        ArchwayQuery::FlatFee {
            contract_address: contract_address.into(),
        }
    }

    /// Builds a query to list the rewards records for the given rewards address.
    pub fn rewards_records(rewards_address: impl Into<String>) -> Self {
        ArchwayQuery::RewardsRecords {
            rewards_address: rewards_address.into(),
            pagination: None,
        }
    }

    /// Builds a query to get a list of rewards records for the given rewards address with
    /// pagination.
    pub fn rewards_records_with_pagination(
        rewards_address: impl Into<String>,
        pagination: PageRequest,
    ) -> Self {
        ArchwayQuery::RewardsRecords {
            rewards_address: rewards_address.into(),
            pagination: Some(pagination),
        }
    }

    /// Builds a query to get the vote for the given proposal ID and voter.
    pub fn gov_vote(proposal_id: u64, voter: impl Into<String>) -> Self {
        ArchwayQuery::GovVote {
            proposal_id,
            voter: voter.into(),
        }
    }
}
