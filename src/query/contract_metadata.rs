use archway_sdk::custom::{query::ArchwayQuery, types::ContractMetadataResponse};
use cosmwasm_std::{Deps, StdResult};

pub fn query(
    deps: Deps<ArchwayQuery>,
    contract_address: String,
) -> StdResult<ContractMetadataResponse> {
    let contract_addr = deps.api.addr_validate(&contract_address)?;
    let request = ArchwayQuery::contract_metadata(contract_addr.to_string()).into();
    deps.querier.query(&request)
}
