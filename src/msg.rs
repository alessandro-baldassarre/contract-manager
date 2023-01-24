use archway_sdk::custom::types::{ContractMetadataResponse, PageRequest, RewardsRecordsResponse};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;

#[cw_serde]
pub struct InstantiateMsg {
    /// Owner that can execute messages (not for migration), optional (if not set takes the sender
    /// address by default)
    pub owner: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Change owner (only current owner is authorized)
    ChangeOwner { new_owner: String },
    /// Instantiate a contract from the contracts-manager (only owner is authorized)
    InstantiateContract {
        /// Code-id of the contract you want instantiate
        code_id: u64,
        /// Base-64 encoded message for instantiate the contract to manage
        instantiate_msg: Binary,
        /// Label for the contract to instantiate
        label: String,
    },
    /// Set your contract metadata (only the owner of the contracts-manager is authorized)
    SetContractMetadata {
        /// Defines the contract address to set metadata
        contract_address: String,
        /// Address to distribute rewards to, if no specified contracts-manager address by default
        rewards_address: Option<String>,
    },
    /// Update your contract metadata (only the owner of the contracts-manager is authorized)
    UpdateContractMetadata {
        /// Defines the contract address to set metadata
        contract_address: String,
        /// Address to distribute rewards to, if no specified contracts-manager address by default
        rewards_address: Option<String>,
    },
    /// Performs collected rewards distribution with a maximum number of rewards
    /// record object to process.(only owner if the contracts-manager)
    WithdrawRewardsRecordsLimit {
        rewards_address: String,
        records_limit: u64,
    },
    /// Performs collected rewards distribution with specific rewards record object IDs to
    /// process.(only owner if the contracts-manager)
    WithdrawRewardsRecordsIds {
        rewards_address: String,
        record_ids: Vec<u64>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Request contract metadata of the provided contract address
    #[returns(ContractMetadataResponse)]
    ContractMetadata { contract_address: String },
    /// Request rewards record with an optional pagination
    #[returns(RewardsRecordsResponse)]
    RewardsRecord { pagination: Option<PageRequest> },
}
