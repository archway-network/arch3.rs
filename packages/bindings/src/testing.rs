use std::marker::PhantomData;

use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{Binary, Coin, ContractResult, OwnedDeps, SystemResult};

use crate::ArchwayQuery;

pub fn mock_dependencies(
    custom_handler: fn(&ArchwayQuery) -> ContractResult<Binary>,
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<ArchwayQuery>, ArchwayQuery> {
    mock_dependencies_with_balance(&[], custom_handler)
}

pub fn mock_dependencies_with_balance(
    contract_balance: &[Coin],
    custom_handler: fn(&ArchwayQuery) -> ContractResult<Binary>,
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<ArchwayQuery>, ArchwayQuery> {
    let custom_querier: MockQuerier<ArchwayQuery> =
        MockQuerier::new(&[(MOCK_CONTRACT_ADDR, contract_balance)])
            .with_custom_handler(move |query| SystemResult::Ok(custom_handler(query)));

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
        custom_query_type: PhantomData,
    }
}
