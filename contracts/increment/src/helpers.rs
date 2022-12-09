use cosmwasm_std::{Binary, Reply, StdError, StdResult, SubMsgResponse};

pub fn parse_reply_result(reply: Reply) -> StdResult<SubMsgResponse> {
    reply.result.into_result().map_err(StdError::generic_err)
}

pub fn parse_reply_data(reply: Reply) -> StdResult<Binary> {
    parse_reply_result(reply)?
        .data
        .ok_or_else(|| StdError::generic_err("Missing reply data".to_owned()))
}
