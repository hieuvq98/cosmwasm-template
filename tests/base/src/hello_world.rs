use cosmwasm_std::{Addr, StdError};
use hello_world::msg::InstantiateMsg;
use cw_multi_test::Executor;

use crate::base_app::BaseApp;

impl BaseApp {
    pub fn init_contract(&mut self, sender: Addr) -> Result<Addr, StdError> {
        let msg = InstantiateMsg { };
        let res = self
          .app
          .instantiate_contract(self.contract_code_id, sender, &msg, &[], "Hello_world", None)
          .unwrap();
        Ok(res)
      }
}