use cosmwasm_schema::cw_serde;
use cosmwasm_std::VoteOption;

#[cw_serde]
pub struct VoteResponse {
    pub vote: Vote,
}

#[cw_serde]
pub struct Vote {
    pub proposal_id: u64,
    pub voter: String,
    pub options: Vec<WeightedVoteOption>,
}

#[cw_serde]
pub struct WeightedVoteOption {
    pub option: VoteOption,
    pub weight: String,
}
