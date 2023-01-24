use archway_sdk::custom::query::ArchwayQuery;
use cosmwasm_std::{Deps, Order, StdResult};

use crate::state::{Contract, CONTRACTS_LIST};

pub fn query(deps: Deps<ArchwayQuery>) -> StdResult<Vec<(String, Contract)>> {
    let contracts: StdResult<Vec<_>> = CONTRACTS_LIST
        .range(deps.storage, None, None, Order::Descending)
        .collect();
    Ok(contracts?)
}
