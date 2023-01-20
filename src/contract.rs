#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
    StdResult, SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw_controllers::Admin;
use cw_utils::parse_reply_instantiate_data;

use crate::error::ContractError;
use crate::helpers::{value_from_attr_key, MessageExt};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{CONTRACTS_LIST, LABEL_CACHE};
use crate::types::archwayrewardsv1beta1::{ContractMetadata, MsgSetContractMetadata};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:contracts-manager";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const INSTANTIATE_REPLY_ID: u64 = 1u64;
const SET_CONTRACT_METADATA_REPLY_ID: u64 = 2u64;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Set the contract version for future migration
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Calculate the address to set for owner (if not set in the message takes the sender address
    // by default)
    let owner_addr = msg
        .owner
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?
        .unwrap_or(info.sender);
    let owner = Admin::new("owner");
    owner.set(deps, Some(owner_addr))?;

    Ok(Response::new().add_attribute("action", "instantiated_contract_manager"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::InstantiateContract {
            code_id,
            instantiate_msg,
            label,
        } => instantiate_contract(deps, env, info, code_id, instantiate_msg, label),
        ExecuteMsg::ChangeOwner { new_owner } => change_owner(deps, info, new_owner),
        ExecuteMsg::SetContractMetadata(contract_metadata) => {
            set_contract_metadata(deps, env, info, contract_metadata)
        }
        _ => unimplemented!(),
    }
}

fn instantiate_contract(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    code_id: u64,
    instantiate_msg: Binary,
    label: String,
) -> Result<Response, ContractError> {
    // Verify sender is the owner of the contracts-manager
    let owner = Admin::new("owner");
    owner.assert_admin(deps.as_ref(), &info.sender)?;

    // Temporary saving the label of the contract to be instantiated to be used in the reply
    LABEL_CACHE.save(deps.storage, &label)?;

    // Create the msg to send
    let instantiate_msg = WasmMsg::Instantiate {
        admin: Some(env.contract.address.to_string()),
        code_id,
        msg: instantiate_msg,
        funds: vec![],
        label: label.clone(),
    };

    let submessage = SubMsg::reply_on_success(instantiate_msg, INSTANTIATE_REPLY_ID);
    Ok(Response::new()
        .add_submessage(submessage)
        .add_attribute("internal_instantiation", "contracts_manager")
        .add_attribute("instantiated_code_id", code_id.to_string())
        .add_attribute("instantiated_label", label))
}

fn change_owner(
    deps: DepsMut,
    info: MessageInfo,
    new_owner: String,
) -> Result<Response, ContractError> {
    // Verify sender is the owner of the contracts-manager
    let owner = Admin::new("owner");
    owner.assert_admin(deps.as_ref(), &info.sender)?;

    // Change owner with the new address
    let new_owner_addr = deps.api.addr_validate(&new_owner)?;
    let res = owner.execute_update_admin(deps, info, Some(new_owner_addr))?;
    Ok(res)
}

fn set_contract_metadata(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    contract_metadata: ContractMetadata,
) -> Result<Response, ContractError> {
    // Verify sender is the owner of the contracts-manager
    let owner = Admin::new("owner");
    owner.assert_admin(deps.as_ref(), &info.sender)?;

    let msg = MsgSetContractMetadata {
        sender_address: env.contract.address.to_string(),
        metadata: Some(contract_metadata),
    };

    let cosmo_msg = CosmosMsg::Stargate {
        type_url: "/archway.rewards.v1beta1.MsgSetContractMetadata".to_owned(),
        value: Binary(msg.to_bytes()?),
    };

    let sub_msg = SubMsg::reply_on_success(cosmo_msg, SET_CONTRACT_METADATA_REPLY_ID);

    Ok(Response::new()
        .add_submessage(sub_msg)
        .add_attribute("action", "set_contract_metadata"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryInstantiatedContract { code_id, label } => {
            to_binary(&query_contract_address(deps, code_id, label)?)
        }
        _ => unimplemented!(),
    }
}

fn query_contract_address(deps: Deps, code_id: String, label: String) -> StdResult<Addr> {
    let contract_addr = CONTRACTS_LIST.load(deps.storage, (&code_id, &label))?;
    Ok(contract_addr)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        INSTANTIATE_REPLY_ID => handle_instantiate_reply(deps, msg),
        id => Err(ContractError::Std(StdError::GenericErr {
            msg: format!("Unknown reply id: {}", id),
        })),
    }
}

fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    // Handle the msg data and save the contract address
    let res = parse_reply_instantiate_data(msg.clone())?;
    let contract_addr = deps.api.addr_validate(&res.contract_address)?;

    // Retrieve code_id from event attribute
    let code_id = value_from_attr_key(msg, "code_id")?;

    let label = LABEL_CACHE.load(deps.storage)?;

    // Save the contract address to the store (initially the metadata is set to false)
    CONTRACTS_LIST.save(deps.storage, (&code_id, &label), &contract_addr)?;

    // Clear the cache
    LABEL_CACHE.remove(deps.storage);

    Ok(Response::new().add_attribute("action", "update_store"))
}

#[cfg(test)]
mod tests {}
