use cosmwasm_std::{DepsMut, Reply, Response};
use cw_utils::parse_reply_instantiate_data;

use crate::{
    helpers::value_from_attr_key,
    state::{Contract, CONTRACTS_LIST, LABEL_CACHE},
    ContractError,
};

pub fn reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    // Handle the msg data and save the contract address
    let res = parse_reply_instantiate_data(msg.clone())?;
    let contract_addr = deps.api.addr_validate(&res.contract_address)?;

    // Retrieve code_id from event attribute
    let code_id = value_from_attr_key(msg, "code_id")?;

    let label = LABEL_CACHE.load(deps.storage)?;

    let contract = Contract {
        contract_address: contract_addr,
        label,
    };

    // Save the contract address to the store (initially the metadata is set to false)
    CONTRACTS_LIST.save(deps.storage, &code_id, &contract)?;

    // Clear the cache
    LABEL_CACHE.remove(deps.storage);

    Ok(Response::new().add_attribute("action", "update_store"))
}
