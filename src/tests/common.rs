use archway_sdk::custom::query::ArchwayQuery;
use cosmwasm_std::{
    testing::{mock_env, mock_info},
    DepsMut,
};

use crate::{contract::instantiate, msg::InstantiateMsg};

pub const INIT_OWNER: &str = "balda";

pub fn mock_instantiate(deps: DepsMut<ArchwayQuery>) {
    let msg = InstantiateMsg {
        owner: Some(INIT_OWNER.into()),
    };
    let info = mock_info("creator", &[]);
    instantiate(deps, mock_env(), info, msg).unwrap();
}
