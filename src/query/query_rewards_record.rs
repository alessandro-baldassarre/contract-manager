use cosmwasm_std::{Binary, Deps, QueryRequest, StdResult};

use crate::{
    helpers::MessageExt,
    types::archwayrewardsv1beta1::{QueryRewardsRecordsRequest, QueryRewardsRecordsResponse},
};

pub fn query(
    deps: Deps,
    records_request: QueryRewardsRecordsRequest,
) -> StdResult<QueryRewardsRecordsResponse> {
    let query = QueryRequest::Stargate {
        path: "/archway.rewards.v1beta1.Query/RewardsRecords".to_owned(),
        data: Binary(
            records_request
                .to_bytes()
                .expect("the encode should not fail"),
        ),
    };
    let res: QueryRewardsRecordsResponse = deps.querier.query(&query)?;
    Ok(res)
}
