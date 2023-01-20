use cosmwasm_std::{Addr, Deps, StdResult};

use crate::state::CONTRACTS_LIST;

pub fn query(deps: Deps, code_id: String, label: String) -> StdResult<Addr> {
    let contract_addr = CONTRACTS_LIST.load(deps.storage, (&code_id, &label))?;
    Ok(contract_addr)
}
