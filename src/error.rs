use cosmwasm_std::StdError;
use cw_controllers::AdminError;
use cw_utils::ParseReplyError;
use prost::EncodeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("{0}")]
    Reply(#[from] ParseReplyError),
    #[error("{0}")]
    Admin(#[from] AdminError),
    #[error("{0}")]
    Prost(#[from] EncodeError),
    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
