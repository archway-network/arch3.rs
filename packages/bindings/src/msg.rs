use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, CustomMsg};

#[cw_serde]
pub enum ArchwayMsg {
    UpdateContractMetadata {
        owner_address: Option<String>,
        rewards_address: Option<String>,
    },
    WithdrawRewards {
        records_limit: Option<u64>,
        record_ids: Vec<u64>,
    },
}

impl CustomMsg for ArchwayMsg {}

impl From<ArchwayMsg> for CosmosMsg<ArchwayMsg> {
    fn from(msg: ArchwayMsg) -> Self {
        CosmosMsg::Custom(msg)
    }
}

impl ArchwayMsg {
    pub fn update_rewards_ownership(owner_address: impl Into<String>) -> Self {
        ArchwayMsg::UpdateContractMetadata {
            owner_address: Some(owner_address.into()),
            rewards_address: None,
        }
    }

    pub fn update_rewards_address(rewards_address: impl Into<String>) -> Self {
        ArchwayMsg::UpdateContractMetadata {
            owner_address: None,
            rewards_address: Some(rewards_address.into()),
        }
    }

    pub fn withdraw_rewards_by_limit(limit: u64) -> Self {
        ArchwayMsg::WithdrawRewards {
            records_limit: Some(limit),
            record_ids: vec![],
        }
    }

    pub fn withdraw_rewards_by_ids(record_ids: Vec<u64>) -> Self {
        ArchwayMsg::WithdrawRewards {
            records_limit: None,
            record_ids,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_archway_msg_works() {
        let owner_address = String::from("owner");
        let rewards_address = String::from("rewards");
        let update_metadata = ArchwayMsg::UpdateContractMetadata {
            owner_address: Some(owner_address),
            rewards_address: Some(rewards_address),
        };
        let msg: CosmosMsg<ArchwayMsg> = update_metadata.clone().into();
        match msg {
            CosmosMsg::Custom(serialized_metadata) => {
                assert_eq!(update_metadata, serialized_metadata)
            }
            _ => panic!("must encode in Custom variant"),
        }
    }
}
