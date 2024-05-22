use cosmwasm_std::{
    coin, entry_point, to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply,
    Response, StakingMsg, StdError, StdResult, SubMsg, Uint128,
};
use cw2::set_contract_version;
use cw_utils::NativeBalance;

use archway_bindings::types::gov::VoteResponse;
use archway_bindings::types::rewards::{
    ContractMetadataResponse, FlatFeeResponse, RewardsRecordsResponse, WithdrawRewardsResponse,
};
use archway_bindings::{ArchwayMsg, ArchwayQuery, ArchwayResult, PageRequest};

use crate::error::ContractError;
use crate::helpers;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, OutstandingRewardsResponse, QueryMsg};
use crate::state::{State, STAKE_REWARDS_VALIDATOR, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:increment";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const STAKE_WITHDRAWN_REWARDS: u64 = 1001;

#[entry_point]
pub fn instantiate(
    deps: DepsMut<ArchwayQuery>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ArchwayResult<ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[entry_point]
pub fn execute(
    deps: DepsMut<ArchwayQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ArchwayResult<ContractError> {
    match msg {
        ExecuteMsg::Increment {} => increment(deps),
        ExecuteMsg::Reset { count } => reset(deps, info, count),
        ExecuteMsg::UpdateRewardsAddress { rewards_address } => {
            update_rewards_address(env, rewards_address)
        }
        ExecuteMsg::SetFlatFee { amount } => {
            set_flat_fee(deps.as_ref(), env.contract.address, amount)
        }
        ExecuteMsg::WithdrawRewards { stake_to_validator } => {
            withdraw_rewards(deps, env, stake_to_validator)
        }
    }
}

pub fn increment(deps: DepsMut<ArchwayQuery>) -> ArchwayResult<ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "increment"))
}

pub fn reset(
    deps: DepsMut<ArchwayQuery>,
    info: MessageInfo,
    count: i32,
) -> ArchwayResult<ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "reset"))
}

pub fn update_rewards_address(
    env: Env,
    rewards_address: Option<Addr>,
) -> ArchwayResult<ContractError> {
    let rewards_address = rewards_address.unwrap_or(env.contract.address);
    let msg = ArchwayMsg::update_rewards_address(rewards_address);

    let res = Response::new()
        .add_message(msg)
        .add_attribute("method", "update_rewards_address");

    Ok(res)
}

fn set_flat_fee(
    deps: Deps<ArchwayQuery>,
    contract_address: Addr,
    amount: Uint128,
) -> ArchwayResult<ContractError> {
    let denom = deps.querier.query_bonded_denom()?;
    let msg = ArchwayMsg::set_flat_fee(contract_address, coin(amount.u128(), denom));

    let res = Response::new()
        .add_message(msg)
        .add_attribute("method", "set_fees");

    Ok(res)
}

fn withdraw_rewards(
    deps: DepsMut<ArchwayQuery>,
    env: Env,
    stake_to_validator: Option<Addr>,
) -> ArchwayResult<ContractError> {
    let msg = ArchwayMsg::withdraw_max_rewards();
    let withdraw_rewards_msg = if let Some(validator_addr) = stake_to_validator {
        let tx_index = env.transaction.map(|tx| tx.index).unwrap_or(0);
        STAKE_REWARDS_VALIDATOR.save(
            deps.storage,
            (env.block.height, tx_index),
            &validator_addr,
        )?;
        SubMsg::reply_always(msg, STAKE_WITHDRAWN_REWARDS)
    } else {
        SubMsg::new(msg)
    };

    let res = Response::new()
        .add_submessage(withdraw_rewards_msg)
        .add_attribute("method", "withdraw_rewards");

    Ok(res)
}

#[entry_point]
pub fn reply(deps: DepsMut<ArchwayQuery>, env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        STAKE_WITHDRAWN_REWARDS => stake_contract_rewards(deps, env, msg),
        id => Err(StdError::not_found(format!("Unknown reply id: {}", id))),
    }
}

fn stake_contract_rewards(
    deps: DepsMut<ArchwayQuery>,
    env: Env,
    msg: Reply,
) -> StdResult<Response> {
    let tx_index = env.transaction.map(|tx| tx.index).unwrap_or(0);
    let validator_addr =
        STAKE_REWARDS_VALIDATOR.load(deps.storage, (env.block.height, tx_index))?;

    STAKE_REWARDS_VALIDATOR.remove(deps.storage, (env.block.height, tx_index));

    let data = helpers::parse_reply_data(msg)?;
    let withdraw_response: WithdrawRewardsResponse =
        serde_json_wasm::from_slice::<WithdrawRewardsResponse>(&data.0)
            .map_err(|e| StdError::generic_err(e.to_string()))?;

    let staking_denom = deps.querier.query_bonded_denom()?;
    let rewards_amount = withdraw_response
        .total_rewards
        .iter()
        .find(|coin| coin.denom == staking_denom)
        .ok_or_else(|| StdError::generic_err("Could not find coin with staking denom"))?;

    let staking_msg = StakingMsg::Delegate {
        validator: validator_addr.to_string(),
        amount: rewards_amount.clone(),
    };

    let res = Response::new()
        .add_message(staking_msg)
        .add_attribute("method", "stake_withdrawn_rewards")
        .add_attribute("records_num", withdraw_response.records_num.to_string())
        .add_attribute("staked_rewards", rewards_amount.to_string())
        .add_attribute("validator", validator_addr);

    Ok(res)
}

#[entry_point]
pub fn query(deps: Deps<ArchwayQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_json_binary(&query_count(deps)?),
        QueryMsg::Metadata { contract_address } => to_json_binary(&contract_metadata(
            deps,
            contract_address.unwrap_or(env.contract.address),
        )?),
        QueryMsg::FlatFee {} => to_json_binary(&flat_fee(deps, env.contract.address)?),
        QueryMsg::OutstandingRewards {} => to_json_binary(&outstanding_rewards(deps, env)?),
        QueryMsg::GovVote { proposal_id, voter } => {
            to_json_binary(&gov_vote(deps, proposal_id, voter)?)
        }
    }
}

fn query_count(deps: Deps<ArchwayQuery>) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count })
}

fn contract_metadata(
    deps: Deps<ArchwayQuery>,
    contract_address: impl Into<String>,
) -> StdResult<ContractMetadataResponse> {
    let req = ArchwayQuery::contract_metadata(contract_address).into();
    deps.querier.query(&req)
}

fn flat_fee(
    deps: Deps<ArchwayQuery>,
    contract_address: impl Into<String>,
) -> StdResult<FlatFeeResponse> {
    let req = ArchwayQuery::flat_fee(contract_address).into();
    deps.querier.query(&req)
}

fn outstanding_rewards(
    deps: Deps<ArchwayQuery>,
    env: Env,
) -> StdResult<OutstandingRewardsResponse> {
    let rewards_address = env.contract.address;
    let req = ArchwayQuery::rewards_records_with_pagination(
        rewards_address,
        PageRequest::new().count_total(),
    )
    .into();

    let response: RewardsRecordsResponse = deps.querier.query(&req)?;
    let rewards_coins = response
        .records
        .iter()
        .flat_map(|r| r.rewards.iter().cloned())
        .collect();
    let mut rewards_balance = NativeBalance(rewards_coins);
    rewards_balance.normalize();

    let total_records = response.pagination.and_then(|p| p.total).unwrap_or(0);

    Ok(OutstandingRewardsResponse {
        rewards_balance: rewards_balance.into_vec(),
        total_records,
    })
}

fn gov_vote(
    deps: Deps<ArchwayQuery>,
    proposal_id: u64,
    voter: impl Into<String>,
) -> StdResult<VoteResponse> {
    let req = ArchwayQuery::gov_vote(proposal_id, voter).into();
    let response: VoteResponse = deps.querier.query(&req)?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{coins, from_json, ContractResult, QueryResponse};

    use archway_bindings::testing::{mock_dependencies, mock_dependencies_with_balance};
    use archway_bindings::types::rewards::RewardsRecord;
    use archway_bindings::PageResponse;

    use super::*;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(archway_query_handler);

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_json(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"), archway_query_handler);

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_json(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"), archway_query_handler);

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_json(&res).unwrap();
        assert_eq!(5, value.count);
    }

    pub fn archway_query_handler(query: &ArchwayQuery) -> ContractResult<QueryResponse> {
        let response = match query {
            ArchwayQuery::ContractMetadata {
                contract_address: _,
            } => to_json_binary(&ContractMetadataResponse {
                owner_address: String::from("owner"),
                rewards_address: String::from("rewards"),
            }),

            ArchwayQuery::RewardsRecords {
                rewards_address: _,
                pagination: _,
            } => to_json_binary(&RewardsRecordsResponse {
                records: vec![
                    RewardsRecord {
                        id: 1,
                        rewards_address: String::from("rewards"),
                        rewards: coins(50, "coin"),
                        calculated_height: 12345,
                        calculated_time: String::from("2022-11-11T11:11:22"),
                    },
                    RewardsRecord {
                        id: 2,
                        rewards_address: String::from("rewards"),
                        rewards: coins(50, "coin"),
                        calculated_height: 12346,
                        calculated_time: String::from("2022-11-11T11:22:33"),
                    },
                ],
                pagination: Some(PageResponse {
                    next_key: None,
                    total: Some(2),
                }),
            }),

            _ => todo!(),
        };
        response.into()
    }
}
