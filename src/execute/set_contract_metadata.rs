use archway_sdk::{
    custom::query::ArchwayQuery, types::archwayrewardsv1beta1::MsgSetContractMetadata,
};
use cosmwasm_std::{CosmosMsg, DepsMut, Env, MessageInfo, Response};

use crate::{contract::ADMIN, ContractError};
pub fn execute(
    deps: DepsMut<ArchwayQuery>,
    env: Env,
    info: MessageInfo,
    contract_address: String,
    rewards_address: Option<String>,
) -> Result<Response, ContractError> {
    // Verify sender is the owner of the contracts-manager
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let contracts_manager_addr = env.contract.address;

    let contract_address = deps.api.addr_validate(&contract_address)?;

    let rewards_address = rewards_address
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?
        .unwrap_or_else(|| contracts_manager_addr.clone());

    // Create stargate msg to dispatch
    let msg: CosmosMsg = MsgSetContractMetadata::new(
        &contracts_manager_addr,
        &contract_address,
        &contracts_manager_addr,
        &rewards_address,
    )
    .into();

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "set_contract_metadata"))
}
