use cosmwasm_std::{Binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, SubMsg};
use cw_controllers::Admin;

use crate::{
    helpers::MessageExt,
    types::archwayrewardsv1beta1::{ContractMetadata, MsgSetContractMetadata},
    ContractError,
};

pub const SET_CONTRACT_METADATA_REPLY_ID: u64 = 2u64;

pub fn execute(
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

    let cosmo_msg: CosmosMsg = CosmosMsg::Stargate {
        type_url: "/archway.rewards.v1beta1.MsgSetContractMetadata".to_owned(),
        value: Binary(msg.to_bytes()?),
    };

    let sub_msg = SubMsg::reply_on_success(cosmo_msg, SET_CONTRACT_METADATA_REPLY_ID);

    Ok(Response::new()
        .add_submessage(sub_msg)
        .add_attribute("action", "set_contract_metadata"))
}
