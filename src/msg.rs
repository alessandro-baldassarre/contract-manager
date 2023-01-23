use archway_sdk::types::{
    archwayrewardsv1beta1::{QueryContractMetadataResponse, QueryRewardsRecordsResponse},
    cosmosbasequeryv1beta1::PageRequest,
};
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
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Request contract metadata of the contract address provided
    #[returns(QueryContractMetadataResponse)]
    ContractMetadata { contract_address: String },
    /// Request rewards record of the contract address provided with an optional pagination
    #[returns(QueryRewardsRecordsResponse)]
    RewardsRecord {
        contract_address: String,
        pagination: Option<PageRequest>,
    },
}
