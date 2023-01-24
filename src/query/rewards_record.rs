use archway_sdk::custom::{
    query::ArchwayQuery,
    types::{PageRequest, RewardsRecordsResponse},
};
use cosmwasm_std::{Deps, Env, StdResult};

pub fn query(
    deps: Deps<ArchwayQuery>,
    env: Env,
    pagination: Option<PageRequest>,
) -> StdResult<RewardsRecordsResponse> {
    let request = match pagination {
        Some(page_request) => {
            ArchwayQuery::rewards_records_with_pagination(&env.contract.address, page_request)
                .into()
        }
        None => ArchwayQuery::rewards_records(&env.contract.address).into(),
    };
    deps.querier.query(&request)
}
