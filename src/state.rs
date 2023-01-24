use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Contract {
    pub contract_address: Addr,
    pub label: String,
}

// List of managed contracts by this contract manager (PK:code_id,VALUE:(contract_address,label)
pub const CONTRACTS_LIST: Map<&str, Contract> = Map::new("contracts_list");

// cache item to store temporarily the label of the contract to be instantiated
pub const LABEL_CACHE: Item<String> = Item::new("label_cache");
