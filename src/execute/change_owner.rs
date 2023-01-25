use archway_sdk::custom::query::ArchwayQuery;
use cosmwasm_std::{DepsMut, MessageInfo, Response};

use crate::{contract::ADMIN, ContractError};

pub fn execute(
    deps: DepsMut<ArchwayQuery>,
    info: MessageInfo,
    new_owner: String,
) -> Result<Response, ContractError> {
    // Verify sender is the owner of the contracts-manager
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    // Change owner with the new address
    let new_owner_addr = deps.api.addr_validate(&new_owner)?;
    let res = ADMIN.execute_update_admin(deps, info, Some(new_owner_addr))?;
    Ok(res)
}
