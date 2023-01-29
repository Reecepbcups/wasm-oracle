use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("Duplicate address: {address}")]
    DuplicateAddress { address: String },

    #[error("The following id can not be submitted: {id}")]
    InvalidId { id: String },

    #[error("Submitting too fast. Next submit allowed in: {blocks} blocks")]
    SubmittingTooQuickly { blocks: u64 },
}
