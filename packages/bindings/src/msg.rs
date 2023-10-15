use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, CosmosMsg, CustomMsg};

/// Custom messages to interact with the Archway Network bindings.
///
/// Those messages work in conjunction with the `cosmwasm_std::CosmosMsg::Custom` variant.
///
/// # Examples
///
/// ```
/// use cosmwasm_std::CosmosMsg;
/// use archway_bindings::ArchwayMsg;
///
/// let msg: CosmosMsg<ArchwayMsg> = CosmosMsg::Custom(
///     ArchwayMsg::UpdateContractMetadata {
///         contract_address: Some(String::from("contract_address")),
///         owner_address: Some(String::from("owner")),
///         rewards_address: Some(String::from("rewards")),
///     }
/// );
#[cw_serde]
pub enum ArchwayMsg {
    /// Updates a contract's metadata. Either `owner_address` or `rewards_address` must be provided.
    UpdateContractMetadata {
        /// If set to `None`, the metadata of the sender contract will be updated.
        /// In case the `contract_address` already has a metadata, the sender contract must be set
        /// as the `owner_address` to be able to update it.
        contract_address: Option<String>,
        /// If set to `None`, the contract's owner will not be updated.
        owner_address: Option<String>,
        /// If set to `None`, the contract's rewards address will not be updated.
        rewards_address: Option<String>,
    },
    /// Sets a premium fee for a contract. This action should be executed from a contract only if
    /// it's set as the `owner_address` in the metadata of `contract_address`. The tx will fail if
    /// the `contract_address` has no metadata.
    SetFlatFee {
        contract_address: String,
        flat_fee_amount: Coin,
    },
    /// Withdraws rewards from the contract. This action should be executed from a contract only if
    /// it's set as the `rewards_address` in a contract metadata. Only one of `records_limit` or
    /// `record_ids` should be set.
    ///
    /// # See also
    ///
    /// * [crate::types::rewards::WithdrawRewardsResponse]
    WithdrawRewards {
        /// Withdraw rewards up to a specified limit. If set to `None`, all rewards will be
        /// withdrawn up to the limit of records specified by the governance parameter
        /// `rewards.MaxWithdrawRecords`.
        records_limit: Option<u64>,
        /// Withdraw rewards by a list of record IDs.
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
    /// Creates an `ArchwayMsg` to update the current contract's metadata ownership.
    ///
    /// # Arguments
    ///
    /// * `owner_address` - The new owner address.
    pub fn update_rewards_ownership(owner_address: impl Into<String>) -> Self {
        ArchwayMsg::UpdateContractMetadata {
            contract_address: None,
            owner_address: Some(owner_address.into()),
            rewards_address: None,
        }
    }

    /// Creates an `ArchwayMsg` to update the ownership of an external contract metadata.
    ///
    /// # Arguments
    ///
    /// * `contract_address` - The other contract address.
    /// * `owner_address` - The new owner address.
    pub fn update_external_rewards_ownership(
        contract_address: impl Into<String>,
        owner_address: impl Into<String>,
    ) -> Self {
        ArchwayMsg::UpdateContractMetadata {
            contract_address: Some(contract_address.into()),
            owner_address: Some(owner_address.into()),
            rewards_address: None,
        }
    }

    /// Creates an `ArchwayMsg` to update the current contract's rewards address.
    ///
    /// # Arguments
    ///
    /// * `rewards_address` - The new rewards address.
    pub fn update_rewards_address(rewards_address: impl Into<String>) -> Self {
        ArchwayMsg::UpdateContractMetadata {
            contract_address: None,
            owner_address: None,
            rewards_address: Some(rewards_address.into()),
        }
    }

    /// Creates an `ArchwayMsg` to update the rewards address of an external contract metadata.
    ///
    /// # Arguments
    ///
    /// * `contract_address` - The other contract address.
    /// * `rewards_address` - The new rewards address.
    pub fn update_external_rewards_address(
        contract_address: impl Into<String>,
        rewards_address: impl Into<String>,
    ) -> Self {
        ArchwayMsg::UpdateContractMetadata {
            contract_address: Some(contract_address.into()),
            owner_address: None,
            rewards_address: Some(rewards_address.into()),
        }
    }

    /// Creates an `ArchwayMsg` to set a flat fee for a contract.
    ///
    /// This action should be executed from a contract only if it's set as the `owner_address` in
    /// the metadata of `contract_address`. The tx will fail if the `contract_address` has no
    /// metadata.
    ///
    /// # Arguments
    ///
    /// * `contract_address` - The contract address.
    /// * `amount` - The flat fee amount.
    pub fn set_flat_fee(contract_address: impl Into<String>, amount: Coin) -> Self {
        ArchwayMsg::SetFlatFee {
            contract_address: contract_address.into(),
            flat_fee_amount: amount,
        }
    }

    /// Creates an `ArchwayMsg` to withdraw all rewards from the contract up to the maximum limit
    /// of records specified by the governance parameter `rewards.MaxWithdrawRecords`.
    ///
    /// This action should be executed from a contract only if its set as the `rewards_address` in
    /// a contract metadata.
    pub fn withdraw_max_rewards() -> Self {
        ArchwayMsg::WithdrawRewards {
            records_limit: Some(0),
            record_ids: vec![],
        }
    }

    /// Creates an `ArchwayMsg` to withdraw rewards from the contract.
    ///
    /// This action should be executed from a contract only if its set as the `rewards_address` in
    /// a contract metadata.
    ///
    /// # Arguments
    ///
    /// * `limit` - Withdraw rewards up to a specified limit.
    pub fn withdraw_rewards_by_limit(limit: u64) -> Self {
        ArchwayMsg::WithdrawRewards {
            records_limit: Some(limit),
            record_ids: vec![],
        }
    }

    /// Creates an `ArchwayMsg` to withdraw rewards from the contract by a list of record IDs.
    ///
    /// This action should be executed from a contract only if its set as the `rewards_address` in
    /// a contract metadata.
    ///
    /// # Arguments
    ///
    /// * `record_ids` - List of record IDs to withdraw rewards from the rewards pool.
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
        let contract_address = String::from("contract_address");
        let owner_address = String::from("owner");
        let rewards_address = String::from("rewards");
        let update_metadata = ArchwayMsg::UpdateContractMetadata {
            contract_address: Some(contract_address),
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
