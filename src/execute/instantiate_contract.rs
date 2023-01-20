use cosmwasm_std::{Binary, DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg};
use cw_controllers::Admin;

use crate::{state::LABEL_CACHE, ContractError};

pub const INSTANTIATE_REPLY_ID: u64 = 1u64;

pub fn execute(
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
