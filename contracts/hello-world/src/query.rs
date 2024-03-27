use cosmwasm_std::{entry_point, Deps, Env, to_json_binary, StdResult, Uint128, Binary};
use crate::{
    msg::QueryMsg,
    state::COUNT
};

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::COUNT{} => to_json_binary(&query_count(deps)?)
    }
}

fn query_count(deps: Deps) -> StdResult<Uint128> {
    let count = COUNT.load(deps.storage).unwrap();
    Ok(count)
}
