use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

// List of managed contracts by this contract manager (PK:code_id,label,VALUE:contract_address)
pub const CONTRACTS_LIST: Map<(&str, &str), Addr> = Map::new("contracts_list");

// (PK:contract_address,VALUE:true/false)
pub const IS_METADATA_SET: Map<&Addr, bool> = Map::new("is_metadata_set");

// cache item to store temporarily the label of the contract to be instantiated
pub const LABEL_CACHE: Item<String> = Item::new("label_cache");
