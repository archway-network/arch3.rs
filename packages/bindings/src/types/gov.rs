use cosmwasm_std::VoteOption;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
