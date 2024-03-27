
#[cfg(test)] 
pub mod test {
    use base::base_app::BaseApp;
    use cosmwasm_std::{Addr, Uint128};
    use hello_world::msg::{ExecuteMsg, QueryMsg};

    #[test]
    fn increase() {
        let mut base_app = BaseApp::default();
        let contract_addr = base_app.init_contract(Addr::unchecked("user1")).unwrap();

        let increase_msg = ExecuteMsg::Increase {  };

        let _increase = base_app
        .execute(Addr::unchecked("user1"), contract_addr.clone(), &increase_msg, &[])
        .unwrap();

        let count: Uint128 = base_app.app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &(QueryMsg::COUNT {  }))
        .unwrap();

        assert_eq!(count, Uint128::from(1u128));

        let decrease_msg = ExecuteMsg::Decrease {  };

        let _decrease = base_app
        .execute(Addr::unchecked("user1"), contract_addr.clone(), &decrease_msg, &[])
        .unwrap();

        let count: Uint128 = base_app.app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &(QueryMsg::COUNT {  }))
        .unwrap();

        assert_eq!(count, Uint128::from(0u128));
    }
}