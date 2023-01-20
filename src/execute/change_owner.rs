use cosmwasm_std::{DepsMut, MessageInfo, Response};
use cw_controllers::Admin;

use crate::ContractError;

pub fn execute(
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
