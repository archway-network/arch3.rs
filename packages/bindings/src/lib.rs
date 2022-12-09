mod msg;
mod pagination;
mod query;

#[cfg(not(target_arch = "wasm32"))]
pub mod testing;

pub use msg::{ArchwayMsg, WithdrawRewardsResponse};
pub use pagination::{PageRequest, PageResponse};
pub use query::{ArchwayQuery, ContractMetadataResponse, RewardsRecord, RewardsRecordsResponse};

pub type Coins = Vec<cosmwasm_std::Coin>;

pub type ArchwayResult<E> = Result<cosmwasm_std::Response<ArchwayMsg>, E>;
