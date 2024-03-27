
use cosmwasm_schema::serde::Serialize;
use cw_multi_test::{App, AppResponse, Contract, ContractWrapper};
use cosmwasm_std::{Addr, Coin, Empty};
use cw_multi_test::Executor;
use anyhow::Error;

pub struct BaseApp {
    pub app: App,
    pub contract_code_id: u64
}

impl Default for BaseApp {
    fn default() -> Self {
        let mut app: App = App::default();
        let contract_code_id = app.store_code(BaseApp::contract());

        Self {
            app,
            contract_code_id
        }
    }
}

impl BaseApp {
    fn contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
          hello_world::exec::execute,
          hello_world::inst::instantiate,
          hello_world::query::query,
        );
        Box::new(contract)
    }

    pub fn execute<T: Serialize + std::fmt::Debug>(
    &mut self,
    sender: Addr,
    contract_addr: Addr,
    msg: &T,
    send_funds: &[Coin]
    ) -> Result<AppResponse, Error> {
    let result = self
        .app
        .execute_contract(sender, contract_addr, &msg, send_funds);
        if result.is_ok() {
        Ok(result.unwrap())
        }
        else {return Err(result.unwrap_err())}
    }
}