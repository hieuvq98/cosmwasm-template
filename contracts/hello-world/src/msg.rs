use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub enum ExecuteMsg {
    Increase{ },
    Decrease{ },
}

#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    #[returns(u64)]
    COUNT{ },
}

#[cw_serde]
pub struct InstantiateMsg {
}