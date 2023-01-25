use archway_sdk::types::archwayrewardsv1beta1::MsgSetContractMetadata;
use archway_test::mock_deps_archway;
use cosmwasm_std::{testing::mock_env, Binary, CosmosMsg, SubMsg, WasmMsg};

use crate::{contract::ADMIN, msg::ExecuteMsg, tests::common::mock_execute};

use super::common::{mock_instantiate, INIT_OWNER};

#[test]
fn instantiate_contract() {
    let mut deps = mock_deps_archway();
    mock_instantiate(deps.as_mut());

    let res = ADMIN.query_admin(deps.as_ref()).unwrap();
    assert_eq!(Some(INIT_OWNER.into()), res.admin);

    let msg = ExecuteMsg::InstantiateContract {
        code_id: 1,
        instantiate_msg: Binary(vec![]),
        label: "test".to_string(),
    };
    let res = mock_execute(deps.as_mut(), msg);

    let instantiate_msg = WasmMsg::Instantiate {
        admin: Some(mock_env().contract.address.to_string()),
        code_id: 1,
        msg: Binary(vec![]),
        funds: vec![],
        label: "test".to_string(),
    };
    assert_eq!(
        vec![SubMsg::reply_on_success(instantiate_msg, 1)],
        res.messages
    );
}

#[test]
fn set_contract_metadata() {
    let mut deps = mock_deps_archway();
    mock_instantiate(deps.as_mut());

    let res = ADMIN.query_admin(deps.as_ref()).unwrap();
    assert_eq!(Some(INIT_OWNER.into()), res.admin);

    let msg = ExecuteMsg::InstantiateContract {
        code_id: 1,
        instantiate_msg: Binary(vec![]),
        label: "test".to_string(),
    };
    mock_execute(deps.as_mut(), msg);

    let contract_address = "test_contract".to_string();
    let msg = ExecuteMsg::SetContractMetadata {
        contract_address: contract_address.clone(),
        rewards_address: Some(mock_env().contract.address.to_string()),
    };

    let res_set_metadata = mock_execute(deps.as_mut(), msg);

    let msg: CosmosMsg = MsgSetContractMetadata::new(
        &mock_env().contract.address,
        contract_address,
        &mock_env().contract.address,
        &mock_env().contract.address,
    )
    .into();
    assert_eq!(vec![SubMsg::new(msg)], res_set_metadata.messages)
}
