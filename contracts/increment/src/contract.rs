use archway_bindings::{ArchwayMsg, ArchwayQuery, ArchwayResult};
use cosmwasm_std::{entry_point, Addr, SubMsg};
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:increment";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const REWARDS_WITHDRAW_REPLY: u64 = 1001;

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
            update_rewards_address(rewards_address.unwrap_or(env.contract.address))
        }
        ExecuteMsg::WithdrawRewards {} => withdraw_rewards(),
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

pub fn update_rewards_address(rewards_address: Addr) -> ArchwayResult<ContractError> {
    let msg = ArchwayMsg::update_rewards_address(rewards_address);

    let res = Response::new()
        .add_message(msg)
        .add_attribute("method", "update_rewards_address");

    Ok(res)
}

fn withdraw_rewards() -> ArchwayResult<ContractError> {
    let msg = ArchwayMsg::withdraw_rewards_by_limit(0);

    let res = Response::new()
        .add_submessage(SubMsg::reply_on_success(msg, REWARDS_WITHDRAW_REPLY))
        .add_attribute("method", "withdraw_rewards");

    Ok(res)
}

#[entry_point]
pub fn query(deps: Deps<ArchwayQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
    }
}

fn query_count(deps: Deps<ArchwayQuery>) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count })
}

#[cfg(test)]
mod tests {
    use super::*;
    use archway_bindings::testing::{mock_dependencies, mock_dependencies_with_balance};
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, ContractResult, QueryResponse};

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
        let value: CountResponse = from_binary(&res).unwrap();
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
        let value: CountResponse = from_binary(&res).unwrap();
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
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }

    pub fn archway_query_handler(_query: &ArchwayQuery) -> ContractResult<QueryResponse> {
        todo!()
    }
}
