use cosmwasm_std::Uint128;
use cw_storage_plus::Item;

pub const COUNT: Item<Uint128> = Item::new("count");