use archway_sdk::custom::{query::ArchwayQuery, types::ContractMetadataResponse};
use cosmwasm_std::{Deps, StdResult};

pub fn query(
    deps: Deps<ArchwayQuery>,
    contract_address: String,
) -> StdResult<ContractMetadataResponse> {
    let request = ArchwayQuery::contract_metadata(contract_address).into();
    deps.querier.query(&request)
}
