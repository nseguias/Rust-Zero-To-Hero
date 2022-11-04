use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Too many poll options")]
    TooManyOptions {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Poll not found")]
    PollNotFound {},

    #[error("Invalid option")]
    InvalidOption {},
}
