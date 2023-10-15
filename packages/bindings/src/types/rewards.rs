use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;

use crate::{Coins, PageResponse};

/// Response to a [crate::ArchwayQuery::ContractMetadata] query.
#[cw_serde]
pub struct ContractMetadataResponse {
    /// Address of the contract owner (the one who can modify rewards parameters).
    pub owner_address: String,
    /// Address of the rewards (the one who can withdraw rewards).
    pub rewards_address: String,
}

/// Response to a [crate::ArchwayQuery::FlatFee] query.
#[cw_serde]
pub struct FlatFeeResponse {
    pub flat_fee_amount: Coin,
}

/// Response to a [crate::ArchwayQuery::RewardsRecords] query.
#[cw_serde]
pub struct RewardsRecordsResponse {
    pub records: Vec<RewardsRecord>,
    pub pagination: Option<PageResponse>,
}

/// Defines a record that is used to distribute rewards later (lazy distribution). This record is
/// created by the `x/rewards` EndBlocker and pruned after the rewards are distributed. An actual
/// rewards `x/bank` transfer can be triggered by a contract via WASM bindings. For a contract to
/// trigger rewards transfer, the sender contract address must be set as the `rewards_address` in
/// the corresponding ContractMetadata.
#[cw_serde]
pub struct RewardsRecord {
    /// The unique ID of the record.
    pub id: u64,
    /// Address to distribute rewards to (bech32 encoded).
    pub rewards_address: String,
    /// Rewards amount to distribute.
    pub rewards: Coins,
    /// Height at which the record was created.
    pub calculated_height: i64,
    /// Time at which the record was created, in RFC3339Nano format.
    pub calculated_time: String,
}

/// Returned by a [crate::ArchwayMsg::WithdrawRewards] message. It can be accessed by listening to
/// a reply from the message.
///
/// See the [sample contract](https://github.com/archway-network/arch3.rs/blob/main/contracts/increment/src/contract.rs) for an example.
#[cw_serde]
pub struct WithdrawRewardsResponse {
    /// The number of records that were withdrawn.
    pub records_num: u64,
    /// The total amount of rewards that were withdrawn.
    pub total_rewards: Coins,
}
