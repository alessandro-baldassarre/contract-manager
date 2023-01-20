use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

// List of managed contracts by this contract manager (PK:code_id,label,VALUE:contract_address)
pub const CONTRACTS_LIST: Map<(&str, &str), Addr> = Map::new("contracts_list");

// cache item to store temporarily the label of the contract to be instantiated
pub const LABEL_CACHE: Item<String> = Item::new("label_cache");
