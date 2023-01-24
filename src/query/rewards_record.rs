use archway_sdk::custom::{
    query::ArchwayQuery,
    types::{PageRequest, RewardsRecordsResponse},
};
use cosmwasm_std::{Deps, StdResult};

pub fn query(
    deps: Deps<ArchwayQuery>,
    rewards_address: String,
    pagination: Option<PageRequest>,
) -> StdResult<RewardsRecordsResponse> {
    let request = match pagination {
        Some(page_request) => {
            ArchwayQuery::rewards_records_with_pagination(rewards_address, page_request).into()
        }
        None => ArchwayQuery::rewards_records(rewards_address).into(),
    };
    deps.querier.query(&request)
}
