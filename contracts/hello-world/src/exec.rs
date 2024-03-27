use cosmwasm_std::{entry_point, Env, MessageInfo, StdError, Response, DepsMut};
use crate::{
    msg::ExecuteMsg,
    state::COUNT
};

#[cfg_attr(not(feature = "library"), entry_point)]#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::Increase{ } => increase(deps),
        ExecuteMsg::Decrease{ } => decrease(deps),
    }
}

fn increase(deps: DepsMut) -> Result<Response, StdError> {
    let mut count = COUNT.load(deps.storage).unwrap();
    count = count.checked_add(1u128.into()).unwrap();
    COUNT.save(deps.storage, &count)?;
    Ok(Response::new())
}

fn decrease(deps: DepsMut) -> Result<Response, StdError> {
    let mut count = COUNT.load(deps.storage).unwrap();
    count = count.checked_sub(1u128.into()).unwrap();
    COUNT.save(deps.storage, &count)?;
    Ok(Response::new())
}