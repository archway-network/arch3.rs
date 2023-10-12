use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");
// Map with key (block height, tx index) and value validator address
pub const STAKE_REWARDS_VALIDATOR: Map<(u64, u32), Addr> = Map::new("stake_rewards_validator");
