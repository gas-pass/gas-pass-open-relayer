use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

/// Errors that may be returned by the GAS PASS program.
#[derive(Error, Debug, Copy, Clone)]
pub enum GasPassError {
    #[error("GAS PASS account already in use")]
    AccountAlreadyInUse,
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Not rent exempt")]
    NotRentExempt,
    #[error("Expected amount mismatch")]
    ExpectedAmountMismatch,
    #[error("Amount overflow")]
    AmountOverflow,
    #[error("Account not initialized")]
    AccountNotInitialized,
    #[error("Owner does not match")]
    OwnerMismatch,
}

impl From<GasPassError> for ProgramError {
    fn from(e: GasPassError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for GasPassError {
    fn type_of() -> &'static str {
        "GAS PASS Error"
    }
}
