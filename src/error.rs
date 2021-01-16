//! Error types

use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the TokenSwap program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum SwapError {
    /// The account cannot be initialized because it is already being used.
    #[error("Swap account already in use")]
    AlreadyInUse,
    /// The address of the admin fee account is incorrect.
    #[error("Address of the admin fee account is incorrect")]
    InvalidAdmin,
    /// The owner of the input isn't set to the program address generated by the program.
    #[error("Input account owner is not the program address")]
    InvalidOwner,
    /// The owner of the pool token output is set to the program address generated by the program.
    #[error("Output pool account owner cannot be the program address")]
    InvalidOutputOwner,
    /// The program address provided doesn't match the value generated by the program.
    #[error("Invalid program address generated from nonce and key")]
    InvalidProgramAddress,
    /// The deserialization of the account returned something besides State::Mint.
    #[error("Deserialized account is not an SPL Token mint")]
    ExpectedMint,
    /// The deserialization of the account returned something besides State::Account.
    #[error("Deserialized account is not an SPL Token account")]
    ExpectedAccount,
    /// The pool supply is empty.
    #[error("Pool token supply is 0")]
    EmptyPool,
    /// The input token account is empty.
    #[error("Input token account empty")]
    EmptySupply,
    /// The pool token mint has a non-zero supply.
    #[error("Pool token mint has a non-zero supply")]
    InvalidSupply,
    /// The provided token account has a delegate.
    #[error("Token account has a delegate")]
    InvalidDelegate,
    /// The input token is invalid for swap.
    #[error("InvalidInput")]
    InvalidInput,
    /// Address of the provided swap token account is incorrect.
    #[error("Address of the provided swap token account is incorrect")]
    IncorrectSwapAccount,
    /// Address of the provided token mint is incorrect
    #[error("Address of the provided token mint is incorrect")]
    IncorrectMint,
    /// The calculation failed.
    #[error("CalculationFailure")]
    CalculationFailure,
    /// Invalid instruction number passed in.
    #[error("Invalid instruction")]
    InvalidInstruction,
    /// Swap input token accounts have the same mint
    #[error("Swap input token accounts have the same mint")]
    RepeatedMint,
    /// Swap instruction exceeds desired slippage limit
    #[error("Swap instruction exceeds desired slippage limit")]
    ExceededSlippage,
    /// The provided token account has a close authority.
    #[error("Token account has a close authority")]
    InvalidCloseAuthority,
    /// The pool token mint has a freeze authority.
    #[error("Pool token mint has a freeze authority")]
    InvalidFreezeAuthority,
    /// ConversionFailure
    #[error("Conversion to u64 failed with an overflow or underflow")]
    ConversionFailure,
    /// Unauthorized
    #[error("Account is not authorized to execute this instruction")]
    Unauthorized,
    /// Swap pool is paused
    #[error("Swap pool is paused")]
    IsPaused,
    /// Amp. coefficient change is within min ramp duration
    #[error("Ramp is locked in this time period")]
    RampLocked,
    /// Insufficient ramp time for the ramp operation
    #[error("Insufficient ramp time")]
    InsufficientRampTime,
    /// Active admin transfer in progress
    #[error("Active admin transfer in progress")]
    ActiveTransfer,
    /// No active admin transfer in progress
    #[error("No active admin transfer in progress")]
    NoActiveTransfer,
    /// Admin transfer deadline exceeded
    #[error("Admin transfer deadline exceeded")]
    AdminDeadlineExceeded,
    /// Token mint decimals must be the same.
    #[error("Token mints must have same decimals")]
    MismatchedDecimals,
}
impl From<SwapError> for ProgramError {
    fn from(e: SwapError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for SwapError {
    fn type_of() -> &'static str {
        "Swap Error"
    }
}
