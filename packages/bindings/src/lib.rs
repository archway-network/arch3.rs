mod msg;
mod pagination;
mod query;

pub mod types;

#[cfg(not(target_arch = "wasm32"))]
pub mod testing;

pub use msg::ArchwayMsg;
pub use pagination::{PageRequest, PageResponse};
pub use query::ArchwayQuery;

pub type Coins = Vec<cosmwasm_std::Coin>;

pub type ArchwayResult<E> = Result<cosmwasm_std::Response<ArchwayMsg>, E>;
