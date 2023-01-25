use archway_sdk::custom::query::ArchwayQuery;
use cosmwasm_std::{
    from_binary,
    testing::{mock_env, mock_info},
    Deps, DepsMut, Response,
};
use serde::de::DeserializeOwned;

use crate::{
    contract::{execute, instantiate, query},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};

pub const INIT_OWNER: &str = "balda";

pub fn mock_instantiate(deps: DepsMut<ArchwayQuery>) {
    let msg = InstantiateMsg {
        owner: Some(INIT_OWNER.into()),
    };
    let info = mock_info("creator", &[]);
    instantiate(deps, mock_env(), info, msg).unwrap();
}

pub fn mock_execute(deps: DepsMut<ArchwayQuery>, msg: ExecuteMsg) -> Response {
    let env = mock_env();
    let info = mock_info("balda", &[]);
    execute(deps, env, info, msg).unwrap()
}

pub fn mock_query<T: DeserializeOwned>(deps: Deps<ArchwayQuery>, msg: QueryMsg) -> T {
    from_binary(&query(deps, mock_env(), msg).unwrap()).unwrap()
}
