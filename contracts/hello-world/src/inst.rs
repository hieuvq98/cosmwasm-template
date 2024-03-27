use cosmwasm_std::{entry_point, Response, StdError, MessageInfo, Env, DepsMut};
use crate::{
    state::COUNT,
    msg::InstantiateMsg
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, StdError> {
    COUNT.save(deps.storage, &0u128.into())?;
    Ok(Response::new())
}