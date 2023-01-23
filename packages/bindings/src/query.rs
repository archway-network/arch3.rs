use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::pagination::PageRequest;
use crate::types::gov::ProposalStatus;

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
    GovProposals {
        voter: Option<String>,
        depositor: Option<String>,
        status: Option<ProposalStatus>,
        page: u32,
        limit: u32,
    },
    GovVote {
        proposal_id: u64,
        voter: String,
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

    pub fn gov_proposals_by_voter<T: Into<String>>(voter: impl Into<String>) -> Self {
        ArchwayQuery::gov_proposals(Some(voter.into()), None::<T>, None)
    }

    pub fn gov_proposals_by_depositor<T: Into<String>>(depositor: impl Into<String>) -> Self {
        ArchwayQuery::gov_proposals(None::<T>, Some(depositor.into()), None)
    }

    pub fn gov_proposals_by_status<T: Into<String>>(status: ProposalStatus) -> Self {
        ArchwayQuery::gov_proposals(None::<T>, None::<T>, Some(status))
    }

    pub fn gov_proposals(
        voter: Option<impl Into<String>>,
        depositor: Option<impl Into<String>>,
        status: Option<ProposalStatus>,
    ) -> Self {
        ArchwayQuery::gov_proposals_at_page(voter, depositor, status, 1)
    }

    pub fn gov_proposals_at_page(
        voter: Option<impl Into<String>>,
        depositor: Option<impl Into<String>>,
        status: Option<ProposalStatus>,
        page: u32,
    ) -> Self {
        ArchwayQuery::GovProposals {
            voter: voter.map(|v| v.into()),
            depositor: depositor.map(|v| v.into()),
            status,
            page,
            limit: 100,
        }
    }

    pub fn gov_vote(proposal_id: u64, voter: impl Into<String>) -> Self {
        ArchwayQuery::GovVote {
            proposal_id,
            voter: voter.into(),
        }
    }
}
