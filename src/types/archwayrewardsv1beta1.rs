use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{
    cosmosbasequeryv1beta1::{PageRequest, PageResponse},
    cosmosbasev1beta1::{Coin, DecCoin},
};

/// Params defines the module parameters.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct Params {
    /// inflation_rewards_ratio defines the percentage of minted inflation tokens that are used for dApp rewards [0.0, 1.0].
    /// If set to 0.0, no inflation rewards are distributed.
    #[prost(string, tag = "1")]
    pub inflation_rewards_ratio: ::prost::alloc::string::String,
    /// tx_fee_rebate_ratio defines the percentage of tx fees that are used for dApp rewards [0.0, 1.0].
    /// If set to 0.0, no fee rewards are distributed.
    #[prost(string, tag = "2")]
    pub tx_fee_rebate_ratio: ::prost::alloc::string::String,
    /// max_withdraw_records defines the maximum number of RewardsRecord objects used for the withdrawal operation.
    #[prost(uint64, tag = "3")]
    pub max_withdraw_records: u64,
}
/// ContractMetadata defines the contract rewards distribution options for a particular contract.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct ContractMetadata {
    /// contract_address defines the contract address (bech32 encoded).
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
    /// owner_address is the contract owner address that can modify contract reward options (bech32 encoded).
    /// That could be the contract admin or the contract itself.
    /// If owner_address is set to contract address, contract can modify the metadata on its own using WASM bindings.
    #[prost(string, tag = "2")]
    pub owner_address: ::prost::alloc::string::String,
    /// rewards_address is an address to distribute rewards to (bech32 encoded).
    /// If not set (empty), rewards are not distributed for this contract.
    #[prost(string, tag = "3")]
    pub rewards_address: ::prost::alloc::string::String,
}
/// BlockRewards defines block related rewards distribution data.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct BlockRewards {
    /// height defines the block height.
    #[prost(int64, tag = "1")]
    pub height: i64,
    /// inflation_rewards is the rewards to be distributed.
    #[prost(message, optional, tag = "2")]
    pub inflation_rewards: ::core::option::Option<Coin>,
    /// max_gas defines the maximum gas for the block that is used to distribute inflation rewards (consensus parameter).
    #[prost(uint64, tag = "3")]
    pub max_gas: u64,
}
/// TxRewards defines transaction related rewards distribution data.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct TxRewards {
    /// tx_id is the tracking transaction ID (x/tracking is the data source for this value).
    #[prost(uint64, tag = "1")]
    pub tx_id: u64,
    /// height defines the block height.
    #[prost(int64, tag = "2")]
    pub height: i64,
    /// fee_rewards is the rewards to be distributed.
    #[prost(message, repeated, tag = "3")]
    pub fee_rewards: ::prost::alloc::vec::Vec<Coin>,
}
/// RewardsRecord defines a record that is used to distribute rewards later (lazy distribution).
/// This record is being created by the x/rewards EndBlocker and pruned after the rewards are distributed.
/// An actual rewards x/bank transfer might be triggered by a Tx (via CLI for example) or by a contract via WASM bindings.
/// For a contract to trigger rewards transfer, contract address must be set as the rewards_address in a
/// corresponding ContractMetadata.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct RewardsRecord {
    /// id is the unique ID of the record.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// rewards_address is the address to distribute rewards to (bech32 encoded).
    #[prost(string, tag = "2")]
    pub rewards_address: ::prost::alloc::string::String,
    /// rewards are the rewards to be transferred later.
    #[prost(message, repeated, tag = "3")]
    pub rewards: ::prost::alloc::vec::Vec<Coin>,
    /// calculated_height defines the block height of rewards calculation event.
    #[prost(int64, tag = "4")]
    pub calculated_height: i64,
    /// calculated_time defines the block time of rewards calculation event.
    #[prost(message, optional, tag = "5")]
    pub calculated_time: ::core::option::Option<Timestamp>,
}
/// MsgSetContractMetadata is the request for Msg.SetContractMetadata.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct MsgSetContractMetadata {
    /// sender_address is the msg sender address (bech32 encoded).
    #[prost(string, tag = "1")]
    pub sender_address: ::prost::alloc::string::String,
    /// metadata is the contract metadata to set / update.
    /// If metadata exists, non-empty fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<ContractMetadata>,
}
/// MsgSetContractMetadataResponse is the response for Msg.SetContractMetadata.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct MsgSetContractMetadataResponse {}
/// MsgWithdrawRewards is the request for Msg.WithdrawRewards.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct MsgWithdrawRewards {
    /// rewards_address is the address to distribute rewards to (bech32 encoded).
    #[prost(string, tag = "1")]
    pub rewards_address: ::prost::alloc::string::String,
    /// mode defines the operation type.
    #[prost(oneof = "msg_withdraw_rewards::Mode", tags = "2, 3")]
    pub mode: ::core::option::Option<msg_withdraw_rewards::Mode>,
}
/// Nested message and enum types in `MsgWithdrawRewards`.
pub mod msg_withdraw_rewards {
    use super::*;
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
    pub struct RecordsLimit {
        #[prost(uint64, tag = "1")]
        pub limit: u64,
    }
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
    pub struct RecordIDs {
        #[prost(uint64, repeated, packed = "false", tag = "1")]
        pub ids: ::prost::alloc::vec::Vec<u64>,
    }
    /// mode defines the operation type.
    #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Oneof)]
    pub enum Mode {
        /// records_limit defines the maximum number of RewardsRecord objects to process.
        /// If provided limit is 0, the default limit is used.
        #[prost(message, tag = "2")]
        RecordsLimit(RecordsLimit),
        /// record_ids defines specific RewardsRecord object IDs to process.
        #[prost(message, tag = "3")]
        RecordIds(RecordIDs),
    }
}
/// MsgWithdrawRewardsResponse is the response for Msg.WithdrawRewards.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct MsgWithdrawRewardsResponse {
    /// records_num is the number of RewardsRecord objects processed.
    #[prost(uint64, tag = "1")]
    pub records_num: u64,
    /// rewards are the total rewards transferred.
    #[prost(message, repeated, tag = "2")]
    pub total_rewards: ::prost::alloc::vec::Vec<Coin>,
}
/// QueryParamsRequest is the request for Query.Params.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response for Query.Params.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryContractMetadataRequest is the request for Query.ContractMetadata.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct QueryContractMetadataRequest {
    /// contract_address is the contract address (bech32 encoded).
    #[prost(string, tag = "1")]
    pub contract_address: ::prost::alloc::string::String,
}
/// QueryContractMetadataResponse is the response for Query.ContractMetadata.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct QueryContractMetadataResponse {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<ContractMetadata>,
}
/// QueryBlockRewardsTrackingRequest is the request for Query.BlockRewardsTracking.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct QueryBlockRewardsTrackingRequest {}
/// QueryBlockRewardsTrackingResponse is the response for Query.BlockRewardsTracking.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct QueryBlockRewardsTrackingResponse {
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<BlockTracking>,
}
/// QueryRewardsPoolRequest is the request for Query.RewardsPool.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct QueryRewardsPoolRequest {}
/// QueryRewardsPoolResponse is the response for Query.RewardsPool.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct QueryRewardsPoolResponse {
    /// undistributed_funds are undistributed yet tokens (ready for withdrawal).
    #[prost(message, repeated, tag = "1")]
    pub undistributed_funds: ::prost::alloc::vec::Vec<Coin>,
    /// treasury_funds are treasury tokens available (no mechanism is available to withdraw ATM).
    /// Treasury tokens are collected on a block basis. Those tokens are unused block rewards.
    #[prost(message, repeated, tag = "2")]
    pub treasury_funds: ::prost::alloc::vec::Vec<Coin>,
}
/// QueryEstimateTxFeesRequest is the request for Query.EstimateTxFees.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct QueryEstimateTxFeesRequest {
    /// gas_limit is the transaction gas limit.
    #[prost(uint64, tag = "1")]
    pub gas_limit: u64,
}
/// QueryEstimateTxFeesResponse is the response for Query.EstimateTxFees.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct QueryEstimateTxFeesResponse {
    /// gas_unit_price defines the minimum transaction fee per gas unit.
    #[prost(message, optional, tag = "1")]
    pub gas_unit_price: ::core::option::Option<DecCoin>,
    /// estimated_fee is the estimated transaction fee for a given gas limit.
    #[prost(message, optional, tag = "2")]
    pub estimated_fee: ::core::option::Option<Coin>,
}
/// BlockTracking is the tracking information for a block.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct BlockTracking {
    /// inflation_rewards defines the inflation rewards for the block.
    #[prost(message, optional, tag = "1")]
    pub inflation_rewards: ::core::option::Option<BlockRewards>,
    /// tx_rewards defines the transaction rewards for the block.
    #[prost(message, repeated, tag = "2")]
    pub tx_rewards: ::prost::alloc::vec::Vec<TxRewards>,
}
/// QueryRewardsRecordsRequest is the request for Query.RewardsRecords.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct QueryRewardsRecordsRequest {
    /// rewards_address is the target address to query records for (bech32 encoded).
    #[prost(string, tag = "1")]
    pub rewards_address: ::prost::alloc::string::String,
    /// pagination is an optional pagination options for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<PageRequest>,
}
/// QueryRewardsRecordsResponse is the response for Query.RewardsRecords.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct QueryRewardsRecordsResponse {
    /// records is the list of rewards records.
    #[prost(message, repeated, tag = "1")]
    pub records: ::prost::alloc::vec::Vec<RewardsRecord>,
    /// pagination is the pagination details in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<PageResponse>,
}
/// QueryOutstandingRewardsRequest is the request for Query.OutstandingRewards.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct QueryOutstandingRewardsRequest {
    /// rewards_address is the target address to query calculated rewards for (bech32 encoded).
    #[prost(string, tag = "1")]
    pub rewards_address: ::prost::alloc::string::String,
}
/// QueryOutstandingRewardsResponse is the response for Query.OutstandingRewards.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct QueryOutstandingRewardsResponse {
    /// total_rewards is the total rewards credited to the rewards_address.
    #[prost(message, repeated, tag = "1")]
    pub total_rewards: ::prost::alloc::vec::Vec<Coin>,
    /// records_num is the total number of RewardsRecord objects stored for the rewards_address.
    #[prost(uint64, tag = "2")]
    pub records_num: u64,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ::prost::Message)]
pub struct Timestamp {
    /// Represents seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59Z inclusive.
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    /// Non-negative fractions of a second at nanosecond resolution. Negative
    /// second values with fractions must still have non-negative nanos values
    /// that count forward in time. Must be from 0 to 999,999,999
    /// inclusive.
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}
