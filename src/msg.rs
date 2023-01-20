use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary};

use crate::types::archwayrewardsv1beta1::{
    ContractMetadata, MsgWithdrawRewards, QueryContractMetadataRequest,
    QueryContractMetadataResponse, QueryOutstandingRewardsRequest, QueryOutstandingRewardsResponse,
    QueryRewardsRecordsRequest, QueryRewardsRecordsResponse,
};

#[cw_serde]
pub struct InstantiateMsg {
    /// Owner that can execute messages (not for migration), optional (if not set takes the sender
    /// address by default)
    pub owner: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Change admin (only current owner is authorized)
    ChangeOwner { new_owner: String },
    /// Instantiate a contract from the contracts-manager (only owner)
    InstantiateContract {
        /// Code-id of the contract you want instantiate
        code_id: u64,
        /// Base-64 encoded message for instantiate the contract to manage
        instantiate_msg: Binary,
        /// Label for the contract to instantiate
        label: String,
    },
    /// Set or update your contract metadata (only owner is authorized)
    SetContractMetadata(ContractMetadata),
    /// Performs collected rewards distribution (only owner is authorized)
    WithdrawRewards(MsgWithdrawRewards),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns the contract rewards parameters
    #[returns(QueryContractMetadataResponse)]
    QueryContractMetadata(QueryContractMetadataRequest),
    /// Returns the paginated list of RewardsRecord object stored for the provided
    /// rewards_address.
    #[returns(QueryRewardsRecordsResponse)]
    QueryRewardsRecords(QueryRewardsRecordsRequest),
    /// Returns total rewards credited from different contracts for the provided rewards_address.
    #[returns(QueryOutstandingRewardsResponse)]
    QueryOutstandingRewards(QueryOutstandingRewardsRequest),
    /// Returns the address of the contract instantiate from the manager for the provided
    /// code_id and label
    #[returns(Addr)]
    QueryInstantiatedContract { code_id: String, label: String },
}
