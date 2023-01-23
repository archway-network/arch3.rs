use cosmwasm_std::VoteOption;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProposalStatus {
    ProposalStatusUnspecified,
    ProposalStatusDepositPeriod,
    ProposalStatusVotingPeriod,
    ProposalStatusPassed,
    ProposalStatusRejected,
    ProposalStatusFailed,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProposalsResponse {
    pub proposals: Vec<Proposal>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Proposal {
    pub proposal_id: u64,
    pub status: ProposalStatus,
    pub final_tally_result: TallyResult,
    pub submit_time: String,
    pub deposit_end_time: String,
    pub total_deposit: Vec<cosmwasm_std::Coin>,
    pub voting_start_time: String,
    pub voting_end_time: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TallyResult {
    pub yes: String,
    pub abstain: String,
    pub no: String,
    pub no_with_veto: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct VoteResponse {
    pub vote: Vote,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Vote {
    pub proposal_id: u64,
    pub voter: String,
    pub options: Vec<WeightedVoteOption>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct WeightedVoteOption {
    pub option: VoteOption,
    pub weight: String,
}
